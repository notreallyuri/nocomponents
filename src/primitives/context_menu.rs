//! A menu summoned by right-clicking a region, anchored to the pointer.
//!
//! Almost none of this is new. A context menu is a dropdown menu whose anchor is a point rather
//! than a button, so the surface, the items, the roving focus and the submenus are all
//! `dropdown_menu.rs`'s — they resolve `FloatingContext` out of the context, and this module
//! provides the same type. What is different is only where the menu is anchored and what opens it.
//!
//! The anchor is a zero-size element parked at the click point and handed to floating-ui as the
//! reference. Positioning against a real element rather than raw coordinates is what keeps flip
//! and shift working: a menu summoned near the bottom of the window flips above the pointer for
//! free. It also fixes dismissal, which is the part that goes wrong if the trigger *region* is
//! used as the reference — the dismissable layer treats the trigger as "inside", so a left-click
//! in the region would fail to close the menu it just opened.

use crate::{
    primitives::{
        dropdown_menu::{
            MenuRoot, focus_element, handle_menu_keys, placement_align, placement_side,
        },
        floating::{FloatingContext, FloatingRoot},
        roving_focus::use_roving_focus,
        typeahead::Typeahead,
    },
    utils::{
        get_placement,
        types::{Align, Orientation, Side, SideOffset},
    },
};
use floating_ui_leptos::{
    Flip, FlipOptions, MiddlewareVec, Offset, OffsetOptions, Shift, ShiftOptions, Strategy,
    UseFloatingOptions, UseFloatingReturn, use_floating,
};
use leptos::{context::Provider, ev, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;

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
    children: ChildrenFn,
) -> impl IntoView {
    let context = ContextMenuContext::default();
    let point = ContextMenuPoint {
        x: RwSignal::new(0.0),
        y: RwSignal::new(0.0),
    };
    let stored_children = StoredValue::new(children);

    view! {
        // `trigger_aria` stays `None`: the region is not a button, and announcing `aria-expanded`
        // on a div nobody can focus would be a lie.
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

/// The region that answers a right-click.
///
/// Re-opening while already open is deliberate rather than a toggle: the dismissable layer sees
/// the pointerdown land outside the menu and closes it, then this moves the anchor and opens it
/// again, so a second right-click somewhere else moves the menu instead of dismissing it.
#[component]
pub fn ContextMenuTriggerRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let ctx = use_context_menu();
    let point = use_context_menu_point();

    view! {
        <div
            data-slot="context-menu-trigger"
            data-state=move || if ctx.is_open.get() { "open" } else { "closed" }
            data-disabled=move || disabled.get().then_some("true")
            on:contextmenu=move |e: ev::MouseEvent| {
                if disabled.get_untracked() {
                    return;
                }
                e.prevent_default();
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

/// The menu surface, anchored to the click point.
///
/// This is the dropdown's content root with one thing added, and that one thing is the reason it
/// is not simply reused: floating-ui's auto-update watches for resizes and scrolls, and an anchor
/// that *teleports* to a new pointer position is neither. Without the explicit `update` below, a
/// second right-click somewhere else re-opens the menu exactly where it already was.
#[component]
pub fn ContextMenuContentRoot(
    #[prop(default = Side::Bottom)] side: Side,
    #[prop(default = Align::Start)] align: Align,
    #[prop(default = SideOffset(0.0))] side_offset: SideOffset,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_context_menu();
    let point = use_context_menu_point();
    let floating_ref = ctx.content_ref;

    let middleware: MiddlewareVec = vec![
        Box::new(Offset::new(OffsetOptions::Value(side_offset.0))),
        Box::new(Flip::new(FlipOptions::default())),
        Box::new(Shift::new(ShiftOptions::default())),
    ];

    let UseFloatingReturn {
        floating_styles,
        is_positioned,
        placement,
        update,
        ..
    } = use_floating(
        ctx.trigger_ref,
        floating_ref,
        UseFloatingOptions::default()
            .placement(get_placement(side, align))
            .strategy(Strategy::Fixed)
            .while_elements_mounted_auto_update()
            .middleware(SendWrapper::new(middleware)),
    );

    // floating-ui's auto-update watches for resizes and scrolls; an anchor that *teleports* to a
    // new pointer position is neither, so a second right-click elsewhere would otherwise re-open
    // the menu exactly where it already was. Verified: removing this pins the menu to the first
    // click point.
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
                role="menu"
                tabindex="-1"
                on:keydown=move |e| {
                    if handle_menu_keys(&e, roving, typeahead, ctx) {
                        e.prevent_default();
                    }
                }
                on:focusin=move |_| roving.on_focus_in()
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
