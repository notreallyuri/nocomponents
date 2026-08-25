//! What `add` wrote, so a later command can tell a file the project has edited from one it has not.
//!
//! `--force` was all-or-nothing without this: the only two answers were "never overwrite" and
//! "overwrite everything", and neither could say which of the files in the way were the project's
//! own work. A hash of each file as it was installed is enough to tell the two apart, and it lives
//! beside the config rather than inside it because it is generated and the config is written.
//!
//! The hash is FNV-1a rather than anything cryptographic. Nothing here is defending against a
//! forged file — the question is only "is this byte-for-byte what we wrote", and a 64-bit hash
//! answers it. `\r` is skipped so a checkout on Windows does not read as edited.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, path::Path};

pub const LOCK_FILE: &str = "nocomponents.lock";

/// What a file on disk is, relative to what was installed.
#[derive(Debug, PartialEq, Eq)]
pub enum State {
    /// Byte-for-byte what `add` wrote.
    Untouched,
    /// Installed, and changed since — the project's work.
    Edited,
    /// Here, but installed before the lock existed, so there is nothing to compare against.
    Unrecorded,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Lock {
    #[serde(default)]
    components: BTreeMap<String, String>,
}

pub fn hash(text: &str) -> String {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;

    let mut h = OFFSET;
    for byte in text.bytes().filter(|b| *b != b'\r') {
        h ^= byte as u64;
        h = h.wrapping_mul(PRIME);
    }
    format!("{h:016x}")
}

impl Lock {
    /// A missing lock is an empty one, never an error: a project that installed components before
    /// this existed should keep working, and every file it has reads as `Unrecorded`.
    pub fn load() -> Self {
        fs::read_to_string(LOCK_FILE)
            .ok()
            .and_then(|text| toml::from_str(&text).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) -> Result<()> {
        if self.components.is_empty() {
            let _ = fs::remove_file(LOCK_FILE);
            return Ok(());
        }
        let text = toml::to_string_pretty(self).context("serialising the lock")?;
        fs::write(LOCK_FILE, text).with_context(|| format!("writing {LOCK_FILE}"))
    }

    pub fn record(&mut self, name: &str, source: &str) {
        self.components.insert(name.to_string(), hash(source));
    }

    pub fn forget(&mut self, name: &str) {
        self.components.remove(name);
    }

    pub fn state(&self, name: &str, on_disk: &str) -> State {
        match self.components.get(name) {
            None => State::Unrecorded,
            Some(recorded) if *recorded == hash(on_disk) => State::Untouched,
            Some(_) => State::Edited,
        }
    }
}

/// Whether `name` still has to be there for the other installed components to compile.
///
/// Their imports were rewritten on the way in, so a dependency reads as `<module_path>::<name>::`
/// in the file that has it — which is what makes this a search rather than a list to maintain.
pub fn dependents_of(dir: &Path, name: &str, module_path: &str) -> Result<Vec<String>> {
    let needle = format!("{module_path}::{name}::");
    let mut found = Vec::new();

    for entry in fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let entry = entry?;
        let file = entry.file_name().to_string_lossy().into_owned();
        let Some(other) = file.strip_suffix(".rs") else {
            continue;
        };
        if other == name || other == "mod" {
            continue;
        }
        if fs::read_to_string(entry.path())
            .map(|text| text.contains(&needle))
            .unwrap_or(false)
        {
            found.push(other.to_string());
        }
    }

    found.sort();
    Ok(found)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_hash_ignores_the_line_endings_a_checkout_may_change() {
        assert_eq!(hash("one\ntwo\n"), hash("one\r\ntwo\r\n"));
        assert_ne!(hash("one\ntwo\n"), hash("one\nthree\n"));
    }

    #[test]
    fn tells_an_edited_file_from_an_untouched_one() {
        let mut lock = Lock::default();
        lock.record("button", "fn button() {}");

        assert_eq!(lock.state("button", "fn button() {}"), State::Untouched);
        assert_eq!(
            lock.state("button", "fn button() { edited }"),
            State::Edited
        );
        assert_eq!(lock.state("dialog", "anything"), State::Unrecorded);
    }

    #[test]
    fn forgetting_a_component_leaves_the_rest() {
        let mut lock = Lock::default();
        lock.record("button", "a");
        lock.record("dialog", "b");
        lock.forget("button");

        assert_eq!(lock.state("button", "a"), State::Unrecorded);
        assert_eq!(lock.state("dialog", "b"), State::Untouched);
    }
}
