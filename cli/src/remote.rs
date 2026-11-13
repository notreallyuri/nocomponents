//! Where component sources are read from: the repository over HTTP, or a local checkout.
//!
//! The HTTP side reads a **tag**, never a branch. An installed component is compiled by the
//! project it lands in, against the `nocomponents` that project depends on, and it names that
//! crate's internals directly — `nocomponents::primitives::button::ButtonRoot`,
//! `nocomponents::utils::types::StyledRender`. Source and library therefore have to be the same
//! release: read from `main` and a component installed today can name a primitive the published
//! crate does not have yet, which fails at the consumer's build with an error about our source
//! rather than about theirs.
//!
//! **The tag comes from the library, not from this binary.** What has to match is the source and
//! the crate it compiles against; this tool's own version is about its flags, its lockfile format
//! and its rewriting rules, which move on their own schedule. Deriving the tag from
//! `CARGO_PKG_VERSION` would force the two crates to release in lockstep *and* still get the
//! important case wrong — somebody pinned to an older `nocomponents` running a newer CLI would be
//! handed source for a library they are not compiling.
//!
//! So the release is resolved from the project being installed into: `Cargo.lock` if it has
//! resolved one, and the latest published version otherwise. A checkout is exempt entirely,
//! because `--from` already means "I know which tree this is".

use crate::manifest;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tokio::sync::OnceCell;

const REPO: &str = "notreallyuri/nocomponents";
const AGENT: &str = "nocomponents-cli";

/// The library release an install is measured against, resolved at most once per run.
///
/// Lazy rather than resolved up front: `components remove` touches no network at all, and neither
/// does anything run with `--from`.
#[derive(Default)]
pub struct Release {
    tag: OnceCell<String>,
}

impl Release {
    async fn tag(&self) -> Result<&str> {
        self.tag.get_or_try_init(resolve).await.map(String::as_str)
    }

    async fn raw(&self, path: &str) -> Result<String> {
        Ok(format!(
            "https://raw.githubusercontent.com/{REPO}/{}/{path}",
            self.tag().await?
        ))
    }

    async fn contents(&self, path: &str) -> Result<String> {
        Ok(format!(
            "https://api.github.com/repos/{REPO}/contents/{path}?ref={}",
            self.tag().await?
        ))
    }
}

async fn resolve() -> Result<String> {
    // A path dependency is a working tree, and the release tagged with its version is not that
    // tree. Refusing is the same call `remove` makes: better than quietly installing source that
    // will not compile against what the project actually builds.
    if manifest::is_path_dependency()? {
        anyhow::bail!(
            "this project depends on nocomponents by path, so it compiles a checkout rather than \
             a release.\nInstall from that same checkout with `--from <path>` — released source \
             would not necessarily compile against it."
        );
    }

    if let Some(version) = manifest::locked_version()? {
        return Ok(format!("v{version}"));
    }

    // Nothing resolved yet, which is every first `init`: take what `cargo add` is about to take.
    Ok(format!("v{}", latest_published().await?))
}

/// The newest release on crates.io. Only reached before the project has a `Cargo.lock` entry.
async fn latest_published() -> Result<String> {
    #[derive(serde::Deserialize)]
    struct Response {
        #[serde(rename = "crate")]
        krate: Krate,
    }
    #[derive(serde::Deserialize)]
    struct Krate {
        max_stable_version: Option<String>,
    }

    const URL: &str = "https://crates.io/api/v1/crates/nocomponents";
    let response = reqwest::Client::new()
        .get(URL)
        .header("User-Agent", AGENT)
        .send()
        .await
        .context("asking crates.io which nocomponents to install")?;

    if response.status() == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!(
            "nocomponents is not on crates.io yet, so there is no release to install from.\n\
             Point at a checkout with `--from <path>` in the meantime."
        );
    }
    if !response.status().is_success() {
        anyhow::bail!("crates.io returned {} for {URL}", response.status());
    }

    response
        .json::<Response>()
        .await
        .context("reading crates.io' answer")?
        .krate
        .max_stable_version
        .context("nocomponents has no released version yet — use `--from <path>`")
}

pub enum Source {
    Repository(Release),
    Checkout(PathBuf),
}

impl Source {
    pub fn new(from: Option<PathBuf>) -> Result<Self> {
        match from {
            None => Ok(Source::Repository(Release::default())),
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
            Source::Repository(_) => None,
            Source::Checkout(dir) => Some(dir),
        }
    }

