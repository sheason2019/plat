use clap::Parser;

pub mod cli;
pub mod tarer;
pub mod untarer;

use crate::cli::Cli;

fn main() {
    Cli::parse().work();
}
