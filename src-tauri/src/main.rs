// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod daemon;

#[tokio::main]
async fn main() {
    tokio::spawn(daemon::start());
    plat_lib::run();
}
