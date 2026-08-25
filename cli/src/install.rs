//! `cargo nocli components add` and `list`.

use crate::{
    config::Config,
    lock::{self, Lock, State},
    manifest,
    remote::Source,
    rewrite,
};
use anyhow::{Context, Result};
use std::{
    collections::{BTreeSet, VecDeque},
    fs,
    path::Path,
};

pub async fn add(source: &Source, names: &[String], force: bool) -> Result<()> {
    manifest::require_cargo_project()?;
    let config = Config::load()?;
    let module_path = config.module_path()?;
    let dir = Path::new(&config.paths.components);
    fs::create_dir_all(dir).with_context(|| format!("creating {}", config.paths.components))?;

    let mut lock = Lock::load();
    let mut queue: VecDeque<String> = names.iter().cloned().collect();
    let mut seen: BTreeSet<String> = names.iter().cloned().collect();
    let mut installed: Vec<String> = Vec::new();
    let mut written = 0usize;
    let mut kept = 0usize;
    let mut clobbered: Vec<String> = Vec::new();
    let mut uses_icons = false;

    // Components built on other components pull those in too, read out of the source just
    // downloaded rather than from a list that could drift.
    while let Some(name) = queue.pop_front() {
        let file = source.component(&name).await?;
        let out = rewrite::rewrite(&file, &module_path);
        uses_icons |= out.uses_icons;

        let path = dir.join(format!("{name}.rs"));
        let state = fs::read_to_string(&path)
            .ok()
            .map(|existing| lock.state(&name, &existing));

        // Never write over a file already there: the installed source is the project's to edit.
        // What the lock adds is *which* file that is — a component still exactly as it was
        // installed is worth refreshing, and one the project has edited is not.
        match state {
            Some(state) if !force => {
                let why = match state {
                    State::Untouched => "unchanged, --force refreshes it",
                    State::Edited => "edited here",
                    State::Unrecorded => "already here",
                };
                println!("  kept      {} ({why})", path.display());
                kept += 1;
            }
            _ => {
                if state == Some(State::Edited) {
                    clobbered.push(name.clone());
                }
                fs::write(&path, &out.source)
                    .with_context(|| format!("writing {}", path.display()))?;
                lock.record(&name, &out.source);
                let reason = if names.contains(&name) {
                    "added"
                } else {
                    "needed by"
                };
                println!("  {reason:<9} {}", path.display());
                written += 1;
            }
        }
        installed.push(name);

        for dep in out.component_deps {
            if seen.insert(dep.clone()) {
                queue.push_back(dep);
            }
        }
    }

    write_module(dir)?;
    lock.save()?;

    let mut features = manifest::current_features()?;
    features.insert("primitives".into());
    if uses_icons {
        features.insert("icons".into());
    }
    // Only what was installed: the rest are feature edges cargo works out from the library's own
    // Cargo.toml.
    features.extend(installed.iter().cloned());

    manifest::add_dependency(&features, source.checkout())?;

    print!("\n{written} written");
    if kept > 0 {
        print!(", {kept} left alone (--force overwrites)");
    }
    println!(" in {}", config.paths.components);

    // Loudly, and after the list rather than inside it: this is the one thing `add` does that
    // cannot be undone by running it again.
    if !clobbered.is_empty() {
        println!(
            "\n--force overwrote {} you had edited: {}",
            if clobbered.len() == 1 {
                "a component"
            } else {
                "components"
            },
            clobbered.join(", ")
        );
    }
    Ok(())
}

const MOD_BEGIN: &str = "// nocli:begin";
const MOD_END: &str = "// nocli:end";

/// Rewritten from the directory, so a file removed by hand stops being declared — but only
/// between the markers. Everything a project adds outside them survives, which it did not before:
/// this is the one generated file in a tree that is otherwise theirs, and it used to take a
/// `pub mod` of their own with it on every `add`.
fn write_module(dir: &Path) -> Result<()> {
    let mut names: Vec<String> = fs::read_dir(dir)
        .with_context(|| format!("reading {}", dir.display()))?
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            name.strip_suffix(".rs")
                .filter(|n| *n != "mod")
                .map(str::to_string)
        })
        .collect();
    names.sort();

    let path = dir.join("mod.rs");
    let existing = fs::read_to_string(&path).unwrap_or_default();

    // A name the project declared itself, outside the markers, is not declared again inside them:
    // the block lists every `.rs` in the directory, and a file the project added *and* wrote a
    // `pub mod` for would otherwise be declared twice and not compile.
    let theirs = outside_the_block(&existing);
    names.retain(|name| !theirs.contains(&format!("pub mod {name};")));

    let mut block = String::from(MOD_BEGIN);
    block.push_str(
        "\n// Written by `cargo nocli` from the directory, every time something is added or\n\
         // removed. The components are yours to edit; this list is not. Anything you write\n\
         // outside these two markers is left alone.\n\n",
    );
    for name in &names {
        block.push_str(&format!("pub mod {name};\n"));
    }
    block.push_str("\npub mod prelude {\n");
    for name in &names {
        block.push_str(&format!("    pub use super::{name}::*;\n"));
    }
    block.push_str("}\n");
    block.push_str(MOD_END);

    fs::write(&path, splice(&existing, &block)).context("writing mod.rs")
}

