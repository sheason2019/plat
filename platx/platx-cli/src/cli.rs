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
        plugin_address: Option<String>,
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
                plugin_address,
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

                // 启动 Plugin
                let plugin_server_tcp_listener =
                    tokio::net::TcpListener::bind("127.0.0.1:0").await?;
                let mut plugin = PlatX::from_plugin_root(path.clone())?;
                let handler = plugin
                    .start_server(
                        plugin_server_tcp_listener,
                        daemon_address_string,
                        plugin_address.clone(),
                    )
                    .await?;
                println!("plugin server started on: {}", plugin.registed_plugin.addr);
                handler.await?;
            }
            None => {}
        };

        Ok(())
    }
}
