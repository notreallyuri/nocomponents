use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
use leptos::prelude::*;
use nocomponents::{
    components::{
        button::{Button, ButtonSize, ButtonVariant},
        button_group::{ButtonGroup, ButtonGroupSeparator, ButtonGroupText},
        input::Input,
    },
    utils::types::Orientation,
};

const DEFAULT: &str = r#"<ButtonGroup>
    <Button variant=ButtonVariant::Outline>"Day"</Button>
    <Button variant=ButtonVariant::Outline>"Week"</Button>
    <Button variant=ButtonVariant::Outline>"Month"</Button>
</ButtonGroup>"#;

const SIZES: &str = r#"<ButtonGroup>
    <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>"Copy"</Button>
    <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>"Paste"</Button>
</ButtonGroup>
<ButtonGroup>
    <Button variant=ButtonVariant::Outline size=ButtonSize::Lg>"Copy"</Button>
    <Button variant=ButtonVariant::Outline size=ButtonSize::Lg>"Paste"</Button>
</ButtonGroup>"#;

const WITH_TEXT: &str = r#"<ButtonGroup>
    <ButtonGroupText>"https://"</ButtonGroupText>
    <Input attr:placeholder="example.com" class="w-56" />
    <Button variant=ButtonVariant::Outline>"Go"</Button>
</ButtonGroup>"#;

const SEPARATOR: &str = r#"<ButtonGroup>
    <Button>"Save"</Button>
    <ButtonGroupSeparator />
    <Button size=ButtonSize::Icon>"⌄"</Button>
</ButtonGroup>"#;

const VERTICAL: &str = r#"<ButtonGroup orientation=Orientation::Vertical>
    <Button variant=ButtonVariant::Outline>"Align left"</Button>
    <Button variant=ButtonVariant::Outline>"Align center"</Button>
    <Button variant=ButtonVariant::Outline>"Align right"</Button>
</ButtonGroup>"#;

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout title="Button Group" description="Buttons welded into a single control.">
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="Corners and borders are flattened where the children meet."
                    code=DEFAULT
                >
                    <ButtonGroup>
                        <Button variant=ButtonVariant::Outline>"Day"</Button>
                        <Button variant=ButtonVariant::Outline>"Week"</Button>
                        <Button variant=ButtonVariant::Outline>"Month"</Button>
                    </ButtonGroup>
                </DemoSection>

                <DemoSection
                    title="Sizes"
                    description="Buttons keep their own size; the group normalises the radius so the seam lines up."
                    code=SIZES
                >
                    <div class="flex flex-col items-start gap-3">
                        <ButtonGroup>
                            <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>
                                "Copy"
                            </Button>
                            <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>
                                "Paste"
                            </Button>
                        </ButtonGroup>
                        <ButtonGroup>
                            <Button variant=ButtonVariant::Outline size=ButtonSize::Lg>
                                "Copy"
                            </Button>
                            <Button variant=ButtonVariant::Outline size=ButtonSize::Lg>
                                "Paste"
                            </Button>
                        </ButtonGroup>
                    </div>
                </DemoSection>

                <DemoSection
                    title="With a label and an input"
                    description="ButtonGroupText is styled like a button but inert; an input inside the group takes the free space."
                    code=WITH_TEXT
                >
                    <ButtonGroup>
                        <ButtonGroupText>"https://"</ButtonGroupText>
                        <Input attr:placeholder="example.com" class="w-56" />
                        <Button variant=ButtonVariant::Outline>"Go"</Button>
                    </ButtonGroup>
                </DemoSection>

                <DemoSection
                    title="Split button"
                    description="A separator draws the seam that two buttons of the same variant would otherwise hide."
                    code=SEPARATOR
                >
                    <ButtonGroup>
                        <Button>"Save"</Button>
                        <ButtonGroupSeparator />
                        <Button size=ButtonSize::Icon>"⌄"</Button>
                    </ButtonGroup>
                </DemoSection>

                <DemoSection
                    title="Vertical"
                    description="A vertical group stacks its children and flattens the horizontal edges instead."
                    code=VERTICAL
                >
                    <ButtonGroup orientation=Orientation::Vertical>
                        <Button variant=ButtonVariant::Outline>"Align left"</Button>
                        <Button variant=ButtonVariant::Outline>"Align center"</Button>
                        <Button variant=ButtonVariant::Outline>"Align right"</Button>
                    </ButtonGroup>
                </DemoSection>
            </div>
        </DocLayout>
    }
}
