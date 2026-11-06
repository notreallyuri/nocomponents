//! A list the reader can put in order — with a pointer, a finger or the keyboard alone.
//!
//! **The list does not own the order.** Like [`crate::primitives::kanban`] it reports what
//! happened — [`SortableMove`], the keys that moved and where they landed — and the caller applies
//! it to their own data. [`apply_move`] is that application, written once and tested, so nobody
//! re-derives which index a block of items lands at after the items ahead of it have left.
//!
//! Items are found in the DOM rather than registered: the order on screen is the order, and a list
//! can add, remove and reorder items without telling anybody. Keys are strings, as they are on the
//! kanban board — they travel through `data-*` attributes and back.
//!
//! # Three ways to move
//!
//! - **Pointer and touch** go through [`crate::primitives::drag::use_drag`]. A press becomes a
//!   drag after [`ACTIVATION_DISTANCE`] pixels, so a click on an item still clicks. The picked
//!   item follows the pointer by a transform; every other item slides by a transform to show
//!   where the drop would land. Nothing is moved in the DOM until the release, which also means
//!   nothing reflows under the gesture. Escape and `pointercancel` put everything back.
//! - **Keyboard**, on the handle (or on the item, when it has none): Space or Enter picks up,
//!   the arrows along the list's axis move, Home and End jump, Space or Enter drops, Escape
//!   cancels. Losing focus also cancels, so a half-finished move cannot be left behind. Focus is
//!   returned to the moved item after a drop, since reordering the DOM would otherwise drop it.
//! - **Block drag.** When the picked item is in `selected`, everything selected moves with it,
//!   keeping its relative order, and lands as one block. The picked item carries the pointer; the
//!   rest ride under it, and `data-count` on it says how many there are.
//!
//! # What a screen reader hears
//!
//! Each root writes a visually hidden `aria-live="assertive"` region and a hidden instruction
//! paragraph that every handle points at with `aria-describedby`. Pick-up, each move, drop and
//! cancel are announced — "Picked up Item 3, position 3 of 10." The wording is
//! [`default_announcement`] unless the root is given `announce`, and the instructions are
//! overridable the same way, so a translated app translates it all.
//!
//! # Scrolling
//!
//! Held near an edge, the list scrolls. It is the nearest ancestor that actually scrolls
//! ([`crate::primitives::drag::scroll_parent`]) and never the page behind it, which is what a
//! list in a dialog needs: the dialog's body is locked, and a scroller that reached for the window
//! would do nothing. The scroll is clamped to the range the list had at the pick-up, since an item
//! held past the end would otherwise grow the range it is scrolling into.
//!
//! One axis only: a list, vertical or horizontal, not a grid.
//!
//! # Data attributes
//!
//! - root: `data-dragging` (while a move is in progress), `data-orientation`, `data-disabled`;
//! - item: `data-dragging` (it is in the block being moved), `data-following` (it is riding the
//!   pointer, so it must not ease), `data-placeholder` (a keyboard move: it is sitting where it
//!   would land), `data-anchor` (it is the one picked), `data-count` (on the anchor, when there
//!   are several), `data-selected`, `data-disabled`;
//! - handle: `data-dragging`, `data-disabled`.
//!
//! Every non-moving item carries an inline `transform` while a move is in progress. Easing it is
//! the styled layer's business — and has to be switched on only while the root has
//! `data-dragging`, so the transforms are cleared instantly on a drop, in the same frame the
//! caller's reorder lands.

use crate::{
    primitives::drag::{Drag, DragPoint, ScrollAnchor, edge_ramp, scroll_parent, use_drag},
    utils::{next_id, types::Orientation},
};
use leptos::{context::Provider, ev, prelude::*, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;
use std::{collections::HashSet, time::Duration};
use web_sys::{Element, HtmlElement, ScrollIntoViewOptions, ScrollLogicalPosition};

const ITEM_ATTR: &str = "data-sortable-item";
const LABEL_ATTR: &str = "data-sortable-label";
const HANDLE_ATTR: &str = "data-sortable-handle";

/// How far, in pixels, the pointer has to travel before a press becomes a drag. Below it the press
/// is a click, and a click has to survive the shake of pressing a mouse button.
pub const ACTIVATION_DISTANCE: f64 = 4.0;
/// How near a scroller's edge a held item has to be before it starts scrolling.
const EDGE: f64 = 72.0;
/// The most it scrolls in one 16 ms tick, at the very edge.
const EDGE_SPEED: f64 = 16.0;

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
/// use nocomponents::primitives::sortable::{SortableMove, apply_move};
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

/// The gesture in progress.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SortableDrag {
    /// The items being moved, in their current relative order.
    pub keys: Vec<String>,
    /// The one that was picked up.
    pub anchor: String,
    /// Where the block would land, in the list without it.
    pub to: usize,
    /// Whether a pointer is holding it. A keyboard move has the block sit in its place instead.
    pub pointer: bool,
}

