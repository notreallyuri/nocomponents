use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
use leptos::prelude::*;
use nocomponents::components::skeleton::Skeleton;

const PROFILE: &str = r#"<Skeleton class="size-12 rounded-full" />
<div class="space-y-2">
    <Skeleton class="h-4 w-[250px]" />
    <Skeleton class="h-4 w-[200px]" />
</div>"#;

const CARD: &str = r#"<Skeleton class="h-[125px] w-[250px] rounded-xl" />
<div class="space-y-2">
    <Skeleton class="h-4 w-[250px]" />
    <Skeleton class="h-4 w-[200px]" />
</div>"#;

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout
            title="Skeleton"
            description="Use to show a placeholder while content is loading."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Profile"
                    description="Combining a circular skeleton with text lines."
                    code=PROFILE
                >
                    <div class="flex items-center space-x-4">
                        <Skeleton class="size-12 rounded-full" />
                        <div class="space-y-2">
                            <Skeleton class="h-4 w-[250px]" />
                            <Skeleton class="h-4 w-[200px]" />
                        </div>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Card"
                    description="A larger block skeleton to mimic a loaded card component."
                    code=CARD
                >
                    <div class="flex flex-col space-y-3">
                        <Skeleton class="h-[125px] w-[250px] rounded-xl" />
                        <div class="space-y-2">
                            <Skeleton class="h-4 w-[250px]" />
                            <Skeleton class="h-4 w-[200px]" />
                        </div>
                    </div>
                </DemoSection>
            </div>
        </DocLayout>
    }
}
