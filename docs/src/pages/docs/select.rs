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
    select::{Select, SelectContent, SelectItem, SelectPortal, SelectTrigger, SelectValue},
};

const DEFAULT: &str = r#"{
    let value = RwSignal::new(None::<String>);
    view! {
        <div class="w-48">
            <Select value=value>
                <SelectTrigger>
                    <SelectValue placeholder="Select a fruit..." />
                </SelectTrigger>
                <SelectPortal>
                    <SelectContent>
                        <SelectItem value="apple">"Apple"</SelectItem>
                        <SelectItem value="banana">"Banana"</SelectItem>
                        <SelectItem value="cherry">"Cherry"</SelectItem>
                        <SelectItem value="durian">"Durian"</SelectItem>
                    </SelectContent>
                </SelectPortal>
            </Select>
        </div>
    }
}"#;

const WITH_LABEL_AND_DEFAULT_VALUE: &str = r#"{
    let value = RwSignal::new(Some("dark".to_string()));
    view! {
        <div class="flex flex-col gap-1.5 w-48">
            <Label>"Theme"</Label>
            <Select value=value>
                <SelectTrigger>
                    <SelectValue placeholder="Pick a theme..." />
                </SelectTrigger>
                <SelectPortal>
                    <SelectContent>
                        <SelectItem value="light">"Light"</SelectItem>
                        <SelectItem value="dark">"Dark"</SelectItem>
                        <SelectItem value="system">"System"</SelectItem>
                    </SelectContent>
                </SelectPortal>
            </Select>
        </div>
    }
}"#;

const CONTROLLED: &str = r#"{
    let value = RwSignal::new(None::<String>);
    view! {
        <div class="flex flex-col gap-3 w-48">
            <Select value=value>
                <SelectTrigger>
                    <SelectValue placeholder="Select a role..." />
                </SelectTrigger>
                <SelectPortal>
                    <SelectContent>
                        <SelectItem value="admin">"Admin"</SelectItem>
                        <SelectItem value="editor">"Editor"</SelectItem>
                        <SelectItem value="viewer">"Viewer"</SelectItem>
                    </SelectContent>
                </SelectPortal>
            </Select>
            <p class="text-sm text-muted-foreground">
                "Selected: "
                {move || value.get().unwrap_or_else(|| "none".to_string())}
            </p>
        </div>
    }
}"#;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "Select",
        description: "A listbox built out of divs, for options that need to look like more than a native menu.",
        props: &[
            Prop {
                name: "value",
                ty: "RwSignal<Option<String>>",
                default: "None",
                description: "The chosen value. Empty until something is picked.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the wrapper's classes.",
            },
            Prop {
                name: "children",
                ty: "ChildrenFn",
                default: "",
                description: "A trigger and a portal.",
            },
        ],
    },
    ApiEntry {
        name: "SelectTrigger",
        description: "The closed control, showing the current value.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the trigger's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Usually a `SelectValue`.",
            },
        ],
    },
    ApiEntry {
        name: "SelectValue",
        description: "The current value, or a placeholder when there is none. Renders the value itself, so an item's value and its text should read the same.",
        props: &[Prop {
            name: "placeholder",
            ty: "String",
            default: "",
            description: "Shown while nothing is chosen.",
        }],
    },
    ApiEntry {
        name: "SelectPortal",
        description: "Renders the listbox at the end of the document, so it escapes any clipping ancestor.",
        props: &[Prop {
            name: "children",
            ty: "ChildrenFn",
            default: "",
            description: "The listbox. Re-run on each mount, hence `ChildrenFn`.",
        }],
    },
    ApiEntry {
        name: "SelectContent",
        description: "The listbox, positioned under the trigger and matched to its width.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the listbox's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Items.",
            },
        ],
    },
    ApiEntry {
        name: "SelectItem",
        description: "One option. Enter, Space or a click commits it and closes the listbox.",
        props: &[
            Prop {
                name: "value",
                ty: "String",
                default: "",
                description: "What committing this option sets the value to.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the item's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The option's text.",
            },
        ],
    },
];

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout
            title="Select"
            description="Displays a list of options for the user to pick from."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="The value signal starts as None, so the trigger shows its placeholder."
                    code=DEFAULT
                >
                    {
                        let value = RwSignal::new(None::<String>);
                        view! {
                            <div class="w-48">
                                <Select value=value>
                                    <SelectTrigger>
                                        <SelectValue placeholder="Select a fruit..." />
                                    </SelectTrigger>
                                    <SelectPortal>
                                        <SelectContent>
                                            <SelectItem value="apple">"Apple"</SelectItem>
                                            <SelectItem value="banana">"Banana"</SelectItem>
                                            <SelectItem value="cherry">"Cherry"</SelectItem>
                                            <SelectItem value="durian">"Durian"</SelectItem>
                                        </SelectContent>
                                    </SelectPortal>
                                </Select>
                            </div>
                        }
                    }
                </DemoSection>

                <DemoSection
                    title="With label and default value"
                    description="Seed the signal with a value to preselect an item."
                    code=WITH_LABEL_AND_DEFAULT_VALUE
                >
                    {
                        let value = RwSignal::new(Some("dark".to_string()));
                        view! {
                            <div class="flex flex-col gap-1.5 w-48">
                                <Label>"Theme"</Label>
                                <Select value=value>
                                    <SelectTrigger>
                                        <SelectValue placeholder="Pick a theme..." />
                                    </SelectTrigger>
                                    <SelectPortal>
                                        <SelectContent>
                                            <SelectItem value="light">"Light"</SelectItem>
                                            <SelectItem value="dark">"Dark"</SelectItem>
                                            <SelectItem value="system">"System"</SelectItem>
                                        </SelectContent>
                                    </SelectPortal>
                                </Select>
                            </div>
                        }
                    }
                </DemoSection>

                <DemoSection
                    title="Controlled"
                    description="The selected value is reflected outside the component."
                    code=CONTROLLED
                >
                    {
                        let value = RwSignal::new(None::<String>);
                        view! {
                            <div class="flex flex-col gap-3 w-48">
                                <Select value=value>
                                    <SelectTrigger>
                                        <SelectValue placeholder="Select a role..." />
                                    </SelectTrigger>
                                    <SelectPortal>
                                        <SelectContent>
                                            <SelectItem value="admin">"Admin"</SelectItem>
                                            <SelectItem value="editor">"Editor"</SelectItem>
                                            <SelectItem value="viewer">"Viewer"</SelectItem>
                                        </SelectContent>
                                    </SelectPortal>
                                </Select>
                                <p class="text-sm text-muted-foreground">
                                    "Selected: "
                                    {move || value.get().unwrap_or_else(|| "none".to_string())}
                                </p>
                            </div>
                        }
                    }
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
