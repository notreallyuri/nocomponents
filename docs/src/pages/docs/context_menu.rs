use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::components::context_menu::{
    ContextMenu, ContextMenuCheckboxItem, ContextMenuContent, ContextMenuItem, ContextMenuLabel,
    ContextMenuPortal, ContextMenuRadioGroup, ContextMenuRadioItem, ContextMenuSeparator,
    ContextMenuSub, ContextMenuSubContent, ContextMenuSubTrigger, ContextMenuTrigger,
};

const DEFAULT: &str = r#"<ContextMenu>
    <ContextMenuTrigger class="flex h-32 items-center justify-center rounded-lg border border-dashed text-sm text-muted-foreground">
        "Right-click here"
    </ContextMenuTrigger>
    <ContextMenuPortal>
        <ContextMenuContent>
            <ContextMenuItem>"Back"</ContextMenuItem>
            <ContextMenuItem>"Forward"</ContextMenuItem>
            <ContextMenuItem>"Reload"</ContextMenuItem>
        </ContextMenuContent>
    </ContextMenuPortal>
</ContextMenu>"#;

const SUBMENU: &str = r#"<ContextMenu>
    <ContextMenuTrigger class="flex h-32 items-center justify-center rounded-lg border border-dashed text-sm text-muted-foreground">
        "Right-click here"
    </ContextMenuTrigger>
    <ContextMenuPortal>
        <ContextMenuContent class="min-w-44">
            <ContextMenuLabel>"Layer"</ContextMenuLabel>
            <ContextMenuItem>"Rename"</ContextMenuItem>
            <ContextMenuItem>"Duplicate"</ContextMenuItem>
            <ContextMenuSeparator />
            <ContextMenuSub>
                <ContextMenuSubTrigger>"Move to"</ContextMenuSubTrigger>
                <ContextMenuSubContent>
                    <ContextMenuItem>"Front"</ContextMenuItem>
                    <ContextMenuItem>"Forward"</ContextMenuItem>
                    <ContextMenuItem>"Backward"</ContextMenuItem>
                    <ContextMenuItem>"Back"</ContextMenuItem>
                </ContextMenuSubContent>
            </ContextMenuSub>
            <ContextMenuSeparator />
            <ContextMenuItem>"Delete"</ContextMenuItem>
        </ContextMenuContent>
    </ContextMenuPortal>
</ContextMenu>"#;

const SELECTION: &str = r#"<ContextMenu>
    <ContextMenuTrigger class="...">"Right-click here"</ContextMenuTrigger>
    <ContextMenuPortal>
        <ContextMenuContent>
            <ContextMenuItem on_click=Callback::new(move |_| action.set("Cut"))>
                "Cut"
            </ContextMenuItem>
            <ContextMenuItem on_click=Callback::new(move |_| action.set("Copy"))>
                "Copy"
            </ContextMenuItem>
            <ContextMenuItem on_click=Callback::new(move |_| action.set("Paste"))>
                "Paste"
            </ContextMenuItem>
        </ContextMenuContent>
    </ContextMenuPortal>
</ContextMenu>"#;

const ON_PHONES: &str = r#"let show_grid = RwSignal::new(true);
let zoom = RwSignal::new("100".to_string());

