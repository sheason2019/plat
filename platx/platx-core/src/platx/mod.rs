use std::{collections::HashMap, path::PathBuf};

use anyhow::{anyhow, Context};
use daemon::{PlatXConfig, RegistedPlugin};
use tokio::sync::mpsc::Sender;

use crate::utils;

pub mod daemon;
pub mod server;

pub struct PlatX {
    pub registed_plugin: RegistedPlugin,
    plugin_root: PathBuf,
    stop_server_sender: Option<Sender<()>>,
}

impl PlatX {
    pub fn from_plugin_root(plugin_root: PathBuf) -> anyhow::Result<Self> {
        let config = PlatXConfig::from_path(plugin_root.clone())?;

        Ok(PlatX {
            registed_plugin: RegistedPlugin {
                addr: String::new(),
                config,
            },
            plugin_root,
            stop_server_sender: None,
        })
    }

    pub async fn start_server(
        &mut self,
        listener: tokio::net::TcpListener,
        deamon_address: String,
    ) -> anyhow::Result<tokio::task::JoinHandle<()>> {
        // 启动服务
        let router = server::create_router(
            self.plugin_root.clone(),
            self.registed_plugin.config.clone(),
        )
        .context("create router failed")?;
        self.registed_plugin.addr = format!("http://{}", listener.local_addr()?.to_string());

        let (handler, tx) = utils::start_server_with_graceful_shutdown_channel(listener, router);
        self.stop_server_sender = Some(tx.clone());

        // 向 Daemon 服务器注册服务，若注册失败则停止服务
        let url = url::Url::parse(&deamon_address)
            .context(format!(
                "parse daemon address {} failed",
                deamon_address.clone()
            ))?
            .join("plugin")?;
        let mut data = HashMap::new();
        data.insert("addr", self.registed_plugin.addr.clone());

        let client = reqwest::Client::new();
        let response = match client.post(url).json(&data).send().await {
            Err(_) => {
                tx.send(()).await?;
                return Err(anyhow!("send regist plugin request failed"));
            }
            Ok(response) => response,
        };
        if !response.status().is_success() {
            tx.send(()).await?;
            return Err(anyhow!("regist plugin failed: {}", response.text().await?));
        }

        Ok(handler)
    }

    pub async fn stop(&self) {
        self.stop_server_sender
            .as_ref()
            .unwrap()
            .clone()
            .send(())
            .await
            .unwrap();
    }

    pub fn delete_in_fs(&self) -> anyhow::Result<()> {
        std::fs::remove_dir_all(self.plugin_root.clone())?;
        Ok(())
    }
}
