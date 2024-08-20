use clap::Parser;

pub mod cli;

use crate::cli::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Cli::parse().work().await?;

    Ok(())
}
