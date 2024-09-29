use clap::Subcommand;
use daemon::DaemonArgs;
use plugin::PluginArgs;

mod daemon;
mod plugin;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Plugin(PluginArgs),
    Daemon(DaemonArgs),
}
