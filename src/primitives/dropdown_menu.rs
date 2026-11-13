use crate::{
    primitives::{
        floating::{
            ArrowOpen, FloatingContext, FloatingPortal, FloatingRoot, FloatingTrigger, OpenFocus,
            TriggerAria, TriggerRender, floating_options, floating_options_unless_sheet,
            shell_class, side_placement,
        },
        roving_focus::{RovingFocus, use_roving_focus},
        sheet_drag::{SheetFrame, use_sheet_grab},
        typeahead::Typeahead,
    },
    utils::{
        controllable::controllable,
        sheet::{
            SHEET_ROLE_STYLE, SheetScrim, close_when_presentation_changes, presentation_attr,
            use_sheet_presentation,
        },
        types::{Align, Direction, Orientation, Side, SideOffset},
    },
};
use floating_ui_leptos::{Placement, UseFloatingReturn, use_floating};
use leptos::{context::Provider, either::Either, ev, prelude::*, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;
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
    let context = DropdownMenuContext {
        is_open: controllable(open, default_open, on_open_change),
        ..Default::default()
    };
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
    #[prop(default = None, into)] render: Option<Callback<TriggerRender, AnyView>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_dropdown();

    view! {
        <FloatingTrigger
            context=ctx
            class=class
            disabled=disabled
            data_slot="dropdown-menu-trigger"
            render=render
        >
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

/// Moves focus to the item under the pointer, so hovering and the arrow keys are one highlight
/// rather than two: without it, hovering "Right" and pressing Enter activates whatever the keys
/// last reached. A disabled item is not focused, and neither is the gap between items.
pub(crate) fn focus_item_under_pointer(e: &ev::PointerEvent, menu_ref: AnyNodeRef) {
    let Some(menu) = menu_ref.get_untracked() else {
        return;
    };
    let Some(target) = e
        .target()
        .and_then(|target| target.dyn_into::<web_sys::Element>().ok())
    else {
        return;
    };
    let Ok(Some(item)) = target.closest("[data-roving-item]") else {
        return;
    };
    if !menu.contains(Some(&item))
        || item.has_attribute("disabled")
        || item.has_attribute("data-disabled")
        || item.get_attribute("aria-disabled").as_deref() == Some("true")
    {
        return;
    }
    if document().active_element().as_ref() == Some(&item) {
        return;
    }
    if let Ok(item) = item.dyn_into::<HtmlElement>() {
        let _ = item.focus();
    }
}

/// When the pointer leaves the surface while an item holds focus, hands focus back to the surface
/// itself: the highlight the pointer was driving goes out with it.
pub(crate) fn focus_menu_off_item(menu_ref: AnyNodeRef) {
    let on_item = document()
        .active_element()
        .is_some_and(|active| active.has_attribute("data-roving-item"));
    if on_item {
        focus_element(menu_ref);
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
pub fn DropdownMenuPortalRoot(
    /// Mount the layer in this element instead of `<body>` — a sticky or fixed ancestor it should
    /// move with. The fullscreen element still wins when the container is not inside it.
    #[prop(default = None)]
    container: Option<AnyNodeRef>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_dropdown();
    let stored_children = StoredValue::new(children);

    view! {
        <FloatingPortal container=container>
            <Show when=move || ctx.is_mounted.get()>{stored_children.with_value(|c| c())}</Show>
        </FloatingPortal>
    }
}

#[component]
pub fn DropdownMenuContentRoot(
    #[prop(optional, into)] side: Signal<Side>,
    #[prop(optional, into)] align: Signal<Align>,
    #[prop(optional, into)] side_offset: Signal<SideOffset>,
    /// What Left and Right do inside this surface. A dropdown ignores them; a menubar's menu
    /// steps to the menu beside it, which is why this is a callback rather than a rule here.
    #[prop(optional, into)]
    on_arrow_horizontal: Option<Callback<Direction>>,
    #[prop(optional, into)] class: Signal<String>,
    /// Below the `md` breakpoint, present the menu as a bottom sheet: a scrim, the menu on the
    /// bottom edge, no positioning. The items, the keys and the dismissal are the same. Written as
    /// `data-presentation="sheet"` (`"floating"` otherwise). Off by default.
    #[prop(optional, into)]
    mobile_sheet: Signal<bool>,
    /// The title in the sheet's header row, beside the close button. Empty shows no title.
    #[prop(optional, into)]
    sheet_title: Signal<String>,
    /// The heights the sheet rests at, as fractions of the viewport height — `vec![0.4, 0.9]` is a
    /// peek and a tall. The sheet is as tall as the highest, opens at the first, and a drag on its
    /// header settles on the nearest (a flick moves one, a release below the lowest dismisses).
    /// Written as `data-snap`, the index of the active one. Empty is a sheet as tall as its content.
    #[prop(optional, into)]
    sheet_snap_points: Signal<Vec<f64>>,
    /// Whether the sheet's header has a close button. On by default; it exists only in a sheet.
    #[prop(default = true.into(), into)]
    sheet_show_close: Signal<bool>,
    /// The close button's accessible name.
    #[prop(default = "Close".into(), into)]
    sheet_close_label: Signal<String>,
    /// What the close button draws; a plain cross when none is given. The styled layer passes its
    /// own glyph.
    #[prop(default = None, into)]
    sheet_close_icon: Option<ViewFn>,
    /// Whether dragging the sheet's header row moves and dismisses it. Off, the header is inert and
    /// leaves the browser's touch scrolling alone; the scrim, Escape and the close button remain.
    #[prop(default = true.into(), into)]
    sheet_draggable: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let ctx = use_dropdown();
    let floating_ref = ctx.content_ref;
    let sheet = use_sheet_presentation(mobile_sheet);
    let close = Callback::new(move |_| ctx.close());
    let as_sheet = sheet.get_untracked();
    let grab = use_sheet_grab(
        sheet,
        ctx.is_open.into(),
        sheet_snap_points,
        sheet_draggable,
        close,
    );

    let UseFloatingReturn {
        floating_styles,
        is_positioned,
        placement,
        ..
    } = use_floating(
        ctx.trigger_ref,
        floating_ref,
        floating_options_unless_sheet(side_placement(side, align), side_offset, sheet),
    );

    close_when_presentation_changes(sheet, close);

    let ready = Signal::derive(move || sheet.get() || is_positioned.get());

    let menu_ref = AnyNodeRef::new();
    let roving = use_roving_focus(menu_ref, Orientation::Vertical);
    let typeahead = Typeahead::new();

    Effect::new(move |_| {
        if ctx.is_open.get() && ready.get() {
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

    let role_element = view! {
        <div
            node_ref=menu_ref
            data-slot="dropdown-menu-content"
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
            on:pointermove=move |e| focus_item_under_pointer(&e, menu_ref)
            on:pointerleave=move |_| focus_menu_off_item(menu_ref)
            data-state=move || if ctx.is_open.get() { "open" } else { "closed" }
            data-presentation=move || presentation_attr(sheet.get())
            data-align=move || (!sheet.get()).then(|| placement_align(placement.get()))
            data-side=move || (!sheet.get()).then(|| placement_side(placement.get()))
            class=move || if as_sheet { String::new() } else { class.get() }
            style=if as_sheet { SHEET_ROLE_STYLE } else { "" }
        >
                {children()}
        </div>
    };

    view! {
        <SheetScrim sheet=sheet open=ctx.is_open.into() />
        <div
            node_ref=floating_ref
            style=move || {
                if sheet.get() {
                    String::new()
                } else if !is_positioned.get() {
                    format!("{} visibility: hidden;", floating_styles.get())
                } else {
                    floating_styles.get().to_string()
                }
            }
            class=move || shell_class(sheet.get(), "fixed z-50 w-max pointer-events-none")
        >
            {if as_sheet {
                view! {
                    <SheetFrame
                        grab=grab
                        sheet=sheet
                        open=ctx.is_open.into()
                        close=close
                        class=class
                        title=sheet_title
                        show_close=sheet_show_close
                        close_label=sheet_close_label
                        close_icon=sheet_close_icon
                    >
                        {role_element}
                    </SheetFrame>
                }
                    .into_any()
            } else {
                role_element.into_any()
            }}
        </div>
    }
}

/// How an item reads, written as `data-variant` so the styled layer can key a destructive row off
/// the attribute instead of a class the caller types by hand.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum DropdownMenuItemVariant {
    #[default]
    Default,
    Destructive,
}

impl DropdownMenuItemVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Destructive => "destructive",
        }
    }
}

/// Closes the panel an item lives in and, from inside a submenu, the whole tree above it.
fn close_tree(ctx: DropdownMenuContext, menu_root: Option<MenuRoot>) {
    ctx.close();
    if let Some(root) = menu_root {
        root.0.close();
    }
}

/// What follows a selection: close the tree, or, when `close_on_select` is off, leave it open with
/// focus on the item just chosen (a click already focuses it; this covers a pointer that did not).
fn finish_select(
    target: Option<web_sys::EventTarget>,
    close_on_select: Signal<bool>,
    ctx: DropdownMenuContext,
    menu_root: Option<MenuRoot>,
) {
    if close_on_select.get_untracked() {
        close_tree(ctx, menu_root);
    } else if let Some(item) = target.and_then(|t| t.dyn_into::<HtmlElement>().ok()) {
        let _ = item.focus();
    }
}

#[component]
pub fn DropdownMenuItemRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] inset: Signal<bool>,
    #[prop(optional, into)] variant: Signal<DropdownMenuItemVariant>,
    /// Whether choosing this closes the menu. Read when the choice is made.
    #[prop(default = true.into(), into)]
    close_on_select: Signal<bool>,
    #[prop(optional, into)] on_click: Option<Callback<ev::MouseEvent>>,
    children: Children,
) -> impl IntoView {
    let ctx = use_dropdown();

    let menu_root = use_context::<MenuRoot>();

    let handle_click = move |e: ev::MouseEvent| {
        // The styled layer's `data-disabled:pointer-events-none` is a courtesy, not the rule: an
        // item styled by someone else still has to refuse the click rather than run it and close
        // the menu underneath the caller.
        if disabled.get_untracked() {
            return;
        }

        let target = e.current_target();
        if let Some(cb) = on_click {
            cb.run(e);
        }
        finish_select(target, close_on_select, ctx, menu_root);
    };

    view! {
        // `data-disabled` is what the styling, the roving focus and the "focus the first item"
        // query all read; `aria-disabled` is what says so out loud. The item keeps its
        // `data-roving-item` either way — it is skipped by being disabled, not by being invisible.
        <div
            data-slot="dropdown-menu-item"
            role="menuitem"
            tabindex="-1"
            data-roving-item=""
            data-disabled=move || disabled.get().then_some("true")
            aria-disabled=move || disabled.get().then_some("true")
            data-inset=move || inset.get().then_some("")
            data-variant=move || variant.get().as_str()
            on:click=handle_click
            class=class
        >
            {children()}
        </div>
    }
}

/// A row that holds an on/off choice. Takes part in the menu's keys exactly as an item does
/// (`data-roving-item`). Choosing one leaves the menu open unless `close_on_select` is set.
#[component]
pub fn DropdownMenuCheckboxItemRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] inset: Signal<bool>,
    #[prop(optional, into)] checked: RwSignal<bool>,
    /// Whether choosing this closes the menu. Read when the choice is made.
    #[prop(optional, into)]
    close_on_select: Signal<bool>,
    #[prop(optional, into)] on_checked_change: Option<Callback<bool>>,
    #[prop(optional, into)] on_click: Option<Callback<ev::MouseEvent>>,
    children: Children,
) -> impl IntoView {
    let ctx = use_dropdown();
    let menu_root = use_context::<MenuRoot>();

    let handle_click = move |e: ev::MouseEvent| {
        if disabled.get_untracked() {
            return;
        }

        let next = !checked.get_untracked();
        checked.set(next);
        if let Some(cb) = on_checked_change {
            cb.run(next);
        }
        let target = e.current_target();
        if let Some(cb) = on_click {
            cb.run(e);
        }
        finish_select(target, close_on_select, ctx, menu_root);
    };

    view! {
        <div
            data-slot="dropdown-menu-checkbox-item"
            role="menuitemcheckbox"
            tabindex="-1"
            data-roving-item=""
            aria-checked=move || checked.get().to_string()
            data-state=move || if checked.get() { "checked" } else { "unchecked" }
            data-disabled=move || disabled.get().then_some("true")
            aria-disabled=move || disabled.get().then_some("true")
            data-inset=move || inset.get().then_some("")
            on:click=handle_click
            class=class
        >
            {children()}
        </div>
    }
}

