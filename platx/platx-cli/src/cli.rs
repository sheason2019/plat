use clap::{Parser, Subcommand};

use platx_core::bundler::{tarer::Tarer, untarer::Untarer};

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
}

impl Cli {
    pub fn work(&self) {
        match &self.command {
            Some(Commands::Tar { path }) => Tarer::new(path.clone()).tar().unwrap(),
            Some(Commands::Untar { path, output }) => {
                Untarer::new(path.clone()).untar(output.clone()).unwrap()
            }
            None => {}
        }
    }
}
