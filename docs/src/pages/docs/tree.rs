//! The tree page: nesting, the keys, and what the sidebar looks like built out of it.

use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::components::tree::{Tree, TreeGroup, TreeItem, TreeRow};

const DEFAULT: &str = r#"<Tree label="Project">
    <TreeItem default_open=true>
        <TreeRow>"src"</TreeRow>
        <TreeGroup>
            <TreeItem>
                <TreeRow>"components"</TreeRow>
                <TreeGroup>
                    <TreeItem>
                        <TreeRow>"button.rs"</TreeRow>
                    </TreeItem>
                </TreeGroup>
            </TreeItem>
            <TreeItem>
                <TreeRow>"lib.rs"</TreeRow>
            </TreeItem>
        </TreeGroup>
    </TreeItem>
    <TreeItem>
        <TreeRow>"Cargo.toml"</TreeRow>
    </TreeItem>
</Tree>"#;

const SELECTION: &str = r#"let selected = RwSignal::new("lib.rs".to_string());

<TreeRow
    active=Signal::derive(move || selected.get() == "lib.rs")
    on_select=Callback::new(move |_| selected.set("lib.rs".to_string()))
>
    "lib.rs"
</TreeRow>"#;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "Tree",
        description: "The tree itself: one tab stop, and role=\"tree\".",
        props: &[
            Prop {
                name: "label",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Names the tree for assistive tech. A tree with no label is a list of rows nobody can place.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the container's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The top-level items.",
            },
        ],
    },
    ApiEntry {
        name: "TreeItem",
        description: "One node. It owns its open state, and is the element the arrow keys land on.",
        props: &[
            Prop {
                name: "default_open",
                ty: "bool",
                default: "false",
                description: "Open to begin with. Uncontrolled — the item owns it from there.",
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
                description: "A TreeRow, and a TreeGroup if it has children.",
            },
        ],
    },
    ApiEntry {
        name: "TreeRow",
        description: "The part you see and click. Indented by its own depth, and carrying the twisty when the item has children.",
        props: &[
            Prop {
                name: "active",
                ty: "Signal<bool>",
                default: "false",
                description: "Draws the row as the current one. Selection is the caller's to keep.",
            },
            Prop {
                name: "on_select",
                ty: "Option<Callback<ev::MouseEvent>>",
                default: "None",
                description: "Run on click and on Enter or Space. A branch also toggles.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the row's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The row's label.",
            },
        ],
    },
    ApiEntry {
        name: "TreeGroup",
        description: "An item's children. Its presence is what makes the item a branch, and a closed one renders nothing at all.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the group's classes.",
            },
            Prop {
                name: "children",
                ty: "ChildrenFn",
                default: "",
                description: "Nested TreeItems.",
            },
        ],
    },
];

#[component]
pub fn Page() -> impl IntoView {
    let selected = RwSignal::new(String::from("lib.rs"));

    view! {
        <DocLayout
            title="Tree"
            description="Nested rows that expand and collapse, with the arrow keys doing the walking."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="Tab once to enter the tree, then Up and Down walk the rows you can see. Right opens a branch and steps into it, Left closes one and climbs out."
                    code=DEFAULT
                >
                    <div class="w-full max-w-xs rounded-lg border p-2">
                        <Tree label="Project">
                            <TreeItem default_open=true>
                                <TreeRow>"src"</TreeRow>
                                <TreeGroup>
                                    <TreeItem>
                                        <TreeRow>"components"</TreeRow>
                                        <TreeGroup>
                                            <TreeItem>
                                                <TreeRow>"button.rs"</TreeRow>
                                            </TreeItem>
                                            <TreeItem>
                                                <TreeRow>"tree.rs"</TreeRow>
                                            </TreeItem>
                                        </TreeGroup>
                                    </TreeItem>
                                    <TreeItem>
                                        <TreeRow>"lib.rs"</TreeRow>
                                    </TreeItem>
                                </TreeGroup>
                            </TreeItem>
                            <TreeItem>
                                <TreeRow>"Cargo.toml"</TreeRow>
                            </TreeItem>
                        </Tree>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Selection"
                    description="The tree does not keep a selection: which row is current is the caller's, the same way a table's sort is."
                    code=SELECTION
                >
                    <div class="flex w-full max-w-xs flex-col gap-3">
                        <div class="rounded-lg border p-2">
                            <Tree label="Files">
                                <TreeItem default_open=true>
                                    <TreeRow>"src"</TreeRow>
                                    <TreeGroup>
                                        {["lib.rs", "main.rs", "tree.rs"]
                                            .into_iter()
                                            .map(|name| {
                                                view! {
                                                    <TreeItem>
                                                        <TreeRow
                                                            active=Signal::derive(move || { selected.get() == name })
                                                            on_select=Callback::new(move |_| {
                                                                selected.set(name.to_string())
                                                            })
                                                        >
                                                            {name}
                                                        </TreeRow>
                                                    </TreeItem>
                                                }
                                            })
                                            .collect_view()}
                                    </TreeGroup>
                                </TreeItem>
                            </Tree>
                        </div>
                        <p class="text-xs text-muted-foreground">
                            "Selected: " <span class="font-medium text-foreground">{selected}</span>
                        </p>
                    </div>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
