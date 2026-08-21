use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::{
    components::carousel::{
        Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPrevious,
    },
    utils::types::Orientation,
};

const DEFAULT: &str = r#"<Carousel class="mx-12 w-full max-w-md">
    <CarouselContent class="-ml-4">
        <CarouselItem>
            <div class="flex h-40 items-center justify-center rounded-lg border bg-muted">"1"</div>
        </CarouselItem>
        …
    </CarouselContent>
    <CarouselPrevious />
    <CarouselNext />
</Carousel>"#;

const SIZES: &str = r#"// A slide's width is its flex basis, so showing three at a time is a class,
// not an option: `basis-1/3`.
<CarouselItem class="basis-1/2 md:basis-1/3">…</CarouselItem>"#;

const VERTICAL: &str = r#"<Carousel orientation=Orientation::Vertical wrap=true class="mx-auto w-full max-w-xs">
    <CarouselContent class="-mt-4 h-56">
        <CarouselItem class="pt-4 pl-0">…</CarouselItem>
    </CarouselContent>
    <CarouselPrevious />
    <CarouselNext />
</Carousel>"#;

const CONTROLLED: &str = r#"{
    let index = RwSignal::new(0);

    view! {
        <Carousel index=index>…</Carousel>
        <p>{move || format!("Slide {} of 5", index.get() + 1)}</p>
    }
}"#;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "Carousel",
        description: "The carousel. Owns which slide is current and binds the arrow keys, so it can be driven without reaching for the buttons.",
        props: &[
            Prop {
                name: "orientation",
                ty: "Orientation",
                default: "Horizontal",
                description: "Which way the slides run. Vertical needs a height on the content.",
            },
            Prop {
                name: "wrap",
                ty: "bool",
                default: "false",
                description: "Whether moving past either end comes round the other side. Named `wrap` because `loop` is a Rust keyword.",
            },
            Prop {
                name: "index",
                ty: "RwSignal<usize>",
                default: "0",
                description: "The slide at the start of the track. Pass one to read the position or to jump elsewhere.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the carousel's classes. It is the positioning context the two buttons hang off.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "A content, and usually the two buttons.",
            },
        ],
    },
    ApiEntry {
        name: "CarouselContent",
        description: "The track: a real scroll container with snap points, which is why a trackpad and a touch drag work without any code. Moving to a slide is a scroll, animated by `scroll-smooth`.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the track's classes; where to set a height for a vertical carousel, and the negative margin that pairs with the slides' padding.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The slides.",
            },
        ],
    },
    ApiEntry {
        name: "CarouselItem",
        description: "One slide. Full width by default; its flex basis is what decides how many are on screen at once.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the slide's classes — `basis-1/3` to show three at a time.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The slide's contents.",
            },
        ],
    },
    ApiEntry {
        name: "CarouselPrevious",
        description: "Steps back one slide. Disabled at the first slide unless the carousel wraps.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the button's classes; where to move it if the default overhang does not suit.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Disables it beyond the automatic end-of-track rule.",
            },
        ],
    },
    ApiEntry {
        name: "CarouselNext",
        description: "Steps forward one slide. Disabled at the last slide unless the carousel wraps.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the button's classes.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Disables it beyond the automatic end-of-track rule.",
            },
        ],
    },
];

#[component]
fn Slide(label: String) -> impl IntoView {
    view! {
        <div class="flex h-40 items-center justify-center rounded-lg border bg-muted text-3xl font-semibold">
            {label}
        </div>
    }
}

#[component]
pub fn Page() -> impl IntoView {
    let index = RwSignal::new(0);

    view! {
        <DocLayout title="Carousel" description="A strip of slides you move through one at a time.">
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="The track is a scroll container with snap points rather than a transform, so a trackpad, a touch drag and the buttons all end up on the same slide — and which slide is current is read back out of the scroll position rather than assumed."
                    code=DEFAULT
                >
                    <div class="flex justify-center px-12">
                        <Carousel index=index class="w-full max-w-md">
                            <CarouselContent class="-ml-4">
                                {(1..=5)
                                    .map(|n| {
                                        view! {
                                            <CarouselItem>
                                                <Slide label=n.to_string() />
                                            </CarouselItem>
                                        }
                                    })
                                    .collect_view()}
                            </CarouselContent>
                            <CarouselPrevious />
                            <CarouselNext />
                        </Carousel>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Sizes"
                    description="How many slides are on screen is a flex basis on the slide, not a prop: the track shows whatever fits."
                    code=SIZES
                >
                    <div class="flex justify-center px-12">
                        <Carousel class="w-full max-w-lg">
                            <CarouselContent class="-ml-4">
                                {(1..=7)
                                    .map(|n| {
                                        view! {
                                            <CarouselItem class="basis-1/2 md:basis-1/3">
                                                <Slide label=n.to_string() />
                                            </CarouselItem>
                                        }
                                    })
                                    .collect_view()}
                            </CarouselContent>
                            <CarouselPrevious />
                            <CarouselNext />
                        </Carousel>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Vertical, wrapping"
                    description="The same thing on its side; the track needs a height of its own. With `wrap`, the ends join up and neither button ever disables."
                    code=VERTICAL
                >
                    <div class="flex justify-center py-6">
                        <Carousel
                            orientation=Orientation::Vertical
                            wrap=true
                            class="w-full max-w-xs"
                        >
                            <CarouselContent class="-mt-4 h-56">
                                {(1..=5)
                                    .map(|n| {
                                        view! {
                                            <CarouselItem class="pt-4 pl-0">
                                                <Slide label=n.to_string() />
                                            </CarouselItem>
                                        }
                                    })
                                    .collect_view()}
                            </CarouselContent>
                            <CarouselPrevious />
                            <CarouselNext />
                        </Carousel>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Reading the position"
                    description="The index is a signal like any other — read it for a counter, write it to jump. It follows a user's own scrolling too, since it is derived from where the track actually is."
                    code=CONTROLLED
                >
                    <p class="text-center text-sm text-muted-foreground">
                        "The first carousel is on slide "
                        <span class="font-mono text-foreground">{move || index.get() + 1}</span>
                        " of 5."
                    </p>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
