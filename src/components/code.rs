use crate::{
    cn,
    icons::{check::Check, copy::Copy},
    primitives::code::{CodeBlockContentRoot, CodeBlockRoot, CodeCopyRoot, use_code},
    utils::types::Language,
};
use leptos::{either::Either, prelude::*};

/// Inline code, for a symbol or a fragment mentioned in running text.
#[component]
pub fn Code(#[prop(optional, into)] class: Signal<String>, children: Children) -> impl IntoView {
    view! {
        <code
            data-slot="code"
            class=move || {
                cn!(
                    "rounded-sm bg-muted px-[0.3rem] py-[0.15rem] font-mono text-[0.85em] font-medium text-foreground",
                    class.get(),
                )
            }
        >
            {children()}
        </code>
    }
}

/// Colours for the `data-token` spans the highlighter emits. Tailwind's own palette rather than
/// theme tokens, so a consumer gets readable code without adding variables to their CSS.
const TOKEN_COLORS: &str = "[&_[data-token=keyword]]:text-rose-600 dark:[&_[data-token=keyword]]:text-rose-400 [&_[data-token=type]]:text-violet-600 dark:[&_[data-token=type]]:text-violet-400 [&_[data-token=macro]]:text-teal-600 dark:[&_[data-token=macro]]:text-teal-300 [&_[data-token=string]]:text-emerald-700 dark:[&_[data-token=string]]:text-emerald-300 [&_[data-token=number]]:text-amber-700 dark:[&_[data-token=number]]:text-amber-300 [&_[data-token=lifetime]]:text-amber-700 dark:[&_[data-token=lifetime]]:text-amber-300 [&_[data-token=attribute]]:text-sky-700 dark:[&_[data-token=attribute]]:text-sky-300 [&_[data-token=comment]]:text-muted-foreground [&_[data-token=comment]]:italic";

/// A block of code with an optional filename header and a copy button.
#[component]
pub fn CodeBlock(
    #[prop(into)] code: String,
    /// Shown in the header — usually a file name or a short caption.
    #[prop(optional, into)]
    label: Option<String>,
    #[prop(optional)] language: Language,
    #[prop(default = true)] copyable: bool,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    let has_header = label.is_some();

    view! {
        <CodeBlockRoot
            code=code
            class=move || {
                cn!(
                    "group/code relative overflow-hidden rounded-xl bg-card text-card-foreground ring-1 ring-foreground/10",
                    class.get(),
                )
            }
        >
            {label
                .map(|label| {
                    view! {
                        <div
                            data-slot="code-block-header"
                            class="flex items-center justify-between gap-2 border-b border-foreground/10 px-4 py-2 font-mono text-xs text-muted-foreground"
                        >
                            <span>{label}</span>
                        </div>
                    }
                })}

            {copyable
                .then(|| {
                    view! { <CodeCopyButton class=if has_header { "top-1.5" } else { "top-3" } /> }
                })}

            <CodeBlockContentRoot
                language=language
                class=cn!(
                    "overflow-x-auto p-4 font-mono text-[0.8rem] leading-relaxed", TOKEN_COLORS
                )
            />
        </CodeBlockRoot>
    }
}

/// Copy trigger for the enclosing [`CodeBlock`]. Swaps to a check for a moment after copying.
#[component]
pub fn CodeCopyButton(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    let ctx = use_code();

    view! {
        <CodeCopyRoot class=move || {
            cn!(
                "absolute right-3 z-10 inline-flex size-7 items-center justify-center rounded-lg border border-transparent bg-card/80 text-muted-foreground opacity-0 backdrop-blur-xs transition-all outline-none hover:bg-muted hover:text-foreground focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50 focus-visible:opacity-100 group-hover/code:opacity-100 data-[state=copied]:text-foreground data-[state=copied]:opacity-100",
                class.get(),
            )
        }>
            {move || {
                if ctx.copied.get() {
                    Either::Left(view! { <Check class="size-3.5" /> })
                } else {
                    Either::Right(view! { <Copy class="size-3.5" /> })
                }
            }}
        </CodeCopyRoot>
    }
}
