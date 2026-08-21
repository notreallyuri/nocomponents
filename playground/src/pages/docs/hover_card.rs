use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
use leptos::prelude::*;
use nocomponents::{
    components::{
        avatar::{Avatar, AvatarFallback, AvatarImage},
        button::{Button, ButtonSize, ButtonVariant},
        hover_card::{HoverCard, HoverCardContent, HoverCardTrigger},
    },
    utils::types::Side,
};

const AVATAR_URL: &str = "https://avatars.githubusercontent.com/u/124599?v=4";

const DEFAULT: &str = r#"<HoverCard>
    <HoverCardTrigger>
        <span class="underline decoration-dotted underline-offset-4">"@yuri"</span>
    </HoverCardTrigger>
    <HoverCardContent>
        <div class="flex gap-3">
            <Avatar>
                <AvatarImage src=AVATAR_URL alt="yuri" class="object-cover" />
                <AvatarFallback>"YR"</AvatarFallback>
            </Avatar>
            <div class="flex flex-col gap-1">
                <p class="font-medium">"@yuri"</p>
                <p class="text-muted-foreground">"Building a Leptos component library."</p>
            </div>
        </div>
    </HoverCardContent>
</HoverCard>"#;

const INTERACTIVE: &str = r#"<HoverCardContent>
    <p class="mb-3 text-muted-foreground">"The card keeps pointer events, so this button is reachable."</p>
    <Button size=ButtonSize::Sm on_click=Callback::new(move |_| follows.update(|n| *n += 1))>
        "Follow"
    </Button>
</HoverCardContent>"#;

const SIDES: &str = r#"<HoverCard>
    <HoverCardTrigger render=… />
    <HoverCardContent side=Side::Right>"Anchored to the right"</HoverCardContent>
</HoverCard>"#;

#[component]
pub fn Page() -> impl IntoView {
    let follows = RwSignal::new(0);

    view! {
        <DocLayout
            title="Hover Card"
            description="A card of detail that opens on hover — and can be reached."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="Slower to open than a tooltip and slower to close, because the pointer has to be able to cross the gap into the card."
                    code=DEFAULT
                >
                    <p class="text-sm">
                        "Maintained by " <HoverCard>
                            <HoverCardTrigger>
                                <span class="underline decoration-dotted underline-offset-4">
                                    "@yuri"
                                </span>
                            </HoverCardTrigger>
                            <HoverCardContent>
                                <div class="flex gap-3">
                                    <Avatar>
                                        <AvatarImage
                                            src=AVATAR_URL
                                            alt="yuri"
                                            class="object-cover"
                                        />
                                        <AvatarFallback>"YR"</AvatarFallback>
                                    </Avatar>
                                    <div class="flex flex-col gap-1">
                                        <p class="font-medium">"@yuri"</p>
                                        <p class="text-muted-foreground">
                                            "Building a Leptos component library."
                                        </p>
                                    </div>
                                </div>
                            </HoverCardContent>
                        </HoverCard> "."
                    </p>
                </DemoSection>

                <DemoSection
                    title="Interactive content"
                    description="This is the whole difference from a tooltip: the card takes pointer events, and hovering it cancels the close the trigger scheduled."
                    code=INTERACTIVE
                >
                    <div class="flex items-center gap-3">
                        <HoverCard>
                            <HoverCardTrigger render=Callback::new(move |node_ref| {
                                view! {
                                    <Button
                                        node_ref=node_ref
                                        variant=ButtonVariant::Outline
                                        size=ButtonSize::Sm
                                    >
                                        "Hover for actions"
                                    </Button>
                                }
                                    .into_any()
                            }) />
                            <HoverCardContent>
                                <p class="mb-3 text-muted-foreground">
                                    "The card keeps pointer events, so this button is reachable."
                                </p>
                                <Button
                                    size=ButtonSize::Sm
                                    on_click=Callback::new(move |_| follows.update(|n| *n += 1))
                                >
                                    "Follow"
                                </Button>
                            </HoverCardContent>
                        </HoverCard>
                        <span class="text-sm text-muted-foreground">
                            "Followed " {move || follows.get()} " times"
                        </span>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Sides"
                    description="Anchored like every other floating layer, flipping when it would leave the viewport."
                    code=SIDES
                >
                    <div class="flex flex-wrap gap-3">
                        <HoverCard>
                            <HoverCardTrigger render=Callback::new(move |node_ref| {
                                view! {
                                    <Button
                                        node_ref=node_ref
                                        variant=ButtonVariant::Outline
                                        size=ButtonSize::Sm
                                    >
                                        "Right"
                                    </Button>
                                }
                                    .into_any()
                            }) />
                            <HoverCardContent side=Side::Right>
                                "Anchored to the right of its trigger."
                            </HoverCardContent>
                        </HoverCard>
                        <HoverCard>
                            <HoverCardTrigger render=Callback::new(move |node_ref| {
                                view! {
                                    <Button
                                        node_ref=node_ref
                                        variant=ButtonVariant::Outline
                                        size=ButtonSize::Sm
                                    >
                                        "Top"
                                    </Button>
                                }
                                    .into_any()
                            }) />
                            <HoverCardContent side=Side::Top>
                                "Anchored above its trigger."
                            </HoverCardContent>
                        </HoverCard>
                    </div>
                </DemoSection>
            </div>
        </DocLayout>
    }
}
