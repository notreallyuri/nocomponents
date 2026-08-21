use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
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
    <DrawerContent side=Side::Right>
        // ...
    </DrawerContent>
</Drawer>"#;

const NESTED: &str = r#"// A drawer inside a drawer. Each has its own dialog context, so the
// layer stack unwinds one Escape at a time and dragging the inner
// panel leaves the outer where it is.
<Drawer>
    <DrawerTrigger>"Open outer"</DrawerTrigger>
    <DrawerContent>
        <DrawerHeader>
            <DrawerTitle>"Choose a project"</DrawerTitle>
        </DrawerHeader>
        <Drawer>
            <DrawerTrigger>"New project"</DrawerTrigger>
            <DrawerContent>
                <DrawerHeader>
                    <DrawerTitle>"New project"</DrawerTitle>
                </DrawerHeader>
            </DrawerContent>
        </Drawer>
    </DrawerContent>
</Drawer>"#;

const NON_MODAL: &str = r#"// No overlay, no focus trap, and the page underneath stays usable.
// A pointer landing outside closes it, since there is no overlay to
// click. Set it once on the root; the content reads it from context.
<Drawer modal=false>
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
                            <DrawerContent side=Side::Right>
                                <DrawerHeader>
                                    <DrawerTitle>"Filters"</DrawerTitle>
                                    <DrawerDescription>"Throw it rightward to dismiss."</DrawerDescription>
                                </DrawerHeader>
                            </DrawerContent>
                        </Drawer>

                        <Drawer side=Side::Top>
                            <DrawerTrigger class="rounded-lg border border-border px-3 py-1.5 text-sm">
                                "From the top"
                            </DrawerTrigger>
                            <DrawerContent side=Side::Top>
                                <DrawerHeader>
                                    <DrawerTitle>"Notifications"</DrawerTitle>
                                    <DrawerDescription>"Throw it upward to dismiss."</DrawerDescription>
                                </DrawerHeader>
                            </DrawerContent>
                        </Drawer>

                        <Drawer side=Side::Left>
                            <DrawerTrigger class="rounded-lg border border-border px-3 py-1.5 text-sm">
                                "From the left"
                            </DrawerTrigger>
                            <DrawerContent side=Side::Left>
                                <DrawerHeader>
                                    <DrawerTitle>"Navigation"</DrawerTitle>
                                    <DrawerDescription>"Throw it leftward to dismiss."</DrawerDescription>
                                </DrawerHeader>
                            </DrawerContent>
                        </Drawer>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Nested"
                    description="A drawer opened from inside another. Each has its own dialog context, so Escape unwinds them one at a time and dragging the inner panel leaves the outer one where it is."
                    code=NESTED
                >
                    <Drawer>
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
                            <Drawer>
                                <DrawerTrigger class="w-full rounded-lg border border-border px-3 py-1.5 text-sm">
                                    "New project"
                                </DrawerTrigger>
                                <DrawerContent>
                                    <DrawerHeader>
                                        <DrawerTitle>"New project"</DrawerTitle>
                                        <DrawerDescription>
                                            "Escape closes this one and leaves the one behind it open."
                                        </DrawerDescription>
                                    </DrawerHeader>
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
                    <Drawer modal=false>
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
            </div>
        </DocLayout>
    }
}
