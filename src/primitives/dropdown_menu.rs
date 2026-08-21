use crate::{
    primitives::{
        floating::{
            ArrowOpen, FloatingContext, FloatingRoot, FloatingTrigger, OpenFocus, TriggerAria,
        },
        roving_focus::{RovingFocus, use_roving_focus},
        typeahead::Typeahead,
    },
    utils::{
        get_placement,
        types::{Align, Direction, Orientation, Side, SideOffset},
    },
};
use floating_ui_leptos::{
    Flip, FlipOptions, MiddlewareVec, Offset, OffsetOptions, Placement, Shift, ShiftOptions,
    Strategy, UseFloatingOptions, UseFloatingReturn, use_floating,
};
use leptos::{
    context::Provider, either::Either, ev, portal::Portal, prelude::*, wasm_bindgen::JsCast,
};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use std::time::Duration;
use web_sys::HtmlElement;

pub type DropdownMenuContext = FloatingContext;

pub fn use_dropdown() -> DropdownMenuContext {
    expect_context::<DropdownMenuContext>()
}

/// The outermost menu's context, reachable from inside a submenu, whose own `FloatingRoot`
/// shadows it. Selecting an item has to close the whole tree, not just the panel it lives in.
#[derive(Copy, Clone)]
pub(crate) struct MenuRoot(pub(crate) DropdownMenuContext);

#[component]
pub fn DropdownMenuRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let context = DropdownMenuContext::default();
    let stored_children = StoredValue::new(children);

    view! {
        <FloatingRoot
            class=class
            context=context
            trigger_aria=TriggerAria::Popup("menu")
            arrow_open=ArrowOpen::OpenIntoList
        >
            <Provider value=MenuRoot(context)>{stored_children.with_value(|c| c())}</Provider>
        </FloatingRoot>
    }
}

#[component]
pub fn DropdownMenuTriggerRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(default = None, into)] render: Option<
        Callback<(AnyNodeRef, Callback<ev::MouseEvent>), AnyView>,
    >,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_dropdown();

    view! {
        <FloatingTrigger context=ctx class=class disabled=disabled render=render>
            {match children {
                Some(child) => Either::Left(child()),
                None => Either::Right(""),
            }}
        </FloatingTrigger>
    }
}

/// Focuses a menu surface itself rather than an item, so a mouse-opened menu paints no focus
/// ring; the first arrow key steps in from there.
pub(crate) fn focus_element(node_ref: AnyNodeRef) {
    if let Some(el) = node_ref.get_untracked()
        && let Ok(el) = el.dyn_into::<HtmlElement>()
    {
        let _ = el.focus();
    }
}

/// Focuses the first item of a menu surface, used when stepping into a submenu.
fn focus_first_item(content_ref: AnyNodeRef) {
    if let Some(content) = content_ref.get_untracked()
        && let Ok(Some(first)) = content.query_selector("[data-roving-item]:not([data-disabled])")
        && let Ok(first) = first.dyn_into::<HtmlElement>()
    {
        let _ = first.set_attribute("tabindex", "0");
        let _ = first.focus();
    }
}

/// Keys every menu surface handles alike: arrows and Home/End move, Enter and Space activate,
/// Tab leaves, and anything printable jumps to the item starting with it.
pub(crate) fn handle_menu_keys(
    e: &ev::KeyboardEvent,
    roving: RovingFocus,
    typeahead: Typeahead,
    ctx: DropdownMenuContext,
) -> bool {
    let key = e.key();

    if roving.on_keydown(&key) {
        return true;
    }

    match key.as_str() {
        // An item is a div, so Enter and Space become the click a button would get for free.
        "Enter" | " " => {
            if let Some(active) = document().active_element()
                && active.has_attribute("data-roving-item")
                && let Ok(active) = active.dyn_into::<HtmlElement>()
            {
                active.click();
                return true;
            }
            false
        }
        // Tabbing out closes the menu rather than leaving it behind the focus.
        "Tab" => {
            ctx.close();
            false
        }
        // Space is spoken for above, so typeahead sees the rest of the printable keys.
        _ => typeahead.push(&key, roving.container()),
    }
}

