//! The application sidebar: a panel that collapses, and remembers that it is collapsed.
//!
//! There are two open states rather than one, because a sidebar is two different things depending
//! on how much room there is. On a wide viewport it is part of the layout and collapsing it is a
//! preference worth keeping between visits; on a narrow one it is an overlay that opens for one
//! errand and closes again, and remembering *that* across a reload would be an odd thing to do.
//! [`SidebarContext`] holds both, and [`SidebarContext::toggle`] flips whichever one applies.
//!
//! What is deliberately not here is the overlay itself. The primitive says how wide the sidebar
//! is, which edge it is on and whether it is open; presenting the narrow case as a sheet is the
//! styled layer's decision, and a consumer who wants a different one only needs `is_mobile` and
//! `open_mobile` to build it.

use crate::utils::types::Side;
use leptos::{context::Provider, ev, prelude::*, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;
use web_sys::HtmlElement;

/// Widths, as CSS custom properties on the wrapper so a caller can override them per sidebar
/// rather than per theme: `style="--sidebar-width: 20rem"` on the provider is enough.
pub const SIDEBAR_WIDTH: &str = "16rem";
pub const SIDEBAR_WIDTH_ICON: &str = "3rem";
pub const SIDEBAR_WIDTH_MOBILE: &str = "18rem";

/// Below this the sidebar is an overlay rather than part of the layout. It matches Tailwind's `md`
/// breakpoint, which is what the styled layer's `md:` classes key off.
const MOBILE_QUERY: &str = "(max-width: 767px)";

/// The desktop open state, and only that — see the module note.
const STORAGE_KEY: &str = "ui-sidebar";

/// Ctrl/Cmd + this toggles the sidebar from anywhere on the page.
const SHORTCUT_KEY: &str = "b";

/// What is left behind when the sidebar collapses.
#[derive(Default, Clone, Copy, PartialEq)]
pub enum SidebarCollapsible {
    /// Nothing: it slides off the edge entirely.
    #[default]
    OffCanvas,
    /// A rail of icons, wide enough for the menu buttons' icons and nothing else.
    Icon,
    /// It does not collapse. The trigger and the rail have nothing to do, and the state stays
    /// `expanded` for good.
    None,
}

impl SidebarCollapsible {
    pub fn as_str(&self) -> &'static str {
        match self {
            SidebarCollapsible::OffCanvas => "offcanvas",
            SidebarCollapsible::Icon => "icon",
            SidebarCollapsible::None => "none",
        }
    }
}

#[derive(Copy, Clone)]
pub struct SidebarContext {
    /// The wide-viewport state, persisted.
    pub open: RwSignal<bool>,
    /// The narrow-viewport state, which is a fresh decision every time.
    pub open_mobile: RwSignal<bool>,
    pub is_mobile: RwSignal<bool>,
    pub side: Side,
    pub collapsible: SidebarCollapsible,
    /// The panel, so the trigger can point `aria-controls` at it. Minted by the provider because
    /// the trigger is usually rendered before the panel it controls.
    pub content_ref: AnyNodeRef,
    pub content_id: RwSignal<String>,
}

impl SidebarContext {
    /// Whether the sidebar is showing, on whichever viewport this is.
    pub fn is_open(&self) -> bool {
        if self.is_mobile.get() {
            self.open_mobile.get()
        } else {
            self.open.get()
        }
    }

    /// `expanded` or `collapsed`, for `data-state`.
    ///
    /// A sidebar that cannot collapse is always expanded, whatever the stored preference says —
    /// otherwise turning `collapsible` off would leave a panel stuck shut with no way to open it.
    pub fn state(&self) -> &'static str {
        if self.collapsible == SidebarCollapsible::None || self.is_open() {
            "expanded"
        } else {
            "collapsed"
        }
    }

    pub fn set_open(&self, open: bool) {
        if self.is_mobile.get_untracked() {
            self.open_mobile.set(open);
        } else {
            self.open.set(open);
        }
    }

    pub fn toggle(&self) {
        let target = if self.is_mobile.get_untracked() {
            self.open_mobile
        } else {
            self.open
        };
        target.update(|open| *open = !*open);
    }
}

