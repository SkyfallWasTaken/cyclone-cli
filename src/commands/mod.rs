use std::path::PathBuf;

use clap::{Parser, Subcommand};

pub mod mount;
pub mod new;

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