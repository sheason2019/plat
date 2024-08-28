use std::{collections::HashMap, path::PathBuf};

use anyhow::{anyhow, Context};
use daemon::{PlatXConfig, RegistedPlugin};
use server::ServerHandler;

pub mod daemon;
pub mod server;

pub struct PlatX {
    pub registed_plugin: RegistedPlugin,
    pub handler: Option<ServerHandler>,
    plugin_root: PathBuf,
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
            handler: None,
        })
    }

    pub async fn start_server(
        &mut self,
        deamon_address: String,
        plugin_address: Option<String>,
    ) -> anyhow::Result<()> {
        // 启动服务
        let handler = server::start_server(
            self.plugin_root.clone(),
            self.registed_plugin.config.clone(),
        )
        .await
        .context("create router failed")?;
        self.handler = Some(handler);
        self.registed_plugin.addr = self.handler.as_ref().unwrap().addr.clone();

        // 向 Daemon 服务器注册服务，若注册失败则停止服务
        let url = url::Url::parse(&deamon_address)
            .context(format!(
                "parse daemon address {} failed",
                deamon_address.clone()
            ))?
            .join("plugin")?;
        let mut data = HashMap::new();
        match plugin_address {
            Some(value) => {
                data.insert("addr", value.clone());
            }
            None => {
                data.insert("addr", self.registed_plugin.addr.clone());
            }
        }

        // TODO: 替换 reqwest
        // let client = reqwest::Client::new();
        // let response = match client.post(url).json(&data).send().await {
        //     Err(_) => {
        //         self.stop();
        //         return Err(anyhow!("send regist plugin request failed"));
        //     }
        //     Ok(response) => response,
        // };
        // if !response.status().is_success() {
        //     self.stop();
        //     return Err(anyhow!("regist plugin failed: {}", response.text().await?));
        // }

        Ok(())
    }

    pub fn stop(&self) {
        self.handler.as_ref().unwrap().handler.abort();
    }

    pub fn delete_in_fs(&self) -> anyhow::Result<()> {
        std::fs::remove_dir_all(self.plugin_root.clone())?;
        Ok(())
    }
}
