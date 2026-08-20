use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
use leptos::prelude::*;
use nocomponents::components::context_menu::{
    ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuLabel, ContextMenuPortal,
    ContextMenuSeparator, ContextMenuSub, ContextMenuSubContent, ContextMenuSubTrigger,
    ContextMenuTrigger,
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

const AREA: &str = "flex h-32 items-center justify-center rounded-lg border border-dashed border-border text-sm text-muted-foreground";

#[component]
pub fn Page() -> impl IntoView {
    let action = RwSignal::new("nothing yet");

    view! {
        <DocLayout
            title="Context Menu"
            description="A menu summoned by right-clicking a region, anchored to the pointer."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="The menu opens at the pointer, not at the region. Right-click near the bottom of the window and it flips above the click instead of hanging off the edge."
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
            </div>
        </DocLayout>
    }
}
