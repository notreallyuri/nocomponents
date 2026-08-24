use crate::{
    cn,
    components::{
        button::{Button, ButtonSize, ButtonVariant},
        input::{Input, InputType},
        textarea::Textarea,
    },
    primitives::input_group::{InputGroupAddonRoot, InputGroupAlign, InputGroupRoot},
};
use leptos::{ev, prelude::*};

#[component]
pub fn InputGroup(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <InputGroupRoot class=move || {
            cn!(
                "group/input-group relative flex w-full flex-wrap items-center gap-1.5 rounded-lg border border-input bg-transparent px-2.5 transition-colors outline-none has-[>[data-align=block-start]]:flex-col has-[>[data-align=block-end]]:flex-col has-[>[data-align=block-start]]:items-stretch has-[>[data-align=block-end]]:items-stretch has-[>[data-align=block-start]]:py-2 has-[>[data-align=block-end]]:py-2 has-[input:focus-visible]:border-ring has-[input:focus-visible]:ring-3 has-[input:focus-visible]:ring-ring/50 has-[textarea:focus-visible]:border-ring has-[textarea:focus-visible]:ring-3 has-[textarea:focus-visible]:ring-ring/50 has-[input[aria-invalid=true]]:border-destructive has-[input[aria-invalid=true]]:ring-3 has-[input[aria-invalid=true]]:ring-destructive/20 has-[input:disabled]:pointer-events-none has-[input:disabled]:bg-input/50 has-[input:disabled]:opacity-50 dark:bg-input/30",
                class.get()
            )
        }>{children()}</InputGroupRoot>
    }
}

#[component]
pub fn InputGroupAddon(
    #[prop(optional)] align: InputGroupAlign,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <InputGroupAddonRoot
            align=align
            class=move || {
                cn!(
                    "flex items-center gap-1.5 text-sm text-muted-foreground select-none data-[align=inline-end]:ml-auto data-[align=block-start]:w-full data-[align=block-end]:w-full [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
                    class.get()
                )
            }
        >
            {children()}
        </InputGroupAddonRoot>
    }
}

#[component]
pub fn InputGroupInput(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] input_type: InputType,
    #[prop(default = None, into)] model: Option<RwSignal<String>>,
    #[prop(optional, into)] value: Signal<String>,
    #[prop(optional, into)] id: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
) -> impl IntoView {
    view! {
        <Input
            input_type=input_type
            model=model
            value=value
            id=id
            disabled=disabled
            class=move || {
                cn!(
                    "h-8 flex-1 rounded-none border-0 bg-transparent px-0 focus-visible:border-0 focus-visible:ring-0 disabled:bg-transparent dark:bg-transparent dark:disabled:bg-transparent",
                    class.get()
                )
            }
        />
    }
}

#[component]
pub fn InputGroupTextarea(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(default = None, into)] model: Option<RwSignal<String>>,
    #[prop(optional, into)] value: Signal<String>,
    #[prop(optional, into)] id: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(default = false)] auto_resize: bool,
) -> impl IntoView {
    view! {
        <Textarea
            model=model
            value=value
            id=id
            disabled=disabled
            auto_resize=auto_resize
            class=move || {
                cn!(
                    "min-h-12 w-full flex-1 resize-none rounded-none border-0 bg-transparent px-0 py-0 focus-visible:border-0 focus-visible:ring-0 disabled:bg-transparent dark:bg-transparent dark:disabled:bg-transparent",
                    class.get()
                )
            }
        />
    }
}

#[component]
pub fn InputGroupText(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <span
            data-slot="input-group-text"
            class=move || cn!("text-sm text-muted-foreground select-none", class.get())
        >
            {children()}
        </span>
    }
}

#[component]
pub fn InputGroupButton(
    #[prop(default = ButtonVariant::Ghost)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Xs)] size: ButtonSize,
    #[prop(optional, into)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    // `Button`'s builder wants a `Callback`, so an absent handler becomes a no-op one.
    let on_click = Callback::new(move |e: ev::MouseEvent| {
        if let Some(cb) = on_click {
            cb.run(e);
        }
    });

    view! {
        <Button
            variant=variant
            size=size
            on_click=on_click
            class=move || cn!("rounded-md", class.get())
            children=children
        />
    }
}
