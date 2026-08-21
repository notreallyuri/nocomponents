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
        button::{Button, ButtonSize, ButtonVariant},
        combobox::{Combobox, ComboboxContent, ComboboxItem, ComboboxTrigger, ComboboxValue},
        command::{CommandEmpty, CommandGroup, CommandInput, CommandList},
        label::Label,
    },
    utils::types::Align,
};

const DEFAULT: &str = r#"<Combobox>
    <ComboboxTrigger>
        <ComboboxValue placeholder="Select a framework…" />
    </ComboboxTrigger>
    <ComboboxContent>
        <CommandInput placeholder="Search framework…" />
        <CommandList>
            <CommandEmpty>"No framework found."</CommandEmpty>
            <ComboboxItem value="Next.js">"Next.js"</ComboboxItem>
            <ComboboxItem value="SvelteKit">"SvelteKit"</ComboboxItem>
            <ComboboxItem value="Nuxt.js">"Nuxt.js"</ComboboxItem>
            <ComboboxItem value="Remix">"Remix"</ComboboxItem>
            <ComboboxItem value="Astro">"Astro"</ComboboxItem>
        </CommandList>
    </ComboboxContent>
</Combobox>"#;

const LABELS: &str = r#"// The value is what the form sends; the label is what the button reads.
<ComboboxItem value="us-east-1" label="US East (N. Virginia)" keywords="virginia america">
    <div class="flex flex-col">
        <span>"US East (N. Virginia)"</span>
        <span class="text-xs text-muted-foreground">"us-east-1"</span>
    </div>
</ComboboxItem>"#;

const CONTROLLED: &str = r#"{
    let value = RwSignal::new(None::<String>);

    view! {
        <Combobox value=value>
            <ComboboxTrigger class="w-44" />
            <ComboboxContent align=Align::End>
                <CommandInput placeholder="Search status…" />
                <CommandList>
                    <CommandEmpty>"No status found."</CommandEmpty>
                    <ComboboxItem value="Backlog">"Backlog"</ComboboxItem>
                    <ComboboxItem value="In progress">"In progress"</ComboboxItem>
                    <ComboboxItem value="Done">"Done"</ComboboxItem>
                </CommandList>
            </ComboboxContent>
        </Combobox>
        <Button
            variant=ButtonVariant::Outline
            size=ButtonSize::Sm
            on:click=move |_| value.set(None)
        >
            "Reset"
        </Button>
    }
}"#;

const GROUPS: &str = r#"<ComboboxContent class="w-72">
    <CommandInput placeholder="Search…" />
    <CommandList>
        <CommandEmpty>"Nothing found."</CommandEmpty>
        <CommandGroup heading="Fruit">
            <ComboboxItem value="Apple" keywords="pome">"Apple"</ComboboxItem>
            <ComboboxItem value="Banana">"Banana"</ComboboxItem>
        </CommandGroup>
        <CommandGroup heading="Vegetable">
            <ComboboxItem value="Carrot">"Carrot"</ComboboxItem>
            <ComboboxItem value="Spinach" disabled=true>"Spinach"</ComboboxItem>
        </CommandGroup>
    </CommandList>
</ComboboxContent>"#;

