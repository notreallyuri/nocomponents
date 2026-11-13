use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::components::{
    label::Label,
    native_select::{NativeSelect, NativeSelectOptGroup, NativeSelectOption, NativeSelectSize},
};

const DEFAULT: &str = r#"<NativeSelect class="max-w-56">
    <option value="apple">"Apple"</option>
    <option value="banana">"Banana"</option>
    <option value="cherry">"Cherry"</option>
</NativeSelect>"#;

const WITH_LABEL: &str = r#"<Label attr:r#for="timezone">"Timezone"</Label>
<NativeSelect attr:id="timezone" class="max-w-56">
    <option value="utc">"UTC"</option>
    <option value="cet">"Central European"</option>
    <option value="jst">"Japan Standard"</option>
</NativeSelect>"#;

const GROUPED: &str = r#"<NativeSelect class="max-w-56">
    <optgroup label="Fruit">
        <option value="apple">"Apple"</option>
        <option value="pear">"Pear"</option>
    </optgroup>
    <optgroup label="Vegetables">
        <option value="leek">"Leek"</option>
        <option value="fennel">"Fennel"</option>
    </optgroup>
</NativeSelect>"#;

const SIZES: &str = r#"<NativeSelect size=NativeSelectSize::Sm class="max-w-56">
    <option>"Small"</option>
</NativeSelect>
<NativeSelect class="max-w-56">
    <option>"Default"</option>
</NativeSelect>"#;

const PARTS: &str = r#"{
    let picked = RwSignal::new(String::new());
    view! {
        <div class="flex max-w-56 flex-col gap-3">
            <NativeSelect on_change=move |value: String| picked.set(value)>
                <NativeSelectOption value="">"Choose a produce"</NativeSelectOption>
                <NativeSelectOptGroup label="Fruit">
                    <NativeSelectOption value="apple">"Apple"</NativeSelectOption>
                    <NativeSelectOption value="pear" disabled=true>
                        "Pear (out of season)"
                    </NativeSelectOption>
                </NativeSelectOptGroup>
                <NativeSelectOptGroup label="Vegetables" disabled=true>
                    <NativeSelectOption value="leek">"Leek"</NativeSelectOption>
                </NativeSelectOptGroup>
            </NativeSelect>
            <p class="text-sm text-muted-foreground">
                "Changed to: " {move || picked.get()}
            </p>
        </div>
    }
}"#;

const DISABLED: &str = r#"<NativeSelect attr:disabled=true class="max-w-56">
    <option>"Locked"</option>
</NativeSelect>"#;

const CONTROLLED: &str = r#"<NativeSelect model=role class="max-w-56">
    <option value="admin">"Admin"</option>
    <option value="editor">"Editor"</option>
    <option value="viewer">"Viewer"</option>
</NativeSelect>
<p class="text-sm text-muted-foreground">"Selected: " {move || role.get()}</p>"#;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "NativeSelect",
        description: "The browser's own `<select>`, bound to a signal. Opens the platform picker on a phone, and keeps `optgroup`.",
        props: &[
            Prop {
                name: "size",
                ty: "Signal<NativeSelectSize>",
                default: "Default",
                description: "One of: Sm, Default.",
            },
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
                description: "Two-way binding. Omit it and pass `value` for a one-way one. A reset of the surrounding `<form>` (or the one named by `form`) writes `default_value` into it; see `default_value`.",
            },
            Prop {
                name: "default_value",
                ty: "Signal<String>",
                default: "\"\"",
                description: "What a reset of the form puts back, read at the moment of the reset — so a value that arrived after mount is honoured. Empty means unset, and the current `value` is used instead. Never the `model`'s own earlier contents. The result is written to the control and to `model`.",
            },
            Prop {
                name: "value",
                ty: "Signal<String>",
                default: "\"\"",
                description: "One-way: the control shows it and never writes back.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Stops it taking pointer events and marks it disabled.",
            },
            Prop {
                name: "id",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Overridden by an enclosing `Field`, which mints its own.",
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
                description: "The browser refuses to submit the form while the selected option's value is empty.",
            },
            Prop {
                name: "form",
                ty: "Signal<String>",
                default: "\"\"",
                description: "The `id` of a `<form>` it belongs to when it is not inside one. Empty means the one around it.",
            },
            Prop {
                name: "on_blur",
                ty: "Option<Callback<FocusEvent>>",
                default: "None",
                description: "Run when the control loses focus. A prop rather than your own `on:blur` because the chevron needs a positioned wrapper around the `<select>`, and that wrapper is what a handler written at the call site attaches to — where it never fires, since `blur` does not bubble. An enclosing `Field` needs none of this: it listens for `focusout`, which does.",
            },
            Prop {
                name: "on_change",
                ty: "Option<Callback<String>>",
                default: "None",
                description: "Called with the chosen option's value whenever the reader changes it, whether or not `model` is bound.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "`NativeSelectOption` and `NativeSelectOptGroup` parts, or `<option>` and `<optgroup>` elements written out as they are.",
            },
        ],
    },
    ApiEntry {
        name: "NativeSelectOption",
        description: "An `<option>`. It carries no classes by default (the select paints the popup's background), so `class` is only yours.",
        props: &[
            Prop {
                name: "value",
                ty: "Signal<String>",
                default: "\"\"",
                description: "What the option submits and what `model` holds while it is chosen. Empty falls back to the option's text.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Shown but not choosable.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged onto the element.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The option's text.",
            },
        ],
    },
    ApiEntry {
        name: "NativeSelectOptGroup",
        description: "An `<optgroup>`: a heading the browser draws over a run of options.",
        props: &[
            Prop {
                name: "label",
                ty: "Signal<String>",
                default: "\"\"",
                description: "The heading.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Disables every option inside it.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged onto the element.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "`NativeSelectOption`s.",
            },
        ],
    },
];