#[derive(Clone)]
struct RadioGroupContext {
    value: RwSignal<String>,
    on_value_change: Option<Callback<String>>,
}

/// A set of radio items sharing one value. Publishes the value to the items below it.
#[component]
pub fn DropdownMenuRadioGroupRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] value: RwSignal<String>,
    #[prop(optional, into)] on_value_change: Option<Callback<String>>,
    children: Children,
) -> impl IntoView {
    provide_context(RadioGroupContext {
        value,
        on_value_change,
    });

    view! {
        <div data-slot="dropdown-menu-radio-group" role="group" class=class>
            {children()}
        </div>
    }
}

/// One choice of a radio group. Selecting it sets the group's value and leaves the menu open unless `close_on_select` is set.
#[component]
pub fn DropdownMenuRadioItemRoot(
    #[prop(into)] value: String,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] inset: Signal<bool>,
    /// Whether choosing this closes the menu. Read when the choice is made.
    #[prop(optional, into)]
    close_on_select: Signal<bool>,
    #[prop(optional, into)] on_click: Option<Callback<ev::MouseEvent>>,
    children: Children,
) -> impl IntoView {
    let ctx = use_dropdown();
    let menu_root = use_context::<MenuRoot>();
    let group = expect_context::<RadioGroupContext>();

    let item_value = value.clone();
    let is_checked = move || group.value.get() == item_value;

    let handle_click = {
        let value = value.clone();
        let group = group.clone();
        move |e: ev::MouseEvent| {
            if disabled.get_untracked() {
                return;
            }

            group.value.set(value.clone());
            if let Some(cb) = group.on_value_change {
                cb.run(value.clone());
            }
            let target = e.current_target();
            if let Some(cb) = on_click {
                cb.run(e);
            }
            finish_select(target, close_on_select, ctx, menu_root);
        }
    };

    view! {
        <div
            data-slot="dropdown-menu-radio-item"
            role="menuitemradio"
            tabindex="-1"
            data-roving-item=""
            aria-checked={
                let is_checked = is_checked.clone();
                move || is_checked().to_string()
            }
            data-state=move || if is_checked() { "checked" } else { "unchecked" }
            data-disabled=move || disabled.get().then_some("true")
            aria-disabled=move || disabled.get().then_some("true")
            data-inset=move || inset.get().then_some("")
            on:click=handle_click
            class=class
        >
            {children()}
        </div>
    }
}

/// A labelled-by-nothing cluster of items; `role="group"` is all it adds.
#[component]
pub fn DropdownMenuGroupRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div data-slot="dropdown-menu-group" role="group" class=class>
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
    #[prop(optional, into)] inset: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let ctx = use_dropdown();
    let hover = expect_context::<SubHoverState>();

    view! {
        <button
            type="button"
            data-slot="dropdown-menu-sub-trigger"
            role="menuitem"
            aria-haspopup="menu"
            aria-expanded=move || ctx.is_open.get().to_string()
            tabindex="-1"
            data-roving-item=""
            disabled=disabled
            data-inset=move || inset.get().then_some("")
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
                ctx.open();
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
    #[prop(optional, into)] side_offset: Signal<SideOffset>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_dropdown();
    let floating_ref = ctx.content_ref;
    let hover = expect_context::<SubHoverState>();

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
        floating_options(Signal::derive(|| Placement::RightStart), side_offset),
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
                data-slot="dropdown-menu-sub-content"
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
                on:pointermove=move |e| focus_item_under_pointer(&e, menu_ref)
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