/// Where an item sat along the axis when the move began, in its own layout — which a transform
/// does not change, so it can be compared against while everything is sliding.
#[derive(Clone, PartialEq, Debug)]
struct ItemBox {
    key: String,
    start: f64,
    size: f64,
}

#[derive(Copy, Clone)]
pub struct SortableContext {
    pub dragging: RwSignal<Option<SortableDrag>>,
    pub orientation: Signal<Orientation>,
    pub disabled: Signal<bool>,
    selected: Signal<HashSet<String>>,
    boxes: RwSignal<Vec<ItemBox>>,
    preview: Memo<Vec<f64>>,
    /// How far the pointer has travelled since the press.
    base: RwSignal<(f64, f64)>,
    /// How far the list has scrolled out from under it since, from [`ScrollAnchor::shift`].
    shift: RwSignal<(f64, f64)>,
    /// Where the pointer was at the last move; auto-scroll needs it between moves.
    pointer: RwSignal<(f64, f64)>,
    anchor: RwSignal<Option<ScrollAnchor>>,
    scroller: RwSignal<Option<Element>>,
    scroll_max: RwSignal<f64>,
    announcement: RwSignal<String>,
    instructions_id: StoredValue<String>,
    root_ref: AnyNodeRef,
    announce: Callback<SortableEvent, String>,
    on_move: Callback<SortableMove>,
    /// The gesture a pointer move is running, so Escape can end it.
    aborter: StoredValue<Option<Drag>, LocalStorage>,
}

pub fn use_sortable() -> SortableContext {
    expect_context::<SortableContext>()
}

impl SortableContext {
    fn vertical(&self) -> bool {
        self.orientation.get_untracked() == Orientation::Vertical
    }

    /// The id the instruction paragraph carries, for `aria-describedby`.
    pub fn instructions_id(&self) -> String {
        self.instructions_id.get_value()
    }

    /// Whether `key` is in the block being moved.
    pub fn is_moved(&self, key: &str) -> bool {
        self.dragging.with(|drag| {
            drag.as_ref()
                .is_some_and(|drag| drag.keys.iter().any(|k| k == key))
        })
    }

    /// Whether `key` is the item that was picked up.
    pub fn is_anchor(&self, key: &str) -> bool {
        self.dragging
            .with(|drag| drag.as_ref().is_some_and(|drag| drag.anchor == key))
    }

    fn elements(&self) -> Vec<Element> {
        let Some(root) = self.root_ref.get_untracked() else {
            return Vec::new();
        };
        let Ok(nodes) = root.query_selector_all(&format!("[{ITEM_ATTR}]")) else {
            return Vec::new();
        };
        (0..nodes.length())
            .filter_map(|i| nodes.get(i)?.dyn_into::<Element>().ok())
            .collect()
    }

    fn element_for(&self, key: &str) -> Option<Element> {
        self.elements()
            .into_iter()
            .find(|element| element.get_attribute(ITEM_ATTR).as_deref() == Some(key))
    }

    fn label_of(&self, key: &str) -> String {
        let Some(element) = self.element_for(key) else {
            return key.to_string();
        };
        element
            .get_attribute(LABEL_ATTR)
            .filter(|label| !label.is_empty())
            .unwrap_or_else(|| {
                element
                    .text_content()
                    .unwrap_or_default()
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ")
            })
    }

    fn status(&self, drag: &SortableDrag, position: usize, total: usize) -> SortableStatus {
        SortableStatus {
            label: self.label_of(&drag.anchor),
            position,
            total,
            count: drag.keys.len(),
        }
    }

    fn say(&self, event: SortableEvent) {
        self.announcement.set(self.announce.run(event));
    }

