//! Range selection: click a row to select it, Shift-click another to select everything between.
//!
//! [`RangeSelection`] is a plain `Copy` handle, not a context: the rows of a list already have the
//! list in one place, and a context would only hide the wiring. It owns a set of selected *keys*
//! (the same `HashSet<String>` that `SortableRoot` and `SortableWindowRoot` take as `selected`) and
//! the anchor a Shift-click measures from, and works in indices of a list that need not be in the
//! page — which is what a windowed list needs.
//!
//! The rules:
//!
//! - A plain click sets the one row to its new state and **moves the anchor to it**.
//! - A Shift-click sets *every* row from the anchor to the clicked one, both included, to the
//!   state the clicked row just took, and moves the anchor to the clicked row. Shift-clicking a
//!   checked row therefore unchecks the run between it and the last click — the anchor is the last
//!   row clicked, not the first.
//! - A Shift-click with no anchor is a plain click.
//! - The anchor is remembered by *key* and found again by it, so a reorder (or a drop) under it
//!   does not leave it pointing at a different row. A row that is gone makes the click a plain one.
//!
//! The checkbox side is two lines: `checked=range.checked(i)` shows the row's state, and
//! `on_click=range.on_click(i)` reads the click, and with it whether Shift was held — a button
//! activated with Space reports the key's modifiers on the click it produces, so the keyboard
//! extends a range too.

use leptos::{ev, prelude::*};
use std::collections::HashSet;

pub use nocomponents_core::selection::{
    AnchorMode, RangeSelectionState, locate, range_for, set_range,
};

/// What a Shift-click measures from, and the keys that are selected.
#[derive(Copy, Clone)]
pub struct RangeSelection {
    /// The selected keys. Pass it as `selected` to a sortable list, read it, or write it (a
    /// "select all" is `selected.set(...)`): the anchor is looked up by key, so it stays valid.
    pub selected: RwSignal<HashSet<String>>,
    anchor: RwSignal<Option<(usize, String)>>,
    anchor_mode: AnchorMode,
    count: Signal<usize>,
    key: Callback<usize, String>,
}

impl RangeSelection {
    /// A selection over a list of `count` rows, whose row at an index is named by `key`. `key`
    /// reads the caller's data, so it follows a reorder.
    pub fn new(
        count: impl Into<Signal<usize>>,
        key: impl Fn(usize) -> String + Send + Sync + 'static,
    ) -> Self {
        Self::with_selected(RwSignal::new(HashSet::new()), count, key)
    }

    /// As [`new`](Self::new), over a set of keys the caller already holds.
    pub fn with_selected(
        selected: RwSignal<HashSet<String>>,
        count: impl Into<Signal<usize>>,
        key: impl Fn(usize) -> String + Send + Sync + 'static,
    ) -> Self {
        Self {
            selected,
            anchor: RwSignal::new(None),
            anchor_mode: AnchorMode::default(),
            count: count.into(),
            key: Callback::new(key),
        }
    }

    /// Sets how Shift-clicks update the anchor (default [`AnchorMode::MoveOnShift`]).
    pub fn with_anchor_mode(mut self, mode: AnchorMode) -> Self {
        self.anchor_mode = mode;
        self
    }

    fn key_of(&self, index: usize) -> String {
        untrack(|| self.key.run(index))
    }

    /// Whether the row at `index` is selected. Tracked: it re-runs when the selection or the
    /// row's key changes.
    pub fn is_selected(&self, index: usize) -> bool {
        let key = self.key.run(index);
        self.selected.with(|selected| selected.contains(&key))
    }

