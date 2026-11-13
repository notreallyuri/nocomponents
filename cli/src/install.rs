//! `cargo nocli components add`, `list`, `remove` and `status`, and `icons add` / `list`.

use crate::{
    config::Config,
    lock::{self, Kind, Lock, State},
    manifest,
    remote::Source,
    rewrite::{self, Targets},
};
use anyhow::{Context, Result};
use std::{
    collections::{BTreeSet, VecDeque},
    fs,
    path::Path,
};

/// One file to write, and what the run has to say about it.
struct Outcome {
    written: usize,
    kept: usize,
    clobbered: Vec<String>,
}

impl Outcome {
    fn new() -> Self {
        Outcome {
            written: 0,
            kept: 0,
            clobbered: Vec::new(),
        }
    }
}

/// Writes `source` to `<dir>/<name>.rs` unless something is already there that should be kept.
///
/// `force` replaces whatever is there. Without it, an icon pulled in by a component is still
/// refreshed when it is exactly what was installed — but one the project has edited (redrawn with
/// its own artwork, which is the point of owning it) is never touched by a *component's* `--force`.
#[allow(clippy::too_many_arguments)]
fn place(
    kind: Kind,
    dir: &Path,
    name: &str,
    source: &str,
    asked_for: bool,
    force: bool,
    lock: &mut Lock,
    outcome: &mut Outcome,
) -> Result<()> {
    let path = dir.join(format!("{name}.rs"));
    let state = fs::read_to_string(&path)
        .ok()
        .map(|existing| lock.state(kind, name, &existing));

    let overwrite = match (&state, kind) {
        (None, _) => true,
        (Some(_), Kind::Component) => force,
        (Some(_), Kind::Icon) if asked_for => force,
        (Some(State::Untouched), Kind::Icon) => force,
        (Some(_), Kind::Icon) => false,
    };

    if !overwrite {
        let why = match (state, kind) {
            (Some(State::Edited), Kind::Icon) if force => {
                "edited here, `icons add <name> --force` replaces it"
            }
            (Some(State::Untouched), _) => "unchanged, --force refreshes it",
            (Some(State::Edited), _) => "edited here",
            _ => "not in lock, cannot tell if edited",
        };
        println!("  kept      {} ({why})", path.display());
        outcome.kept += 1;
        return Ok(());
    }

    if state == Some(State::Edited) {
        outcome.clobbered.push(name.to_string());
    }
    fs::write(&path, source).with_context(|| format!("writing {}", path.display()))?;
    lock.record(kind, name, source);
    let reason = match (&state, asked_for) {
        (Some(_), _) => "replaced",
        (None, true) => "added",
        (None, false) => "needed by",
    };
    let note = match state {
        Some(State::Edited) => " (was edited here)",
        Some(State::Unrecorded) => " (not in lock, cannot tell if it was edited)",
        _ => "",
    };
    println!("  {reason:<9} {}{note}", path.display());
    outcome.written += 1;
    Ok(())
}

/// Whether the release being installed from ships its glyphs as source. Before 0.4 the styled
/// layer drew them out of `nocomponents::icons`, and a CLI newer than the library a project has
/// pinned has to keep installing it that way.
pub async fn ships_icons(source: &Source) -> bool {
    source.icon("icon").await.is_ok()
}