#[component]
pub fn Page() -> impl IntoView {
    let role = RwSignal::new("editor".to_string());

    view! {
        <DocLayout
            title="Native Select"
            description="The browser's own select element, styled to match the rest."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="A real <select>: it costs nothing, works everywhere, and opens the platform picker on a phone. Reach for Select instead when the options need to look like anything other than a native menu."
                    code=DEFAULT
                >
                    <NativeSelect class="max-w-56">
                        <option value="apple">"Apple"</option>
                        <option value="banana">"Banana"</option>
                        <option value="cherry">"Cherry"</option>
                    </NativeSelect>
                </DemoSection>

                <DemoSection
                    title="With a label"
                    description="Point for at the control's id, as with any other field."
                    code=WITH_LABEL
                >
                    <div class="flex max-w-56 flex-col gap-1.5">
                        <Label attr:r#for="timezone">"Timezone"</Label>
                        <NativeSelect attr:id="timezone">
                            <option value="utc">"UTC"</option>
                            <option value="cet">"Central European"</option>
                            <option value="jst">"Japan Standard"</option>
                        </NativeSelect>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Grouped options"
                    description="optgroup works, which is most of the reason to use the native control."
                    code=GROUPED
                >
                    <NativeSelect class="max-w-56">
                        <optgroup label="Fruit">
                            <option value="apple">"Apple"</option>
                            <option value="pear">"Pear"</option>
                        </optgroup>
                        <optgroup label="Vegetables">
                            <option value="leek">"Leek"</option>
                            <option value="fennel">"Fennel"</option>
                        </optgroup>
                    </NativeSelect>
                </DemoSection>

                <DemoSection
                    title="Option parts"
                    description="NativeSelectOption and NativeSelectOptGroup are the same elements with props instead of attributes; on_change hears every pick, with or without a model."
                    code=PARTS
                >
                    {
                        let picked = RwSignal::new(String::new());
                        view! {
                            <div class="flex max-w-56 flex-col gap-3">
                                <NativeSelect on_change=move |value: String| picked.set(value)>
                                    <NativeSelectOption value="">
                                        "Choose a produce"
                                    </NativeSelectOption>
                                    <NativeSelectOptGroup label="Fruit">
                                        <NativeSelectOption value="apple">
                                            "Apple"
                                        </NativeSelectOption>
                                        <NativeSelectOption value="pear" disabled=true>
                                            "Pear (out of season)"
                                        </NativeSelectOption>
                                    </NativeSelectOptGroup>
                                    <NativeSelectOptGroup label="Vegetables" disabled=true>
                                        <NativeSelectOption value="leek">"Leek"</NativeSelectOption>
                                    </NativeSelectOptGroup>
                                </NativeSelect>
                                <p class="text-sm text-muted-foreground">
                                    "Changed to: " {move || picked.get()}
                                </p>
                            </div>
                        }
                    }
                </DemoSection>

                <DemoSection
                    title="Sizes"
                    description="Two heights, matching the small and default inputs."
                    code=SIZES
                >
                    <div class="flex flex-col gap-3">
                        <NativeSelect size=NativeSelectSize::Sm class="max-w-56">
                            <option>"Small"</option>
                        </NativeSelect>
                        <NativeSelect class="max-w-56">
                            <option>"Default"</option>
                        </NativeSelect>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Disabled"
                    description="A disabled select dims and fills its background, like a disabled input."
                    code=DISABLED
                >
                    <NativeSelect attr:disabled=true class="max-w-56">
                        <option>"Locked"</option>
                    </NativeSelect>
                </DemoSection>

                <DemoSection
                    title="Controlled"
                    description="model binds the value both ways; pass value on its own for a one-way binding."
                    code=CONTROLLED
                >
                    <div class="flex max-w-56 flex-col gap-3">
                        <NativeSelect model=role>
                            <option value="admin">"Admin"</option>
                            <option value="editor">"Editor"</option>
                            <option value="viewer">"Viewer"</option>
                        </NativeSelect>
                        <p class="text-sm text-muted-foreground">
                            "Selected: " {move || role.get()}
                        </p>
                    </div>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
