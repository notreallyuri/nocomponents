//! A sortable list for thousands of rows: only the ones in view are in the page.
//!
//! [`super::SortableRoot`] finds its items in the DOM, measures every one at the pick-up, and has
//! every item read the whole drag state on every move — which is right for twenty items and
//! quadratic for five thousand. This is the same gesture on arithmetic instead. Every row is
//! [`SortableWindowRoot::row_height`] tall, so row `i` sits at `i * row_height`, and nothing has
//! to be measured, queried or looked up:
//!
//! - which rows to render is [`visible_range`], from the scroll offset and the viewport;
//! - where a held row would land is [`drop_index`], `O(1)`;
//! - where every other row slides to is [`slot_of`], `O(1)` from a single `(from, to)` pair, so a
//!   row's style reads only that pair and nothing about its 4,999 neighbours;
//! - which row is shown at a slot is [`row_at`], its inverse, so the window is chosen by what is
//!   *shown* in view while a row is carried a long way.
//!
//! Those are tested against [`super::drop_index`] and [`super::preview_offsets`], the maths of the
//! DOM-measuring list, for uniform heights.
//!
//! **Fixed row height, vertical only.** Equal heights are what make a row's place a
//! multiplication; a list of uneven rows wants the DOM-measuring [`super::SortableRoot`] (or a
//! measured-height window, which this is not).
//!
//! # Block drag
//!
//! Give the root `selected`, the same `HashSet` of keys [`super::SortableRoot`] takes, and picking
//! up a row that is in it takes every selected row with it, in their current order, as one block.
//! The selection is read by *key*, over the whole list, when the block is picked up — a selected
//! row that is not in the page (it scrolled away, or was never rendered) moves with the rest.
//! That is one pass of `key` over the list, once per gesture, and nothing per frame:
//!
//! - which rows are in hand is [`block_of`], from the indices that are selected;
//! - where the held row would land is [`block_drop_index`], `O(log k)` for a block of `k`;
//! - where every other row slides to is [`block_slot_of`] and the row shown at a slot is
//!   [`block_row_at`], each `O(log k)` from the sorted indices and one target, so a row's style
//!   reads only that pair and the block, not its neighbours.
//!
//! They are tested against [`super::drop_index`] and [`super::preview_offsets`] for blocks, and
//! against [`slot_of`], [`row_at`] and [`drop_index`] for a block of one. They describe where the
//! block *ends up*; what is drawn while it is held is the collapsed list below.
//!
//! ## The block is one stack while it is held
//!
//! Ten ghosts strewn through a list are noise, and worse when the selection is scattered or out of
//! view, so a block of `k > 1` collapses at the pick-up. The other `k - 1` selected rows leave the
//! page, the list closes up around them, and the list is [`collapsed_len`] rows long — one row for
//! the block, which is the picked row: *it is the stack*, and wears `data-count`. It rides the
//! pointer, or, for a keyboard move, sits in its slot. Exactly one row-sized slot shows where the
//! block will land: while a pointer holds it, an empty marker (`data-slot="sortable-window-drop"`)
//! stands in the slot, and every other row is a plain row. On the drop the stack expands to `k`
//! contiguous rows at that slot; on a cancel everything goes back. A block of one is not collapsed
//! and looks as it always did.
//!
//! The same arithmetic, on the collapsed list:
//!
//! - where the stack is drawn is [`stack_slot`] at the pick-up, and the slot of every other row is
//!   [`collapsed_slot_of`], `O(log k)` (a row that is in the block has none, and is not drawn);
//! - the row shown at a slot is [`collapsed_row_at`], its inverse, which the window is chosen
//!   through and which says `None` for the slot the stack is in;
//! - where the stack lands is [`collapsed_drop_index`]: the single row's [`drop_index`] over
//!   [`collapsed_len`] rows, so the number of rows it has passed *on screen* is what decides, and
//!   that count is the `to` that [`SortableWindowMove`] reports, in the list without the block.
//!
//! A pointer therefore lands a block where the stack is seen to be, not where the rows it left
//! behind used to be: dragging the stack past three rows passes three rows, whatever the selection
//! had between them. (Counted on the uncollapsed list, the same pointer would have to be
//! carried across the empty slots of the block too.) Picked up and let go without moving, a
//! scattered block therefore gathers where the held row is, and a block that is already one run
//! stays where it is.
//!
//! **Scrolling does not jump.** Closing the list up would move whatever is on screen by the number
//! of selected rows above the held one, and with a selection that is mostly off-screen that is the
//! whole page sliding under the user's hand. The scroller's `scrollTop` is lowered by exactly that
//! many rows at the pick-up ([`collapsed_top`]), which leaves the held row where it was under the
//! pointer, and everything near it unmoved when the selection is out of view; only selected rows
//! that were in view disappear, and the rows below them close up. Where `scrollTop` cannot go far
//! enough (the top of the list) the stack is drawn the remainder lower, so it still starts under
//! the pointer. A drop needs no compensation, since the stack expands downward from its own slot and
//! nothing above it moves; a cancel keeps the first whole row in view where it is
//! ([`expanded_top_shift`]). The scroller opts out of the browser's own scroll anchoring, which
//! would otherwise answer the height change by scrolling too.
//!
//! A keyboard pick-up with a selection moves the block the same way, the stack in its slot with
//! the same count, and every announcement says "N items" as the measured list's do. Each move
//! reveals the stack with a row of room on either side ([`reveal_top_within`]), so that its layers
//! and shadow, which lie outside the row, are never clipped by the scroller's edge, and while a
//! stack is held the list has `PILE_ROOM` of margin below its last row for the same reason at the
//! very end of the list.
//!
//! The drop marker is opaque: while a row is still sliding past it, a translucent slot would show
//! that row's border through it as a stray line.
//!
//! **The list still does not own the order.** [`SortableWindowMove`] says which indices went
//! where, and the caller applies it ([`SortableWindowMove::apply`]) to its own data. The rows are
//! *positional*: the node at index 41 stays the node at index 41 whatever the data does, and its
//! content is `row(41)`, re-run when the signals it read change. That is what makes a reorder
//! cost the rows in view, and it means `key`, `item_label` and `row` read the caller's data
//! rather than being handed a snapshot.
//!
//! # The scroller is the root's
//!
//! The root renders the scroll container, because a window is only computable when the list knows
//! its own viewport. Give it a height through `class` (`h-96`, `max-h-[60vh]`); without one it
//! grows to hold every row and the window is all of them. Its height is watched with a
//! `ResizeObserver`, so a list that resizes keeps the right rows.
//!
//! # Pointer, touch and keyboard
//!
//! The pointer goes through [`crate::primitives::drag::use_drag`], as it does for the DOM-measuring
//! list: a press becomes a drag after [`super::ACTIVATION_DISTANCE`] pixels. Held within
//! `72px` of the scroller's edge, the scroller scrolls at [`edge_ramp`]'s speed and the window
//! follows it; the held row stays in the page wherever it is carried, and keeps its place under
//! the pointer through [`ScrollAnchor`]. Escape and `pointercancel` put everything back.
//!
//! The keyboard is the same model as [`super::SortableRoot`]: Space or Enter picks up, the arrows
//! move, Home and End jump, Space or Enter drops, Escape or losing focus cancels. When nothing is
//! held, the arrows, Home and End move *focus*, and moving it
//! to a row that is not in the page scrolls it in first, by
//! setting `scrollTop` — there is no element to `scrollIntoView` yet. The row last given focus is
//! always kept in the page, so it is never unmounted under the user, and the arrows can start from it.
//!
//! The listeners are the scroller's, not the rows': one `pointerdown`, `keydown`, `focusin` and
//! `focusout` serve every row, and read the row off the node the event landed on, so mounting a
//! row attaches nothing.
//!
//! Every enabled handle (every enabled row, when there is no handle) is a tab stop, as in
//! [`super::SortableRoot`]: Tab walks the rows in the page, and the arrows are the way to reach the
//! ones that are not.
//!
//! # What a screen reader hears
//!
//! The same live regions, instructions and wording as [`super::SortableRoot`] —
//! [`super::default_announcement`], replaceable with `announce`, in the assertive region, and
//! [`super::default_progress`], replaceable with `progress`, in a polite one that says "Moving 4
//! items. Destination position 12 of 5,000." while the move lasts and empties on a drop or cancel —
//! with the item's name taken from `item_label` rather than from DOM text, since the row may not
//! be rendered when it is wanted. A caller who wants the list's own noun says it in those two
//! callbacks.
//! Rows carry `aria-setsize` and `aria-posinset`, because most of the list is not in the page.
//!
//! # Data attributes
//!
//! - scroller: `data-dragging` (a move is in progress), `data-disabled`, `data-orientation`;
//! - row: `data-index`, `data-dragging` (it is the picked row of a move), `data-following` (it is
//!   riding the pointer, so it must not ease), `data-anchor` (it is the one picked), `data-count`
//!   (on the anchor, when it holds several: the row is a stack, and wants a pile and a badge),
//!   `data-selected`, `data-disabled`. The rest of a block are not in the page while it is held;
//! - drop marker: `data-slot="sortable-window-drop"`, present while a pointer holds a block of more
//!   than one, in the slot it would land in, and given `drop_class`;
//! - handle: `data-dragging`, `data-disabled`.
//!
//! Every row has an inline `transform`, which is its place. Easing it is the styled layer's
//! business, switched on only while the scroller has `data-dragging`, so the rows are put where
//! they belong instantly on a drop, in the same frame the caller's reorder lands.

