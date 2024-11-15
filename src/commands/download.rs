use color_eyre::eyre::eyre;
use color_eyre::Result;
use dialoguer::{theme::ColorfulTheme, Select};
use owo_colors::OwoColorize;

use crate::util::download_file;
use crate::DIRS;

pub async fn cmd(repo: String) -> Result<()> {
    let (owner, repo) = repo.split_once('/').unwrap();
    let dir = DIRS.cache_dir().join(owner).join(repo);
    tokio::fs::create_dir_all(&dir).await?;

    let octocrab = octocrab::instance();
    let Ok(release) = octocrab.repos(owner, repo).releases().get_latest().await else {
        eprintln!(
            "{} failed to fetch latest release; are you sure a release exists on this repo?",
            "error:".red().bold()
        );
        return Ok(());
    };
    let asset_names: Vec<_> = release.assets.iter().map(|a| a.name.clone()).collect();

    if release.assets.is_empty() {
        eprintln!("{} this repository has no assets", "error:".red().bold());
        return Ok(());
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an asset to download")
        .items(&asset_names)
        .default(0)
        .interact()?;

    let asset = &release.assets[selection];
    log::info!("{} {}", "Downloading".green().bold(), asset.name.bold());

    download_file(
        asset.browser_download_url.clone(),
        dir.join(asset.name.clone()).to_str().unwrap(),
    )
    .await
    .map_err(|e| eyre!("failed to download asset: {}", e))?;

    Ok(())
}
