use std::path::PathBuf;

use crate::platx_config::PlatXConfig;
use anyhow::{Context, Ok};
use tokio::sync::mpsc::{self, Sender};

pub mod server;

pub struct PlatX {
    pub port: u16,
    pub config: PlatXConfig,
    plugin_root: PathBuf,
    stop_server_sender: Option<Sender<bool>>,
}

impl PlatX {
    pub fn from_plugin_root(plugin_root: PathBuf) -> anyhow::Result<Self> {
        let config = PlatXConfig::from_path(plugin_root.clone())?;

        Ok(PlatX {
            config,
            port: 0,
            plugin_root,
            stop_server_sender: None,
        })
    }

    pub async fn start_server(
        &mut self,
        listener: tokio::net::TcpListener,
    ) -> anyhow::Result<tokio::task::JoinHandle<()>> {
        let (tx, mut rx) = mpsc::channel::<bool>(1);
        let router = server::create_router(self.plugin_root.clone(), self.config.clone())
            .context("create router failed")?;

        self.port = listener.local_addr()?.port();
        self.stop_server_sender = Some(tx.clone());

        let handler = tokio::task::spawn(async move {
            let tx = tx.clone();

            axum::serve(listener, router)
                .with_graceful_shutdown(async move {
                    rx.recv().await;
                })
                .await
                .expect("start axum server failed");

            tx.send(true).await.expect("send message failed");
        });

        Ok(handler)
    }

    pub async fn stop(&self) {
        self.stop_server_sender
            .as_ref()
            .unwrap()
            .clone()
            .send(true)
            .await
            .unwrap();
    }

    pub fn delete_in_fs(&self) -> anyhow::Result<()> {
        std::fs::remove_dir_all(self.plugin_root.clone())?;
        Ok(())
    }
}
