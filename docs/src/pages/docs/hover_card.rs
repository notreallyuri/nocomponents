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

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "HoverCard",
        description: "A card that opens on hover, whose contents can be reached — unlike a tooltip's.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the wrapper's classes.",
            },
            Prop {
                name: "delay",
                ty: "u64",
                default: "600",
                description: "How long the pointer has to rest before the card opens.",
            },
            Prop {
                name: "close_delay",
                ty: "u64",
                default: "250",
                description: "The grace period after leaving, long enough to cross the gap into the card.",
            },
            Prop {
                name: "children",
                ty: "ChildrenFn",
                default: "",
                description: "A trigger and a content.",
            },
        ],
    },
    ApiEntry {
        name: "HoverCardTrigger",
        description: "What the card is about.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the wrapper's classes.",
            },
            Prop {
                name: "render",
                ty: "Option<Callback<AnyNodeRef, AnyView>>",
                default: "None",
                description: "Render the trigger as something else — a link, a `Button`. Forward the node ref.",
            },
            Prop {
                name: "children",
                ty: "Option<Children>",
                default: "None",
                description: "The trigger's contents.",
            },
        ],
    },
    ApiEntry {
        name: "HoverCardContent",
        description: "The card. Keeps pointer events, so what is in it can be used.",
        props: &[
            Prop {
                name: "side",
                ty: "Side",
                default: "Side::Bottom",
                description: "One of: Left, Right, Bottom, Top.",
            },
            Prop {
                name: "align",
                ty: "Align",
                default: "Start",
                description: "One of: Start, Center, End.",
            },
            Prop {
                name: "side_offset",
                ty: "SideOffset",
                default: "SideOffset(8.0)",
                description: "Distance from the trigger, in pixels.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the card's classes.",
            },
            Prop {
                name: "children",
                ty: "ChildrenFn",
                default: "",
                description: "The card's contents. Re-run on each mount, hence `ChildrenFn`.",
            },
        ],
    },
];

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

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
