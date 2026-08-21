use leptos::prelude::*;
use nocomponents::{
    components::{
        code::CodeBlock,
        collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger},
    },
    icons::chevron::ChevronDown,
};

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

            // The demo and its snippet are one block: the toggle is a strip under the demo and the
            // code unfolds inside the same border, so it is never in doubt which demo it belongs
            // to. Folded by default — the demo is the point, and the snippet is for afterwards.
            <div class="overflow-hidden rounded-lg border bg-card">
                <div class="p-6">{children()}</div>

                {code
                    .map(|code| {
                        view! {
                            <Collapsible class="gap-0">
                                <CollapsibleTrigger class="group/code-toggle justify-start gap-1.5 rounded-none border-t px-3 py-1.5 text-xs text-muted-foreground hover:bg-muted/50 hover:text-foreground">
                                    <ChevronDown class="size-3.5 transition-transform group-data-[state=open]/code-toggle:rotate-180" />
                                    <span class="group-data-[state=open]/code-toggle:hidden">
                                        "Show code"
                                    </span>
                                    <span class="hidden group-data-[state=open]/code-toggle:inline">
                                        "Hide code"
                                    </span>
                                </CollapsibleTrigger>
                                <CollapsibleContent>
                                    <CodeBlock code=code class="rounded-none border-t ring-0" />
                                </CollapsibleContent>
                            </Collapsible>
                        }
                    })}
            </div>
        </section>
    }
}
