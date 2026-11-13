use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::components::{
    button::{Button, ButtonSize, ButtonType, ButtonVariant},
    input::{Input, InputType},
    label::Label,
    native_select::{NativeSelect, NativeSelectOption},
    textarea::Textarea,
};

const DEFAULT: &str = r#"<Input attr:placeholder="Enter text..." />"#;

const TYPES: &str = r#"<div class="flex flex-col gap-1.5">
    <Label>"Email"</Label>
    <Input input_type=InputType::Email attr:placeholder="you@example.com" />
</div>
<div class="flex flex-col gap-1.5">
    <Label>"Password"</Label>
    <Input
        input_type=InputType::Password
        attr:placeholder="••••••••"
    />
</div>
<div class="flex flex-col gap-1.5">
    <Label>"Number"</Label>
    <Input input_type=InputType::Number attr:placeholder="0" />
</div>"#;

const CONTROLLED: &str = r#"{
    let value = RwSignal::new(String::new());
    view! {
        <div class="flex flex-col gap-3 max-w-sm">
            <Input model=value attr:placeholder="Type something..." />
            <p class="text-sm text-muted-foreground">
                "Value: " {move || value.get()}
            </p>
        </div>
    }
}"#;

const REVEAL: &str = r#"{
    let shown = RwSignal::new(false);
    let input_type = Signal::derive(move || {
        if shown.get() { InputType::Text } else { InputType::Password }
    });
    view! {
        <div class="flex max-w-sm gap-2">
            <Input input_type=input_type attr:placeholder="Password" />
            <Button
                variant=ButtonVariant::Outline
                size=ButtonSize::Sm
                on_click=move |_| shown.update(|s| *s = !*s)
            >
                {move || if shown.get() { "Hide" } else { "Show" }}
            </Button>
        </div>
    }
}"#;

const MORE_TYPES: &str = r#"<div class="flex flex-col gap-3 max-w-sm">
    <Input input_type=InputType::Tel attr:placeholder="+44 20 7946 0000" />
    <Input input_type=InputType::Url attr:placeholder="https://example.com" />
    <Input input_type=InputType::Search attr:placeholder="Search..." />
    <Input input_type=InputType::Date />
</div>"#;

const RESET: &str = r#"{
    let name = RwSignal::new("Ada".to_string());
    let default_name = RwSignal::new("Ada".to_string());
    let bio = RwSignal::new("Mathematician".to_string());
    let role = RwSignal::new("editor".to_string());
    view! {
        <form class="flex max-w-sm flex-col gap-3">
            <Input model=name default_value=default_name />
            <Textarea model=bio default_value="Mathematician" rows=2 />
            <NativeSelect model=role default_value="editor">
                <NativeSelectOption value="admin">"Admin"</NativeSelectOption>
                <NativeSelectOption value="editor">"Editor"</NativeSelectOption>
            </NativeSelect>
            <Button
                variant=ButtonVariant::Ghost
                on_click=move |_| {
                    default_name.set("Grace".to_string());
                    name.set("Grace".to_string());
                }
            >
                "Load profile"
            </Button>
            <Button r#type=ButtonType::Reset variant=ButtonVariant::Outline>
                "Reset"
            </Button>
            <p data-reset-output class="text-sm text-muted-foreground">
                {move || format!("{} / {} / {}", name.get(), bio.get(), role.get())}
            </p>
        </form>
    }
}"#;

const DISABLED: &str = r#"<Input attr:disabled=true value="Cannot edit this" class="max-w-sm" />"#;

const FILE: &str = r#"<Input input_type=InputType::File class="max-w-sm" />"#;

