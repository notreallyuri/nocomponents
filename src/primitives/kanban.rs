//! A board of columns of cards, and the gesture that moves one between them.
//!
//! **The board does not own the cards.** It reports where one was dropped — `on_move` with the
//! card, the column it came from, and the column and index it landed at — and the caller applies
//! that to their own data. A board that reordered a list it did not own would be a board that
//! disagreed with the caller's list the moment anything else touched it, which is the same reason
//! `DataTable` sorts what it is handed rather than keeping rows.
//!
//! Where a card would land is worked out from the DOM rather than from a registry the parts keep
//! in step: the columns are found by `data-kanban-column`, the cards inside one by
//! `data-kanban-card`, and the index is however many card midpoints the pointer is past. Columns
//! and cards can then appear, move and vanish without telling anybody.
//!
//! The gesture is [`crate::primitives::drag`], so it ends on `pointercancel` like every other one
//! here — a drag that dies when the browser takes the pointer away (a scroll gesture, a context
//! menu) leaves no card stuck to the cursor.

use crate::primitives::drag::{DragPoint, use_drag};
use leptos::{context::Provider, ev, prelude::*, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;
use std::time::Duration;
use web_sys::Element;

const COLUMN_ATTR: &str = "data-kanban-column";
/// How far the pointer has to travel before a press becomes a drag. Below it the press is a
/// click, and a click has to survive the shake of pressing a mouse button.
const MOVE_THRESHOLD: f64 = 4.0;
/// How near a board's edge a dragged card has to be before the board starts scrolling itself.
const EDGE: f64 = 72.0;
/// The most it scrolls in one tick, at the very edge. Ramped, not stepped: a board that lurches
/// the moment you enter the zone is harder to aim than one that eases into it.
const EDGE_SPEED: f64 = 16.0;
const CARD_ATTR: &str = "data-kanban-card";

/// Where a card would land: which column, and how many cards down.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct KanbanSlot {
    pub column: String,
    pub index: usize,
}

/// A card that has been let go, and where.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct KanbanMove {
    pub card: String,
    pub from: String,
    pub to: String,
    pub index: usize,
}

/// The gesture in progress.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct KanbanDrag {
    pub card: String,
    pub from: String,
    /// Recomputed on every move, and `None` while the pointer is off the board.
    pub over: Option<KanbanSlot>,
}

/// Where the dragged card's own container sat, and how far it was scrolled, at the press.
///
/// See [`KanbanContext::sync_shift`] for why those two are all it takes.
#[derive(Copy, Clone)]
struct ScrollOrigin {
    left: f64,
    top: f64,
    scroll_x: f64,
    scroll_y: f64,
}

#[derive(Copy, Clone)]
pub struct KanbanContext {
    pub dragging: RwSignal<Option<KanbanDrag>>,
    /// Where the pointer was at the last move. The board's own scrolling needs it between moves:
    /// a pointer held still at the edge stops sending events but should not stop scrolling.
    pointer: RwSignal<(f64, f64)>,
    /// The container the card in hand sits in, and where it was when the card was picked up.
    origin: RwSignal<Option<(Element, ScrollOrigin)>>,
    /// How far the card has to be pushed to stay under the cursor, from [`Self::sync_shift`].
    shift: RwSignal<(f64, f64)>,
    /// The board's scroll range as it was before the press. A card held to the right is
    /// translated past the content, which *grows* the scrollable width — so scrolling to the live
    /// maximum runs past the real end and snaps back the moment the card is let go.
    scroll_max: RwSignal<f64>,
    board_ref: AnyNodeRef,
    on_move: Callback<KanbanMove>,
}

impl KanbanContext {
    /// Whether `column` is where the card in hand would land, and at which index.
    pub fn slot_in(&self, column: &str) -> Option<usize> {
        self.dragging
            .get()
            .and_then(|drag| drag.over)
            .filter(|slot| slot.column == column)
            .map(|slot| slot.index)
    }

    /// Whether this card is the one being dragged.
    pub fn is_dragging(&self, card: &str) -> bool {
        self.dragging.get().is_some_and(|drag| drag.card == card)
    }

