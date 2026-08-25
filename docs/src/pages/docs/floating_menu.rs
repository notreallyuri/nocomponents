//! The floating menu page: a panel you can put somewhere else.

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
        floating_menu::{
            FloatingMenu, FloatingMenuContent, FloatingMenuHandle, FloatingMenuItem,
            FloatingMenuSeparator,
        },
        switch::Switch,
    },
    icons::{check::Check, copy::Copy, search::Search, x::X},
    utils::types::Orientation,
};

const DEFAULT: &str = r#"// Horizontal by default: a strip of actions floating over the thing they
// act on. Fixed to the viewport, so it stays where it was put while the
// page scrolls under it, and dragged by the grip rather than by the whole
// panel — one that moves when you press a button in it is one you cannot use.
<FloatingMenu default_position=(320.0, 160.0)>
    <FloatingMenuHandle />
    <FloatingMenuContent>
        <FloatingMenuItem label="Record"><Check /></FloatingMenuItem>
        <FloatingMenuItem label="Transcribe"><Search /></FloatingMenuItem>
        <FloatingMenuSeparator />
        <FloatingMenuItem label="Close"><X /></FloatingMenuItem>
    </FloatingMenuContent>
</FloatingMenu>"#;

const VERTICAL: &str = r#"// The variation: a panel rather than a strip. The handle moves to the top
// with the flow, and the separator turns with it — level in a column,
// upright in a row.
<FloatingMenu orientation=Orientation::Vertical>
    <FloatingMenuHandle>"Tools"</FloatingMenuHandle>
    <FloatingMenuContent>
        <FloatingMenuItem label="Record"><Check /></FloatingMenuItem>
        <FloatingMenuItem label="Transcribe"><Search /></FloatingMenuItem>
        <FloatingMenuSeparator />
        <FloatingMenuItem label="Close"><X /></FloatingMenuItem>
    </FloatingMenuContent>
</FloatingMenu>"#;

const CONTROLLED: &str = r#"// The position is a plain signal. Pass one to put the panel somewhere — or
// just to read where the reader left it, which is the half a caller
// usually wants.
let at = RwSignal::new((420.0, 260.0));

<FloatingMenu position=at>
    <FloatingMenuHandle />
    <FloatingMenuContent>
        <FloatingMenuItem label="Copy"><Copy /></FloatingMenuItem>
    </FloatingMenuContent>
</FloatingMenu>

<Button on:click=move |_| at.set((60.0, 120.0))>"Send it to the corner"</Button>"#;

const TOOLBAR: &str = r#"// Compact drops the labels for the icons — and gives each item a tooltip
// and an aria-label, since a label that has been taken away still has to
// be readable somehow.
<FloatingMenu compact=true>
    <FloatingMenuHandle />
    <FloatingMenuContent>
        <FloatingMenuItem label="Search"><Search /></FloatingMenuItem>
        <FloatingMenuItem label="Copy"><Copy /></FloatingMenuItem>
        <FloatingMenuSeparator />
        <FloatingMenuItem label="Accept"><Check /></FloatingMenuItem>
        <FloatingMenuItem label="Dismiss"><X /></FloatingMenuItem>
    </FloatingMenuContent>
</FloatingMenu>"#;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "FloatingMenu",
        description: "The panel. Fixed to the viewport, and kept far enough inside it to stay reachable.",
        props: &[
            Prop {
                name: "position",
                ty: "Option<RwSignal<(f64, f64)>>",
                default: "None",
                description: "Top-left in client coordinates. Pass one to drive it from outside, or to read where the reader left it. Left out, the panel owns its own.",
            },
            Prop {
                name: "default_position",
                ty: "(f64, f64)",
                default: "(24.0, 24.0)",
                description: "Where it starts when no position is passed.",
            },
            Prop {
                name: "orientation",
                ty: "Signal<Orientation>",
                default: "Vertical",
                description: "Vertical is a panel, horizontal is a toolbar. The handle moves to the start of whichever it is.",
            },
            Prop {
                name: "compact",
                ty: "Signal<bool>",
                default: "false",
                description: "Items drop their labels for their icons, and each grows a tooltip carrying the label instead.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the panel's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "A handle, and whatever the panel holds.",
            },
        ],
    },
    ApiEntry {
        name: "FloatingMenuHandle",
        description: "The strip a press moves the panel by. Its own part, so the rest of the panel stays usable.",
        props: &[
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Pins the panel where it is.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the handle's classes.",
            },
            Prop {
                name: "children",
                ty: "Option<Children>",
                default: "None",
                description: "A label beside the grip. Leave it out for a bare grip.",
            },
        ],
    },
    ApiEntry {
        name: "FloatingMenuItem",
        description: "One action: an icon for children, and a label as a prop so a compact menu can still say what the icon means.",
        props: &[
            Prop {
                name: "label",
                ty: "Signal<String>",
                default: "",
                description: "Shown beside the icon, or carried by a tooltip and aria-label when the menu is compact.",
            },
            Prop {
                name: "on_click",
                ty: "Option<Callback<ev::MouseEvent>>",
                default: "None",
                description: "Run when it is pressed.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Refuses the press and dims the item.",
            },
            Prop {
                name: "children",
                ty: "Option<ChildrenFn>",
                default: "None",
                description: "The icon. A ChildrenFn because a compact item renders inside a tooltip and a full one does not.",
            },
        ],
    },
    ApiEntry {
        name: "FloatingMenuSeparator",
        description: "A rule across the flow: level in a column of items, upright in a row of them.",
        props: &[Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Merged over the rule's classes.",
        }],
    },
    ApiEntry {
        name: "FloatingMenuContent",
        description: "Everything under the handle.",
        props: &[Prop {
            name: "children",
            ty: "Children",
            default: "",
            description: "The panel's contents.",
        }],
    },
];

