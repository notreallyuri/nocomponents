use crate::{
    cn,
    components::button::{Button, ButtonSize, ButtonVariant},
    primitives::dialog::{
        DialogContentRoot, DialogPart, DialogPortalRoot, DialogRoot, DialogTriggerRoot, use_dialog,
        use_dialog_label_id,
    },
    utils::controllable::forward,
};
use leptos::{either::Either, ev, prelude::*};

#[allow(unused_imports)]
pub use crate::utils::types::TriggerRender;

#[component]
pub fn AlertDialog(
    #[prop(default = None, into)] open: Option<RwSignal<bool>>,
    #[prop(optional)] default_open: bool,
    #[prop(optional, into)] on_open_change: Option<Callback<bool>>,
    children: Children,
) -> impl IntoView {
    view! {
        <DialogRoot open=open default_open=default_open on_open_change=forward(on_open_change)>
            {children()}
        </DialogRoot>
    }
}

#[component]
pub fn AlertDialogTrigger(
    #[prop(optional, into)] size: Signal<ButtonSize>,
    #[prop(optional, into)] variant: Signal<ButtonVariant>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] render: Option<Callback<TriggerRender, AnyView>>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let styled = Callback::new(move |trigger: TriggerRender| {
        view! {
            <Button
                size=size
                variant=variant
                node_ref=trigger.node_ref
                on_click=trigger.on_click
                disabled=trigger.disabled
                class=class
            >
                {move || {
                    children
                        .with_value(|children| match children {
                            Some(child) => Either::Left(child()),
                            None => Either::Right(view! { "" }.into_any()),
                        })
                }}
            </Button>
        }
        .into_any()
    });

    view! {
        <DialogTriggerRoot
            data_slot="alert-dialog-trigger"
            disabled=disabled
            as_child=render.unwrap_or(styled)
        />
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

                <DialogContentRoot
                    role="alertdialog"
                    class=move || {
                        cn!(
                            "fixed top-1/2 left-1/2 z-50 grid w-full max-w-[calc(100%-2rem)] -translate-x-1/2 -translate-y-1/2 gap-4 rounded-xl bg-popover p-4 text-sm text-popover-foreground ring-1 ring-foreground/10 duration-100 outline-none sm:max-w-sm data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=closed]:pointer-events-none",
                            class.get()
                        )
                    }
                >
                    {stored_children.with_value(|c| c())}
                </DialogContentRoot>
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
    let ctx = use_dialog();
    let handle_click = move |e: ev::MouseEvent| {
        if let Some(cb) = on_click {
            cb.run(e);
        }
        ctx.close();
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