use super::{
    ACTIVATION_DISTANCE, DEFAULT_INSTRUCTIONS, SortableEvent, SortableMove, SortableProgress,
    SortableStatus, default_announcement, default_progress, from_a_control, is_noop, move_block,
};
use crate::{
    primitives::drag::{Drag, DragPoint, ScrollAnchor, edge_ramp, use_drag},
    utils::next_id,
};
use leptos::{
    context::Provider,
    ev, html,
    prelude::*,
    wasm_bindgen::{JsCast, closure::Closure},
};
use std::{collections::HashSet, ops::Range, sync::Arc, time::Duration};
use web_sys::{Element, FocusOptions, HtmlElement, ResizeObserver};

/// How near the scroller's edge a held row has to be before it starts scrolling.
const EDGE: f64 = 72.0;
/// The most it scrolls in one 16 ms tick, at the very edge.
const EDGE_SPEED: f64 = 16.0;

const INDEX_ATTR: &str = "data-index";
const HANDLE_ATTR: &str = "data-sortable-handle";
const HANDLE_SELECTOR: &str = "[data-sortable-handle]";
const ROW_SELECTOR: &str = "[data-slot=sortable-window-item]";

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

/// The gesture in progress.
#[derive(Clone, PartialEq, Eq, Debug)]
struct Gesture {
    from: usize,
    to: usize,
    /// Whether a pointer is holding it. A keyboard move has the row sit in its place instead.
    pointer: bool,
    key: String,
    label: String,
    /// The rows in hand, ascending: `from` and, when it was selected, the rest of the selection.
    moved: Arc<Vec<usize>>,
    keys: Vec<String>,
    /// The slot the stack started in, [`stack_slot`].
    stack: usize,
}

/// All a row needs to know about the gesture, and nothing it would have to re-read to know it.
#[derive(Clone, Debug)]
struct Plan {
    from: usize,
    to: usize,
    pointer: bool,
    moved: Arc<Vec<usize>>,
    stack: usize,
}

impl PartialEq for Plan {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from
            && self.stack == other.stack
            && self.to == other.to
            && self.pointer == other.pointer
            && Arc::ptr_eq(&self.moved, &other.moved)
    }
}

impl Plan {
    fn contains(&self, row: usize) -> bool {
        self.moved.binary_search(&row).is_ok()
    }

    fn slot(&self, row: usize) -> usize {
        collapsed_slot_of(row, &self.moved, self.to)
    }

    fn row_at(&self, slot: usize) -> Option<usize> {
        collapsed_row_at(slot, &self.moved, self.to)
    }

    fn marked(&self) -> bool {
        self.pointer && self.moved.len() > 1
    }
}

