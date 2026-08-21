use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::{
    components::resizable::{ResizableGroup, ResizableHandle, ResizablePanel},
    utils::types::Orientation,
};

const DEFAULT: &str = r#"<ResizableGroup class="h-48 rounded-lg border">
    <ResizablePanel default_size=35.0 min_size=20.0>
        <div class="flex h-full items-center justify-center p-4">"One"</div>
    </ResizablePanel>
    <ResizableHandle index=0 />
    <ResizablePanel min_size=20.0>
        <div class="flex h-full items-center justify-center p-4">"Two"</div>
    </ResizablePanel>
</ResizableGroup>"#;

const VERTICAL: &str = r#"<ResizableGroup direction=Orientation::Vertical class="h-64 rounded-lg border">
    <ResizablePanel default_size=30.0 min_size=15.0>…</ResizablePanel>
    <ResizableHandle index=0 with_handle=true />
    <ResizablePanel min_size=15.0>…</ResizablePanel>
</ResizableGroup>"#;

const NESTED: &str = r#"// A handle's `index` is the boundary it sits on: the one after the panel
// with that index, counting within its own group.
<ResizableGroup>
    <ResizablePanel default_size=30.0 min_size=15.0>…</ResizablePanel>
    <ResizableHandle index=0 with_handle=true />
    <ResizablePanel>
        <ResizableGroup direction=Orientation::Vertical>
            <ResizablePanel default_size=60.0 min_size=20.0>…</ResizablePanel>
            <ResizableHandle index=0 with_handle=true />
            <ResizablePanel min_size=20.0>…</ResizablePanel>
        </ResizableGroup>
    </ResizablePanel>
</ResizableGroup>"#;

const PERSISTED: &str = r#"{
    // The layout is a plain `Vec<f64>` of percentages, so it can be read out and put back.
    let sizes = RwSignal::new(Vec::<f64>::new());

    view! {
        <ResizableGroup sizes=sizes>…</ResizableGroup>
        <p>{move || sizes.get().iter().map(|s| format!("{s:.0}%")).collect::<Vec<_>>().join(" · ")}</p>
    }
}"#;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "ResizableGroup",
        description: "A row or column of panels. Owns the sizes, as percentages that always add up to 100.",
        props: &[
            Prop {
                name: "direction",
                ty: "Orientation",
                default: "Horizontal",
                description: "`Horizontal` lays the panels side by side, `Vertical` stacks them.",
            },
            Prop {
                name: "sizes",
                ty: "RwSignal<Vec<f64>>",
                default: "[]",
                description: "One percentage per panel, in document order. Read it to persist a layout; write it to restore one.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the group's classes. The group needs a height of its own.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Panels with handles between them.",
            },
        ],
    },
    ApiEntry {
        name: "ResizablePanel",
        description: "One panel. A flex item whose grow factor is its percentage, so the handles' own widths never enter the arithmetic.",
        props: &[
            Prop {
                name: "default_size",
                ty: "Option<f64>",
                default: "None",
                description: "The size to open at, as a percentage. Panels without one share whatever the sized panels left over.",
            },
            Prop {
                name: "min_size",
                ty: "f64",
                default: "0.0",
                description: "How small this panel may be dragged, as a percentage.",
            },
            Prop {
                name: "max_size",
                ty: "f64",
                default: "100.0",
                description: "How large this panel may be dragged, as a percentage.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the panel's classes. The panel already clips its overflow.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The panel's contents.",
            },
        ],
    },
    ApiEntry {
        name: "ResizableHandle",
        description: "The divider after a panel. A `role=separator` that can be dragged or moved with the arrow keys, five points at a time.",
        props: &[
            Prop {
                name: "index",
                ty: "usize",
                default: "",
                description: "Which boundary this is: the one after the panel with this index, counting within its own group.",
            },
            Prop {
                name: "with_handle",
                ty: "bool",
                default: "false",
                description: "Draws a grip on the divider, so it reads as something to take hold of.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Fixes the boundary: no drag, no arrows, and out of the tab order.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the divider's classes.",
            },
        ],
    },
];

