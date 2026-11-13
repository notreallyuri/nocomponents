use crate::{
    cn,
    components::button::{Button, ButtonSize, ButtonVariant},
    icons::x::X,
    primitives::dialog::{
        DialogContentRoot, DialogPart, DialogPortalRoot, DialogRoot, DialogTriggerRoot, use_dialog,
        use_dialog_label_id,
    },
    utils::{controllable::forward, icon::IconSlot, types::AsClass},
};
use leptos::{either::Either, prelude::*};

#[allow(unused_imports)]
pub use crate::utils::types::{Side, TriggerRender};

struct SheetSide(Side);

impl AsClass for SheetSide {
    fn as_class(&self) -> &'static str {
        match self.0 {
            Side::Right => {
                "inset-y-0 right-0 h-full w-3/4 border-l sm:max-w-sm data-[state=open]:slide-in-from-right data-[state=closed]:slide-out-to-right"
            }
            Side::Left => {
                "inset-y-0 left-0 h-full w-3/4 border-r sm:max-w-sm data-[state=open]:slide-in-from-left data-[state=closed]:slide-out-to-left"
            }
            Side::Top => {
                "inset-x-0 top-0 h-auto border-b data-[state=open]:slide-in-from-top data-[state=closed]:slide-out-to-top"
            }
            Side::Bottom => {
                "inset-x-0 bottom-0 h-auto border-t data-[state=open]:slide-in-from-bottom data-[state=closed]:slide-out-to-bottom"
            }
        }
    }
}

#[component]
pub fn Sheet(
    #[prop(default = None, into)] open: Option<RwSignal<bool>>,
    #[prop(optional)] default_open: bool,
    #[prop(optional, into)] on_open_change: Option<Callback<bool>>,
    #[prop(default = true)] modal: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <DialogRoot
            open=open
            default_open=default_open
            on_open_change=forward(on_open_change)
            modal=modal
        >
            {children()}
        </DialogRoot>
    }
}

#[component]
pub fn SheetTrigger(
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
            data_slot="sheet-trigger"
            disabled=disabled
            as_child=render.unwrap_or(styled)
        />
    }
}

#[component]
pub fn SheetContent(
    #[prop(default = Side::Right.into(), into)] side: Signal<Side>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] id: Option<String>,
    #[prop(default = true.into(), into)] show_close: Signal<bool>,
    #[prop(optional, into)] close_icon: Option<ViewFn>,
    children: ChildrenFn,
) -> impl IntoView {
    let context = use_dialog();
    if let Some(id) = id {
        context.content_id.set(id);
    }
    let close_icon = StoredValue::new(close_icon);
    let stored_children = StoredValue::new(children);

    view! {
        <DialogPortalRoot>
            <div data-slot="sheet-portal">
                {context
                    .modal
                    .then(|| {
                        view! {
                            <div
                                data-state=move || {
                                    if context.is_open.get() { "open" } else { "closed" }
                                }
                                class="fixed inset-0 isolate z-50 bg-black/10 duration-100 supports-backdrop-filter:backdrop-blur-xs data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:pointer-events-none"
                                on:click=move |_| context.close()
                            />
                        }
                    })}
                <DialogContentRoot class=move || {
                    cn!(
                        "fixed z-50 flex flex-col gap-4 bg-popover p-4 text-sm text-popover-foreground shadow-lg duration-200 outline-none data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:pointer-events-none",
                        SheetSide(side.get()).as_class(),
                        class.get()
                    )
                }>
                    {stored_children.with_value(|children| children())}
                    {move || {
                        show_close
                            .get()
                            .then(|| {
                                view! {
                                    <button
                                        type="button"
                                        data-slot="sheet-close"
                                        on:click=move |_| context.close()
                                        class="absolute top-4 right-4 rounded-sm opacity-70 transition-opacity outline-none hover:opacity-100 focus-visible:ring-3 focus-visible:ring-ring/50"
                                    >
                                        <IconSlot
                                            icon=close_icon.get_value()
                                            fallback=|| view! { <X /> }
                                            class="flex size-4 items-center justify-center [&>svg]:size-full"
                                        />
                                        <span class="sr-only">"Close"</span>
                                    </button>
                                }
                            })
                    }}
                </DialogContentRoot>
            </div>
        </DialogPortalRoot>
    }
}

#[component]
pub fn SheetHeader(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! { <div class=move || { cn!("flex flex-col gap-1.5 pr-8", class.get()) }>{children()}</div> }
}

#[component]
pub fn SheetTitle(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let id = use_dialog_label_id(DialogPart::Title);

    view! {
        <div id=id class=move || cn!("text-base font-semibold tracking-tight", class.get())>
            {children()}
        </div>
    }
}

#[component]
pub fn SheetDescription(
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
pub fn SheetFooter(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || {
            cn!("mt-auto flex flex-col-reverse gap-2 sm:flex-row sm:justify-end", class.get())
        }>{children()}</div>
    }
}

#[component]
pub fn SheetClose(
    #[prop(optional, into)] size: Signal<ButtonSize>,
    #[prop(default = ButtonVariant::Outline.into(), into)] variant: Signal<ButtonVariant>,
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_dialog();
    let stored_children = StoredValue::new(children);

    view! {
        <Button size=size variant=variant class=class on:click=move |_| ctx.close()>
            {move || stored_children.with_value(|children| children())}
        </Button>
    }
}
