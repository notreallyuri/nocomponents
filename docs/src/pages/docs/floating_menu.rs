//! The floating menu page: a panel you can put somewhere else.

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
    floating_menu::{FloatingMenu, FloatingMenuContent, FloatingMenuHandle},
    switch::Switch,
};

const DEFAULT: &str = r#"// Fixed to the viewport, so it stays where it was put while the page
// scrolls under it. Dragged by the handle, not by the whole panel — a
// panel that moves when you press a button in it is a panel you cannot use.
<FloatingMenu default_position=(320.0, 180.0)>
    <FloatingMenuHandle>"Tools"</FloatingMenuHandle>
    <FloatingMenuContent>
        <Button variant=ButtonVariant::Ghost size=ButtonSize::Sm>"Record"</Button>
        <Button variant=ButtonVariant::Ghost size=ButtonSize::Sm>"Transcribe"</Button>
    </FloatingMenuContent>
</FloatingMenu>"#;

const CONTROLLED: &str = r#"// Pass a signal to drive it — or just to read where the reader left it.
let at = RwSignal::new((420.0, 260.0));

<FloatingMenu position=at>
    <FloatingMenuHandle>"Panel"</FloatingMenuHandle>
    <FloatingMenuContent>{move || format!("{:?}", at.get())}</FloatingMenuContent>
</FloatingMenu>

<Button on:click=move |_| at.set((24.0, 24.0))>"Send it home"</Button>"#;

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
    let showing = RwSignal::new(false);
    let at: RwSignal<(f64, f64)> = RwSignal::new((420.0, 260.0));
    let controlled = RwSignal::new(false);

    view! {
        <DocLayout
            title="Floating Menu"
            description="A panel the reader can pick up and put somewhere else. Fixed to the viewport, dragged by its handle."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="Turn it on and drag it by the strip at the top. It is fixed to the viewport, so scrolling this page leaves it where it is — and it will not go far enough off an edge to be unreachable, nor above the top at all, since there would be nothing above it to grab."
                    code=DEFAULT
                >
                    <div class="flex items-center gap-3">
                        <Switch checked=showing />
                        <span class="text-sm text-muted-foreground">"Show the panel"</span>
                    </div>
                    <Show when=move || showing.get()>
                        <FloatingMenu default_position=(320.0, 180.0)>
                            <FloatingMenuHandle>"Tools"</FloatingMenuHandle>
                            <FloatingMenuContent>
                                <Button variant=ButtonVariant::Ghost size=ButtonSize::Sm>
                                    "Record"
                                </Button>
                                <Button variant=ButtonVariant::Ghost size=ButtonSize::Sm>
                                    "Transcribe"
                                </Button>
                                <Button
                                    variant=ButtonVariant::Ghost
                                    size=ButtonSize::Sm
                                    on:click=move |_| showing.set(false)
                                >
                                    "Close"
                                </Button>
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
                            <FloatingMenuHandle>"Panel"</FloatingMenuHandle>
                            <FloatingMenuContent>
                                <span class="px-1 py-0.5 text-xs text-muted-foreground">
                                    "Drag me, and watch the numbers."
                                </span>
                                <Button
                                    variant=ButtonVariant::Ghost
                                    size=ButtonSize::Sm
                                    on:click=move |_| controlled.set(false)
                                >
                                    "Close"
                                </Button>
                            </FloatingMenuContent>
                        </FloatingMenu>
                    </Show>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
