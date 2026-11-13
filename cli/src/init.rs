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

    index_html(&config)?;

    // `icons` is only for a release from before the glyphs were installed as source.
    let mut features: std::collections::BTreeSet<String> = ["primitives", "theme"]
        .into_iter()
        .map(String::from)
        .collect();
    if !crate::install::ships_icons(source).await? {
        features.insert("icons".to_string());
    }
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

fn index_html(config: &Config) -> Result<()> {
    let path = Path::new("index.html");
    if path.exists() {
        println!("index.html is already here, leaving it alone");
        return Ok(());
    }
    fs::write(path, page(&config.style.css, &package_name())).context("writing index.html")?;
    println!("wrote index.html");
    Ok(())
}

fn package_name() -> String {
    fs::read_to_string("Cargo.toml")
        .ok()
        .and_then(|text| toml::from_str::<toml::Table>(&text).ok())
        .and_then(|doc| {
            doc.get("package")?
                .get("name")?
                .as_str()
                .map(str::to_string)
        })
        .unwrap_or_else(|| "app".to_string())
}

fn page(css: &str, title: &str) -> String {
    format!(
        "<!doctype html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"utf-8\" />\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />\n    <link data-trunk rel=\"rust\" />\n    <link data-trunk rel=\"tailwind-css\" href=\"{css}\" />\n    <title>{title}</title>\n  </head>\n  <body></body>\n</html>\n"
    )
}

/// The docs stylesheet, minus its `@source` (a consumer's components are in their own `src`,
/// already scanned) and its font import — and with the import gone, the font token falls back to
/// the system stack rather than naming a face nothing loads.
fn project_stylesheet(source: &str) -> String {
    source
        .lines()
        .filter(|line| {
            let line = line.trim();
            !line.starts_with("@source") && !line.starts_with("@import url(")
        })
        .map(|line| {
            if line.trim_start().starts_with("--font-sans:") {
                let indent = &line[..line.len() - line.trim_start().len()];
                format!("{indent}--font-sans: ui-sans-serif, system-ui, sans-serif;")
            } else {
                line.to_string()
            }
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
    fn the_font_token_falls_back_to_the_system_stack() {
        let out = super::project_stylesheet(
            "@theme inline {\n  --font-sans: \"Figtree\", sans-serif;\n}",
        );
        assert!(out.contains("  --font-sans: ui-sans-serif, system-ui, sans-serif;"));
        assert!(!out.contains("Figtree"));
    }

    #[test]
    fn the_page_links_the_stylesheet_for_trunk() {
        let html = super::page("styles/globals.css", "demo");
        assert!(html.contains("<link data-trunk rel=\"rust\" />"));
        assert!(html.contains("rel=\"tailwind-css\" href=\"styles/globals.css\""));
        assert!(html.contains("<title>demo</title>"));
    }

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