#[derive(Copy, Clone)]
pub struct SortableWindowContext {
    pub count: Signal<usize>,
    pub row_height: f64,
    pub disabled: Signal<bool>,
    key: Callback<usize, String>,
    item_label: Option<Callback<usize, String>>,
    selected: Signal<HashSet<String>>,
    gesture: RwSignal<Option<Gesture>>,
    plan: Memo<Option<Plan>>,
    /// How far the pointer has travelled since the press.
    dy: RwSignal<f64>,
    /// How far the scroller has moved out from under it since, from [`ScrollAnchor::shift`].
    shift: RwSignal<f64>,
    /// How far below its slot the stack starts, when the scroller could not follow the collapse
    /// all the way: the pointer is still on it.
    lift: RwSignal<f64>,
    pointer_y: RwSignal<f64>,
    anchor: RwSignal<Option<ScrollAnchor>>,
    /// The row last given focus, or last picked. Always in the page.
    active: RwSignal<usize>,
    scroll_top: RwSignal<f64>,
    height: RwSignal<f64>,
    scroller: NodeRef<html::Div>,
    list: NodeRef<html::Div>,
    announcement: RwSignal<String>,
    progress_text: RwSignal<String>,
    instructions_id: StoredValue<String>,
    announce: Callback<SortableEvent, String>,
    progress: Callback<SortableProgress, String>,
    on_move: Callback<SortableWindowMove>,
    /// The one pointer gesture of the list: a press on any row or handle starts it.
    drag: StoredValue<Option<Drag>, LocalStorage>,
    /// The row the press that started it landed on.
    pressed: StoredValue<usize>,
}

pub fn use_sortable_window() -> SortableWindowContext {
    expect_context::<SortableWindowContext>()
}

impl SortableWindowContext {
    /// The id the instruction paragraph carries, for `aria-describedby`.
    pub fn instructions_id(&self) -> String {
        self.instructions_id.get_value()
    }

    /// Whether the row at `index` is the one picked up.
    pub fn is_held(&self, index: usize) -> bool {
        self.plan
            .with(|plan| plan.as_ref().is_some_and(|plan| plan.from == index))
    }

    /// Whether the row at `index` is in the block being moved: the one picked up, or a selected
    /// row that went with it.
    pub fn is_moved(&self, index: usize) -> bool {
        self.plan
            .with(|plan| plan.as_ref().is_some_and(|plan| plan.contains(index)))
    }

    fn element(&self) -> Option<HtmlElement> {
        self.scroller
            .get_untracked()
            .map(|element| element.unchecked_into::<HtmlElement>())
    }

    /// The row last given focus, kept inside the list whatever happened to its length.
    fn current(&self) -> usize {
        self.active.get().min(self.count.get().saturating_sub(1))
    }

    fn key_of(&self, index: usize) -> String {
        untrack(|| self.key.run(index))
    }

    fn label_of(&self, index: usize) -> String {
        untrack(|| {
            let label = self.item_label.map(|label| label.run(index));
            match label {
                Some(label) if !label.is_empty() => label,
                _ => self.key.run(index),
            }
        })
    }

    fn status(&self, gesture: &Gesture, position: usize) -> SortableStatus {
        SortableStatus {
            label: gesture.label.clone(),
            position,
            total: self.count.get_untracked(),
            count: gesture.moved.len(),
        }
    }

    /// Where the picked row would be, counting from 1, if the move ended now.
    fn position_of_anchor(gesture: &Gesture) -> usize {
        gesture.moved.binary_search(&gesture.from).unwrap_or(0) + gesture.to + 1
    }

    /// The rows a pick-up of `index` takes. Found by key over the whole list, so a selected row
    /// that is not in the page is in it.
    fn block_for(&self, index: usize) -> Vec<usize> {
        let selected = self.selected.get_untracked();
        if selected.len() < 2 {
            return vec![index];
        }
        block_of(self.count.get_untracked(), index, |row| {
            selected.contains(&self.key_of(row))
        })
    }

    fn say(&self, event: SortableEvent) {
        self.announcement.set(self.announce.run(event));
    }

    fn say_progress(&self, gesture: &Gesture) {
        let progress = SortableProgress {
            label: gesture.label.clone(),
            count: gesture.moved.len(),
            destination: gesture.to + 1,
            total: self.count.get_untracked(),
        };
        self.progress_text.set(self.progress.run(progress));
    }

    fn sync_scroll(&self) {
        if let Some(element) = self.element() {
            let top = element.scroll_top() as f64;
            if self.scroll_top.get_untracked() != top {
                self.scroll_top.set(top);
            }
        }
    }

    /// Scrolls the scroller, if it has to, so the row at `index` is in view, with `margin` pixels
    /// to spare on either side.
    fn reveal(&self, index: usize, margin: f64) {
        let Some(element) = self.element() else {
            return;
        };
        if let Some(top) = reveal_top_within(
            index,
            element.scroll_top() as f64,
            element.client_height() as f64,
            self.row_height,
            margin,
        ) {
            element.set_scroll_top(top.round() as i32);
            self.sync_scroll();
        }
    }

    /// Gives the row at `index` the tab stop and focus. The row may not be in the page yet — it is
    /// kept there from the moment it has the tab stop — so this scrolls first and then waits for it.
    fn focus_index(&self, index: usize) {
        self.focus_index_with(index, true);
    }

    fn focus_index_with(&self, index: usize, reveal: bool) {
        let index = index.min(self.count.get_untracked().saturating_sub(1));
        self.active.set(index);
        if reveal {
            self.reveal(index, 0.0);
        }
        let ctx = *self;
        request_animation_frame(move || ctx.focus_now(index, 3));
    }

    fn focus_now(&self, index: usize, retries: usize) {
        let found = self.element().and_then(|root| {
            let row = root
                .query_selector(&format!("[{INDEX_ATTR}=\"{index}\"]"))
                .ok()
                .flatten()?;
            let handle = row
                .query_selector(&format!("[{HANDLE_ATTR}]"))
                .ok()
                .flatten();
            handle.unwrap_or(row).dyn_into::<HtmlElement>().ok()
        });

        match found {
            Some(target) => {
                let options = FocusOptions::new();
                options.set_prevent_scroll(true);
                let _ = target.focus_with_options(&options);
            }
            None if retries > 0 => {
                let ctx = *self;
                request_animation_frame(move || ctx.focus_now(index, retries - 1));
            }
            None => {}
        }
    }