pub fn use_sidebar() -> SidebarContext {
    expect_context::<SidebarContext>()
}

fn stored_open() -> Option<bool> {
    let stored = window()
        .local_storage()
        .ok()
        .flatten()?
        .get_item(STORAGE_KEY)
        .ok()
        .flatten()?;

    Some(stored == "true")
}

fn matches_mobile() -> bool {
    window()
        .match_media(MOBILE_QUERY)
        .ok()
        .flatten()
        .map(|query| query.matches())
        .unwrap_or(false)
}

/// Provides the sidebar state to everything under it.
///
/// `open` may be passed in to drive the sidebar from outside; left alone, the provider owns the
/// signal and restores its last value from local storage.
#[component]
pub fn SidebarProviderRoot(
    /// Which edge the panel is pinned to. `Side::Bottom` and `Side::Top` are meaningless for a
    /// sidebar and are treated as left.
    #[prop(default = Side::Left)]
    side: Side,
    #[prop(optional)] collapsible: SidebarCollapsible,
    #[prop(default = None, into)] open: Option<RwSignal<bool>>,
    /// Where an uncontrolled sidebar starts, before local storage has anything to say.
    #[prop(default = true)]
    default_open: bool,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] style: Signal<String>,
    children: Children,
) -> impl IntoView {
    let controlled = open.is_some();
    let open = open.unwrap_or_else(|| RwSignal::new(stored_open().unwrap_or(default_open)));

    let context = SidebarContext {
        open,
        open_mobile: RwSignal::new(false),
        is_mobile: RwSignal::new(matches_mobile()),
        side,
        collapsible,
        content_ref: AnyNodeRef::new(),
        content_id: RwSignal::new(crate::utils::next_id("sidebar")),
    };

    // A controlled sidebar belongs to its caller; writing its state into storage behind their back
    // would resurrect it on the next load and contradict whatever they initialise it to.
    if !controlled {
        Effect::new(move |_| {
            let open = context.open.get();
            if let Ok(Some(storage)) = window().local_storage() {
                let _ = storage.set_item(STORAGE_KEY, if open { "true" } else { "false" });
            }
        });
    }

    // `resize` rather than a `matchMedia` listener: leptos cleans a window listener up with the
    // component, and the query is re-read on each event, so the two agree by construction.
    let resize = window_event_listener(ev::resize, move |_| {
        let is_mobile = matches_mobile();
        if context.is_mobile.get_untracked() != is_mobile {
            context.is_mobile.set(is_mobile);
            // Crossing the breakpoint with the overlay open would otherwise leave it open behind
            // the layout sidebar, and reopening the overlay later would flash the stale panel.
            context.open_mobile.set(false);
        }
    });

    let shortcut = window_event_listener(ev::keydown, move |e| {
        if e.key() == SHORTCUT_KEY && (e.meta_key() || e.ctrl_key()) {
            e.prevent_default();
            context.toggle();
        }
    });

    on_cleanup(move || {
        resize.remove();
        shortcut.remove();
    });

    view! {
        <Provider value=context>
            <div
                data-slot="sidebar-wrapper"
                data-state=move || context.state()
                style=move || {
                    format!(
                        "--sidebar-width: {SIDEBAR_WIDTH}; --sidebar-width-icon: {SIDEBAR_WIDTH_ICON}; --sidebar-width-mobile: {SIDEBAR_WIDTH_MOBILE}; {}",
                        style.get(),
                    )
                }
                class=class
            >
                {children()}
            </div>
        </Provider>
    }
}

