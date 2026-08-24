use crate::{
    cn,
    primitives::input_otp::{
        InputOtpGroupRoot, InputOtpRoot, InputOtpSeparatorRoot, InputOtpSlotRoot, OtpPattern,
    },
};
use leptos::prelude::*;

#[component]
pub fn InputOtp(
    #[prop(default = 6)] length: usize,
    #[prop(optional)] pattern: OtpPattern,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(default = None, into)] value: Option<RwSignal<String>>,
    #[prop(default = None, into)] on_complete: Option<Callback<String>>,
    children: Children,
) -> impl IntoView {
    view! {
        <InputOtpRoot
            length=length
            pattern=pattern
            disabled=disabled
            value=value
            on_complete=on_complete
            class=move || {
                cn!(
                    "relative flex w-fit items-center gap-2 data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50",
                    class.get()
                )
            }
        >
            {children()}
        </InputOtpRoot>
    }
}

#[component]
pub fn InputOtpGroup(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <InputOtpGroupRoot class=move || {
            cn!(
                // The seam treatment is the button group's: one welded control, not six boxes.
                "flex items-center [&>*:not(:first-child)]:rounded-l-none [&>*:not(:first-child)]:border-l-0 [&>*:not(:last-child)]:rounded-r-none",
                class.get()
            )
        }>{children()}</InputOtpGroupRoot>
    }
}

#[component]
pub fn InputOtpSlot(
    #[prop(default = 0)] index: usize,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    view! {
        <InputOtpSlotRoot
            index=index
            class=move || {
                cn!(
                    "relative flex h-10 w-10 items-center justify-center border border-input text-sm font-medium tabular-nums transition-all first:rounded-l-lg last:rounded-r-lg",
                    "data-[active=true]:z-10 data-[active=true]:border-ring data-[active=true]:ring-3 data-[active=true]:ring-ring/50",
                    // Drawn, not real: the input's caret is transparent, and there is one of
                    // it for six boxes.
                    "data-[active=true]:after:absolute data-[active=true]:after:h-4 data-[active=true]:after:w-px data-[active=true]:after:animate-caret-blink data-[active=true]:after:bg-foreground",
                    "data-[filled=true]:after:hidden",
                    class.get()
                )
            }
        />
    }
}

#[component]
pub fn InputOtpSeparator(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <InputOtpSeparatorRoot class=move || {
            cn!("h-px w-2 shrink-0 rounded-full bg-border", class.get())
        } />
    }
}