    fn focus_inside(&self) -> bool {
        self.element().is_some_and(|root| {
            document()
                .active_element()
                .is_some_and(|active| root.contains(Some(active.as_ref())))
        })
    }

    /// Sets the list's height now, ahead of the reactive style that says the same, so that the
    /// `scrollTop` set next is clamped against the list it is for.
    fn set_list_length(&self, rows: usize, pile: bool) {
        if let Some(list) = self.list.get_untracked() {
            let _ = list.set_attribute("style", &list_style(rows, self.row_height, pile));
        }
    }

    /// Takes the row in hand. `false` if a move is already running, or there is no such row.
    fn pick_up(&self, index: usize, pointer: bool) -> bool {
        if self.gesture.get_untracked().is_some() {
            return false;
        }
        let total = self.count.get_untracked();
        if index >= total {
            return false;
        }
        let Some(element) = self.element() else {
            return false;
        };

        let moved = self.block_for(index);
        let keys: Vec<String> = moved.iter().map(|&row| self.key_of(row)).collect();
        let stack = stack_slot(index, &moved);
        let to = stack;
        let gesture = Gesture {
            from: index,
            to,
            pointer,
            key: self.key_of(index),
            label: self.label_of(index),
            moved: Arc::new(moved),
            keys,
            stack,
        };
        let status = self.status(&gesture, index + 1);

        let mut lift = 0.0;
        if gesture.moved.len() > 1 {
            let hidden_above = index - stack;
            let length = collapsed_len(total, gesture.moved.len());
            self.set_list_length(length, true);
            let before = element.scroll_top() as f64;
            let wanted = collapsed_top(
                before,
                hidden_above,
                self.row_height,
                element.client_height() as f64,
                length,
            );
            element.set_scroll_top(wanted.round() as i32);
            let after = element.scroll_top() as f64;
            lift = (hidden_above as f64 * self.row_height - (before - after)).max(0.0);
            self.sync_scroll();
        }

        self.dy.set(0.0);
        self.shift.set(0.0);
        self.lift.set(lift);
        self.active.set(index);
        self.anchor
            .set(pointer.then(|| ScrollAnchor::capture(element.into())));
        self.say_progress(&gesture);
        self.gesture.set(Some(gesture));
        self.say(SortableEvent::PickedUp(status));
        true
    }

    fn set_to(&self, to: usize) {
        let Some(mut gesture) = self.gesture.get_untracked() else {
            return;
        };
        let to = to.min(
            self.count
                .get_untracked()
                .saturating_sub(gesture.moved.len()),
        );
        if gesture.to == to {
            return;
        }
        gesture.to = to;
        let status = self.status(&gesture, Self::position_of_anchor(&gesture));
        self.say_progress(&gesture);
        self.gesture.set(Some(gesture));
        self.say(SortableEvent::Moved(status));
    }

    fn pointer_move(&self, index: usize, point: DragPoint) {
        self.pointer_y.set(point.client_y);

        if self.gesture.get_untracked().is_none() {
            if point.dx.abs().max(point.dy.abs()) < ACTIVATION_DISTANCE {
                return;
            }
            self.pick_up(index, true);
        }

        let ours = self
            .gesture
            .with_untracked(|g| g.as_ref().is_some_and(|g| g.pointer && g.from == index));
        if !ours {
            return;
        }

        self.dy.set(point.dy);
        self.retarget();
    }

    fn pointer_end(&self, index: usize) {
        let ours = self
            .gesture
            .with_untracked(|g| g.as_ref().is_some_and(|g| g.pointer && g.from == index));
        if ours {
            self.finish(true);
        }
    }

    /// Where the held row is, worked out from where it was picked up and how far it has come.
    fn retarget(&self) {
        let Some(gesture) = self.gesture.get_untracked() else {
            return;
        };
        if !gesture.pointer {
            return;
        }
        let height = self.row_height;
        let held = gesture.stack as f64 * height
            + self.lift.get_untracked()
            + height / 2.0
            + self.dy.get_untracked()
            + self.shift.get_untracked();
        self.set_to(collapsed_drop_index(
            self.count.get_untracked(),
            &gesture.moved,
            gesture.stack,
            held,
            height,
        ));
    }

    /// One 16 ms beat while a pointer holds the row: scroll if it is near an edge, re-measure how
    /// far the scroller has moved, and work out the drop again. On a timer rather than on moves,
    /// because a pointer held still at the edge sends none and is exactly when the reader waits.
    fn tick(&self) {
        self.autoscroll();

        if let Some(anchor) = self.anchor.get_untracked() {
            let (_, shift) = anchor.shift();
            if self.shift.get_untracked() != shift {
                self.shift.set(shift);
            }
        }

        self.retarget();
    }

    fn autoscroll(&self) {
        let Some(element) = self.element() else {
            return;
        };
        let rect = element.get_bounding_client_rect();
        let viewport = window()
            .inner_height()
            .ok()
            .and_then(|size| size.as_f64())
            .unwrap_or(0.0);
        let step = edge_ramp(
            self.pointer_y.get_untracked(),
            rect.top().max(0.0),
            rect.bottom().min(viewport),
            EDGE,
            EDGE_SPEED,
        );
        if step == 0.0 {
            return;
        }
        element.set_scroll_top((element.scroll_top() as f64 + step).round() as i32);
        self.sync_scroll();
    }

