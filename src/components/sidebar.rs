//! The styled sidebar.
//!
//! Two presentations of the same state, picked by viewport. Wide: a fixed panel that the layout
//! makes room for, which is why `Sidebar` renders *two* boxes — one in the flow whose only job is
//! to reserve the width, and the fixed one the reader actually sees. Animating a fixed panel and
//! a flow gap separately is what makes the content beside it slide instead of jump. Narrow: a
//! `Sheet`, because there is no room to reserve.
//!
//! Everything below the panel is style-only. The state arrives as `data-*` on the container, so a
//! collapsed sidebar is `group-data-[collapsible=icon]:…` on each part rather than a branch here.

use crate::{
    cn,
    components::{
        input::Input,
        separator::Separator,
        sheet::{Sheet, SheetContent, SheetDescription, SheetTitle},
        skeleton::Skeleton,
        tooltip::{Tooltip, TooltipContent, TooltipTrigger},
    },
    icons::panel::PanelLeft,
    primitives::sidebar::{
        SidebarCollapsible, SidebarMenuButtonRoot, SidebarProviderRoot, SidebarRailRoot,
        SidebarRoot, SidebarTriggerRoot, use_sidebar,
    },
    utils::types::{AsClass, Side},
};
use leptos::{either::Either, prelude::*};
use leptos_node_ref::AnyNodeRef;

/// How the panel sits in the page.
#[derive(Default, Clone, Copy, PartialEq)]
pub enum SidebarVariant {
    /// Flush against the edge, with a border where it meets the content.
    #[default]
    Sidebar,
    /// Inset from the edges, rounded, with a shadow — a panel resting on the page.
    Floating,
    /// The panel keeps the page's background and the *content* becomes the floating card.
    Inset,
}

impl SidebarVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            SidebarVariant::Sidebar => "sidebar",
            SidebarVariant::Floating => "floating",
            SidebarVariant::Inset => "inset",
        }
    }

    /// Whether the panel is inset from the window, which changes both boxes' widths: the gap has
    /// to account for the padding around a collapsed icon rail.
    fn is_inset(&self) -> bool {
        matches!(self, SidebarVariant::Floating | SidebarVariant::Inset)
    }
}

/// Size of a menu row, rendered into `data-size` so badges and actions can line up with it.
#[derive(Default, Clone, Copy, PartialEq)]
pub enum SidebarMenuSize {
    #[default]
    Default,
    Sm,
    Lg,
}

impl SidebarMenuSize {
    fn as_str(&self) -> &'static str {
        match self {
            SidebarMenuSize::Default => "default",
            SidebarMenuSize::Sm => "sm",
            SidebarMenuSize::Lg => "lg",
        }
    }
}

impl AsClass for SidebarMenuSize {
    fn as_class(&self) -> &'static str {
        match self {
            SidebarMenuSize::Default => "h-8 text-sm",
            SidebarMenuSize::Sm => "h-7 text-xs",
            SidebarMenuSize::Lg => "h-12 text-sm group-data-[collapsible=icon]:!p-0",
        }
    }
}

#[component]
pub fn SidebarProvider(
    #[prop(default = Side::Left)] side: Side,
    #[prop(optional)] collapsible: SidebarCollapsible,
    #[prop(default = None, into)] open: Option<RwSignal<bool>>,
    #[prop(default = true)] default_open: bool,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] style: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <SidebarProviderRoot
            side=side
            collapsible=collapsible
            open=open
            default_open=default_open
            style=style
            class=move || {
                cn!(
                    "group/sidebar-wrapper flex min-h-svh w-full has-data-[variant=inset]:bg-sidebar",
                    class.get(),
                )
            }
        >
            {children()}
        </SidebarProviderRoot>
    }
}

