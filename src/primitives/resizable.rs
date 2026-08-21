//! Resizable panels: a row or column of panels with draggable dividers between them.
//!
//! Sizes are percentages of the group, never pixels, so the layout survives the window changing
//! size — the panels are flex items whose `flex-grow` *is* their percentage, which leaves the
//! dividers' own widths out of the arithmetic entirely.
//!
//! A drag moves one boundary: it takes room from the panel on one side and gives exactly that
//! much to the other, so the total is always 100 and no third panel twitches. Both panels' limits
//! are honoured, which means the amount actually applied can be less than the pointer asked for.

use crate::{
    primitives::drag::{DragPoint, use_drag},
    utils::types::Orientation,
};
use leptos::{context::Provider, ev, prelude::*};
use leptos_node_ref::AnyNodeRef;

/// How much an arrow key moves a divider, in percentage points.
const KEYBOARD_STEP: f64 = 5.0;

/// What a panel will accept, in percentage points.
#[derive(Clone, Copy, Debug)]
struct PanelLimits {
    min: f64,
    max: f64,
    /// The size asked for at mount. `None` shares out whatever the sized panels left over.
    default: Option<f64>,
}

#[derive(Copy, Clone)]
pub struct ResizableGroupContext {
    pub direction: Orientation,
    /// One percentage per panel, in the order the panels registered — which is document order,
    /// since that is the order they mount in.
    pub sizes: RwSignal<Vec<f64>>,
    /// The index of the divider being dragged, if one is.
    pub dragging: RwSignal<Option<usize>>,
    /// The group, whose width or height is what a pixel of pointer movement is measured against.
    pub group_ref: AnyNodeRef,

    limits: RwSignal<Vec<PanelLimits>>,
}

impl ResizableGroupContext {
    /// Adds a panel and returns its index. Called during render, so registration order is
    /// document order.
    fn register(&self, min: f64, max: f64, default: Option<f64>) -> usize {
        let mut index = 0;
        self.limits.update(|limits| {
            index = limits.len();
            limits.push(PanelLimits { min, max, default });
        });
        index
    }

    /// Hands out the room: the panels that asked for a size get it, and whatever is left is split
    /// evenly between the ones that did not. Runs once every panel has registered.
    fn distribute(&self) {
        let limits = self.limits.get();
        if limits.is_empty() {
            return;
        }

        let claimed: f64 = limits.iter().filter_map(|panel| panel.default).sum();
        let unclaimed = limits
            .iter()
            .filter(|panel| panel.default.is_none())
            .count();
        let share = if unclaimed > 0 {
            ((100.0 - claimed) / unclaimed as f64).max(0.0)
        } else {
            0.0
        };

        let mut sizes: Vec<f64> = limits
            .iter()
            .map(|panel| panel.default.unwrap_or(share))
            .collect();

        // A set of defaults that does not add up is the caller's arithmetic, not the layout's
        // problem; scaling keeps the group full either way.
        let total: f64 = sizes.iter().sum();
        if total > 0.0 && (total - 100.0).abs() > f64::EPSILON {
            for size in &mut sizes {
                *size = *size / total * 100.0;
            }
        }

        self.sizes.set(sizes);
    }

    pub fn size(&self, index: usize) -> f64 {
        self.sizes
            .with(|sizes| sizes.get(index).copied().unwrap_or(0.0))
    }

    /// The group's length along its own axis, in pixels.
    fn extent(&self) -> f64 {
        self.group_ref
            .get_untracked()
            .map(|group| {
                let rect = group.get_bounding_client_rect();
                match self.direction {
                    Orientation::Horizontal => rect.width(),
                    Orientation::Vertical => rect.height(),
                }
            })
            .unwrap_or(0.0)
    }

    /// Moves the divider after panel `index` by `delta` percentage points, honouring both
    /// neighbours' limits. `from` is the pair of sizes the gesture started with, so a drag stays
    /// absolute rather than accumulating rounding.
    pub fn resize(&self, index: usize, from: (f64, f64), delta: f64) {
        let limits = self.limits.get_untracked();
        let (Some(before), Some(after)) = (limits.get(index), limits.get(index + 1)) else {
            return;
        };

        let (start_before, start_after) = from;

        // Clamp against the panel that is growing *and* the one that is shrinking: whichever
        // gives out first is what the divider can actually do.
        let wanted = (start_before + delta).clamp(before.min, before.max);
        let applied = wanted - start_before;
        let other = (start_after - applied).clamp(after.min, after.max);
        let applied = start_after - other;

        self.sizes.update(|sizes| {
            if let (Some(a), Some(b)) = (sizes.get(index).copied(), sizes.get(index + 1).copied())
                && (a - (start_before + applied)).abs() + (b - other).abs() > f64::EPSILON
            {
                sizes[index] = start_before + applied;
                sizes[index + 1] = other;
            }
        });
    }
}

