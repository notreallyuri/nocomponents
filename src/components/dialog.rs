use crate::{
    cn,
    components::button::{Button, ButtonSize, ButtonVariant},
    icons::x::X,
    primitives::dialog::{
        DialogContentRoot, DialogPart, DialogPortalRoot, DialogRoot, DialogTriggerRoot, use_dialog,
        use_dialog_label_id,
    },
    utils::{
        controllable::forward,
        icon::{IconSlot, slot_view},
    },
};

const SHEET_CLOSE_ICON: &str = "flex size-4 items-center justify-center [&>svg]:size-full";
use leptos::{either::Either, prelude::*};

#[allow(unused_imports)]
pub use crate::utils::types::TriggerRender;

#[component]
pub fn Dialog(
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
pub fn DialogTrigger(
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
            data_slot="dialog-trigger"
            disabled=disabled
            as_child=render.unwrap_or(styled)
        />
    }
}

#[component]
pub fn DialogPortal(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(default = true.into(), into)] show_close: Signal<bool>,
    #[prop(optional, into)] mobile_sheet: Signal<bool>,
    #[prop(optional, into)] sheet_title: Signal<String>,
    #[prop(optional, into)] sheet_snap_points: Signal<Vec<f64>>,
    #[prop(default = true.into(), into)] sheet_show_close: Signal<bool>,
    #[prop(default = "Close".into(), into)] sheet_close_label: Signal<String>,
    #[prop(optional, into)] sheet_close_icon: Option<ViewFn>,
    #[prop(default = true.into(), into)] sheet_draggable: Signal<bool>,
    #[prop(optional, into)] close_icon: Option<ViewFn>,
    #[prop(default = "Close".into(), into)] close_label: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let context = use_dialog();
    let close_icon = StoredValue::new(close_icon);
    let sheet_close_icon = StoredValue::new(sheet_close_icon);
    let stored_children = StoredValue::new(children);
    let sheet_glyph =
        |icon: Option<ViewFn>| slot_view(icon, ViewFn::from(|| view! { <X /> }), SHEET_CLOSE_ICON);

    view! {
        <DialogPortalRoot>
            <div data-slot="dialog-portal">
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
                <DialogContentRoot
                    mobile_sheet=mobile_sheet
                    sheet_title=sheet_title
                    sheet_snap_points=sheet_snap_points
                    sheet_show_close=sheet_show_close
                    sheet_draggable=sheet_draggable
                    sheet_close_label=sheet_close_label
                    sheet_close_icon=sheet_close_icon.get_value().map(|icon| sheet_glyph(Some(icon)))
                    show_close=show_close
                    close_icon=Some(sheet_glyph(close_icon.get_value()))
                    class=move || {
                        cn!(
                            "fixed top-1/2 left-1/2 z-50 grid w-full max-w-[calc(100%-2rem)] -translate-x-1/2 -translate-y-1/2 gap-4 rounded-xl bg-popover p-4 text-sm text-popover-foreground ring-1 ring-foreground/10 duration-100 outline-none sm:max-w-sm data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=closed]:pointer-events-none",
                        "data-[presentation=sheet]:top-auto data-[presentation=sheet]:bottom-0 data-[presentation=sheet]:left-0 data-[presentation=sheet]:w-full data-[presentation=sheet]:max-w-none data-[presentation=sheet]:max-h-[90dvh] data-[presentation=sheet]:translate-x-0 data-[presentation=sheet]:translate-y-0 data-[presentation=sheet]:overflow-y-auto data-[presentation=sheet]:overscroll-contain data-[presentation=sheet]:rounded-b-none data-[presentation=sheet]:rounded-t-xl data-[presentation=sheet]:pb-[max(1rem,env(safe-area-inset-bottom))] data-[presentation=sheet]:data-[state=open]:slide-in-from-bottom-full data-[presentation=sheet]:data-[state=closed]:slide-out-to-bottom-full data-[presentation=sheet]:data-[state=open]:zoom-in-100 data-[presentation=sheet]:data-[state=closed]:zoom-out-100 [&_[data-slot=menu-sheet-title]]:px-0 [&_[data-slot=menu-sheet-title]]:pb-2 [&_[data-slot=menu-sheet-title]]:text-sm [&_[data-slot=menu-sheet-title]]:font-semibold [&_[data-slot=menu-sheet-title]]:text-foreground [&_[data-slot=menu-sheet-close]]:flex [&_[data-slot=menu-sheet-close]]:size-8 [&_[data-slot=menu-sheet-close]]:items-center [&_[data-slot=menu-sheet-close]]:justify-center [&_[data-slot=menu-sheet-close]]:rounded-md [&_[data-slot=menu-sheet-close]]:text-muted-foreground [&_[data-slot=menu-sheet-close]]:outline-none [&_[data-slot=menu-sheet-close]]:hover:bg-accent [&_[data-slot=menu-sheet-close]]:hover:text-accent-foreground [&_[data-slot=menu-sheet-close]]:focus-visible:ring-2 [&_[data-slot=menu-sheet-close]]:focus-visible:ring-ring data-[presentation=sheet]:transition-transform data-[presentation=sheet]:duration-200 motion-reduce:data-[presentation=sheet]:transition-none data-[presentation=sheet]:data-[dragging=true]:transition-none",
                        class.get()
                        )
                    }
                >
                    {stored_children.with_value(|c| c())}
                    {move || {
                        show_close
                            .get()
                            .then(|| {
                                view! {
                                    <button
                                        type="button"
                                        data-slot="dialog-close"
                                        on:click=move |_| context.close()
                                        class="absolute right-4 top-4 in-data-[presentation=sheet]:hidden rounded-sm opacity-70 ring-offset-background transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:pointer-events-none data-[state=open]:bg-accent data-[state=open]:text-muted-foreground"
                                    >
                                        <IconSlot
                                            icon=close_icon.get_value()
                                            fallback=|| view! { <X /> }
                                            class="flex size-4 items-center justify-center [&>svg]:size-full"
                                        />
                                        <span class="sr-only">{move || close_label.get()}</span>
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
pub fn DialogDescription(
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
pub fn DialogClose(
    #[prop(optional, into)] size: Signal<ButtonSize>,
    #[prop(default = ButtonVariant::Outline.into(), into)] variant: Signal<ButtonVariant>,
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_dialog();
    let stored_children = StoredValue::new(children);

    view! {
        <Button size=size variant=variant class=class on_click=move |_| ctx.close()>
            {move || stored_children.with_value(|children| children())}
        </Button>
    }
}
