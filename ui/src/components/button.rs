use leptos::{either::Either, ev, prelude::*};

#[derive(Default, Clone, Copy, PartialEq)]
pub enum ButtonSize {
    Xs,
    Sm,
    #[default]
    Default,
    Lg,

    Icon,
    IconSm,
    IconXs,
    IconLg,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    #[default]
    Default,
    Outline,
    Ghost,
    Secondary,
    Link,
}

fn generate_button_props(
    variant: ButtonVariant,
    size: ButtonSize,
    is_disabled: bool,
    extra_class: &str,
) -> String {
    let base_classes = "group/button inline-flex shrink-0 items-center justify-center rounded-lg border bg-clip-padding text-sm font-medium whitespace-nowrap transition-all outline-none select-none focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50 active:not-aria-[haspopup]:translate-y-px disabled:pointer-events-none disabled:opacity-50 aria-invalid:border-destructive aria-invalid:ring-3 aria-invalid:ring-destructive/20 dark:aria-invalid:border-destructive/50 dark:aria-invalid:ring-destructive/40 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4";

    let variant_classes = match variant {
        ButtonVariant::Default => {
            "bg-primary border-transparent text-primary-foreground [a]:hover:bg-primary/80"
        }
        ButtonVariant::Outline => {
            "border-border bg-background hover:bg-muted hover:text-foreground aria-expanded:bg-muted aria-expanded:text-foreground dark:border-input dark:bg-input/30 dark:hover:bg-input/50"
        }
        ButtonVariant::Ghost => {
            "border-transparent hover:bg-muted hover:text-foreground aria-expanded:bg-muted aria-expanded:text-foreground dark:hover:bg-muted/50"
        }
        ButtonVariant::Secondary => {
            "border-transparent bg-neutral-200 text-neutral-800 hover:bg-neutral-300 disabled:hover:bg-neutral-200"
        }
        ButtonVariant::Link => {
            "text-primary underline-offset-4 hover:underline border-transparent"
        }
    };

    let size_classes = match size {
        ButtonSize::Default => {
            "h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
        }
        ButtonSize::Xs => "h-6 gap-1 rounded-[min(var(--radius-md),10px)] px-2 text-xs in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&_svg:not([class*='size-'])]:size-3",
        ButtonSize::Sm => "h-7 gap-1 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&_svg:not([class*='size-'])]:size-3.5",
        ButtonSize::Lg => "h-9 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2",

        ButtonSize::Icon => "size-8",
        ButtonSize::IconLg => "size-9",
        ButtonSize::IconSm => "size-7 rounded-[min(var(--radius-md),12px)] in-data-[slot=button-group]:rounded-lg",
        ButtonSize::IconXs => "size-6 rounded-[min(var(--radius-md),10px)] in-data-[slot=button-group]:rounded-lg [&_svg:not([class*='size-'])]:size-3",
    };

    let disabled_props = if is_disabled {
        "opacity-50 cursor-not-allowed"
    } else {
        ""
    };

    format!("{base_classes} {variant_classes} {size_classes} {disabled_props} {extra_class}")
}

#[component]
pub fn Button(
    #[prop(optional)] size: ButtonSize,
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(optional, into)] disabled: Signal<bool>,
) -> impl IntoView {
    view! {
        <button
            disabled=move || disabled.get()
            class=move || { generate_button_props(variant, size, disabled.get(), &class.get()) }
            on:click=move |ev| {
                if let Some(cb) = on_click {
                    cb.run(ev);
                }
            }
        >
            {match children {
                Some(child) => Either::Left(child()),
                None => Either::Right(""),
            }}
        </button>
    }
}
