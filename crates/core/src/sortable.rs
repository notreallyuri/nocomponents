//! Pure sortable-list reorder arithmetic, preview offsets, and announcement formatting.

pub mod window;

/// How far, in pixels, the pointer has to travel before a press becomes a drag. Below it the press
/// is a click, and a click has to survive the shake of pressing a mouse button.
pub const ACTIVATION_DISTANCE: f64 = 4.0;

/// Shown to a screen reader once per root, and what every handle is described by.
pub const DEFAULT_INSTRUCTIONS: &str = "Press Space or Enter to pick up an item. Use the arrow keys to move it, Space or Enter to drop it, and Escape to cancel.";

/// What a list reports when an item, or a block of them, is let go somewhere new.
///
/// `to` is an index into the list **as it is once the moved items have left it** — the same
/// convention as the kanban board's `index`, and the one that makes the common case plain:
/// `to: 0` is the front of the list and `to: len - keys.len()` is the end, whatever the items
/// ahead of the old position were. The moved items go in as one block, at `to`, in `keys` order.
///
/// `keys` are the moved items in their *current* relative order, not the order they were picked
/// in, so a selection made out of order still lands in order. [`apply_move`] does the rest.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SortableMove {
    pub keys: Vec<String>,
    pub to: usize,
}

/// Applies a reported move to the caller's own list.
///
/// `key` names an element the way the items' `value` did. Keys that are not in the list are
/// ignored, and `to` is clamped to the end, so a move that arrives after the list changed under
/// it does something reasonable rather than panicking.
///
/// ```
/// use nocomponents_core::sortable::{SortableMove, apply_move};
///
/// let mut items = vec!["a", "b", "c", "d", "e"];
/// let moved = SortableMove { keys: vec!["a".into(), "c".into()], to: 2 };
/// apply_move(&mut items, &moved, |item| item.to_string());
/// assert_eq!(items, ["b", "d", "a", "c", "e"]);
/// ```
pub fn apply_move<T>(items: &mut Vec<T>, moved: &SortableMove, key: impl Fn(&T) -> String) {
    let indices: Vec<usize> = items
        .iter()
        .enumerate()
        .filter(|(_, item)| moved.keys.contains(&key(item)))
        .map(|(index, _)| index)
        .collect();

    move_block(items, &indices, moved.to);
}

/// [`apply_move`] by position: takes the items at `indices` out, keeping their relative order,
/// and puts them back as one block at `to`, counted in the list without them. Out-of-range and
/// repeated indices are ignored.
pub fn move_block<T>(items: &mut Vec<T>, indices: &[usize], to: usize) {
    let mut indices: Vec<usize> = indices
        .iter()
        .copied()
        .filter(|&index| index < items.len())
        .collect();
    indices.sort_unstable();
    indices.dedup();

    let mut taken = Vec::with_capacity(indices.len());
    for &index in indices.iter().rev() {
        taken.push(items.remove(index));
    }
    taken.reverse();

    let at = to.min(items.len());
    items.splice(at..at, taken);
}

/// Whether putting the items at `moved` (sorted) back at `to` would leave the list as it was:
/// they are one unbroken run, and it starts where `to` says.
pub fn is_noop(moved: &[usize], to: usize) -> bool {
    match (moved.first(), moved.last()) {
        (Some(&first), Some(&last)) => last - first + 1 == moved.len() && first == to,
        _ => true,
    }
}

/// Where a held item would land: how many of the items that are staying have their middle before
/// `held_center`. `centers` is every item's middle along the axis, `moved` the indices in hand.
///
/// Original positions, deliberately — not where the other items have slid to. An item that moves
/// out of the way and is then measured where it moved to would decide the drop was somewhere
/// else, and move back.
pub fn drop_index(centers: &[f64], moved: &[usize], held_center: f64) -> usize {
    centers
        .iter()
        .enumerate()
        .filter(|(index, center)| !moved.contains(index) && **center < held_center)
        .count()
}

