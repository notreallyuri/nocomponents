use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::{
    components::{
        checkbox::Checkbox,
        field::{Field, FieldDescription, FieldError, FieldGroup, FieldLabel},
        input::Input,
        native_select::NativeSelect,
        textarea::Textarea,
    },
    utils::types::Orientation,
};

const DEFAULT: &str = r#"<Field>
    <FieldLabel>"Project name"</FieldLabel>
    <Input />
    <FieldDescription>"Shown in the sidebar and in every invite."</FieldDescription>
</Field>"#;

const VALIDATION: &str = r#"let email = RwSignal::new("not-an-address".to_string());
let invalid = Signal::derive(move || !email.get().contains('@'));

view! {
    <Field invalid=invalid>
        <FieldLabel>"Email"</FieldLabel>
        <Input model=email />
        <FieldDescription>"We only use this for billing receipts."</FieldDescription>
        <FieldError>"Enter an email address."</FieldError>
    </Field>
}"#;

const HORIZONTAL: &str = r#"<Field orientation=Orientation::Horizontal>
    <Checkbox checked=notify />
    <FieldLabel>"Email me when a build fails"</FieldLabel>
</Field>"#;

const GROUP: &str = r#"<FieldGroup>
    <Field>
        <FieldLabel>"Display name"</FieldLabel>
        <Input />
    </Field>
    <Field>
        <FieldLabel>"Visibility"</FieldLabel>
        <NativeSelect>
            <option value="private">"Private"</option>
            <option value="team">"Team"</option>
            <option value="public">"Public"</option>
        </NativeSelect>
        <FieldDescription>"Public projects appear on your profile."</FieldDescription>
    </Field>
    <Field>
        <FieldLabel>"Description"</FieldLabel>
        <Textarea />
    </Field>
</FieldGroup>"#;

const DISABLED: &str = r#"<Field disabled=true>
    <FieldLabel>"Workspace slug"</FieldLabel>
    <Input value="acme-corp" />
    <FieldDescription>"Slugs cannot be changed after creation."</FieldDescription>
</Field>"#;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "Field",
        description: "A control with its label, description and error, wired together: the label's `for`, the control's `aria-describedby` and the invalid state all come from here.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the layout classes.",
            },
            Prop {
                name: "orientation",
                ty: "Orientation",
                default: "Orientation::Vertical",
                description: "One of: Horizontal, Vertical.",
            },
            Prop {
                name: "invalid",
                ty: "Signal<bool>",
                default: "false",
                description: "Marks the control invalid and lets the error render.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Disables the control inside it, and greys the parts.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "A label, a control, and optionally a description and an error.",
            },
        ],
    },
    ApiEntry {
        name: "FieldLabel",
        description: "The control's name. Its `for` is filled in, so no id has to be invented.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the label's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The label text.",
            },
        ],
    },
    ApiEntry {
        name: "FieldDescription",
        description: "The hint under the control, announced through `aria-describedby`.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the muted text classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The hint text.",
            },
        ],
    },
    ApiEntry {
        name: "FieldError",
        description: "Shown only while the field is invalid; see `FieldErrorRoot`.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the error's classes.",
            },
            Prop {
                name: "children",
                ty: "ChildrenFn",
                default: "",
                description: "The message. Re-run on each mount, hence `ChildrenFn`.",
            },
        ],
    },
    ApiEntry {
        name: "FieldGroup",
        description: "Stacks fields into a form. Style-only and contextless: unrelated fields should not pretend to be a `<fieldset>`.",
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
                description: "Fields.",
            },
        ],
    },
];

#[component]
pub fn Page() -> impl IntoView {
    let email = RwSignal::new("not-an-address".to_string());
    let notify = RwSignal::new(true);
    let invalid = Signal::derive(move || !email.get().contains('@'));

    view! {
        <DocLayout
            title="Field"
            description="A labelled control with its description and error, wired together."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="The field mints the control's id and the label points at it, so clicking the label focuses the input without anybody naming it."
                    code=DEFAULT
                >
                    <div class="w-full max-w-sm">
                        <Field>
                            <FieldLabel>"Project name"</FieldLabel>
                            <Input />
                            <FieldDescription>
                                "Shown in the sidebar and in every invite."
                            </FieldDescription>
                        </Field>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Validation"
                    description="The error renders only while the field is invalid, and only then does the control's aria-describedby name it. Type an @ to clear it."
                    code=VALIDATION
                >
                    <div class="w-full max-w-sm">
                        <Field invalid=invalid>
                            <FieldLabel>"Email"</FieldLabel>
                            <Input model=email />
                            <FieldDescription>
                                "We only use this for billing receipts."
                            </FieldDescription>
                            <FieldError>"Enter an email address."</FieldError>
                        </Field>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Horizontal"
                    description="Control first, label beside it — the shape a checkbox or switch row wants."
                    code=HORIZONTAL
                >
                    <div class="w-full max-w-sm">
                        <Field orientation=Orientation::Horizontal>
                            <Checkbox checked=notify />
                            <FieldLabel>"Email me when a build fails"</FieldLabel>
                        </Field>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Group"
                    description="FieldGroup decides the spacing between fields in one place. Input, Textarea and NativeSelect all pick up the field around them."
                    code=GROUP
                >
                    <div class="w-full max-w-sm">
                        <FieldGroup>
                            <Field>
                                <FieldLabel>"Display name"</FieldLabel>
                                <Input />
                            </Field>
                            <Field>
                                <FieldLabel>"Visibility"</FieldLabel>
                                <NativeSelect>
                                    <option value="private">"Private"</option>
                                    <option value="team">"Team"</option>
                                    <option value="public">"Public"</option>
                                </NativeSelect>
                                <FieldDescription>
                                    "Public projects appear on your profile."
                                </FieldDescription>
                            </Field>
                            <Field>
                                <FieldLabel>"Description"</FieldLabel>
                                <Textarea />
                            </Field>
                        </FieldGroup>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Disabled"
                    description="Disabling the field disables the control inside it, so the state is set in one place."
                    code=DISABLED
                >
                    <div class="w-full max-w-sm">
                        <Field disabled=true>
                            <FieldLabel>"Workspace slug"</FieldLabel>
                            <Input value="acme-corp" />
                            <FieldDescription>
                                "Slugs cannot be changed after creation."
                            </FieldDescription>
                        </Field>
                    </div>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