/// The panel itself: the state as attributes, and nothing about how it looks.
///
/// `data-state`, `data-side`, `data-collapsible` and `data-variant` are what the styled layer —
/// and anyone else — styles against, so a collapsed sidebar is a selector rather than a branch.
#[component]
pub fn SidebarRoot(
    /// Named by the styled layer (`sidebar`, `floating`, `inset`) and rendered verbatim, since a
    /// primitive has no opinion about how many variants there are.
    #[prop(optional, into)]
    variant: Signal<String>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_sidebar();

    view! {
        <div
            node_ref=ctx.content_ref
            id=move || ctx.content_id.get()
            data-slot="sidebar"
            data-state=move || ctx.state()
            data-side=ctx.side.as_str()
            data-collapsible=move || {
                (ctx.state() == "collapsed").then(|| ctx.collapsible.as_str())
            }
            data-variant=move || variant.get()
            data-mobile=move || ctx.is_mobile.get().then_some("true")
            class=class
        >
            {children()}
        </div>
    }
}

/// The button that opens and closes it.
#[component]
pub fn SidebarTriggerRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let ctx = use_sidebar();

    view! {
        <button
            type="button"
            data-slot="sidebar-trigger"
            data-state=move || ctx.state()
            aria-controls=move || ctx.content_id.get()
            aria-expanded=move || ctx.is_open().to_string()
            disabled=disabled
            on:click=move |_| ctx.toggle()
            class=class
        >
            {children()}
        </button>
    }
}

/// The strip along the sidebar's inner edge that toggles it.
///
/// A second trigger, not a resize handle: it is hidden from the tab order because the real trigger
/// is already in it, and reached by pointer, which is the only way anyone finds a rail.
#[component]
pub fn SidebarRailRoot(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    let ctx = use_sidebar();

    view! {
        <button
            type="button"
            data-slot="sidebar-rail"
            data-state=move || ctx.state()
            data-side=ctx.side.as_str()
            aria-label="Toggle sidebar"
            aria-controls=move || ctx.content_id.get()
            tabindex="-1"
            on:click=move |_| ctx.toggle()
            class=class
        />
    }
}

/// A row in a sidebar menu.
///
/// Renders a button unless `render` is given, which is the common case in a real app: these are
/// usually links, and a link has to stay a link for middle-click and copy-address to work.
#[component]
pub fn SidebarMenuButtonRoot(
    /// The current page's row. Announced with `aria-current`, not just painted.
    #[prop(optional, into)]
    active: Signal<bool>,
    /// Rendered into `data-size`, for the styled layer to key off.
    #[prop(optional, into)]
    size: Signal<String>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    /// Render the row as something else — an `<a>`, almost always. The callback is handed the
    /// class this would have rendered and the node ref the state attributes are written onto,
    /// which is the same bargain `Button` offers.
    #[prop(default = None, into)]
    render: Option<Callback<(Signal<String>, AnyNodeRef), AnyView>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let node_ref = AnyNodeRef::new();

    // The trigger is the caller's element when `render` is given, so the attributes are written
    // onto the node rather than declared here — the same trick `FloatingRoot` uses.
    if render.is_some() {
        Effect::new(move |_| {
            let Some(el) = node_ref.get() else {
                return;
            };
            let _ = el.set_attribute("data-slot", "sidebar-menu-button");
            let _ = el.set_attribute("data-size", &size.get());
            if active.get() {
                let _ = el.set_attribute("data-active", "true");
                let _ = el.set_attribute("aria-current", "page");
            } else {
                let _ = el.remove_attribute("data-active");
                let _ = el.remove_attribute("aria-current");
            }
        });
    }

    match render {
        Some(render_fn) => leptos::either::Either::Left(render_fn.run((class, node_ref))),
        None => leptos::either::Either::Right(view! {
            <button
                type="button"
                node_ref=node_ref
                data-slot="sidebar-menu-button"
                data-size=move || size.get()
                data-active=move || active.get().then_some("true")
                aria-current=move || active.get().then_some("page")
                disabled=disabled
                class=class
            >
                {children.map(|children| children())}
            </button>
        }),
    }
}

/// Moves focus to the sidebar's first menu button. For a skip link, or a shortcut into the nav.
pub fn focus_first_menu_button(ctx: SidebarContext) {
    if let Some(panel) = ctx.content_ref.get_untracked()
        && let Ok(Some(first)) = panel.query_selector("[data-slot=sidebar-menu-button]")
        && let Ok(first) = first.dyn_into::<HtmlElement>()
    {
        let _ = first.focus();
    }
}