    /// How many are selected.
    pub fn len(&self) -> usize {
        self.selected.with(HashSet::len)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The index the next Shift-click measures from, if there is one and its row is still there.
    pub fn anchor(&self) -> Option<usize> {
        let (hint, key) = self.anchor.get_untracked()?;
        locate(
            hint,
            self.count.get_untracked(),
            |index| self.key_of(index),
            &key,
        )
    }

    /// The row at `index` was set to `checked`; with `shift`, so was everything from the anchor to
    /// it. Updates the anchor according to `anchor_mode`.
    pub fn select(&self, index: usize, checked: bool, shift: bool) {
        let count = self.count.get_untracked();
        let existing_anchor = self.anchor();
        let range = range_for(existing_anchor, index, shift, count);
        if range.is_empty() {
            return;
        }
        self.selected
            .update(|selected| set_range(selected, range, checked, |at| self.key_of(at)));
        if !(shift && existing_anchor.is_some() && self.anchor_mode == AnchorMode::KeepOnShift) {
            self.anchor.set(Some((index, self.key_of(index))));
        }
    }

    /// As [`select`](Self::select), with the state the row does not have now: a click on a title
    /// that toggles its row.
    pub fn toggle(&self, index: usize, shift: bool) {
        let checked = !self
            .selected
            .with_untracked(|selected| selected.contains(&self.key_of(index)));
        self.select(index, checked, shift);
    }

    /// Selects every row of the list, and forgets the anchor.
    pub fn select_all(&self) {
        let keys: HashSet<String> = (0..self.count.get_untracked())
            .map(|index| self.key_of(index))
            .collect();
        self.selected.set(keys);
        self.anchor.set(None);
    }

    /// Selects nothing, and forgets the anchor.
    pub fn clear(&self) {
        self.selected.set(HashSet::new());
        self.anchor.set(None);
    }

    /// The state of the row at `index`, for a `Checkbox`'s `checked`. It follows the selection, so
    /// the rows a Shift-click reaches change with it.
    pub fn checked(&self, index: usize) -> RwSignal<bool> {
        let checked = RwSignal::new(untrack(|| self.is_selected(index)));
        let range = *self;
        Effect::new(move |_| {
            checked.set(range.is_selected(index));
        });
        checked
    }

    /// For a `Checkbox`'s `on_click`: the box was clicked, so its row is set to the state the click
    /// gives it, and with Shift held so is the run from the anchor.
    pub fn on_click(&self, index: usize) -> Callback<ev::MouseEvent> {
        let range = *self;
        Callback::new(move |event: ev::MouseEvent| range.toggle(index, event.shift_key()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn keys(selected: &HashSet<String>) -> Vec<usize> {
        let mut keys: Vec<usize> = selected.iter().map(|key| key.parse().unwrap()).collect();
        keys.sort_unstable();
        keys
    }

    fn selection(count: usize) -> RangeSelection {
        RangeSelection::new(count, |index| index.to_string())
    }

    #[test]
    fn clicks_toggle_and_shift_clicks_extend() {
        let owner = Owner::new();
        owner.with(|| {
            let range = selection(10);
            range.select(2, true, false);
            assert_eq!(keys(&range.selected.get_untracked()), [2]);
            range.select(6, true, true);
            assert_eq!(keys(&range.selected.get_untracked()), [2, 3, 4, 5, 6]);
            // The anchor is now 6: Shift-unchecking 4 clears 4..=6 and leaves 2 and 3.
            range.select(4, false, true);
            assert_eq!(keys(&range.selected.get_untracked()), [2, 3]);
            assert_eq!(range.anchor(), Some(4));
            // A plain click does one and moves the anchor.
            range.select(8, true, false);
            range.select(9, true, true);
            assert_eq!(keys(&range.selected.get_untracked()), [2, 3, 8, 9]);
        });
    }

    #[test]
    fn a_shift_click_with_no_anchor_is_a_plain_click() {
        let owner = Owner::new();
        owner.with(|| {
            let range = selection(10);
            range.select(5, true, true);
            assert_eq!(keys(&range.selected.get_untracked()), [5]);
        });
    }

    #[test]
    fn toggling_takes_the_opposite_of_what_the_row_has() {
        let owner = Owner::new();
        owner.with(|| {
            let range = selection(10);
            range.toggle(3, false);
            assert!(range.is_selected(3));
            range.toggle(6, true);
            assert_eq!(keys(&range.selected.get_untracked()), [3, 4, 5, 6]);
            range.toggle(3, false);
            assert_eq!(keys(&range.selected.get_untracked()), [4, 5, 6]);
            range.clear();
            assert!(range.is_empty());
            assert_eq!(range.anchor(), None);
            range.select_all();
            assert_eq!(range.len(), 10);
        });
    }

    #[test]
    fn the_anchor_follows_its_row_through_a_reorder() {
        let owner = Owner::new();
        owner.with(|| {
            let order = RwSignal::new((0..10).collect::<Vec<usize>>());
            let range = RangeSelection::new(10usize, move |index| {
                order.with(|order| order[index].to_string())
            });
            range.select(2, true, false);
            order.update(|order| {
                let moved = order.remove(2);
                order.insert(7, moved);
            });
            // Row "2" now sits at index 7, so the run to index 9 is the rows there: 2, 8 and 9.
            range.select(9, true, true);
            assert_eq!(keys(&range.selected.get_untracked()), [2, 8, 9]);
        });
    }

    #[test]
    fn keep_on_shift_preserves_anchor_across_shift_clicks() {
        let owner = Owner::new();
        owner.with(|| {
            let range = selection(10).with_anchor_mode(AnchorMode::KeepOnShift);
            range.select(2, true, true);
            assert_eq!(keys(&range.selected.get_untracked()), [2]);
            assert_eq!(range.anchor(), Some(2));

            range.select(6, true, true);
            assert_eq!(keys(&range.selected.get_untracked()), [2, 3, 4, 5, 6]);
            assert_eq!(range.anchor(), Some(2));

            range.select(4, false, true);
            assert_eq!(keys(&range.selected.get_untracked()), [5, 6]);
            assert_eq!(range.anchor(), Some(2));

            range.select(7, true, false);
            assert_eq!(range.anchor(), Some(7));
            range.select(9, true, true);
            assert_eq!(keys(&range.selected.get_untracked()), [5, 6, 7, 8, 9]);
            assert_eq!(range.anchor(), Some(7));
        });
    }
}