/// How far each item has to slide along the axis to show the block at `moved` landing at `to`.
///
/// `starts` and `sizes` are the items' places at the pick-up. The slots the list had stay the
/// slots it has — the first item's start, then each next item after the one before it plus the
/// gap that sat there originally — so uneven items and gaps keep their spacing rather than
/// being averaged. An item that is moving gets the delta to its place in the block; a caller that
/// has it riding the pointer ignores that.
pub fn preview_offsets(starts: &[f64], sizes: &[f64], moved: &[usize], to: usize) -> Vec<f64> {
    let count = starts.len().min(sizes.len());
    let mut order: Vec<usize> = (0..count).filter(|index| !moved.contains(index)).collect();
    let at = to.min(order.len());
    order.splice(at..at, moved.iter().copied().filter(|&index| index < count));

    let gap_after_slot = |slot: usize| {
        if slot + 1 < count {
            starts[slot + 1] - starts[slot] - sizes[slot]
        } else {
            0.0
        }
    };

    let mut offsets = vec![0.0; count];
    let mut cursor = starts.first().copied().unwrap_or(0.0);
    for (slot, &index) in order.iter().enumerate() {
        offsets[index] = cursor - starts[index];
        cursor += sizes[index] + gap_after_slot(slot);
    }

    offsets
}

/// The shown list of a collapsed block: the items that stay, in order, with `anchor` (the pile) in
/// as one entry at `to` — counted, like [`SortableMove::to`], in the list without the block.
fn collapsed_order(count: usize, moved: &[usize], anchor: usize, to: usize) -> Vec<usize> {
    let mut order: Vec<usize> = (0..count)
        .filter(|index| *index != anchor && !moved.contains(index))
        .collect();
    let at = to.min(order.len());
    if anchor < count {
        order.insert(at, anchor);
    }
    order
}

/// [`preview_offsets`] for a block that is collapsed into one pile: how far each item has to slide
/// along the axis to show the pile at `to` (the block's `moved` indices, ascending, containing
/// `anchor`) and every item that stays closed up around it.
///
/// The shown list is the items that stay plus one entry for the pile. It is laid out as
/// [`preview_offsets`] lays out a list — the first item's start, then each next one after the one
/// before it plus the gap that sat in that place originally — so a block of one is exactly
/// [`preview_offsets`], and variable sizes and gaps keep their spacing. The other members of the
/// block are not shown and get `0.0`; the pile's own offset is where it sits when nothing carries
/// it, which is what a keyboard move draws. Closing up is toward the start of the list, so the
/// items after hidden ones slide back by the room those took.
pub fn collapsed_preview_offsets(
    starts: &[f64],
    sizes: &[f64],
    moved: &[usize],
    anchor: usize,
    to: usize,
) -> Vec<f64> {
    let count = starts.len().min(sizes.len());
    let mut offsets = vec![0.0; count];
    let order = collapsed_order(count, moved, anchor, to);

    let gap_after_slot = |slot: usize| {
        if slot + 1 < count {
            starts[slot + 1] - starts[slot] - sizes[slot]
        } else {
            0.0
        }
    };

    let mut cursor = starts.first().copied().unwrap_or(0.0);
    for (slot, &index) in order.iter().enumerate() {
        offsets[index] = cursor - starts[index];
        cursor += sizes[index] + gap_after_slot(slot);
    }

    offsets
}

/// [`drop_index`] for a collapsed block: how many of the items that stay have their middle before
/// `held_center`, on the shown list with the pile in the slot it was picked up in. That is what
/// "how many items has the pile passed" means on screen, and it is the `to` a [`SortableMove`]
/// reports. `held_center` is the pile's middle where it is carried, in the list's own coordinates.
pub fn collapsed_drop_index(
    starts: &[f64],
    sizes: &[f64],
    moved: &[usize],
    anchor: usize,
    held_center: f64,
) -> usize {
    let ahead = moved.iter().filter(|&&index| index < anchor).count();
    let at = collapsed_preview_offsets(starts, sizes, moved, anchor, anchor - ahead.min(anchor));
    let centers: Vec<f64> = (0..at.len())
        .map(|index| starts[index] + at[index] + sizes[index] / 2.0)
        .collect();
    drop_index(&centers, moved, held_center)
}

/// What a list tells a screen reader about the item in hand.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SortableStatus {
    /// The item's `label`, or its text when it was given none.
    pub label: String,
    /// One-based, in the list as it would be if the move ended now.
    pub position: usize,
    pub total: usize,
    /// How many items are in the block. One, unless a selection was picked up with it.
    pub count: usize,
}