/// `data-side` / `data-align` for a resolved placement; shared with `context_menu.rs`.
pub(crate) fn placement_side(placement: Placement) -> &'static str {
    match placement {
        Placement::Top | Placement::TopStart | Placement::TopEnd => "top",
        Placement::Bottom | Placement::BottomStart | Placement::BottomEnd => "bottom",
        Placement::Left | Placement::LeftStart | Placement::LeftEnd => "left",
        Placement::Right | Placement::RightStart | Placement::RightEnd => "right",
    }
}

pub(crate) fn placement_align(placement: Placement) -> &'static str {
    match placement {
        Placement::TopStart
        | Placement::BottomStart
        | Placement::LeftStart
        | Placement::RightStart => "start",
        Placement::TopEnd | Placement::BottomEnd | Placement::LeftEnd | Placement::RightEnd => {
            "end"
        }
        _ => "center",
    }
}

#[component]
pub fn DropdownMenuPortalRoot(children: ChildrenFn) -> impl IntoView {
    let ctx = use_dropdown();
    let stored_children = StoredValue::new(children);

    view! {
        <Portal>
            <Show when=move || ctx.is_mounted.get()>{stored_children.with_value(|c| c())}</Show>
        </Portal>
    }
}

#[component]
pub fn DropdownMenuContentRoot(
    #[prop(optional)] side: Side,
    #[prop(optional)] align: Align,
    #[prop(optional)] side_offset: SideOffset,
    /// What Left and Right do inside this surface. A dropdown ignores them; a menubar's menu
    /// steps to the menu beside it, which is why this is a callback rather than a rule here.
    #[prop(default = None, into)]
    on_arrow_horizontal: Option<Callback<Direction>>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_dropdown();
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

    let menu_ref = AnyNodeRef::new();
    let roving = use_roving_focus(menu_ref, Orientation::Vertical);
    let typeahead = Typeahead::new();

    Effect::new(move |_| {
        if ctx.is_open.get() && is_positioned.get() {
            match ctx.open_focus.get_untracked() {
                // Opened with a pointer or Enter: focus the menu itself, so nothing wears a
                // ring until an arrow asks for it.
                OpenFocus::Auto => {
                    focus_element(menu_ref);
                    roving.sync_tab_stop();
                }
                // Opened with an arrow, which asks to be in the list already.
                OpenFocus::First => roving.focus_first(),
                OpenFocus::Last => roving.focus_last(),
            }
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
                aria-labelledby=move || ctx.trigger_id.get()
                tabindex="-1"
                on:keydown=move |e| {
                    if let Some(on_arrow_horizontal) = on_arrow_horizontal {
                        let step = match e.key().as_str() {
                            "ArrowLeft" => Some(Direction::Left),
                            "ArrowRight" => Some(Direction::Right),
                            _ => None,
                        };
                        if let Some(step) = step {
                            e.prevent_default();
                            on_arrow_horizontal.run(step);
                            return;
                        }
                    }
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

#[component]
pub fn DropdownMenuItemRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(default = None, into)] on_click: Option<Callback<ev::MouseEvent>>,
    children: Children,
) -> impl IntoView {
    let ctx = use_dropdown();

    let menu_root = use_context::<MenuRoot>();

    let handle_click = move |e: ev::MouseEvent| {
        if let Some(cb) = on_click {
            cb.run(e);
        }
        ctx.close();
        if let Some(root) = menu_root {
            root.0.close();
        }
    };

    view! {
        <div role="menuitem" tabindex="-1" data-roving-item="" on:click=handle_click class=class>
            {children()}
        </div>
    }
}

#[derive(Copy, Clone)]
struct SubHoverState(RwSignal<bool>);

#[component]
pub fn DropdownMenuSubRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    provide_context(SubHoverState(RwSignal::new(false)));
    view! { <FloatingRoot class=class>{children()}</FloatingRoot> }
}

#[component]
pub fn DropdownMenuSubTriggerRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let ctx = use_dropdown();
    let hover = expect_context::<SubHoverState>();

    view! {
        <button
            type="button"
            role="menuitem"
            aria-haspopup="menu"
            aria-expanded=move || ctx.is_open.get().to_string()
            tabindex="-1"
            data-roving-item=""
            disabled=disabled
            node_ref=ctx.trigger_ref
            class=class
            data-state=move || if ctx.is_open.get() { "open" } else { "closed" }
            on:keydown=move |e| {
                if matches!(e.key().as_str(), "ArrowRight" | "Enter" | " ") {
                    e.prevent_default();
                    e.stop_propagation();
                    ctx.open();
                    set_timeout(
                        move || focus_first_item(ctx.content_ref),
                        Duration::from_millis(16),
                    );
                }
            }
            on:click=move |e| {
                e.stop_propagation();
                ctx.toggle();
            }
            on:pointerenter=move |_| {
                hover.0.set(true);
                ctx.open();
            }
            on:pointerleave=move |_| {
                hover.0.set(false);
                set_timeout(
                    move || {
                        if !hover.0.get() {
                            ctx.close();
                        }
                    },
                    Duration::from_millis(50),
                );
            }
        >
            {children()}
        </button>
    }
}

#[component]
pub fn DropdownMenuSubContentRoot(
    #[prop(optional)] side_offset: SideOffset,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_dropdown();
    let floating_ref = ctx.content_ref;
    let hover = expect_context::<SubHoverState>();

    let middleware: MiddlewareVec = vec![
        Box::new(Offset::new(OffsetOptions::Value(side_offset.0))),
        Box::new(Flip::new(FlipOptions::default())),
        Box::new(Shift::new(ShiftOptions::default())),
    ];

    let menu_ref = AnyNodeRef::new();
    let roving = use_roving_focus(menu_ref, Orientation::Vertical);
    let typeahead = Typeahead::new();

    let UseFloatingReturn {
        floating_styles,
        is_positioned,
        placement,
        ..
    } = use_floating(
        ctx.trigger_ref,
        floating_ref,
        UseFloatingOptions::default()
            .placement(Placement::RightStart)
            .strategy(Strategy::Fixed)
            .while_elements_mounted_auto_update()
            .middleware(SendWrapper::new(middleware)),
    );

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
            on:pointerenter=move |_| {
                hover.0.set(true);
            }
            on:pointerleave=move |_| {
                hover.0.set(false);
                set_timeout(
                    move || {
                        if !hover.0.get() {
                            ctx.close();
                        }
                    },
                    Duration::from_millis(50),
                );
            }
        >
            <div
                node_ref=menu_ref
                id=move || ctx.content_id.get()
                role="menu"
                aria-labelledby=move || ctx.trigger_id.get()
                tabindex="-1"
                on:keydown=move |e| {
                    if e.key() == "ArrowLeft" {
                        e.prevent_default();
                        e.stop_propagation();
                        ctx.close();
                        focus_element(ctx.trigger_ref);
                        return;
                    }
                    if handle_menu_keys(&e, roving, typeahead, ctx) {
                        e.prevent_default();
                        e.stop_propagation();
                    }
                }
                on:focusin=move |_| roving.on_focus_in()
                data-state=move || if ctx.is_open.get() { "open" } else { "closed" }
                data-side=move || match placement.get() {
                    Placement::Top | Placement::TopStart | Placement::TopEnd => "top",
                    Placement::Bottom | Placement::BottomStart | Placement::BottomEnd => "bottom",
                    Placement::Left | Placement::LeftStart | Placement::LeftEnd => "left",
                    Placement::Right | Placement::RightStart | Placement::RightEnd => "right",
                }
                class=class
            >
                {children()}
            </div>
        </div>
    }
}
