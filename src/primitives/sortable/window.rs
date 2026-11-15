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
//! against [`slot_of`], [`row_at`] and [`drop_index`] for a block of one.
//!
//! While a pointer holds the block the rows that are in the page ride under the held row, which
//! wears `data-count`; the others are where they were. A keyboard pick-up with a selection moves the
//! block too, and every announcement says "N items" as the measured list's do.
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
//! held, the arrows, Home and End move *focus*: only one row is in the tab order (a roving
//! tabindex by index), and moving it to a row that is not in the page scrolls it in first, by
//! setting `scrollTop` — there is no element to `scrollIntoView` yet. The row that has the tab
//! stop is always kept in the page, so Tab can find it however far the list has scrolled.
//!
//! # What a screen reader hears
//!
//! The same live region, instructions and wording as [`super::SortableRoot`] —
//! [`super::default_announcement`], replaceable with `announce` — with the item's name taken from
//! `item_label` rather than from DOM text, since the row may not be rendered when it is wanted.
//! Rows carry `aria-setsize` and `aria-posinset`, because most of the list is not in the page.
//!
//! # Data attributes
//!
//! - scroller: `data-dragging` (a move is in progress), `data-disabled`, `data-orientation`;
//! - row: `data-index`, `data-dragging` (it is in the block being moved), `data-following` (it is
//!   riding the pointer, so it must not ease), `data-placeholder` (a keyboard move: it is sitting
//!   where it would land), `data-anchor` (it is the one picked), `data-count` (on the anchor, when
//!   there are several), `data-selected`, `data-disabled`;
//! - handle: `data-dragging`, `data-disabled`.
//!
//! Every row has an inline `transform`, which is its place. Easing it is the styled layer's
//! business, switched on only while the scroller has `data-dragging`, so the rows are put where
//! they belong instantly on a drop, in the same frame the caller's reorder lands.

