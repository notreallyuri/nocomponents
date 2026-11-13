//! `cargo nocli components add`, `list`, `remove`, `status` and `update`, and `icons add` / `list` / `remove`.

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
pub async fn ships_icons(source: &Source) -> Result<bool> {
    Ok(source.icon_opt("icon").await?.is_some())
}

/// Components built on other components pull those in too, read out of the source just
/// downloaded rather than from a list that could drift.
#[allow(clippy::too_many_arguments)]
async fn install_components(
    source: &Source,
    targets: &Targets<'_>,
    dir: &Path,
    wanted: &[String],
    asked_for: &[String],
    force: bool,
    lock: &mut Lock,
    outcome: &mut Outcome,
    icons: &mut BTreeSet<String>,
) -> Result<Vec<String>> {
    let mut queue: VecDeque<String> = wanted.iter().cloned().collect();
    let mut seen: BTreeSet<String> = wanted.iter().cloned().collect();
    let mut installed: Vec<String> = Vec::new();

    while let Some(name) = queue.pop_front() {
        let file = source.component(&name).await?;
        let out = rewrite::rewrite(&file, targets);
        icons.extend(out.icon_deps.iter().cloned());

        place(
            Kind::Component,
            dir,
            &name,
            &out.source,
            asked_for.contains(&name),
            force,
            lock,
            outcome,
        )?;
        installed.push(name);

        for dep in out.component_deps {
            if seen.insert(dep.clone()) {
                queue.push_back(dep);
            }
        }
    }
    Ok(installed)
}