pub async fn add(source: &Source, names: &[String], force: bool) -> Result<()> {
    manifest::require_cargo_project()?;
    let config = Config::load()?;
    let module_path = config.module_path()?;
    let installs_icons = ships_icons(source).await;
    let icons_path = if installs_icons {
        config.icons_module_path()?
    } else {
        "nocomponents::icons".to_string()
    };
    let targets = Targets {
        components: &module_path,
        icons: &icons_path,
    };
    let dir = Path::new(&config.paths.components);
    fs::create_dir_all(dir).with_context(|| format!("creating {}", config.paths.components))?;

    let mut lock = Lock::load();
    let mut queue: VecDeque<String> = names.iter().cloned().collect();
    let mut seen: BTreeSet<String> = names.iter().cloned().collect();
    let mut installed: Vec<String> = Vec::new();
    let mut icons: BTreeSet<String> = BTreeSet::new();
    let mut outcome = Outcome::new();

    // Components built on other components pull those in too, read out of the source just
    // downloaded rather than from a list that could drift.
    while let Some(name) = queue.pop_front() {
        let file = source.component(&name).await?;
        let out = rewrite::rewrite(&file, &targets);
        icons.extend(out.icon_deps.iter().cloned());

        // Never write over a file already there: the installed source is the project's to edit.
        // What the lock adds is *which* file that is — a component still exactly as it was
        // installed is worth refreshing, and one the project has edited is not.
        place(
            Kind::Component,
            dir,
            &name,
            &out.source,
            names.contains(&name),
            force,
            &mut lock,
            &mut outcome,
        )?;
        installed.push(name);

        for dep in out.component_deps {
            if seen.insert(dep.clone()) {
                queue.push_back(dep);
            }
        }
    }

    write_module(dir)?;
    let root_hint = crate::modules::declare_install_dir(&config.paths.components)?;

    let needs_icon_feature = !installs_icons && !icons.is_empty();
    let icon_hint_dir = installs_icons && !icons.is_empty();
    let mut icon_hint = None;
    if installs_icons && !icons.is_empty() {
        println!();
        icon_hint = install_icons(
            source,
            &config,
            &targets,
            icons,
            &[],
            force,
            &mut lock,
            &mut outcome,
        )
        .await?;
    }
    lock.save()?;

    let mut features = BTreeSet::new();
    features.insert("primitives".to_string());
    if needs_icon_feature {
        features.insert("icons".into());
    }
    // Only what was installed: the rest are feature edges cargo works out from the library's own
    // Cargo.toml.
    features.extend(installed.iter().cloned());

    manifest::add_dependency(&features, source.checkout())?;

    let dirs = if icon_hint_dir {
        format!("{} and {}", config.paths.components, config.paths.icons)
    } else {
        config.paths.components.clone()
    };
    report(&outcome, &dirs, [root_hint, icon_hint]);
    Ok(())
}

/// `cargo nocli icons add`: the named glyphs, and the wrapper they are drawn with.
pub async fn add_icons(source: &Source, names: &[String], force: bool) -> Result<()> {
    manifest::require_cargo_project()?;
    let config = Config::load()?;
    if !ships_icons(source).await {
        anyhow::bail!(
            "the nocomponents this project compiles against draws its icons from the crate rather \
             than shipping them as source — upgrade it to 0.4 or later to install them"
        );
    }
    let module_path = config.module_path()?;
    let icons_path = config.icons_module_path()?;
    let targets = Targets {
        components: &module_path,
        icons: &icons_path,
    };
    let mut lock = Lock::load();
    let mut outcome = Outcome::new();
    let hint = install_icons(
        source,
        &config,
        &targets,
        names.iter().cloned().collect(),
        names,
        force,
        &mut lock,
        &mut outcome,
    )
    .await?;
    lock.save()?;
    report(&outcome, &config.paths.icons, [hint, None]);
    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn install_icons(
    source: &Source,
    config: &Config,
    targets: &Targets<'_>,
    mut wanted: BTreeSet<String>,
    asked_for: &[String],
    force: bool,
    lock: &mut Lock,
    outcome: &mut Outcome,
) -> Result<Option<String>> {
    let dir = Path::new(&config.paths.icons);
    fs::create_dir_all(dir).with_context(|| format!("creating {}", config.paths.icons))?;

    // Every glyph is drawn through the wrapper, so it always comes along.
    wanted.insert("icon".to_string());
    let mut queue: VecDeque<String> = wanted.iter().cloned().collect();
    while let Some(name) = queue.pop_front() {
        let file = source.icon(&name).await?;
        let out = rewrite::rewrite(&file, targets);
        place(
            Kind::Icon,
            dir,
            &name,
            &out.source,
            asked_for.contains(&name),
            force,
            lock,
            outcome,
        )?;
        for dep in out.icon_deps {
            if wanted.insert(dep.clone()) {
                queue.push_back(dep);
            }
        }
    }

    write_module(dir)?;
    crate::modules::declare_install_dir(&config.paths.icons)
}

fn report(outcome: &Outcome, dir: &str, hints: [Option<String>; 2]) {
    print!("\n{} written", outcome.written);
    if outcome.kept > 0 {
        print!(", {} left alone (--force overwrites)", outcome.kept);
    }
    println!(" in {dir}");

    for hint in hints.into_iter().flatten() {
        println!("\n{hint}");
    }

    // Loudly, and after the list rather than inside it: this is the one thing `add` does that
    // cannot be undone by running it again.
    if !outcome.clobbered.is_empty() {
        println!(
            "\n--force overwrote {} you had edited: {}",
            if outcome.clobbered.len() == 1 {
                "a file"
            } else {
                "files"
            },
            outcome.clobbered.join(", ")
        );
    }
}

pub const MOD_BEGIN: &str = "// nocli:begin";
pub const MOD_END: &str = "// nocli:end";

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
    block.push_str("\n#[allow(unused_imports)]\npub mod prelude {\n");
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
            .map(|text| lock.state(Kind::Component, name, &text) == State::Edited)
            .unwrap_or(false);
        if edited && !force {
            println!("  kept      {} (edited here)", path.display());
            kept += 1;
            continue;
        }

        fs::remove_file(&path).with_context(|| format!("removing {}", path.display()))?;
        lock.forget(Kind::Component, name);
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

pub fn status() -> Result<()> {
    manifest::require_cargo_project()?;
    let config = Config::load()?;
    let lock = Lock::load();

    let unrecorded = status_of(Kind::Component, &config.paths.components, &lock)?;
    let icon_dir = Path::new(&config.paths.icons);
    if icon_dir.is_dir() {
        println!();
        status_of(Kind::Icon, &config.paths.icons, &lock)?;
    }

    if unrecorded > 0 {
        println!(
            "Components not in the lock were installed before it existed, or by hand — `components add <name> --force` replaces one and records it."
        );
    }
    Ok(())
}

/// Prints one tree's rows and its summary line, and says how many were not in the lock.
fn status_of(kind: Kind, dir: &str, lock: &Lock) -> Result<usize> {
    let files = read_installed(Path::new(dir))?;
    let rows = status_rows(kind, &files, lock);
    for (name, state) in &rows {
        println!("  {:<12} {name}", state.label());
    }

    let count = |wanted: State| rows.iter().filter(|(_, s)| *s == wanted).count();
    println!(
        "\n{} {} installed in {dir}: {} unchanged, {} edited, {} not in lock",
        rows.len(),
        match kind {
            Kind::Component => "components",
            Kind::Icon => "icons",
        },
        count(State::Untouched),
        count(State::Edited),
        count(State::Unrecorded),
    );
    Ok(match kind {
        Kind::Component => count(State::Unrecorded),
        Kind::Icon => 0,
    })
}

fn read_installed(dir: &Path) -> Result<Vec<(String, String)>> {
    let mut files: Vec<(String, String)> = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let file = entry.file_name().to_string_lossy().into_owned();
            let Some(name) = file.strip_suffix(".rs").filter(|n| *n != "mod") else {
                continue;
            };
            let text = fs::read_to_string(entry.path())
                .with_context(|| format!("reading {}", entry.path().display()))?;
            files.push((name.to_string(), text));
        }
    }
    Ok(files)
}