#[component]
pub fn Sidebar(
    #[prop(optional)] variant: SidebarVariant,
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_sidebar();
    let stored_children = StoredValue::new(children);

    // The panel is the same tree either way; only the box around it changes.
    let panel = move || {
        view! {
            <div
                data-slot="sidebar-inner"
                class="flex h-full w-full flex-col bg-sidebar group-data-[variant=floating]:rounded-lg group-data-[variant=floating]:border group-data-[variant=floating]:border-sidebar-border group-data-[variant=floating]:shadow-sm"
            >
                {stored_children.with_value(|children| children())}
            </div>
        }
    };

    let gap_class = if variant.is_inset() {
        "group-data-[collapsible=icon]:w-[calc(var(--sidebar-width-icon)+1rem)]"
    } else {
        "group-data-[collapsible=icon]:w-[var(--sidebar-width-icon)]"
    };

    let fixed_class = if variant.is_inset() {
        "p-2 group-data-[collapsible=icon]:w-[calc(var(--sidebar-width-icon)+1rem+2px)]"
    } else {
        "group-data-[collapsible=icon]:w-[var(--sidebar-width-icon)] group-data-[side=left]:border-r group-data-[side=right]:border-l"
    };

    let edge_class = match ctx.side {
        Side::Right => {
            "right-0 group-data-[collapsible=offcanvas]:right-[calc(var(--sidebar-width)*-1)]"
        }
        _ => "left-0 group-data-[collapsible=offcanvas]:left-[calc(var(--sidebar-width)*-1)]",
    };

    move || {
        if ctx.is_mobile.get() {
            // On a phone the sidebar is an errand, so it arrives as a sheet and takes the layer
            // stack, the focus trap and the dismissal with it. The sheet's own close button is
            // hidden: the sidebar has a trigger of its own and two would be one too many.
            Either::Left(view! {
                <Sheet open=ctx.open_mobile>
                    <SheetContent
                        side=ctx.side
                        // The sheet is portalled to `<body>`, above the wrapper that defines the
                        // widths, so `--sidebar-width` does not reach it — hence the mobile width
                        // and its own fallback. Overriding it means setting
                        // `--sidebar-width-mobile` somewhere the portal inherits from, `:root`.
                        class="w-[var(--sidebar-width-mobile,18rem)] bg-sidebar p-0 text-sidebar-foreground [&>button]:hidden"
                    >
                        <SheetTitle class="sr-only">"Sidebar"</SheetTitle>
                        <SheetDescription class="sr-only">
                            "Navigation for this application."
                        </SheetDescription>
                        <div class="flex h-full w-full flex-col">{panel()}</div>
                    </SheetContent>
                </Sheet>
            })
        } else {
            Either::Right(view! {
                <SidebarRoot
                    variant=variant.as_str()
                    class=move || {
                        cn!(
                            "group peer hidden text-sidebar-foreground md:block",
                            // The panel pins itself to its edge, but the box that reserves its
                            // width is in the flow and would otherwise sit where the markup put
                            // it — a right-hand sidebar with a left-hand gap squeezes the content
                            // from both sides. Ordering it last makes the markup order stop
                            // mattering.
                            "data-[side=right]:order-last",
                            class.get(),
                        )
                    }
                >
                    // Reserves the width in the flow. Nothing is drawn here — it is the
                    // reason the page beside the sidebar slides rather than jumps.
                    <div class=cn!(
                        "relative w-[var(--sidebar-width)] bg-transparent transition-[width] duration-200 ease-linear",
                        "group-data-[collapsible=offcanvas]:w-0 group-data-[side=right]:rotate-180",
                        gap_class,
                    ) />
                    <div class=cn!(
                        // `inset-y-0` rather than a viewport height: a fixed box is measured
                        // against the nearest transformed ancestor when there is one, so a demo
                        // can hold the whole sidebar without it escaping to the window.
                        "fixed inset-y-0 z-10 hidden w-[var(--sidebar-width)] transition-[left,right,width] duration-200 ease-linear md:flex",
                        edge_class,
                        fixed_class,
                    )>{panel()}</div>
                </SidebarRoot>
            })
        }
    }
}

/// The page beside the sidebar. Takes the room the sidebar is not using.
#[component]
pub fn SidebarInset(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <main
            data-slot="sidebar-inset"
            class=move || {
                cn!(
                    "relative flex w-full flex-1 flex-col bg-background",
                    // The inset variant floats the content instead of the panel, and pulls its
                    // margin back in when the sidebar collapses.
                    "md:peer-data-[variant=inset]:m-2 md:peer-data-[variant=inset]:ml-0 md:peer-data-[variant=inset]:rounded-xl md:peer-data-[variant=inset]:shadow-sm",
                    "md:peer-data-[variant=inset]:peer-data-[state=collapsed]:ml-2",
                    class.get(),
                )
            }
        >
            {children()}
        </main>
    }
}

