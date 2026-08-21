use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
use leptos::prelude::*;
use nocomponents::{
    components::input_group::{
        InputGroup, InputGroupAddon, InputGroupButton, InputGroupInput, InputGroupText,
        InputGroupTextarea,
    },
    icons::{copy::Copy, search::Search},
    primitives::input_group::InputGroupAlign,
};

const DEFAULT: &str = r#"<InputGroup>
    <InputGroupAddon>
        <Search />
    </InputGroupAddon>
    <InputGroupInput value="" />
</InputGroup>"#;

const SUFFIX: &str = r#"<InputGroup>
    <InputGroupInput value="nocomponents" />
    <InputGroupAddon align=InputGroupAlign::InlineEnd>
        <InputGroupText>".dev"</InputGroupText>
    </InputGroupAddon>
</InputGroup>"#;

const BUTTON: &str = r#"<InputGroup>
    <InputGroupInput value="https://nocomponents.dev/docs" />
    <InputGroupAddon align=InputGroupAlign::InlineEnd>
        <InputGroupButton on_click=Callback::new(move |_| copied.set(true))>
            <Copy />
        </InputGroupButton>
    </InputGroupAddon>
</InputGroup>"#;

const BLOCK: &str = r#"<InputGroup>
    <InputGroupTextarea value="" />
    <InputGroupAddon align=InputGroupAlign::BlockEnd>
        <InputGroupText>{move || format!("{} / 280", note.get().len())}</InputGroupText>
        <InputGroupButton class="ml-auto">"Post"</InputGroupButton>
    </InputGroupAddon>
</InputGroup>"#;

const DISABLED: &str = r#"<InputGroup>
    <InputGroupAddon>
        <InputGroupText>"$"</InputGroupText>
    </InputGroupAddon>
    <InputGroupInput value="0.00" disabled=true />
</InputGroup>"#;

#[component]
pub fn Page() -> impl IntoView {
    let query = RwSignal::new(String::new());
    let domain = RwSignal::new("nocomponents".to_string());
    let url = RwSignal::new("https://nocomponents.dev/docs".to_string());
    let note = RwSignal::new("Unstyled behaviour, styled separately.".to_string());
    let copied = RwSignal::new(false);

    view! {
        <DocLayout
            title="Input Group"
            description="An input and its trimmings, behaving as one control."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="The border moves off the input and onto the box around it. Clicking anywhere in that box — the padding, the icon — focuses the input, the way a real field has no dead zone inside its own border."
                    code=DEFAULT
                >
                    <div class="w-full max-w-sm">
                        <InputGroup>
                            <InputGroupAddon>
                                <Search />
                            </InputGroupAddon>
                            <InputGroupInput model=query />
                        </InputGroup>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Suffix"
                    description="An end-aligned addon pushes itself to the far edge, so the control keeps whatever is left."
                    code=SUFFIX
                >
                    <div class="w-full max-w-sm">
                        <InputGroup>
                            <InputGroupInput model=domain />
                            <InputGroupAddon align=InputGroupAlign::InlineEnd>
                                <InputGroupText>".dev"</InputGroupText>
                            </InputGroupAddon>
                        </InputGroup>
                    </div>
                </DemoSection>

                <DemoSection
                    title="With a button"
                    description="Pressing a button in an addon presses the button — the group only redirects clicks that landed on its own chrome."
                    code=BUTTON
                >
                    <div class="flex w-full max-w-sm flex-col gap-2">
                        <InputGroup>
                            <InputGroupInput model=url />
                            <InputGroupAddon align=InputGroupAlign::InlineEnd>
                                <InputGroupButton on_click=Callback::new(move |_| {
                                    copied.set(true)
                                })>
                                    <Copy />
                                </InputGroupButton>
                            </InputGroupAddon>
                        </InputGroup>
                        <span class="text-sm text-muted-foreground">
                            {move || if copied.get() { "Copied." } else { "\u{a0}" }}
                        </span>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Block addon"
                    description="A block-aligned addon takes a row of its own, which is what a textarea with a counter and an action wants."
                    code=BLOCK
                >
                    <div class="w-full max-w-sm">
                        <InputGroup>
                            <InputGroupTextarea model=note />
                            <InputGroupAddon align=InputGroupAlign::BlockEnd>
                                <InputGroupText>
                                    {move || format!("{} / 280", note.get().len())}
                                </InputGroupText>
                                <InputGroupButton class="ml-auto">"Post"</InputGroupButton>
                            </InputGroupAddon>
                        </InputGroup>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Disabled"
                    description="Disabling the control dims the whole field, since the field is what the reader sees."
                    code=DISABLED
                >
                    <div class="w-full max-w-sm">
                        <InputGroup>
                            <InputGroupAddon>
                                <InputGroupText>"$"</InputGroupText>
                            </InputGroupAddon>
                            <InputGroupInput value="0.00" disabled=true />
                        </InputGroup>
                    </div>
                </DemoSection>
            </div>
        </DocLayout>
    }
}