fn status_rows(kind: Kind, files: &[(String, String)], lock: &Lock) -> Vec<(String, State)> {
    let mut rows: Vec<(String, State)> = files
        .iter()
        .map(|(name, text)| (name.clone(), lock.state(kind, name, text)))
        .collect();
    rows.sort_by(|a, b| a.0.cmp(&b.0));
    rows
}

pub async fn list(source: &Source) -> Result<()> {
    let names = source.catalogue().await?;
    let dir = Config::load().ok().map(|c| c.paths.components);
    print_listing(&names, dir.as_deref(), "components");
    Ok(())
}

pub async fn list_icons(source: &Source) -> Result<()> {
    let mut names = source.icon_catalogue().await?;
    names.retain(|name| name != "icon");
    let dir = Config::load().ok().map(|c| c.paths.icons);
    print_listing(&names, dir.as_deref(), "icons");
    Ok(())
}

fn print_listing(names: &[String], dir: Option<&str>, what: &str) {
    let installed: BTreeSet<String> = dir
        .and_then(|dir| fs::read_dir(dir).ok())
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

    for name in names {
        let mark = if installed.contains(name) { "*" } else { " " };
        println!("{mark} {name}");
    }
    let count = names.iter().filter(|n| installed.contains(*n)).count();
    println!("\n{} {what} available, {count} installed", names.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_sorts_files_into_edited_unchanged_and_not_in_lock() {
        let mut lock = Lock::default();
        lock.record(Kind::Component, "button", "a");
        lock.record(Kind::Component, "dialog", "b");

        let files = vec![
            ("dialog".to_string(), "b changed".to_string()),
            ("button".to_string(), "a".to_string()),
            ("tabs".to_string(), "c".to_string()),
        ];

        assert_eq!(
            status_rows(Kind::Component, &files, &lock),
            vec![
                ("button".to_string(), State::Untouched),
                ("dialog".to_string(), State::Edited),
                ("tabs".to_string(), State::Unrecorded),
            ]
        );
    }

    #[test]
    fn the_generated_prelude_is_allowed_to_go_unused() {
        let dir = std::env::temp_dir().join(format!("nocli-mod-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("button.rs"), "").unwrap();
        write_module(&dir).unwrap();
        let text = fs::read_to_string(dir.join("mod.rs")).unwrap();
        fs::remove_dir_all(&dir).unwrap();

        assert!(text.contains("#[allow(unused_imports)]\npub mod prelude"));
    }

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
