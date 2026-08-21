use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::components::{
    button::{Button, ButtonSize, ButtonVariant},
    kbd::{Kbd, KbdGroup},
};

const COMBINATION: &str = r#"<KbdGroup>
    <Kbd>"⌘"</Kbd>
    <Kbd>"K"</Kbd>
</KbdGroup>"#;

const SINGLE_KEYS: &str = r#"<Kbd>"⌘"</Kbd>
<Kbd>"⇧"</Kbd>
<Kbd>"⌥"</Kbd>
<Kbd>"Esc"</Kbd>
<Kbd>"↵"</Kbd>"#;

const IN_A_BUTTON: &str = r#"<Button variant=ButtonVariant::Outline size=ButtonSize::Sm>
    "Search"
    <KbdGroup class="ml-2">
        <Kbd>"⌘"</Kbd>
        <Kbd>"K"</Kbd>
    </KbdGroup>
</Button>"#;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "KbdGroup",
        description: "A run of keys, for a chord like Ctrl+Shift+P.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the layout classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The keys.",
            },
        ],
    },
    ApiEntry {
        name: "Kbd",
        description: "One key, drawn as a keycap.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the keycap classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The key name.",
            },
        ],
    },
];

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout title="Kbd" description="Displays a keyboard key or shortcut.">
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Single keys"
                    description="One key per element; a glyph or a short word both work."
                    code=SINGLE_KEYS
                >
                    <div class="flex flex-wrap items-center gap-2">
                        <Kbd>"⌘"</Kbd>
                        <Kbd>"⇧"</Kbd>
                        <Kbd>"⌥"</Kbd>
                        <Kbd>"Esc"</Kbd>
                        <Kbd>"↵"</Kbd>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Combination"
                    description="Group keys that are pressed together."
                    code=COMBINATION
                >
                    <KbdGroup>
                        <Kbd>"⌘"</Kbd>
                        <Kbd>"K"</Kbd>
                    </KbdGroup>
                </DemoSection>

                <DemoSection
                    title="In a button"
                    description="A shortcut hint riding along with the label it belongs to."
                    code=IN_A_BUTTON
                >
                    <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>
                        "Search"
                        <KbdGroup class="ml-2">
                            <Kbd>"⌘"</Kbd>
                            <Kbd>"K"</Kbd>
                        </KbdGroup>
                    </Button>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