    /// Ends the move. `commit` drops it where it is; otherwise everything goes back.
    fn finish(&self, commit: bool) {
        let Some(gesture) = self.gesture.get_untracked() else {
            return;
        };

        let changed = commit && !is_noop(&gesture.moved, gesture.to);
        let landed = if changed {
            Self::position_of_anchor(&gesture) - 1
        } else {
            gesture.from
        };
        let status = self.status(&gesture, landed + 1);
        let refocus = !gesture.pointer || self.focus_inside();

        let total = self.count.get_untracked();
        if gesture.moved.len() > 1 {
            self.set_list_length(total, false);
            if !changed && let Some(element) = self.element() {
                let top = element.scroll_top() as f64;
                let shift = expanded_top_shift(
                    top,
                    self.row_height,
                    &gesture.moved,
                    gesture.to,
                    collapsed_len(total, gesture.moved.len()),
                );
                if shift != 0.0 {
                    element.set_scroll_top((top + shift).round() as i32);
                }
            }
            self.sync_scroll();
        }

        self.gesture.set(None);
        self.progress_text.set(String::new());
        self.anchor.set(None);
        self.dy.set(0.0);
        self.shift.set(0.0);
        self.lift.set(0.0);
        self.active.set(landed);

        if changed {
            self.on_move.run(SortableWindowMove {
                key: gesture.key.clone(),
                from: gesture.from,
                to: gesture.to,
                keys: gesture.keys.clone(),
                indices: gesture.moved.to_vec(),
            });
        }

        self.say(if commit {
            SortableEvent::Dropped(status)
        } else {
            SortableEvent::Cancelled(status)
        });

        if refocus {
            self.focus_index_with(landed, !gesture.pointer || gesture.moved.len() < 2);
        }
    }

    /// Escape, or anything else that ends a pointer move without dropping it.
    fn abort_pointer(&self) {
        if let Some(Some(drag)) = self.drag.try_get_value() {
            drag.stop();
        }
        self.finish(false);
    }

    /// A key on the handle, or on the row when it has none.
    fn key_down(&self, event: &ev::KeyboardEvent, index: usize) {
        if self.disabled.get_untracked() || event.alt_key() || event.ctrl_key() || event.meta_key()
        {
            return;
        }

        let pressed = event.key();
        let last = self.count.get_untracked().saturating_sub(1);

        let Some(gesture) = self.gesture.get_untracked() else {
            let to = match pressed.as_str() {
                " " | "Enter" => {
                    event.prevent_default();
                    self.pick_up(index, false);
                    return;
                }
                "ArrowDown" => (index + 1).min(last),
                "ArrowUp" => index.saturating_sub(1),
                "Home" => 0,
                "End" => last,
                _ => return,
            };
            event.prevent_default();
            self.focus_index(to);
            return;
        };
        if gesture.pointer || gesture.from != index {
            return;
        }

        let last_to = self
            .count
            .get_untracked()
            .saturating_sub(gesture.moved.len());
        let target = match pressed.as_str() {
            "ArrowDown" => Some((gesture.to + 1).min(last_to)),
            "ArrowUp" => Some(gesture.to.saturating_sub(1)),
            "Home" => Some(0),
            "End" => Some(last_to),
            _ => None,
        };

        if let Some(to) = target {
            event.prevent_default();
            self.set_to(to);
            if let Some(now) = self.gesture.get_untracked() {
                let margin = if now.moved.len() > 1 {
                    self.row_height
                } else {
                    0.0
                };
                self.reveal(now.to, margin);
            }
            return;
        }

        match pressed.as_str() {
            " " | "Enter" => {
                event.prevent_default();
                self.finish(true);
            }
            "Escape" => {
                event.prevent_default();
                event.stop_propagation();
                self.finish(false);
            }
            _ => {}
        }
    }

    /// The row, or its handle, took focus: it is the one to keep in the page and to start the arrows from.
    fn focused(&self, index: usize) {
        if self.active.get_untracked() != index {
            self.active.set(index);
        }
    }

    /// Focus left the row in the middle of a keyboard move.
    fn blurred(&self, index: usize) {
        let ours = self
            .gesture
            .with_untracked(|g| g.as_ref().is_some_and(|g| !g.pointer && g.from == index));
        if ours {
            self.finish(false);
        }
    }

    /// A press anywhere in the list. One listener serves every row: the row, and whether the press
    /// is on its handle, are read off the node it landed on.
    fn pointer_down(&self, event: &ev::PointerEvent) {
        let Some(hit) = Hit::of(event.target()) else {
            return;
        };
        if !(hit.in_handle || hit.plain && !from_a_control(event)) {
            return;
        }
        if self.disabled.get_untracked() || event.button() != 0 {
            return;
        }
        let Some(Some(drag)) = self.drag.try_get_value() else {
            return;
        };
        self.pressed.set_value(hit.index);
        drag.start(event);
    }

    fn key_press(&self, event: &ev::KeyboardEvent) {
        if let Some(hit) = Hit::of(event.target()).filter(|hit| hit.on_stop) {
            self.key_down(event, hit.index);
        }
    }

    fn focus_in(&self, event: &ev::FocusEvent) {
        if let Some(hit) = Hit::of(event.target()).filter(|hit| hit.on_stop) {
            self.focused(hit.index);
        }
    }

    fn focus_out(&self, event: &ev::FocusEvent) {
        if let Some(hit) = Hit::of(event.target()).filter(|hit| hit.on_stop) {
            self.blurred(hit.index);
        }
    }
}

/// How much room a pile's layers and shadow take below the row, which the last row of the list
/// has nowhere to put when it is the pile.
const PILE_ROOM: f64 = 12.0;

/// The list's own style: as tall as its rows, and, while a pile is held, with room under the last
/// of them for the pile's layers.
fn list_style(rows: usize, row_height: f64, pile: bool) -> String {
    format!(
        "position: relative; height: {}px;{}",
        rows as f64 * row_height,
        if pile {
            format!(" margin-bottom: {PILE_ROOM}px;")
        } else {
            String::new()
        }
    )
}

/// Where an event landed: which row, and in what part of it.
struct Hit {
    index: usize,
    /// On the handle, or inside it.
    in_handle: bool,
    /// On the row's own node when it has no handle, or on the handle's: a place that takes focus
    /// and keys, and not something inside them.
    on_stop: bool,
    /// The row has no handle: it is what is dragged and what takes focus.
    plain: bool,
}

impl Hit {
    fn of(target: Option<web_sys::EventTarget>) -> Option<Self> {
        let node = target?.dyn_into::<Element>().ok()?;
        let handle = node.closest(HANDLE_SELECTOR).ok().flatten();
        let row = handle
            .as_ref()
            .unwrap_or(&node)
            .closest(ROW_SELECTOR)
            .ok()
            .flatten()?;
        let index = row.get_attribute(INDEX_ATTR)?.parse().ok()?;
        let plain =
            handle.is_none() && row.query_selector(HANDLE_SELECTOR).ok().flatten().is_none();
        let on_stop = match &handle {
            Some(handle) => *handle == node,
            None => plain && row == node,
        };
        Some(Self {
            index,
            in_handle: handle.is_some(),
            on_stop,
            plain,
        })
    }
}

