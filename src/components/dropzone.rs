use crate::{
    cn,
    primitives::dropzone::{DropzoneRoot, use_dropzone},
};
// Through `leptos` rather than `web_sys` directly: an installed component may only name leptos
// and what `deps` re-exports, and the consumer compiling this file has leptos either way.
use leptos::{prelude::*, web_sys::File};

#[component]
pub fn Dropzone(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] accept: Signal<String>,
    #[prop(default = true)] multiple: bool,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(into)] on_files: Callback<Vec<File>>,
    #[prop(default = None, into)] on_rejected: Option<Callback<Vec<File>>>,
    children: Children,
) -> impl IntoView {
    view! {
        <DropzoneRoot
            accept=accept
            multiple=multiple
            disabled=disabled
            on_files=on_files
            on_rejected=on_rejected
            class=move || {
                cn!(
                    "flex w-full cursor-pointer flex-col items-center justify-center gap-2 rounded-lg border border-dashed border-input px-6 py-10 text-center transition-colors outline-none",
                    "hover:bg-accent/40 focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50",
                    "data-[over=true]:border-primary data-[over=true]:bg-primary/5",
                    "data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50",
                    class.get(),
                )
            }
        >
            {children()}
        </DropzoneRoot>
    }
}

#[component]
pub fn DropzoneLabel(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <span
            data-slot="dropzone-label"
            class=move || cn!("text-sm font-medium text-foreground", class.get())
        >
            {children()}
        </span>
    }
}

#[component]
pub fn DropzoneDescription(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <span
            data-slot="dropzone-description"
            class=move || cn!("text-xs text-muted-foreground", class.get())
        >
            {children()}
        </span>
    }
}

#[component]
pub fn DropzoneTrigger(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_dropzone();

    view! {
        <button
            type="button"
            data-slot="dropzone-trigger"
            on:click=move |e| {
                e.stop_propagation();
                ctx.open();
            }
            class=move || {
                cn!(
                    "rounded-md text-sm font-medium text-primary underline underline-offset-4 outline-none hover:text-primary/80 focus-visible:ring-3 focus-visible:ring-ring/50",
                    class.get(),
                )
            }
        >
            {children()}
        </button>
    }
}