    /// Recomputes how far the card in hand has to be pushed to stay under the cursor.
    ///
    /// A card follows the pointer by a transform from where it was pressed, so anything that moves
    /// its *layout* position mid-gesture slides it out from under the cursor: the board scrolling
    /// itself at an edge, the reader flicking the board sideways with a wheel, a column scrolling,
    /// the page scrolling under all of it. The pointer positions the gesture works in are client
    /// ones and hear about none of it.
    ///
    /// All four are the same subtraction rather than four causes to keep a tally of — where the
    /// card's own container is now against where it was at the press. Its rect answers for
    /// everything outside it, its scroll offsets for the one thing that moves the card without
    /// moving the container: the container being the scroller.
    fn sync_shift(&self) {
        let Some((anchor, origin)) = self.origin.get_untracked() else {
            return;
        };

        let rect = anchor.get_bounding_client_rect();
        let shift = (
            origin.left - rect.left() + anchor.scroll_left() as f64 - origin.scroll_x,
            origin.top - rect.top() + anchor.scroll_top() as f64 - origin.scroll_y,
        );

        if self.shift.get_untracked() != shift {
            self.shift.set(shift);
        }
    }
}

pub fn use_kanban() -> KanbanContext {
    expect_context::<KanbanContext>()
}

/// The column a card is rendered inside.
#[derive(Clone)]
pub struct KanbanColumnContext {
    pub id: String,
}

/// How far to scroll this tick, from where the pointer sits relative to the board's edges.
///
/// Ramped from nothing at the outer edge of the zone to [`EDGE_SPEED`] at the board's own edge,
/// and signed: negative scrolls left.
fn edge_step(board: &Element, x: f64) -> f64 {
    let rect = board.get_bounding_client_rect();
    if rect.width() <= 0.0 {
        return 0.0;
    }

    if x < rect.left() + EDGE {
        -((rect.left() + EDGE - x) / EDGE).clamp(0.0, 1.0) * EDGE_SPEED
    } else if x > rect.right() - EDGE {
        ((x - (rect.right() - EDGE)) / EDGE).clamp(0.0, 1.0) * EDGE_SPEED
    } else {
        0.0
    }
}

fn attribute(element: &Element, name: &str) -> Option<String> {
    element.get_attribute(name)
}

fn contains(element: &Element, x: f64, y: f64) -> bool {
    let rect = element.get_bounding_client_rect();
    x >= rect.left() && x <= rect.right() && y >= rect.top() && y <= rect.bottom()
}

/// The slot the pointer is over, or `None` when it is off every column.
fn slot_at(board: &Element, x: f64, y: f64, dragged: &str) -> Option<KanbanSlot> {
    let columns = board.query_selector_all(&format!("[{COLUMN_ATTR}]")).ok()?;

    let column = (0..columns.length())
        .filter_map(|i| columns.get(i)?.dyn_into::<Element>().ok())
        .find(|column| contains(column, x, y))?;
    let id = attribute(&column, COLUMN_ATTR)?;

    let cards = column.query_selector_all(&format!("[{CARD_ATTR}]")).ok()?;
    let mut index = 0;
    for i in 0..cards.length() {
        let Some(card) = cards.get(i).and_then(|n| n.dyn_into::<Element>().ok()) else {
            continue;
        };
        // The card in hand is still in the DOM where it started, and counting it would make the
        // index it came from read as one further down than it is.
        if attribute(&card, CARD_ATTR).as_deref() == Some(dragged) {
            continue;
        }
        let rect = card.get_bounding_client_rect();
        if y > rect.top() + rect.height() / 2.0 {
            index += 1;
        }
    }

    Some(KanbanSlot { column: id, index })
}