    /// The one-based position the anchor would have if the move ended now.
    fn position_of_anchor(&self, drag: &SortableDrag) -> usize {
        drag.keys
            .iter()
            .position(|key| *key == drag.anchor)
            .map_or(0, |rank| drag.to + rank)
            + 1
    }

    /// Takes the block in hand. `false` if a move is already running, or the item is not here.
    fn pick_up(&self, key: &str, pointer: bool) -> bool {
        if self.dragging.get_untracked().is_some() {
            return false;
        }
        let Some(root) = self.root_ref.get_untracked() else {
            return false;
        };
        let vertical = self.vertical();

        let boxes: Vec<ItemBox> = self
            .elements()
            .into_iter()
            .filter_map(|element| {
                let item = element.dyn_into::<HtmlElement>().ok()?;
                let (start, size) = if vertical {
                    (item.offset_top(), item.offset_height())
                } else {
                    (item.offset_left(), item.offset_width())
                };
                Some(ItemBox {
                    key: item.get_attribute(ITEM_ATTR)?,
                    start: start as f64,
                    size: size as f64,
                })
            })
            .collect();

        if !boxes.iter().any(|item| item.key == key) {
            return false;
        }

        let selected = self.selected.get_untracked();
        let keys: Vec<String> = if selected.contains(key) {
            boxes
                .iter()
                .filter(|item| selected.contains(&item.key))
                .map(|item| item.key.clone())
                .collect()
        } else {
            vec![key.to_string()]
        };

        let moved = moved_indices(&boxes, &keys);
        let to = moved.first().copied().unwrap_or(0);
        let drag = SortableDrag {
            keys,
            anchor: key.to_string(),
            to,
            pointer,
        };

        let scroller = scroll_parent(&root, vertical);
        let scroll_max = scroller.as_ref().map_or(0.0, |scroller| {
            if vertical {
                (scroller.scroll_height() - scroller.client_height()).max(0) as f64
            } else {
                (scroller.scroll_width() - scroller.client_width()).max(0) as f64
            }
        });

        let total = boxes.len();
        let original = boxes.iter().position(|item| item.key == key).unwrap_or(0) + 1;
        let status = self.status(&drag, original, total);

        self.base.set((0.0, 0.0));
        self.shift.set((0.0, 0.0));
        self.anchor
            .set(pointer.then(|| ScrollAnchor::capture(root)));
        self.scroller.set(scroller);
        self.scroll_max.set(scroll_max);
        self.boxes.set(boxes);
        self.dragging.set(Some(drag));
        self.say(SortableEvent::PickedUp(status));
        true
    }

    fn set_to(&self, to: usize) {
        let Some(mut drag) = self.dragging.get_untracked() else {
            return;
        };
        let total = self.boxes.with_untracked(|boxes| boxes.len());
        let to = to.min(total.saturating_sub(drag.keys.len()));
        if drag.to == to {
            return;
        }

        drag.to = to;
        let status = self.status(&drag, self.position_of_anchor(&drag), total);
        self.dragging.set(Some(drag));
        self.say(SortableEvent::Moved(status));
    }

    /// The pointer moved: `dx` and `dy` from the press.
    fn pointer_move(&self, key: &str, point: DragPoint) {
        self.pointer.set((point.client_x, point.client_y));

        if self.dragging.get_untracked().is_none() {
            if point.dx.abs().max(point.dy.abs()) < ACTIVATION_DISTANCE {
                return;
            }
            self.pick_up(key, true);
        }

        let ours = self
            .dragging
            .with_untracked(|drag| drag.as_ref().is_some_and(|d| d.pointer && d.anchor == key));
        if !ours {
            return;
        }

        self.base.set((point.dx, point.dy));
        self.retarget();
    }

    fn pointer_end(&self, key: &str) {
        let ours = self
            .dragging
            .with_untracked(|drag| drag.as_ref().is_some_and(|d| d.pointer && d.anchor == key));
        if ours {
            self.finish(true);
        }
    }

    /// Where the held block is, worked out from where it was picked up and how far it has come.
    fn retarget(&self) {
        let Some(drag) = self.dragging.get_untracked() else {
            return;
        };
        if !drag.pointer {
            return;
        }

        let vertical = self.vertical();
        let (base_x, base_y) = self.base.get_untracked();
        let (shift_x, shift_y) = self.shift.get_untracked();
        let travelled = if vertical {
            base_y + shift_y
        } else {
            base_x + shift_x
        };

        let to = self.boxes.with_untracked(|boxes| {
            let moved = moved_indices(boxes, &drag.keys);
            let held = boxes
                .iter()
                .find(|item| item.key == drag.anchor)
                .map_or(0.0, |item| item.start + item.size / 2.0 + travelled);
            let centers: Vec<f64> = boxes
                .iter()
                .map(|item| item.start + item.size / 2.0)
                .collect();
            drop_index(&centers, &moved, held)
        });

        self.set_to(to);
    }