#[component]
pub fn Page() -> impl IntoView {
    let horizontal = RwSignal::new(false);
    let vertical = RwSignal::new(false);
    let controlled = RwSignal::new(false);
    let toolbar = RwSignal::new(false);
    let at: RwSignal<(f64, f64)> = RwSignal::new((420.0, 260.0));

    view! {
        <DocLayout
            title="Floating Menu"
            description="A panel the reader can pick up and put somewhere else. Fixed to the viewport, dragged by its handle."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="A strip of actions over the thing they act on. Turn it on and drag it by the grip on the left. It is fixed to the viewport, so scrolling this page leaves it where it is — and it will not go far enough off an edge to be unreachable, nor above the top at all, there being nothing up there to grab it back by."
                    code=DEFAULT
                >
                    <div class="flex items-center gap-3">
                        <Switch checked=horizontal />
                        <span class="text-sm text-muted-foreground">"Show it"</span>
                    </div>
                    <Show when=move || horizontal.get()>
                        <FloatingMenu default_position=(320.0, 160.0)>
                            <FloatingMenuHandle />
                            <FloatingMenuContent>
                                <FloatingMenuItem label="Record">
                                    <Check />
                                </FloatingMenuItem>
                                <FloatingMenuItem label="Transcribe">
                                    <Search />
                                </FloatingMenuItem>
                                <FloatingMenuSeparator />
                                <FloatingMenuItem
                                    label="Close"
                                    on_click=Callback::new(move |_| horizontal.set(false))
                                >
                                    <X />
                                </FloatingMenuItem>
                            </FloatingMenuContent>
                        </FloatingMenu>
                    </Show>
                </DemoSection>

                <DemoSection
                    title="Vertical"
                    description="The variation: a panel rather than a strip. The handle moves to the top with the flow and takes a label if you give it one, and the separator turns with it — level in a column, upright in a row."
                    code=VERTICAL
                >
                    <div class="flex items-center gap-3">
                        <Switch checked=vertical />
                        <span class="text-sm text-muted-foreground">"Show it"</span>
                    </div>
                    <Show when=move || vertical.get()>
                        <FloatingMenu
                            default_position=(320.0, 300.0)
                            orientation=Orientation::Vertical
                        >
                            <FloatingMenuHandle>"Tools"</FloatingMenuHandle>
                            <FloatingMenuContent>
                                <FloatingMenuItem label="Record">
                                    <Check />
                                </FloatingMenuItem>
                                <FloatingMenuItem label="Transcribe">
                                    <Search />
                                </FloatingMenuItem>
                                <FloatingMenuSeparator />
                                <FloatingMenuItem
                                    label="Close"
                                    on_click=Callback::new(move |_| vertical.set(false))
                                >
                                    <X />
                                </FloatingMenuItem>
                            </FloatingMenuContent>
                        </FloatingMenu>
                    </Show>
                </DemoSection>

                <DemoSection
                    title="Controlled"
                    description="The position is a signal like any other. Passing one in lets you put the panel somewhere — and lets you read where it ended up, which is the half a caller usually wants."
                    code=CONTROLLED
                >
                    <div class="flex flex-wrap items-center gap-3">
                        <Switch checked=controlled />
                        <span class="text-sm text-muted-foreground">"Show it"</span>
                        <Button
                            variant=ButtonVariant::Outline
                            size=ButtonSize::Sm
                            on:click=move |_| at.set((60.0, 120.0))
                        >
                            "Send it to the corner"
                        </Button>
                        <span class="font-mono text-xs text-muted-foreground">
                            {move || {
                                let (x, y) = at.get();
                                format!("({}, {})", x.round(), y.round())
                            }}
                        </span>
                    </div>
                    <Show when=move || controlled.get()>
                        <FloatingMenu position=at>
                            <FloatingMenuHandle />
                            <FloatingMenuContent>
                                <FloatingMenuItem label="Copy">
                                    <Copy />
                                </FloatingMenuItem>
                                <FloatingMenuItem
                                    label="Close"
                                    on_click=Callback::new(move |_| controlled.set(false))
                                >
                                    <X />
                                </FloatingMenuItem>
                            </FloatingMenuContent>
                        </FloatingMenu>
                    </Show>
                </DemoSection>

                <DemoSection
                    title="A compact toolbar"
                    description="Compact drops the labels for the icons, and each item grows a tooltip and an aria-label carrying the label instead — a label taken away still has to be readable somehow. The same thing the sidebar does when it collapses to a rail."
                    code=TOOLBAR
                >
                    <div class="flex items-center gap-3">
                        <Switch checked=toolbar />
                        <span class="text-sm text-muted-foreground">"Show it"</span>
                    </div>
                    <Show when=move || toolbar.get()>
                        <FloatingMenu default_position=(360.0, 440.0) compact=true>
                            <FloatingMenuHandle />
                            <FloatingMenuContent>
                                <FloatingMenuItem label="Search">
                                    <Search />
                                </FloatingMenuItem>
                                <FloatingMenuItem label="Copy">
                                    <Copy />
                                </FloatingMenuItem>
                                <FloatingMenuSeparator />
                                <FloatingMenuItem label="Accept">
                                    <Check />
                                </FloatingMenuItem>
                                <FloatingMenuItem
                                    label="Dismiss"
                                    on_click=Callback::new(move |_| toolbar.set(false))
                                >
                                    <X />
                                </FloatingMenuItem>
                            </FloatingMenuContent>
                        </FloatingMenu>
                    </Show>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
