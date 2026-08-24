use crate::{cn, icons::chevron::ChevronDown, primitives::native_select::NativeSelectRoot};
use leptos::{ev, prelude::*};

#[derive(Default, Clone, Copy)]
pub enum NativeSelectSize {
    Sm,
    #[default]
    Default,
}

impl NativeSelectSize {
    fn as_class(&self) -> &'static str {
        match self {
            NativeSelectSize::Sm => "h-7 text-[0.8rem]",
            NativeSelectSize::Default => "h-8 text-sm",
        }
    }
}

#[component]
pub fn NativeSelect(
    #[prop(optional)] size: NativeSelectSize,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(default = None, into)] model: Option<RwSignal<String>>,
    #[prop(optional, into)] value: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] id: Signal<String>,
    /// Run when the control loses focus. The chevron needs a positioned wrapper around the
    /// `<select>`, and that wrapper is what an `on:blur` at the call site would attach to — where
    /// it would never fire, since `blur` does not bubble. This reaches the control itself.
    #[prop(default = None, into)]
    on_blur: Option<Callback<ev::FocusEvent>>,
    children: Children,
) -> impl IntoView {
    let size_class = size.as_class();

    view! {
        // The chevron is ours because `appearance-none` takes the platform one away; it sits over
        // the control and lets clicks through to it.
        <div class="relative inline-flex w-full items-center">
            <NativeSelectRoot
                id=id
                model=model
                value=value
                disabled=disabled
                on_blur=on_blur
                class=move || {
                    cn!(
                        "w-full appearance-none rounded-lg border border-input bg-transparent py-1 pr-8 pl-2.5 transition-colors outline-none focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:bg-input/50 disabled:opacity-50 aria-invalid:border-destructive aria-invalid:ring-3 aria-invalid:ring-destructive/20 dark:bg-input/30 dark:disabled:bg-input/80 [&>optgroup]:bg-popover [&>option]:bg-popover",
                        size_class,
                        class.get()
                    )
                }
            >
                {children()}
            </NativeSelectRoot>
            <ChevronDown class="pointer-events-none absolute right-2.5 size-4 text-muted-foreground" />
        </div>
    }
}
