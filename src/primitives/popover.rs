use crate::{
    primitives::floating::{
        FloatingContext, FloatingPortal, FloatingRoot, FloatingTrigger, TriggerAria, TriggerRender,
        floating_options, shell_class, side_placement,
    },
    utils::{
        controllable::forward,
        sheet::{SheetHeader, SheetScrim, presentation_attr, use_sheet_presentation},
        types::{Align, Side, SideOffset},
    },
};
use floating_ui_leptos::{Placement, UseFloatingReturn, use_floating};
use leptos::{either::Either, prelude::*, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;
use web_sys::{FocusOptions, HtmlElement};

const FOCUSABLE: &str = "a[href], button:not([disabled]), input:not([disabled]), textarea:not([disabled]), select:not([disabled]), [tabindex]:not([tabindex='-1'])";

pub type PopoverContext = FloatingContext;

pub fn use_popover() -> PopoverContext {
    expect_context::<PopoverContext>()
}

#[component]
pub fn PopoverRoot(
    #[prop(optional, into)] class: Signal<String>,
    /// Whether the popover is open, held by the caller. Omit it and the root keeps its own.
    #[prop(default = None, into)]
    open: Option<RwSignal<bool>>,
    /// Whether the popover starts open. Ignored when `open` is given.
    #[prop(optional)]
    default_open: bool,
    /// Called with the new value whenever the popover opens or closes, however that came about.
    #[prop(optional, into)]
    on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <FloatingRoot
            class=class
            trigger_aria=TriggerAria::Popup("dialog")
            open=open
            default_open=default_open
            on_open_change=forward(on_open_change)
        >
            {children()}
        </FloatingRoot>
    }
}

#[component]
pub fn PopoverTriggerRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(default = None, into)] as_child: Option<Callback<TriggerRender, AnyView>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_popover();
    view! {
        <FloatingTrigger
            context=ctx
            class=class
            disabled=disabled
            data_slot="popover-trigger"
            render=as_child
        >
            {match children {
                Some(child) => Either::Left(child()),
                None => Either::Right(""),
            }}
        </FloatingTrigger>
    }
}

#[component]
pub fn PopoverPortalRoot(
    /// Mount the popover in this element instead of `<body>` — a sticky or fixed ancestor it
    /// should move with, so scrolling does not leave it behind until the next position update.
    /// The fullscreen element still wins when the container is not inside it.
    #[prop(default = None)]
    container: Option<AnyNodeRef>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_popover();
    let stored_children = StoredValue::new(children);

    view! {
        <FloatingPortal container=container>
            <Show when=move || ctx.is_mounted.get()>{stored_children.with_value(|c| c())}</Show>
        </FloatingPortal>
    }
}

