use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

pub const CONFIG_FILE: &str = "nocomponents.toml";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub style: StyleConfig,
    pub paths: PathConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StyleConfig {
    pub css: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PathConfig {
    pub components: String,
    #[serde(default = "default_icons")]
    pub icons: String,
}

fn default_icons() -> String {
    "src/icons/nc".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Config {
            style: StyleConfig {
                css: "styles/globals.css".to_string(),
            },
            paths: PathConfig {
                components: "src/components/nc".to_string(),
                icons: default_icons(),
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = Path::new(CONFIG_FILE);
        if !path.exists() {
            anyhow::bail!("no {CONFIG_FILE} here — run `cargo nocli init` first");
        }
        let text = fs::read_to_string(path).with_context(|| format!("reading {CONFIG_FILE}"))?;
        toml::from_str(&text).with_context(|| format!("parsing {CONFIG_FILE}"))
    }

    pub fn save(&self) -> Result<()> {
        let text = toml::to_string_pretty(self).context("serialising the config")?;
        fs::write(CONFIG_FILE, text).with_context(|| format!("writing {CONFIG_FILE}"))
    }

    /// The install directory as a Rust path: `src/components/nc` is `crate::components::nc`.
    pub fn module_path(&self) -> Result<String> {
        module_path_of("paths.components", &self.paths.components)
    }

    /// The icon directory as a Rust path: `src/icons/nc` is `crate::icons::nc`.
    pub fn icons_module_path(&self) -> Result<String> {
        module_path_of("paths.icons", &self.paths.icons)
    }
}

fn module_path_of(key: &str, dir: &str) -> Result<String> {
    let rel = dir.trim_start_matches("./").trim_matches('/');

    let rest = rel
        .strip_prefix("src/")
        .filter(|rest| !rest.is_empty())
        .ok_or_else(|| anyhow::anyhow!("{key} ({rel}) has to name a directory under `src/`"))?;

    let segments: Vec<&str> = rest.split('/').filter(|s| !s.is_empty()).collect();
    Ok(format!("crate::{}", segments.join("::")))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(components: &str) -> Config {
        Config {
            paths: PathConfig {
                components: components.to_string(),
                icons: default_icons(),
            },
            ..Config::default()
        }
    }

    #[test]
    fn derives_the_module_path_from_the_install_directory() {
        assert_eq!(
            at("src/components/nc").module_path().unwrap(),
            "crate::components::nc"
        );
        assert_eq!(at("./src/ui/").module_path().unwrap(), "crate::ui");
        assert_eq!(at("src/a/b/c").module_path().unwrap(), "crate::a::b::c");
    }

    #[test]
    fn a_config_from_before_icons_were_installed_gets_the_default() {
        let config: Config =
            toml::from_str("[style]\ncss = \"a.css\"\n\n[paths]\ncomponents = \"src/ui\"\n")
                .unwrap();
        assert_eq!(config.paths.icons, "src/icons/nc");
        assert_eq!(config.icons_module_path().unwrap(), "crate::icons::nc");
    }

    #[test]
    fn refuses_a_directory_with_no_module_path() {
        assert!(at("components/nc").module_path().is_err());
        assert!(at("src").module_path().is_err());
    }
}
