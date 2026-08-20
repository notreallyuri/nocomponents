use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
use leptos::prelude::*;
use nocomponents::components::{button::Button, scroll_area::ScrollArea};

const DEFAULT: &str = r#"<ScrollArea class="h-56 w-full rounded-lg border">
    <div class="p-4 text-sm">
        // ...a long list...
    </div>
</ScrollArea>"#;

const HORIZONTAL: &str = r#"<ScrollArea horizontal=true class="w-full rounded-lg border">
    <div class="flex w-max gap-3 p-4">
        // ...wide content...
    </div>
</ScrollArea>"#;

const GROWING: &str = r#"// The thumb is measured by a ResizeObserver on the viewport *and* its
// content, so content that grows after mount resizes the thumb without
// any scroll event ever firing.
<ScrollArea class="h-48 w-full rounded-lg border">
    <div class="flex flex-col gap-2 p-4 text-sm">
        {move || {
            (1..=rows.get())
                .map(|i| view! { <div class="rounded border px-3 py-2">"Row " {i}</div> })
                .collect_view()
        }}
    </div>
</ScrollArea>"#;

const FITS: &str = r#"// Content that fits gets no scrollbar at all.
<ScrollArea class="h-40 w-full rounded-lg border">
    <div class="p-4 text-sm">"Three short lines."</div>
</ScrollArea>"#;

#[component]
pub fn Page() -> impl IntoView {
    let rows = RwSignal::new(3);

    let tags: Vec<String> = (1..=40).map(|i| format!("v1.2.{i}")).collect();
    let wide: Vec<String> = (1..=14).map(|i| format!("Column {i}")).collect();

    view! {
        <DocLayout
            title="Scroll Area"
            description="A scrolling box with scrollbars this library draws itself."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="The box still scrolls natively — wheel, trackpad, touch and keyboard are all the browser's. Only the scrollbar is drawn here, because a native one cannot be styled the same way across platforms."
                    code=DEFAULT
                >
                    <div class="w-full max-w-sm">
                        <ScrollArea class="h-56 w-full rounded-lg border border-border">
                            <div class="flex flex-col gap-2 p-4 text-sm">
                                {tags
                                    .into_iter()
                                    .map(|tag| {
                                        view! {
                                            <div class="rounded border border-border px-3 py-1.5 text-muted-foreground">
                                                {tag}
                                            </div>
                                        }
                                    })
                                    .collect_view()}
                            </div>
                        </ScrollArea>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Horizontal"
                    description="Both axes are measured; the horizontal track is opt-in, since a track under content that never overflows sideways is just a line."
                    code=HORIZONTAL
                >
                    <div class="w-full max-w-sm">
                        <ScrollArea
                            horizontal=true
                            class="w-full rounded-lg border border-border"
                        >
                            <div class="flex w-max gap-3 p-4 pb-6">
                                {wide
                                    .into_iter()
                                    .map(|column| {
                                        view! {
                                            <div class="flex h-20 w-28 shrink-0 items-center justify-center rounded border border-border text-sm text-muted-foreground">
                                                {column}
                                            </div>
                                        }
                                    })
                                    .collect_view()}
                            </div>
                        </ScrollArea>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Content that grows"
                    description="Three rows fit, so there is no scrollbar. Add rows and one appears and then shrinks, without anything scrolling — a scroll listener alone would never notice."
                    code=GROWING
                >
                    <div class="flex w-full max-w-sm flex-col gap-3">
                        <ScrollArea class="h-48 w-full rounded-lg border border-border">
                            <div class="flex flex-col gap-2 p-4 text-sm">
                                {move || {
                                    (1..=rows.get())
                                        .map(|i| {
                                            view! {
                                                <div class="rounded border border-border px-3 py-2 text-muted-foreground">
                                                    "Row " {i}
                                                </div>
                                            }
                                        })
                                        .collect_view()
                                }}
                            </div>
                        </ScrollArea>
                        <div class="flex gap-2">
                            <Button on_click=Callback::new(move |_| rows.update(|r| *r += 5))>
                                "Add five rows"
                            </Button>
                            <Button on_click=Callback::new(move |_| rows.set(3))>"Reset"</Button>
                        </div>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Content that fits"
                    description="No overflow, no scrollbar — the track is not drawn rather than drawn empty."
                    code=FITS
                >
                    <div class="w-full max-w-sm">
                        <ScrollArea class="h-40 w-full rounded-lg border border-border">
                            <div class="p-4 text-sm text-muted-foreground">
                                "Three short lines, well inside the box."
                            </div>
                        </ScrollArea>
                    </div>
                </DemoSection>
            </div>
        </DocLayout>
    }
}