    /// One 16 ms beat while a pointer holds the block: re-measure the scroll, scroll if it is
    /// near an edge, and work out the drop again. On a timer rather than on moves, because a
    /// pointer held still at the edge sends none and is exactly when the reader is waiting.
    fn tick(&self) {
        if let Some(anchor) = self.anchor.get_untracked() {
            let shift = anchor.shift();
            if self.shift.get_untracked() != shift {
                self.shift.set(shift);
            }
        }

        self.autoscroll();
        self.retarget();
    }

    fn autoscroll(&self) {
        let vertical = self.vertical();
        let (x, y) = self.pointer.get_untracked();
        let position = if vertical { y } else { x };
        let viewport = if vertical {
            window().inner_height()
        } else {
            window().inner_width()
        }
        .ok()
        .and_then(|size| size.as_f64())
        .unwrap_or(0.0);

        match self.scroller.get_untracked() {
            Some(scroller) => {
                let rect = scroller.get_bounding_client_rect();
                let (start, end) = if vertical {
                    (rect.top(), rect.bottom())
                } else {
                    (rect.left(), rect.right())
                };
                let step = edge_ramp(
                    position,
                    start.max(0.0),
                    end.min(viewport),
                    EDGE,
                    EDGE_SPEED,
                );
                if step == 0.0 {
                    return;
                }
                let max = self.scroll_max.get_untracked();
                if vertical {
                    let target = (scroller.scroll_top() as f64 + step).clamp(0.0, max);
                    scroller.set_scroll_top(target.round() as i32);
                } else {
                    let target = (scroller.scroll_left() as f64 + step).clamp(0.0, max);
                    scroller.set_scroll_left(target.round() as i32);
                }
            }
            None => {
                let step = edge_ramp(position, 0.0, viewport, EDGE, EDGE_SPEED);
                if step == 0.0 {
                    return;
                }
                if vertical {
                    window().scroll_by_with_x_and_y(0.0, step);
                } else {
                    window().scroll_by_with_x_and_y(step, 0.0);
                }
            }
        }
    }

    /// Ends the move. `commit` drops it where it is; otherwise everything goes back.
    fn finish(&self, commit: bool) {
        let Some(drag) = self.dragging.get_untracked() else {
            return;
        };

        let total = self.boxes.with_untracked(|boxes| boxes.len());
        let moved = self
            .boxes
            .with_untracked(|boxes| moved_indices(boxes, &drag.keys));
        let original = self
            .boxes
            .with_untracked(|boxes| boxes.iter().position(|item| item.key == drag.anchor))
            .unwrap_or(0)
            + 1;
        let changed = commit && !is_noop(&moved, drag.to);
        let position = if changed {
            self.position_of_anchor(&drag)
        } else {
            original
        };
        let status = self.status(&drag, position, total);

        self.dragging.set(None);
        self.anchor.set(None);
        self.scroller.set(None);
        self.base.set((0.0, 0.0));
        self.shift.set((0.0, 0.0));
        self.aborter.set_value(None);

        if changed {
            self.on_move.run(SortableMove {
                keys: drag.keys.clone(),
                to: drag.to,
            });
        }

        self.say(if commit {
            SortableEvent::Dropped(status)
        } else {
            SortableEvent::Cancelled(status)
        });

        if !drag.pointer {
            self.refocus(drag.anchor);
        }
    }

    /// Escape, or anything else that ends a pointer move without dropping it.
    fn abort_pointer(&self) {
        if let Some(drag) = self.aborter.get_value() {
            drag.stop();
        }
        self.finish(false);
    }

    /// Puts focus back on what was moved. Reordering the DOM drops it, so this waits a frame for
    /// the caller's reorder to land.
    fn refocus(&self, key: String) {
        let ctx = *self;
        request_animation_frame(move || {
            let Some(element) = ctx.element_for(&key) else {
                return;
            };
            let target = element
                .query_selector(&format!("[{HANDLE_ATTR}]"))
                .ok()
                .flatten()
                .unwrap_or(element);
            if let Ok(target) = target.dyn_into::<HtmlElement>() {
                let _ = target.focus();
            }
        });
    }

