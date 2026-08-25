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
