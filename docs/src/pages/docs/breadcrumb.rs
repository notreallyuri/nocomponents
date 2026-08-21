use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
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

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "Breadcrumb",
        description: "The trail back up. A labelled `<nav>` around an ordered list.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the nav's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Usually one list.",
            },
        ],
    },
    ApiEntry {
        name: "BreadcrumbList",
        description: "The list itself.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the list's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Items and separators.",
            },
        ],
    },
    ApiEntry {
        name: "BreadcrumbItem",
        description: "One step in the trail.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the item's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "A link, a page, or an ellipsis.",
            },
        ],
    },
    ApiEntry {
        name: "BreadcrumbLink",
        description: "A step you can go back to.",
        props: &[
            Prop {
                name: "href",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Where it goes.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the link's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The step's name.",
            },
        ],
    },
    ApiEntry {
        name: "BreadcrumbPage",
        description: "The page you are on: not a link, and announced as the current one.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the page's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The page's name.",
            },
        ],
    },
    ApiEntry {
        name: "BreadcrumbSeparator",
        description: "The mark between two steps. Hidden from assistive tech — punctuation, not content.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the separator's classes.",
            },
            Prop {
                name: "children",
                ty: "Option<Children>",
                default: "None",
                description: "Defaults to a chevron.",
            },
        ],
    },
    ApiEntry {
        name: "BreadcrumbEllipsis",
        description: "Stands in for the middle of a long trail.",
        props: &[Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Merged over the ellipsis's classes.",
        }],
    },
];

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

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
