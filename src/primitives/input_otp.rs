//! A one-time code, typed a character at a time.
//!
//! One real input holds the whole value, laid transparently over painted boxes. One input per box
//! is the obvious build and the wrong one: paste drops everything after the first character,
//! phones will not offer the SMS code, and password managers fill one box and give up.
//!
//! Paste, autofill and the numeric keyboard come from the platform; this filters the value, caps
//! it at `length`, publishes the live slot, and keeps the caret at the end by refusing the keys
//! that move it — collapsing selections instead would also kill select-all, and then pasting over
//! a full field would do nothing.

use leptos::{context::Provider, ev, prelude::*, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;
use web_sys::HtmlInputElement;

/// Which characters a code may contain. Also decides the keyboard a phone puts up.
#[derive(Default, Clone, Copy, PartialEq)]
pub enum OtpPattern {
    #[default]
    Digits,
    Alphanumeric,
    Any,
}

impl OtpPattern {
    pub fn allows(&self, c: char) -> bool {
        match self {
            OtpPattern::Digits => c.is_ascii_digit(),
            OtpPattern::Alphanumeric => c.is_ascii_alphanumeric(),
            OtpPattern::Any => !c.is_control(),
        }
    }

    /// `inputmode`, so a phone offers digits rather than a full keyboard.
    pub fn input_mode(&self) -> &'static str {
        match self {
            OtpPattern::Digits => "numeric",
            OtpPattern::Alphanumeric | OtpPattern::Any => "text",
        }
    }
}

#[derive(Copy, Clone)]
pub struct InputOtpContext {
    pub value: RwSignal<String>,
    pub length: usize,
    pub disabled: Signal<bool>,
    pub focused: RwSignal<bool>,
    pub input_ref: AnyNodeRef,
}

impl InputOtpContext {
    pub fn char_at(&self, index: usize) -> Option<char> {
        self.value.with(|value| value.chars().nth(index))
    }

    pub fn filled(&self) -> usize {
        self.value.with(|value| value.chars().count())
    }

    pub fn is_complete(&self) -> bool {
        self.filled() >= self.length
    }

    /// The slot the next character lands in, or the last once full. `None` while unfocused, so
    /// an idle field does not look like it is waiting.
    pub fn active_index(&self) -> Option<usize> {
        if !self.focused.get() || self.length == 0 {
            return None;
        }

        Some(self.filled().min(self.length - 1))
    }
}

pub fn use_input_otp() -> InputOtpContext {
    expect_context::<InputOtpContext>()
}

#[component]
pub fn InputOtpRoot(
    #[prop(default = 6)] length: usize,
    #[prop(optional)] pattern: OtpPattern,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    /// Omit for an uncontrolled field.
    #[prop(default = None, into)]
    value: Option<RwSignal<String>>,
    /// Fired once the last slot fills, with the finished code.
    #[prop(default = None, into)]
    on_complete: Option<Callback<String>>,
    children: Children,
) -> impl IntoView {
    let value = value.unwrap_or_else(|| RwSignal::new(String::new()));
    let input_ref = AnyNodeRef::new();

    let context = InputOtpContext {
        value,
        length,
        disabled,
        focused: RwSignal::new(false),
        input_ref,
    };

    let input_element = move || {
        input_ref
            .get_untracked()
            .and_then(|node| node.dyn_into::<HtmlInputElement>().ok())
    };

    // The caret belongs at the end: the boxes are painted output, and the text behind them does
    // not line up with them anyway.
    let snap_caret = move || {
        if let Some(input) = input_element() {
            let end = input.value().chars().count() as u32;
            let _ = input.set_selection_range(end, end);
        }
    };

    // Refuse the keys that park the caret mid-string; selection keys are left alone, since
    // Ctrl+A then typing is how a code gets replaced.
    let handle_keydown = move |e: ev::KeyboardEvent| {
        if matches!(
            e.key().as_str(),
            "ArrowLeft" | "ArrowRight" | "ArrowUp" | "ArrowDown" | "Home" | "End"
        ) && !e.ctrl_key()
            && !e.meta_key()
        {
            e.prevent_default();
        }
    };

    let handle_input = move |e: ev::Event| {
        let raw = event_target_value(&e);

        let filtered: String = raw
            .chars()
            .filter(|c| pattern.allows(*c))
            .take(length)
            .collect();

        // The DOM keeps whatever was typed, rejected characters included, so write it back.
        if let Some(input) = input_element()
            && input.value() != filtered
        {
            input.set_value(&filtered);
        }

        let completed = filtered.chars().count() == length && value.get_untracked() != filtered;
        value.set(filtered.clone());
        snap_caret();

        if completed && let Some(callback) = on_complete {
            callback.run(filtered);
        }
    };

    view! {
        <Provider value=context>
            <div
                data-slot="input-otp"
                data-disabled=move || disabled.get().then_some("true")
                data-complete=move || context.is_complete().then_some("true")
                class=class
            >
                {children()}
                <input
                    node_ref=input_ref
                    // A real, focusable, pasteable input, merely invisible; the slots draw
                    // their own cursor.
                    style="position: absolute; inset: 0; width: 100%; height: 100%; opacity: 0; caret-color: transparent; background: transparent; border: 0; outline: none; padding: 0; cursor: default;"
                    inputmode=pattern.input_mode()
                    autocomplete="one-time-code"
                    spellcheck="false"
                    // No `maxlength`: it truncates the raw text, so "12-34-56" would be cut to
                    // "12-34-" and then filtered to four digits. The cap comes after filtering.
                    disabled=disabled
                    prop:value=move || value.get()
                    on:input=handle_input
                    on:focus=move |_| {
                        context.focused.set(true);
                        snap_caret();
                    }
                    on:blur=move |_| context.focused.set(false)
                    on:click=move |_| snap_caret()
                    on:keydown=handle_keydown
                />
            </div>
        </Provider>
    }
}

/// A group of slots; presentational, so a code can be drawn as `123 456`.
#[component]
pub fn InputOtpGroupRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div data-slot="input-otp-group" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn InputOtpSlotRoot(
    #[prop(default = 0)] index: usize,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    let ctx = use_input_otp();

    view! {
        <div
            data-slot="input-otp-slot"
            data-active=move || (ctx.active_index() == Some(index)).then_some("true")
            data-filled=move || ctx.char_at(index).is_some().then_some("true")
            data-disabled=move || ctx.disabled.get().then_some("true")
            // The slots are output: announcing each would read the code out character by
            // character.
            aria-hidden="true"
            class=class
        >
            {move || ctx.char_at(index).map(|c| c.to_string())}
        </div>
    }
}

#[component]
pub fn InputOtpSeparatorRoot(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! { <div data-slot="input-otp-separator" role="separator" aria-hidden="true" class=class></div> }
}
