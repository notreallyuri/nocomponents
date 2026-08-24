//! The per-element features have to match what the sources actually import.
//!
//! `accordion = ["collapsible", "roving_focus"]` is not a preference, it is a fact about
//! `primitives/accordion.rs`, and the failure when it goes stale is quiet: everything still builds
//! with `--features full`, and only a consumer who asked for that one element finds out that a
//! module it names is not there. Nobody would think to check, so this checks.
//!
//! On a mismatch the test prints the table it wanted, ready to paste into `Cargo.toml`.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

/// Expands a use-tree body to one full path per leaf: `a::{b, c::{d, e}}` gives `a::b`, `a::c::d`,
/// `a::c::e`. The imports are nested trees, so `crate::components::x` is very often not a
/// substring of the source that depends on it.
fn flatten(body: &str, prefix: &str, out: &mut Vec<String>) {
    let mut depth = 0usize;
    let mut current = String::new();

    for ch in body.chars() {
        match ch {
            '{' => {
                depth += 1;
                current.push(ch);
            }
            '}' => {
                depth = depth.saturating_sub(1);
                current.push(ch);
            }
            ',' if depth == 0 => {
                expand(&current, prefix, out);
                current.clear();
            }
            _ => current.push(ch),
        }
    }
    expand(&current, prefix, out);
}

fn expand(item: &str, prefix: &str, out: &mut Vec<String>) {
    let item = item.trim();
    if item.is_empty() {
        return;
    }
    match item.split_once('{') {
        Some((head, rest)) => {
            let inner = rest.strip_suffix('}').unwrap_or(rest);
            flatten(inner, &format!("{prefix}{}", head.trim()), out);
        }
        None => out.push(format!(
            "{prefix}{}",
            item.split_whitespace().next().unwrap_or("")
        )),
    }
}

/// Every `crate::…` path a file names, from its imports and from its body.
fn paths(source: &str) -> Vec<String> {
    let stripped: String = source
        .lines()
        .map(|line| match line.find("//") {
            Some(i) => &line[..i],
            None => line,
        })
        .collect::<Vec<_>>()
        .join("\n");

    let mut out = Vec::new();
    let bytes = stripped.as_bytes();
    let mut i = 0;

    for (start, line) in line_starts(&stripped) {
        let trimmed = line.trim_start();
        let keyword = if trimmed.starts_with("pub use ") {
            8
        } else if trimmed.starts_with("use ") {
            4
        } else {
            continue;
        };
        let body_start = start + (line.len() - trimmed.len()) + keyword;
        if body_start < i {
            continue; // inside a statement already consumed
        }
        let mut depth = 0usize;
        let mut j = body_start;
        while j < bytes.len() {
            match bytes[j] {
                b'{' => depth += 1,
                b'}' => depth = depth.saturating_sub(1),
                b';' if depth == 0 => break,
                _ => {}
            }
            j += 1;
        }
        flatten(&stripped[body_start..j.min(stripped.len())], "", &mut out);
        i = j;
    }

    // Paths written out in expression position, which no import mentions.
    for prefix in [
        "crate::components::",
        "crate::primitives::",
        "crate::utils::",
    ] {
        let mut rest = stripped.as_str();
        while let Some(k) = rest.find(prefix) {
            let tail = &rest[k + prefix.len()..];
            let name: String = tail
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect();
            out.push(format!("{prefix}{name}"));
            rest = &rest[k + prefix.len()..];
        }
    }
    out
}

fn line_starts(source: &str) -> Vec<(usize, &str)> {
    let mut out = Vec::new();
    let mut at = 0;
    for line in source.split('\n') {
        out.push((at, line));
        at += line.len() + 1;
    }
    out
}

