use leptos::prelude::*;
use std::time::Duration;

#[derive(Copy, Clone, PartialEq)]
pub enum ImageStatus {
    Idle,
    Loading,
    Loaded,
    Error,
}

#[derive(Copy, Clone)]
pub struct AvatarContext {
    pub status: RwSignal<ImageStatus>,
}

pub fn use_avatar() -> AvatarContext {
    expect_context::<AvatarContext>()
}

#[component]
pub fn AvatarRoot(children: Children) -> impl IntoView {
    provide_context(AvatarContext {
        status: RwSignal::new(ImageStatus::Idle),
    });
    view! { {children()} }
}

#[component]
pub fn AvatarImageRoot(
    #[prop(into)] src: String,
    #[prop(optional, into)] alt: String,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] data_slot: &'static str,
) -> impl IntoView {
    let ctx = use_avatar();

    ctx.status.set(ImageStatus::Loading);

    view! {
        <img
            src=src
            alt=alt
            class=class
            data-slot=data_slot
            on:load=move |_| ctx.status.set(ImageStatus::Loaded)
            on:error=move |_| ctx.status.set(ImageStatus::Error)
            style=move || {
                if ctx.status.get() == ImageStatus::Loaded { "" } else { "display: none;" }
            }
        />
    }
}

#[component]
pub fn AvatarFallbackRoot(
    #[prop(default = None, into)] delay_ms: Option<u64>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_avatar();
    let show = RwSignal::new(delay_ms.is_none());

    if let Some(delay) = delay_ms {
        Effect::new(move |_| {
            set_timeout(move || show.set(true), Duration::from_millis(delay));
        });
    }

    view! {
        <Show when=move || show.get() && ctx.status.get() != ImageStatus::Loaded>{children()}</Show>
    }
}
