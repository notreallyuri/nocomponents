use crate::cn;
use leptos::prelude::*;

#[component]
pub fn Kbd(#[prop(optional, into)] class: Signal<String>, children: Children) -> impl IntoView {
    view! {
        <kbd
            data-slot="kbd"
            class=move || {
                cn!(
                    "pointer-events-none inline-flex h-5 w-fit min-w-5 items-center justify-center gap-1 rounded-sm bg-muted px-1 font-sans text-xs font-medium text-muted-foreground select-none [&_svg:not([class*='size-'])]:size-3",
                    class.get()
                )
            }
        >
            {children()}
        </kbd>
    }
}

#[component]
pub fn KbdGroup(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <kbd data-slot="kbd-group" class=move || cn!("inline-flex items-center gap-1", class.get())>
            {children()}
        </kbd>
    }
}
