use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::components::skeleton::{Skeleton, SkeletonVariant};

const PROFILE: &str = r#"<Skeleton class="size-12 rounded-full" />
<div class="space-y-2">
    <Skeleton class="h-4 w-[250px]" />
    <Skeleton class="h-4 w-[200px]" />
</div>"#;

const CARD: &str = r#"<Skeleton class="h-[125px] w-[250px] rounded-xl" />
<div class="space-y-2">
    <Skeleton class="h-4 w-[250px]" />
    <Skeleton class="h-4 w-[200px]" />
</div>"#;

const VARIANTS: &str = r#"<Skeleton variant=SkeletonVariant::Tile class="w-24" />
<div class="w-40 space-y-2">
    <Skeleton variant=SkeletonVariant::Line />
    <Skeleton variant=SkeletonVariant::Line class="w-2/3" />
</div>
<Skeleton variant=SkeletonVariant::Card class="w-40" />"#;

const API: &[ApiEntry] = &[ApiEntry {
    name: "Skeleton",
    description: "A pulsing placeholder in the shape of what is loading. Size it with `class`. The pulse stops for visitors who ask for reduced motion.",
    props: &[
        Prop {
            name: "variant",
            ty: "Signal<SkeletonVariant>",
            default: "Default",
            description: "Default is a bare bone sized by `class`. Tile is a square, Line a line of text, Card a square cover over two lines — written as `data-variant`.",
        },
        Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "The shape: width, height and rounding. On a Card it sizes the whole card.",
        },
    ],
}];

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout
            title="Skeleton"
            description="Use to show a placeholder while content is loading."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Profile"
                    description="Combining a circular skeleton with text lines."
                    code=PROFILE
                >
                    <div class="flex items-center space-x-4">
                        <Skeleton class="size-12 rounded-full" />
                        <div class="space-y-2">
                            <Skeleton class="h-4 w-[250px]" />
                            <Skeleton class="h-4 w-[200px]" />
                        </div>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Card"
                    description="A larger block skeleton to mimic a loaded card component."
                    code=CARD
                >
                    <div class="flex flex-col space-y-3">
                        <Skeleton class="h-[125px] w-[250px] rounded-xl" />
                        <div class="space-y-2">
                            <Skeleton class="h-4 w-[250px]" />
                            <Skeleton class="h-4 w-[200px]" />
                        </div>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Variants"
                    description="Tile, Line and Card are the shapes a media grid and a list of rows keep reaching for. Each is marked aria-hidden, and none pulses under prefers-reduced-motion."
                    code=VARIANTS
                >
                    <div class="flex items-start gap-6">
                        <Skeleton variant=SkeletonVariant::Tile class="w-24" />
                        <div class="w-40 space-y-2">
                            <Skeleton variant=SkeletonVariant::Line />
                            <Skeleton variant=SkeletonVariant::Line class="w-2/3" />
                        </div>
                        <Skeleton variant=SkeletonVariant::Card class="w-40" />
                    </div>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
