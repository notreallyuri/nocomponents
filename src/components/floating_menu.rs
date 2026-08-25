use crate::{
    cn,
    icons::ellipsis::Ellipsis,
    primitives::floating_menu::{FloatingMenuHandleRoot, FloatingMenuRoot},
};
use leptos::prelude::*;

#[component]
pub fn FloatingMenu(
    #[prop(optional)] position: Option<RwSignal<(f64, f64)>>,
    #[prop(default = (24.0, 24.0))] default_position: (f64, f64),
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <FloatingMenuRoot
            position=position
            default_position=default_position
            class=move || {
                cn!(
                    "z-50 flex min-w-40 flex-col overflow-hidden rounded-lg border bg-popover text-popover-foreground shadow-lg ring-1 ring-foreground/10",
                    // Lifted while it travels, and never transitioned: the panel is following a
                    // pointer, and easing it would have it trail behind the cursor.
                    "data-[dragging=true]:shadow-2xl data-[dragging=true]:ring-primary/40",
                    class.get(),
                )
            }
        >
            {children()}
        </FloatingMenuRoot>
    }
}

/// The strip you move it by. Give it a label, or leave it empty for a bare grip.
#[component]
pub fn FloatingMenuHandle(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <FloatingMenuHandleRoot
            disabled=disabled
            class=move || {
                cn!(
                    "flex cursor-grab touch-none items-center gap-2 border-b bg-muted/50 px-2 py-1.5 text-xs font-medium select-none",
                    "data-[dragging=true]:cursor-grabbing",
                    class.get(),
                )
            }
        >
            <Ellipsis class="size-3.5 shrink-0 text-muted-foreground" />
            {children.map(|children| children())}
        </FloatingMenuHandleRoot>
    }
}

#[component]
pub fn FloatingMenuContent(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="floating-menu-content"
            class=move || cn!("flex flex-col gap-1 p-1.5", class.get())
        >
            {children()}
        </div>
    }
}
