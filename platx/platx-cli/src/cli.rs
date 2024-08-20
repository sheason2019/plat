use clap::{Parser, Subcommand};

use platx_core::{
    bundler::{tarer::Tarer, untarer::Untarer},
    platx::PlatX,
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
                let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
                println!("plugin server started at {}", listener.local_addr()?);
                let handler = PlatX::from_path(path.clone())?
                    .start_server(listener)
                    .await?;
                handler.await?;
            }
            None => {}
        };

        Ok(())
    }
}
