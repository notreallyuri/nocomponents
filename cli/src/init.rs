//! `cargo nocli init` — the config, the stylesheet, and the dependency.

use crate::{config::Config, manifest, remote::Source};
use anyhow::{Context, Result};
use std::{fs, path::Path};

pub async fn run(source: &Source) -> Result<()> {
    manifest::require_cargo_project()?;

    let config = if Path::new(crate::config::CONFIG_FILE).exists() {
        println!("{} is already here, keeping it", crate::config::CONFIG_FILE);
        Config::load()?
    } else {
        let config = Config::default();
        config.save()?;
        println!("wrote {}", crate::config::CONFIG_FILE);
        config
    };

    install_styles(source, &config).await?;
    trunk_tailwind()?;

    let mut features = manifest::current_features()?;
    features.insert("primitives".into());
    features.insert("icons".into());
    features.insert("theme".into());
    manifest::add_dependency(&features, source.checkout())?;

    println!("\nNow add components: cargo nocli components add button dialog");
    Ok(())
}

async fn install_styles(source: &Source, config: &Config) -> Result<()> {
    let css = Path::new(&config.style.css);
    if let Some(parent) = css.parent() {
        fs::create_dir_all(parent).with_context(|| format!("creating {}", parent.display()))?;
    }

    if css.exists() {
        println!("{} is already here, leaving it alone", css.display());
    } else {
        let sheet = source.stylesheet("globals.css").await?;
        fs::write(css, project_stylesheet(&sheet))
            .with_context(|| format!("writing {}", css.display()))?;
        println!("wrote {}", css.display());
    }

    let animate = css.with_file_name("animate.css");
    if animate.exists() {
        println!("{} is already here, leaving it alone", animate.display());
    } else {
        fs::write(&animate, source.stylesheet("animate.css").await?)
            .with_context(|| format!("writing {}", animate.display()))?;
        println!("wrote {}", animate.display());
    }
    Ok(())
}

/// The docs stylesheet, minus its `@source` (a consumer's components are in their own `src`,
/// already scanned) and its font import.
fn project_stylesheet(source: &str) -> String {
    source
        .lines()
        .filter(|line| {
            let line = line.trim();
            !line.starts_with("@source") && !line.starts_with("@import url(")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Trunk needs telling which Tailwind to fetch, in `Trunk.toml`.
fn trunk_tailwind() -> Result<()> {
    let path = Path::new("Trunk.toml");
    let existing = fs::read_to_string(path).unwrap_or_default();

    if existing.contains("tailwindcss") {
        println!("Trunk.toml already names a tailwindcss, leaving it alone");
        return Ok(());
    }

    let updated = if existing.contains("[tools]") {
        existing.replace("[tools]", "[tools]\ntailwindcss = \"4.3.0\"")
    } else {
        let mut out = existing;
        if !out.is_empty() && !out.ends_with('\n') {
            out.push('\n');
        }
        out.push_str("\n[tools]\ntailwindcss = \"4.3.0\"\n");
        out
    };

    fs::write(path, updated).context("writing Trunk.toml")?;
    println!("added tailwindcss to Trunk.toml");
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn drops_the_docs_only_lines_and_keeps_the_rest() {
        let source = "@import url(\"https://fonts.googleapis.com/x\");\n@import \"tailwindcss\";\n@import \"./animate.css\";\n@source \"../../src\";\n\n@custom-variant dark (&:is(.dark *));\n";
        let out = super::project_stylesheet(source);
        assert!(!out.contains("fonts.googleapis"));
        assert!(!out.contains("@source"));
        assert!(out.contains("@import \"tailwindcss\";"));
        assert!(out.contains("@import \"./animate.css\";"));
        assert!(out.contains("@custom-variant dark"));
    }
}
