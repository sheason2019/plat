use clap::Parser;

pub mod cli;

use crate::cli::Cli;

fn main() {
    Cli::parse().work();
}