#[component]
pub fn SidebarTrigger(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <SidebarTriggerRoot
            disabled=disabled
            class=move || {
                cn!(
                    "inline-flex size-7 shrink-0 items-center justify-center rounded-md text-sidebar-foreground transition-colors hover:bg-sidebar-accent hover:text-sidebar-accent-foreground focus-visible:ring-2 focus-visible:ring-sidebar-ring focus-visible:outline-none disabled:pointer-events-none disabled:opacity-50",
                    class.get(),
                )
            }
        >
            {match children {
                Some(children) => Either::Left(children()),
                None => {
                    Either::Right(
                        view! {
                            <PanelLeft />
                            <span class="sr-only">"Toggle sidebar"</span>
                        },
                    )
                }
            }}
        </SidebarTriggerRoot>
    }
}

/// The strip along the panel's inner edge, which toggles it.
#[component]
pub fn SidebarRail(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <SidebarRailRoot class=move || {
            cn!(
                "absolute inset-y-0 z-20 hidden w-4 -translate-x-1/2 transition-all ease-linear after:absolute after:inset-y-0 after:left-1/2 after:w-[2px] hover:after:bg-sidebar-border sm:flex",
                "group-data-[side=left]:-right-4 group-data-[side=right]:left-0",
                "group-data-[side=left]:cursor-w-resize group-data-[side=right]:cursor-e-resize",
                "group-data-[collapsible=offcanvas]:translate-x-0 group-data-[collapsible=offcanvas]:after:left-full group-data-[collapsible=offcanvas]:hover:bg-sidebar",
                "group-data-[collapsible=none]:hidden",
                class.get(),
            )
        } />
    }
}

#[component]
pub fn SidebarHeader(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div data-slot="sidebar-header" class=move || cn!("flex flex-col gap-2 p-2", class.get())>
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarFooter(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div data-slot="sidebar-footer" class=move || cn!("flex flex-col gap-2 p-2", class.get())>
            {children()}
        </div>
    }
}

/// Everything between the header and the footer, and the part that scrolls.
#[component]
pub fn SidebarContent(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="sidebar-content"
            class=move || {
                cn!(
                    "flex min-h-0 flex-1 flex-col gap-2 overflow-auto group-data-[collapsible=icon]:overflow-hidden",
                    class.get(),
                )
            }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarSeparator(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <Separator class=move || cn!("mx-2 w-auto bg-sidebar-border", class.get()) />
    }
}

/// A search box in the sidebar's header, sized for it.
#[component]
pub fn SidebarInput(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(default = None, into)] model: Option<RwSignal<String>>,
    #[prop(optional, into)] placeholder: Signal<String>,
) -> impl IntoView {
    view! {
        <Input
            model=model
            attr:placeholder=move || placeholder.get()
            class=move || cn!("h-8 w-full bg-background shadow-none", class.get())
        />
    }
}

#[component]
pub fn SidebarGroup(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="sidebar-group"
            class=move || cn!("relative flex w-full min-w-0 flex-col p-2", class.get())
        >
            {children()}
        </div>
    }
}

/// The heading over a group, which folds away when the sidebar collapses to icons.
#[component]
pub fn SidebarGroupLabel(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="sidebar-group-label"
            class=move || {
                cn!(
                    "flex h-8 shrink-0 items-center rounded-md px-2 text-xs font-medium text-sidebar-foreground/70 outline-none ring-sidebar-ring transition-[margin,opacity] duration-200 ease-linear focus-visible:ring-2 [&>svg]:size-4 [&>svg]:shrink-0",
                    // Pulled up under the group above rather than hidden, so the icons keep their
                    // rhythm instead of shifting when the label goes.
                    "group-data-[collapsible=icon]:-mt-8 group-data-[collapsible=icon]:opacity-0",
                    class.get(),
                )
            }
        >
            {children()}
        </div>
    }
}

/// A button in a group's top-right corner — "add", "more", and the like.
#[component]
pub fn SidebarGroupAction(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-slot="sidebar-group-action"
            class=move || {
                cn!(
                    "absolute top-3.5 right-3 flex aspect-square w-5 items-center justify-center rounded-md p-0 text-sidebar-foreground outline-none ring-sidebar-ring transition-transform hover:bg-sidebar-accent hover:text-sidebar-accent-foreground focus-visible:ring-2 [&>svg]:size-4 [&>svg]:shrink-0",
                    // A 20px target is fine for a mouse and too small for a thumb, so the hit area
                    // is widened on touch and left alone from `md` up.
                    "after:absolute after:-inset-2 md:after:hidden",
                    "group-data-[collapsible=icon]:hidden",
                    class.get(),
                )
            }
        >
            {children()}
        </button>
    }
}