const REGIONS: &[(&str, &str, &str)] = &[
    ("us-east-1", "US East (N. Virginia)", "virginia america"),
    ("us-west-2", "US West (Oregon)", "oregon america"),
    ("eu-west-1", "Europe (Ireland)", "ireland dublin"),
    ("eu-central-1", "Europe (Frankfurt)", "germany"),
    ("sa-east-1", "South America (São Paulo)", "brazil brasil"),
];

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "Combobox",
        description: "The root. Owns the chosen value and the floating layer; everything below reads it from context.",
        props: &[
            Prop {
                name: "value",
                ty: "RwSignal<Option<String>>",
                default: "None",
                description: "The chosen value. Pass one to read the selection or to preset it.",
            },
            Prop {
                name: "display_value",
                ty: "RwSignal<Option<String>>",
                default: "None",
                description: "What the trigger shows for that value, when the two differ. An item sets it as it is chosen; preset it alongside `value` for a selection that arrives from outside.",
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
                description: "A trigger and a content.",
            },
        ],
    },
    ApiEntry {
        name: "ComboboxTrigger",
        description: "The button. An outline `Button` carrying the current value and a chevron, unless `render` says otherwise.",
        props: &[
            Prop {
                name: "size",
                ty: "ButtonSize",
                default: "Default",
                description: "Forwarded to the underlying `Button`.",
            },
            Prop {
                name: "variant",
                ty: "ButtonVariant",
                default: "Outline",
                description: "Forwarded to the underlying `Button`.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the button's classes; where to change the `w-56` default width.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Rendered as the `disabled` attribute.",
            },
            Prop {
                name: "render",
                ty: "Option<Callback<(AnyNodeRef, Callback<MouseEvent>), AnyView>>",
                default: "None",
                description: "Render the trigger as something else. Forward the node ref — it is what the list is positioned against.",
            },
            Prop {
                name: "children",
                ty: "Option<ChildrenFn>",
                default: "None",
                description: "The button's contents. A `ComboboxValue` when left out.",
            },
        ],
    },
    ApiEntry {
        name: "ComboboxValue",
        description: "The chosen label, or the placeholder. Carries `data-placeholder` while nothing is chosen.",
        props: &[
            Prop {
                name: "placeholder",
                ty: "Signal<String>",
                default: "\"Select…\"",
                description: "Shown while nothing is chosen.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the label's classes.",
            },
        ],
    },
    ApiEntry {
        name: "ComboboxContent",
        description: "The floating list: a command palette in a popover, at least as wide as the trigger. Portalled, and mounted only while open.",
        props: &[
            Prop {
                name: "side",
                ty: "Side",
                default: "Side::Bottom",
                description: "One of: Left, Right, Bottom, Top.",
            },
            Prop {
                name: "align",
                ty: "Align",
                default: "Start",
                description: "One of: Start, Center, End.",
            },
            Prop {
                name: "side_offset",
                ty: "SideOffset",
                default: "SideOffset(4.0)",
                description: "Distance from the trigger, in pixels.",
            },
            Prop {
                name: "should_filter",
                ty: "bool",
                default: "true",
                description: "Off when the list arrives already narrowed, as an async search does. Forwarded to the command root inside.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the surface's classes; where to set a width wider than the trigger's.",
            },
            Prop {
                name: "children",
                ty: "ChildrenFn",
                default: "",
                description: "`CommandInput`, `CommandList` and the items. Re-run on each mount, hence `ChildrenFn`.",
            },
        ],
    },
    ApiEntry {
        name: "ComboboxItem",
        description: "One option. A command item that writes the combobox's value and closes the list, and shows a check while it is the chosen one.",
        props: &[
            Prop {
                name: "value",
                ty: "String",
                default: "",
                description: "What is written into the combobox's value, and what the search matches against.",
            },
            Prop {
                name: "label",
                ty: "Signal<String>",
                default: "\"\"",
                description: "What the trigger reads once this item is chosen. Empty means the value itself.",
            },
            Prop {
                name: "keywords",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Extra words the item can be found by but which are not shown.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Skipped by the arrows and unclickable.",
            },
            Prop {
                name: "on_select",
                ty: "Option<Callback<String>>",
                default: "None",
                description: "Run after the value has been written, with the item's value.",
            },
            Prop {
                name: "deselectable",
                ty: "bool",
                default: "true",
                description: "Whether choosing the item again clears the selection, as a filter chip does.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the item's classes.",
            },
            Prop {
                name: "children",
                ty: "ChildrenFn",
                default: "",
                description: "The item's contents. Re-run on each mount, hence `ChildrenFn`.",
            },
        ],
    },
];

