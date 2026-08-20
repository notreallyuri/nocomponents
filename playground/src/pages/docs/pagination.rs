use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
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
            </div>
        </DocLayout>
    }
}