#[component]
pub fn SidebarGroupContent(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div data-slot="sidebar-group-content" class=move || cn!("w-full text-sm", class.get())>
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarMenu(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <ul
            data-slot="sidebar-menu"
            class=move || cn!("flex w-full min-w-0 flex-col gap-1", class.get())
        >
            {children()}
        </ul>
    }
}

#[component]
pub fn SidebarMenuItem(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <li
            data-slot="sidebar-menu-item"
            class=move || cn!("group/menu-item relative", class.get())
        >
            {children()}
        </li>
    }
}

/// A row in the menu: the icon, the label, and the current-page state.
///
/// `tooltip` is what the row says when the sidebar is collapsed to icons and the label is gone.
/// It is only rendered in that state — a tooltip repeating a label the reader can already see is
/// noise.
#[component]
pub fn SidebarMenuButton(
    #[prop(optional, into)] active: Signal<bool>,
    #[prop(optional)] size: SidebarMenuSize,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(default = None, into)] tooltip: Option<Signal<String>>,
    /// Render the row as something else — usually an `<a>`. The callback is handed the row's
    /// class and the node ref its state attributes are written onto.
    #[prop(default = None, into)]
    render: Option<Callback<(Signal<String>, AnyNodeRef), AnyView>>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let ctx = use_sidebar();
    let size_class = size.as_class();
    let stored_children = StoredValue::new(children);

    let button = move || {
        let children = stored_children.get_value();
        view! {
            <SidebarMenuButtonRoot
                active=active
                size=size.as_str()
                disabled=disabled
                render=render
                class=move || {
                    cn!(
                        "peer/menu-button flex w-full items-center gap-2 overflow-hidden rounded-md p-2 text-left text-sm outline-none ring-sidebar-ring transition-[width,height,padding] hover:bg-sidebar-accent hover:text-sidebar-accent-foreground focus-visible:ring-2 active:bg-sidebar-accent active:text-sidebar-accent-foreground disabled:pointer-events-none disabled:opacity-50 aria-disabled:pointer-events-none aria-disabled:opacity-50",
                        "data-[active=true]:bg-sidebar-accent data-[active=true]:font-medium data-[active=true]:text-sidebar-accent-foreground",
                        "data-[state=open]:hover:bg-sidebar-accent data-[state=open]:hover:text-sidebar-accent-foreground",
                        // A row with an action beside it leaves room for it.
                        "group-has-data-[slot=sidebar-menu-action]/menu-item:pr-8",
                        // Collapsed, the row is exactly one icon.
                        "group-data-[collapsible=icon]:!size-8 group-data-[collapsible=icon]:!p-2",
                        "[&>span:last-child]:truncate [&>svg]:size-4 [&>svg]:shrink-0",
                        size_class,
                        class.get(),
                    )
                }
            >
                {children.map(|children| children())}
            </SidebarMenuButtonRoot>
        }
    };

    move || match tooltip {
        // Only while the label is missing: expanded, or on a phone where the panel is full width,
        // the row already says what it is.
        Some(tooltip) if ctx.state() == "collapsed" && !ctx.is_mobile.get() => {
            Either::Left(view! {
                <Tooltip>
                    <TooltipTrigger>{button()}</TooltipTrigger>
                    <TooltipContent side=Side::Right>{move || tooltip.get()}</TooltipContent>
                </Tooltip>
            })
        }
        _ => Either::Right(button()),
    }
}

/// A second control on a menu row — a "more" menu, a dismiss.
#[component]
pub fn SidebarMenuAction(
    #[prop(optional, into)] class: Signal<String>,
    /// Keep it hidden until the row is hovered or focused. For actions that would otherwise be a
    /// row of buttons shouting at the reader.
    #[prop(default = false)]
    show_on_hover: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-slot="sidebar-menu-action"
            class=move || {
                cn!(
                    "absolute top-1.5 right-1 flex aspect-square w-5 items-center justify-center rounded-md p-0 text-sidebar-foreground outline-none ring-sidebar-ring transition-transform hover:bg-sidebar-accent hover:text-sidebar-accent-foreground focus-visible:ring-2 peer-hover/menu-button:text-sidebar-accent-foreground [&>svg]:size-4 [&>svg]:shrink-0",
                    "after:absolute after:-inset-2 md:after:hidden",
                    // Lines up with whichever row height the button asked for.
                    "peer-data-[size=sm]/menu-button:top-1 peer-data-[size=default]/menu-button:top-1.5 peer-data-[size=lg]/menu-button:top-2.5",
                    "group-data-[collapsible=icon]:hidden",
                    if show_on_hover {
                        "opacity-0 focus-within:opacity-100 group-focus-within/menu-item:opacity-100 group-hover/menu-item:opacity-100 peer-data-[active=true]/menu-button:opacity-100 md:opacity-0"
                    } else {
                        ""
                    },
                    class.get(),
                )
            }
        >
            {children()}
        </button>
    }
}