#[component]
pub fn Page() -> impl IntoView {
    let framework = RwSignal::new(None::<String>);

    let region = RwSignal::new(Some("us-west-2".to_string()));
    let region_label = RwSignal::new(Some("US West (Oregon)".to_string()));

    let status = RwSignal::new(None::<String>);

    view! {
        <DocLayout
            title="Combobox"
            description="A button that opens a searchable list and comes back with one value."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="A command palette in a popover: the search, the filtering and the arrow keys are the palette's, so the parts inside are `CommandInput`, `CommandList` and `CommandEmpty`. The list opens at least as wide as the button, focus goes straight to the search box, and choosing the same item again clears the selection."
                    code=DEFAULT
                >
                    <div class="flex flex-wrap items-center gap-3">
                        <Combobox value=framework>
                            <ComboboxTrigger>
                                <ComboboxValue placeholder="Select a framework…" />
                            </ComboboxTrigger>
                            <ComboboxContent>
                                <CommandInput placeholder="Search framework…" />
                                <CommandList>
                                    <CommandEmpty>"No framework found."</CommandEmpty>
                                    <ComboboxItem value="Next.js">"Next.js"</ComboboxItem>
                                    <ComboboxItem value="SvelteKit">"SvelteKit"</ComboboxItem>
                                    <ComboboxItem value="Nuxt.js">"Nuxt.js"</ComboboxItem>
                                    <ComboboxItem value="Remix">"Remix"</ComboboxItem>
                                    <ComboboxItem value="Astro">"Astro"</ComboboxItem>
                                </CommandList>
                            </ComboboxContent>
                        </Combobox>
                        <span class="text-sm text-muted-foreground">
                            {move || match framework.get() {
                                Some(value) => format!("Chose: {value}"),
                                None => "Nothing chosen.".to_string(),
                            }}
                        </span>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Labels"
                    description="An item's value is what the selection is reported as; its label is what the button reads. Give an item a `label` whenever the two differ, and preset `display_value` beside `value` for a selection that arrives from outside — the items that know the labels do not exist until the list first opens."
                    code=LABELS
                >
                    <div class="flex flex-wrap items-center gap-3">
                        <Combobox value=region display_value=region_label>
                            <ComboboxTrigger />
                            <ComboboxContent class="w-72">
                                <CommandInput placeholder="Search regions…" />
                                <CommandList>
                                    <CommandEmpty>"No region found."</CommandEmpty>
                                    {REGIONS
                                        .iter()
                                        .map(|(value, label, keywords)| {
                                            view! {
                                                <ComboboxItem value=*value label=*label keywords=*keywords>
                                                    <div class="flex flex-col">
                                                        <span>{*label}</span>
                                                        <span class="text-xs text-muted-foreground">{*value}</span>
                                                    </div>
                                                </ComboboxItem>
                                            }
                                        })
                                        .collect_view()}
                                </CommandList>
                            </ComboboxContent>
                        </Combobox>
                        <span class="font-mono text-sm text-muted-foreground">
                            {move || region.get().unwrap_or_else(|| "—".to_string())}
                        </span>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Groups"
                    description="Groups, keywords and disabled items are the palette's, unchanged. A group whose items have all been filtered away takes itself out of the list."
                    code=GROUPS
                >
                    <div class="flex flex-col gap-2">
                        <Label>"Produce"</Label>
                        <Combobox>
                            <ComboboxTrigger>
                                <ComboboxValue placeholder="Pick something…" />
                            </ComboboxTrigger>
                            <ComboboxContent class="w-72">
                                <CommandInput placeholder="Search…" />
                                <CommandList>
                                    <CommandEmpty>"Nothing found."</CommandEmpty>
                                    <CommandGroup heading="Fruit">
                                        <ComboboxItem value="Apple" keywords="pome">
                                            "Apple"
                                        </ComboboxItem>
                                        <ComboboxItem value="Banana">"Banana"</ComboboxItem>
                                        <ComboboxItem value="Cherry" keywords="stone fruit">
                                            "Cherry"
                                        </ComboboxItem>
                                    </CommandGroup>
                                    <CommandGroup heading="Vegetable">
                                        <ComboboxItem value="Carrot">"Carrot"</ComboboxItem>
                                        <ComboboxItem value="Spinach" disabled=true>
                                            "Spinach"
                                        </ComboboxItem>
                                    </CommandGroup>
                                </CommandList>
                            </ComboboxContent>
                        </Combobox>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Controlled"
                    description="The value is a signal like any other, so a form can read it, reset it or drive it from elsewhere. `align=Align::End` hangs the list off the button's right edge instead of its left."
                    code=CONTROLLED
                >
                    <div class="flex flex-wrap items-center gap-3">
                        <Combobox value=status>
                            <ComboboxTrigger class="w-44" />
                            <ComboboxContent align=Align::End>
                                <CommandInput placeholder="Search status…" />
                                <CommandList>
                                    <CommandEmpty>"No status found."</CommandEmpty>
                                    <ComboboxItem value="Backlog">"Backlog"</ComboboxItem>
                                    <ComboboxItem value="In progress">"In progress"</ComboboxItem>
                                    <ComboboxItem value="Done">"Done"</ComboboxItem>
                                </CommandList>
                            </ComboboxContent>
                        </Combobox>
                        <Button
                            variant=ButtonVariant::Outline
                            size=ButtonSize::Sm
                            on:click=move |_| status.set(None)
                        >
                            "Reset"
                        </Button>
                    </div>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
