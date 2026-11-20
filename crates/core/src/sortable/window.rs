//! Pure fixed-height windowed sortable-list arithmetic and move reports.

use super::{SortableMove, move_block};
use std::ops::Range;

/// The rows to have in the page: the ones in view, and `overscan` more on either side.
///
/// `top` is the scroller's `scrollTop` and `viewport` its `clientHeight`. Always inside
/// `0..count`, and empty for an empty list.
pub fn visible_range(
    top: f64,
    viewport: f64,
    count: usize,
    row_height: f64,
    overscan: usize,
) -> Range<usize> {
    if count == 0 || row_height <= 0.0 {
        return 0..0;
    }
    let top = top.max(0.0);
    let first = (top / row_height).floor() as usize;
    let last = ((top + viewport.max(0.0)) / row_height).ceil() as usize;

    let start = first.saturating_sub(overscan).min(count);
    let end = last.saturating_add(overscan).min(count);
    start..end.max(start)
}

/// Where a held row would land: how many of the *other* rows have their middle above
/// `held_center` (pixels from the top of the list). Counted in the list without the held row,
/// like [`SortableMove::to`], so it is the index the row ends up at.
///
/// The rule of [`super::drop_index`], on a uniform column, without a vector of centres: rows
/// `0..above` have their middle above the held one, and the held row is one of them or it is not.
pub fn drop_index(count: usize, from: usize, held_center: f64, row_height: f64) -> usize {
    if count == 0 || row_height <= 0.0 {
        return 0;
    }
    let above = ((held_center / row_height - 0.5).ceil().max(0.0) as usize).min(count);
    let to = if from < above { above - 1 } else { above };
    to.min(count - 1)
}

/// The slot row `row` is shown in while the row at `from` is previewed landing at `to`: the held
/// row is at `to`, and the ones it passed have each shifted one slot the other way.
pub fn slot_of(row: usize, from: usize, to: usize) -> usize {
    if row == from {
        return to;
    }
    let rest = if row < from { row } else { row - 1 };
    if rest < to { rest } else { rest + 1 }
}

/// The row shown at `slot`: the inverse of [`slot_of`].
pub fn row_at(slot: usize, from: usize, to: usize) -> usize {
    if slot == to {
        return from;
    }
    let rest = if slot < to { slot } else { slot - 1 };
    if rest < from { rest } else { rest + 1 }
}

/// The rows a pick-up of `picked` takes: all of the selected rows, in list order, when `picked` is
/// one of them, and otherwise `picked` alone. `selected` says whether the row at an index is
/// selected; it is asked about every row of the list, once.
pub fn block_of(count: usize, picked: usize, selected: impl Fn(usize) -> bool) -> Vec<usize> {
    if picked >= count {
        return Vec::new();
    }
    if !selected(picked) {
        return vec![picked];
    }
    (0..count).filter(|&index| selected(index)).collect()
}

/// [`drop_index`] for a block: the index its first row ends up at, counted in the list without it.
/// `moved` is the block's indices, ascending, and `held_center` the middle of the row that is held.
/// Clamped so the block still fits.
pub fn block_drop_index(count: usize, moved: &[usize], held_center: f64, row_height: f64) -> usize {
    if count == 0 || row_height <= 0.0 || moved.is_empty() {
        return 0;
    }
    let above = ((held_center / row_height - 0.5).ceil().max(0.0) as usize).min(count);
    let moved_above = moved.partition_point(|&index| index < above);
    (above - moved_above).min(count.saturating_sub(moved.len()))
}

/// [`slot_of`] for a block: the rows in hand sit together from `to`, in order, and the others close
/// up around them.
pub fn block_slot_of(row: usize, moved: &[usize], to: usize) -> usize {
    match moved.binary_search(&row) {
        Ok(rank) => to + rank,
        Err(before) => {
            let rest = row - before;
            if rest < to { rest } else { rest + moved.len() }
        }
    }
}