/// The list. Renders the instructions, the live region and the scroller its rows are in.
#[component]
pub fn SortableWindowRoot(
    /// The scroller's classes. Give it a height; the window is as many rows as that holds.
    #[prop(optional, into)]
    class: Signal<String>,
    /// How many rows the list has.
    #[prop(into)]
    count: Signal<usize>,
    /// The height of every row, in CSS pixels. Fixed: a row's place is its index times this.
    row_height: f64,
    /// How many rows to keep in the page above and below the ones in view.
    #[prop(default = 6)]
    overscan: usize,
    /// Names the row at an index in a [`SortableWindowMove`]. Reads the caller's data.
    #[prop(into)]
    key: Callback<usize, String>,
    /// What a screen reader is told the row at an index is called. Without one, or when it says nothing, its key.
    #[prop(optional, into)]
    item_label: Option<Callback<usize, String>>,
    /// The content of the row at an index. Re-run when a signal it reads changes, which is how a
    /// reorder reaches the rows: they are positional, and this is what is in them.
    #[prop(into)]
    row: Callback<usize, AnyView>,
    /// The classes of every row.
    #[prop(optional, into)]
    row_class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    /// The list's accessible name.
    #[prop(optional, into)]
    label: Signal<String>,
    /// What a row is described by. Empty means [`DEFAULT_INSTRUCTIONS`].
    #[prop(optional, into)]
    instructions: Signal<String>,
    /// Replaces [`default_announcement`]. Presence-sensitive: unset keeps the English wording.
    #[prop(default = None, into)]
    announce: Option<Callback<SortableEvent, String>>,
    /// Replaces [`default_progress`], the polite live region that says what the move in progress
    /// is while it lasts. Presence-sensitive: unset keeps the English wording.
    #[prop(default = None, into)]
    progress: Option<Callback<SortableProgress, String>>,
    /// The keys, as `key` names them, that are selected. Picking up one of them takes all of
    /// them, as one block, whether or not they are in the page.
    #[prop(optional, into)]
    selected: Signal<HashSet<String>>,
    /// The classes of the marker that stands in the slot a pointer-held block will land in. It
    /// fills that slot (it is a row tall, positioned and translated by inline style), and it is
    /// only in the page while a block of more than one is held.
    #[prop(optional, into)]
    drop_class: Signal<String>,
    /// A row, or a block, was let go somewhere new. The caller applies it, with [`SortableWindowMove::apply`].
    #[prop(into)]
    on_move: Callback<SortableWindowMove>,
) -> impl IntoView {
    let scroller = NodeRef::<html::Div>::new();
    let gesture = RwSignal::new(None::<Gesture>);
    let plan = Memo::new(move |_| {
        gesture.with(|g| {
            g.as_ref().map(|g| Plan {
                from: g.from,
                to: g.to,
                pointer: g.pointer,
                moved: g.moved.clone(),
                stack: g.stack,
            })
        })
    });
    let marked = Memo::new(move |_| plan.with(|plan| plan.as_ref().is_some_and(Plan::marked)));
    let stacked = Memo::new(move |_| {
        plan.with(|plan| plan.as_ref().is_some_and(|plan| plan.moved.len() > 1))
    });
    let length = Memo::new(move |_| {
        let total = count.get();
        plan.with(|plan| {
            plan.as_ref()
                .map_or(total, |plan| collapsed_len(total, plan.moved.len()))
        })
    });

    let context = SortableWindowContext {
        count,
        row_height,
        disabled,
        key,
        item_label,
        selected,
        gesture,
        plan,
        dy: RwSignal::new(0.0),
        shift: RwSignal::new(0.0),
        lift: RwSignal::new(0.0),
        pointer_y: RwSignal::new(0.0),
        anchor: RwSignal::new(None),
        active: RwSignal::new(0),
        scroll_top: RwSignal::new(0.0),
        height: RwSignal::new(0.0),
        scroller,
        list: NodeRef::new(),
        announcement: RwSignal::new(String::new()),
        progress_text: RwSignal::new(String::new()),
        instructions_id: StoredValue::new(next_id("sortable-window-instructions")),
        announce: announce.unwrap_or_else(|| Callback::new(|e| default_announcement(&e))),
        progress: progress.unwrap_or_else(|| Callback::new(|p| default_progress(&p))),
        on_move,
        drag: StoredValue::new_local(None),
        pressed: StoredValue::new(0),
    };

    let drag = use_drag(move |point| context.pointer_move(context.pressed.get_value(), point))
        .on_end(move |_| context.pointer_end(context.pressed.get_value()));
    context.drag.set_value(Some(drag));

    let range = Memo::new(move |_| {
        visible_range(
            context.scroll_top.get(),
            context.height.get(),
            length.get(),
            row_height,
            overscan,
        )
    });

    // The rows in the page, by the index the data has them at: the ones in view (by the slot they
    // are *shown* at in the collapsed list, which is not their index while a block is held), the
    // held row wherever it has been carried, and the row with the tab stop. The rest of a block
    // are not in the page at all. In index order, so a row is never moved in the
    // DOM — a node that moves loses focus.
    let indices = Memo::new(move |_| {
        let total = count.get();
        if total == 0 {
            return Vec::new();
        }
        let window = range.get();
        let current = context.current();
        let Some(plan) = plan.get() else {
            let mut rows: Vec<usize> = window.clone().collect();
            if !window.contains(&current) {
                rows.insert(rows.partition_point(|row| *row < current), current);
            }
            return rows;
        };
        let mut rows: Vec<usize> = window.filter_map(|slot| plan.row_at(slot)).collect();
        rows.push(plan.from);
        if !plan.contains(current) {
            rows.push(current);
        }
        rows.retain(|row| *row < total);
        rows.sort_unstable();
        rows.dedup();
        rows
    });

    let observer = StoredValue::new_local(None::<ResizeObserver>);
    scroller.on_load(move |element| {
        let element: HtmlElement = element.unchecked_into();
        context.height.set(element.client_height() as f64);

        let measured = element.clone();
        let on_resize = Closure::<dyn FnMut()>::new(move || {
            context.height.try_set(measured.client_height() as f64);
        });
        if let Ok(found) = ResizeObserver::new(on_resize.as_ref().unchecked_ref()) {
            found.observe(&element);
            observer.set_value(Some(found));
        }
        on_resize.forget();
    });
    on_cleanup(move || {
        if let Some(Some(found)) = observer.try_update_value(|slot| slot.take()) {
            found.disconnect();
        }
    });

    let held = Memo::new(move |_| gesture.with(|g| g.as_ref().is_some_and(|g| g.pointer)));
    Effect::new(move |previous: Option<Option<IntervalHandle>>| {
        if let Some(Some(handle)) = previous {
            handle.clear();
        }
        if !held.get() {
            return None;
        }
        set_interval_with_handle(move || context.tick(), Duration::from_millis(16)).ok()
    });

    let escape = window_event_listener(ev::keydown, move |event| {
        let pointer = gesture
            .try_with_untracked(|g| g.as_ref().is_some_and(|g| g.pointer))
            .unwrap_or(false);
        if pointer && event.key() == "Escape" {
            context.abort_pointer();
        }
    });
    on_cleanup(move || escape.remove());

    let text = move || {
        let text = instructions.get();
        if text.is_empty() {
            DEFAULT_INSTRUCTIONS.to_string()
        } else {
            text
        }
    };

    view! {
        <p id=context.instructions_id() class="sr-only">
            {text}
        </p>
        <div aria-live="polite" aria-atomic="true" class="sr-only">
            {move || context.progress_text.get()}
        </div>
        <div aria-live="assertive" aria-atomic="true" class="sr-only">
            {move || context.announcement.get()}
        </div>
        <div
            node_ref=scroller
            data-slot="sortable-window"
            data-orientation="vertical"
            data-dragging=move || gesture.with(|g| g.is_some()).then_some("true")
            data-disabled=move || disabled.get().then_some("true")
            aria-disabled=move || disabled.get().then_some("true")
            style="position: relative; overflow-y: auto; overscroll-behavior: contain; overflow-anchor: none;"
            on:scroll=move |_| context.sync_scroll()
            on:pointerdown=move |e: ev::PointerEvent| context.pointer_down(&e)
            on:keydown=move |e: ev::KeyboardEvent| context.key_press(&e)
            on:focusin=move |e: ev::FocusEvent| context.focus_in(&e)
            on:focusout=move |e: ev::FocusEvent| context.focus_out(&e)
            class=class
        >
            <div
                node_ref=context.list
                role="list"
                data-slot="sortable-window-list"
                aria-label=move || Some(label.get()).filter(|label| !label.is_empty())
                style=move || list_style(length.get(), row_height, stacked.get())
            >
                <Provider value=context>
                    <For each=move || indices.get() key=|index| *index let:index>
                        <SortableWindowRow index=index row=row class=row_class />
                    </For>
                </Provider>
                {move || {
                    marked
                        .get()
                        .then(|| {
                            view! {
                                <div
                                    data-slot="sortable-window-drop"
                                    aria-hidden="true"
                                    class=drop_class
                                    style=move || {
                                        let to = plan
                                            .with(|plan| plan.as_ref().map_or(0, |plan| plan.to));
                                        format!(
                                            "position: absolute; top: 0; left: 0; right: 0; height: {row_height}px; pointer-events: none; transform: translate3d(0, {}px, 0);",
                                            to as f64 * row_height,
                                        )
                                    }
                                ></div>
                            }
                        })
                }}
            </div>
        </div>
    }
}

