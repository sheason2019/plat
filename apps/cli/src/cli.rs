use std::{env, fs, path::PathBuf};

use anyhow::anyhow;
use bundler::{tarer::Tarer, untarer::Untarer};
use clap::{Parser, Subcommand};
use daemon::{daemon::PluginDaemon, service::PluginDaemonService};
use plugin::{models::PluginConfig, PluginService};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
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
    Daemon {
        #[arg(short, long)]
        path: Option<PathBuf>,
        #[arg(short, long)]
        port: Option<u16>,
        #[arg(short, long)]
        ephemeral: Option<bool>,
    },
}

impl Cli {
    pub async fn work(&self) -> anyhow::Result<()> {
        match &self.command {
            Some(Commands::Tar { path, output }) => {
                Tarer::new(path.clone()).tar(output.clone()).unwrap()
            }
            Some(Commands::Untar { path, output }) => {
                Untarer::new(path.clone()).untar(output.clone()).unwrap()
            }
            Some(Commands::Daemon {
                path,
                port,
                ephemeral,
            }) => {
                let port = match port.as_ref() {
                    Some(val) => val.clone(),
                    None => 0,
                };

                if ephemeral.is_some() {
                    if ephemeral.unwrap() {
                        let daemon = PluginDaemon::new_random()?;
                        let service =
                            PluginDaemonService::new(daemon, env::current_dir()?, port).await?;
                        println!("start daemon success:");
                        println!(
                            "daemon address: {}",
                            service.plugin_daemon.address.as_ref().unwrap()
                        );
                        service.wait().await?;
                        return Ok(());
                    }
                }

                let path = path.as_ref().unwrap();
                let daemon: PluginDaemon = serde_json::from_slice(&fs::read(path)?)?;
                let service = PluginDaemonService::new(daemon, env::current_dir()?, port).await?;
                println!("start daemon success:");
                println!(
                    "daemon address: {}",
                    service.plugin_daemon.address.as_ref().unwrap()
                );
                service.wait().await?;
                return Ok(());
            }
            Some(Commands::Serve {
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
                    false => std::env::current_dir()?,
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
            }
            None => {}
        };

        Ok(())
    }
}