    fn reveal(&self, key: String) {
        let ctx = *self;
        set_timeout(
            move || {
                if let Some(element) = ctx.element_for(&key) {
                    let options = ScrollIntoViewOptions::new();
                    options.set_block(ScrollLogicalPosition::Nearest);
                    options.set_inline(ScrollLogicalPosition::Nearest);
                    element.scroll_into_view_with_scroll_into_view_options(&options);
                }
            },
            Duration::from_millis(170),
        );
    }

    /// A key on the handle, or on the item when it has none.
    fn key_down(&self, event: &ev::KeyboardEvent, key: &str) {
        if self.disabled.get_untracked() || event.alt_key() || event.ctrl_key() || event.meta_key()
        {
            return;
        }
        if event.target() != event.current_target() {
            return;
        }

        let vertical = self.vertical();
        let pressed = event.key();
        let pick = matches!(pressed.as_str(), " " | "Enter");

        let Some(drag) = self.dragging.get_untracked() else {
            if pick {
                event.prevent_default();
                self.pick_up(key, false);
            }
            return;
        };
        if drag.pointer || drag.anchor != key {
            return;
        }

        let total = self.boxes.with_untracked(|boxes| boxes.len());
        let last = total.saturating_sub(drag.keys.len());
        let forward = if vertical { "ArrowDown" } else { "ArrowRight" };
        let backward = if vertical { "ArrowUp" } else { "ArrowLeft" };

        let target = match pressed.as_str() {
            k if k == forward => Some((drag.to + 1).min(last)),
            k if k == backward => Some(drag.to.saturating_sub(1)),
            "Home" => Some(0),
            "End" => Some(last),
            _ => None,
        };

        if let Some(to) = target {
            event.prevent_default();
            self.set_to(to);
            self.reveal(drag.anchor);
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

    /// Focus left the handle in the middle of a keyboard move.
    fn blurred(&self, key: &str) {
        let ours = self.dragging.with_untracked(|drag| {
            drag.as_ref()
                .is_some_and(|drag| !drag.pointer && drag.anchor == key)
        });
        if ours {
            self.finish(false);
        }
    }

    /// The inline style an item wears while a move is in progress.
    fn item_style(&self, key: &str) -> String {
        let Some(drag) = self.dragging.get() else {
            return String::new();
        };
        let boxes = self.boxes.get();
        let Some(index) = boxes.iter().position(|item| item.key == key) else {
            return String::new();
        };
        let vertical = self.orientation.get() == Orientation::Vertical;
        let moved = drag.keys.iter().any(|k| k == key);

        if moved && drag.pointer {
            let (base_x, base_y) = self.base.get();
            let (shift_x, shift_y) = self.shift.get();
            let (mut x, mut y) = (base_x + shift_x, base_y + shift_y);
            let anchor_start = boxes
                .iter()
                .find(|item| item.key == drag.anchor)
                .map_or(boxes[index].start, |item| item.start);
            let stacked = anchor_start - boxes[index].start;
            if vertical {
                y += stacked;
            } else {
                x += stacked;
            }
            return format!(
                "transform: translate3d({x}px, {y}px, 0); pointer-events: none; will-change: transform;"
            );
        }

        let delta = self.preview.get().get(index).copied().unwrap_or(0.0);
        if delta == 0.0 {
            return String::new();
        }
        if vertical {
            format!("transform: translate3d(0, {delta}px, 0);")
        } else {
            format!("transform: translate3d({delta}px, 0, 0);")
        }
    }
}

fn moved_indices(boxes: &[ItemBox], keys: &[String]) -> Vec<usize> {
    boxes
        .iter()
        .enumerate()
        .filter(|(_, item)| keys.contains(&item.key))
        .map(|(index, _)| index)
        .collect()
}

/// The list. Renders the instructions, the live region and a `role="list"` that its items go in.
#[component]
pub fn SortableRoot(
    #[prop(optional, into)] class: Signal<String>,
    /// Which way the list runs. Decides which arrow keys move an item and which axis scrolls.
    #[prop(default = Orientation::Vertical.into(), into)]
    orientation: Signal<Orientation>,
    #[prop(optional, into)] disabled: Signal<bool>,
    /// The keys that are selected. Picking up one of them takes all of them.
    #[prop(optional, into)]
    selected: Signal<HashSet<String>>,
    /// The list's accessible name.
    #[prop(optional, into)]
    label: Signal<String>,
    /// What a handle is described by. Empty means [`DEFAULT_INSTRUCTIONS`].
    #[prop(optional, into)]
    instructions: Signal<String>,
    /// Replaces [`default_announcement`]. Presence-sensitive: unset keeps the English wording.
    #[prop(default = None, into)]
    announce: Option<Callback<SortableEvent, String>>,
    /// Items or a block were let go somewhere new. The caller applies it, with [`apply_move`].
    #[prop(into)]
    on_move: Callback<SortableMove>,
    children: Children,
) -> impl IntoView {
    let root_ref = AnyNodeRef::new();
    let dragging = RwSignal::new(None::<SortableDrag>);
    let boxes = RwSignal::new(Vec::<ItemBox>::new());

    let preview = Memo::new(move |_| {
        let Some(drag) = dragging.get() else {
            return Vec::new();
        };
        boxes.with(|boxes| {
            let starts: Vec<f64> = boxes.iter().map(|item| item.start).collect();
            let sizes: Vec<f64> = boxes.iter().map(|item| item.size).collect();
            preview_offsets(&starts, &sizes, &moved_indices(boxes, &drag.keys), drag.to)
        })
    });

    let context = SortableContext {
        dragging,
        orientation,
        disabled,
        selected,
        boxes,
        preview,
        base: RwSignal::new((0.0, 0.0)),
        shift: RwSignal::new((0.0, 0.0)),
        pointer: RwSignal::new((0.0, 0.0)),
        anchor: RwSignal::new(None),
        scroller: RwSignal::new(None),
        scroll_max: RwSignal::new(0.0),
        announcement: RwSignal::new(String::new()),
        instructions_id: StoredValue::new(next_id("sortable-instructions")),
        root_ref,
        announce: announce.unwrap_or_else(|| Callback::new(|e| default_announcement(&e))),
        on_move,
        aborter: StoredValue::new_local(None),
    };

    let held = Memo::new(move |_| dragging.with(|d| d.as_ref().is_some_and(|d| d.pointer)));
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
        let pointer = context
            .dragging
            .with_untracked(|drag| drag.as_ref().is_some_and(|drag| drag.pointer));
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
            node_ref=root_ref
            role="list"
            data-slot="sortable"
            aria-label=move || Some(label.get()).filter(|label| !label.is_empty())
            aria-disabled=move || disabled.get().then_some("true")
            data-disabled=move || disabled.get().then_some("true")
            data-orientation=move || orientation.get().as_str()
            data-dragging=move || dragging.with(|drag| drag.is_some()).then_some("true")
            class=class
        >
            <Provider value=context>{children()}</Provider>
        </div>
    }
}

#[derive(Copy, Clone)]
pub struct SortableItemContext {
    pub key: StoredValue<String>,
    pub label: Signal<String>,
    pub disabled: Signal<bool>,
    pub has_handle: RwSignal<bool>,
    drag: Drag,
}

pub fn use_sortable_item() -> SortableItemContext {
    expect_context::<SortableItemContext>()
}

impl SortableItemContext {
    fn press(&self, event: &ev::PointerEvent) {
        if self.disabled.get_untracked() || event.button() != 0 {
            return;
        }
        self.drag.start(event);
    }
}

/// Whether the press landed on something that handles its own presses.
fn from_a_control(event: &ev::PointerEvent) -> bool {
    event
        .target()
        .and_then(|target| target.dyn_into::<Element>().ok())
        .and_then(|target| {
            target
                .closest("button, a, input, select, textarea, label, [role=checkbox], [data-sortable-no-drag]")
                .ok()
                .flatten()
        })
        .is_some()
}

/// One item. Without a [`SortableHandleRoot`] inside it, the whole item is what is dragged and
/// what takes focus.
#[component]
pub fn SortableItemRoot(
    /// Names the item in a [`SortableMove`].
    #[prop(into)]
    value: String,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    /// What a screen reader is told the item is called. Without one, its text.
    #[prop(optional, into)]
    label: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_sortable();
    let key = StoredValue::new(value.clone());
    let off = Signal::derive(move || ctx.disabled.get() || disabled.get());

    let gesture: StoredValue<Option<Drag>, LocalStorage> = StoredValue::new_local(None);
    let drag = use_drag(move |point| ctx.pointer_move(&key.get_value(), point))
        .on_start(move |_| ctx.aborter.set_value(gesture.get_value()))
        .on_end(move |_| ctx.pointer_end(&key.get_value()));
    gesture.set_value(Some(drag));

    let has_handle = RwSignal::new(false);
    let item = SortableItemContext {
        key,
        label,
        disabled: off,
        has_handle,
        drag,
    };

    let moved = move || key.with_value(|key| ctx.is_moved(key));
    let anchor = move || key.with_value(|key| ctx.is_anchor(key));
    let following = move || moved() && ctx.dragging.with(|d| d.as_ref().is_some_and(|d| d.pointer));
    let attribute = value.clone();
    let described = ctx.instructions_id();

    view! {
        <div
            role="listitem"
            data-slot="sortable-item"
            data-sortable-item=attribute
            data-sortable-label=move || Some(label.get()).filter(|label| !label.is_empty())
            data-dragging=move || moved().then_some("true")
            data-following=move || following().then_some("true")
            data-placeholder=move || (moved() && !following()).then_some("true")
            data-anchor=move || anchor().then_some("true")
            data-count=move || {
                if !anchor() {
                    return None;
                }
                ctx.dragging
                    .with(|drag| drag.as_ref().map(|d| d.keys.len()).filter(|n| *n > 1))
                    .map(|n| n.to_string())
            }
            data-selected=move || {
                key.with_value(|key| ctx.selected.with(|set| set.contains(key))).then_some("true")
            }
            data-disabled=move || off.get().then_some("true")
            aria-disabled=move || off.get().then_some("true")
            tabindex=move || (!has_handle.get() && !off.get()).then_some("0")
            aria-roledescription=move || (!has_handle.get()).then_some("sortable")
            aria-describedby=move || (!has_handle.get()).then(|| described.clone())
            style=move || {
                let style = key.with_value(|key| ctx.item_style(key));
                if has_handle.get() {
                    style
                } else {
                    format!("{style} touch-action: none; user-select: none;")
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
                    key.with_value(|key| ctx.key_down(&e, key));
                }
            }
            on:blur=move |_| {
                if !has_handle.get_untracked() {
                    key.with_value(|key| ctx.blurred(key));
                }
            }
            class=class
        >
            <Provider value=item>{children()}</Provider>
        </div>
    }
}

/// The part of an item that starts a drag and takes focus. A `role="button"`, not a `<button>`:
/// Space on a real button clicks on key *up*, after the pick-up has already used the key.
#[component]
pub fn SortableHandleRoot(
    #[prop(optional, into)] class: Signal<String>,
    /// The handle's accessible name. Without one: "Reorder", and the item's label if it has one.
    #[prop(optional, into)]
    label: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_sortable();
    let item = use_sortable_item();

    item.has_handle.set(true);
    on_cleanup(move || item.has_handle.set(false));

    let name = move || {
        let label = label.get();
        if !label.is_empty() {
            return label;
        }
        let item_label = item.label.get();
        if item_label.is_empty() {
            "Reorder".to_string()
        } else {
            format!("Reorder {item_label}")
        }
    };
    let moved = move || item.key.with_value(|key| ctx.is_moved(key));
    let anchor = move || item.key.with_value(|key| ctx.is_anchor(key));
    let described = ctx.instructions_id();

    view! {
        <span
            role="button"
            data-slot="sortable-handle"
            data-sortable-handle=""
            aria-label=name
            aria-roledescription="sortable"
            aria-describedby=described
            aria-pressed=move || { anchor().then_some("true") }
            aria-disabled=move || item.disabled.get().then_some("true")
            data-disabled=move || item.disabled.get().then_some("true")
            data-dragging=move || moved().then_some("true")
            tabindex=move || if item.disabled.get() { "-1" } else { "0" }
            style="touch-action: none; user-select: none;"
            on:pointerdown=move |e: ev::PointerEvent| item.press(&e)
            on:keydown=move |e: ev::KeyboardEvent| item.key.with_value(|key| ctx.key_down(&e, key))
            on:blur=move |_| item.key.with_value(|key| ctx.blurred(key))
            class=class
        >
            {children()}
        </span>
    }
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