#[derive(Copy, Clone)]
pub struct SortableWindowRowContext {
    pub index: usize,
    pub has_handle: RwSignal<bool>,
    pub disabled: Signal<bool>,
    saw_handle: StoredValue<bool>,
}

pub fn use_sortable_window_row() -> SortableWindowRowContext {
    expect_context::<SortableWindowRowContext>()
}

/// Where a row is drawn.
#[derive(Clone, Copy, PartialEq, Default)]
enum Place {
    #[default]
    Unset,
    Slot(usize),
    /// Riding the pointer, at this offset from the top of the list.
    Riding(f64),
    /// In the block that is held but not the stack: folded into it, and not drawn.
    Hidden,
}

/// Everything about a row that is written to its node, so a change writes only what differs.
#[derive(Clone, PartialEq, Default)]
struct RowLook {
    size: usize,
    moved: bool,
    anchor: bool,
    badge: usize,
    selected: bool,
    following: bool,
    disabled: bool,
    plain: bool,
    place: Place,
}

fn flag(node: &HtmlElement, name: &str, on: bool) {
    let _ = if on {
        node.set_attribute(name, "true")
    } else {
        node.remove_attribute(name)
    };
}

impl RowLook {
    fn read(ctx: &SortableWindowContext, index: usize, plain: bool) -> Self {
        let disabled = ctx.disabled.get();
        let size = ctx.count.get();
        let selected = ctx.selected.with(|set| {
            !set.is_empty() && {
                let key = ctx.key.run(index);
                set.contains(&key)
            }
        });
        let height = ctx.row_height;
        let mut look = Self {
            size,
            selected,
            disabled,
            plain,
            place: Place::Slot(index),
            ..Self::default()
        };
        let Some(plan) = ctx.plan.get() else {
            return look;
        };
        look.moved = plan.contains(index);
        look.anchor = plan.from == index;
        look.following = plan.pointer && look.anchor;
        if look.anchor && plan.moved.len() > 1 {
            look.badge = plan.moved.len();
        }
        look.place = if look.moved && !look.anchor {
            Place::Hidden
        } else if look.following {
            Place::Riding(
                plan.stack as f64 * height + ctx.lift.get() + ctx.dy.get() + ctx.shift.get(),
            )
        } else {
            Place::Slot(plan.slot(index))
        };
        look
    }

