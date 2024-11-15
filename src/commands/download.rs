use color_eyre::eyre::eyre;
use color_eyre::Result;
use dialoguer::{theme::ColorfulTheme, Select};
use owo_colors::OwoColorize;

use crate::smart_select::get_compatible_file_index;
use crate::util::download_file;
use crate::DIRS;

pub async fn cmd(repo: String) -> Result<()> {
    let (owner, repo_name) = repo.split_once('/').unwrap();

    let octocrab = octocrab::instance();
    let Ok(repo) = octocrab.repos(owner, repo_name).get().await else {
        eprintln!(
            "{} failed to fetch repo; are you sure it exists?",
            "error:".red().bold()
        );
        return Ok(());
    };
    let Ok(release) = octocrab
        .repos(owner, repo_name)
        .releases()
        .get_latest()
        .await
    else {
        eprintln!(
            "{} failed to fetch latest release; are you sure a release exists on this repo?",
            "error:".red().bold()
        );
        return Ok(());
    };
    let asset_names: Vec<_> = release.assets.iter().map(|a| a.name.clone()).collect();

    if release.assets.is_empty() {
        eprintln!("{} this release has no assets", "error:".red().bold());
        return Ok(());
    }

    let default = get_compatible_file_index(&asset_names).unwrap_or(0);
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an asset to download")
        .items(&asset_names)
        .default(default)
        .interact()?;

    let asset = &release.assets[selection];
    log::info!("{} {}", "Downloading".green().bold(), asset.name.bold());

    let dir = DIRS.cache_dir().join(repo.id.to_string());
    tokio::fs::create_dir_all(&dir).await?;

    download_file(
        asset.browser_download_url.clone(),
        dir.join(asset.name.clone()).to_str().unwrap(),
    )
    .await
    .map_err(|e| eyre!("failed to download asset: {}", e))?;

    Ok(())
}
