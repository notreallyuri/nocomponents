use crate::install::{MOD_BEGIN, MOD_END};
use anyhow::{Context, Result};
use std::{fs, path::Path};

pub fn declare_install_dir(components: &str) -> Result<Option<String>> {
    let rel = components.trim_start_matches("./").trim_matches('/');
    let segments: Vec<&str> = rel.split('/').filter(|s| !s.is_empty()).collect();
    if segments.len() < 2 || segments[0] != "src" {
        return Ok(None);
    }

    for depth in (2..segments.len()).rev() {
        let parent = segments[..depth].join("/");
        let child = segments[depth];
        let file = module_file(&parent);
        declare(&file, child)?;
    }

    let top = segments[1];
    let root = ["src/main.rs", "src/lib.rs"]
        .into_iter()
        .filter_map(|p| fs::read_to_string(p).ok())
        .collect::<Vec<_>>();
    if root.iter().any(|text| declares(text, top)) {
        return Ok(None);
    }
    let root_name = if Path::new("src/lib.rs").exists() && !Path::new("src/main.rs").exists() {
        "src/lib.rs"
    } else {
        "src/main.rs"
    };
    Ok(Some(format!(
        "{root_name} does not declare `mod {top};` — add it there, or the install directory is not part of the crate"
    )))
}

fn module_file(dir: &str) -> String {
    let flat = format!("{dir}.rs");
    if Path::new(&flat).exists() {
        flat
    } else {
        format!("{dir}/mod.rs")
    }
}

fn declare(file: &str, child: &str) -> Result<()> {
    let path = Path::new(file);
    let existing = fs::read_to_string(path).ok();
    let Some(updated) = with_declaration(existing.as_deref().unwrap_or(""), child) else {
        return Ok(());
    };
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("creating {}", parent.display()))?;
    }
    fs::write(path, updated).with_context(|| format!("writing {file}"))?;
    println!(
        "  {:<9} {file} (declares {child})",
        if existing.is_some() {
            "updated"
        } else {
            "wrote"
        }
    );
    Ok(())
}

pub fn declares(text: &str, child: &str) -> bool {
    let wanted = format!("mod {child};");
    text.lines().any(|line| {
        let line = line.trim();
        let line = match line.strip_prefix("pub") {
            Some(rest) => match rest.strip_prefix('(') {
                Some(inner) => inner.split_once(')').map_or("", |(_, after)| after),
                None => rest,
            },
            None => line,
        };
        line.trim() == wanted
    })
}

fn with_declaration(existing: &str, child: &str) -> Option<String> {
    if declares(existing, child) {
        return None;
    }
    let line = format!("pub mod {child};\n");
    if let (Some(start), Some(end)) = (existing.find(MOD_BEGIN), existing.find(MOD_END))
        && end > start
    {
        let mut out = String::from(&existing[..end]);
        if !out.ends_with('\n') {
            out.push('\n');
        }
        out.push_str(&line);
        out.push_str(&existing[end..]);
        return Some(out);
    }
    let mut out = String::from(existing);
    if !out.is_empty() {
        if !out.ends_with('\n') {
            out.push('\n');
        }
        out.push('\n');
    }
    out.push_str(&format!("{MOD_BEGIN}\n{line}{MOD_END}\n"));
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_missing_file_becomes_the_block_declaring_the_module() {
        assert_eq!(
            with_declaration("", "nc").unwrap(),
            "// nocli:begin\npub mod nc;\n// nocli:end\n"
        );
    }

    #[test]
    fn a_file_of_the_projects_own_is_kept_and_the_block_goes_after_it() {
        let out = with_declaration("pub mod ours;\n", "nc").unwrap();
        assert_eq!(
            out,
            "pub mod ours;\n\n// nocli:begin\npub mod nc;\n// nocli:end\n"
        );
    }

    #[test]
    fn an_existing_block_gains_the_line_inside_it() {
        let out = with_declaration(
            "// nocli:begin\npub mod a;\n// nocli:end\npub mod ours;\n",
            "nc",
        )
        .unwrap();
        assert_eq!(
            out,
            "// nocli:begin\npub mod a;\npub mod nc;\n// nocli:end\npub mod ours;\n"
        );
    }

    #[test]
    fn a_declaration_the_project_wrote_is_not_repeated() {
        assert!(with_declaration("pub mod nc;\n", "nc").is_none());
        assert!(with_declaration("mod nc;\n", "nc").is_none());
        assert!(with_declaration("pub(crate) mod nc;\n", "nc").is_none());
        assert!(with_declaration("pub mod nca;\n", "nc").is_some());
    }

    #[test]
    fn a_declaration_run_twice_changes_nothing() {
        let once = with_declaration("", "nc").unwrap();
        assert!(with_declaration(&once, "nc").is_none());
    }
}
