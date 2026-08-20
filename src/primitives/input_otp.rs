//! A one-time code, typed a character at a time.
//!
//! The obvious build — one `<input>` per box, with focus hopping between them — is the one that
//! breaks everywhere it matters: paste drops everything after the first character, iOS and Android
//! will not offer the SMS code, password managers fill the first box and give up, and selecting
//! across boxes is impossible. So there is exactly **one** real input here, holding the whole
//! value, laid transparently over the boxes. The boxes are just painted output.
//!
//! That inverts what needs writing. Paste, autofill, `autocomplete="one-time-code"`, backspace and
//! the mobile numeric keyboard all come from the platform. What this module adds is filtering the
//! value to the allowed characters, capping it at `length`, publishing which slot is live, and
//! keeping the caret at the end — a code is a queue, typed at the back and rubbed out from the
//! back, and letting the caret sit in the middle only produces digits that shuffle sideways.
//!
//! "Caret at the end" is enforced by refusing the keys that move it, not by collapsing every
//! selection. Collapsing selections looks equivalent and is not: it also kills select-all, and
//! then pasting a fresh code over a full field does nothing at all, because the selection is gone
//! by the time the paste lands and `maxlength` rejects it.

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

    /// The slot the next character will land in, or the last one once the code is full. `None`
    /// while the input is not focused — an unfocused field should not look like it is waiting.
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

    // The caret belongs at the end, always. Clicking the fourth box does not mean "insert here":
    // the boxes are painted output, and the invisible text behind them does not line up with them
    // anyway, so honouring the browser's caret would put it somewhere the user did not point at.
    let snap_caret = move || {
        if let Some(input) = input_element() {
            let end = input.value().chars().count() as u32;
            let _ = input.set_selection_range(end, end);
        }
    };

    // Refuse the keys that would park the caret mid-string. Selection keys are deliberately left
    // alone: Ctrl+A then typing or pasting is how a code gets replaced.
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

        // The DOM keeps whatever was typed, including characters the pattern rejects, so it has to
        // be written back rather than left to drift out of step with the signal.
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
                    // A real, focusable, pasteable input — merely invisible. `caret-color` is
                    // transparent because the slots draw their own cursor.
                    style="position: absolute; inset: 0; width: 100%; height: 100%; opacity: 0; caret-color: transparent; background: transparent; border: 0; outline: none; padding: 0; cursor: default;"
                    inputmode=pattern.input_mode()
                    autocomplete="one-time-code"
                    spellcheck="false"
                    // Deliberately no `maxlength`: it truncates the *raw* text, so pasting
                    // "12-34-56" into a six-slot field would be cut to "12-34-" and then filtered
                    // down to four digits. The cap is applied after filtering instead, and the
                    // filtered value is written straight back to the element.
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

/// A group of slots. Purely presentational — it exists so a code can be drawn as `123 456`.
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
            // The value lives in the input; the slots are output, and announcing each box
            // separately would read the code out one meaningless character at a time.
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
