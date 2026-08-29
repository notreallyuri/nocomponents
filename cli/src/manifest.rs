use anyhow::{Context, Result};
use std::{collections::BTreeSet, fs, path::Path, process::Command};

pub fn require_cargo_project() -> Result<()> {
    if !Path::new("Cargo.toml").exists() {
        anyhow::bail!("no Cargo.toml here — run this from the root of your project");
    }
    Ok(())
}

pub fn current_features() -> Result<BTreeSet<String>> {
    let text = fs::read_to_string("Cargo.toml").context("reading Cargo.toml")?;
    let doc: toml::Table = toml::from_str(&text).context("parsing Cargo.toml")?;

    Ok(doc
        .get("dependencies")
        .and_then(|d| d.get("nocomponents"))
        .and_then(|n| n.get("features"))
        .and_then(|f| f.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default())
}

/// Adds `nocomponents` with the union of what is already asked for and what was just installed —
/// a second `add` must not turn off what the first one switched on.
pub fn add_dependency(features: &BTreeSet<String>, checkout: Option<&Path>) -> Result<()> {
    let list = features.iter().cloned().collect::<Vec<_>>().join(",");

    let mut command = Command::new("cargo");
    command.args(["add", "nocomponents", "--features", &list]);
    if let Some(dir) = checkout {
        command.arg("--path").arg(dir);
    }

    let status = command
        .status()
        .context("running `cargo add` — is cargo on PATH?")?;

    if !status.success() {
        anyhow::bail!("`cargo add nocomponents --features {list}` failed");
    }
    Ok(())
}

/// The exact `nocomponents` this project compiles against, read out of `Cargo.lock`.
///
/// The lock rather than the manifest, because the manifest holds a *requirement* — `"0.1"` — and
/// there is no tag by that name. The lock holds the one version cargo actually resolved, which is
/// the one the installed source has to compile against.
///
/// Walked upward, since a workspace member has no lock of its own.
///
/// `None` for a project that has not resolved it yet, which is every first `init`.
pub fn locked_version() -> Result<Option<String>> {
    let mut dir = std::env::current_dir().context("reading the current directory")?;

    loop {
        let lock = dir.join("Cargo.lock");
        if lock.exists() {
            let doc: toml::Table = toml::from_str(&fs::read_to_string(&lock)?)
                .with_context(|| format!("parsing {}", lock.display()))?;

            return Ok(doc
                .get("package")
                .and_then(|p| p.as_array())
                .and_then(|packages| {
                    packages.iter().find(|entry| {
                        entry.get("name").and_then(|n| n.as_str()) == Some("nocomponents")
                    })
                })
                .and_then(|entry| entry.get("version"))
                .and_then(|v| v.as_str())
                .map(str::to_string));
        }
        if !dir.pop() {
            return Ok(None);
        }
    }
}

/// Whether this project's `nocomponents` is a path dependency — a checkout, in other words.
///
/// Worth asking before reading a version off it: the checkout is whatever the tree happens to be,
/// and the release tagged with the same number is not it. Installing released source into a
/// project that compiles a working tree is the exact mismatch the tag exists to prevent, so the
/// caller refuses rather than guesses.
pub fn is_path_dependency() -> Result<bool> {
    let Ok(text) = fs::read_to_string("Cargo.toml") else {
        return Ok(false);
    };
    let doc: toml::Table = toml::from_str(&text).context("parsing Cargo.toml")?;

    Ok(doc
        .get("dependencies")
        .and_then(|d| d.get("nocomponents"))
        .and_then(|n| n.get("path"))
        .is_some())
}
