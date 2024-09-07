use bundler::{tarer::Tarer, untarer::Untarer};
use clap::{Parser, Subcommand};
use plugin::PluginService;

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
                // 启动 Plugin Daemon
                println!("connecting to plugin daemon: {}", daemon_address);

                let port = match port {
                    Some(val) => *val,
                    None => 0,
                };
                // 启动 Plugin
                let service = PluginService::new(
                    path.clone(),
                    daemon_address.clone(),
                    regist_address.clone(),
                    port,
                )
                .await?;
                service.wait().await;
            }
            None => {}
        };

        Ok(())
    }
}