#[component]
pub fn KanbanBoardRoot(
    #[prop(optional, into)] class: Signal<String>,
    /// Where a card was let go. The caller applies it to their own data.
    #[prop(into)]
    on_move: Callback<KanbanMove>,
    children: Children,
) -> impl IntoView {
    let board_ref = AnyNodeRef::new();
    let context = KanbanContext {
        dragging: RwSignal::new(None),
        pointer: RwSignal::new((0.0, 0.0)),
        origin: RwSignal::new(None),
        shift: RwSignal::new((0.0, 0.0)),
        scroll_max: RwSignal::new(0.0),
        board_ref,
        on_move,
    };

    // The board scrolls itself while a card is held near an edge. On a timer rather than on
    // pointer moves: a pointer held still at the edge sends no more events, and that is exactly
    // when the reader is waiting for the board to come to them.
    Effect::new(move |previous: Option<Option<IntervalHandle>>| {
        if let Some(Some(handle)) = previous {
            handle.clear();
        }
        context.dragging.get()?;
        set_interval_with_handle(
            move || {
                // The timer is the gesture's heartbeat as well as its edge scrolling, so the
                // card's compensation is recomputed here rather than from a listener per thing
                // that could have scrolled. It runs only while a card is in hand.
                context.sync_shift();

                let Some(board) = context.board_ref.get_untracked() else {
                    return;
                };
                let (x, _) = context.pointer.get_untracked();
                let step = edge_step(&board, x);
                if step == 0.0 {
                    return;
                }
                let target = (board.scroll_left() as f64 + step)
                    .clamp(0.0, context.scroll_max.get_untracked());
                board.set_scroll_left(target.round() as i32);
            },
            Duration::from_millis(16),
        )
        .ok()
    });

    view! {
        <div node_ref=board_ref data-slot="kanban-board" class=class>
            <Provider value=context>{children()}</Provider>
        </div>
    }
}

#[component]
pub fn KanbanColumnRoot(
    #[prop(into)] id: String,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_kanban();
    let column = id.clone();
    let attribute = id.clone();
    let over = move || ctx.slot_in(&column).is_some();

    view! {
        <div
            data-slot="kanban-column"
            data-kanban-column=attribute
            data-over=move || over().then_some("true")
            class=class
        >
            <Provider value=KanbanColumnContext { id }>{children()}</Provider>
        </div>
    }
}

#[component]
pub fn KanbanCardRoot(
    #[prop(into)] id: String,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    /// The card was pressed and let go without travelling. What opens a card's own dialog.
    ///
    /// A prop rather than the caller's own `on:click`, because a card cannot receive one: the
    /// moment it starts following the pointer it goes `pointer-events: none`, so the release
    /// lands on whatever is underneath. Only the gesture knows whether the pointer moved.
    #[prop(default = None, into)]
    on_select: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let ctx = use_kanban();
    let column = expect_context::<KanbanColumnContext>().id;
    let card_ref = AnyNodeRef::new();

    let card = StoredValue::new(id.clone());
    let from = StoredValue::new(column);

    // How far the card has come since the press. It follows the pointer by transform rather than
    // by leaving the list: the gap it came from stays open, so no column reflows under the
    // gesture and the slot the pointer is over does not shift as the card moves.
    let offset = RwSignal::new((0.0f64, 0.0f64));

    let track = move |point: DragPoint| {
        ctx.pointer.set((point.client_x, point.client_y));

        // A drag begins on the first move past the threshold, not on the press. Announcing it any
        // earlier would light a column up under a click and — because the card goes
        // `pointer-events: none` to follow the pointer — swallow the click that never happened.
        if ctx.dragging.get_untracked().is_none() {
            if point.dx.abs().max(point.dy.abs()) < MOVE_THRESHOLD {
                return;
            }
            ctx.dragging.set(Some(KanbanDrag {
                card: card.get_value(),
                from: from.get_value(),
                over: None,
            }));
        }

        offset.set((point.dx, point.dy));

        let Some(board) = ctx.board_ref.get_untracked() else {
            return;
        };
        let held = card.get_value();
        let over = slot_at(&board, point.client_x, point.client_y, &held);
        ctx.dragging.update(|drag| {
            if let Some(drag) = drag {
                drag.over = over;
            }
        });
    };

    let drag = use_drag(track)
        .on_start(move |_| {
            offset.set((0.0, 0.0));
            ctx.shift.set((0.0, 0.0));

            // The card's own container — whatever moves when the card's layout position does.
            // Taken from the card rather than named in advance, since only the caller knows what
            // it put its cards inside.
            let anchor = card_ref
                .get_untracked()
                .and_then(|card| card.parent_element());
            ctx.origin.set(anchor.map(|anchor| {
                let rect = anchor.get_bounding_client_rect();
                let origin = ScrollOrigin {
                    left: rect.left(),
                    top: rect.top(),
                    scroll_x: anchor.scroll_left() as f64,
                    scroll_y: anchor.scroll_top() as f64,
                };
                (anchor, origin)
            }));

            // Measured now, while nothing is translated and the width is the real one.
            if let Some(board) = ctx.board_ref.get_untracked() {
                let max = (board.scroll_width() - board.client_width()).max(0) as f64;
                ctx.scroll_max.set(max);
            }
        })
        .on_end(move |_| {
            let landed = ctx.dragging.get_untracked();
            ctx.dragging.set(None);
            offset.set((0.0, 0.0));
            ctx.shift.set((0.0, 0.0));
            ctx.origin.set(None);

            // Nothing was ever dragged, so the press was a click.
            let Some(landed) = landed else {
                if let Some(on_select) = on_select {
                    on_select.run(());
                }
                return;
            };

            let KanbanDrag {
                card,
                from,
                over: Some(slot),
            } = landed
            else {
                return;
            };
            // A card put back exactly where it came from is not a move, and reporting it would
            // have every board rewrite its own list for nothing.
            if slot.column == from && slot.index == index_of(&card, &from) {
                return;
            }
            ctx.on_move.run(KanbanMove {
                card,
                from,
                to: slot.column,
                index: slot.index,
            });
        });

    // `Copy`, so both the attribute closures below can read it.
    let dragging = move || card.with_value(|card| ctx.is_dragging(card));
    let attribute = id.clone();

    view! {
        <div
            node_ref=card_ref
            data-slot="kanban-card"
            data-kanban-card=attribute
            data-dragging=move || dragging().then_some("true")
            aria-grabbed=move || dragging().then_some("true")
            style=move || {
                if !dragging() {
                    return String::new();
                }
                let (dx, dy) = offset.get();
                // Anything may have scrolled out from under the card while it was held — the
                // board at an edge, a column, the page. The card rides in whatever did, so
                // without this it slides away from the cursor by however far that went.
                let (shift_x, shift_y) = ctx.shift.get();
                let (dx, dy) = (dx + shift_x, dy + shift_y);
                // `pointer-events: none` so the card under the cursor is whatever it is over,
                // not itself — the gesture's own listeners are on the window and do not care.
                format!(
                    "transform: translate3d({dx}px, {dy}px, 0); pointer-events: none; will-change: transform;",
                )
            }
            on:pointerdown=move |e: ev::PointerEvent| {
                // The primary button only, and never from a control inside the card: a menu in a
                // card's corner is there to be pressed, not to pick the card up.
                if disabled.get_untracked() || e.button() != 0 || from_a_control(&e) {
                    return;
                }
                drag.start(&e);
            }
            class=class
        >
            {children()}
        </div>
    }
}

