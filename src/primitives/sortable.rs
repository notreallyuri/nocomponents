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
//!   keeping its relative order, and lands as one block. See below.
//!
//! # A block is one pile while it is held
//!
//! Ten full rows riding a pointer are unreadable, and ten ring-marked slots through the list are
//! worse, so a block of more than one is *collapsed*: the picked item is the pile (it wears
//! `data-count`, and the styled layer draws the layers and the badge), and it rides the pointer, or
//! for the keyboard sits in its slot. The other selected items are `data-hidden` (and inline
//! `visibility: hidden`, so a primitive without the styled layer hides them too), and the items
//! that stay slide to close the gaps those left, with exactly one slot, the size of the pile, open
//! at the target. Nothing is moved in the DOM and no item changes its layout, so there is nothing
//! to compensate for: the list's tail just shows the room the hidden items used to take.
//!
//! The arithmetic is [`collapsed_preview_offsets`], which is [`preview_offsets`] on the list as
//! it will be *shown*: the items that stay, and one entry, the pile, where the block lands. A block
//! of one gives exactly [`preview_offsets`]. Where the pile lands is [`collapsed_drop_index`],
//! measured on that shown list, so carrying the pile past three items passes three items, whatever
//! the selection had between them, and the number is the same `to` that [`SortableMove`] reports.
//!
//! Closing is toward the start of the list, so the room the hidden items took is empty space at
//! the *end* of it, never at the start. The pile is held where the pointer is, and the slot opens
//! wherever the pile currently is in the closed-up list — the one nearest it, from the first frame
//! — so what is seen is where the block lands, and a block let go without moving gathers at the
//! pile's visible position, as [`window`]'s stack does. Everything is still a transform.
//!
//! # What a screen reader hears
//!
//! Each root writes a visually hidden `aria-live="assertive"` region and a hidden instruction
//! paragraph that every handle points at with `aria-describedby`. Pick-up, each move, drop and
//! cancel are announced — "Picked up Item 3, position 3 of 10." The wording is
//! [`default_announcement`] unless the root is given `announce`, and the instructions are
//! overridable the same way, so a translated app translates it all.
//!
//! A second, `aria-live="polite"` region says what the move *is* while it lasts — "Moving 4
//! items. Destination position 12 of 5,000." — and is re-written whenever the destination changes
//! and emptied on a drop or cancel. Its wording is [`default_progress`] unless the root is given
//! `progress`, which is handed a [`SortableProgress`].
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
//! # A long list
//!
//! Everything above measures the DOM, which is right for a list of dozens. For thousands of rows
//! there is [`window`]: the same gestures on fixed-height arithmetic, with only the rows in view
//! in the page.
//!
//! # Data attributes
//!
//! - root: `data-dragging` (while a move is in progress), `data-orientation`, `data-disabled`;
//! - item: `data-dragging` (it is in the block being moved), `data-following` (it is the picked
//!   item and a pointer holds it, so it must not ease), `data-placeholder` (a keyboard move of one
//!   item: it is sitting where it would land), `data-anchor` (it is the one picked), `data-count`
//!   (on the anchor, when there are several: it is the pile), `data-hidden` (it is in a block but
//!   not the pile, and is not drawn), `data-selected`, `data-disabled`;
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

pub mod window;

pub use nocomponents_core::sortable::{
    ACTIVATION_DISTANCE, DEFAULT_INSTRUCTIONS, SortableEvent, SortableMove, SortableProgress,
    SortableStatus, apply_move, collapsed_drop_index, collapsed_preview_offsets,
    default_announcement, default_progress, drop_index, is_noop, move_block, preview_offsets,
};

const ITEM_ATTR: &str = "data-sortable-item";
const LABEL_ATTR: &str = "data-sortable-label";
const HANDLE_ATTR: &str = "data-sortable-handle";

