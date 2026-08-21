//! A navigation menu: a row of links, some of which open a panel.
//!
//! It is not a menubar, however similar the markup looks. A menubar is a set of *commands* and is
//! driven like one — one tab stop, arrows to move, items that act. A navigation menu is a set of
//! *links*: every link is tabbable, the panels are content rather than choices, and the whole
//! thing is built to be used with a pointer, which is why opening and closing are on a delay and
//! crossing the gap between a trigger and its panel does not close it.
//!
//! One panel is open at a time, named by its item's `value`, and the panels are positioned
//! against the menu itself rather than each item, so they all appear in the same place.

use crate::{
    primitives::{
        dismiss::{LayerBounds, use_dismissable_layer},
        roving_focus::use_roving_focus,
    },
    utils::{next_id, types::Orientation},
};
use leptos::{context::Provider, ev, prelude::*, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;
use std::time::Duration;
use web_sys::HtmlElement;

/// How long the pointer has to rest on a trigger before its panel opens, and how long the panel
/// survives the pointer leaving — long enough to cross the gap into it.
const OPEN_DELAY: u64 = 200;
const CLOSE_DELAY: u64 = 300;

/// How long a panel stays mounted after closing, so its exit animation can run. The same 150ms
/// the floating layers use.
const UNMOUNT_DELAY: u64 = 150;

#[derive(Copy, Clone)]
pub struct NavigationMenuContext {
    /// The `value` of the item whose panel is open.
    pub value: RwSignal<Option<String>>,
    /// The menu itself: what the panels are positioned against, and what counts as "inside" for
    /// an outside click.
    pub root_ref: AnyNodeRef,
    /// The list, which is the roving-focus group over the triggers.
    pub list_ref: AnyNodeRef,

    pub open_delay: u64,
    pub close_delay: u64,

    /// The one pending open-or-close. Shared, because a pointer moving along the row is
    /// cancelling one intent and starting another.
    pending: StoredValue<Option<TimeoutHandle>>,
}

impl NavigationMenuContext {
    pub fn is_open(&self, value: &str) -> bool {
        self.value.with(|open| open.as_deref() == Some(value))
    }

    pub fn cancel(&self) {
        if let Some(handle) = self.pending.get_value() {
            handle.clear();
            self.pending.set_value(None);
        }
    }

    pub fn open_now(&self, value: &str) {
        self.cancel();
        self.value.set(Some(value.to_string()));
    }

    pub fn close_now(&self) {
        self.cancel();
        self.value.set(None);
    }

    /// Opens after the delay — or at once if another panel is already open, since by then the
    /// menu is showing and hesitating just looks broken.
    pub fn open_soon(&self, value: &str) {
        self.cancel();

        if self.value.get_untracked().is_some() {
            self.value.set(Some(value.to_string()));
            return;
        }

        let value = value.to_string();
        let signal = self.value;
        if let Ok(handle) = set_timeout_with_handle(
            move || signal.set(Some(value)),
            Duration::from_millis(self.open_delay),
        ) {
            self.pending.set_value(Some(handle));
        }
    }

    pub fn close_soon(&self) {
        self.cancel();

        let signal = self.value;
        if let Ok(handle) = set_timeout_with_handle(
            move || signal.set(None),
            Duration::from_millis(self.close_delay),
        ) {
            self.pending.set_value(Some(handle));
        }
    }
}

pub fn use_navigation_menu() -> NavigationMenuContext {
    expect_context::<NavigationMenuContext>()
}

/// An item's own name and the id of its panel, so the trigger can point `aria-controls` at it.
#[derive(Copy, Clone)]
pub struct NavigationMenuItemContext {
    pub value: StoredValue<String>,
    pub trigger_id: StoredValue<String>,
    pub content_id: StoredValue<String>,
}

pub fn use_navigation_menu_item() -> NavigationMenuItemContext {
    expect_context::<NavigationMenuItemContext>()
}

#[component]
pub fn NavigationMenuRoot(
    /// Which panel is open, for a caller that wants to drive the menu or watch it.
    #[prop(optional, into)]
    value: RwSignal<Option<String>>,
    #[prop(default = OPEN_DELAY)] open_delay: u64,
    #[prop(default = CLOSE_DELAY)] close_delay: u64,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = NavigationMenuContext {
        value,
        root_ref: AnyNodeRef::new(),
        list_ref: AnyNodeRef::new(),
        open_delay,
        close_delay,
        pending: StoredValue::new(None),
    };

    // The dismissable layer stack speaks in booleans, and this menu's state is "which one", so
    // the two are kept in step rather than duplicated: the flag follows the value, and dismissing
    // clears the value.
    let is_open = RwSignal::new(value.get_untracked().is_some());

    Effect::new(move |_| {
        let open = ctx.value.get().is_some();
        if is_open.get_untracked() != open {
            is_open.set(open);
        }
    });

    // Everything — triggers and panels alike — lives inside the menu, so it is its own bounds.
    use_dismissable_layer(
        is_open,
        LayerBounds::new(ctx.root_ref, ctx.root_ref),
        move || ctx.close_now(),
    );

    on_cleanup(move || ctx.cancel());

    view! {
        <Provider value=ctx>
            <nav
                node_ref=ctx.root_ref
                data-slot="navigation-menu"
                data-state=move || if ctx.value.get().is_some() { "open" } else { "closed" }
                // Leaving is decided here rather than on each trigger. Per-trigger, moving along
                // the row fires a `pointerleave` on the one you came from *after* the one you
                // arrived at has opened its panel, and the stale close then takes it down again.
                // Enter and leave are DOM-scoped, so a panel counts as inside the menu however
                // far below it is drawn; the delay is what carries the pointer over the gap.
                on:pointerenter=move |_| ctx.cancel()
                on:pointerleave=move |_| ctx.close_soon()
                class=class
            >
                {children()}
            </nav>
        </Provider>
    }
}

