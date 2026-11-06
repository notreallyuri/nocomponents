//! A menu summoned by right-clicking a region, anchored to the pointer.
//!
//! A dropdown menu whose anchor is a point rather than a button: the surface, the items, the
//! roving focus and the submenus are all `dropdown_menu.rs`'s, resolved from the same context type.
//!
//! The anchor is a zero-size element parked at the click point. Positioning against a real element
//! keeps flip and shift working, and keeps dismissal right — with the trigger *region* as the
//! reference, the layer counts it as inside and a left-click there fails to close the menu.

use crate::{
    primitives::{
        dropdown_menu::{
            MenuRoot, focus_element, focus_item_under_pointer, focus_menu_off_item,
            handle_menu_keys, placement_align, placement_side,
        },
        floating::{FloatingContext, FloatingRoot, floating_options, side_placement},
        long_press::{DEFAULT_DELAY_MS, use_long_press},
        roving_focus::use_roving_focus,
        typeahead::Typeahead,
    },
    utils::{
        controllable::controllable,
        types::{Align, Orientation, Side, SideOffset},
    },
};
use floating_ui_leptos::{UseFloatingReturn, use_floating};
use leptos::{context::Provider, ev, prelude::*};
use leptos_node_ref::AnyNodeRef;

pub type ContextMenuContext = FloatingContext;

pub fn use_context_menu() -> ContextMenuContext {
    expect_context::<ContextMenuContext>()
}

/// Where the pointer was when the menu was summoned, in viewport coordinates.
#[derive(Copy, Clone)]
pub struct ContextMenuPoint {
    pub x: RwSignal<f64>,
    pub y: RwSignal<f64>,
}

pub fn use_context_menu_point() -> ContextMenuPoint {
    expect_context::<ContextMenuPoint>()
}

#[component]
pub fn ContextMenuRoot(
    #[prop(optional, into)] class: Signal<String>,
    /// Whether the menu is open, held by the caller. Omit it and the root keeps its own.
    #[prop(default = None, into)]
    open: Option<RwSignal<bool>>,
    /// Whether the menu starts open. Ignored when `open` is given.
    #[prop(optional)]
    default_open: bool,
    /// Called with the new value whenever the menu opens or closes, however that came about.
    #[prop(optional, into)]
    on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    let context = ContextMenuContext {
        is_open: controllable(open, default_open, on_open_change),
        ..Default::default()
    };
    let point = ContextMenuPoint {
        x: RwSignal::new(0.0),
        y: RwSignal::new(0.0),
    };
    let stored_children = StoredValue::new(children);

    view! {
        // `trigger_aria` stays `None`: the region is not a button.
        <FloatingRoot class=class context=context>
            <Provider value=point>
                <Provider value=MenuRoot(context)>
                    <span
                        node_ref=context.trigger_ref
                        aria-hidden="true"
                        tabindex="-1"
                        style=move || {
                            format!(
                                "position: fixed; left: {}px; top: {}px; width: 0; height: 0; outline: none;",
                                point.x.get(),
                                point.y.get(),
                            )
                        }
                    ></span>
                    {stored_children.with_value(|c| c())}
                </Provider>
            </Provider>
        </FloatingRoot>
    }
}

/// How long after a long press has opened the menu a `contextmenu` event is taken to be the
/// browser's own echo of it. Android fires one a little after the press, where the cursor would
/// be, and without this it would reposition a menu that is already open.
const NATIVE_ECHO_WINDOW_MS: f64 = 1000.0;

