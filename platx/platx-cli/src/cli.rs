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
    },
    Untar {
        path: std::path::PathBuf,
        #[arg(short, long)]
        output: std::path::PathBuf,
    },
    Serve {
        path: std::path::PathBuf,
    },
}

impl Cli {
    pub async fn work(&self) -> anyhow::Result<()> {
        match &self.command {
            Some(Commands::Tar { path }) => Tarer::new(path.clone()).tar().unwrap(),
            Some(Commands::Untar { path, output }) => {
                Untarer::new(path.clone()).untar(output.clone()).unwrap()
            }
            Some(Commands::Serve { path }) => {
                // 启动 Plugin Daemon
                let mut daemon = PlatXDaemon::new();
                daemon.start_server().await?;
                println!("plugin daemon started on: {}", daemon.addr.to_string());

                // 启动 Plugin
                let plugin_server_tcp_listener =
                    tokio::net::TcpListener::bind("127.0.0.1:0").await?;
                println!(
                    "plugin server started on: {}",
                    plugin_server_tcp_listener.local_addr()?
                );
                let handler = PlatX::from_plugin_root(path.clone())?
                    .start_server(plugin_server_tcp_listener, daemon.addr.clone())
                    .await?;
                handler.await?;
            }
            None => {}
        };

        Ok(())
    }
}