view! {
    <ContextMenu>
        <ContextMenuTrigger class="...">"Right-click or press and hold"</ContextMenuTrigger>
        <ContextMenuPortal>
            <ContextMenuContent class="min-w-44" mobile_sheet=true sheet_title="Layer">
                <ContextMenuItem>"Rename"</ContextMenuItem>
                <ContextMenuItem>"Duplicate"</ContextMenuItem>
                <ContextMenuSub>
                    <ContextMenuSubTrigger>"Move to"</ContextMenuSubTrigger>
                    <ContextMenuSubContent>
                        <ContextMenuItem>"Front"</ContextMenuItem>
                        <ContextMenuItem>"Forward"</ContextMenuItem>
                        <ContextMenuItem>"Backward"</ContextMenuItem>
                        <ContextMenuItem>"Back"</ContextMenuItem>
                    </ContextMenuSubContent>
                </ContextMenuSub>
                <ContextMenuSeparator />
                <ContextMenuCheckboxItem checked=show_grid>"Show grid"</ContextMenuCheckboxItem>
                <ContextMenuSeparator />
                <ContextMenuLabel>"Zoom"</ContextMenuLabel>
                <ContextMenuRadioGroup value=zoom>
                    <ContextMenuRadioItem value="50">"50%"</ContextMenuRadioItem>
                    <ContextMenuRadioItem value="100">"100%"</ContextMenuRadioItem>
                    <ContextMenuRadioItem value="200">"200%"</ContextMenuRadioItem>
                </ContextMenuRadioGroup>
                <ContextMenuSeparator />
                <ContextMenuItem>"Delete"</ContextMenuItem>
            </ContextMenuContent>
        </ContextMenuPortal>
    </ContextMenu>
}"#;

const AREA: &str = "flex h-32 items-center justify-center rounded-lg border border-dashed border-border text-sm text-muted-foreground";

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "ContextMenu",
        description: "A menu summoned by right-clicking a region, or long-pressing it on touch, anchored to the pointer. Everything below the trigger is the dropdown menu's.",
        props: &[
            Prop {
                name: "open",
                ty: "Option<RwSignal<bool>>",
                default: "None",
                description: "Pass one to drive the menu from outside or to watch it; omit it and the root owns the signal.",
            },
            Prop {
                name: "default_open",
                ty: "bool",
                default: "false",
                description: "Whether the menu starts open. Ignored when `open` is given.",
            },
            Prop {
                name: "on_open_change",
                ty: "Option<Callback<bool>>",
                default: "None",
                description: "Called with the new value whenever the menu opens or closes, including when the signal passed as `open` is written from outside.",
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
        name: "ContextMenuTrigger",
        description: "The region that answers a right-click, or a long press on touch.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the region's classes.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Ignores the right-click and the long press entirely.",
            },
            Prop {
                name: "long_press",
                ty: "Signal<bool>",
                default: "true",
                description: "Whether a touch or pen press held still opens the menu at the touch point, as a right-click does at the cursor. A mouse press never does. Off removes that timer only: a browser that sends its own `contextmenu` for a long press (Android) still opens the menu, once.",
            },
            Prop {
                name: "long_press_ms",
                ty: "Signal<f64>",
                default: "450.0",
                description: "How long the press is held, in milliseconds. It is abandoned if the finger moves more than 10 pixels, lifts, or is taken by a scroll.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Whatever the menu is about.",
            },
        ],
    },
    ApiEntry {
        name: "ContextMenuContent",
        description: "The menu surface. Anchored to the click point, so it grows down and right with no offset.",
        props: &[
            Prop {
                name: "side",
                ty: "Signal<Side>",
                default: "Side::Bottom",
                description: "One of: Left, Right, Bottom, Top.",
            },
            Prop {
                name: "align",
                ty: "Signal<Align>",
                default: "Align::Start",
                description: "One of: Start, Center, End.",
            },
            Prop {
                name: "side_offset",
                ty: "Signal<SideOffset>",
                default: "SideOffset(0.0)",
                description: "Distance from the pointer, in pixels.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the menu's classes.",
            },
            Prop {
                name: "mobile_sheet",
                ty: "Signal<bool>",
                default: "false",
                description: "Below the `md` breakpoint (768px), present the menu as a bottom sheet over a scrim instead of a menu at the press point: no positioning, `data-presentation=\"sheet\"`. At `md` and up nothing changes, and it is written as `\"floating\"`. Escape, a tap on the scrim, choosing an item and the header's close button close it, and dragging the header down dismisses it.",
            },
            Prop {
                name: "sheet_title",
                ty: "Signal<String>",
                default: "\"\"",
                description: "The title in the sheet's header row, beside the close button. Empty shows no title. Ignored when the content is not a sheet.",
            },
            Prop {
                name: "sheet_snap_points",
                ty: "Signal<Vec<f64>>",
                default: "vec![]",
                description: "The heights the sheet rests at, as fractions of the viewport height — `vec![0.4, 0.9]` is a peek and a tall. The sheet is as tall as the highest and opens at the first; dragging its header settles on the nearest, a flick moves one point, and a release below the lowest dismisses it. `data-snap` carries the active index. Empty is a sheet as tall as its content. Ignored when the content is not a sheet.",
            },
            Prop {
                name: "sheet_show_close",
                ty: "Signal<bool>",
                default: "true",
                description: "Whether the sheet's header has a close button. Present only in a sheet.",
            },
            Prop {
                name: "sheet_close_label",
                ty: "Signal<String>",
                default: "\"Close\"",
                description: "The sheet close button's screen-reader text. Translate it here.",
            },
            Prop {
                name: "sheet_close_icon",
                ty: "Option<ViewFn>",
                default: "X",
                description: "The glyph of the sheet's close button, in place of the cross. Any view; see Icons.",
            },
            Prop {
                name: "sheet_draggable",
                ty: "Signal<bool>",
                default: "true",
                description: "Whether dragging the sheet's header row (grabber and title) moves and dismisses it. Off, the header is inert: no `touch-action: none`, and the scrim, Escape and the close button remain.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Items, labels and separators.",
            },
        ],
    },
];

