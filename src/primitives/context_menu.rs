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
            InlineSubmenus, MenuRoot, focus_element, focus_item_under_pointer, focus_menu_off_item,
            handle_menu_keys, placement_align, placement_side,
        },
        floating::{
            FloatingContext, FloatingRoot, floating_options_unless_sheet, shell_class,
            side_placement,
        },
        long_press::{DEFAULT_DELAY_MS, use_long_press},
        roving_focus::use_roving_focus,
        sheet_drag::{SheetFrame, use_sheet_grab},
        typeahead::Typeahead,
    },
    utils::{
        controllable::controllable,
        sheet::{
            SHEET_ROLE_STYLE, SheetScrim, close_when_presentation_changes, presentation_attr,
            use_sheet_presentation,
        },
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
    /// Below the `md` breakpoint, present the menu as a bottom sheet instead of anchoring it to
    /// the press point: a scrim, the menu on the bottom edge, no positioning. Written as
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
    let ctx = use_context_menu();
    let point = use_context_menu_point();
    let floating_ref = ctx.content_ref;
    let sheet = use_sheet_presentation(mobile_sheet);
    let close = Callback::new(move |_| ctx.close());
    let as_sheet = sheet.get_untracked();
    provide_context(InlineSubmenus(as_sheet));
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
        update,
        ..
    } = use_floating(
        ctx.trigger_ref,
        floating_ref,
        floating_options_unless_sheet(side_placement(side, align), side_offset, sheet),
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

    close_when_presentation_changes(sheet, close);

    let ready = Signal::derive(move || sheet.get() || is_positioned.get());

    let menu_ref = AnyNodeRef::new();
    let roving = use_roving_focus(menu_ref, Orientation::Vertical);
    let typeahead = Typeahead::new();

    Effect::new(move |_| {
        if ctx.is_open.get() && ready.get() {
            focus_element(menu_ref);
            roving.sync_tab_stop();
        }
    });

    let role_element = view! {
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
