//! Headless tree expand/select state and visible-row keyboard navigation.

use std::collections::HashSet;

/// Metadata for one row in a flattened tree view.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TreeRowMeta {
    pub id: String,
    pub parent_id: Option<String>,
    pub has_children: bool,
}

impl TreeRowMeta {
    pub fn new(
        id: impl Into<String>,
        parent_id: Option<impl Into<String>>,
        has_children: bool,
    ) -> Self {
        Self {
            id: id.into(),
            parent_id: parent_id.map(Into::into),
            has_children,
        }
    }
}

/// Keyboard navigation key for a tree.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TreeKey {
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
}

/// Headless expand and selection state for a tree.
#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct TreeState {
    pub expanded: HashSet<String>,
    pub selected: Option<String>,
}

impl TreeState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_expanded(&self, id: &str) -> bool {
        self.expanded.contains(id)
    }

    pub fn expand(&mut self, id: impl Into<String>) -> bool {
        self.expanded.insert(id.into())
    }

    pub fn collapse(&mut self, id: &str) -> bool {
        self.expanded.remove(id)
    }

    /// Flips expansion of `id` and returns whether it is now expanded.
    pub fn toggle_expand(&mut self, id: impl Into<String>) -> bool {
        let id = id.into();
        if self.expanded.remove(&id) {
            false
        } else {
            self.expanded.insert(id);
            true
        }
    }

    pub fn select(&mut self, id: impl Into<String>) {
        self.selected = Some(id.into());
    }

    pub fn clear_selection(&mut self) {
        self.selected = None;
    }

    pub fn is_selected(&self, id: &str) -> bool {
        self.selected.as_deref() == Some(id)
    }

    /// Filters a pre-order slice of all tree rows down to those whose ancestors are all expanded.
    pub fn visible_rows<'a>(&self, all_rows: &'a [TreeRowMeta]) -> Vec<&'a TreeRowMeta> {
        let mut visible_ids = HashSet::new();
        let mut out = Vec::new();

        for row in all_rows {
            let shown = match &row.parent_id {
                None => true,
                Some(parent) => visible_ids.contains(parent.as_str()) && self.is_expanded(parent),
            };
            if shown {
                visible_ids.insert(row.id.as_str());
                out.push(row);
            }
        }

        out
    }

    /// Navigates over a flat slice of currently visible rows (`rows`) from `current_id`.
    ///
    /// - `Up` / `Down` / `Home` / `End` move focus between visible rows.
    /// - `Right` expands a collapsed branch, or moves into its first child if already expanded.
    /// - `Left` collapses an expanded branch, or moves focus to its parent row.
    ///
    /// Returns the id of the row that should hold focus after the key press.
    pub fn navigate(
        &mut self,
        rows: &[TreeRowMeta],
        current_id: Option<&str>,
        key: TreeKey,
    ) -> Option<String> {
        if rows.is_empty() {
            return None;
        }

        let current_idx = current_id.and_then(|id| rows.iter().position(|row| row.id == id));

        match key {
            TreeKey::Home => Some(rows[0].id.clone()),
            TreeKey::End => Some(rows[rows.len() - 1].id.clone()),
            TreeKey::Down => {
                let next = match current_idx {
                    Some(idx) => (idx + 1).min(rows.len() - 1),
                    None => 0,
                };
                Some(rows[next].id.clone())
            }
            TreeKey::Up => {
                let prev = match current_idx {
                    Some(idx) => idx.saturating_sub(1),
                    None => rows.len() - 1,
                };
                Some(rows[prev].id.clone())
            }
            TreeKey::Right => {
                let Some(idx) = current_idx else {
                    return Some(rows[0].id.clone());
                };
                let row = &rows[idx];
                if row.has_children {
                    if !self.is_expanded(&row.id) {
                        self.expand(row.id.clone());
                        return Some(row.id.clone());
                    }
                    if let Some(next) = rows.get(idx + 1)
                        && next.parent_id.as_deref() == Some(row.id.as_str())
                    {
                        return Some(next.id.clone());
                    }
                }
                Some(row.id.clone())
            }
            TreeKey::Left => {
                let Some(idx) = current_idx else {
                    return Some(rows[0].id.clone());
                };
                let row = &rows[idx];
                if row.has_children && self.is_expanded(&row.id) {
                    self.collapse(&row.id);
                    return Some(row.id.clone());
                }
                if let Some(parent_id) = &row.parent_id
                    && rows.iter().any(|r| r.id == *parent_id)
                {
                    return Some(parent_id.clone());
                }
                Some(row.id.clone())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tree() -> Vec<TreeRowMeta> {
        vec![
            TreeRowMeta::new("src", None::<&str>, true),
            TreeRowMeta::new("main.rs", Some("src"), false),
            TreeRowMeta::new("lib.rs", Some("src"), false),
            TreeRowMeta::new("Cargo.toml", None::<&str>, false),
        ]
    }

    #[test]
    fn expand_collapse_toggle_and_select_manage_state() {
        let mut tree = TreeState::new();
        assert!(!tree.is_expanded("src"));

        assert!(tree.expand("src"));
        assert!(!tree.expand("src"));
        assert!(tree.is_expanded("src"));

        assert!(!tree.toggle_expand("src"));
        assert!(!tree.is_expanded("src"));
        assert!(tree.toggle_expand("src"));
        assert!(tree.is_expanded("src"));

        assert!(tree.collapse("src"));
        assert!(!tree.collapse("src"));

        tree.select("main.rs");
        assert!(tree.is_selected("main.rs"));
        tree.clear_selection();
        assert!(!tree.is_selected("main.rs"));
    }

    #[test]
    fn visible_rows_respects_expanded_ancestors() {
        let all = sample_tree();
        let mut tree = TreeState::new();

        let ids = |t: &TreeState| -> Vec<String> {
            t.visible_rows(&all)
                .into_iter()
                .map(|r| r.id.clone())
                .collect()
        };

        assert_eq!(ids(&tree), ["src", "Cargo.toml"]);
        tree.expand("src");
        assert_eq!(ids(&tree), ["src", "main.rs", "lib.rs", "Cargo.toml"]);
    }

    #[test]
    fn keyboard_navigation_walks_visible_rows_and_expands_or_collapses() {
        let all = sample_tree();
        let mut tree = TreeState::new();

        let visible = |t: &TreeState| -> Vec<TreeRowMeta> {
            t.visible_rows(&all).into_iter().cloned().collect()
        };

        // Right on collapsed branch expands it without moving focus.
        let mut focus = tree.navigate(&visible(&tree), Some("src"), TreeKey::Right);
        assert_eq!(focus.as_deref(), Some("src"));
        assert!(tree.is_expanded("src"));

        // Right on already-expanded branch enters its first child.
        focus = tree.navigate(&visible(&tree), focus.as_deref(), TreeKey::Right);
        assert_eq!(focus.as_deref(), Some("main.rs"));

        // Down moves to sibling, End to last visible, Up back, Left from leaf to parent.
        focus = tree.navigate(&visible(&tree), focus.as_deref(), TreeKey::Down);
        assert_eq!(focus.as_deref(), Some("lib.rs"));

        focus = tree.navigate(&visible(&tree), focus.as_deref(), TreeKey::Left);
        assert_eq!(focus.as_deref(), Some("src"));

        // Left on expanded branch collapses it.
        focus = tree.navigate(&visible(&tree), focus.as_deref(), TreeKey::Left);
        assert_eq!(focus.as_deref(), Some("src"));
        assert!(!tree.is_expanded("src"));

        assert_eq!(
            tree.navigate(&visible(&tree), focus.as_deref(), TreeKey::End)
                .as_deref(),
            Some("Cargo.toml")
        );
        assert_eq!(
            tree.navigate(&visible(&tree), Some("Cargo.toml"), TreeKey::Home)
                .as_deref(),
            Some("src")
        );
    }
}
