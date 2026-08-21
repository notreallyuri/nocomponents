use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
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

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "InputGroup",
        description: "An input and its trimmings behaving as one field: the border moves off the control and onto the box, and clicking the box's chrome lands in the control.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the box's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "A control and its addons.",
            },
        ],
    },
    ApiEntry {
        name: "InputGroupAddon",
        description: "An icon, a unit, a button — whatever shares the field with the control.",
        props: &[
            Prop {
                name: "align",
                ty: "InputGroupAlign",
                default: "InlineStart",
                description: "One of: InlineStart, InlineEnd, BlockStart, BlockEnd.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the addon's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The addon's contents.",
            },
        ],
    },
    ApiEntry {
        name: "InputGroupInput",
        description: "The control, stripped of the chrome the group now draws.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the control's classes.",
            },
            Prop {
                name: "input_type",
                ty: "InputType",
                default: "Text",
                description: "One of: Text, Email, Password, Number, File.",
            },
            Prop {
                name: "model",
                ty: "Option<RwSignal<String>>",
                default: "None",
                description: "Two-way binding. Pass this or `value`, not both.",
            },
            Prop {
                name: "value",
                ty: "Signal<String>",
                default: "\"\"",
                description: "One-way: the field shows it and never writes back.",
            },
            Prop {
                name: "id",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Overridden by an enclosing `Field`, which mints its own.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Stops it taking pointer events and marks it disabled.",
            },
        ],
    },
    ApiEntry {
        name: "InputGroupTextarea",
        description: "The same, for the block-aligned groups where the control needs more than one line.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the control's classes.",
            },
            Prop {
                name: "model",
                ty: "Option<RwSignal<String>>",
                default: "None",
                description: "Two-way binding. Pass this or `value`, not both.",
            },
            Prop {
                name: "value",
                ty: "Signal<String>",
                default: "\"\"",
                description: "One-way: the field shows it and never writes back.",
            },
            Prop {
                name: "id",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Overridden by an enclosing `Field`, which mints its own.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Stops it taking pointer events and marks it disabled.",
            },
            Prop {
                name: "auto_resize",
                ty: "bool",
                default: "false",
                description: "Grow with the text instead of scrolling.",
            },
        ],
    },
    ApiEntry {
        name: "InputGroupText",
        description: "Text that belongs to the field but is not part of its value: a unit, a character count.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the text's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The text.",
            },
        ],
    },
    ApiEntry {
        name: "InputGroupButton",
        description: "A button sized to sit inside the field rather than beside it.",
        props: &[
            Prop {
                name: "variant",
                ty: "ButtonVariant",
                default: "ButtonVariant::Ghost",
                description: "One of: Default, Outline, Ghost, Secondary, Link, Destructive.",
            },
            Prop {
                name: "size",
                ty: "ButtonSize",
                default: "ButtonSize::Xs",
                description: "One of: Xs, Sm, Default, Lg, Icon, IconSm, IconXs, IconLg.",
            },
            Prop {
                name: "on_click",
                ty: "Option<Callback<ev::MouseEvent>>",
                default: "None",
                description: "Called on press. The group leaves the click alone, so it reaches the button.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the button's classes.",
            },
            Prop {
                name: "children",
                ty: "ChildrenFn",
                default: "",
                description: "The label, usually an icon.",
            },
        ],
    },
];

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

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