/// The region that answers a right-click, or a long press on touch. Opening rather than toggling
/// is deliberate: the layer closes on the pointerdown, then this moves the anchor and opens it
/// where the pointer is.
///
/// `long_press` is on by default and opens the menu at the touch point once a touch or pen press
/// has been held still for `long_press_ms`, the same as `contextmenu` does at the cursor. Turning
/// it off removes that timer only; a browser that sends its own `contextmenu` for a long press
/// (Android) still opens the menu from it.
#[component]
pub fn ContextMenuTriggerRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    /// Whether a long press on touch opens the menu.
    #[prop(default = true.into(), into)]
    long_press: Signal<bool>,
    /// How long a press is held before it opens the menu, in milliseconds.
    #[prop(default = DEFAULT_DELAY_MS.into(), into)]
    long_press_ms: Signal<f64>,
    children: Children,
) -> impl IntoView {
    let ctx = use_context_menu();
    let point = use_context_menu_point();
    let opened_by_press = StoredValue::new(f64::NEG_INFINITY);

    let press = use_long_press(move |at| {
        if disabled.get_untracked() || !long_press.get_untracked() {
            return;
        }
        opened_by_press.set_value(js_sys::Date::now());
        point.x.set(at.client_x);
        point.y.set(at.client_y);
        ctx.open();
    })
    .delay(long_press_ms);

    view! {
        <div
            data-slot="context-menu-trigger"
            data-state=move || if ctx.is_open.get() { "open" } else { "closed" }
            data-disabled=move || disabled.get().then_some("true")
            style=move || {
                if !long_press.get() {
                    return String::new();
                }
                let mut style = String::from("-webkit-touch-callout: none;");
                if press.pressed.get() {
                    style.push_str(" -webkit-user-select: none; user-select: none;");
                }
                style
            }
            on:pointerdown=move |e: ev::PointerEvent| {
                if !disabled.get_untracked() && long_press.get_untracked() {
                    press.start(&e);
                }
            }
            on:contextmenu=move |e: ev::MouseEvent| {
                if disabled.get_untracked() {
                    return;
                }
                e.prevent_default();
                if js_sys::Date::now() - opened_by_press.get_value() < NATIVE_ECHO_WINDOW_MS {
                    return;
                }
                press.cancel();
                point.x.set(e.client_x() as f64);
                point.y.set(e.client_y() as f64);
                ctx.open();
            }
            class=class
        >
            {children()}
        </div>
    }
}

/// The menu surface, anchored to the click point. The dropdown's content root plus the explicit
/// `update` below, which is why it is not simply reused.
#[component]
pub fn ContextMenuContentRoot(
    #[prop(into, default = Side::Bottom.into())] side: Signal<Side>,
    #[prop(into, default = Align::Start.into())] align: Signal<Align>,
    #[prop(into, default = SideOffset(0.0).into())] side_offset: Signal<SideOffset>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_context_menu();
    let point = use_context_menu_point();
    let floating_ref = ctx.content_ref;

    let UseFloatingReturn {
        floating_styles,
        is_positioned,
        placement,
        update,
        ..
    } = use_floating(
        ctx.trigger_ref,
        floating_ref,
        floating_options(side_placement(side, align), side_offset),
    );

    // Auto-update watches resizes and scrolls; an anchor that teleports is neither, so without
    // this a second right-click re-opens the menu where it already was.
    Effect::new(move |_| {
        point.x.track();
        point.y.track();
        if ctx.is_open.get() {
            update();
        }
    });

    let menu_ref = AnyNodeRef::new();
    let roving = use_roving_focus(menu_ref, Orientation::Vertical);
    let typeahead = Typeahead::new();

    Effect::new(move |_| {
        if ctx.is_open.get() && is_positioned.get() {
            focus_element(menu_ref);
            roving.sync_tab_stop();
        }
    });

    view! {
        <div
            node_ref=floating_ref
            style=move || {
                if !is_positioned.get() {
                    format!("{} visibility: hidden;", floating_styles.get())
                } else {
                    floating_styles.get().to_string()
                }
            }
            class="fixed z-50 w-max pointer-events-none"
        >
            <div
                node_ref=menu_ref
                id=move || ctx.content_id.get()
                data-slot="context-menu-content"
                role="menu"
                tabindex="-1"
                on:keydown=move |e| {
                    if handle_menu_keys(&e, roving, typeahead, ctx) {
                        e.prevent_default();
                    }
                }
                on:focusin=move |_| roving.on_focus_in()
                on:pointermove=move |e| focus_item_under_pointer(&e, menu_ref)
                on:pointerleave=move |_| focus_menu_off_item(menu_ref)
                data-state=move || if ctx.is_open.get() { "open" } else { "closed" }
                data-align=move || placement_align(placement.get())
                data-side=move || placement_side(placement.get())
                class=class
            >
                {children()}
            </div>
        </div>
    }
}