pub async fn add(source: &Source, names: &[String], force: bool) -> Result<()> {
    manifest::require_cargo_project()?;
    let config = Config::load()?;
    let module_path = config.module_path()?;
    let installs_icons = ships_icons(source).await?;
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
    let mut icons: BTreeSet<String> = BTreeSet::new();
    let mut outcome = Outcome::new();

    let installed = install_components(
        source,
        &targets,
        dir,
        names,
        names,
        force,
        &mut lock,
        &mut outcome,
        &mut icons,
    )
    .await?;

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
    if !ships_icons(source).await? {
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
    check_names(names)?;
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

pub fn remove_icons(names: &[String], force: bool) -> Result<()> {
    check_names(names)?;
    manifest::require_cargo_project()?;
    let config = Config::load()?;
    let icons_path = config.icons_module_path()?;
    let components_dir = Path::new(&config.paths.components);
    let dir = Path::new(&config.paths.icons);
    let mut lock = Lock::load();

    let mut removed = 0usize;
    let mut kept = 0usize;

    for name in names {
        let path = dir.join(format!("{name}.rs"));
        if !path.exists() {
            println!("  not here  {}", path.display());
            continue;
        }

        let dependents = lock::icon_dependents(components_dir, dir, name, &icons_path)?;
        if !dependents.is_empty() && !force {
            println!(
                "  kept      {} (used by {})",
                path.display(),
                dependents.join(", ")
            );
            kept += 1;
            continue;
        }

        let edited = fs::read_to_string(&path)
            .map(|text| lock.state(Kind::Icon, name, &text) == State::Edited)
            .unwrap_or(false);
        if edited && !force {
            println!("  kept      {} (edited here)", path.display());
            kept += 1;
            continue;
        }

        fs::remove_file(&path).with_context(|| format!("removing {}", path.display()))?;
        lock.forget(Kind::Icon, name);
        println!("  removed   {}", path.display());
        removed += 1;
    }

    if dir.is_dir() {
        write_module(dir)?;
    }
    lock.save()?;

    print!("\n{removed} removed");
    if kept > 0 {
        print!(", {kept} left alone (--force removes them anyway)");
    }
    println!(" in {}", config.paths.icons);
    Ok(())
}

fn check_names(names: &[String]) -> Result<()> {
    for name in names {
        let ok = !name.is_empty()
            && name
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_');
        if !ok {
            anyhow::bail!(
                "`{name}` is not a component or icon name — names are lowercase letters, digits and `_`"
            );
        }
    }
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
pub enum UpdateRow {
    Current,
    Outdated,
    Edited { changed_upstream: bool },
    Unrecorded,
    Gone,
}

pub fn classify(on_disk: &str, recorded: Option<&str>, upstream: Option<&str>) -> UpdateRow {
    let Some(upstream) = upstream else {
        return UpdateRow::Gone;
    };
    let Some(recorded) = recorded else {
        return UpdateRow::Unrecorded;
    };
    let upstream = lock::hash(upstream);
    if recorded == lock::hash(on_disk) {
        if recorded == upstream {
            UpdateRow::Current
        } else {
            UpdateRow::Outdated
        }
    } else {
        UpdateRow::Edited {
            changed_upstream: recorded != upstream,
        }
    }
}

impl UpdateRow {
    fn label(&self) -> &'static str {
        match self {
            UpdateRow::Current => "current",
            UpdateRow::Outdated => "outdated",
            UpdateRow::Edited { .. } => "edited",
            UpdateRow::Unrecorded => "not in lock",
            UpdateRow::Gone => "not in release",
        }
    }

    fn note(&self, kind: Kind) -> Option<String> {
        match self {
            UpdateRow::Current | UpdateRow::Outdated => None,
            UpdateRow::Edited {
                changed_upstream: true,
            } => Some("edited here, and changed upstream — diff it".to_string()),
            UpdateRow::Edited { .. } => Some("edited here".to_string()),
            UpdateRow::Unrecorded => {
                let what = match kind {
                    Kind::Component => "components",
                    Kind::Icon => "icons",
                };
                Some(format!(
                    "not in lock, cannot tell if edited; `{what} add <name> --force` records it"
                ))
            }
            UpdateRow::Gone => Some("yours, or dropped by the release".to_string()),
        }
    }

    fn ignored(&self) -> bool {
        !matches!(self, UpdateRow::Current | UpdateRow::Outdated)
    }
}

struct Examined {
    kind: Kind,
    name: String,
    row: UpdateRow,
    upstream: Option<rewrite::Rewritten>,
}

async fn examine(
    source: &Source,
    kind: Kind,
    dir: &str,
    targets: &Targets<'_>,
    lock: &Lock,
) -> Result<Vec<Examined>> {
    let mut files = read_installed(Path::new(dir))?;
    files.sort_by(|a, b| a.0.cmp(&b.0));

    let mut out = Vec::new();
    for (name, text) in files {
        let fetched = match kind {
            Kind::Component => source.component_opt(&name).await?,
            Kind::Icon => source.icon_opt(&name).await?,
        };
        let upstream = fetched.map(|file| rewrite::rewrite(&file, targets));
        let row = classify(
            &text,
            lock.recorded(kind, &name),
            upstream.as_ref().map(|u| u.source.as_str()),
        );
        out.push(Examined {
            kind,
            name,
            row,
            upstream,
        });
    }
    Ok(out)
}

fn print_rows(rows: &[Examined], dir: &str, what: &str) {
    println!("{what} in {dir}:");
    for e in rows {
        match e.row.note(e.kind) {
            Some(note) => println!("  {:<14} {} ({note})", e.row.label(), e.name),
            None => println!("  {:<14} {}", e.row.label(), e.name),
        }
    }
    if rows.is_empty() {
        println!("  (none installed)");
    }
}

pub async fn update(source: &Source, apply: bool) -> Result<()> {
    manifest::require_cargo_project()?;
    let config = Config::load()?;
    let module_path = config.module_path()?;
    let installs_icons = ships_icons(source).await?;
    let icons_path = if installs_icons {
        config.icons_module_path()?
    } else {
        "nocomponents::icons".to_string()
    };
    let targets = Targets {
        components: &module_path,
        icons: &icons_path,
    };
    source.catalogue().await?;
    let mut lock = Lock::load();

    let components = examine(
        source,
        Kind::Component,
        &config.paths.components,
        &targets,
        &lock,
    )
    .await?;
    let icons = if installs_icons {
        examine(source, Kind::Icon, &config.paths.icons, &targets, &lock).await?
    } else {
        Vec::new()
    };

    let all = || components.iter().chain(icons.iter());
    let outdated = all().filter(|e| e.row == UpdateRow::Outdated).count();
    let current = all().filter(|e| e.row == UpdateRow::Current).count();
    let ignored = all()
        .filter(|e| e.row.ignored() && e.row != UpdateRow::Gone)
        .count();
    let gone = all().filter(|e| e.row == UpdateRow::Gone).count();

    if !apply {
        print_rows(&components, &config.paths.components, "components");
        if installs_icons {
            println!();
            print_rows(&icons, &config.paths.icons, "icons");
        }
        print!(
            "\n{outdated} outdated, {current} current, {ignored} ignored (edited or not in lock)"
        );
        if gone > 0 {
            print!(", {gone} not in the release");
        }
        println!();
        if outdated > 0 {
            println!("run with --apply to update the {outdated} unchanged ones");
        }
        return Ok(());
    }

    let component_dir = Path::new(&config.paths.components);
    let icon_dir = Path::new(&config.paths.icons);
    let mut component_deps: BTreeSet<String> = BTreeSet::new();
    let mut icon_deps: BTreeSet<String> = BTreeSet::new();
    let mut components_touched = false;
    let mut icons_touched = false;

    for e in all().filter(|e| e.row == UpdateRow::Outdated) {
        let Some(upstream) = &e.upstream else {
            continue;
        };
        let dir = match e.kind {
            Kind::Component => component_dir,
            Kind::Icon => icon_dir,
        };
        let path = dir.join(format!("{}.rs", e.name));
        fs::write(&path, &upstream.source)
            .with_context(|| format!("writing {}", path.display()))?;
        lock.record(e.kind, &e.name, &upstream.source);
        println!("  updated   {}", path.display());
        component_deps.extend(upstream.component_deps.iter().cloned());
        icon_deps.extend(upstream.icon_deps.iter().cloned());
        match e.kind {
            Kind::Component => components_touched = true,
            Kind::Icon => icons_touched = true,
        }
    }
    lock.save()?;

    let missing_components: Vec<String> = component_deps
        .into_iter()
        .filter(|name| !component_dir.join(format!("{name}.rs")).exists())
        .collect();
    let mut outcome = Outcome::new();
    let mut hints = [None, None];
    let mut fresh: Vec<String> = Vec::new();

    if !missing_components.is_empty() {
        components_touched = true;
        fs::create_dir_all(component_dir)
            .with_context(|| format!("creating {}", config.paths.components))?;
        println!();
        let existed: BTreeSet<String> = read_installed(component_dir)?
            .into_iter()
            .map(|(name, _)| name)
            .collect();
        let mut pulled_icons = BTreeSet::new();
        let installed = install_components(
            source,
            &targets,
            component_dir,
            &missing_components,
            &[],
            false,
            &mut lock,
            &mut outcome,
            &mut pulled_icons,
        )
        .await?;
        icon_deps.extend(pulled_icons);
        fresh = installed
            .into_iter()
            .filter(|name| !existed.contains(name))
            .collect();
        hints[0] = crate::modules::declare_install_dir(&config.paths.components)?;
    }
    if components_touched {
        write_module(component_dir)?;
    }

    if installs_icons {
        let missing_icons: BTreeSet<String> = icon_deps
            .into_iter()
            .filter(|name| !icon_dir.join(format!("{name}.rs")).exists())
            .collect();
        if !missing_icons.is_empty() {
            println!();
            hints[1] = install_icons(
                source,
                &config,
                &targets,
                missing_icons,
                &[],
                false,
                &mut lock,
                &mut outcome,
            )
            .await?;
        } else if icons_touched {
            write_module(icon_dir)?;
        }
    }
    lock.save()?;

    if !fresh.is_empty() {
        let mut features = BTreeSet::new();
        features.insert("primitives".to_string());
        features.extend(fresh);
        manifest::add_dependency(&features, source.checkout())?;
    }

    for hint in hints.into_iter().flatten() {
        println!("\n{hint}");
    }

    let updated = all().filter(|e| e.row == UpdateRow::Outdated).count();
    print!("\n{updated} updated");
    if outcome.written > 0 {
        print!(", {} newly installed", outcome.written);
    }
    println!();

    for (heading, wanted_gone) in [("ignored:", false), ("not in the release:", true)] {
        let left: Vec<&Examined> = all()
            .filter(|e| e.row.ignored() && (e.row == UpdateRow::Gone) == wanted_gone)
            .collect();
        if left.is_empty() {
            continue;
        }
        println!("\n{heading}");
        for e in left {
            let dir = match e.kind {
                Kind::Component => &config.paths.components,
                Kind::Icon => &config.paths.icons,
            };
            match (&e.row, e.row.note(e.kind)) {
                (UpdateRow::Gone, _) | (_, None) => println!("  {dir}/{}.rs", e.name),
                (_, Some(note)) => println!("  {dir}/{}.rs: {note}", e.name),
            }
        }
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

    #[test]
    fn classifies_every_row_of_an_update() {
        let h = lock::hash;
        let installed = h("v1");

        assert_eq!(
            classify("v1", Some(&installed), Some("v1")),
            UpdateRow::Current
        );
        assert_eq!(
            classify("v1", Some(&installed), Some("v2")),
            UpdateRow::Outdated
        );
        assert_eq!(
            classify("v1 mine", Some(&installed), Some("v1")),
            UpdateRow::Edited {
                changed_upstream: false
            }
        );
        assert_eq!(
            classify("v1 mine", Some(&installed), Some("v2")),
            UpdateRow::Edited {
                changed_upstream: true
            }
        );
        assert_eq!(classify("v1", None, Some("v1")), UpdateRow::Unrecorded);
        assert_eq!(classify("v1", Some(&installed), None), UpdateRow::Gone);
        assert_eq!(classify("v1", None, None), UpdateRow::Gone);
    }

    #[test]
    fn a_name_cannot_be_a_path() {
        let names = |n: &str| vec![n.to_string()];
        assert!(check_names(&names("dropdown_menu")).is_ok());
        assert!(check_names(&names("../x")).is_err());
        assert!(check_names(&names("a/b")).is_err());
        assert!(check_names(&names("a\\b")).is_err());
        assert!(check_names(&names("..")).is_err());
        assert!(check_names(&names("")).is_err());
    }
}