pub fn use_resizable_group() -> ResizableGroupContext {
    expect_context::<ResizableGroupContext>()
}

#[component]
pub fn ResizableGroupRoot(
    /// Which way the panels are laid out: `Horizontal` puts them side by side.
    #[prop(default = Orientation::Horizontal)]
    direction: Orientation,
    /// The panel sizes, as percentages. Read it to persist a layout; write it to restore one.
    #[prop(optional, into)]
    sizes: RwSignal<Vec<f64>>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = ResizableGroupContext {
        direction,
        sizes,
        dragging: RwSignal::new(None),
        group_ref: AnyNodeRef::new(),
        limits: RwSignal::new(Vec::new()),
    };

    provide_context(ctx);

    // After the panels have registered — an effect runs once they have all rendered — and only
    // if the caller did not hand us a layout to restore.
    Effect::new(move |_| {
        let registered = ctx.limits.with(|limits| limits.len());
        if registered > 0 && ctx.sizes.with_untracked(|sizes| sizes.len()) != registered {
            ctx.distribute();
        }
    });

    view! {
        <Provider value=ctx>
            <div
                node_ref=ctx.group_ref
                data-slot="resizable-group"
                data-orientation=direction.as_str()
                class=class
            >
                {children()}
            </div>
        </Provider>
    }
}

#[component]
pub fn ResizablePanelRoot(
    /// The size to open at, as a percentage. Panels without one share whatever is left.
    #[prop(default = None, into)]
    default_size: Option<f64>,
    #[prop(default = 0.0)] min_size: f64,
    #[prop(default = 100.0)] max_size: f64,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_resizable_group();
    let index = ctx.register(min_size, max_size, default_size);

    view! {
        <div
            data-slot="resizable-panel"
            data-panel-index=index.to_string()
            // `flex-grow` is the percentage, with a zero basis, so the dividers' own widths come
            // out of the group before the panels share what is left.
            style=move || format!("flex: {} 1 0px; overflow: hidden;", ctx.size(index))
            class=class
        >
            {children()}
        </div>
    }
}

/// The divider after panel `index`. A `separator` in ARIA terms, focusable because it can be
/// moved with the arrow keys as well as dragged.
#[component]
pub fn ResizableHandleRoot(
    /// Which boundary this is: the one after the panel with this index.
    index: usize,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_resizable_group();

    // The pair of sizes the gesture started from, and the group's length at that moment. A drag
    // is absolute against both: adding up per-move deltas would drift as they round.
    let from = StoredValue::new((0.0, 0.0));
    let extent = StoredValue::new(0.0);

    let drag = use_drag(move |point: DragPoint| {
        let extent = extent.get_value();
        if extent <= 0.0 {
            return;
        }

        let travel = match ctx.direction {
            Orientation::Horizontal => point.dx,
            Orientation::Vertical => point.dy,
        };

        ctx.resize(index, from.get_value(), travel / extent * 100.0);
    })
    .on_start(move |_| ctx.dragging.set(Some(index)))
    .on_end(move |_| ctx.dragging.set(None));

    let start_drag = move |e: ev::PointerEvent| {
        if disabled.get_untracked() {
            return;
        }

        let measured = ctx.extent();
        if measured <= 0.0 {
            return;
        }

        extent.set_value(measured);
        from.set_value((ctx.size(index), ctx.size(index + 1)));
        drag.start(&e);
    };

    let on_keydown = move |e: ev::KeyboardEvent| {
        if disabled.get_untracked() {
            return;
        }

        let horizontal = ctx.direction == Orientation::Horizontal;
        let step = match (e.key().as_str(), horizontal) {
            ("ArrowLeft", true) | ("ArrowUp", false) => -KEYBOARD_STEP,
            ("ArrowRight", true) | ("ArrowDown", false) => KEYBOARD_STEP,
            _ => return,
        };

        e.prevent_default();
        ctx.resize(index, (ctx.size(index), ctx.size(index + 1)), step);
    };

    view! {
        <div
            data-slot="resizable-handle"
            data-handle-index=index.to_string()
            data-orientation=ctx.direction.as_str()
            data-dragging=move || (ctx.dragging.get() == Some(index)).then_some("")
            data-disabled=move || disabled.get().then_some("")
            role="separator"
            aria-orientation=match ctx.direction {
                // A separator between two panels laid out side by side is itself vertical, which
                // is the opposite of the group's own orientation.
                Orientation::Horizontal => "vertical",
                Orientation::Vertical => "horizontal",
            }
            aria-valuenow=move || ctx.size(index).round().to_string()
            aria-valuemin="0"
            aria-valuemax="100"
            tabindex=move || if disabled.get() { "-1" } else { "0" }
            on:pointerdown=start_drag
            on:keydown=on_keydown
            class=class
        >
            {children.map(|children| children())}
        </div>
    }
}
