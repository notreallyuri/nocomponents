use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::components::pagination::{
    Pagination, PaginationContent, PaginationEllipsis, PaginationItem, PaginationLink,
    PaginationNext, PaginationPrevious,
};

const DEFAULT: &str = r##"<Pagination>
    <PaginationContent>
        <PaginationItem>
            <PaginationPrevious href="#" />
        </PaginationItem>
        <PaginationItem>
            <PaginationLink href="#">"1"</PaginationLink>
        </PaginationItem>
        <PaginationItem>
            <PaginationLink href="#" active=true>"2"</PaginationLink>
        </PaginationItem>
        <PaginationItem>
            <PaginationLink href="#">"3"</PaginationLink>
        </PaginationItem>
        <PaginationItem>
            <PaginationNext href="#" />
        </PaginationItem>
    </PaginationContent>
</Pagination>"##;

const ELLIPSIS: &str = r##"<PaginationItem>
    <PaginationEllipsis />
</PaginationItem>"##;

const CONTROLLED: &str = r##"<PaginationLink
    href="#"
    active=Signal::derive(move || page.get() == n)
    on:click=move |_| page.set(n)
>
    {n}
</PaginationLink>"##;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "Pagination",
        description: "Page links for a list that does not fit on one page.",
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
                description: "Usually one content.",
            },
        ],
    },
    ApiEntry {
        name: "PaginationContent",
        description: "The row of links.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the row's classes.",
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
        name: "PaginationItem",
        description: "One cell in the row.",
        props: &[Prop {
            name: "children",
            ty: "Children",
            default: "",
            description: "A link, a previous/next control, or an ellipsis.",
        }],
    },
    ApiEntry {
        name: "PaginationLink",
        description: "One page. A link rather than a button, because a page of results has a URL.",
        props: &[
            Prop {
                name: "href",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Where it goes.",
            },
            Prop {
                name: "active",
                ty: "Signal<bool>",
                default: "false",
                description: "The page currently being shown.",
            },
            Prop {
                name: "size",
                ty: "ButtonSize",
                default: "ButtonSize::Icon",
                description: "One of: Xs, Sm, Default, Lg, Icon, IconSm, IconXs, IconLg.",
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
                description: "The page number.",
            },
        ],
    },
    ApiEntry {
        name: "PaginationPrevious",
        description: "The step back, with a label beside the chevron.",
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
        ],
    },
    ApiEntry {
        name: "PaginationNext",
        description: "The step forward, with a label beside the chevron.",
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
        ],
    },
    ApiEntry {
        name: "PaginationEllipsis",
        description: "Stands in for the pages not listed.",
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
    let page = RwSignal::new(2);

    view! {
        <DocLayout title="Pagination" description="Page links.">
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="Links rather than buttons: a page of results has a URL, and the reader should be able to open page three in a new tab."
                    code=DEFAULT
                >
                    <Pagination>
                        <PaginationContent>
                            <PaginationItem>
                                <PaginationPrevious href="#" />
                            </PaginationItem>
                            <PaginationItem>
                                <PaginationLink href="#">"1"</PaginationLink>
                            </PaginationItem>
                            <PaginationItem>
                                <PaginationLink href="#" active=true>
                                    "2"
                                </PaginationLink>
                            </PaginationItem>
                            <PaginationItem>
                                <PaginationLink href="#">"3"</PaginationLink>
                            </PaginationItem>
                            <PaginationItem>
                                <PaginationNext href="#" />
                            </PaginationItem>
                        </PaginationContent>
                    </Pagination>
                </DemoSection>

                <DemoSection
                    title="With an ellipsis"
                    description="For the gap in a long list; it is hidden from assistive tech, which reads the page numbers on either side."
                    code=ELLIPSIS
                >
                    <Pagination>
                        <PaginationContent>
                            <PaginationItem>
                                <PaginationPrevious href="#" />
                            </PaginationItem>
                            <PaginationItem>
                                <PaginationLink href="#">"1"</PaginationLink>
                            </PaginationItem>
                            <PaginationItem>
                                <PaginationEllipsis />
                            </PaginationItem>
                            <PaginationItem>
                                <PaginationLink href="#" active=true>
                                    "8"
                                </PaginationLink>
                            </PaginationItem>
                            <PaginationItem>
                                <PaginationLink href="#">"9"</PaginationLink>
                            </PaginationItem>
                            <PaginationItem>
                                <PaginationEllipsis />
                            </PaginationItem>
                            <PaginationItem>
                                <PaginationLink href="#">"24"</PaginationLink>
                            </PaginationItem>
                            <PaginationItem>
                                <PaginationNext href="#" />
                            </PaginationItem>
                        </PaginationContent>
                    </Pagination>
                </DemoSection>

                <DemoSection
                    title="Driven by a signal"
                    description="active takes a signal, so the current page can come from your router or your state."
                    code=CONTROLLED
                >
                    <div class="flex flex-col items-center gap-3">
                        <Pagination>
                            <PaginationContent>
                                {(1..=5)
                                    .map(|n| {
                                        view! {
                                            <PaginationItem>
                                                <PaginationLink
                                                    href="#"
                                                    active=Signal::derive(move || page.get() == n)
                                                    on:click=move |_| page.set(n)
                                                >
                                                    {n}
                                                </PaginationLink>
                                            </PaginationItem>
                                        }
                                    })
                                    .collect_view()}
                            </PaginationContent>
                        </Pagination>
                        <span class="text-sm text-muted-foreground">
                            "Page " {move || page.get()}
                        </span>
                    </div>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
