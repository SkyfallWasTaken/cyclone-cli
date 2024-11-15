use std::sync::LazyLock;

use clap::Parser;
use color_eyre::Result;
use directories::ProjectDirs;
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};

mod commands;
mod config;
mod smart_select;
pub mod util;
use commands::{Cli, Command};
use config::Config;

pub static DIRS: LazyLock<ProjectDirs> =
    LazyLock::new(|| ProjectDirs::from("dev", "skyfall", "Cyclone").unwrap());
pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    Figment::new()
        .merge(Toml::file(DIRS.config_dir().join("config.toml")))
        .merge(Env::prefixed("CYCLONE_"))
        .extract()
        .unwrap()
});

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();

    let mut crab = octocrab::Octocrab::builder();
    if let Some(token) = &CONFIG.github_token {
        crab = crab.personal_token(token.clone());
    }
    octocrab::initialise(crab.build()?);

    let cli = Cli::parse();
    match cli.command {
        Command::Download { repo } => commands::download::cmd(repo).await?,
    }

    Ok(())
}
