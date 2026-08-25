use crate::{
    cn,
    components::button::{Button, ButtonSize, ButtonVariant},
    primitives::dialog::{
        DialogContentRoot, DialogPart, DialogPortalRoot, DialogRoot, use_dialog,
        use_dialog_label_id,
    },
};
use leptos::{either::Either, ev, prelude::*};

#[component]
pub fn AlertDialog(
    #[prop(optional)] open: Option<RwSignal<bool>>,
    children: Children,
) -> impl IntoView {
    view! { <DialogRoot open=open>{children()}</DialogRoot> }
}

#[component]
pub fn AlertDialogTrigger(
    #[prop(optional)] size: ButtonSize,
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] render: Option<Callback<Callback<ev::MouseEvent>, AnyView>>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let ctx = use_dialog();
    let on_click = Callback::new(move |_: ev::MouseEvent| ctx.toggle());

    match render {
        Some(render_fn) => Either::Left(render_fn.run(on_click)),
        None => Either::Right(view! {
            <Button
                size=size
                variant=variant
                class=class
                disabled=disabled
                on:click=move |e| on_click.run(e)
            >
                {
                    let children = children.clone();
                    move || match &children {
                        Some(child) => Either::Left(child()),
                        None => Either::Right(view! { "" }.into_any()),
                    }
                }
            </Button>
        }),
    }
}

#[component]
pub fn AlertDialogPortal(
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
                    class="fixed inset-0 isolate z-50 bg-black/10 duration-100 supports-backdrop-filter:backdrop-blur-xs data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:pointer-events-none"
                />

                <DialogContentRoot class=move || {
                    cn!(
                        "fixed top-1/2 left-1/2 z-50 grid w-full max-w-[calc(100%-2rem)] -translate-x-1/2 -translate-y-1/2 gap-4 rounded-xl bg-popover p-4 text-sm text-popover-foreground ring-1 ring-foreground/10 duration-100 outline-none sm:max-w-sm data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=closed]:pointer-events-none",
                            class.get()
                    )
                }>{stored_children.with_value(|c| c())}</DialogContentRoot>
            </div>
        </DialogPortalRoot>
    }
}

#[component]
pub fn AlertDialogHeader(
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
pub fn AlertDialogFooter(
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
pub fn AlertDialogTitle(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let id = use_dialog_label_id(DialogPart::Title);

    view! {
        <div
            id=id
            class=move || cn!("text-lg font-semibold leading-none tracking-tight", class.get())
        >
            {children()}
        </div>
    }
}

#[component]
pub fn AlertDialogDescription(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let id = use_dialog_label_id(DialogPart::Description);

    view! {
        <div id=id class=move || cn!("text-sm text-muted-foreground", class.get())>
            {children()}
        </div>
    }
}

#[component]
pub fn AlertDialogAction(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let handle_click = move |e: ev::MouseEvent| {
        if let Some(cb) = on_click {
            cb.run(e);
        }
    };

    view! {
        <Button class=class on_click=handle_click>
            {
                let children = children.clone();
                move || match &children {
                    Some(child) => Either::Left(child()),
                    None => Either::Right(view! { "" }.into_any()),
                }
            }
        </Button>
    }
}

#[component]
pub fn AlertDialogCancel(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let ctx = use_dialog();

    let handle_click = move |e: ev::MouseEvent| {
        if let Some(cb) = on_click {
            cb.run(e);
        }
        ctx.close();
    };

    view! {
        <Button
            variant=ButtonVariant::Outline
            class=move || cn!("mt-2 sm:mt-0", class.get())
            on_click=handle_click
        >
            {
                let children = children.clone();
                move || match &children {
                    Some(child) => Either::Left(child()),
                    None => Either::Right(view! { "Cancel" }.into_any()),
                }
            }
        </Button>
    }
}
