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
        button::ButtonVariant,
        drawer::{
            Drawer, DrawerClose, DrawerContent, DrawerDescription, DrawerFooter, DrawerHeader,
            DrawerTitle, DrawerTrigger,
        },
        scroll_area::ScrollArea,
    },
    utils::types::Side,
};

const DEFAULT: &str = r#"<Drawer>
    <DrawerTrigger>"Open drawer"</DrawerTrigger>
    <DrawerContent>
        <DrawerHeader>
            <DrawerTitle>"Move to project"</DrawerTitle>
            <DrawerDescription>
                "Drag the panel down, or throw it, to dismiss."
            </DrawerDescription>
        </DrawerHeader>
        <DrawerFooter>
            <DrawerClose>"Cancel"</DrawerClose>
        </DrawerFooter>
    </DrawerContent>
</Drawer>"#;

const SIDES: &str = r#"// Any edge. The drag axis and its direction follow the side.
<Drawer side=Side::Right>
    <DrawerTrigger>"From the right"</DrawerTrigger>
    <DrawerContent>
        // ...
    </DrawerContent>
</Drawer>"#;

const NESTED: &str = r#"// Drawers inside drawers, four deep. Each has its own dialog context, so
// the layer stack unwinds one Escape at a time and dragging the inner panel
// leaves the ones behind it where they are. Every open child pushes its
// ancestors a step further back, and the recede stops compounding after
// three. The side is set once per root: it decides the drag axis as well as
// the edge.
<Drawer side=Side::Right>
    <DrawerTrigger>"Open outer"</DrawerTrigger>
    <DrawerContent>
        <DrawerHeader>
            <DrawerTitle>"Choose a project"</DrawerTitle>
        </DrawerHeader>
        <Drawer side=Side::Right>
            <DrawerTrigger>"New project"</DrawerTrigger>
            <DrawerContent>
                <DrawerHeader>
                    <DrawerTitle>"New project"</DrawerTitle>
                </DrawerHeader>
                <Drawer side=Side::Right>
                    <DrawerTrigger>"Pick a template"</DrawerTrigger>
                    <DrawerContent>
                        <DrawerHeader>
                            <DrawerTitle>"Templates"</DrawerTitle>
                        </DrawerHeader>
                        <Drawer side=Side::Right>
                            <DrawerTrigger>"Configure the repository"</DrawerTrigger>
                            <DrawerContent>
                                <DrawerHeader>
                                    <DrawerTitle>"Repository"</DrawerTitle>
                                </DrawerHeader>
                                <DrawerFooter>
                                    <DrawerClose>"Back"</DrawerClose>
                                </DrawerFooter>
                            </DrawerContent>
                        </Drawer>
                    </DrawerContent>
                </Drawer>
            </DrawerContent>
        </Drawer>
    </DrawerContent>
</Drawer>"#;

const NON_MODAL: &str = r#"// No overlay, no focus trap, and the page underneath stays usable.
// A pointer landing outside closes it, since there is no overlay to
// click. Set it once on the root; the content reads it from context.
<Drawer side=Side::Right modal=false>
    <DrawerTrigger>"Open non-modal"</DrawerTrigger>
    <DrawerContent>
        <DrawerHeader>
            <DrawerTitle>"Now playing"</DrawerTitle>
        </DrawerHeader>
    </DrawerContent>
</Drawer>"#;

const SCROLLING: &str = r#"// A press that lands in something already scrolled belongs to that
// scroller, so the list scrolls instead of the drawer leaving. Scroll
// back to the top and the drag dismisses again.
<DrawerContent>
    <DrawerHeader>
        <DrawerTitle>"Releases"</DrawerTitle>
    </DrawerHeader>
    <ScrollArea class="h-64">
        // ...a long list...
    </ScrollArea>
</DrawerContent>"#;

const SNAP_POINTS: &str = r#"// The panel is as tall as the highest point and rests at the active one.
// Drag the handle to move between them: a slow release goes to the nearest,
// a flick moves one point, and letting go under 22 percent of the screen
// dismisses. The snap point in force is written to `data-snap`.
<Drawer snap_points=vec![0.4, 0.88]>
    <DrawerTrigger>"Open asset info"</DrawerTrigger>
    <DrawerContent>
        <DrawerHeader>
            <DrawerTitle>"Asset info"</DrawerTitle>
            <DrawerDescription>
                "Peeks at 40 percent; drag the handle up for the rest."
            </DrawerDescription>
        </DrawerHeader>
        <div class="min-h-0 flex-1 overflow-y-auto">
            // ...details...
        </div>
    </DrawerContent>