#[component]
pub fn Page() -> impl IntoView {
    let sizes = RwSignal::new(Vec::<f64>::new());

    view! {
        <DocLayout
            title="Resizable"
            description="Panels with draggable dividers between them."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="Drag the divider, or focus it and use the arrow keys. Sizes are percentages of the group, so the layout holds when the window changes; a panel stops at its own limits, and the panel beside it takes exactly what the other gave up."
                    code=DEFAULT
                >
                    <ResizableGroup class="h-48 rounded-lg border">
                        <ResizablePanel default_size=35.0 min_size=20.0>
                            <div class="flex h-full items-center justify-center p-4 text-sm font-medium">
                                "One"
                            </div>
                        </ResizablePanel>
                        <ResizableHandle index=0 />
                        <ResizablePanel min_size=20.0>
                            <div class="flex h-full items-center justify-center p-4 text-sm font-medium">
                                "Two"
                            </div>
                        </ResizablePanel>
                    </ResizableGroup>
                </DemoSection>

                <DemoSection
                    title="Vertical"
                    description="The same thing on its side. The divider's ARIA orientation is the opposite of the group's — a separator between two panels side by side is itself vertical."
                    code=VERTICAL
                >
                    <ResizableGroup
                        direction=Orientation::Vertical
                        class="h-64 rounded-lg border"
                    >
                        <ResizablePanel default_size=30.0 min_size=15.0>
                            <div class="flex h-full items-center justify-center p-4 text-sm font-medium">
                                "Header"
                            </div>
                        </ResizablePanel>
                        <ResizableHandle index=0 with_handle=true />
                        <ResizablePanel min_size=15.0>
                            <div class="flex h-full items-center justify-center p-4 text-sm font-medium">
                                "Body"
                            </div>
                        </ResizablePanel>
                    </ResizableGroup>
                </DemoSection>

                <DemoSection
                    title="Nested"
                    description="A panel can hold another group. Each group counts its own panels, so a handle's index is always local to the group it sits in."
                    code=NESTED
                >
                    <ResizableGroup class="h-72 rounded-lg border">
                        <ResizablePanel default_size=30.0 min_size=15.0>
                            <div class="flex h-full items-center justify-center p-4 text-sm font-medium">
                                "Sidebar"
                            </div>
                        </ResizablePanel>
                        <ResizableHandle index=0 with_handle=true />
                        <ResizablePanel>
                            <ResizableGroup direction=Orientation::Vertical>
                                <ResizablePanel default_size=60.0 min_size=20.0>
                                    <div class="flex h-full items-center justify-center p-4 text-sm font-medium">
                                        "Editor"
                                    </div>
                                </ResizablePanel>
                                <ResizableHandle index=0 with_handle=true />
                                <ResizablePanel min_size=20.0>
                                    <div class="flex h-full items-center justify-center p-4 text-sm font-medium">
                                        "Terminal"
                                    </div>
                                </ResizablePanel>
                            </ResizableGroup>
                        </ResizablePanel>
                    </ResizableGroup>
                </DemoSection>

                <DemoSection
                    title="Reading the layout"
                    description="The sizes are a plain vector of percentages, in document order. Pass one in to watch the layout, or to restore a saved one."
                    code=PERSISTED
                >
                    <div class="flex flex-col gap-3">
                        <ResizableGroup sizes=sizes class="h-40 rounded-lg border">
                            <ResizablePanel min_size=15.0>
                                <div class="flex h-full items-center justify-center p-4 text-sm font-medium">
                                    "A"
                                </div>
                            </ResizablePanel>
                            <ResizableHandle index=0 with_handle=true />
                            <ResizablePanel min_size=15.0>
                                <div class="flex h-full items-center justify-center p-4 text-sm font-medium">
                                    "B"
                                </div>
                            </ResizablePanel>
                            <ResizableHandle index=1 with_handle=true />
                            <ResizablePanel min_size=15.0>
                                <div class="flex h-full items-center justify-center p-4 text-sm font-medium">
                                    "C"
                                </div>
                            </ResizablePanel>
                        </ResizableGroup>
                        <p class="font-mono text-sm text-muted-foreground">
                            {move || {
                                sizes
                                    .get()
                                    .iter()
                                    .map(|size| format!("{size:.0}%"))
                                    .collect::<Vec<_>>()
                                    .join(" · ")
                            }}
                        </p>
                    </div>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
