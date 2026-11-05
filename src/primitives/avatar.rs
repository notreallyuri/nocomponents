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
    /// Changing it starts loading again: the status resets, so the fallback comes back until the
    /// new image has arrived.
    #[prop(into)]
    src: Signal<String>,
    #[prop(optional, into)] alt: Signal<String>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] data_slot: &'static str,
) -> impl IntoView {
    let ctx = use_avatar();

    ctx.status.set(ImageStatus::Loading);

    Effect::new(move |previous: Option<()>| {
        src.track();
        if previous.is_some() {
            ctx.status.set(ImageStatus::Loading);
        }
    });

    view! {
        <img
            src=move || src.get()
            alt=move || alt.get()
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
        let pending = StoredValue::new(None::<TimeoutHandle>);

        Effect::new(move |_| {
            if let Some(handle) = pending.try_update_value(|handle| handle.take()).flatten() {
                handle.clear();
            }
            if let Ok(handle) =
                set_timeout_with_handle(move || show.set(true), Duration::from_millis(delay))
            {
                pending.set_value(Some(handle));
            }
        });

        on_cleanup(move || {
            if let Some(handle) = pending.try_update_value(|handle| handle.take()).flatten() {
                handle.clear();
            }
        });
    }

    view! {
        <Show when=move || show.get() && ctx.status.get() != ImageStatus::Loaded>{children()}</Show>
    }
}
