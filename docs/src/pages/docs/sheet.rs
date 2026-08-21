use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
use leptos::prelude::*;
use nocomponents::{
    components::{
        button::{Button, ButtonVariant},
        input::Input,
        label::Label,
        sheet::{
            Sheet, SheetClose, SheetContent, SheetDescription, SheetFooter, SheetHeader,
            SheetTitle, SheetTrigger,
        },
    },
    utils::types::Side,
};

const DEFAULT: &str = r#"<Sheet>
    <SheetTrigger variant=ButtonVariant::Outline>"Open sheet"</SheetTrigger>
    <SheetContent>
        <SheetHeader>
            <SheetTitle>"Edit profile"</SheetTitle>
            <SheetDescription>"Change your details and save when you are done."</SheetDescription>
        </SheetHeader>
        <div class="flex flex-col gap-3">
            <div class="flex flex-col gap-1.5">
                <Label>"Name"</Label>
                <Input attr:placeholder="Your name" />
            </div>
        </div>
        <SheetFooter>
            <SheetClose>"Cancel"</SheetClose>
            <Button>"Save"</Button>
        </SheetFooter>
    </SheetContent>
</Sheet>"#;

const SIDES: &str = r#"<Sheet>
    <SheetTrigger variant=ButtonVariant::Outline>"Left"</SheetTrigger>
    <SheetContent side=Side::Left>
        <SheetHeader>
            <SheetTitle>"From the left"</SheetTitle>
        </SheetHeader>
    </SheetContent>
</Sheet>"#;

const CONTROLLED: &str = r#"<Button on_click=Callback::new(move |_| open.set(true))>"Open"</Button>
<Sheet open=open>
    <SheetContent side=Side::Bottom>
        <SheetHeader>
            <SheetTitle>"Controlled"</SheetTitle>
            <SheetDescription>"Open state managed externally."</SheetDescription>
        </SheetHeader>
    </SheetContent>
</Sheet>"#;

#[component]
pub fn Page() -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <DocLayout title="Sheet" description="A dialog that arrives from an edge.">
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="The dialog primitive underneath, so Escape, the focus trap and the overlay all behave the same — a sheet only changes which edge it is pinned to."
                    code=DEFAULT
                >
                    <Sheet>
                        <SheetTrigger variant=ButtonVariant::Outline>"Open sheet"</SheetTrigger>
                        <SheetContent>
                            <SheetHeader>
                                <SheetTitle>"Edit profile"</SheetTitle>
                                <SheetDescription>
                                    "Change your details and save when you are done."
                                </SheetDescription>
                            </SheetHeader>
                            <div class="flex flex-col gap-3">
                                <div class="flex flex-col gap-1.5">
                                    <Label>"Name"</Label>
                                    <Input attr:placeholder="Your name" />
                                </div>
                                <div class="flex flex-col gap-1.5">
                                    <Label>"Username"</Label>
                                    <Input attr:placeholder="@handle" />
                                </div>
                            </div>
                            <SheetFooter>
                                <SheetClose>"Cancel"</SheetClose>
                                <Button>"Save"</Button>
                            </SheetFooter>
                        </SheetContent>
                    </Sheet>
                </DemoSection>

                <DemoSection
                    title="Sides"
                    description="Each side pins the sheet to that edge and slides it in from there."
                    code=SIDES
                >
                    <div class="flex flex-wrap gap-3">
                        <Sheet>
                            <SheetTrigger variant=ButtonVariant::Outline>"Left"</SheetTrigger>
                            <SheetContent side=Side::Left>
                                <SheetHeader>
                                    <SheetTitle>"From the left"</SheetTitle>
                                    <SheetDescription>"Pinned to the left edge."</SheetDescription>
                                </SheetHeader>
                            </SheetContent>
                        </Sheet>
                        <Sheet>
                            <SheetTrigger variant=ButtonVariant::Outline>"Right"</SheetTrigger>
                            <SheetContent side=Side::Right>
                                <SheetHeader>
                                    <SheetTitle>"From the right"</SheetTitle>
                                    <SheetDescription>"The default."</SheetDescription>
                                </SheetHeader>
                            </SheetContent>
                        </Sheet>
                        <Sheet>
                            <SheetTrigger variant=ButtonVariant::Outline>"Top"</SheetTrigger>
                            <SheetContent side=Side::Top>
                                <SheetHeader>
                                    <SheetTitle>"From the top"</SheetTitle>
                                    <SheetDescription>
                                        "Full width, height from content."
                                    </SheetDescription>
                                </SheetHeader>
                            </SheetContent>
                        </Sheet>
                        <Sheet>
                            <SheetTrigger variant=ButtonVariant::Outline>"Bottom"</SheetTrigger>
                            <SheetContent side=Side::Bottom>
                                <SheetHeader>
                                    <SheetTitle>"From the bottom"</SheetTitle>
                                    <SheetDescription>
                                        "Where a phone would put it."
                                    </SheetDescription>
                                </SheetHeader>
                            </SheetContent>
                        </Sheet>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Controlled"
                    description="Open state managed externally, so any control can raise the sheet."
                    code=CONTROLLED
                >
                    <div class="flex items-center gap-3">
                        <Button on_click=Callback::new(move |_| open.set(true))>"Open"</Button>
                        <span class="text-sm text-muted-foreground">
                            {move || if open.get() { "Open" } else { "Closed" }}
                        </span>
                        <Sheet open=open>
                            <SheetContent side=Side::Bottom>
                                <SheetHeader>
                                    <SheetTitle>"Controlled"</SheetTitle>
                                    <SheetDescription>
                                        "Open state managed externally."
                                    </SheetDescription>
                                </SheetHeader>
                                <SheetFooter>
                                    <SheetClose>"Done"</SheetClose>
                                </SheetFooter>
                            </SheetContent>
                        </Sheet>
                    </div>
                </DemoSection>
            </div>
        </DocLayout>
    }
}
