use crate::{
    cn,
    primitives::timeline::{
        TimelineConnectorRoot, TimelineContentRoot, TimelineItemRoot, TimelineMarkerRoot,
        TimelineRoot, TimelineSide, TimelineState, TimelineTimeRoot,
    },
    utils::types::Orientation,
};
use leptos::{either::Either, prelude::*};

#[component]
pub fn Timeline(
    #[prop(default = Orientation::Vertical)] orientation: Orientation,
    #[prop(default = false)] alternate: bool,
    #[prop(optional)] side: TimelineSide,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <TimelineRoot
            orientation=orientation
            alternate=alternate
            side=side
            class=move || {
                cn!(
                    "flex data-[orientation=vertical]:flex-col data-[orientation=horizontal]:flex-row",
                    class.get(),
                )
            }
        >
            {children()}
        </TimelineRoot>
    }
}

#[component]
pub fn TimelineItem(
    #[prop(optional, into)] state: Signal<TimelineState>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <TimelineItemRoot
            state=state
            class=move || {
                cn!(
                    "grid gap-y-0",
                    "data-[orientation=vertical]:gap-x-4",
                    "data-[orientation=vertical]:data-[alternate=false]:data-[side=end]:grid-cols-[auto_1fr]",
                    "data-[orientation=vertical]:data-[alternate=false]:data-[side=start]:grid-cols-[1fr_auto]",
                    "data-[orientation=vertical]:data-[alternate=true]:grid-cols-[1fr_auto_1fr]",
                    "data-[orientation=horizontal]:min-w-0 data-[orientation=horizontal]:flex-1 data-[orientation=horizontal]:grid-cols-[auto_1fr] data-[orientation=horizontal]:gap-x-0 data-[orientation=horizontal]:gap-y-3",
                    "data-[orientation=horizontal]:data-[alternate=false]:data-[side=end]:grid-rows-[auto_1fr]",
                    "data-[orientation=horizontal]:data-[alternate=false]:data-[side=start]:grid-rows-[1fr_auto]",
                    "data-[orientation=horizontal]:data-[alternate=true]:grid-rows-[1fr_auto_1fr]",
                    class.get(),
                )
            }
        >
            {children()}
        </TimelineItemRoot>
    }
}

#[component]
pub fn TimelineMarker(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let marker_class = move || {
        cn!(
            "relative inline-flex size-3 shrink-0 items-center justify-center rounded-full border-2 bg-background text-xs font-medium transition-colors",
            "data-[orientation=vertical]:mt-1 data-[orientation=vertical]:row-start-1",
            "data-[orientation=vertical]:data-[alternate=false]:data-[side=end]:col-start-1",
            "data-[orientation=vertical]:data-[alternate=false]:data-[side=start]:col-start-2",
            "data-[orientation=vertical]:data-[alternate=true]:col-start-2",
            "data-[orientation=horizontal]:col-start-1",
            "data-[orientation=horizontal]:data-[alternate=false]:data-[side=end]:row-start-1",
            "data-[orientation=horizontal]:data-[alternate=false]:data-[side=start]:row-start-2",
            "data-[orientation=horizontal]:data-[alternate=true]:row-start-2",
            "data-[state=complete]:border-primary data-[state=complete]:bg-primary data-[state=complete]:text-primary-foreground",
            "data-[state=current]:border-primary data-[state=current]:bg-background data-[state=current]:ring-4 data-[state=current]:ring-primary/20",
            "data-[state=current]:before:absolute data-[state=current]:before:-inset-1 data-[state=current]:before:animate-beacon data-[state=current]:before:rounded-full data-[state=current]:before:bg-primary/25",
            "motion-reduce:before:animate-none",
            "data-[state=upcoming]:border-border data-[state=upcoming]:bg-muted data-[state=upcoming]:text-muted-foreground",
            class.get(),
        )
    };

    match children {
        Some(children) => Either::Left(
            view! { <TimelineMarkerRoot class=marker_class>{children()}</TimelineMarkerRoot> },
        ),
        None => Either::Right(view! { <TimelineMarkerRoot class=marker_class /> }),
    }
}

#[component]
pub fn TimelineConnector(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <TimelineConnectorRoot class=move || {
            cn!(
                "rounded-full bg-border transition-colors",
                "data-[orientation=vertical]:row-start-2 data-[orientation=vertical]:my-1 data-[orientation=vertical]:w-px data-[orientation=vertical]:justify-self-center",
                "data-[orientation=vertical]:data-[alternate=false]:data-[side=end]:col-start-1",
                "data-[orientation=vertical]:data-[alternate=false]:data-[side=start]:col-start-2",
                "data-[orientation=vertical]:data-[alternate=true]:col-start-2",
                "data-[orientation=horizontal]:col-start-2 data-[orientation=horizontal]:mx-2 data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-auto data-[orientation=horizontal]:self-center",
                "data-[orientation=horizontal]:data-[alternate=false]:data-[side=end]:row-start-1",
                "data-[orientation=horizontal]:data-[alternate=false]:data-[side=start]:row-start-2",
                "data-[orientation=horizontal]:data-[alternate=true]:row-start-2",
                "data-[state=complete]:bg-primary/40",
                "data-[last=true]:invisible",
                class.get(),
            )
        } />
    }
}

#[component]
pub fn TimelineContent(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <TimelineContentRoot class=move || {
            cn!(
                "min-w-0",
                "data-[orientation=vertical]:row-start-1 data-[orientation=vertical]:row-span-2 data-[orientation=vertical]:pb-8 data-[orientation=vertical]:data-[last=true]:pb-0",
                "data-[orientation=vertical]:data-[alternate=false]:data-[side=end]:col-start-2",
                "data-[orientation=vertical]:data-[alternate=false]:data-[side=start]:col-start-1 data-[orientation=vertical]:data-[alternate=false]:data-[side=start]:text-right",
                "data-[orientation=vertical]:data-[alternate=true]:data-[side=start]:col-start-1 data-[orientation=vertical]:data-[alternate=true]:data-[side=start]:text-right",
                "data-[orientation=vertical]:data-[alternate=true]:data-[side=end]:col-start-3",
                "data-[orientation=horizontal]:col-start-1 data-[orientation=horizontal]:col-span-2 data-[orientation=horizontal]:pr-6",
                "data-[orientation=horizontal]:data-[alternate=false]:data-[side=end]:row-start-2",
                "data-[orientation=horizontal]:data-[alternate=false]:data-[side=start]:row-start-1",
                "data-[orientation=horizontal]:data-[alternate=true]:data-[side=start]:row-start-1",
                "data-[orientation=horizontal]:data-[alternate=true]:data-[side=end]:row-start-3",
                "data-[state=upcoming]:opacity-60",
                class.get(),
            )
        }>{children()}</TimelineContentRoot>
    }
}

#[component]
pub fn TimelineTime(
    #[prop(optional, into)] datetime: Signal<String>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <TimelineTimeRoot
            datetime=datetime
            class=move || {
                cn!("block text-xs font-medium text-muted-foreground tabular-nums", class.get())
            }
        >
            {children()}
        </TimelineTimeRoot>
    }
}

#[component]
pub fn TimelineTitle(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="timeline-title"
            class=move || cn!("text-sm leading-tight font-semibold", class.get())
        >
            {children()}
        </div>
    }
}

#[component]
pub fn TimelineDescription(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <p
            data-slot="timeline-description"
            class=move || cn!("mt-1 text-sm text-muted-foreground", class.get())
        >
            {children()}
        </p>
    }
}
