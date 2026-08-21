use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::{
    components::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger},
    primitives::accordion::AccordionType,
};

const DEFAULT: &str = r#"<Accordion>
    <AccordionItem value="what">
        <AccordionTrigger>"What is nocomponents?"</AccordionTrigger>
        <AccordionContent>
            "Unstyled behaviour primitives for Leptos, plus a styled layer over them."
        </AccordionContent>
    </AccordionItem>
    <AccordionItem value="why">
        <AccordionTrigger>"Why two layers?"</AccordionTrigger>
        <AccordionContent>"…"</AccordionContent>
    </AccordionItem>
</Accordion>"#;

const MULTIPLE: &str = r#"<Accordion kind=AccordionType::Multiple>
    <AccordionItem value="shipping">…</AccordionItem>
    <AccordionItem value="returns">…</AccordionItem>
</Accordion>"#;

const NOT_COLLAPSIBLE: &str = r#"<Accordion collapsible=false value=faq>
    <AccordionItem value="first">…</AccordionItem>
    <AccordionItem value="second">…</AccordionItem>
</Accordion>"#;

const DISABLED: &str = r#"<AccordionItem value="locked" disabled=true>
    <AccordionTrigger>"Enterprise settings"</AccordionTrigger>
    <AccordionContent>"Unreachable."</AccordionContent>
</AccordionItem>"#;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "Accordion",
        description: "Stacked sections, of which one or several can be open.",
        props: &[
            Prop {
                name: "kind",
                ty: "AccordionType",
                default: "Single",
                description: "One of: Single, Multiple.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the wrapper's classes.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Disables every trigger in the accordion.",
            },
            Prop {
                name: "collapsible",
                ty: "bool",
                default: "true",
                description: "Whether a `Single` accordion can be closed entirely. Ignored for `Multiple`.",
            },
            Prop {
                name: "value",
                ty: "Option<RwSignal<Vec<String>>>",
                default: "None",
                description: "Omit for an uncontrolled accordion that owns its own selection.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Items.",
            },
        ],
    },
    ApiEntry {
        name: "AccordionItem",
        description: "One section: a trigger and its panel.",
        props: &[
            Prop {
                name: "value",
                ty: "String",
                default: "",
                description: "Its key, as used in the accordion's value.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the item's classes.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Disables this section only.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "A trigger and a content.",
            },
        ],
    },
    ApiEntry {
        name: "AccordionTrigger",
        description: "The clickable header, in its own heading element.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the header's classes.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Disables this trigger only.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The header text.",
            },
        ],
    },
    ApiEntry {
        name: "AccordionContent",
        description: "The panel. Stays mounted through its close animation.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the panel's classes.",
            },
            Prop {
                name: "children",
                ty: "ChildrenFn",
                default: "",
                description: "The panel's contents. Re-run on each mount, hence `ChildrenFn`.",
            },
        ],
    },
];

#[component]
pub fn Page() -> impl IntoView {
    let faq = RwSignal::new(vec!["first".to_string()]);

    view! {
        <DocLayout title="Accordion" description="Stacked sections, one shared selection.">
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="Single: opening a section closes its sibling. The headers are one tab stop and the arrow keys walk between them."
                    code=DEFAULT
                >
                    <Accordion class="max-w-lg">
                        <AccordionItem value="what">
                            <AccordionTrigger>"What is nocomponents?"</AccordionTrigger>
                            <AccordionContent>
                                "Unstyled behaviour primitives for Leptos, plus a styled layer over them that mirrors shadcn/ui."
                            </AccordionContent>
                        </AccordionItem>
                        <AccordionItem value="why">
                            <AccordionTrigger>"Why two layers?"</AccordionTrigger>
                            <AccordionContent>
                                "So the behaviour can be reused with entirely different styling. The primitive knows about state and ARIA; the styled layer only adds classes."
                            </AccordionContent>
                        </AccordionItem>
                        <AccordionItem value="how">
                            <AccordionTrigger>"How does the height animate?"</AccordionTrigger>
                            <AccordionContent>
                                "Each panel measures itself once it mounts and publishes its own height, which the keyframes animate to."
                            </AccordionContent>
                        </AccordionItem>
                    </Accordion>
                </DemoSection>

                <DemoSection
                    title="Multiple"
                    description="Any number of sections can be open at once."
                    code=MULTIPLE
                >
                    <Accordion kind=AccordionType::Multiple class="max-w-lg">
                        <AccordionItem value="shipping">
                            <AccordionTrigger>"Shipping"</AccordionTrigger>
                            <AccordionContent>
                                "Orders leave the warehouse within two working days."
                            </AccordionContent>
                        </AccordionItem>
                        <AccordionItem value="returns">
                            <AccordionTrigger>"Returns"</AccordionTrigger>
                            <AccordionContent>
                                "Thirty days, unopened, with the receipt."
                            </AccordionContent>
                        </AccordionItem>
                    </Accordion>
                </DemoSection>

                <DemoSection
                    title="Always one open"
                    description="collapsible=false keeps the open section open when its own header is clicked, so the accordion always shows something."
                    code=NOT_COLLAPSIBLE
                >
                    <Accordion collapsible=false value=faq class="max-w-lg">
                        <AccordionItem value="first">
                            <AccordionTrigger>"First"</AccordionTrigger>
                            <AccordionContent>
                                "Clicking this header again leaves it open."
                            </AccordionContent>
                        </AccordionItem>
                        <AccordionItem value="second">
                            <AccordionTrigger>"Second"</AccordionTrigger>
                            <AccordionContent>
                                "Opening this one closes the first."
                            </AccordionContent>
                        </AccordionItem>
                    </Accordion>
                </DemoSection>

                <DemoSection
                    title="Disabled section"
                    description="A disabled item dims and the arrow keys skip its header."
                    code=DISABLED
                >
                    <Accordion class="max-w-lg">
                        <AccordionItem value="general">
                            <AccordionTrigger>"General"</AccordionTrigger>
                            <AccordionContent>"Reachable."</AccordionContent>
                        </AccordionItem>
                        <AccordionItem value="locked" disabled=true>
                            <AccordionTrigger>"Enterprise settings"</AccordionTrigger>
                            <AccordionContent>"Unreachable."</AccordionContent>
                        </AccordionItem>
                        <AccordionItem value="advanced">
                            <AccordionTrigger>"Advanced"</AccordionTrigger>
                            <AccordionContent>"Also reachable."</AccordionContent>
                        </AccordionItem>
                    </Accordion>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