/// Everything in `mod.rs` that is the project's rather than ours.
fn outside_the_block(existing: &str) -> String {
    match (existing.find(MOD_BEGIN), existing.find(MOD_END)) {
        (Some(start), Some(end)) if end > start => {
            format!("{}{}", &existing[..start], &existing[end + MOD_END.len()..])
        }
        _ => String::new(),
    }
}

/// The generated block put back where the last one was, or a new file if there is no block to
/// replace. A `mod.rs` from before the markers existed was generated in full anyway, so replacing
/// it whole is what it was always getting.
fn splice(existing: &str, block: &str) -> String {
    match (existing.find(MOD_BEGIN), existing.find(MOD_END)) {
        (Some(start), Some(end)) if end > start => {
            let mut out = String::with_capacity(existing.len() + block.len());
            out.push_str(&existing[..start]);
            out.push_str(block);
            out.push_str(&existing[end + MOD_END.len()..]);
            out
        }
        _ => format!("{block}\n"),
    }
}

/// `cargo nocli components remove` — the other half of owning the source.
///
/// Refuses twice before it deletes anything: once for a component another installed one is built
/// on, which would leave the project not compiling, and once for a file the project has edited,
/// which is work `add` was careful never to overwrite and this should be no keener to throw away.
pub fn remove(names: &[String], force: bool) -> Result<()> {
    manifest::require_cargo_project()?;
    let config = Config::load()?;
    let module_path = config.module_path()?;
    let dir = Path::new(&config.paths.components);
    let mut lock = Lock::load();

    let mut removed = 0usize;
    let mut kept = 0usize;

    for name in names {
        let path = dir.join(format!("{name}.rs"));
        if !path.exists() {
            println!("  not here  {}", path.display());
            continue;
        }

        let dependents = lock::dependents_of(dir, name, &module_path)?;
        if !dependents.is_empty() && !force {
            println!(
                "  kept      {} (built on by {})",
                path.display(),
                dependents.join(", ")
            );
            kept += 1;
            continue;
        }

        let edited = fs::read_to_string(&path)
            .map(|text| lock.state(name, &text) == State::Edited)
            .unwrap_or(false);
        if edited && !force {
            println!("  kept      {} (edited here)", path.display());
            kept += 1;
            continue;
        }

        fs::remove_file(&path).with_context(|| format!("removing {}", path.display()))?;
        lock.forget(name);
        println!("  removed   {}", path.display());
        removed += 1;
    }

    write_module(dir)?;
    lock.save()?;

    print!("\n{removed} removed");
    if kept > 0 {
        print!(", {kept} left alone (--force removes them anyway)");
    }
    println!(" in {}", config.paths.components);

    // Left deliberately: the feature also switches on the primitive the component was built from,
    // which anything still installed may be using.
    if removed > 0 {
        println!("Cargo.toml is untouched — drop the features by hand if nothing else needs them.");
    }
    Ok(())
}

pub async fn list(source: &Source) -> Result<()> {
    let names = source.catalogue().await?;
    let installed: BTreeSet<String> = Config::load()
        .ok()
        .and_then(|c| fs::read_dir(c.paths.components).ok())
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter_map(|e| {
                    e.file_name()
                        .to_string_lossy()
                        .strip_suffix(".rs")
                        .map(str::to_string)
                })
                .collect()
        })
        .unwrap_or_default();

    for name in &names {
        let mark = if installed.contains(name) { "*" } else { " " };
        println!("{mark} {name}");
    }
    println!("\n{} available, {} installed", names.len(), installed.len());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_first_mod_rs_is_the_block_and_nothing_else() {
        let out = splice("", "// nocli:begin\npub mod button;\n// nocli:end");
        assert!(out.starts_with(MOD_BEGIN));
        assert!(out.contains("pub mod button;"));
    }

    #[test]
    fn keeps_what_the_project_wrote_around_the_block() {
        let existing = "//! Our components.\n\n// nocli:begin\npub mod button;\n// nocli:end\n\npub mod our_own;\n";
        let out = splice(
            existing,
            "// nocli:begin\npub mod button;\npub mod dialog;\n// nocli:end",
        );

        assert!(out.starts_with("//! Our components."));
        assert!(
            out.contains("pub mod dialog;"),
            "the new component is declared"
        );
        assert!(
            out.trim_end().ends_with("pub mod our_own;"),
            "theirs survives"
        );
    }

    #[test]
    fn a_module_the_project_declared_itself_is_not_declared_twice() {
        let existing = "// nocli:begin\npub mod button;\n// nocli:end\n\npub mod our_own;\n";
        let theirs = outside_the_block(existing);

        assert!(theirs.contains("pub mod our_own;"));
        assert!(
            !theirs.contains("pub mod button;"),
            "the block is not theirs"
        );
    }

    #[test]
    fn a_mod_rs_from_before_the_markers_is_replaced_whole() {
        let old = "// Generated by `cargo nocli`. …\n\npub mod button;\n\npub mod prelude {\n    pub use super::button::*;\n}\n";
        let out = splice(old, "// nocli:begin\npub mod button;\n// nocli:end");

        assert!(!out.contains("Generated by"));
        assert_eq!(
            out.matches("pub mod button;").count(),
            1,
            "not declared twice"
        );
    }
}