#[component]
pub fn PopoverContentRoot(
    #[prop(optional, into)] side: Signal<Side>,
    #[prop(optional, into)] align: Signal<Align>,
    #[prop(optional, into)] side_offset: Signal<SideOffset>,
    #[prop(optional, into)] class: Signal<String>,
    /// Below the `md` breakpoint, present the popover as a bottom sheet: a scrim, the panel on
    /// the bottom edge, no positioning. Written as `data-presentation="sheet"` (`"floating"`
    /// otherwise) for the styled layer to key on. Off by default.
    #[prop(optional, into)]
    mobile_sheet: Signal<bool>,
    /// The title row shown at the top of the sheet. Empty means a grabber and no row.
    #[prop(optional, into)]
    sheet_title: Signal<String>,
    /// Move focus into the popover when it opens: onto the first focusable element inside, or the
    /// panel itself when there is none (it carries `tabindex="-1"` for that). Closing hands focus
    /// back to the trigger when the keyboard closed it. Turn it off for a popover that should
    /// leave focus where it is, such as one opened by hover. Happens however the popover came to
    /// open, a controlled `open` set from code included.
    #[prop(default = true.into(), into)]
    auto_focus: Signal<bool>,
    /// A CSS selector for what `auto_focus` lands on, tried before the first focusable element —
    /// a date picker names the calendar's current day. Empty is the default behaviour.
    #[prop(optional)]
    initial_focus: &'static str,
    children: Children,
) -> impl IntoView {
    let ctx = use_popover();
    let floating_ref = ctx.content_ref;
    let panel_ref = AnyNodeRef::new();
    let sheet = use_sheet_presentation(mobile_sheet);

    let UseFloatingReturn {
        floating_styles,
        is_positioned,
        placement,
        ..
    } = use_floating(
        ctx.trigger_ref,
        floating_ref,
        floating_options(side_placement(side, align), side_offset),
    );

    let ready = Signal::derive(move || sheet.get() || is_positioned.get());

    Effect::new(move |was_shown: Option<bool>| {
        let shown = ctx.is_open.get() && ready.get();

        if shown && was_shown != Some(true) && auto_focus.get_untracked() {
            request_animation_frame(move || {
                if ctx.is_open.get_untracked() {
                    focus_into(panel_ref, initial_focus);
                }
            });
        }

        shown
    });

    view! {
        <SheetScrim sheet=sheet open=ctx.is_open.into() />
        // The positioned shell, hidden until floating-ui has measured it.
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
            <div
                node_ref=panel_ref
                tabindex="-1"
                id=move || ctx.content_id.get()
                role="dialog"
                aria-labelledby=move || ctx.trigger_id.get()
                data-state=move || {
                    if ctx.is_open.get() && ready.get() { "open" } else { "closed" }
                }
                data-slot="popover-content"
                data-presentation=move || presentation_attr(sheet.get())
                data-align=move || (!sheet.get()).then(|| match placement.get() {
                    Placement::TopStart
                    | Placement::BottomStart
                    | Placement::LeftStart
                    | Placement::RightStart => "start",
                    Placement::TopEnd
                    | Placement::BottomEnd
                    | Placement::LeftEnd
                    | Placement::RightEnd => "end",
                    _ => "center",
                })
                data-side=move || (!sheet.get()).then(|| match placement.get() {
                    Placement::Top | Placement::TopStart | Placement::TopEnd => "top",
                    Placement::Bottom | Placement::BottomStart | Placement::BottomEnd => "bottom",
                    Placement::Left | Placement::LeftStart | Placement::LeftEnd => "left",
                    Placement::Right | Placement::RightStart | Placement::RightEnd => "right",
                })
                class=class
            >
                <SheetHeader sheet=sheet title=sheet_title />
                {children()}
            </div>
        </div>
    }
}

/// `preferred` if it matches, else the first focusable element inside `panel`, else `panel`
/// itself. Left alone when focus has already moved inside — an `autofocus` the caller wrote wins.
/// The panel may be gone by the time a frame fires, which is not an error.
fn focus_into(panel: AnyNodeRef, preferred: &str) {
    let Some(panel) = panel.try_get_untracked().flatten() else {
        return;
    };

    let active = document().active_element();
    if active.as_ref() != Some(&panel) && panel.contains(active.as_deref()) {
        return;
    }

    let first = |selector: &str| {
        panel.query_selector_all(selector).ok().and_then(|found| {
            (0..found.length())
                .filter_map(|index| found.item(index))
                .filter_map(|node| node.dyn_into::<HtmlElement>().ok())
                .find(|el| el.get_attribute("aria-hidden").as_deref() != Some("true"))
        })
    };

    let target = (!preferred.is_empty())
        .then(|| first(preferred))
        .flatten()
        .or_else(|| first(FOCUSABLE))
        .or_else(|| panel.clone().dyn_into::<HtmlElement>().ok());

    if let Some(target) = target {
        let options = FocusOptions::new();
        options.set_prevent_scroll(true);
        let _ = target.focus_with_options(&options);
    }
}
