pub use crate::{
    cn,
    primitives::toast::{
        use_toast, ToastContext, ToastData, ToastPosition, ToastProviderRoot, ToastSize,
        ToastVariant,
    },
};
use leptos::{ev, prelude::*};

#[component]
pub fn ToastProvider(
    #[prop(optional)] default_variant: Option<ToastVariant>,
    #[prop(optional)] default_size: Option<ToastSize>,
    #[prop(optional)] default_position: Option<ToastPosition>,
    children: Children,
) -> impl IntoView {
    view! {
        <ToastProviderRoot
            default_variant=default_variant
            default_size=default_size
            default_position=default_position
        >
            {children()}
            <Toaster />
        </ToastProviderRoot>
    }
}

#[component]
fn Toaster() -> impl IntoView {
    let ctx = use_toast();

    let regions = vec![
        (ToastPosition::TopLeft, "top-0 left-0 flex-col"),
        (
            ToastPosition::TopCenter,
            "top-0 left-1/2 -translate-x-1/2 flex-col",
        ),
        (ToastPosition::TopRight, "top-0 right-0 flex-col"),
        (
            ToastPosition::BottomLeft,
            "bottom-0 left-0 flex-col-reverse",
        ),
        (
            ToastPosition::BottomCenter,
            "bottom-0 left-1/2 -translate-x-1/2 flex-col-reverse",
        ),
        (
            ToastPosition::BottomRight,
            "bottom-0 right-0 flex-col-reverse",
        ),
    ];

    view! {
        {regions
            .into_iter()
            .map(|(pos, placement)| {
                view! {
                    <div class=move || {
                        let has_toasts = ctx.toasts.get().iter().any(|t| t.position == pos);
                        if has_toasts {
                            cn!(
                                "fixed z-100 flex max-h-screen w-full p-4 sm:w-89 gap-4 pointer-events-none", placement
                            )
                        } else {
                            "hidden".to_string()
                        }
                    }>
                        <For
                            each=move || {
                                ctx.toasts
                                    .get()
                                    .into_iter()
                                    .filter(|t| t.position == pos)
                                    .collect::<Vec<_>>()
                            }
                            key=|t| t.id
                            children=move |toast| {
                                view! { <ToastItem toast=toast /> }
                            }
                        />
                    </div>
                }
            })
            .collect_view()}
    }
}

#[component]
fn ToastItem(toast: ToastData) -> impl IntoView {
    let ctx = expect_context::<ToastContext>();
    let id = toast.id;

    let is_dragging = RwSignal::new(false);
    let drag_offset_x = RwSignal::new(0);
    let drag_offset_y = RwSignal::new(0);
    let start_x = RwSignal::new(0);
    let start_y = RwSignal::new(0);

    let on_pointer_down = move |e: ev::PointerEvent| {
        is_dragging.set(true);
        start_x.set(e.client_x());
        start_y.set(e.client_y());
    };

    let on_pointer_move = move |e: ev::PointerEvent| {
        if is_dragging.get() {
            let diff_x = e.client_x() - start_x.get();
            let diff_y = e.client_y() - start_y.get();

            match toast.position {
                ToastPosition::TopLeft | ToastPosition::BottomLeft => {
                    if diff_x < 0 {
                        drag_offset_x.set(diff_x);
                    }
                }
                ToastPosition::TopRight | ToastPosition::BottomRight => {
                    if diff_x > 0 {
                        drag_offset_x.set(diff_x);
                    }
                }
                ToastPosition::TopCenter => {
                    if diff_y < 0 {
                        drag_offset_y.set(diff_y);
                    }
                }
                ToastPosition::BottomCenter => {
                    if diff_y > 0 {
                        drag_offset_y.set(diff_y);
                    }
                }
            }
        }
    };

    let on_pointer_up = move |_: ev::PointerEvent| {
        if is_dragging.get() {
            is_dragging.set(false);
            if drag_offset_x.get().abs() > 60 || drag_offset_y.get().abs() > 60 {
                ctx.dismiss(id);
            } else {
                drag_offset_x.set(0);
                drag_offset_y.set(0);
            }
        }
    };

    view! {
        <div
            data-state=move || { if toast.is_open.get() { "open" } else { "closed" } }
            style=move || {
                let x = drag_offset_x.get();
                let y = drag_offset_y.get();
                if x != 0 || y != 0 {
                    format!("transform: translate({}px, {}px)", x, y)
                } else {
                    String::new()
                }
            }
            on:pointerdown=on_pointer_down
            on:pointermove=on_pointer_move
            on:pointerup=on_pointer_up
            on:pointerleave=on_pointer_up
            on:pointercancel=on_pointer_up
            class=move || {
                let drag_classes = if is_dragging.get() {
                    "transition-none cursor-grabbing"
                } else {
                    "duration-300 cursor-grab"
                };
                let animation_classes = match toast.position {
                    ToastPosition::TopRight => {
                        "data-[state=open]:slide-in-from-top-full data-[state=open]:sm:slide-in-from-right-full data-[state=closed]:slide-out-to-right-full"
                    }
                    ToastPosition::BottomRight => {
                        "data-[state=open]:slide-in-from-bottom-full data-[state=open]:sm:slide-in-from-right-full data-[state=closed]:slide-out-to-right-full"
                    }
                    ToastPosition::TopLeft => {
                        "data-[state=open]:slide-in-from-top-full data-[state=open]:sm:slide-in-from-left-full data-[state=closed]:slide-out-to-left-full"
                    }
                    ToastPosition::BottomLeft => {
                        "data-[state=open]:slide-in-from-bottom-full data-[state=open]:sm:slide-in-from-left-full data-[state=closed]:slide-out-to-left-full"
                    }
                    ToastPosition::TopCenter => {
                        "data-[state=open]:slide-in-from-top-full data-[state=closed]:slide-out-to-top-full"
                    }
                    ToastPosition::BottomCenter => {
                        "data-[state=open]:slide-in-from-bottom-full data-[state=closed]:slide-out-to-bottom-full"
                    }
                };
                cn!(
                    "group pointer-events-auto relative flex w-full items-center justify-between space-x-2 text-popover-foreground overflow-hidden rounded-md border shadow-lg data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 [animation-duration:300ms] fill-mode-forwards touch-none select-none",
                    drag_classes,
                    animation_classes,
                    match toast.variant {
                        ToastVariant::Default => "bg-popover text-foreground",
                        ToastVariant::Destructive => "destructive group border-destructive bg-destructive text-destructive-foreground",
                    },
                    match toast.size {
                        ToastSize::Default => "py-2 px-4 text-sm",
                        ToastSize::Lg => "py-6 px-8 text-base",
                    }
                )
            }
        >
            <div class="grid gap-1">
                <div class="text-sm font-medium">{toast.title.clone()}</div>
                {toast
                    .description
                    .clone()
                    .map(|desc| view! { <div class="text-xs text-muted-foreground">{desc}</div> })}
            </div>
        </div>
    }
}