</Drawer>"#;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "Drawer",
        description: "A sheet you can throw away with your thumb: drag it toward its edge and let go.",
        props: &[
            Prop {
                name: "side",
                ty: "Side",
                default: "Side::Bottom",
                description: "One of: Left, Right, Bottom, Top.",
            },
            Prop {
                name: "open",
                ty: "Option<RwSignal<bool>>",
                default: "None",
                description: "Pass one to drive it from outside; omit it and the root owns the signal.",
            },
            Prop {
                name: "default_open",
                ty: "bool",
                default: "false",
                description: "Whether the drawer starts open. Ignored when `open` is given.",
            },
            Prop {
                name: "on_open_change",
                ty: "Option<Callback<bool>>",
                default: "None",
                description: "Called with the new value whenever the drawer opens or closes, including when the signal passed as `open` is written from outside.",
            },
            Prop {
                name: "modal",
                ty: "bool",
                default: "true",
                description: "Non-modal leaves the page usable underneath: no overlay, no focus trap, and an outside pointer closes the drawer. Set here only; the content reads it from context.",
            },
            Prop {
                name: "snap_points",
                ty: "Signal<Vec<f64>>",
                default: "vec![]",
                description: "Heights the drawer rests at, as fractions of the viewport along its axis: `vec![0.4, 0.88]` is a peek and a full. The panel is as tall as the highest and slides to the active one; the first is where it opens. Empty is an ordinary drawer.",
            },
            Prop {
                name: "active_snap",
                ty: "Option<RwSignal<Option<f64>>>",
                default: "None",
                description: "The snap point the drawer is at, held by the caller; `None` inside means the first. Omit it and the root owns the signal. A value that is not in `snap_points` resolves to the nearest one.",
            },
            Prop {
                name: "default_active_snap",
                ty: "Option<f64>",
                default: "None",
                description: "The snap point the drawer starts at. Ignored when `active_snap` is given.",
            },
            Prop {
                name: "on_active_snap_change",
                ty: "Option<Callback<Option<f64>>>",
                default: "None",
                description: "Called with the new snap point whenever it changes, and with `None` when a drag dismisses the drawer, so the next opening is at the first point.",
            },
            Prop {
                name: "dismiss_below",
                ty: "Signal<f64>",
                default: "0.22",
                description: "The share of the viewport under which a released snapping drawer is dismissed, however slowly it got there.",
            },
            Prop {
                name: "flick_velocity",
                ty: "Signal<f64>",
                default: "0.6",
                description: "Release speed in pixels per millisecond that moves a snapping drawer one snap point in the flick's direction, or dismisses it from the lowest. Slower releases go to the nearest point.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "A trigger and a content.",
            },
        ],
    },
    ApiEntry {
        name: "DrawerContent",
        description: "The panel. Dragging it moves it; releasing past a quarter of its length, or at speed, dismisses it.",
        props: &[
            Prop {
                name: "handle",
                ty: "bool",
                default: "true",
                description: "Draw the grab bar. On by default: a drawer that does not look draggable will not be.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the panel's classes.",
            },
            Prop {
                name: "children",
                ty: "ChildrenFn",
                default: "",
                description: "The panel's contents. Re-run on each mount, hence `ChildrenFn`.",
            },
        ],
    },
    ApiEntry {
        name: "DrawerHandle",
        description: "The grab bar. Rendered by `DrawerContent` unless `handle=false`, so it is rarely written by hand.",
        props: &[Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Merged over the handle's classes.",
        }],
    },
    ApiEntry {
        name: "DrawerTrigger",
        description: "The button that opens the drawer.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "The trigger is unstyled: everything it looks like comes from here.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Stops the trigger opening the drawer, and resolves against any `Field` around it.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The label.",
            },
        ],
    },
];

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout title="Drawer" description="A sheet you can throw away with your thumb.">
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="Everything a drawer shares with a dialog is the dialog's — modality, focus trap, Escape. What it adds is the gesture: drag the panel toward its edge and let go past a quarter of its length, or flick it at speed and let go anywhere."
                    code=DEFAULT
                >
                    <Drawer>
                        <DrawerTrigger class="rounded-lg border border-border px-3 py-1.5 text-sm">
                            "Open drawer"
                        </DrawerTrigger>
                        <DrawerContent>
                            <DrawerHeader>
                                <DrawerTitle>"Move to project"</DrawerTitle>
                                <DrawerDescription>
                                    "Drag the panel down, or throw it, to dismiss."
                                </DrawerDescription>
                            </DrawerHeader>
                            <DrawerFooter>
                                <DrawerClose>"Cancel"</DrawerClose>
                            </DrawerFooter>
                        </DrawerContent>
                    </Drawer>
                </DemoSection>

                <DemoSection
                    title="Any edge"
                    description="The drag axis and its direction come from the side, so a right-hand drawer is thrown right and a top one upward. Dragging the other way does nothing — there is nothing behind the edge to reveal."
                    code=SIDES
                >
                    <div class="flex flex-wrap gap-2">
                        <Drawer side=Side::Right>
                            <DrawerTrigger class="rounded-lg border border-border px-3 py-1.5 text-sm">
                                "From the right"
                            </DrawerTrigger>
                            <DrawerContent>
                                <DrawerHeader>
                                    <DrawerTitle>"Filters"</DrawerTitle>
                                    <DrawerDescription>
                                        "Throw it rightward to dismiss."
                                    </DrawerDescription>
                                </DrawerHeader>
                            </DrawerContent>
                        </Drawer>

                        <Drawer side=Side::Top>
                            <DrawerTrigger class="rounded-lg border border-border px-3 py-1.5 text-sm">
                                "From the top"
                            </DrawerTrigger>
                            <DrawerContent>
                                <DrawerHeader>
                                    <DrawerTitle>"Notifications"</DrawerTitle>
                                    <DrawerDescription>
                                        "Throw it upward to dismiss."
                                    </DrawerDescription>
                                </DrawerHeader>
                            </DrawerContent>
                        </Drawer>

                        <Drawer side=Side::Left>
                            <DrawerTrigger class="rounded-lg border border-border px-3 py-1.5 text-sm">
                                "From the left"
                            </DrawerTrigger>
                            <DrawerContent>
                                <DrawerHeader>
                                    <DrawerTitle>"Navigation"</DrawerTitle>
                                    <DrawerDescription>
                                        "Throw it leftward to dismiss."
                                    </DrawerDescription>
                                </DrawerHeader>
                            </DrawerContent>
                        </Drawer>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Nested"
                    description="Drawers opened from inside one another. Each has its own dialog context, so Escape unwinds them one at a time and dragging the inner panel leaves the ones behind it where they are. Every open child pushes its ancestors a step further back, and the recede stops compounding after three of them."
                    code=NESTED
                >
                    <Drawer side=Side::Right>
                        <DrawerTrigger class="rounded-lg border border-border px-3 py-1.5 text-sm">
                            "Open outer"
                        </DrawerTrigger>
                        <DrawerContent>
                            <DrawerHeader>
                                <DrawerTitle>"Choose a project"</DrawerTitle>
                                <DrawerDescription>
                                    "Opening the inner drawer stacks a second layer on this one."
                                </DrawerDescription>
                            </DrawerHeader>
                            <Drawer side=Side::Right>
                                <DrawerTrigger class="w-full rounded-lg border border-border px-3 py-1.5 text-sm">
                                    "New project"
                                </DrawerTrigger>
                                <DrawerContent>
                                    <DrawerHeader>
                                        <DrawerTitle>"New project"</DrawerTitle>
                                        <DrawerDescription>
                                            "The panel behind this one has given up a little room. Go deeper and it gives up a little more."
                                        </DrawerDescription>
                                    </DrawerHeader>
                                    <Drawer side=Side::Right>
                                        <DrawerTrigger class="w-full rounded-lg border border-border px-3 py-1.5 text-sm">
                                            "Pick a template"
                                        </DrawerTrigger>
                                        <DrawerContent>
                                            <DrawerHeader>
                                                <DrawerTitle>"Templates"</DrawerTitle>
                                                <DrawerDescription>
                                                    "Two layers are behind this one now, each set back one step further than the last."
                                                </DrawerDescription>
                                            </DrawerHeader>
                                            <Drawer side=Side::Right>
                                                <DrawerTrigger class="w-full rounded-lg border border-border px-3 py-1.5 text-sm">
                                                    "Configure the repository"
                                                </DrawerTrigger>
                                                <DrawerContent>
                                                    <DrawerHeader>
                                                        <DrawerTitle>"Repository"</DrawerTitle>
                                                        <DrawerDescription>
                                                            "The bottom of the stack has stopped receding — past three layers the panel would be a sliver."
                                                        </DrawerDescription>
                                                    </DrawerHeader>
                                                    <DrawerFooter>
                                                        <DrawerClose>"Back"</DrawerClose>
                                                    </DrawerFooter>
                                                </DrawerContent>
                                            </Drawer>
                                            <DrawerFooter>
                                                <DrawerClose>"Back"</DrawerClose>
                                            </DrawerFooter>
                                        </DrawerContent>
                                    </Drawer>
                                    <DrawerFooter>
                                        <DrawerClose>"Back"</DrawerClose>
                                    </DrawerFooter>
                                </DrawerContent>
                            </Drawer>
                        </DrawerContent>
                    </Drawer>
                </DemoSection>

                <DemoSection
                    title="Non-modal"
                    description="No overlay, no focus trap, and the page underneath stays usable — scroll it, click it, tab through it. A pointer landing outside closes the drawer, since there is no overlay to catch it."
                    code=NON_MODAL
                >
                    <Drawer side=Side::Right modal=false>
                        <DrawerTrigger class="rounded-lg border border-border px-3 py-1.5 text-sm">
                            "Open non-modal"
                        </DrawerTrigger>
                        <DrawerContent>
                            <DrawerHeader>
                                <DrawerTitle>"Now playing"</DrawerTitle>
                                <DrawerDescription>
                                    "Keep reading the page behind this — it is still yours."
                                </DrawerDescription>
                            </DrawerHeader>
                        </DrawerContent>
                    </Drawer>
                </DemoSection>

                <DemoSection
                    title="With scrolling content"
                    description="Scroll the list down, then try to drag the drawer from inside it: the gesture belongs to the list. Scroll back to the top and dragging dismisses again."
                    code=SCROLLING
                >
                    <Drawer>
                        <DrawerTrigger class="rounded-lg border border-border px-3 py-1.5 text-sm">
                            "Open a long drawer"
                        </DrawerTrigger>
                        <DrawerContent>
                            <DrawerHeader>
                                <DrawerTitle>"Releases"</DrawerTitle>
                                <DrawerDescription>
                                    "Drag from the handle to dismiss; drag from the list to scroll."
                                </DrawerDescription>
                            </DrawerHeader>
                            <ScrollArea class="h-64 rounded-lg border border-border">
                                <div class="flex flex-col gap-2 p-3">
                                    {(1..=30)
                                        .map(|i| {
                                            view! {
                                                <div class="rounded border border-border px-3 py-1.5 text-muted-foreground">
                                                    {format!("v1.2.{i}")}
                                                </div>
                                            }
                                        })
                                        .collect_view()}
                                </div>
                            </ScrollArea>
                            <DrawerFooter>
                                <DrawerClose variant=ButtonVariant::Outline>"Close"</DrawerClose>
                            </DrawerFooter>
                        </DrawerContent>
                    </Drawer>
                </DemoSection>

                <DemoSection
                    title="Snap points"
                    description="Give the drawer heights to rest at and it stops being all-or-nothing. It opens at the first; drag the handle up past the midpoint and it settles at the next, drag it down and it comes back, and let go under 22 percent of the screen and it is dismissed. A quick flick moves one point in its direction, or dismisses from the lowest. Best tried at phone size."
                    code=SNAP_POINTS
                >
                    <Drawer snap_points=vec![0.4, 0.88]>
                        <DrawerTrigger class="rounded-lg border border-border px-3 py-1.5 text-sm">
                            "Open asset info"
                        </DrawerTrigger>
                        <DrawerContent>
                            <DrawerHeader>
                                <DrawerTitle>"Asset info"</DrawerTitle>
                                <DrawerDescription>
                                    "Peeks at 40 percent; drag the handle up for the rest."
                                </DrawerDescription>
                            </DrawerHeader>
                            <div class="min-h-0 flex-1 overflow-y-auto">
                                <div class="flex flex-col gap-2 p-3">
                                    {(1..=20)
                                        .map(|i| {
                                            view! {
                                                <div class="rounded border border-border px-3 py-1.5 text-muted-foreground">
                                                    {format!("Detail {i}")}
                                                </div>
                                            }
                                        })
                                        .collect_view()}
                                </div>
                            </div>
                        </DrawerContent>
                    </Drawer>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