use super::{
    ACTIVATION_DISTANCE, DEFAULT_INSTRUCTIONS, SortableEvent, SortableMove, SortableStatus,
    default_announcement, from_a_control, is_noop, move_block,
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
use web_sys::{FocusOptions, HtmlElement, ResizeObserver};

/// How near the scroller's edge a held row has to be before it starts scrolling.
const EDGE: f64 = 72.0;
/// The most it scrolls in one 16 ms tick, at the very edge.
const EDGE_SPEED: f64 = 16.0;

const INDEX_ATTR: &str = "data-index";
const HANDLE_ATTR: &str = "data-sortable-handle";

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
    // The `rest`-th row that is not in hand: every moved row `m` with `m - rank <= rest` (the rows
    // before it that stay) is ahead of it. `m - rank` never falls, so this is a binary search.
    let (mut low, mut high) = (0, count);
    while low < high {
        let mid = (low + high) / 2;
        if moved[mid] - mid <= rest {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    rest + low
}

/// How far to set `scrollTop` so the row at `index` is in view, moving as little as possible
/// (`block: "nearest"`); `None` when it already is.
pub fn reveal_top(index: usize, top: f64, viewport: f64, row_height: f64) -> Option<f64> {
    let start = index as f64 * row_height;
    let end = start + row_height;
    if start < top {
        Some(start)
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
}

/// All a row needs to know about the gesture, and nothing it would have to re-read to know it.
#[derive(Clone, Debug)]
struct Plan {
    from: usize,
    to: usize,
    pointer: bool,
    moved: Arc<Vec<usize>>,
}

impl PartialEq for Plan {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from
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
        block_slot_of(row, &self.moved, self.to)
    }

    fn row_at(&self, slot: usize) -> usize {
        block_row_at(slot, &self.moved, self.to)
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
    pointer_y: RwSignal<f64>,
    anchor: RwSignal<Option<ScrollAnchor>>,
    /// The row with the tab stop. Always in the page.
    active: RwSignal<usize>,
    scroll_top: RwSignal<f64>,
    height: RwSignal<f64>,
    scroller: NodeRef<html::Div>,
    announcement: RwSignal<String>,
    instructions_id: StoredValue<String>,
    announce: Callback<SortableEvent, String>,
    on_move: Callback<SortableWindowMove>,
    aborter: StoredValue<Option<Drag>, LocalStorage>,
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

    /// The row with the tab stop, kept inside the list whatever happened to its length.
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

    fn sync_scroll(&self) {
        if let Some(element) = self.element() {
            let top = element.scroll_top() as f64;
            if self.scroll_top.get_untracked() != top {
                self.scroll_top.set(top);
            }
        }
    }

    /// Scrolls the scroller, if it has to, so the row at `index` is in view.
    fn reveal(&self, index: usize) {
        let Some(element) = self.element() else {
            return;
        };
        if let Some(top) = reveal_top(
            index,
            element.scroll_top() as f64,
            element.client_height() as f64,
            self.row_height,
        ) {
            element.set_scroll_top(top.round() as i32);
            self.sync_scroll();
        }
    }

    /// Gives the row at `index` the tab stop and focus. The row may not be in the page yet — it is
    /// kept there from the moment it has the tab stop — so this scrolls first and then waits for it.
    fn focus_index(&self, index: usize) {
        let index = index.min(self.count.get_untracked().saturating_sub(1));
        self.active.set(index);
        self.reveal(index);
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
        let to = moved.first().copied().unwrap_or(index);
        let gesture = Gesture {
            from: index,
            to,
            pointer,
            key: self.key_of(index),
            label: self.label_of(index),
            moved: Arc::new(moved),
            keys,
        };
        let status = self.status(&gesture, index + 1);

        self.dy.set(0.0);
        self.shift.set(0.0);
        self.active.set(index);
        self.anchor
            .set(pointer.then(|| ScrollAnchor::capture(element.into())));
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
        let held = gesture.from as f64 * height
            + height / 2.0
            + self.dy.get_untracked()
            + self.shift.get_untracked();
        self.set_to(block_drop_index(
            self.count.get_untracked(),
            &gesture.moved,
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

        self.gesture.set(None);
        self.anchor.set(None);
        self.dy.set(0.0);
        self.shift.set(0.0);
        self.aborter.set_value(None);
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
            self.focus_index(landed);
        }
    }

    /// Escape, or anything else that ends a pointer move without dropping it.
    fn abort_pointer(&self) {
        if let Some(drag) = self.aborter.get_value() {
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
        if event.target() != event.current_target() {
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
                self.reveal(Self::position_of_anchor(&now) - 1);
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

    /// Focus left the row in the middle of a keyboard move.
    fn blurred(&self, index: usize) {
        let ours = self
            .gesture
            .with_untracked(|g| g.as_ref().is_some_and(|g| !g.pointer && g.from == index));
        if ours {
            self.finish(false);
        }
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
    /// The keys, as `key` names them, that are selected. Picking up one of them takes all of
    /// them, as one block, whether or not they are in the page.
    #[prop(optional, into)]
    selected: Signal<HashSet<String>>,
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
            })
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
        pointer_y: RwSignal::new(0.0),
        anchor: RwSignal::new(None),
        active: RwSignal::new(0),
        scroll_top: RwSignal::new(0.0),
        height: RwSignal::new(0.0),
        scroller,
        announcement: RwSignal::new(String::new()),
        instructions_id: StoredValue::new(next_id("sortable-window-instructions")),
        announce: announce.unwrap_or_else(|| Callback::new(|e| default_announcement(&e))),
        on_move,
        aborter: StoredValue::new_local(None),
    };

    let range = Memo::new(move |_| {
        visible_range(
            context.scroll_top.get(),
            context.height.get(),
            count.get(),
            row_height,
            overscan,
        )
    });

    // The rows in the page, by the index the data has them at: the ones in view (by the slot they
    // are *shown* at, which is not their index while a row is held), the held row wherever it has
    // been carried, and the row with the tab stop. In index order, so a row is never moved in the
    // DOM — a node that moves loses focus.
    let indices = Memo::new(move |_| {
        let total = count.get();
        let plan = plan.get();
        let mut rows: Vec<usize> = match &plan {
            Some(plan) => range.get().map(|slot| plan.row_at(slot)).collect(),
            None => range.get().collect(),
        };
        if let Some(plan) = &plan {
            rows.push(plan.from);
        }
        if total > 0 {
            rows.push(context.current());
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
            style="position: relative; overflow-y: auto; overscroll-behavior: contain;"
            on:scroll=move |_| context.sync_scroll()
            class=class
        >
            <div
                role="list"
                data-slot="sortable-window-list"
                aria-label=move || Some(label.get()).filter(|label| !label.is_empty())
                style=move || {
                    format!("position: relative; height: {}px;", count.get() as f64 * row_height)
                }
            >
                <Provider value=context>
                    <For each=move || indices.get() key=|index| *index let:index>
                        <SortableWindowRow index=index row=row class=row_class />
                    </For>
                </Provider>
            </div>
        </div>
    }
}

#[derive(Copy, Clone)]
pub struct SortableWindowRowContext {
    pub index: usize,
    pub has_handle: RwSignal<bool>,
    pub disabled: Signal<bool>,
    drag: Drag,
}

pub fn use_sortable_window_row() -> SortableWindowRowContext {
    expect_context::<SortableWindowRowContext>()
}

impl SortableWindowRowContext {
    fn press(&self, event: &ev::PointerEvent) {
        if self.disabled.get_untracked() || event.button() != 0 {
            return;
        }
        self.drag.start(event);
    }
}

/// One row, at the place a multiplication gives it. Rendered once per index in the window and
/// kept: what is in it is `row(index)`, which follows the caller's data.
#[component]
fn SortableWindowRow(
    index: usize,
    row: Callback<usize, AnyView>,
    class: Signal<String>,
) -> impl IntoView {
    let ctx = use_sortable_window();
    let height = ctx.row_height;
    let off = ctx.disabled;

    let gesture: StoredValue<Option<Drag>, LocalStorage> = StoredValue::new_local(None);
    let drag = use_drag(move |point| ctx.pointer_move(index, point))
        .on_start(move |_| ctx.aborter.set_value(gesture.get_value()))
        .on_end(move |_| ctx.pointer_end(index));
    gesture.set_value(Some(drag));

    let has_handle = RwSignal::new(false);
    let item = SortableWindowRowContext {
        index,
        has_handle,
        disabled: off,
        drag,
    };

    let held = Memo::new(move |_| ctx.is_held(index));
    let moved = Memo::new(move |_| ctx.is_moved(index));
    let following = Memo::new(move |_| {
        ctx.plan.with(|plan| {
            plan.as_ref()
                .is_some_and(|plan| plan.pointer && plan.contains(index))
        })
    });
    let slot = Memo::new(move |_| {
        ctx.plan
            .with(|plan| plan.as_ref().map_or(index, |plan| plan.slot(index)))
    });
    let count = Memo::new(move |_| {
        ctx.plan.with(|plan| {
            plan.as_ref()
                .filter(|plan| plan.from == index)
                .map(|plan| plan.moved.len())
                .filter(|count| *count > 1)
        })
    });
    let selected = move || {
        ctx.selected.with(|set| {
            !set.is_empty() && {
                let key = ctx.key.run(index);
                set.contains(&key)
            }
        })
    };
    let described = ctx.instructions_id();

    view! {
        <div
            role="listitem"
            data-slot="sortable-window-item"
            data-index=index
            data-sortable-item=move || ctx.key_of(index)
            aria-setsize=move || ctx.count.get()
            aria-posinset=index + 1
            data-dragging=move || moved.get().then_some("true")
            data-anchor=move || held.get().then_some("true")
            data-count=move || count.get().map(|count| count.to_string())
            data-selected=move || selected().then_some("true")
            data-following=move || following.get().then_some("true")
            data-placeholder=move || (moved.get() && !following.get()).then_some("true")
            data-disabled=move || off.get().then_some("true")
            aria-disabled=move || off.get().then_some("true")
            tabindex=move || {
                if has_handle.get() {
                    None
                } else if !off.get() && ctx.current() == index {
                    Some("0")
                } else {
                    Some("-1")
                }
            }
            aria-roledescription=move || (!has_handle.get()).then_some("sortable")
            aria-describedby=move || (!has_handle.get()).then(|| described.clone())
            style=move || {
                let base = format!(
                    "position: absolute; top: 0; left: 0; right: 0; height: {height}px;{}",
                    if has_handle.get() { "" } else { " touch-action: none; user-select: none;" },
                );
                if following.get() {
                    let from = ctx.plan.with(|plan| plan.as_ref().map_or(index, |plan| plan.from));
                    let y = from as f64 * height + ctx.dy.get() + ctx.shift.get();
                    format!(
                        "{base} transform: translate3d(0, {y}px, 0); pointer-events: none; will-change: transform;",
                    )
                } else {
                    format!(
                        "{base} transform: translate3d(0, {}px, 0);",
                        slot.get() as f64 * height,
                    )
                }
            }
            on:pointerdown=move |e: ev::PointerEvent| {
                if has_handle.get_untracked() || from_a_control(&e) {
                    return;
                }
                item.press(&e);
            }
            on:keydown=move |e: ev::KeyboardEvent| {
                if !has_handle.get_untracked() {
                    ctx.key_down(&e, index);
                }
            }
            on:blur=move |_| {
                if !has_handle.get_untracked() {
                    ctx.blurred(index);
                }
            }
            class=class
        >
            <Provider value=item>{move || row.run(index)}</Provider>
        </div>
    }
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

    item.has_handle.set(true);
    on_cleanup(move || {
        let _ = item.has_handle.try_set(false);
    });

    let name = move || {
        let label = label.get();
        if !label.is_empty() {
            return label;
        }
        let item_label = ctx.label_of(index);
        if item_label.is_empty() {
            "Reorder".to_string()
        } else {
            format!("Reorder {item_label}")
        }
    };
    let held = move || ctx.is_held(index);
    let described = ctx.instructions_id();

    view! {
        <span
            role="button"
            data-slot="sortable-window-handle"
            data-sortable-handle=""
            aria-label=name
            aria-roledescription="sortable"
            aria-describedby=described
            aria-pressed=move || held().then_some("true")
            aria-disabled=move || item.disabled.get().then_some("true")
            data-disabled=move || item.disabled.get().then_some("true")
            data-dragging=move || held().then_some("true")
            tabindex=move || {
                if !item.disabled.get() && ctx.current() == index { "0" } else { "-1" }
            }
            style="touch-action: none; user-select: none;"
            on:pointerdown=move |e: ev::PointerEvent| item.press(&e)
            on:keydown=move |e: ev::KeyboardEvent| ctx.key_down(&e, index)
            on:blur=move |_| ctx.blurred(index)
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
