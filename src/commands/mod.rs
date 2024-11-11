use clap::{Parser, Subcommand};

pub mod download;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Download a GitHub Release
    Download {
        /// owner/repo
        repo: String,
    },
}