fn elements_in(dir: &Path) -> BTreeMap<String, Vec<String>> {
    let mut out = BTreeMap::new();
    for entry in fs::read_dir(dir).expect("reading the layer") {
        let entry = entry.expect("a directory entry");
        let name = entry.file_name().to_string_lossy().into_owned();
        let Some(stem) = name.strip_suffix(".rs") else {
            continue;
        };
        if stem == "mod" {
            continue;
        }
        let source = fs::read_to_string(entry.path()).expect("reading a source file");
        out.insert(stem.to_string(), paths(&source));
    }
    out
}

/// What `Cargo.toml` ought to say, worked out from the sources.
fn expected() -> BTreeMap<String, BTreeSet<String>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let primitives = elements_in(&root.join("src/primitives"));
    let components = elements_in(&root.join("src/components"));

    let names: BTreeSet<String> = primitives
        .keys()
        .chain(components.keys())
        .cloned()
        .collect();

    let mut table = BTreeMap::new();
    for name in &names {
        let mut deps = BTreeSet::new();
        for layer in [primitives.get(name), components.get(name)] {
            for path in layer.into_iter().flatten() {
                for prefix in ["crate::components::", "crate::primitives::"] {
                    if let Some(rest) = path.strip_prefix(prefix)
                        && let Some(dep) = rest.split("::").next()
                        && !dep.is_empty()
                    {
                        deps.insert(dep.to_string());
                    }
                }
                // `utils::highlight` is behind its own feature, so naming it is a feature edge too.
                if path.starts_with("crate::utils::highlight") {
                    deps.insert("highlight".to_string());
                }
            }
        }
        deps.remove(name);
        table.insert(name.clone(), deps);
    }
    table
}

/// What `Cargo.toml` does say, for the element features only.
fn declared() -> (BTreeMap<String, BTreeSet<String>>, BTreeSet<String>) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let text = fs::read_to_string(root.join("Cargo.toml")).expect("reading Cargo.toml");
    let doc: toml::Table = toml::from_str(&text).expect("parsing Cargo.toml");

    let features = doc["features"].as_table().expect("a [features] table");
    let all: BTreeSet<String> = features["all-elements"]
        .as_array()
        .expect("all-elements is a list")
        .iter()
        .map(|v| v.as_str().expect("a feature name").to_string())
        .collect();

    let table = all
        .iter()
        .map(|name| {
            let deps = features
                .get(name)
                .and_then(|v| v.as_array())
                .map(|a| {
                    a.iter()
                        .map(|v| v.as_str().expect("a feature name").to_string())
                        .collect()
                })
                .unwrap_or_default();
            (name.clone(), deps)
        })
        .collect();

    (table, all)
}

fn render(table: &BTreeMap<String, BTreeSet<String>>) -> String {
    let mut out = String::from("all-elements = [\n");
    for name in table.keys() {
        out.push_str(&format!("    \"{name}\",\n"));
    }
    out.push_str("]\n");
    for (name, deps) in table {
        let list = deps
            .iter()
            .map(|d| format!("\"{d}\""))
            .collect::<Vec<_>>()
            .join(", ");
        out.push_str(&format!("{name} = [{list}]\n"));
    }
    out
}

#[test]
fn the_feature_table_matches_what_the_sources_import() {
    let expected = expected();
    let (declared, all) = declared();

    let expected_names: BTreeSet<String> = expected.keys().cloned().collect();
    assert_eq!(
        all,
        expected_names,
        "`all-elements` does not list one feature per file in src/primitives and src/components.\n\n\
         Cargo.toml wants:\n\n{}",
        render(&expected)
    );

    let mut wrong: Vec<String> = Vec::new();
    for (name, deps) in &expected {
        let declared = declared.get(name).cloned().unwrap_or_default();
        if &declared != deps {
            wrong.push(format!(
                "  {name} = {declared:?}\n    but its source imports {deps:?}"
            ));
        }
    }

    assert!(
        wrong.is_empty(),
        "these features do not match what their sources import:\n\n{}\n\nCargo.toml wants:\n\n{}",
        wrong.join("\n"),
        render(&expected)
    );
}