#[component]
pub fn NavigationMenuListRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_navigation_menu();

    // Horizontal arrows move along the row. Unlike a menubar this is not the only way in — every
    // trigger and link is tabbable, because they are navigation, not commands.
    let roving = use_roving_focus(ctx.list_ref, Orientation::Horizontal);

    view! {
        <ul
            node_ref=ctx.list_ref
            data-slot="navigation-menu-list"
            on:keydown=move |e| {
                if roving.on_keydown(&e.key()) {
                    e.prevent_default();
                }
            }
            class=class
        >
            {children()}
        </ul>
    }
}

#[component]
pub fn NavigationMenuItemRoot(
    /// What this item's panel is called in the menu's `value`. Unique within the menu.
    #[prop(into)]
    value: String,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let item = NavigationMenuItemContext {
        value: StoredValue::new(value),
        trigger_id: StoredValue::new(next_id("nav-trigger")),
        content_id: StoredValue::new(next_id("nav-content")),
    };

    view! {
        <Provider value=item>
            <li data-slot="navigation-menu-item" class=class>
                {children()}
            </li>
        </Provider>
    }
}

#[component]
pub fn NavigationMenuTriggerRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let ctx = use_navigation_menu();
    let item = use_navigation_menu_item();

    let is_open = Signal::derive(move || item.value.with_value(|value| ctx.is_open(value)));

    // Whether the pointer that is on this trigger is the one that opened its panel. Without it,
    // arriving and clicking in one motion opens on `pointerenter` and then closes on `click`,
    // which reads as the panel refusing to open. Consumed by that first click, so a second one
    // still closes.
    let opened_on_arrival = StoredValue::new(false);

    view! {
        <button
            type="button"
            id=item.trigger_id.get_value()
            data-slot="navigation-menu-trigger"
            data-roving-item=""
            data-value=item.value.get_value()
            aria-expanded=move || is_open.get().to_string()
            aria-controls=move || is_open.get().then(|| item.content_id.get_value())
            disabled=disabled
            data-state=move || if is_open.get() { "open" } else { "closed" }
            on:click=move |_| {
                if opened_on_arrival.get_value() {
                    opened_on_arrival.set_value(false);
                    return;
                }
                if is_open.get_untracked() {
                    ctx.close_now();
                } else {
                    item.value.with_value(|value| ctx.open_now(value));
                }
            }
            on:pointerenter=move |_| {
                opened_on_arrival.set_value(!is_open.get_untracked());
                item.value.with_value(|value| ctx.open_soon(value));
            }
            on:pointerleave=move |_| opened_on_arrival.set_value(false)
            // Focus opens nothing on its own — tabbing along a row of links must not throw
            // panels open — but Down asks for the panel explicitly.
            on:keydown=move |e| {
                match e.key().as_str() {
                    "ArrowDown" | "Enter" | " " => {
                        e.prevent_default();
                        item.value.with_value(|value| ctx.open_now(value));
                    }
                    "Escape" => ctx.close_now(),
                    _ => {}
                }
            }
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn NavigationMenuContentRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_navigation_menu();
    let item = use_navigation_menu_item();
    let children = StoredValue::new(children);

    let is_open = Signal::derive(move || item.value.with_value(|value| ctx.is_open(value)));

    // The same trailing mount the floating layers use: the panel outlives the state by long
    // enough to animate its way out.
    let is_mounted = RwSignal::new(is_open.get_untracked());
    let unmount: StoredValue<Option<TimeoutHandle>> = StoredValue::new(None);

    let cancel_unmount = move || {
        if let Some(handle) = unmount.get_value() {
            handle.clear();
            unmount.set_value(None);
        }
    };

    Effect::new(move |_| {
        if is_open.get() {
            cancel_unmount();
            is_mounted.set(true);
        } else if let Ok(handle) = set_timeout_with_handle(
            move || is_mounted.set(false),
            Duration::from_millis(UNMOUNT_DELAY),
        ) {
            cancel_unmount();
            unmount.set_value(Some(handle));
        }
    });

    on_cleanup(cancel_unmount);

    view! {
        <Show when=move || is_mounted.get()>
            <div
                id=item.content_id.get_value()
                data-slot="navigation-menu-content"
                aria-labelledby=item.trigger_id.get_value()
                data-state=move || if is_open.get() { "open" } else { "closed" }
                on:keydown=move |e| {
                    if e.key() == "Escape" {
                        ctx.close_now();
                        focus_trigger(ctx, item);
                    }
                }
                class=class
            >
                {children.with_value(|children| children())}
            </div>
        </Show>
    }
}

