use crate::{
    cn,
    icons::x::X,
    primitives::{
        dialog::{DialogPortalRoot, use_dialog},
        drawer::{DrawerContentRoot, DrawerHandleRoot, DrawerRoot, use_drawer},
    },
    utils::types::{AsClass, Side},
};
use leptos::prelude::*;

pub use crate::components::sheet::{
    SheetClose as DrawerClose, SheetDescription as DrawerDescription, SheetFooter as DrawerFooter,
    SheetHeader as DrawerHeader, SheetTitle as DrawerTitle,
};

struct DrawerSide(Side);

impl AsClass for DrawerSide {
    fn as_class(&self) -> &'static str {
        match self.0 {
            Side::Bottom => {
                "inset-x-0 bottom-0 max-h-[85vh] rounded-t-2xl border-t sm:inset-x-4 sm:bottom-4 sm:rounded-2xl sm:border"
            }
            Side::Top => {
                "inset-x-0 top-0 max-h-[85vh] pb-6 rounded-b-2xl border-b sm:inset-x-4 sm:top-4 sm:rounded-2xl sm:border"
            }
            Side::Right => {
                "inset-y-0 right-0 w-3/4 pl-6 rounded-l-2xl border-l sm:inset-y-4 sm:right-4 sm:max-w-sm sm:rounded-2xl sm:border"
            }
            Side::Left => {
                "inset-y-0 left-0 w-3/4 pr-6 rounded-r-2xl border-r sm:inset-y-4 sm:left-4 sm:max-w-sm sm:rounded-2xl sm:border"
            }
        }
    }
}

#[component]
pub fn Drawer(
    #[prop(default = Side::Bottom)] side: Side,
    #[prop(default = None, into)] open: Option<RwSignal<bool>>,
    #[prop(default = true)] modal: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <DrawerRoot side=side open=open modal=modal>
            {children()}
        </DrawerRoot>
    }
}

#[component]
pub fn DrawerContent(
    #[prop(default = true)] handle: bool,
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_dialog();
    let modal = ctx.modal;
    let stored_children = StoredValue::new(children);
    let side_class = DrawerSide(use_drawer().side).as_class();

    view! {
        <DialogPortalRoot>
            <div data-slot="drawer-portal">
                {modal
                    .then(|| {
                        view! {
                            <div
                                data-state=move || if ctx.is_open.get() { "open" } else { "closed" }
                                class="fixed inset-0 isolate z-50 bg-black/40 duration-100 data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:pointer-events-none"
                                on:click=move |_| ctx.close()
                            />
                        }
                    })}

                <DrawerContentRoot class=move || {
                    cn!(
                        "fixed z-50 flex flex-col gap-4 bg-popover p-4 text-sm text-popover-foreground shadow-lg outline-none",
                        "transition-transform duration-300 data-[dragging=true]:transition-none",
                        "data-[state=closed]:pointer-events-none",
                        "data-[side=bottom]:data-[state=closed]:translate-y-full data-[side=top]:data-[state=closed]:-translate-y-full",
                        "data-[side=right]:data-[state=closed]:translate-x-full data-[side=left]:data-[state=closed]:-translate-x-full",
                        "sm:data-[side=bottom]:data-[state=closed]:translate-y-[calc(100%+1rem)] sm:data-[side=top]:data-[state=closed]:-translate-y-[calc(100%+1rem)]",
                        "sm:data-[side=right]:data-[state=closed]:translate-x-[calc(100%+1rem)] sm:data-[side=left]:data-[state=closed]:-translate-x-[calc(100%+1rem)]",
                        side_class,
                        class.get()
                    )
                }>
                    {handle.then(|| view! { <DrawerHandle /> })}
                    {stored_children.with_value(|children| children())}
                    <button
                        on:click=move |_| ctx.close()
                        class="absolute top-4 right-4 rounded-sm opacity-70 transition-opacity outline-none hover:opacity-100 focus-visible:ring-3 focus-visible:ring-ring/50"
                    >
                        <X />
                        <span class="sr-only">"Close"</span>
                    </button>
                </DrawerContentRoot>
            </div>
        </DialogPortalRoot>
    }
}

#[component]
pub fn DrawerHandle(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <DrawerHandleRoot class=move || {
            cn!(
                "shrink-0 rounded-full bg-muted-foreground/40",
                "data-[side=bottom]:mx-auto data-[side=bottom]:h-1.5 data-[side=bottom]:w-12",
                "data-[side=top]:absolute data-[side=top]:bottom-2 data-[side=top]:left-1/2 data-[side=top]:-translate-x-1/2 data-[side=top]:h-1.5 data-[side=top]:w-12",
                "data-[side=right]:absolute data-[side=right]:left-2 data-[side=right]:top-1/2 data-[side=right]:-translate-y-1/2 data-[side=right]:h-12 data-[side=right]:w-1.5",
                "data-[side=left]:absolute data-[side=left]:right-2 data-[side=left]:top-1/2 data-[side=left]:-translate-y-1/2 data-[side=left]:h-12 data-[side=left]:w-1.5",
                class.get()
            )
        } />
    }
}

#[component]
pub fn DrawerTrigger(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_dialog();

    view! {
        <button type="button" on:click=move |_| ctx.toggle() class=class>
            {children()}
        </button>
    }
}