/// The four things worth announcing.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SortableEvent {
    PickedUp(SortableStatus),
    Moved(SortableStatus),
    Dropped(SortableStatus),
    Cancelled(SortableStatus),
}

/// The English wording, which `announce` replaces wholesale.
pub fn default_announcement(event: &SortableEvent) -> String {
    match event {
        SortableEvent::PickedUp(s) if s.count > 1 => format!(
            "Picked up {} items, starting with {}. Position {} of {}.",
            s.count, s.label, s.position, s.total
        ),
        SortableEvent::PickedUp(s) => format!(
            "Picked up {}, position {} of {}.",
            s.label, s.position, s.total
        ),
        SortableEvent::Moved(s) if s.count > 1 => format!(
            "{} items, starting with {}, moved to position {} of {}.",
            s.count, s.label, s.position, s.total
        ),
        SortableEvent::Moved(s) => format!(
            "{} moved to position {} of {}.",
            s.label, s.position, s.total
        ),
        SortableEvent::Dropped(s) if s.count > 1 => format!(
            "Dropped {} items, starting with {}, at position {} of {}.",
            s.count, s.label, s.position, s.total
        ),
        SortableEvent::Dropped(s) => format!(
            "Dropped {} at position {} of {}.",
            s.label, s.position, s.total
        ),
        SortableEvent::Cancelled(s) => format!(
            "Move cancelled. {} is back at position {} of {}.",
            s.label, s.position, s.total
        ),
    }
}

/// What a list tells a screen reader *while* a move is in progress, as opposed to at its
/// four moments ([`SortableEvent`]). Written to a polite live region, and re-written when the
/// destination changes.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SortableProgress {
    /// The picked item's name.
    pub label: String,
    /// How many items are in the block. One, unless a selection was picked up with it.
    pub count: usize,
    /// One-based: where the *first* item of the block would land if the move ended now. For one
    /// item, its own position.
    pub destination: usize,
    pub total: usize,
}

/// The English sentence for [`SortableProgress`], which `progress` replaces wholesale:
/// "Moving 4 items. Destination position 12 of 5,000."
pub fn default_progress(progress: &SortableProgress) -> String {
    let what = if progress.count > 1 {
        format!("{} items", progress.count)
    } else {
        progress.label.clone()
    };
    format!(
        "Moving {what}. Destination position {} of {}.",
        group_digits(progress.destination),
        group_digits(progress.total)
    )
}

