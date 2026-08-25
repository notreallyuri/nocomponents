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
use web_sys::Element;

const COLUMN_ATTR: &str = "data-kanban-column";
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

#[derive(Copy, Clone)]
pub struct KanbanContext {
    pub dragging: RwSignal<Option<KanbanDrag>>,
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
}

pub fn use_kanban() -> KanbanContext {
    expect_context::<KanbanContext>()
}

/// The column a card is rendered inside.
#[derive(Clone)]
pub struct KanbanColumnContext {
    pub id: String,
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
        board_ref,
        on_move,
    };

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
            ctx.dragging.set(Some(KanbanDrag {
                card: card.get_value(),
                from: from.get_value(),
                over: None,
            }));
        })
        .on_end(move |_| {
            let landed = ctx.dragging.get_untracked();
            ctx.dragging.set(None);
            offset.set((0.0, 0.0));

            let Some(KanbanDrag {
                card,
                from,
                over: Some(slot),
            }) = landed
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
