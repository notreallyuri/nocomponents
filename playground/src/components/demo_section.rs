use leptos::prelude::*;
use nocomponents::components::code::CodeBlock;

#[component]
pub fn DemoSection(
    title: &'static str,
    #[prop(optional)] description: Option<&'static str>,
    #[prop(optional)] code: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    view! {
        <section class="flex flex-col gap-3">
            <div class="flex flex-col gap-0.5">
                <h2 class="text-sm font-semibold text-foreground">{title}</h2>
                {description.map(|d| view! { <p class="text-xs text-muted-foreground">{d}</p> })}
            </div>
            <div class="rounded-lg border bg-card p-6">{children()}</div>
            {code.map(|code| view! { <CodeBlock code=code /> })}
        </section>
    }
}