/// [`row_at`] for a block: the inverse of [`block_slot_of`].
pub fn block_row_at(slot: usize, moved: &[usize], to: usize) -> usize {
    let count = moved.len();
    if slot >= to && slot < to + count {
        return moved[slot - to];
    }
    let rest = if slot < to { slot } else { slot - count };
    rest + moved_ahead(rest, moved)
}

/// How many rows the list has while a block of `block` is collapsed into one: the rows that stay,
/// and the stack. `count` for a block of one.
pub fn collapsed_len(count: usize, block: usize) -> usize {
    count.saturating_sub(block.saturating_sub(1))
}

/// The slot the stack starts in: where the picked row `from` is once the rows of the block ahead of
/// it (`moved`, ascending, containing `from`) have left.
pub fn stack_slot(from: usize, moved: &[usize]) -> usize {
    from - moved.partition_point(|&index| index < from)
}

/// The number of rows ahead of the `rest`-th row that is not in hand that *are* in hand: every
/// moved row `m` with `m - rank <= rest` (the rows before it that stay) is ahead of it. `m - rank`
/// never falls, so this is a binary search.
fn moved_ahead(rest: usize, moved: &[usize]) -> usize {
    let (mut low, mut high) = (0, moved.len());
    while low < high {
        let mid = (low + high) / 2;
        if moved[mid] - mid <= rest {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

/// The slot row `row` is shown in while the block `moved` is collapsed into one stack at `to`: the
/// rows that stay close up, and the stack is a slot between them. The rows in hand have no slot of
/// their own (the picked one *is* the stack and gets `to`), so only that one is drawn.
pub fn collapsed_slot_of(row: usize, moved: &[usize], to: usize) -> usize {
    match moved.binary_search(&row) {
        Ok(_) => to,
        Err(before) => {
            let rest = row - before;
            if rest < to { rest } else { rest + 1 }
        }
    }
}

/// The row shown at `slot` of the collapsed list: the inverse of [`collapsed_slot_of`] for the rows
/// that stay. `None` for the stack's own slot, `to`.
pub fn collapsed_row_at(slot: usize, moved: &[usize], to: usize) -> Option<usize> {
    if slot == to {
        return None;
    }
    let rest = if slot < to { slot } else { slot - 1 };
    Some(rest + moved_ahead(rest, moved))
}

/// Where the stack lands: [`drop_index`] over the collapsed list, whose one stack row starts in
/// slot `stack` ([`stack_slot`]). `held_center` is in the collapsed list's own pixels. The result is
/// the index the block's first row ends up at in the list without it, which is what
/// [`SortableWindowMove::to`] says.
pub fn collapsed_drop_index(
    count: usize,
    moved: &[usize],
    stack: usize,
    held_center: f64,
    row_height: f64,
) -> usize {
    drop_index(
        collapsed_len(count, moved.len()),
        stack,
        held_center,
        row_height,
    )
}

/// The `scrollTop` to set when a block is collapsed, so that what is on screen stays where it is:
/// `hidden_above` rows (the rows of the block ahead of the held one) leave the list above it, so
/// the scroller moves up by as many. Kept inside the shorter list, `length` rows.
pub fn collapsed_top(
    top: f64,
    hidden_above: usize,
    row_height: f64,
    viewport: f64,
    length: usize,
) -> f64 {
    let most = (length as f64 * row_height - viewport).max(0.0);
    (top - hidden_above as f64 * row_height).min(most).max(0.0)
}

/// How far to move `scrollTop` when a collapsed block goes back to where it was (a cancel), so
/// the first whole row in view stays where it is: the row shown at that slot is at a different
/// index once the rows of the block are back. `0` at the top of the list, and when the row in
/// view is the stack's slot or beyond the end.
pub fn expanded_top_shift(
    top: f64,
    row_height: f64,
    moved: &[usize],
    to: usize,
    length: usize,
) -> f64 {
    if top <= 0.0 || row_height <= 0.0 {
        return 0.0;
    }
    let mut slot = (top / row_height).ceil() as usize;
    if slot == to {
        slot += 1;
    }
    if slot >= length {
        return 0.0;
    }
    collapsed_row_at(slot, moved, to).map_or(0.0, |row| (row as f64 - slot as f64) * row_height)
}

/// How far to set `scrollTop` so the row at `index` is in view, moving as little as possible
/// (`block: "nearest"`); `None` when it already is.
pub fn reveal_top(index: usize, top: f64, viewport: f64, row_height: f64) -> Option<f64> {
    reveal_top_within(index, top, viewport, row_height, 0.0)
}

/// [`reveal_top`] with `margin` pixels of the list kept visible on either side of the row, so that
/// what is drawn around it (a pile's layers and shadow) is not clipped by the scroller's edge. The
/// margin is cut down when the viewport is too short to hold it twice beside the row, so a short
/// list does not scroll back and forth. The scroller's own limits still apply: at the ends of the
/// list it cannot go further.
pub fn reveal_top_within(
    index: usize,
    top: f64,
    viewport: f64,
    row_height: f64,
    margin: f64,
) -> Option<f64> {
    let margin = margin.min(((viewport - row_height) / 2.0).max(0.0));
    let start = index as f64 * row_height - margin;
    let end = index as f64 * row_height + row_height + margin;
    if start < top {
        Some(start.max(0.0))
    } else if end > top + viewport {
        Some((end - viewport).max(0.0))
    } else {
        None
    }
}

/// What a windowed list reports when a row, or a block of them, is let go somewhere new.
///
/// `to` is the index the first of the moved rows ends up at, counted in the list **without** them —
/// [`SortableMove::to`]'s convention. For one row that is also the index it ends up at.
/// [`as_move`](Self::as_move) is the same report in the DOM-measuring list's terms.
///
/// `key` and `from` name the row that was picked up. `keys` and `indices` are every row that
/// moved, in list order and as the list was when it was picked up; with no selection they hold just
/// that row. The block lands in that order.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SortableWindowMove {
    /// What `key` said of the picked row, at the moment it was picked up.
    pub key: String,
    /// The picked row's index at the pick-up.
    pub from: usize,
    pub to: usize,
    /// What `key` said of every moved row, in list order.
    pub keys: Vec<String>,
    /// The moved rows' indices at the pick-up, ascending.
    pub indices: Vec<usize>,
}

impl SortableWindowMove {
    /// A move of one row.
    pub fn single(key: impl Into<String>, from: usize, to: usize) -> Self {
        let key = key.into();
        Self {
            keys: vec![key.clone()],
            indices: vec![from],
            key,
            from,
            to,
        }
    }

    /// How many rows moved.
    pub fn count(&self) -> usize {
        self.indices.len()
    }

    /// Whether a selection moved as one.
    pub fn is_block(&self) -> bool {
        self.indices.len() > 1
    }

    /// Applies the move to the caller's own list. Out-of-range indices are ignored, so a move
    /// that arrives after the list changed under it does something reasonable.
    pub fn apply<T>(&self, items: &mut Vec<T>) {
        move_block(items, &self.indices, self.to);
    }

    /// The same move as a [`SortableMove`], for code that already handles that one.
    pub fn as_move(&self) -> SortableMove {
        SortableMove {
            keys: self.keys.clone(),
            to: self.to,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sortable::{apply_move, drop_index as measured_drop_index, preview_offsets};

    const H: f64 = 44.0;

    fn centers(total: usize) -> Vec<f64> {
        (0..total).map(|row| row as f64 * H + H / 2.0).collect()
    }

    #[test]
    fn the_window_is_the_rows_in_view_and_a_few_more() {
        assert_eq!(visible_range(0.0, 440.0, 5000, H, 6), 0..16);
        assert_eq!(visible_range(4400.0, 440.0, 5000, H, 6), 94..116);
        assert_eq!(visible_range(0.0, 440.0, 3, H, 6), 0..3);
        assert_eq!(visible_range(0.0, 440.0, 0, H, 6), 0..0);
        assert_eq!(visible_range(0.0, 440.0, 100, H, 0), 0..10);
    }

    #[test]
    fn the_window_stays_inside_the_list_at_its_ends() {
        let end = visible_range(5000.0 * H - 440.0, 440.0, 5000, H, 6);
        assert_eq!(end.end, 5000);
        assert!(end.start > 4900);

        let past = visible_range(1.0e9, 440.0, 5000, H, 6);
        assert!(past.is_empty() && past.end <= 5000);
        assert_eq!(visible_range(-50.0, 440.0, 5000, H, 6).start, 0);
    }

    #[test]
    fn twenty_thousand_rows_keep_a_few_dozen_in_the_page() {
        for top in [0.0, 123_456.0, 20_000.0 * H - 900.0] {
            assert!(visible_range(top, 900.0, 20_000, H, 6).len() < 40, "{top}");
        }
    }

    #[test]
    fn a_drop_index_matches_the_measured_one_at_every_height() {
        let total = 12;
        for from in 0..total {
            for held in (0..=(total * 44)).step_by(7) {
                let held = held as f64;
                assert_eq!(
                    drop_index(total, from, held, H),
                    measured_drop_index(&centers(total), &[from], held),
                    "from {from}, held {held}"
                );
            }
        }
    }

    #[test]
    fn a_drop_index_is_exact_on_the_middle_of_a_row() {
        // Level with another row's middle is not past it, as in the measured list.
        assert_eq!(drop_index(10, 0, 3.0 * H + H / 2.0, H), 2);
        assert_eq!(drop_index(10, 0, 3.0 * H + H / 2.0 + 0.5, H), 3);
        assert_eq!(drop_index(10, 9, -500.0, H), 0);
        assert_eq!(drop_index(10, 0, 1.0e9, H), 9);
        assert_eq!(drop_index(0, 0, 10.0, H), 0);
    }

    #[test]
    fn the_slots_match_the_measured_offsets() {
        let total = 9;
        let starts: Vec<f64> = (0..total).map(|row| row as f64 * H).collect();
        let sizes = vec![H; total];
        for from in 0..total {
            for to in 0..total {
                let offsets = preview_offsets(&starts, &sizes, &[from], to);
                for row in 0..total {
                    let shown = slot_of(row, from, to) as f64 * H - starts[row];
                    assert_eq!(shown, offsets[row], "row {row}, from {from} to {to}");
                }
            }
        }
    }

    #[test]
    fn a_slot_names_the_row_shown_there_and_back() {
        let total = 10;
        for from in 0..total {
            for to in 0..total {
                let mut slots: Vec<usize> = (0..total).map(|row| slot_of(row, from, to)).collect();
                for row in 0..total {
                    assert_eq!(row_at(slot_of(row, from, to), from, to), row);
                    assert_eq!(slot_of(row_at(row, from, to), from, to), row);
                }
                slots.sort_unstable();
                assert_eq!(slots, (0..total).collect::<Vec<_>>(), "every slot once");
            }
        }
    }

    #[test]
    fn only_the_rows_between_from_and_to_move() {
        assert_eq!(slot_of(2, 2, 6), 6);
        assert_eq!(slot_of(3, 2, 6), 2);
        assert_eq!(slot_of(6, 2, 6), 5);
        assert_eq!(slot_of(7, 2, 6), 7);
        assert_eq!(slot_of(1, 2, 6), 1);
        assert_eq!(slot_of(4, 6, 2), 5);
        assert_eq!(slot_of(2, 6, 2), 3);
        assert_eq!(slot_of(1, 6, 2), 1);
        assert_eq!(slot_of(7, 6, 2), 7);
    }

    #[test]
    fn revealing_scrolls_as_little_as_it_can() {
        assert_eq!(reveal_top(2, 0.0, 440.0, H), None);
        assert_eq!(reveal_top(10, 0.0, 440.0, H), Some(44.0));
        assert_eq!(reveal_top(1, 440.0, 440.0, H), Some(44.0));
        assert_eq!(reveal_top(4999, 0.0, 440.0, H), Some(5000.0 * H - 440.0));
        assert!(reveal_top(0, 0.0, 20.0, H).is_none_or(|top| top >= 0.0));
    }

    #[test]
    fn revealing_a_pile_keeps_a_row_of_room_around_it() {
        assert_eq!(reveal_top_within(5, 0.0, 440.0, H, H), None);
        assert_eq!(reveal_top_within(9, 0.0, 440.0, H, H), Some(44.0));
        assert_eq!(reveal_top_within(10, 0.0, 440.0, H, H), Some(88.0));
        assert_eq!(reveal_top_within(3, 440.0, 440.0, H, H), Some(88.0));
        assert_eq!(reveal_top_within(1, 0.0, 440.0, H, H), None);
        assert_eq!(reveal_top_within(0, 400.0, 440.0, H, H), Some(0.0));
        assert_eq!(
            reveal_top_within(4999, 0.0, 440.0, H, H),
            Some(5000.0 * H - 440.0 + H)
        );
        assert_eq!(
            reveal_top_within(7, 0.0, 440.0, H, 0.0),
            reveal_top(7, 0.0, 440.0, H)
        );
    }

    #[test]
    fn a_short_viewport_does_not_scroll_back_and_forth() {
        let viewport = 2.0 * H;
        let top = reveal_top_within(5, 0.0, viewport, H, H).unwrap();
        assert_eq!(reveal_top_within(5, top, viewport, H, H), None);
    }

    #[test]
    fn a_move_applies_to_the_callers_list() {
        let mut items: Vec<usize> = (0..6).collect();
        let moved = SortableWindowMove::single("1", 1, 4);
        moved.apply(&mut items);
        assert_eq!(items, [0, 2, 3, 4, 1, 5]);

        let mut again: Vec<String> = (0..6).map(|n| n.to_string()).collect();
        apply_move(&mut again, &moved.as_move(), |s| s.clone());
        assert_eq!(again, ["0", "2", "3", "4", "1", "5"]);
    }

    #[test]
    fn a_drag_over_twenty_thousand_rows_is_arithmetic() {
        let started = std::time::Instant::now();
        let mut sum = 0usize;
        for step in 0..100_000 {
            let to = drop_index(20_000, 7, step as f64 * 9.0, H);
            sum += slot_of(step % 20_000, 7, to);
        }
        assert!(sum > 0);
        assert!(
            started.elapsed().as_millis() < 2000,
            "{:?}",
            started.elapsed()
        );
    }

    fn blocks() -> Vec<Vec<usize>> {
        vec![
            vec![4],
            vec![0],
            vec![9],
            vec![0, 1, 2],
            vec![3, 4],
            vec![1, 5, 9],
            vec![0, 9],
            vec![2, 3, 7, 8],
            vec![7, 8, 9],
        ]
    }

    #[test]
    fn a_block_of_one_is_the_single_row_arithmetic() {
        let total = 10;
        for from in 0..total {
            for to in 0..total {
                for row in 0..total {
                    assert_eq!(block_slot_of(row, &[from], to), slot_of(row, from, to));
                }
                for slot in 0..total {
                    assert_eq!(block_row_at(slot, &[from], to), row_at(slot, from, to));
                }
            }
            for held in (0..=(total * 44)).step_by(5) {
                let held = held as f64;
                assert_eq!(
                    block_drop_index(total, &[from], held, H),
                    drop_index(total, from, held, H),
                    "from {from}, held {held}"
                );
            }
        }
    }

    #[test]
    fn a_blocks_drop_index_matches_the_measured_one() {
        let total = 12;
        for moved in blocks() {
            for held in (0..=(total * 44)).step_by(7) {
                let held = held as f64;
                assert_eq!(
                    block_drop_index(total, &moved, held, H),
                    measured_drop_index(&centers(total), &moved, held),
                    "moved {moved:?}, held {held}"
                );
            }
        }
    }

    #[test]
    fn a_blocks_slots_match_the_measured_offsets() {
        let total = 12;
        let starts: Vec<f64> = (0..total).map(|row| row as f64 * H).collect();
        let sizes = vec![H; total];
        for moved in blocks() {
            for to in 0..=(total - moved.len()) {
                let offsets = preview_offsets(&starts, &sizes, &moved, to);
                for row in 0..total {
                    let shown = block_slot_of(row, &moved, to) as f64 * H - starts[row];
                    assert_eq!(shown, offsets[row], "row {row}, moved {moved:?}, to {to}");
                }
            }
        }
    }

    #[test]
    fn a_blocks_slots_and_rows_are_each_others_inverse() {
        let total = 12;
        for moved in blocks() {
            for to in 0..=(total - moved.len()) {
                let mut slots: Vec<usize> = (0..total)
                    .map(|row| block_slot_of(row, &moved, to))
                    .collect();
                for row in 0..total {
                    assert_eq!(
                        block_row_at(block_slot_of(row, &moved, to), &moved, to),
                        row,
                        "moved {moved:?}, to {to}"
                    );
                    assert_eq!(
                        block_slot_of(block_row_at(row, &moved, to), &moved, to),
                        row
                    );
                }
                slots.sort_unstable();
                assert_eq!(slots, (0..total).collect::<Vec<_>>(), "every slot once");
            }
        }
    }

    #[test]
    fn a_block_never_drops_where_it_does_not_fit() {
        for moved in blocks() {
            assert_eq!(block_drop_index(12, &moved, 1.0e9, H), 12 - moved.len());
            assert_eq!(block_drop_index(12, &moved, -1.0e9, H), 0);
        }
        assert_eq!(block_drop_index(0, &[], 10.0, H), 0);
        assert_eq!(block_drop_index(5, &[], 10.0, H), 0);
    }

    #[test]
    fn a_pick_up_takes_the_selection_only_if_it_holds_the_row() {
        let selected = |row: usize| matches!(row, 2 | 5 | 11);
        assert_eq!(block_of(20, 5, selected), [2, 5, 11]);
        assert_eq!(block_of(20, 11, selected), [2, 5, 11]);
        assert_eq!(block_of(20, 3, selected), [3]);
        assert_eq!(block_of(20, 25, selected), Vec::<usize>::new());
        // A selected row far from the page is as much a part of it as one in view.
        assert_eq!(
            block_of(5000, 4000, |row| row == 3 || row == 4000),
            [3, 4000]
        );
    }

    #[test]
    fn a_block_applies_as_the_measured_list_would() {
        let letters = ["a", "b", "c", "d", "e", "f", "g", "h"];
        for moved in blocks() {
            let moved: Vec<usize> = moved
                .into_iter()
                .filter(|&row| row < letters.len())
                .collect();
            if moved.is_empty() {
                continue;
            }
            for to in 0..=(letters.len() - moved.len()) {
                let report = SortableWindowMove {
                    key: letters[moved[0]].to_string(),
                    from: moved[0],
                    to,
                    keys: moved.iter().map(|&row| letters[row].to_string()).collect(),
                    indices: moved.clone(),
                };
                assert!(report.is_block() == (moved.len() > 1));
                let mut by_index: Vec<String> = letters.iter().map(|s| s.to_string()).collect();
                report.apply(&mut by_index);
                let mut by_key: Vec<String> = letters.iter().map(|s| s.to_string()).collect();
                apply_move(&mut by_key, &report.as_move(), |s| s.clone());
                assert_eq!(by_index, by_key, "moved {moved:?}, to {to}");
                // The block is contiguous at `to`, and the rest keep their order.
                let landed: Vec<&String> = by_index[to..to + moved.len()].iter().collect();
                let wanted: Vec<String> =
                    moved.iter().map(|&row| letters[row].to_string()).collect();
                assert_eq!(landed, wanted.iter().collect::<Vec<_>>());
                let rest: Vec<&String> = by_index
                    .iter()
                    .enumerate()
                    .filter(|(at, _)| *at < to || *at >= to + moved.len())
                    .map(|(_, s)| s)
                    .collect();
                let before: Vec<String> = letters
                    .iter()
                    .enumerate()
                    .filter(|(at, _)| !moved.contains(at))
                    .map(|(_, s)| s.to_string())
                    .collect();
                assert_eq!(rest, before.iter().collect::<Vec<_>>());
            }
        }
    }

    /// The collapsed list as a list: `None` is the stack.
    fn collapsed(total: usize, moved: &[usize], to: usize) -> Vec<Option<usize>> {
        let mut rest: Vec<Option<usize>> = (0..total)
            .filter(|row| !moved.contains(row))
            .map(Some)
            .collect();
        rest.insert(to, None);
        rest
    }

    #[test]
    fn the_collapsed_list_is_one_row_for_the_block() {
        assert_eq!(collapsed_len(5000, 1), 5000);
        assert_eq!(collapsed_len(5000, 10), 4991);
        assert_eq!(collapsed_len(5, 5), 1);
        assert_eq!(collapsed_len(0, 0), 0);
        for total in 3..12 {
            for moved in blocks() {
                if moved.iter().any(|&row| row >= total) {
                    continue;
                }
                for to in 0..=(total - moved.len()) {
                    assert_eq!(
                        collapsed(total, &moved, to).len(),
                        collapsed_len(total, moved.len())
                    );
                }
            }
        }
    }

    #[test]
    fn a_collapsed_slot_is_what_a_brute_force_collapse_says() {
        let total = 12;
        for moved in blocks() {
            for from in moved.clone() {
                let stack = stack_slot(from, &moved);
                assert_eq!(
                    stack,
                    (0..from).filter(|row| !moved.contains(row)).count(),
                    "{moved:?} {from}"
                );
            }
            for to in 0..=(total - moved.len()) {
                let list = collapsed(total, &moved, to);
                for (slot, shown) in list.iter().enumerate() {
                    assert_eq!(collapsed_row_at(slot, &moved, to), *shown, "{moved:?} {to}");
                    if let Some(row) = shown {
                        assert_eq!(collapsed_slot_of(*row, &moved, to), slot);
                    }
                }
                for &row in &moved {
                    assert_eq!(collapsed_slot_of(row, &moved, to), to);
                }
            }
        }
    }

    #[test]
    fn a_block_of_one_collapses_to_the_single_rows_slots() {
        let total = 10;
        for from in 0..total {
            assert_eq!(stack_slot(from, &[from]), from);
            for to in 0..total {
                for row in (0..total).filter(|&row| row != from) {
                    assert_eq!(collapsed_slot_of(row, &[from], to), slot_of(row, from, to));
                }
                for slot in (0..total).filter(|&slot| slot != to) {
                    assert_eq!(
                        collapsed_row_at(slot, &[from], to),
                        Some(row_at(slot, from, to))
                    );
                }
            }
            for held in (0..=(total * 44)).step_by(5) {
                let held = held as f64;
                assert_eq!(
                    collapsed_drop_index(total, &[from], from, held, H),
                    drop_index(total, from, held, H)
                );
            }
        }
    }

    #[test]
    fn a_collapsed_drop_index_matches_the_measured_one_on_the_collapsed_list() {
        let total = 12;
        for moved in blocks() {
            for &from in &moved {
                let stack = stack_slot(from, &moved);
                let length = collapsed_len(total, moved.len());
                for held in (0..=(length * 44 + 40)).step_by(7) {
                    let held = held as f64;
                    assert_eq!(
                        collapsed_drop_index(total, &moved, stack, held, H),
                        measured_drop_index(&centers(length), &[stack], held),
                        "moved {moved:?}, held row {from}, at {held}"
                    );
                }
            }
        }
    }

    #[test]
    fn what_the_stack_shows_is_what_the_drop_applies() {
        let total = 12;
        for moved in blocks() {
            for to in 0..=(total - moved.len()) {
                let mut applied: Vec<usize> = (0..total).collect();
                move_block(&mut applied, &moved, to);
                let shown: Vec<usize> = collapsed(total, &moved, to)
                    .into_iter()
                    .flat_map(|slot| slot.map_or_else(|| moved.clone(), |row| vec![row]))
                    .collect();
                assert_eq!(applied, shown, "moved {moved:?}, to {to}");
            }
        }
    }

    #[test]
    fn dragging_the_stack_across_n_rows_passes_n_rows() {
        // A scattered block, held at its second row. Carrying the stack down by whole rows of the
        // collapsed list moves it past exactly that many rows, whatever the block left behind.
        let moved = [2, 5, 9, 14];
        let (total, from) = (30, 5);
        let stack = stack_slot(from, &moved);
        assert_eq!(stack, 4);
        for rows in 0..10 {
            let held = stack as f64 * H + H / 2.0 + rows as f64 * H + 1.0;
            assert_eq!(
                collapsed_drop_index(total, &moved, stack, held, H),
                stack + rows
            );
        }
        let to = collapsed_drop_index(total, &moved, stack, 5.0 * H, H);
        let mut order: Vec<usize> = (0..total).collect();
        move_block(&mut order, &moved, to);
        assert_eq!(&order[to..to + 4], &moved);
    }

    #[test]
    fn collapsing_keeps_what_is_on_screen_where_it_is() {
        // Rows 3, 8 and 40 are in the block, the viewport shows rows 100..110, and the held row is
        // 105: the three rows above it leave, the scroller moves up three rows, and the same rows
        // are on screen at the same places.
        let moved = [3, 8, 40, 105];
        let top = 100.0 * H;
        let length = collapsed_len(5000, 4);
        let now = collapsed_top(top, 3, H, 10.0 * H, length);
        for row in 100..110usize {
            if moved.contains(&row) {
                continue;
            }
            let before = row as f64 * H - top;
            let after = collapsed_slot_of(row, &moved, stack_slot(105, &moved)) as f64 * H - now;
            assert_eq!(before, after, "row {row}");
        }
        assert_eq!(collapsed_top(2.0 * H, 3, H, 10.0 * H, length), 0.0);
        let end = length as f64 * H - 10.0 * H;
        assert_eq!(collapsed_top(1.0e9, 0, H, 10.0 * H, length), end);
        assert_eq!(collapsed_top(5.0, 0, H, 10.0 * H, 3), 0.0);
    }

    #[test]
    fn a_cancel_keeps_the_first_whole_row_where_it_is() {
        let moved = [3, 8, 40, 105];
        let (length, to) = (collapsed_len(5000, 4), 60);
        for top in [H, 20.0 * H, 77.0 * H, 59.5 * H, 4000.0 * H] {
            let slot = (top / H).ceil() as usize;
            let shift = expanded_top_shift(top, H, &moved, to, length);
            let shown = if slot == to { slot + 1 } else { slot };
            let row = collapsed_row_at(shown, &moved, to).unwrap();
            assert_eq!(
                row as f64 * H - (top + shift),
                shown as f64 * H - top,
                "top {top}"
            );
        }
        assert_eq!(expanded_top_shift(0.0, H, &moved, to, length), 0.0);
        assert_eq!(expanded_top_shift(1.0e9, H, &moved, to, length), 0.0);
    }

    #[test]
    fn a_collapsed_drag_over_twenty_thousand_rows_is_arithmetic() {
        let moved: Vec<usize> = (0..20_000).step_by(3).collect();
        let stack = stack_slot(300, &moved);
        let started = std::time::Instant::now();
        let mut sum = 0usize;
        for step in 0..100_000 {
            let to = collapsed_drop_index(20_000, &moved, stack, step as f64 * 9.0, H);
            sum += collapsed_slot_of(step % 20_000, &moved, to);
            sum += collapsed_row_at(step % 13_000, &moved, to).unwrap_or(0);
        }
        assert!(sum > 0);
        assert!(
            started.elapsed().as_millis() < 2000,
            "{:?}",
            started.elapsed()
        );
    }

    #[test]
    fn a_block_drag_over_twenty_thousand_rows_is_arithmetic() {
        let moved: Vec<usize> = (0..20_000).step_by(3).collect();
        let started = std::time::Instant::now();
        let mut sum = 0usize;
        for step in 0..100_000 {
            let to = block_drop_index(20_000, &moved, step as f64 * 9.0, H);
            sum += block_slot_of(step % 20_000, &moved, to);
            sum += block_row_at(step % 20_000, &moved, to);
        }
        assert!(sum > 0);
        assert!(
            started.elapsed().as_millis() < 2000,
            "{:?}",
            started.elapsed()
        );
    }
}
