use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
use leptos::prelude::*;
use nocomponents::components::avatar::{
    Avatar, AvatarFallback, AvatarGroup, AvatarGroupCount, AvatarImage, AvatarSize,
};

const SIZES: &str = r#"<Avatar size=AvatarSize::Sm>
    <AvatarImage src=AVATAR_URL alt="User" class="object-cover" />
    <AvatarFallback>"U"</AvatarFallback>
</Avatar>
<Avatar>
    <AvatarImage src=AVATAR_URL alt="User" class="object-cover" />
    <AvatarFallback>"U"</AvatarFallback>
</Avatar>
<Avatar size=AvatarSize::Lg>
    <AvatarImage src=AVATAR_URL alt="User" class="object-cover" />
    <AvatarFallback>"U"</AvatarFallback>
</Avatar>"#;

const FALLBACK: &str = r#"<Avatar>
    <AvatarImage src="broken-url" alt="User" />
    <AvatarFallback>"YR"</AvatarFallback>
</Avatar>
<Avatar>
    <AvatarImage src="broken-url" alt="User" />
    <AvatarFallback delay_ms=150>"AB"</AvatarFallback>
</Avatar>"#;

const GROUP: &str = r#"<AvatarGroup>
    <Avatar>
        <AvatarImage src=AVATAR_URL alt="User 1" class="object-cover" />
        <AvatarFallback>"A"</AvatarFallback>
    </Avatar>
    <Avatar>
        <AvatarImage src="broken-url" alt="User 2" />
        <AvatarFallback>"B"</AvatarFallback>
    </Avatar>
    <Avatar>
        <AvatarImage src="broken-url" alt="User 3" />
        <AvatarFallback>"C"</AvatarFallback>
    </Avatar>
    <AvatarGroupCount>"+4"</AvatarGroupCount>
</AvatarGroup>"#;

const AVATAR_URL: &str = "https://w.wallhaven.cc/full/5d/wallhaven-5d23j9.png";

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout
            title="Avatar"
            description="An image element with a fallback for missing or loading states."
        >
            <div class="flex flex-col gap-8">
                <DemoSection title="Sizes" description="Sm, the default, and Lg." code=SIZES>
                    <div class="flex items-end gap-4">
                        <Avatar size=AvatarSize::Sm>
                            <AvatarImage src=AVATAR_URL alt="User" class="object-cover" />
                            <AvatarFallback>"U"</AvatarFallback>
                        </Avatar>
                        <Avatar>
                            <AvatarImage src=AVATAR_URL alt="User" class="object-cover" />
                            <AvatarFallback>"U"</AvatarFallback>
                        </Avatar>
                        <Avatar size=AvatarSize::Lg>
                            <AvatarImage src=AVATAR_URL alt="User" class="object-cover" />
                            <AvatarFallback>"U"</AvatarFallback>
                        </Avatar>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Fallback"
                    description="Shown while the image is loading or on error."
                    code=FALLBACK
                >
                    <div class="flex items-center gap-4">
                        <Avatar>
                            <AvatarImage src="broken-url" alt="User" />
                            <AvatarFallback>"YR"</AvatarFallback>
                        </Avatar>
                        <Avatar>
                            <AvatarImage src="broken-url" alt="User" />
                            <AvatarFallback delay_ms=150>"AB"</AvatarFallback>
                        </Avatar>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Group"
                    description="Members overlap in order; AvatarGroupCount closes the row with an overflow count."
                    code=GROUP
                >
                    <AvatarGroup>
                        <Avatar>
                            <AvatarImage src=AVATAR_URL alt="User 1" class="object-cover" />
                            <AvatarFallback>"A"</AvatarFallback>
                        </Avatar>
                        <Avatar>
                            <AvatarImage src="broken-url" alt="User 2" />
                            <AvatarFallback>"B"</AvatarFallback>
                        </Avatar>
                        <Avatar>
                            <AvatarImage src="broken-url" alt="User 3" />
                            <AvatarFallback>"C"</AvatarFallback>
                        </Avatar>
                        <AvatarGroupCount>"+4"</AvatarGroupCount>
                    </AvatarGroup>
                </DemoSection>
            </div>
        </DocLayout>
    }
}
