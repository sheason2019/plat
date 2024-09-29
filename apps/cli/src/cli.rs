use clap::Parser;

use crate::commands::{self, Commands};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<commands::Commands>,
}

impl Cli {
    pub async fn work(&self) -> anyhow::Result<()> {
        match &self.command {
            Some(Commands::Daemon(daemon_args)) => daemon_args.work().await,
            Some(Commands::Plugin(plugin_args)) => plugin_args.work().await,
            None => {
                Ok(())
            }
        }
    }
}