/// Puts focus back on the item's trigger, for Escape out of a panel.
fn focus_trigger(ctx: NavigationMenuContext, item: NavigationMenuItemContext) {
    let Some(list) = ctx.list_ref.get_untracked() else {
        return;
    };
    let selector = format!(
        "[data-slot=navigation-menu-trigger][data-value=\"{}\"]",
        item.value.get_value()
    );

    if let Ok(Some(trigger)) = list.query_selector(&selector)
        && let Ok(trigger) = trigger.dyn_into::<HtmlElement>()
    {
        let _ = trigger.focus();
    }
}

#[component]
pub fn NavigationMenuLinkRoot(
    #[prop(optional, into)] href: Signal<String>,
    /// Whether this is the page the reader is on, announced as `aria-current` and exposed as
    /// `data-active`.
    #[prop(optional, into)]
    active: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    /// Render the link as something else — a router's `A`, which owns its own `href`. The
    /// callback is handed the click handler that closes the menu.
    #[prop(default = None, into)]
    render: Option<Callback<Callback<ev::MouseEvent>, AnyView>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_navigation_menu();
    let on_click = Callback::new(move |_: ev::MouseEvent| ctx.close_now());

    match render {
        Some(render) => leptos::either::Either::Left(render.run(on_click)),
        None => leptos::either::Either::Right(view! {
            <a
                href=move || href.get()
                data-slot="navigation-menu-link"
                data-active=move || active.get().then_some("")
                aria-current=move || active.get().then_some("page")
                on:click=move |e| on_click.run(e)
                class=class
            >
                {match children {
                    Some(children) => leptos::either::Either::Left(children()),
                    None => leptos::either::Either::Right(""),
                }}
            </a>
        }),
    }
}

/// The mark that slides under whichever trigger is open. Positioned by measuring that trigger,
/// which is the only way to know where it is: the triggers are laid out by the caller's classes.
#[component]
pub fn NavigationMenuIndicatorRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let ctx = use_navigation_menu();
    let children = StoredValue::new(children);

    let geometry = RwSignal::new(None::<(f64, f64)>);

    Effect::new(move |_| {
        let Some(value) = ctx.value.get() else {
            geometry.set(None);
            return;
        };
        let Some(list) = ctx.list_ref.get() else {
            return;
        };

        let selector = format!("[data-slot=navigation-menu-trigger][data-value=\"{value}\"]");
        if let Ok(Some(trigger)) = list.query_selector(&selector) {
            // Client rects, not `offsetLeft`: that is measured from whichever ancestor happens to
            // be positioned, and each item usually is.
            let trigger = trigger.get_bounding_client_rect();
            let list = list.get_bounding_client_rect();
            geometry.set(Some((trigger.left() - list.left(), trigger.width())));
        }
    });

    view! {
        <Show when=move || geometry.get().is_some()>
            <div
                data-slot="navigation-menu-indicator"
                data-state=move || if ctx.value.get().is_some() { "open" } else { "closed" }
                style=move || match geometry.get() {
                    Some((left, width)) => format!("left: {left}px; width: {width}px;"),
                    None => String::new(),
                }
                class=class
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </div>
        </Show>
    }
}
