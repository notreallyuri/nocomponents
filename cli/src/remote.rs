//! Where component sources are read from: the repository over HTTP, or a local checkout.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

const RAW: &str = "https://raw.githubusercontent.com/notreallyuri/nocomponents/main";
const API: &str = "https://api.github.com/repos/notreallyuri/nocomponents/contents";
const AGENT: &str = "nocomponents-cli";

pub enum Source {
    Repository,
    Checkout(PathBuf),
}

impl Source {
    pub fn new(from: Option<PathBuf>) -> Result<Self> {
        match from {
            None => Ok(Source::Repository),
            Some(dir) => {
                if !dir.join("src/components").is_dir() {
                    anyhow::bail!(
                        "{} does not look like a nocomponents checkout — no src/components in it",
                        dir.display()
                    );
                }
                Ok(Source::Checkout(dir))
            }
        }
    }

    pub fn checkout(&self) -> Option<&Path> {
        match self {
            Source::Repository => None,
            Source::Checkout(dir) => Some(dir),
        }
    }

    pub async fn component(&self, name: &str) -> Result<String> {
        match self {
            Source::Repository => get_text(&format!("{RAW}/src/components/{name}.rs"))
                .await
                .with_context(|| format!("no component called `{name}`")),
            Source::Checkout(dir) => read(&dir.join(format!("src/components/{name}.rs")))
                .with_context(|| format!("no component called `{name}`")),
        }
    }

    pub async fn stylesheet(&self, name: &str) -> Result<String> {
        match self {
            Source::Repository => get_text(&format!("{RAW}/docs/styles/{name}")).await,
            Source::Checkout(dir) => read(&dir.join("docs/styles").join(name)),
        }
    }

    pub async fn catalogue(&self) -> Result<Vec<String>> {
        let mut names = match self {
            Source::Repository => {
                #[derive(serde::Deserialize)]
                struct Entry {
                    name: String,
                }
                let entries: Vec<Entry> = get(&format!("{API}/src/components"))
                    .await?
                    .json()
                    .await
                    .context("reading the component listing")?;
                entries.into_iter().map(|e| e.name).collect::<Vec<_>>()
            }
            Source::Checkout(dir) => std::fs::read_dir(dir.join("src/components"))
                .context("reading src/components")?
                .filter_map(|e| e.ok())
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .collect(),
        };

        names.retain(|n| n.ends_with(".rs") && n != "mod.rs");
        let mut names: Vec<String> = names
            .iter()
            .map(|n| n.trim_end_matches(".rs").to_string())
            .collect();
        names.sort();
        Ok(names)
    }
}

fn read(path: &Path) -> Result<String> {
    std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))
}

async fn get(url: &str) -> Result<reqwest::Response> {
    let response = reqwest::Client::new()
        .get(url)
        .header("User-Agent", AGENT)
        .send()
        .await
        .with_context(|| format!("fetching {url}"))?;

    if !response.status().is_success() {
        anyhow::bail!("{url} returned {}", response.status());
    }
    Ok(response)
}

async fn get_text(url: &str) -> Result<String> {
    Ok(get(url).await?.text().await?)
}