/// How near a scroller's edge a held item has to be before it starts scrolling.
const EDGE: f64 = 72.0;
/// The most it scrolls in one 16 ms tick, at the very edge.
const EDGE_SPEED: f64 = 16.0;

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
    /// The polite region: what the move in progress is, re-said as its destination changes.
    progress_text: RwSignal<String>,
    instructions_id: StoredValue<String>,
    root_ref: AnyNodeRef,
    announce: Callback<SortableEvent, String>,
    progress: Callback<SortableProgress, String>,
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

    fn say_progress(&self, drag: &SortableDrag) {
        let progress = SortableProgress {
            label: self.label_of(&drag.anchor),
            count: drag.keys.len(),
            destination: drag.to + 1,
            total: self.boxes.with_untracked(|boxes| boxes.len()),
        };
        self.progress_text.set(self.progress.run(progress));
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
        self.say_progress(&drag);
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
        self.say_progress(&drag);
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
            let anchor = boxes.iter().position(|item| item.key == drag.anchor);
            let held = anchor.map_or(0.0, |anchor| {
                boxes[anchor].start + boxes[anchor].size / 2.0 + travelled
            });
            match anchor {
                Some(anchor) if moved.len() > 1 => {
                    let starts: Vec<f64> = boxes.iter().map(|item| item.start).collect();
                    let sizes: Vec<f64> = boxes.iter().map(|item| item.size).collect();
                    collapsed_drop_index(&starts, &sizes, &moved, anchor, held)
                }
                _ => {
                    let centers: Vec<f64> = boxes
                        .iter()
                        .map(|item| item.start + item.size / 2.0)
                        .collect();
                    drop_index(&centers, &moved, held)
                }
            }
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
        self.progress_text.set(String::new());
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

    /// Whether `key` is in a block of several but is not the pile: it is not drawn.
    pub fn is_hidden(&self, key: &str) -> bool {
        self.dragging.with(|drag| {
            drag.as_ref().is_some_and(|drag| {
                drag.keys.len() > 1 && drag.anchor != key && drag.keys.iter().any(|k| k == key)
            })
        })
    }

    /// The inline style an item wears while a move is in progress.
    fn item_style(&self, key: &str) -> String {
        let Some(drag) = self.dragging.get() else {
            return String::new();
        };
        if self.is_hidden(key) {
            return "visibility: hidden; pointer-events: none;".to_string();
        }
        let boxes = self.boxes.get();
        let Some(index) = boxes.iter().position(|item| item.key == key) else {
            return String::new();
        };
        let vertical = self.orientation.get() == Orientation::Vertical;

        if drag.pointer && drag.anchor == key {
            let (base_x, base_y) = self.base.get();
            let (shift_x, shift_y) = self.shift.get();
            return format!(
                "transform: translate3d({}px, {}px, 0); pointer-events: none; will-change: transform;",
                base_x + shift_x,
                base_y + shift_y
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
    /// Replaces [`default_progress`], the polite live region that says what the move in progress
    /// is while it lasts. Presence-sensitive: unset keeps the English wording.
    #[prop(default = None, into)]
    progress: Option<Callback<SortableProgress, String>>,
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
            let moved = moved_indices(boxes, &drag.keys);
            let anchor = boxes.iter().position(|item| item.key == drag.anchor);
            match anchor {
                Some(anchor) if moved.len() > 1 => {
                    collapsed_preview_offsets(&starts, &sizes, &moved, anchor, drag.to)
                }
                _ => preview_offsets(&starts, &sizes, &moved, drag.to),
            }
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
        progress_text: RwSignal::new(String::new()),
        instructions_id: StoredValue::new(next_id("sortable-instructions")),
        root_ref,
        announce: announce.unwrap_or_else(|| Callback::new(|e| default_announcement(&e))),
        progress: progress.unwrap_or_else(|| Callback::new(|p| default_progress(&p))),
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
        <div aria-live="polite" aria-atomic="true" class="sr-only">
            {move || context.progress_text.get()}
        </div>
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
    let following =
        move || anchor() && ctx.dragging.with(|d| d.as_ref().is_some_and(|d| d.pointer));
    let hidden = move || key.with_value(|key| ctx.is_hidden(key));
    let block = move || {
        ctx.dragging
            .with(|d| d.as_ref().is_some_and(|d| d.keys.len() > 1))
    };
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
            data-placeholder=move || (moved() && !following() && !block()).then_some("true")
            data-hidden=move || hidden().then_some("true")
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
