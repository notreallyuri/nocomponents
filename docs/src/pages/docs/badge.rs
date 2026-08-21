use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::components::badge::{Badge, BadgeVariant};

const VARIANTS: &str = r#"<Badge variant=BadgeVariant::Default>"Default"</Badge>
<Badge variant=BadgeVariant::Secondary>"Secondary"</Badge>
<Badge variant=BadgeVariant::Outline>"Outline"</Badge>
<Badge variant=BadgeVariant::Ghost>"Ghost"</Badge>
<Badge variant=BadgeVariant::Destructive>"Destructive"</Badge>"#;

const API: &[ApiEntry] = &[ApiEntry {
    name: "Badge",
    description: "A small label for a status or a count. Style-only.",
    props: &[
        Prop {
            name: "variant",
            ty: "BadgeVariant",
            default: "Default",
            description: "One of: Default, Secondary, Destructive, Outline, Ghost, Link.",
        },
        Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Merged over the variant's classes.",
        },
        Prop {
            name: "children",
            ty: "Children",
            default: "",
            description: "The label.",
        },
    ],
}];

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout title="Badge" description="Small status and label indicators.">
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Variants"
                    description="Five variants, sized to sit inline with the text around them."
                    code=VARIANTS
                >
                    <div class="flex flex-wrap items-center gap-3">
                        <Badge variant=BadgeVariant::Default>"Default"</Badge>
                        <Badge variant=BadgeVariant::Secondary>"Secondary"</Badge>
                        <Badge variant=BadgeVariant::Outline>"Outline"</Badge>
                        <Badge variant=BadgeVariant::Ghost>"Ghost"</Badge>
                        <Badge variant=BadgeVariant::Destructive>"Destructive"</Badge>
                    </div>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
