use std::time::Duration;

use anyhow::anyhow;
use futures_util::{SinkExt, StreamExt};
use models::{PluginConfig, RegistedPlugin};
use tokio::{net::TcpListener, sync::broadcast::Sender};
use tokio_tungstenite::tungstenite::Message;
use url::Url;

pub struct PluginPre {
    pub tcp_listener: TcpListener,
    daemon_address: String,
    pub registed_plugin: RegistedPlugin,
}

impl PluginPre {
    pub async fn new(
        daemon_address: String,
        plugin_config: PluginConfig,
        regist_address: Option<String>,
        port: u16,
    ) -> anyhow::Result<Self> {
        let tcp_listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;

        let addr = match regist_address {
            None => format!("http://{}", tcp_listener.local_addr()?),
            Some(value) => value,
        };

        Ok(PluginPre {
            daemon_address,
            tcp_listener,
            registed_plugin: RegistedPlugin {
                addr,
                config: plugin_config,
            },
        })
    }

    pub async fn create_regist_plugin_handle(&self) -> anyhow::Result<Sender<()>> {
        let mut regist_plugin_address = Url::parse(&self.daemon_address)?.join("api/plugin")?;
        match regist_plugin_address.scheme() {
            "http" => {
                regist_plugin_address.set_scheme("ws").unwrap();
            }
            "https" => {
                regist_plugin_address.set_scheme("wss").unwrap();
            }
            _ => {}
        };

        let (ws_stream, _response) =
            tokio_tungstenite::connect_async(regist_plugin_address.to_string()).await?;
        let (mut write, mut read) = ws_stream.split();

        match read.next().await.unwrap()? {
            Message::Text(text) => {
                if text != "Ready" {
                    return Err(anyhow!("注册 Plugin 失败"));
                }
            }
            _ => return Err(anyhow!("注册 Plugin 失败")),
        }

        write
            .send(Message::text(
                serde_json::to_string(&self.registed_plugin).unwrap(),
            ))
            .await?;

        let (tx, _rx) = tokio::sync::broadcast::channel::<()>(4);
        tokio::task::spawn({
            let tx = tx.clone();
            async move {
                let mut sub = tx.subscribe();
                loop {
                    tokio::select! {
                        _ = tokio::time::sleep(Duration::from_secs(4)) => {
                            write.send(Message::Ping(Vec::new())).await.unwrap();
                        },
                        _ = sub.recv() => break,
                    }
                }
                let _ = tx.send(());
            }
        });
        tokio::task::spawn({
            let tx = tx.clone();
            async move {
                let mut sub = tx.subscribe();
                loop {
                    tokio::select! {
                        _ = tokio::time::sleep(Duration::from_secs(10)) => break,
                        _ = sub.recv() => break,
                        recv = read.next() => {
                            match recv {
                                Some(_) => (),
                                None => break,
                            }
                        },
                    }
                }
                let _ = tx.send(());
            }
        });

        Ok(tx)
    }
}
