use std::fs;

use anyhow::anyhow;
use clap::{command, Args, Subcommand};
use plugin::{models::PluginConfig, PluginService};

#[derive(Debug, Args)]
pub struct PluginArgs {
    #[command(subcommand)]
    command: Option<PluginCommands>,
}

#[derive(Debug, Subcommand)]
pub enum PluginCommands {
    Tar {
        path: std::path::PathBuf,
        #[arg(short, long)]
        output: std::path::PathBuf,
    },
    Untar {
        path: std::path::PathBuf,
        #[arg(short, long)]
        output: std::path::PathBuf,
    },
    Serve {
        path: std::path::PathBuf,
        #[arg(short, long)]
        daemon_address: String,
        #[arg(short, long)]
        regist_address: Option<String>,
        #[arg(short, long)]
        port: Option<u16>,
    },
}

impl PluginArgs {
    pub async fn work(&self) -> anyhow::Result<()> {
        match self.command.as_ref() {
            Some(PluginCommands::Tar { path, output }) => {
                let config_path = match path.is_dir() {
                    true => path.join("plugin.json"),
                    false => path.clone(),
                };
                bundler::plugin::tar(config_path, output.clone())
            }
            Some(PluginCommands::Untar { path, output }) => {
                bundler::plugin::untar(path.clone(), output.clone())
            }
            Some(PluginCommands::Serve {
                path,
                daemon_address,
                regist_address,
                port,
            }) => {
                let port = match port {
                    Some(val) => *val,
                    None => 0,
                };

                let plugin_path = match path.is_absolute() {
                    true => path.clone(),
                    false => std::env::current_dir()?.join(path),
                };
                let plugin_path = match plugin_path.is_dir() {
                    true => plugin_path.join("plugin.json"),
                    false => plugin_path,
                };
                if !plugin_path.exists() {
                    return Err(anyhow!("未找到指定的 Plugin 配置文件"));
                }

                let mut plugin_config: PluginConfig =
                    serde_json::from_slice(&fs::read(&plugin_path)?)?;
                plugin_config.daemon_address = Some(daemon_address.clone());
                plugin_config.regist_address = regist_address.clone();

                // 启动 Plugin
                let service = PluginService::new(plugin_path, plugin_config, port).await?;

                println!("start plugin success:");
                println!("plugin address: {}", service.addr().unwrap());
                println!("daemon address: {}", daemon_address);

                // 等待服务停止
                service.wait().await;
                Ok(())
            }
            None => Ok(()),
        }
    }
}
