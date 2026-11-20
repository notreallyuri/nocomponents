//! Range selection: click a row to select it, Shift-click another to select everything between.
//!
//! Pure selection arithmetic and headless state ([`RangeSelectionState`]), independent of any UI
//! framework. Keys are owned strings so an anchor survives list reorders, and indices work against
//! virtualized/windowed lists whose off-screen rows are not mounted.

use std::{collections::HashSet, ops::RangeInclusive};

/// The indices a click at `clicked` selects: `anchor..=clicked` (either way round) when `shift` is
/// held and there is an anchor, and the one row otherwise. Clamped to a list of `count` rows;
/// empty for an empty list or a click past its end.
pub fn range_for(
    anchor: Option<usize>,
    clicked: usize,
    shift: bool,
    count: usize,
) -> RangeInclusive<usize> {
    if count == 0 || clicked >= count {
        return RangeInclusive::new(1, 0);
    }
    match anchor.filter(|_| shift) {
        Some(anchor) => {
            let anchor = anchor.min(count - 1);
            anchor.min(clicked)..=anchor.max(clicked)
        }
        None => clicked..=clicked,
    }
}

/// Sets every row in `range` selected or not, by the key `key_at` gives it.
pub fn set_range(
    selected: &mut HashSet<String>,
    range: RangeInclusive<usize>,
    checked: bool,
    key_at: impl Fn(usize) -> String,
) {
    for index in range {
        let key = key_at(index);
        if checked {
            selected.insert(key);
        } else {
            selected.remove(&key);
        }
    }
}

/// Where the row named `target` is now: at `hint` if it still is, found by a pass over the list if
/// it moved, `None` if it is gone.
pub fn locate(
    hint: usize,
    count: usize,
    key_at: impl Fn(usize) -> String,
    target: &str,
) -> Option<usize> {
    if hint < count && key_at(hint) == target {
        return Some(hint);
    }
    (0..count).find(|&index| key_at(index) == target)
}

/// Whether a Shift-click moves the anchor to the clicked row or keeps the original anchor.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AnchorMode {
    #[default]
    MoveOnShift,
    KeepOnShift,
}

/// Headless range-selection state: the selected keys and the anchor a Shift-click measures from.
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct RangeSelectionState {
    pub selected: HashSet<String>,
    anchor: Option<(usize, String)>,
    anchor_mode: AnchorMode,
}