/// `5000` as `5,000`.
fn group_digits(n: usize) -> String {
    let digits = n.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (at, digit) in digits.chars().enumerate() {
        if at > 0 && (digits.len() - at).is_multiple_of(3) {
            out.push(',');
        }
        out.push(digit);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn moved(keys: &[&str], to: usize) -> SortableMove {
        SortableMove {
            keys: keys.iter().map(|k| k.to_string()).collect(),
            to,
        }
    }

    fn apply(items: &[&str], keys: &[&str], to: usize) -> Vec<String> {
        let mut items: Vec<String> = items.iter().map(|s| s.to_string()).collect();
        apply_move(&mut items, &moved(keys, to), |s| s.clone());
        items
    }

    #[test]
    fn one_item_moves_forward_and_back() {
        assert_eq!(
            apply(&["a", "b", "c", "d"], &["a"], 2),
            ["b", "c", "a", "d"]
        );
        assert_eq!(
            apply(&["a", "b", "c", "d"], &["d"], 1),
            ["a", "d", "b", "c"]
        );
    }

    #[test]
    fn to_counts_the_list_without_the_moved_items() {
        assert_eq!(
            apply(&["a", "b", "c", "d"], &["a"], 3),
            ["b", "c", "d", "a"]
        );
        assert_eq!(
            apply(&["a", "b", "c", "d"], &["d"], 0),
            ["d", "a", "b", "c"]
        );
        assert_eq!(
            apply(&["a", "b", "c", "d"], &["b", "c"], 2),
            ["a", "d", "b", "c"]
        );
    }

    #[test]
    fn a_block_lands_together_in_the_order_the_list_had_it() {
        assert_eq!(
            apply(&["a", "b", "c", "d", "e"], &["a", "c"], 2),
            ["b", "d", "a", "c", "e"]
        );
        assert_eq!(
            apply(&["a", "b", "c", "d", "e"], &["e", "b"], 1),
            ["a", "b", "e", "c", "d"]
        );
    }

    #[test]
    fn keys_in_the_wrong_order_still_land_in_list_order() {
        assert_eq!(
            apply(&["a", "b", "c", "d"], &["c", "a"], 1),
            ["b", "a", "c", "d"]
        );
    }

    #[test]
    fn a_stale_move_does_something_reasonable() {
        assert_eq!(apply(&["a", "b", "c"], &["a"], 99), ["b", "c", "a"]);
        assert_eq!(apply(&["a", "b", "c"], &["zzz"], 1), ["a", "b", "c"]);
        assert_eq!(apply(&[], &["a"], 0), Vec::<String>::new());
    }

    #[test]
    fn move_block_ignores_repeats_and_out_of_range() {
        let mut items = vec![1, 2, 3, 4];
        move_block(&mut items, &[1, 1, 9], 2);
        assert_eq!(items, [1, 3, 2, 4]);
    }

    #[test]
    fn putting_a_run_back_where_it_was_is_a_noop() {
        assert!(is_noop(&[2], 2));
        assert!(is_noop(&[1, 2, 3], 1));
        assert!(is_noop(&[], 0));
        assert!(!is_noop(&[2], 3));
        assert!(!is_noop(&[1, 3], 1));
        assert!(!is_noop(&[0, 1], 1));
    }

    #[test]
    fn noop_agrees_with_what_apply_does() {
        for moved_set in [vec![0usize], vec![2], vec![1, 2], vec![0, 2], vec![3]] {
            for to in 0..=(5 - moved_set.len()) {
                let mut items: Vec<usize> = (0..5).collect();
                move_block(&mut items, &moved_set, to);
                assert_eq!(
                    is_noop(&moved_set, to),
                    items == (0..5).collect::<Vec<_>>(),
                    "{moved_set:?} to {to}"
                );
            }
        }
    }

    #[test]
    fn the_drop_index_counts_staying_items_before_the_held_middle() {
        let centers = [5.0, 15.0, 25.0, 35.0];
        assert_eq!(drop_index(&centers, &[0], 0.0), 0);
        assert_eq!(drop_index(&centers, &[0], 20.0), 1);
        assert_eq!(drop_index(&centers, &[0], 100.0), 3);
        assert_eq!(drop_index(&centers, &[1, 2], 100.0), 2);
        assert_eq!(drop_index(&centers, &[1, 2], 5.0), 0);
    }

    #[test]
    fn a_held_item_returns_to_its_own_slot() {
        let centers = [5.0, 15.0, 25.0, 35.0];
        assert_eq!(drop_index(&centers, &[2], 25.0), 2);
        assert_eq!(drop_index(&centers, &[2], 26.0), 2);
    }

    #[test]
    fn uniform_items_slide_by_one_slot() {
        let starts = [0.0, 10.0, 20.0, 30.0];
        let sizes = [10.0; 4];
        assert_eq!(
            preview_offsets(&starts, &sizes, &[0], 2),
            [20.0, -10.0, -10.0, 0.0]
        );
        assert_eq!(
            preview_offsets(&starts, &sizes, &[3], 0),
            [10.0, 10.0, 10.0, -30.0]
        );
        assert_eq!(preview_offsets(&starts, &sizes, &[1], 1), [0.0; 4]);
    }

    #[test]
    fn the_progress_sentence_names_the_block_and_the_destination() {
        let progress = |count, destination| SortableProgress {
            label: "Track 0001".into(),
            count,
            destination,
            total: 5000,
        };
        assert_eq!(
            default_progress(&progress(4, 12)),
            "Moving 4 items. Destination position 12 of 5,000."
        );
        assert_eq!(
            default_progress(&progress(1, 3)),
            "Moving Track 0001. Destination position 3 of 5,000."
        );
        assert_eq!(group_digits(0), "0");
        assert_eq!(group_digits(999), "999");
        assert_eq!(group_digits(1000), "1,000");
        assert_eq!(group_digits(1234567), "1,234,567");
    }

    #[test]
    fn a_block_leaves_a_hole_as_tall_as_itself() {
        let starts = [0.0, 10.0, 20.0, 30.0, 40.0];
        let sizes = [10.0; 5];
        assert_eq!(
            preview_offsets(&starts, &sizes, &[0, 1], 3),
            [30.0, 30.0, -20.0, -20.0, -20.0]
        );
    }

    #[test]
    fn gaps_and_uneven_sizes_keep_their_spacing() {
        let starts = [0.0, 24.0, 68.0];
        let sizes = [20.0, 40.0, 20.0];
        let offsets = preview_offsets(&starts, &sizes, &[0], 1);
        assert_eq!(offsets[1], -24.0);
        assert_eq!(offsets[0], 44.0);
        assert_eq!(offsets[2], 0.0);
    }

    /// Items of mixed sizes with a gap of `gap` between each, as `(starts, sizes)`.
    fn laid_out(sizes: &[f64], gap: f64) -> (Vec<f64>, Vec<f64>) {
        let mut at = 0.0;
        let starts = sizes
            .iter()
            .map(|size| {
                let start = at;
                at += size + gap;
                start
            })
            .collect();
        (starts, sizes.to_vec())
    }

    fn blocks(count: usize) -> Vec<(Vec<usize>, usize)> {
        let mut found = Vec::new();
        for moved in [
            vec![0usize],
            vec![count - 1],
            vec![1, 2],
            vec![1, 4, 6],
            vec![0, 3, 7],
            vec![2, 3, 4, 5],
            vec![0, 1, 2, 3, 4, 5, 6, 7],
            vec![0, 7],
        ] {
            for &anchor in &moved {
                found.push((moved.clone(), anchor));
            }
        }
        found
    }

    /// The items as shown, in order along the axis, once the members of the block other than the
    /// pile are taken away: sorted by where `offsets` puts them.
    fn shown(starts: &[f64], offsets: &[f64], moved: &[usize], anchor: usize) -> Vec<usize> {
        let mut shown: Vec<usize> = (0..starts.len())
            .filter(|index| *index == anchor || !moved.contains(index))
            .collect();
        shown.sort_by(|a, b| {
            (starts[*a] + offsets[*a])
                .partial_cmp(&(starts[*b] + offsets[*b]))
                .unwrap()
        });
        shown
    }

    #[test]
    fn a_collapsed_block_of_one_is_the_plain_preview() {
        for gap in [0.0, 4.0] {
            let (starts, sizes) = laid_out(&[20.0, 40.0, 20.0, 30.0, 10.0], gap);
            for held in 0..5 {
                for to in 0..5 {
                    assert_eq!(
                        collapsed_preview_offsets(&starts, &sizes, &[held], held, to),
                        preview_offsets(&starts, &sizes, &[held], to),
                        "{held} to {to}, gap {gap}"
                    );
                }
            }
        }
    }

    #[test]
    fn what_a_collapsed_block_shows_is_what_the_move_applies() {
        for gap in [0.0, 4.0] {
            let (starts, sizes) = laid_out(&[20.0, 40.0, 20.0, 30.0, 10.0, 25.0, 15.0, 35.0], gap);
            for (moved, anchor) in blocks(8) {
                for to in 0..=(8 - moved.len()) {
                    let offsets = collapsed_preview_offsets(&starts, &sizes, &moved, anchor, to);
                    let mut applied: Vec<usize> = (0..8).collect();
                    move_block(&mut applied, &moved, to);
                    applied.retain(|index| *index == anchor || !moved.contains(index));
                    assert_eq!(
                        shown(&starts, &offsets, &moved, anchor),
                        applied,
                        "{moved:?} ({anchor}) to {to}, gap {gap}"
                    );
                }
            }
        }
    }

    #[test]
    fn a_collapsed_block_leaves_one_slot_and_no_other_gap() {
        let (starts, sizes) = laid_out(&[12.0; 8], 4.0);
        for (moved, anchor) in blocks(8) {
            for to in 0..=(8 - moved.len()) {
                let offsets = collapsed_preview_offsets(&starts, &sizes, &moved, anchor, to);
                let order = shown(&starts, &offsets, &moved, anchor);
                assert_eq!(starts[order[0]] + offsets[order[0]], starts[0]);
                for pair in order.windows(2) {
                    let end = starts[pair[0]] + offsets[pair[0]] + sizes[pair[0]];
                    assert_eq!(starts[pair[1]] + offsets[pair[1]] - end, 4.0, "{moved:?}");
                }
            }
        }
    }

    #[test]
    fn items_slide_back_by_the_room_the_hidden_ones_took() {
        let starts = [0.0, 10.0, 20.0, 30.0, 40.0, 50.0];
        let sizes = [10.0; 6];
        assert_eq!(
            collapsed_preview_offsets(&starts, &sizes, &[1, 3], 3, 2),
            [0.0, 0.0, -10.0, -10.0, -10.0, -10.0]
        );
    }

    #[test]
    fn the_slot_opens_at_the_pile_or_as_near_as_the_list_reaches() {
        let (starts, sizes) = laid_out(&[10.0; 12], 2.0);
        for (moved, anchor) in blocks(8) {
            let held = starts[anchor] + sizes[anchor] / 2.0;
            let to = collapsed_drop_index(&starts, &sizes, &moved, anchor, held);
            let at = collapsed_preview_offsets(&starts, &sizes, &moved, anchor, to);
            let slot = starts[anchor] + at[anchor] + sizes[anchor] / 2.0;
            let end = 12 - moved.len();
            assert!(
                (slot - held).abs() <= 12.0 || (to == end && slot < held),
                "{moved:?} ({anchor}): slot {slot}, pile {held}"
            );
        }
    }

    #[test]
    fn a_collapsed_drop_is_counted_in_the_items_passed() {
        let (starts, sizes) = laid_out(&[10.0; 8], 2.0);
        let (moved, anchor) = (vec![1, 3, 5], 3);
        let stack = collapsed_preview_offsets(&starts, &sizes, &moved, anchor, 2);
        let rest = [0usize, 2, 4, 6, 7];
        let middle = |index: usize| starts[index] + stack[index] + 5.0;
        for to in 0..=rest.len() {
            let held = match to {
                0 => middle(rest[0]) - 1.0,
                n if n == rest.len() => middle(rest[n - 1]) + 1.0,
                n => (middle(rest[n - 1]) + middle(rest[n])) / 2.0,
            };
            assert_eq!(
                collapsed_drop_index(&starts, &sizes, &moved, anchor, held),
                to
            );
        }
    }

    #[test]
    fn a_collapsed_drop_agrees_with_the_plain_one_for_a_block_of_one() {
        let (starts, sizes) = laid_out(&[20.0, 40.0, 20.0, 30.0, 10.0], 4.0);
        let centers: Vec<f64> = (0..5).map(|i| starts[i] + sizes[i] / 2.0).collect();
        for held in 0..5 {
            for at in (-10..200).step_by(7) {
                assert_eq!(
                    collapsed_drop_index(&starts, &sizes, &[held], held, at as f64),
                    drop_index(&centers, &[held], at as f64)
                );
            }
        }
    }

    #[test]
    fn a_horizontal_list_is_the_same_arithmetic() {
        let (starts, sizes) = laid_out(&[60.0, 90.0, 60.0, 120.0, 60.0], 8.0);
        let moved = [0, 2];
        let offsets = collapsed_preview_offsets(&starts, &sizes, &moved, 2, 1);
        let mut applied: Vec<usize> = (0..5).collect();
        move_block(&mut applied, &moved, 1);
        applied.retain(|index| *index == 2 || !moved.contains(index));
        assert_eq!(shown(&starts, &offsets, &moved, 2), applied);
    }

    fn status(count: usize) -> SortableStatus {
        SortableStatus {
            label: "Item 3".into(),
            position: 3,
            total: 10,
            count,
        }
    }

    #[test]
    fn the_default_wording_names_the_item_and_its_place() {
        assert_eq!(
            default_announcement(&SortableEvent::PickedUp(status(1))),
            "Picked up Item 3, position 3 of 10."
        );
        assert_eq!(
            default_announcement(&SortableEvent::Moved(status(1))),
            "Item 3 moved to position 3 of 10."
        );
        assert_eq!(
            default_announcement(&SortableEvent::Dropped(status(1))),
            "Dropped Item 3 at position 3 of 10."
        );
        assert_eq!(
            default_announcement(&SortableEvent::Cancelled(status(1))),
            "Move cancelled. Item 3 is back at position 3 of 10."
        );
    }

    #[test]
    fn a_block_is_announced_as_a_count() {
        assert_eq!(
            default_announcement(&SortableEvent::PickedUp(status(3))),
            "Picked up 3 items, starting with Item 3. Position 3 of 10."
        );
    }
}
