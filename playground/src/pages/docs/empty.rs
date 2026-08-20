use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
use leptos::prelude::*;
use nocomponents::components::{
    button::{Button, ButtonSize, ButtonVariant},
    empty::{Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyTitle},
    input::Input,
};

const DEFAULT: &str = r#"<Empty>
    <EmptyHeader>
        <EmptyMedia>"📁"</EmptyMedia>
        <EmptyTitle>"No projects yet"</EmptyTitle>
        <EmptyDescription>
            "Projects group your work and its settings. Create one to get started."
        </EmptyDescription>
    </EmptyHeader>
    <EmptyContent>
        <Button>"New project"</Button>
    </EmptyContent>
</Empty>"#;

const WITH_FORM: &str = r#"<Empty>
    <EmptyHeader>
        <EmptyMedia>"✉️"</EmptyMedia>
        <EmptyTitle>"Invite your team"</EmptyTitle>
        <EmptyDescription>"Nobody else is here yet."</EmptyDescription>
    </EmptyHeader>
    <EmptyContent>
        <div class="flex w-full gap-2">
            <Input attr:placeholder="name@company.com" />
            <Button>"Invite"</Button>
        </div>
    </EmptyContent>
</Empty>"#;

const PLAIN_MEDIA: &str = r#"<Empty>
    <EmptyHeader>
        <EmptyMedia variant=EmptyMediaVariant::Plain>
            <span class="text-4xl">"🔍"</span>
        </EmptyMedia>
        <EmptyTitle>"No results for \"orbital\""</EmptyTitle>
        <EmptyDescription>"Check the spelling, or search for something broader."</EmptyDescription>
    </EmptyHeader>
    <EmptyContent>
        <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>"Clear search"</Button>
    </EmptyContent>
</Empty>"#;

#[component]
pub fn Page() -> impl IntoView {
    use nocomponents::components::empty::EmptyMediaVariant;

    view! {
        <DocLayout title="Empty" description="The state before there is anything to show.">
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="A centred stack: media, title, description, then the way out of the empty state."
                    code=DEFAULT
                >
                    <Empty>
                        <EmptyHeader>
                            <EmptyMedia>"📁"</EmptyMedia>
                            <EmptyTitle>"No projects yet"</EmptyTitle>
                            <EmptyDescription>
                                "Projects group your work and its settings. Create one to get started."
                            </EmptyDescription>
                        </EmptyHeader>
                        <EmptyContent>
                            <Button>"New project"</Button>
                        </EmptyContent>
                    </Empty>
                </DemoSection>

                <DemoSection
                    title="With a form"
                    description="EmptyContent is a plain container — a field and a button work as well as a single action."
                    code=WITH_FORM
                >
                    <Empty>
                        <EmptyHeader>
                            <EmptyMedia>"✉️"</EmptyMedia>
                            <EmptyTitle>"Invite your team"</EmptyTitle>
                            <EmptyDescription>"Nobody else is here yet."</EmptyDescription>
                        </EmptyHeader>
                        <EmptyContent>
                            <div class="flex w-full gap-2">
                                <Input attr:placeholder="name@company.com" />
                                <Button>"Invite"</Button>
                            </div>
                        </EmptyContent>
                    </Empty>
                </DemoSection>

                <DemoSection
                    title="Undecorated media"
                    description="The Plain variant drops the tinted tile, for an illustration that brings its own shape."
                    code=PLAIN_MEDIA
                >
                    <Empty>
                        <EmptyHeader>
                            <EmptyMedia variant=EmptyMediaVariant::Plain>
                                <span class="text-4xl">"🔍"</span>
                            </EmptyMedia>
                            <EmptyTitle>"No results for \"orbital\""</EmptyTitle>
                            <EmptyDescription>
                                "Check the spelling, or search for something broader."
                            </EmptyDescription>
                        </EmptyHeader>
                        <EmptyContent>
                            <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>
                                "Clear search"
                            </Button>
                        </EmptyContent>
                    </Empty>
                </DemoSection>
            </div>
        </DocLayout>
    }
}