#[component]
pub fn Page() -> impl IntoView {
    let action = RwSignal::new("nothing yet");

    view! {
        <DocLayout
            title="Context Menu"
            description="A menu summoned by right-clicking a region, or long-pressing it on touch, anchored to the pointer. Set `mobile_sheet` and it becomes a bottom sheet on phones; the Dropdown Menu page shows it."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="The menu opens at the pointer, not at the region. Right-click near the bottom of the window and it flips above the click instead of hanging off the edge. On a touch screen, press and hold: the menu opens at your finger after about half a second, and the browser's own callout stays out of the way."
                    code=DEFAULT
                >
                    <div class="w-full max-w-sm">
                        <ContextMenu>
                            <ContextMenuTrigger class=AREA>"Right-click here"</ContextMenuTrigger>
                            <ContextMenuPortal>
                                <ContextMenuContent>
                                    <ContextMenuItem>"Back"</ContextMenuItem>
                                    <ContextMenuItem>"Forward"</ContextMenuItem>
                                    <ContextMenuItem>"Reload"</ContextMenuItem>
                                </ContextMenuContent>
                            </ContextMenuPortal>
                        </ContextMenu>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Labels, separators and submenus"
                    description="Everything below the trigger is the dropdown menu's, so the arrow keys, the roving focus and the nested surfaces all behave the same way here."
                    code=SUBMENU
                >
                    <div class="w-full max-w-sm">
                        <ContextMenu>
                            <ContextMenuTrigger class=AREA>"Right-click here"</ContextMenuTrigger>
                            <ContextMenuPortal>
                                <ContextMenuContent class="min-w-44">
                                    <ContextMenuLabel>"Layer"</ContextMenuLabel>
                                    <ContextMenuItem>"Rename"</ContextMenuItem>
                                    <ContextMenuItem>"Duplicate"</ContextMenuItem>
                                    <ContextMenuSeparator />
                                    <ContextMenuSub>
                                        <ContextMenuSubTrigger>"Move to"</ContextMenuSubTrigger>
                                        <ContextMenuSubContent>
                                            <ContextMenuItem>"Front"</ContextMenuItem>
                                            <ContextMenuItem>"Forward"</ContextMenuItem>
                                            <ContextMenuItem>"Backward"</ContextMenuItem>
                                            <ContextMenuItem>"Back"</ContextMenuItem>
                                        </ContextMenuSubContent>
                                    </ContextMenuSub>
                                    <ContextMenuSeparator />
                                    <ContextMenuItem>"Delete"</ContextMenuItem>
                                </ContextMenuContent>
                            </ContextMenuPortal>
                        </ContextMenu>
                    </div>
                </DemoSection>

                <DemoSection
                    title="On phones"
                    description="Set mobile_sheet and, below the md breakpoint (768px), the menu is a bottom sheet instead of a menu at the press point. A submenu opens in place, under its trigger, rather than beside it: tap Move to to expand it and tap again to fold it. Checkbox and radio items work as they do in the menu. From md up it is the floating menu; narrow the window to see it."
                    code=ON_PHONES
                >
                    {
                        let show_grid = RwSignal::new(true);
                        let zoom = RwSignal::new("100".to_string());
                        view! {
                            <div class="w-full max-w-sm">
                                <ContextMenu>
                                    <ContextMenuTrigger class=AREA>
                                        "Right-click or press and hold"
                                    </ContextMenuTrigger>
                                    <ContextMenuPortal>
                                        <ContextMenuContent
                                            class="min-w-44"
                                            mobile_sheet=true
                                            sheet_title="Layer"
                                        >
                                            <ContextMenuItem>"Rename"</ContextMenuItem>
                                            <ContextMenuItem>"Duplicate"</ContextMenuItem>
                                            <ContextMenuSub>
                                                <ContextMenuSubTrigger>"Move to"</ContextMenuSubTrigger>
                                                <ContextMenuSubContent>
                                                    <ContextMenuItem>"Front"</ContextMenuItem>
                                                    <ContextMenuItem>"Forward"</ContextMenuItem>
                                                    <ContextMenuItem>"Backward"</ContextMenuItem>
                                                    <ContextMenuItem>"Back"</ContextMenuItem>
                                                </ContextMenuSubContent>
                                            </ContextMenuSub>
                                            <ContextMenuSeparator />
                                            <ContextMenuCheckboxItem checked=show_grid>
                                                "Show grid"
                                            </ContextMenuCheckboxItem>
                                            <ContextMenuSeparator />
                                            <ContextMenuLabel>"Zoom"</ContextMenuLabel>
                                            <ContextMenuRadioGroup value=zoom>
                                                <ContextMenuRadioItem value="50">
                                                    "50%"
                                                </ContextMenuRadioItem>
                                                <ContextMenuRadioItem value="100">
                                                    "100%"
                                                </ContextMenuRadioItem>
                                                <ContextMenuRadioItem value="200">
                                                    "200%"
                                                </ContextMenuRadioItem>
                                            </ContextMenuRadioGroup>
                                            <ContextMenuSeparator />
                                            <ContextMenuItem>"Delete"</ContextMenuItem>
                                        </ContextMenuContent>
                                    </ContextMenuPortal>
                                </ContextMenu>
                            </div>
                        }
                    }
                </DemoSection>

                <DemoSection
                    title="Choosing an item"
                    description="Selecting an item closes the menu, including from inside a submenu."
                    code=SELECTION
                >
                    <div class="flex w-full max-w-sm flex-col gap-2">
                        <ContextMenu>
                            <ContextMenuTrigger class=AREA>"Right-click here"</ContextMenuTrigger>
                            <ContextMenuPortal>
                                <ContextMenuContent>
                                    <ContextMenuItem on_click=Callback::new(move |_| {
                                        action.set("Cut")
                                    })>"Cut"</ContextMenuItem>
                                    <ContextMenuItem on_click=Callback::new(move |_| {
                                        action.set("Copy")
                                    })>"Copy"</ContextMenuItem>
                                    <ContextMenuItem on_click=Callback::new(move |_| {
                                        action.set("Paste")
                                    })>"Paste"</ContextMenuItem>
                                </ContextMenuContent>
                            </ContextMenuPortal>
                        </ContextMenu>
                        <span class="text-sm text-muted-foreground">
                            "Chose: " {move || action.get()}
                        </span>
                    </div>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