/// A count on a menu row. Not interactive, so it never takes the row's click.
#[component]
pub fn SidebarMenuBadge(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="sidebar-menu-badge"
            class=move || {
                cn!(
                    "pointer-events-none absolute right-1 flex h-5 min-w-5 items-center justify-center rounded-md px-1 text-xs font-medium tabular-nums text-sidebar-foreground select-none",
                    "peer-hover/menu-button:text-sidebar-accent-foreground peer-data-[active=true]/menu-button:text-sidebar-accent-foreground",
                    "peer-data-[size=sm]/menu-button:top-1 peer-data-[size=default]/menu-button:top-1.5 peer-data-[size=lg]/menu-button:top-2.5",
                    "group-data-[collapsible=icon]:hidden",
                    class.get(),
                )
            }
        >
            {children()}
        </div>
    }
}

/// A placeholder row, for a menu that is still loading.
#[component]
pub fn SidebarMenuSkeleton(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(default = true)] show_icon: bool,
    /// Width of the text bar, as a percentage. Varying it across a list is what stops a loading
    /// menu looking like a table.
    #[prop(default = 70)]
    width: u32,
) -> impl IntoView {
    view! {
        <div
            data-slot="sidebar-menu-skeleton"
            class=move || cn!("flex h-8 items-center gap-2 rounded-md px-2", class.get())
        >
            {show_icon.then(|| view! { <Skeleton class="size-4 rounded-md" /> })}
            <Skeleton
                class="h-4 max-w-[--skeleton-width] flex-1"
                attr:style=format!("--skeleton-width: {width}%")
            />
        </div>
    }
}

/// The nested list under an expanded row.
#[component]
pub fn SidebarMenuSub(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <ul
            data-slot="sidebar-menu-sub"
            class=move || {
                cn!(
                    "mx-3.5 flex min-w-0 translate-x-px flex-col gap-1 border-l border-sidebar-border px-2.5 py-0.5",
                    "group-data-[collapsible=icon]:hidden",
                    class.get(),
                )
            }
        >
            {children()}
        </ul>
    }
}

#[component]
pub fn SidebarMenuSubItem(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <li
            data-slot="sidebar-menu-sub-item"
            class=move || cn!("group/menu-sub-item relative", class.get())
        >
            {children()}
        </li>
    }
}

#[component]
pub fn SidebarMenuSubButton(
    #[prop(optional, into)] active: Signal<bool>,
    #[prop(default = SidebarMenuSize::Default)] size: SidebarMenuSize,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(default = None, into)] render: Option<Callback<(Signal<String>, AnyNodeRef), AnyView>>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let size_class = match size {
        SidebarMenuSize::Sm => "text-xs",
        _ => "text-sm",
    };

    view! {
        <SidebarMenuButtonRoot
            active=active
            size=size.as_str()
            disabled=disabled
            render=render
            class=move || {
                cn!(
                    "flex h-7 min-w-0 -translate-x-px items-center gap-2 overflow-hidden rounded-md px-2 text-sidebar-foreground outline-none ring-sidebar-ring hover:bg-sidebar-accent hover:text-sidebar-accent-foreground focus-visible:ring-2 active:bg-sidebar-accent active:text-sidebar-accent-foreground disabled:pointer-events-none disabled:opacity-50 aria-disabled:pointer-events-none aria-disabled:opacity-50",
                    "data-[active=true]:bg-sidebar-accent data-[active=true]:text-sidebar-accent-foreground",
                    "[&>span:last-child]:truncate [&>svg]:size-4 [&>svg]:shrink-0 [&>svg]:text-sidebar-accent-foreground",
                    size_class,
                    class.get(),
                )
            }
        >
            {children.map(|children| children())}
        </SidebarMenuButtonRoot>
    }
}
