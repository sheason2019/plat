use clap::{Parser, Subcommand};

use platx_core::{
    bundler::{tarer::Tarer, untarer::Untarer},
    platx::{daemon::PlatXDaemon, PlatX},
};

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
        daemon_address: Option<String>,
        #[arg(short, long)]
        regist_address: Option<String>,
        #[arg(short, long)]
        port: Option<u16>,
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
            Some(Commands::Serve {
                path,
                daemon_address,
                regist_address,
                port,
            }) => {
                let daemon_address_string = match daemon_address {
                    None => {
                        let mut daemon = PlatXDaemon::new();
                        let _ = daemon.start_server().await?;
                        daemon.addr
                    }
                    Some(address) => address.clone(),
                };
                // 启动 Plugin Daemon
                println!("plugin daemon started on: {}", &daemon_address_string);

                let port = match port {
                    Some(val) => *val,
                    None => 0,
                };
                // 启动 Plugin
                let mut plugin = PlatX::from_plugin_root(path.clone())?;
                plugin
                    .start_server(port, daemon_address_string, regist_address.clone())
                    .await?;
                println!("plugin server started on: {}", plugin.registed_plugin.addr);
                plugin.handler.unwrap().handler.await?;
            }
            None => {}
        };

        Ok(())
    }
}
