use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::{components::separator::Separator, utils::types::Orientation};

const HORIZONTAL: &str = r#"<Separator />"#;

const VERTICAL: &str = r#""Docs" <Separator orientation=Orientation::Vertical /> "Source"
<Separator orientation=Orientation::Vertical /> "Playground""#;

const SEMANTIC: &str = r#"<p class="text-sm">"Account settings"</p>
<Separator decorative=false />
<p class="text-sm">"Danger zone"</p>"#;

const API: &[ApiEntry] = &[ApiEntry {
    name: "Separator",
    description: "A rule between things.",
    props: &[
        Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Merged over the rule's own classes.",
        },
        Prop {
            name: "orientation",
            ty: "Orientation",
            default: "Horizontal",
            description: "One of: Horizontal, Vertical.",
        },
        Prop {
            name: "decorative",
            ty: "bool",
            default: "true",
            description: "Hidden from assistive tech. Set false when the rule groups unrelated things rather than decorating.",
        },
    ],
}];

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout title="Separator" description="Visually or semantically separates content.">
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Horizontal"
                    description="The default orientation — a full-width rule between blocks."
                    code=HORIZONTAL
                >
                    <div class="flex flex-col gap-4">
                        <div class="flex flex-col gap-1">
                            <h4 class="text-sm font-medium">"nocomponents"</h4>
                            <p class="text-sm text-muted-foreground">
                                "Radix-style primitives for Leptos."
                            </p>
                        </div>
                        <Separator />
                        <p class="text-sm text-muted-foreground">"Everything below the rule."</p>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Vertical"
                    description="Give the parent a height — a vertical rule fills it."
                    code=VERTICAL
                >
                    <div class="flex h-6 items-center gap-4 text-sm">
                        "Docs" <Separator orientation=Orientation::Vertical /> "Source"
                        <Separator orientation=Orientation::Vertical /> "Playground"
                    </div>
                </DemoSection>

                <DemoSection
                    title="Semantic"
                    description="Pass decorative=false when the rule separates meaningful groups; it is then exposed to assistive tech as a separator."
                    code=SEMANTIC
                >
                    <div class="flex flex-col gap-4">
                        <p class="text-sm">"Account settings"</p>
                        <Separator decorative=false />
                        <p class="text-sm">"Danger zone"</p>
                    </div>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
