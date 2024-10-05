use std::{env, fs, path::PathBuf};

use clap::{command, Args, Subcommand};
use daemon::{daemon::PluginDaemon, service::PluginDaemonService};

#[derive(Debug, Args)]
pub struct DaemonArgs {
    #[command(subcommand)]
    command: Option<DaemonCommands>,
}

#[derive(Debug, Subcommand)]
pub enum DaemonCommands {
    Init {},
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
        #[arg(short, long)]
        path: Option<PathBuf>,
        #[arg(short, long)]
        port: Option<u16>,
    },
}

impl DaemonArgs {
    pub async fn work(&self) -> anyhow::Result<()> {
        match &self.command {
            Some(DaemonCommands::Serve { path, port }) => {
                let port = match port.as_ref() {
                    Some(val) => val.clone(),
                    None => 0,
                };

                let path = path.as_ref().unwrap();
                let daemon: PluginDaemon = serde_json::from_slice(&fs::read(path)?)?;
                let service = PluginDaemonService::new(
                    daemon,
                    env::current_dir()?
                        .join(path)
                        .parent()
                        .unwrap()
                        .to_path_buf(),
                    port,
                )
                .await?;
                println!("start daemon success:");
                println!(
                    "daemon address: {}",
                    service.plugin_daemon.address.as_ref().unwrap()
                );
                service.wait().await?;
                Ok(())
            }
            Some(DaemonCommands::Tar { path, output }) => {
                bundler::daemon::tar(path.clone(), output.clone())
            }
            Some(DaemonCommands::Untar { path, output }) => {
                bundler::daemon::untar(path.clone(), output.clone())
            }
            _ => Ok(()),
        }
    }
}
