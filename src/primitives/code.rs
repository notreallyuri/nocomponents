use crate::utils::types::Language;
use leptos::{context::Provider, ev, prelude::*};
use std::time::Duration;

#[derive(Copy, Clone)]
pub struct CodeContext {
    pub code: StoredValue<String>,
    /// True for a short window after a successful copy, so the trigger can acknowledge it.
    pub copied: RwSignal<bool>,
    reset_after: u64,
}

impl CodeContext {
    pub fn copy(&self) {
        let text = self.code.get_value();
        let _ = window().navigator().clipboard().write_text(&text);

        self.copied.set(true);

        let copied = self.copied;
        set_timeout(
            move || copied.set(false),
            Duration::from_millis(self.reset_after),
        );
    }
}

pub fn use_code() -> CodeContext {
    expect_context::<CodeContext>()
}

#[component]
pub fn CodeBlockRoot(
    #[prop(into)] code: String,
    #[prop(optional, into)] class: Signal<String>,
    /// How long the copied acknowledgement stays up, in milliseconds.
    #[prop(default = 1500)]
    reset_after: u64,
    children: Children,
) -> impl IntoView {
    let context = CodeContext {
        code: StoredValue::new(code),
        copied: RwSignal::new(false),
        reset_after,
    };

    view! {
        <Provider value=context>
            <div data-slot="code-block" class=class>
                {children()}
            </div>
        </Provider>
    }
}

#[component]
pub fn CodeBlockContentRoot(
    #[prop(optional)] language: Language,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    let ctx = use_code();

    view! {
        <pre data-slot="code-block-content" data-language=language.as_str() class=class>
            <code>{move || render_code(ctx.code.get_value(), language)}</code>
        </pre>
    }
}

/// Each token becomes a `data-token` span the styled layer can colour; plain runs stay bare text
/// so the DOM does not fill up with spans that carry no meaning.
#[cfg(feature = "highlight")]
fn render_code(code: String, language: Language) -> AnyView {
    use crate::utils::highlight::{TokenKind, tokenize};
    use leptos::either::Either;

    tokenize(&code, language)
        .into_iter()
        .map(|token| {
            let text = token.text.to_string();
            match token.kind {
                TokenKind::Plain => Either::Left(text),
                kind => Either::Right(view! { <span data-token=kind.as_str()>{text}</span> }),
            }
        })
        .collect_view()
        .into_any()
}

#[cfg(not(feature = "highlight"))]
fn render_code(code: String, _language: Language) -> AnyView {
    code.into_any()
}

#[component]
pub fn CodeCopyRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_code();

    view! {
        <button
            type="button"
            data-slot="code-copy"
            aria-label=move || if ctx.copied.get() { "Copied" } else { "Copy code" }
            data-state=move || if ctx.copied.get() { "copied" } else { "idle" }
            on:click=move |_: ev::MouseEvent| ctx.copy()
            class=class
        >
            {children.map(|c| c())}
        </button>
    }
}