impl RangeSelectionState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_selected(selected: HashSet<String>) -> Self {
        Self {
            selected,
            anchor: None,
            anchor_mode: AnchorMode::default(),
        }
    }

    pub fn with_anchor_mode(mut self, mode: AnchorMode) -> Self {
        self.anchor_mode = mode;
        self
    }

    pub fn is_selected(&self, key: &str) -> bool {
        self.selected.contains(key)
    }

    pub fn len(&self) -> usize {
        self.selected.len()
    }

    pub fn is_empty(&self) -> bool {
        self.selected.is_empty()
    }

    /// The index the next Shift-click measures from, if there is one and its row is still present.
    pub fn anchor(&self, count: usize, key_at: impl Fn(usize) -> String) -> Option<usize> {
        let (hint, key) = self.anchor.as_ref()?;
        locate(*hint, count, key_at, key)
    }

    /// Sets the row at `index` to `checked`; with `shift`, sets everything from the anchor to it.
    /// Updates the anchor according to `anchor_mode`.
    pub fn select(
        &mut self,
        index: usize,
        checked: bool,
        shift: bool,
        count: usize,
        key_at: impl Fn(usize) -> String,
    ) {
        let existing_anchor = self.anchor(count, &key_at);
        let range = range_for(existing_anchor, index, shift, count);
        if range.is_empty() {
            return;
        }
        set_range(&mut self.selected, range, checked, &key_at);
        if !(shift && existing_anchor.is_some() && self.anchor_mode == AnchorMode::KeepOnShift) {
            self.anchor = Some((index, key_at(index)));
        }
    }

    /// As [`select`](Self::select), flipping the current state of the row at `index`.
    pub fn toggle(
        &mut self,
        index: usize,
        shift: bool,
        count: usize,
        key_at: impl Fn(usize) -> String,
    ) {
        if index >= count {
            return;
        }
        let checked = !self.selected.contains(&key_at(index));
        self.select(index, checked, shift, count, key_at);
    }

    /// Selects every row in `0..count` and forgets the anchor.
    pub fn select_all(&mut self, count: usize, key_at: impl Fn(usize) -> String) {
        self.selected = (0..count).map(key_at).collect();
        self.anchor = None;
    }

    /// Clears the selection and forgets the anchor.
    pub fn clear(&mut self) {
        self.selected.clear();
        self.anchor = None;
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

    #[test]
    fn a_plain_click_is_the_one_row() {
        assert_eq!(range_for(Some(2), 6, false, 10), 6..=6);
        assert_eq!(range_for(None, 6, true, 10), 6..=6);
    }

    #[test]
    fn a_shift_click_runs_from_the_anchor_either_way() {
        assert_eq!(range_for(Some(2), 6, true, 10), 2..=6);
        assert_eq!(range_for(Some(6), 2, true, 10), 2..=6);
        assert_eq!(range_for(Some(4), 4, true, 10), 4..=4);
    }

    #[test]
    fn a_range_stays_inside_the_list() {
        assert!(range_for(Some(2), 6, true, 0).is_empty());
        assert!(range_for(Some(2), 10, true, 10).is_empty());
        assert_eq!(range_for(Some(50), 6, true, 10), 6..=9);
    }

    #[test]
    fn setting_a_range_adds_or_removes_exactly_it() {
        let mut selected: HashSet<String> = ["0", "8"].iter().map(|s| s.to_string()).collect();
        set_range(&mut selected, 2..=5, true, |i| i.to_string());
        assert_eq!(keys(&selected), [0, 2, 3, 4, 5, 8]);
        set_range(&mut selected, 4..=8, false, |i| i.to_string());
        assert_eq!(keys(&selected), [0, 2, 3]);
    }

    #[test]
    fn the_anchor_is_found_again_where_it_went() {
        let rows = ["a", "b", "c", "d"];
        let at = |i: usize| rows[i].to_string();
        assert_eq!(locate(2, 4, at, "c"), Some(2));
        assert_eq!(locate(0, 4, at, "c"), Some(2));
        assert_eq!(locate(9, 4, at, "d"), Some(3));
        assert_eq!(locate(1, 4, at, "z"), None);
    }

    #[test]
    fn state_machine_handles_range_toggle_and_reorder() {
        let mut order: Vec<usize> = (0..10).collect();
        let mut state = RangeSelectionState::new();

        state.toggle(2, false, order.len(), |i| order[i].to_string());
        assert_eq!(keys(&state.selected), [2]);

        let moved = order.remove(2);
        order.insert(7, moved);

        state.select(9, true, true, order.len(), |i| order[i].to_string());
        assert_eq!(keys(&state.selected), [2, 8, 9]);
        assert_eq!(state.anchor(order.len(), |i| order[i].to_string()), Some(9));

        state.clear();
        assert!(state.is_empty());
        state.select_all(order.len(), |i| order[i].to_string());
        assert_eq!(state.len(), 10);
    }

    #[test]
    fn keep_on_shift_preserves_anchor_across_shift_clicks() {
        let mut state = RangeSelectionState::new().with_anchor_mode(AnchorMode::KeepOnShift);
        let key_at = |i: usize| i.to_string();

        // Shift-click with no anchor sets the anchor.
        state.select(2, true, true, 10, key_at);
        assert_eq!(keys(&state.selected), [2]);
        assert_eq!(state.anchor(10, key_at), Some(2));

        // Shift-click extends from 2 to 6, keeping anchor at 2.
        state.select(6, true, true, 10, key_at);
        assert_eq!(keys(&state.selected), [2, 3, 4, 5, 6]);
        assert_eq!(state.anchor(10, key_at), Some(2));

        // Shift-unchecking 4 unchecks 2..=4 (from kept anchor 2), still keeping anchor at 2.
        state.select(4, false, true, 10, key_at);
        assert_eq!(keys(&state.selected), [5, 6]);
        assert_eq!(state.anchor(10, key_at), Some(2));

        // Plain click moves the anchor to 7.
        state.select(7, true, false, 10, key_at);
        assert_eq!(state.anchor(10, key_at), Some(7));
        state.select(9, true, true, 10, key_at);
        assert_eq!(keys(&state.selected), [5, 6, 7, 8, 9]);
        assert_eq!(state.anchor(10, key_at), Some(7));
    }
}