const API: &[ApiEntry] = &[ApiEntry {
    name: "Input",
    description: "A single-line text field.",
    props: &[
        Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Merged over the field's own classes.",
        },
        Prop {
            name: "input_type",
            ty: "Signal<InputType>",
            default: "Text",
            description: "One of: Text, Email, Password, Number, File, Tel, Url, Search, Date.",
        },
        Prop {
            name: "model",
            ty: "Option<RwSignal<String>>",
            default: "None",
            description: "Two-way binding. Pass this or `value`, not both. A reset of the surrounding `<form>` (or the one named by `form`) writes `default_value` into it; see `default_value`.",
        },
        Prop {
            name: "value",
            ty: "Signal<String>",
            default: "\"\"",
            description: "One-way: the field shows it and never writes back.",
        },
        Prop {
            name: "default_value",
            ty: "Signal<String>",
            default: "\"\"",
            description: "What a reset of the form puts back, read at the moment of the reset — so a profile loaded after mount is honoured, as with React's `defaultValue`. Empty means unset, and the current `value` is used instead. Never the `model`'s own earlier contents. The result is written to the field and to `model`.",
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
            name: "name",
            ty: "Signal<String>",
            default: "\"\"",
            description: "The name it is submitted under in a form's data. Empty leaves it out.",
        },
        Prop {
            name: "required",
            ty: "Signal<bool>",
            default: "false",
            description: "The browser refuses to submit the form while it is empty.",
        },
        Prop {
            name: "readonly",
            ty: "Signal<bool>",
            default: "false",
            description: "Focusable and selectable but not editable; still submitted.",
        },
        Prop {
            name: "placeholder",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Hint text shown while empty.",
        },
        Prop {
            name: "autocomplete",
            ty: "Signal<String>",
            default: "\"\"",
            description: "A browser autofill hint such as `\"email\"`. Empty leaves the attribute off.",
        },
        Prop {
            name: "form",
            ty: "Signal<String>",
            default: "\"\"",
            description: "The `id` of a `<form>` it belongs to when it is not inside one. Empty means the one around it.",
        },
        Prop {
            name: "on_input",
            ty: "Option<Callback<String>>",
            default: "None",
            description: "Every edit, with the text as it stands — for reacting to typing without owning the value through `model`.",
        },
        Prop {
            name: "on_change",
            ty: "Option<Callback<String>>",
            default: "None",
            description: "The committed value, when the field loses focus or Enter is pressed.",
        },
    ],
}];

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout title="Input" description="A single-line text field for user input.">
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="A bare field — extra attributes pass through with attr:."
                    code=DEFAULT
                >
                    <Input attr:placeholder="Enter text..." />
                </DemoSection>

                <DemoSection
                    title="Types"
                    description="input_type maps to the native type, so the browser keeps its own keyboard and validation."
                    code=TYPES
                >
                    <div class="flex flex-col gap-3 max-w-sm">
                        <div class="flex flex-col gap-1.5">
                            <Label>"Email"</Label>
                            <Input input_type=InputType::Email attr:placeholder="you@example.com" />
                        </div>
                        <div class="flex flex-col gap-1.5">
                            <Label>"Password"</Label>
                            <Input
                                input_type=InputType::Password
                                attr:placeholder="••••••••"
                            />
                        </div>
                        <div class="flex flex-col gap-1.5">
                            <Label>"Number"</Label>
                            <Input input_type=InputType::Number attr:placeholder="0" />
                        </div>
                    </div>
                </DemoSection>

                <DemoSection
                    title="More types"
                    description="Tel, Url, Search and Date join the first five: a phone keyboard, URL validation, a clear button, a date picker, all the browser's."
                    code=MORE_TYPES
                >
                    <div class="flex flex-col gap-3 max-w-sm">
                        <Input input_type=InputType::Tel attr:placeholder="+44 20 7946 0000" />
                        <Input input_type=InputType::Url attr:placeholder="https://example.com" />
                        <Input input_type=InputType::Search attr:placeholder="Search..." />
                        <Input input_type=InputType::Date />
                    </div>
                </DemoSection>

                <DemoSection
                    title="Controlled"
                    description="Value bound to a signal via the model prop."
                    code=CONTROLLED
                >
                    {
                        let value = RwSignal::new(String::new());
                        view! {
                            <div class="flex flex-col gap-3 max-w-sm">
                                <Input model=value attr:placeholder="Type something..." />
                                <p class="text-sm text-muted-foreground">
                                    "Value: " {move || value.get()}
                                </p>
                            </div>
                        }
                    }
                </DemoSection>

                <DemoSection
                    title="Resetting a form"
                    description="Load profile stands for data that arrives after mount: it moves the default and the field together, and a reset returns to the new one. A reset puts `Input`, `Textarea` and `NativeSelect` back to their `default_value` — read when the reset happens, so a value that arrived late is honoured — and writes it into their `model`, so the signal and the screen agree afterwards."
                    code=RESET
                >
                    {
                        let name = RwSignal::new("Ada".to_string());
                        let default_name = RwSignal::new("Ada".to_string());
                        let bio = RwSignal::new("Mathematician".to_string());
                        let role = RwSignal::new("editor".to_string());
                        view! {
                            <form class="flex max-w-sm flex-col gap-3">
                                <Input model=name default_value=default_name />
                                <Textarea model=bio default_value="Mathematician" rows=2 />
                                <NativeSelect model=role default_value="editor">
                                    <NativeSelectOption value="admin">"Admin"</NativeSelectOption>
                                    <NativeSelectOption value="editor">"Editor"</NativeSelectOption>
                                </NativeSelect>
                                <Button
                                    variant=ButtonVariant::Ghost
                                    on_click=move |_| {
                                        default_name.set("Grace".to_string());
                                        name.set("Grace".to_string());
                                    }
                                >
                                    "Load profile"
                                </Button>
                                <Button r#type=ButtonType::Reset variant=ButtonVariant::Outline>
                                    "Reset"
                                </Button>
                                <p data-reset-output class="text-sm text-muted-foreground">
                                    {move || {
                                        format!("{} / {} / {}", name.get(), bio.get(), role.get())
                                    }}
                                </p>
                            </form>
                        }
                    }
                </DemoSection>

                <DemoSection
                    title="Show and hide"
                    description="input_type takes a signal, so the same field flips between Password and Text without being rebuilt — the caret and what has been typed stay where they are."
                    code=REVEAL
                >
                    {
                        let shown = RwSignal::new(false);
                        let input_type = Signal::derive(move || {
                            if shown.get() { InputType::Text } else { InputType::Password }
                        });
                        view! {
                            <div class="flex max-w-sm gap-2">
                                <Input input_type=input_type attr:placeholder="Password" />
                                <Button
                                    variant=ButtonVariant::Outline
                                    size=ButtonSize::Sm
                                    on_click=move |_| shown.update(|s| *s = !*s)
                                >
                                    {move || if shown.get() { "Hide" } else { "Show" }}
                                </Button>
                            </div>
                        }
                    }
                </DemoSection>

                <DemoSection
                    title="Disabled"
                    description="A disabled field dims, fills its background and refuses the pointer."
                    code=DISABLED
                >
                    <Input attr:disabled=true value="Cannot edit this" class="max-w-sm" />
                </DemoSection>

                <DemoSection
                    title="File"
                    description="The native file control, restyled through file: variants."
                    code=FILE
                >
                    <Input input_type=InputType::File class="max-w-sm" />
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
