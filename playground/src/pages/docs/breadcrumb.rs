use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
use leptos::prelude::*;
use nocomponents::components::breadcrumb::{
    Breadcrumb, BreadcrumbEllipsis, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage,
    BreadcrumbSeparator,
};

const DEFAULT: &str = r#"<Breadcrumb>
    <BreadcrumbList>
        <BreadcrumbItem>
            <BreadcrumbLink href="/">"Home"</BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbSeparator />
        <BreadcrumbItem>
            <BreadcrumbLink href="/docs">"Components"</BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbSeparator />
        <BreadcrumbItem>
            <BreadcrumbPage>"Breadcrumb"</BreadcrumbPage>
        </BreadcrumbItem>
    </BreadcrumbList>
</Breadcrumb>"#;

const ELLIPSIS: &str = r#"<BreadcrumbItem>
    <BreadcrumbEllipsis />
</BreadcrumbItem>"#;

const CUSTOM_SEPARATOR: &str = r#"<BreadcrumbSeparator>
    <span class="text-muted-foreground">"/"</span>
</BreadcrumbSeparator>"#;

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout title="Breadcrumb" description="The trail back up.">
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="A labelled nav around an ordered list. The last crumb is a BreadcrumbPage, not a link — you are already there."
                    code=DEFAULT
                >
                    <Breadcrumb>
                        <BreadcrumbList>
                            <BreadcrumbItem>
                                <BreadcrumbLink href="/">"Home"</BreadcrumbLink>
                            </BreadcrumbItem>
                            <BreadcrumbSeparator />
                            <BreadcrumbItem>
                                <BreadcrumbLink href="/docs">"Components"</BreadcrumbLink>
                            </BreadcrumbItem>
                            <BreadcrumbSeparator />
                            <BreadcrumbItem>
                                <BreadcrumbPage>"Breadcrumb"</BreadcrumbPage>
                            </BreadcrumbItem>
                        </BreadcrumbList>
                    </Breadcrumb>
                </DemoSection>

                <DemoSection
                    title="Collapsed"
                    description="An ellipsis stands in for the middle of a long trail; it is hidden from assistive tech, which reads the crumbs that remain."
                    code=ELLIPSIS
                >
                    <Breadcrumb>
                        <BreadcrumbList>
                            <BreadcrumbItem>
                                <BreadcrumbLink href="/">"Home"</BreadcrumbLink>
                            </BreadcrumbItem>
                            <BreadcrumbSeparator />
                            <BreadcrumbItem>
                                <BreadcrumbEllipsis />
                            </BreadcrumbItem>
                            <BreadcrumbSeparator />
                            <BreadcrumbItem>
                                <BreadcrumbLink href="/docs">"Components"</BreadcrumbLink>
                            </BreadcrumbItem>
                            <BreadcrumbSeparator />
                            <BreadcrumbItem>
                                <BreadcrumbPage>"Breadcrumb"</BreadcrumbPage>
                            </BreadcrumbItem>
                        </BreadcrumbList>
                    </Breadcrumb>
                </DemoSection>

                <DemoSection
                    title="Custom separator"
                    description="Pass children to the separator to replace the chevron; it stays hidden from assistive tech either way."
                    code=CUSTOM_SEPARATOR
                >
                    <Breadcrumb>
                        <BreadcrumbList>
                            <BreadcrumbItem>
                                <BreadcrumbLink href="/">"Home"</BreadcrumbLink>
                            </BreadcrumbItem>
                            <BreadcrumbSeparator>
                                <span class="text-muted-foreground">"/"</span>
                            </BreadcrumbSeparator>
                            <BreadcrumbItem>
                                <BreadcrumbLink href="/docs">"Components"</BreadcrumbLink>
                            </BreadcrumbItem>
                            <BreadcrumbSeparator>
                                <span class="text-muted-foreground">"/"</span>
                            </BreadcrumbSeparator>
                            <BreadcrumbItem>
                                <BreadcrumbPage>"Breadcrumb"</BreadcrumbPage>
                            </BreadcrumbItem>
                        </BreadcrumbList>
                    </Breadcrumb>
                </DemoSection>
            </div>
        </DocLayout>
    }
}
