use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use leptos_router::components::A;
use nocomponents::components::badge::{Badge, BadgeVariant, StyledRender};

const AS_A_LINK: &str = r#"<Badge
    variant=BadgeVariant::Outline
    render=Callback::new(|badge: StyledRender| {
        view! { <A href="/blocks?uses=badge" attr:class=badge.class>"Blocks using badge"</A> }
            .into_any()
    })
/>"#;

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
            ty: "Signal<BadgeVariant>",
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
        Prop {
            name: "render",
            ty: "Option<Callback<StyledRender, AnyView>>",
            default: "None",
            description: "Render as something else — an `<a>` or a router `<A>`, where the variants' hover styles apply. Handed the classes and a node ref.",
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

                <DemoSection
                    title="As a link"
                    description="With render the badge is the anchor itself, which is where the variants' hover styles apply."
                    code=AS_A_LINK
                >
                    <Badge
                        variant=BadgeVariant::Outline
                        render=Callback::new(|badge: StyledRender| {
                            view! {
                                <A href="/blocks?uses=badge" attr:class=badge.class>
                                    "Blocks using badge"
                                </A>
                            }
                                .into_any()
                        })
                    />
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
