use anyhow::{Context, Result};
use std::{collections::BTreeSet, fs, path::Path, process::Command};

pub fn require_cargo_project() -> Result<()> {
    if !Path::new("Cargo.toml").exists() {
        anyhow::bail!("no Cargo.toml here — run this from the root of your project");
    }
    Ok(())
}

/// Asks cargo for `nocomponents` with the union of what is already asked for and `required` — a
/// second `add` must not turn off what the first one switched on. Returns whether it touched
/// `Cargo.toml` at all: when the dependency is there and already asks for everything, it did not,
/// so the project's own formatting is not handed to `cargo add` for nothing.
pub fn add_dependency(required: &BTreeSet<String>, checkout: Option<&Path>) -> Result<bool> {
    let text = fs::read_to_string("Cargo.toml").context("reading Cargo.toml")?;
    let current = features_in(&text)?;

    if let Some(current) = &current
        && required.is_subset(current)
    {
        return Ok(false);
    }

    let wanted: BTreeSet<String> = required
        .union(&current.unwrap_or_default())
        .cloned()
        .collect();
    let list = wanted.iter().cloned().collect::<Vec<_>>().join(",");

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
    Ok(true)
}

/// The features `nocomponents` is asked for in this manifest, or `None` if it is not a dependency.
fn features_in(manifest: &str) -> Result<Option<BTreeSet<String>>> {
    let doc: toml::Table = toml::from_str(manifest).context("parsing Cargo.toml")?;

    Ok(doc
        .get("dependencies")
        .and_then(|d| d.get("nocomponents"))
        .map(|dep| {
            dep.get("features")
                .and_then(|f| f.as_array())
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_str().map(str::to_string))
                        .collect()
                })
                .unwrap_or_default()
        }))
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

#[cfg(test)]
mod tests {
    use super::*;

    fn set(names: &[&str]) -> BTreeSet<String> {
        names.iter().map(|n| n.to_string()).collect()
    }

    #[test]
    fn reads_the_features_of_an_inline_dependency() {
        let manifest = "[dependencies]\nnocomponents = { version = \"0.2\", features = [\"button\", \"icons\"] }\n";
        assert_eq!(
            features_in(manifest).unwrap(),
            Some(set(&["button", "icons"]))
        );
    }

    #[test]
    fn a_dependency_with_no_features_has_an_empty_set() {
        let manifest = "[dependencies]\nnocomponents = \"0.2\"\n";
        assert_eq!(features_in(manifest).unwrap(), Some(set(&[])));
    }

    #[test]
    fn no_dependency_is_none() {
        assert_eq!(
            features_in("[dependencies]\nleptos = \"0.8\"\n").unwrap(),
            None
        );
    }
}
