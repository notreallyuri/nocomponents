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
    let last = RwSignal::new("nothing yet".to_string());
    view! {
        <div class="flex flex-col gap-1.5 w-48">
            <Label>"Theme"</Label>
            <Select
                default_value="dark"
                on_value_change={move |value: Option<String>| last.set(value.unwrap_or_default())}
            >
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
            <p class="text-sm text-muted-foreground">"Last change: " {move || last.get()}</p>
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
                ty: "Option<RwSignal<Option<String>>>",
                default: "None",
                description: "The chosen value, held by the caller. Empty until something is picked; omit it and the root owns the signal.",
            },
            Prop {
                name: "default_value",
                ty: "Option<String>",
                default: "None",
                description: "The value to start with, as in `default_value=\"apple\"`. Ignored when `value` is given.",
            },
            Prop {
                name: "on_value_change",
                ty: "Option<Callback<Option<String>>>",
                default: "None",
                description: "Called with the new value whenever it changes, including when the signal passed as `value` is written from outside.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the wrapper's classes.",
            },
            Prop {
                name: "name",
                ty: "Signal<String>",
                default: "\"\"",
                description: "The name the choice is submitted under, written on the hidden native `<select>`. Empty submits nothing.",
            },
            Prop {
                name: "required",
                ty: "Signal<bool>",
                default: "false",
                description: "The browser refuses to submit the form while nothing is chosen.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Disables the trigger, and keeps the choice out of a form's data.",
            },
            Prop {
                name: "form",
                ty: "Signal<String>",
                default: "\"\"",
                description: "The `id` of a `<form>` it belongs to when it is not inside one. Empty means the one around it.",
            },
            Prop {
                name: "open",
                ty: "Option<RwSignal<bool>>",
                default: "None",
                description: "Pass one to drive the list from outside or to watch it; omit it and the root owns the signal.",
            },
            Prop {
                name: "default_open",
                ty: "bool",
                default: "false",
                description: "Whether the list starts open. Ignored when `open` is given.",
            },
            Prop {
                name: "on_open_change",
                ty: "Option<Callback<bool>>",
                default: "None",
                description: "Called with the new value whenever the list opens or closes, including when the signal passed as `open` is written from outside.",
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
            Prop {
                name: "icon",
                ty: "Option<ViewFn>",
                default: "ChevronDown",
                description: "The glyph, in place of the chevron. Any view — an `Icon`, an SVG of your own; see Icons.",
            },
        ],
    },
    ApiEntry {
        name: "SelectValue",
        description: "The current value, or a placeholder when there is none. Renders the value itself, so an item's value and its text should read the same.",
        props: &[Prop {
            name: "placeholder",
            ty: "Signal<String>",
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
                name: "mobile_sheet",
                ty: "Signal<bool>",
                default: "false",
                description: "Below the `md` breakpoint (768px), present the list as a bottom sheet over a scrim instead of a popup under the trigger: no positioning, `data-presentation=\"sheet\"`. At `md` and up nothing changes, and it is written as `\"floating\"`. Escape, a tap on the scrim and choosing an item still close it.",
            },
            Prop {
                name: "sheet_title",
                ty: "Signal<String>",
                default: "\"\"",
                description: "The title row under the sheet's grabber. Empty means a grabber and no row. Ignored when the content is not a sheet.",
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
                name: "label",
                ty: "Signal<String>",
                default: "\"\"",
                description: "What `SelectValue` shows once this option is chosen. Empty means the option's own text. Labels are kept after the list unmounts, and a value chosen before the list has ever opened is resolved by building the content once off-screen.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Skipped by the arrow keys and typeahead, refuses a click, and is marked `data-disabled` and `aria-disabled`.",
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
            Prop {
                name: "icon",
                ty: "Option<ViewFn>",
                default: "Check",
                description: "The glyph, in place of the check mark on the chosen item. Any view — an `Icon`, an SVG of your own; see Icons.",
            },
        ],
    },
];

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout
            title="Select"
            description="Displays a list of options for the user to pick from. Set `mobile_sheet` and it becomes a bottom sheet on phones; the Dropdown Menu page shows it."
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
                    description="default_value preselects an item without a signal to hold, and on_value_change hears each pick."
                    code=WITH_LABEL_AND_DEFAULT_VALUE
                >
                    {
                        let last = RwSignal::new("nothing yet".to_string());
                        view! {
                            <div class="flex flex-col gap-1.5 w-48">
                                <Label>"Theme"</Label>
                                <Select
                                    default_value="dark"
                                    on_value_change={move |value: Option<String>| last.set(value.unwrap_or_default())}
                                >
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
                                <p class="text-sm text-muted-foreground">
                                    "Last change: " {move || last.get()}
                                </p>
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
