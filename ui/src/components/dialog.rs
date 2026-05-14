use crate::{
    cn,
    components::button::{Button, ButtonSize, ButtonVariant},
    primitives::dialog::{use_dialog, DialogPortalRoot, DialogRoot},
};
use leptos::{either::Either, ev, prelude::*};

#[component]
pub fn Dialog(#[prop(optional)] open: Option<RwSignal<bool>>, children: Children) -> impl IntoView {
    view! { <DialogRoot open=open>{children()}</DialogRoot> }
}

#[component]
pub fn DialogTrigger(
    #[prop(optional)] size: ButtonSize,
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] as_child: Option<Callback<Callback<ev::MouseEvent>, AnyView>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_dialog();
    let on_click = Callback::new(move |_: ev::MouseEvent| ctx.toggle());

    match as_child {
        Some(render_fn) => Either::Left(render_fn.run(on_click)),
        None => Either::Right(view! {
            <Button size=size variant=variant class=class disabled=disabled on_click=on_click>
                {match children {
                    Some(child) => Either::Left(child()),
                    None => Either::Right(""),
                }}
            </Button>
        }),
    }
}

#[component]
pub fn DialogPortal(
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let context = use_dialog();
    let stored_children = StoredValue::new(children);

    view! {
        <DialogPortalRoot>
            <div data-slot="dialog-portal">
                <div
                    data-state=move || if context.is_open.get() { "open" } else { "closed" }
                    class="fixed inset-0 isolate z-50 bg-black/10 duration-100 supports-backdrop-filter:backdrop-blur-xs data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=closed]:animate-out data-[state=closed]:fade-out-0"
                    on:click=move |_| context.close()
                />

                <div
                    data-state=move || if context.is_open.get() { "open" } else { "closed" }
                    class=move || {
                        cn!(
                            "fixed top-1/2 left-1/2 z-50 grid w-full max-w-[calc(100%-2rem)] -translate-x-1/2 -translate-y-1/2 gap-4 rounded-xl bg-popover p-4 text-sm text-popover-foreground ring-1 ring-foreground/10 duration-100 outline-none sm:max-w-sm data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95",
                            class.get()
                        )
                    }
                >
                    {stored_children.with_value(|c| c())}

                    <button
                        on:click=move |_| context.close()
                        class="absolute right-4 top-4 rounded-sm opacity-70 ring-offset-background transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:pointer-events-none data-[state=open]:bg-accent data-[state=open]:text-muted-foreground"
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="24"
                            height="24"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="h-4 w-4"
                        >
                            <line x1="18" y1="6" x2="6" y2="18"></line>
                            <line x1="6" y1="6" x2="18" y2="18"></line>
                        </svg>
                        <span class="sr-only">"Close"</span>
                    </button>
                </div>
            </div>
        </DialogPortalRoot>
    }
}

#[component]
pub fn DialogHeader(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || {
            cn!("flex flex-col space-y-1.5 text-center sm:text-left", class.get())
        }>{children()}</div>
    }
}

#[component]
pub fn DialogFooter(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || {
            cn!("flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2", class.get())
        }>{children()}</div>
    }
}

#[component]
pub fn DialogTitle(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || {
            cn!("text-lg font-semibold leading-none tracking-tight", class.get())
        }>{children()}</div>
    }
}

#[component]
pub fn DialogDescription(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! { <div class=move || cn!("text-sm text-muted-foreground", class.get())>{children()}</div> }
}