    fn write(&self, ctx: &SortableWindowContext, node: &HtmlElement, before: &Self) {
        if self.size != before.size {
            let _ = node.set_attribute("aria-setsize", &self.size.to_string());
        }
        if self.moved != before.moved {
            flag(node, "data-dragging", self.moved);
        }
        if self.anchor != before.anchor {
            flag(node, "data-anchor", self.anchor);
        }
        if self.badge != before.badge {
            let _ = if self.badge > 0 {
                node.set_attribute("data-count", &self.badge.to_string())
            } else {
                node.remove_attribute("data-count")
            };
        }
        if self.selected != before.selected {
            flag(node, "data-selected", self.selected);
        }
        if self.following != before.following {
            flag(node, "data-following", self.following);
        }
        if self.disabled != before.disabled {
            flag(node, "data-disabled", self.disabled);
            flag(node, "aria-disabled", self.disabled);
        }
        if self.plain != before.plain {
            let _ = if self.plain {
                node.set_attribute("aria-roledescription", "sortable")
                    .and_then(|_| node.set_attribute("aria-describedby", &ctx.instructions_id()))
            } else {
                node.remove_attribute("aria-roledescription")
                    .and_then(|_| node.remove_attribute("aria-describedby"))
            };
        }
        let tabbable = self.plain && !self.disabled;
        if tabbable != (before.plain && !before.disabled) {
            let _ = if tabbable {
                node.set_attribute("tabindex", "0")
            } else {
                node.remove_attribute("tabindex")
            };
        }
        if self.plain != before.plain || self.place != before.place {
            let height = ctx.row_height;
            let touch = if self.plain {
                " touch-action: none; user-select: none;"
            } else {
                ""
            };
            let style = match self.place {
                Place::Unset => String::new(),
                Place::Hidden => "display: none;".to_string(),
                Place::Slot(slot) => format!(
                    "position: absolute; top: 0; left: 0; right: 0; height: {height}px;{touch} transform: translate3d(0, {}px, 0);",
                    slot as f64 * height
                ),
                Place::Riding(y) => format!(
                    "position: absolute; top: 0; left: 0; right: 0; height: {height}px;{touch} transform: translate3d(0, {y}px, 0); pointer-events: none; will-change: transform;"
                ),
            };
            let _ = node.set_attribute("style", &style);
        }
    }
}

/// One row, at the place a multiplication gives it. Rendered once per index in the window and
/// kept: what is in it is `row(index)`, which follows the caller's data.
///
/// A row is one node, one effect for everything the gesture and the selection say about it, one
/// for its content and one for its classes, and no listeners: the list's own serve every row.
#[component]
fn SortableWindowRow(
    index: usize,
    row: Callback<usize, AnyView>,
    class: Signal<String>,
) -> impl IntoView {
    let ctx = use_sortable_window();

    // Assumed to have a handle until the content has been built without one, so that the common
    // row is written once.
    let has_handle = RwSignal::new(true);
    let saw_handle = StoredValue::new(false);
    provide_context(SortableWindowRowContext {
        index,
        has_handle,
        disabled: ctx.disabled,
        saw_handle,
    });

    let paint = move |node: web_sys::Element| {
        let node: HtmlElement = node.unchecked_into();
        let effect = RenderEffect::new(move |before: Option<RowLook>| {
            let look = RowLook::read(&ctx, index, !has_handle.get());
            look.write(&ctx, &node, &before.unwrap_or_default());
            look
        });
        StoredValue::new_local(effect);
    };

    view! {
        <div
            role="listitem"
            data-slot="sortable-window-item"
            data-index=index
            aria-posinset=index + 1
            use:paint
            class=class
        >
            {move || {
                saw_handle.set_value(false);
                let content = row.run(index);
                if !saw_handle.get_value() && has_handle.get_untracked() {
                    has_handle.set(false);
                }
                content
            }}
        </div>
    }
}

/// What the handle says about itself, written only where it changes.
#[derive(Clone, PartialEq, Default)]
struct HandleLook {
    name: String,
    held: bool,
    disabled: bool,
}

/// The part of a row that starts a drag and takes focus. A `role="button"`, not a `<button>`:
/// Space on a real button clicks on key *up*, after the pick-up has already used the key.
#[component]
pub fn SortableWindowHandleRoot(
    #[prop(optional, into)] class: Signal<String>,
    /// The handle's accessible name. Without one: "Reorder", and the row's label.
    #[prop(optional, into)]
    label: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_sortable_window();
    let item = use_sortable_window_row();
    let index = item.index;

    item.saw_handle.set_value(true);
    if !item.has_handle.get_untracked() {
        item.has_handle.set(true);
    }
    on_cleanup(move || {
        let _ = item.has_handle.try_set(false);
    });

    let described = ctx.instructions_id();
    let paint = move |node: web_sys::Element| {
        let node: HtmlElement = node.unchecked_into();
        let effect = RenderEffect::new(move |before: Option<HandleLook>| {
            let name = {
                let label = label.get();
                if !label.is_empty() {
                    label
                } else {
                    let item_label = ctx.label_of(index);
                    if item_label.is_empty() {
                        "Reorder".to_string()
                    } else {
                        format!("Reorder {item_label}")
                    }
                }
            };
            let look = HandleLook {
                name,
                held: ctx.is_held(index),
                disabled: item.disabled.get(),
            };
            let first = before.is_none();
            let before = before.unwrap_or_default();
            if look.name != before.name {
                let _ = node.set_attribute("aria-label", &look.name);
            }
            if look.held != before.held {
                flag(&node, "aria-pressed", look.held);
                flag(&node, "data-dragging", look.held);
            }
            if look.disabled != before.disabled {
                flag(&node, "aria-disabled", look.disabled);
                flag(&node, "data-disabled", look.disabled);
            }
            if first || look.disabled != before.disabled {
                let _ = node.set_attribute("tabindex", if look.disabled { "-1" } else { "0" });
            }
            look
        });
        StoredValue::new_local(effect);
    };

    view! {
        <span
            role="button"
            data-slot="sortable-window-handle"
            data-sortable-handle=""
            aria-roledescription="sortable"
            aria-describedby=described
            style="touch-action: none; user-select: none;"
            use:paint
            class=class
        >
            {children()}
        </span>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::sortable::{drop_index as measured_drop_index, preview_offsets};

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
        crate::primitives::sortable::apply_move(&mut again, &moved.as_move(), |s| s.clone());
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
                crate::primitives::sortable::apply_move(&mut by_key, &report.as_move(), |s| {
                    s.clone()
                });
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
