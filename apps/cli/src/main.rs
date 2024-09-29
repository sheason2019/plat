use clap::Parser;

pub mod cli;
mod commands;

use crate::cli::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Cli::parse().work().await?;

    Ok(())
}
