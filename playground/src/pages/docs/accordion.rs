use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
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
            </div>
        </DocLayout>
    }
}
