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
    resolved: OnceCell<Resolved>,
    client: reqwest::Client,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Origin {
    Lock,
    Published,
}

struct Resolved {
    tag: String,
    origin: Origin,
}

pub fn describe(tag: &str, origin: Origin) -> String {
    match origin {
        Origin::Lock => format!("nocomponents {tag} (from Cargo.lock)"),
        Origin::Published => format!("nocomponents {tag} (latest on crates.io)"),
    }
}

impl Release {
    async fn resolved(&self) -> Result<&Resolved> {
        self.resolved
            .get_or_try_init(|| resolve(&self.client))
            .await
    }

    async fn tag(&self) -> Result<&str> {
        Ok(&self.resolved().await?.tag)
    }

    pub async fn describe(&self) -> Result<String> {
        let resolved = self.resolved().await?;
        Ok(describe(&resolved.tag, resolved.origin))
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

async fn resolve(client: &reqwest::Client) -> Result<Resolved> {
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
        return Ok(Resolved {
            tag: format!("v{version}"),
            origin: Origin::Lock,
        });
    }

    // Nothing resolved yet, which is every first `init`: take what `cargo add` is about to take.
    Ok(Resolved {
        tag: format!("v{}", latest_published(client).await?),
        origin: Origin::Published,
    })
}

fn parse_version(text: &str) -> Option<(u64, u64, u64, bool)> {
    let text = text.split_once('+').map_or(text, |(rest, _)| rest);
    let (core, pre) = match text.split_once('-') {
        Some((core, _)) => (core, true),
        None => (text, false),
    };
    let mut parts = core.split('.').map(|p| p.parse::<u64>().ok());
    let version = (parts.next()??, parts.next()??, parts.next()??, !pre);
    parts.next().is_none().then_some(version)
}

pub fn is_newer(candidate: &str, current: &str) -> bool {
    match (parse_version(candidate), parse_version(current)) {
        (Some(a), Some(b)) => a > b,
        _ => false,
    }
}

pub fn upgrade_requirement(version: &str) -> Option<String> {
    let (major, minor, _, _) = parse_version(version)?;
    Some(if major == 0 {
        format!("0.{minor}")
    } else {
        major.to_string()
    })
}

pub fn upgrade_hint(latest: &str, locked: &str) -> Option<String> {
    let requirement = upgrade_requirement(latest)?;
    let command = if upgrade_requirement(locked).as_deref() == Some(requirement.as_str()) {
        "cargo update -p nocomponents && cargo nocli components update".to_string()
    } else {
        format!("cargo add nocomponents@{requirement} && cargo nocli components update")
    };
    Some(format!(
        "nocomponents {latest} is available; this project is on {locked}. To move to it: {command}"
    ))
}

/// The newest release on crates.io. Only reached before the project has a `Cargo.lock` entry.
async fn latest_published(client: &reqwest::Client) -> Result<String> {
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
    let response = client
        .get(URL)
        .header("User-Agent", AGENT)
        .timeout(std::time::Duration::from_secs(5))
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
        .context("reading crates.io's answer")?
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

    pub async fn describe(&self, verb: &str) -> Result<String> {
        match self {
            Source::Repository(release) => Ok(format!("{verb} {}", release.describe().await?)),
            Source::Checkout(dir) => Ok(format!("{verb} the checkout at {}", dir.display())),
        }
    }

    pub async fn announce(&self, verb: &str) -> Result<()> {
        println!("{}", self.describe(verb).await?);
        Ok(())
    }

    pub fn is_remote(&self) -> bool {
        matches!(self, Source::Repository(_))
    }

    pub async fn newer_release(&self) -> Option<String> {
        let Source::Repository(release) = self else {
            return None;
        };
        let resolved = release.resolved().await.ok()?;
        if resolved.origin != Origin::Lock {
            return None;
        }
        let locked = resolved.tag.trim_start_matches('v');
        let latest = latest_published(&release.client).await.ok()?;
        if is_newer(&latest, locked) {
            upgrade_hint(&latest, locked)
        } else {
            None
        }
    }

    pub async fn component(&self, name: &str) -> Result<String> {
        match self {
            Source::Repository(release) => get_text(
                &release.client,
                &release.raw(&format!("src/components/{name}.rs")).await?,
            )
            .await
            .with_context(|| format!("no component called `{name}`")),
            Source::Checkout(dir) => read(&dir.join(format!("src/components/{name}.rs")))
                .with_context(|| format!("no component called `{name}`")),
        }
    }

    pub async fn icon(&self, name: &str) -> Result<String> {
        match self {
            Source::Repository(release) => get_text(
                &release.client,
                &release.raw(&format!("src/icons/{name}.rs")).await?,
            )
            .await
            .with_context(|| format!("no icon called `{name}`")),
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
            Source::Repository(release) => {
                match get_opt(&release.client, &release.raw(path).await?).await? {
                    Some(response) => Ok(Some(response.text().await?)),
                    None => Ok(None),
                }
            }
            Source::Checkout(dir) => read_opt(&dir.join(path)),
        }
    }

    pub async fn stylesheet(&self, name: &str) -> Result<String> {
        match self {
            Source::Repository(release) => {
                get_text(
                    &release.client,
                    &release.raw(&format!("docs/styles/{name}")).await?,
                )
                .await
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
                let entries: Vec<Entry> = get(&release.client, &release.contents(dir).await?)
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

async fn get(client: &reqwest::Client, url: &str) -> Result<reqwest::Response> {
    get_opt(client, url).await?.with_context(|| {
        format!(
            "{url} returned 404 — either that release is not tagged in the repository, or the \
             file is not in it.\nWork against a tree instead with `--from <path>`."
        )
    })
}

async fn get_opt(client: &reqwest::Client, url: &str) -> Result<Option<reqwest::Response>> {
    let response = client
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

async fn get_text(client: &reqwest::Client, url: &str) -> Result<String> {
    Ok(get(client, url).await?.text().await?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn versions_compare_numerically() {
        assert!(is_newer("0.6.1", "0.3.0"));
        assert!(is_newer("0.10.0", "0.9.9"));
        assert!(is_newer("1.0.0", "0.99.99"));
        assert!(!is_newer("0.3.0", "0.3.0"));
        assert!(!is_newer("0.3.0", "0.6.1"));
    }

    #[test]
    fn a_pre_release_is_older_than_its_release() {
        assert!(is_newer("0.7.0", "0.7.0-rc.1"));
        assert!(!is_newer("0.7.0-rc.1", "0.7.0"));
        assert!(is_newer("0.7.0-rc.1", "0.6.9"));
    }

    #[test]
    fn an_unparseable_version_is_never_newer() {
        assert!(!is_newer("latest", "0.1.0"));
        assert!(!is_newer("0.2", "0.1.0"));
        assert!(!is_newer("0.2.0", "x.y.z"));
        assert!(!is_newer("0.2.0.1", "0.1.0"));
    }

    #[test]
    fn the_upgrade_requirement_follows_semver_for_zero_and_after() {
        assert_eq!(upgrade_requirement("0.6.1").as_deref(), Some("0.6"));
        assert_eq!(upgrade_requirement("2.3.4").as_deref(), Some("2"));
        assert_eq!(upgrade_requirement("junk"), None);
    }

    #[test]
    fn the_hint_names_both_releases_and_the_command() {
        assert_eq!(
            upgrade_hint("0.6.1", "0.3.0").unwrap(),
            "nocomponents 0.6.1 is available; this project is on 0.3.0. To move to it: cargo add \
             nocomponents@0.6 && cargo nocli components update"
        );
    }

    #[test]
    fn the_hint_is_a_plain_update_when_the_requirement_already_allows_it() {
        assert_eq!(
            upgrade_hint("0.6.1", "0.6.0").unwrap(),
            "nocomponents 0.6.1 is available; this project is on 0.6.0. To move to it: cargo \
             update -p nocomponents && cargo nocli components update"
        );
        assert!(
            upgrade_hint("1.4.0", "1.2.0")
                .unwrap()
                .contains("cargo update -p")
        );
    }

    #[test]
    fn build_metadata_is_ignored_and_a_pre_release_is_not() {
        assert!(!is_newer("0.7.0+build.5", "0.7.0"));
        assert!(!is_newer("0.7.0", "0.7.0+build.5"));
        assert!(is_newer("0.7.1+x", "0.7.0"));
        assert!(is_newer("0.7.0+x", "0.7.0-rc.1+y"));
        assert!(!is_newer("0.7.0-rc.1+x", "0.7.0"));
    }

    #[test]
    fn the_release_description_names_where_the_tag_came_from() {
        assert_eq!(
            describe("v0.3.0", Origin::Lock),
            "nocomponents v0.3.0 (from Cargo.lock)"
        );
        assert_eq!(
            describe("v0.6.1", Origin::Published),
            "nocomponents v0.6.1 (latest on crates.io)"
        );
    }

    #[tokio::test]
    async fn a_checkout_is_described_by_its_path() {
        let dir = std::env::temp_dir().join(format!("nocli-describe-{}", std::process::id()));
        std::fs::create_dir_all(dir.join("src/components")).unwrap();
        let source = Source::new(Some(dir.clone())).unwrap();
        let line = source.describe("installing from").await.unwrap();
        let hint = source.newer_release().await;
        std::fs::remove_dir_all(&dir).unwrap();

        assert_eq!(
            line,
            format!("installing from the checkout at {}", dir.display())
        );
        assert_eq!(hint, None);
    }

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