    pub async fn component(&self, name: &str) -> Result<String> {
        match self {
            Source::Repository(release) => {
                get_text(&release.raw(&format!("src/components/{name}.rs")).await?)
                    .await
                    .with_context(|| format!("no component called `{name}`"))
            }
            Source::Checkout(dir) => read(&dir.join(format!("src/components/{name}.rs")))
                .with_context(|| format!("no component called `{name}`")),
        }
    }

    pub async fn icon(&self, name: &str) -> Result<String> {
        match self {
            Source::Repository(release) => {
                get_text(&release.raw(&format!("src/icons/{name}.rs")).await?)
                    .await
                    .with_context(|| format!("no icon called `{name}`"))
            }
            Source::Checkout(dir) => read(&dir.join(format!("src/icons/{name}.rs")))
                .with_context(|| format!("no icon called `{name}`")),
        }
    }

    pub async fn component_opt(&self, name: &str) -> Result<Option<String>> {
        self.file_opt(&format!("src/components/{name}.rs")).await
    }

    pub async fn icon_opt(&self, name: &str) -> Result<Option<String>> {
        self.file_opt(&format!("src/icons/{name}.rs")).await
    }

    async fn file_opt(&self, path: &str) -> Result<Option<String>> {
        match self {
            Source::Repository(release) => match get_opt(&release.raw(path).await?).await? {
                Some(response) => Ok(Some(response.text().await?)),
                None => Ok(None),
            },
            Source::Checkout(dir) => read_opt(&dir.join(path)),
        }
    }

    pub async fn stylesheet(&self, name: &str) -> Result<String> {
        match self {
            Source::Repository(release) => {
                get_text(&release.raw(&format!("docs/styles/{name}")).await?).await
            }
            Source::Checkout(dir) => read(&dir.join("docs/styles").join(name)),
        }
    }

    pub async fn catalogue(&self) -> Result<Vec<String>> {
        self.listing("src/components").await
    }

    pub async fn icon_catalogue(&self) -> Result<Vec<String>> {
        self.listing("src/icons").await
    }

    async fn listing(&self, dir: &str) -> Result<Vec<String>> {
        let mut names = match self {
            Source::Repository(release) => {
                #[derive(serde::Deserialize)]
                struct Entry {
                    name: String,
                }
                let entries: Vec<Entry> = get(&release.contents(dir).await?)
                    .await?
                    .json()
                    .await
                    .context("reading the component listing")?;
                entries.into_iter().map(|e| e.name).collect::<Vec<_>>()
            }
            Source::Checkout(root) => std::fs::read_dir(root.join(dir))
                .with_context(|| format!("reading {dir}"))?
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

fn read_opt(path: &Path) -> Result<Option<String>> {
    match std::fs::read_to_string(path) {
        Ok(text) => Ok(Some(text)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e).with_context(|| format!("reading {}", path.display())),
    }
}

async fn get(url: &str) -> Result<reqwest::Response> {
    get_opt(url).await?.with_context(|| {
        format!(
            "{url} returned 404 — either that release is not tagged in the repository, or the \
             file is not in it.\nWork against a tree instead with `--from <path>`."
        )
    })
}

async fn get_opt(url: &str) -> Result<Option<reqwest::Response>> {
    let response = reqwest::Client::new()
        .get(url)
        .header("User-Agent", AGENT)
        .send()
        .await
        .with_context(|| format!("fetching {url}"))?;

    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }
    if !response.status().is_success() {
        anyhow::bail!("{url} returned {}", response.status());
    }
    Ok(Some(response))
}

async fn get_text(url: &str) -> Result<String> {
    Ok(get(url).await?.text().await?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn a_file_missing_from_a_checkout_is_none_not_an_error() {
        let dir = std::env::temp_dir().join(format!("nocli-remote-{}", std::process::id()));
        std::fs::create_dir_all(dir.join("src/components")).unwrap();
        std::fs::write(dir.join("src/components/button.rs"), "fn button() {}").unwrap();
        let source = Source::new(Some(dir.clone())).unwrap();

        let here = source.component_opt("button").await.unwrap();
        let gone = source.component_opt("nothing").await.unwrap();
        let icon = source.icon_opt("nothing").await.unwrap();
        std::fs::remove_dir_all(&dir).unwrap();

        assert_eq!(here.as_deref(), Some("fn button() {}"));
        assert_eq!(gone, None);
        assert_eq!(icon, None);
    }
}