/// Where a card sits in its column now, so a drop back into the same place can be recognised.
fn index_of(card: &str, column: &str) -> usize {
    let Some(document) = leptos::prelude::document()
        .query_selector(&format!("[{COLUMN_ATTR}=\"{column}\"]"))
        .ok()
        .flatten()
    else {
        return usize::MAX;
    };

    let Ok(cards) = document.query_selector_all(&format!("[{CARD_ATTR}]")) else {
        return usize::MAX;
    };

    (0..cards.length())
        .filter_map(|i| cards.get(i)?.dyn_into::<Element>().ok())
        .position(|element| attribute(&element, CARD_ATTR).as_deref() == Some(card))
        .unwrap_or(usize::MAX)
}

/// Whether the press landed on something that handles its own presses.
fn from_a_control(event: &ev::PointerEvent) -> bool {
    let Some(target) = event.target().and_then(|t| t.dyn_into::<Element>().ok()) else {
        return false;
    };
    target
        .closest("button, a, input, select, textarea, [data-kanban-no-drag]")
        .ok()
        .flatten()
        .is_some()
}

/// A place a card would land, for a column to draw a line at.
///
/// Rendered by the caller between its cards, because only the caller knows where "between" is.
#[component]
pub fn KanbanSlotRoot(
    /// How many cards down this gap sits.
    index: usize,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    let ctx = use_kanban();
    let column = expect_context::<KanbanColumnContext>().id;
    let active = move || ctx.slot_in(&column) == Some(index);

    view! {
        <div
            data-slot="kanban-slot"
            data-active=move || active().then_some("true")
            aria-hidden="true"
            class=class
        />
    }
}
