use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::components::{
    avatar::{Avatar, AvatarFallback, AvatarImage},
    badge::{Badge, BadgeVariant},
    button::{Button, ButtonSize, ButtonVariant},
    item::{
        Item, ItemActions, ItemContent, ItemDescription, ItemGroup, ItemHeader, ItemMedia,
        ItemMediaVariant, ItemSeparator, ItemSize, ItemTitle, ItemVariant,
    },
    switch::Switch,
};

const AVATAR_URL: &str = "https://w.wallhaven.cc/full/5d/wallhaven-5d23j9.png";

const DEFAULT: &str = r#"<Item variant=ItemVariant::Outline>
    <ItemMedia>
        <Avatar>
            <AvatarImage src=AVATAR_URL alt="Ada" class="object-cover" />
            <AvatarFallback>"AL"</AvatarFallback>
        </Avatar>
    </ItemMedia>
    <ItemContent>
        <ItemTitle>"Ada Lovelace"</ItemTitle>
        <ItemDescription>"ada@analytical.engine"</ItemDescription>
    </ItemContent>
    <ItemActions>
        <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>"Message"</Button>
    </ItemActions>
</Item>"#;

const VARIANTS: &str = r#"<Item>"Default — no border, no fill"</Item>
<Item variant=ItemVariant::Outline>"Outline"</Item>
<Item variant=ItemVariant::Muted>"Muted"</Item>"#;

const GROUP: &str = r#"<ItemGroup>
    <Item size=ItemSize::Sm>…</Item>
    <ItemSeparator />
    <Item size=ItemSize::Sm>…</Item>
</ItemGroup>"#;

const ICON_MEDIA: &str = r#"<Item variant=ItemVariant::Outline>
    <ItemMedia variant=ItemMediaVariant::Icon>"⚡"</ItemMedia>
    <ItemContent>
        <ItemTitle>"Edge functions"</ItemTitle>
        <ItemDescription>"Run code close to your users."</ItemDescription>
    </ItemContent>
    <ItemActions>
        <Switch checked=edge />
    </ItemActions>
</Item>"#;

const HEADER: &str = r#"<Item variant=ItemVariant::Outline>
    <ItemHeader>
        <span class="text-xs text-muted-foreground">"Deploy 4821"</span>
        <Badge variant=BadgeVariant::Secondary>"Live"</Badge>
    </ItemHeader>
    <ItemContent>
        <ItemTitle>"main@8f2c1d0"</ItemTitle>
        <ItemDescription>"Deployed 12 minutes ago by grace."</ItemDescription>
    </ItemContent>
</Item>"#;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "ItemGroup",
        description: "Stacks items, with the spacing decided in one place.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the layout classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Items and separators.",
            },
        ],
    },
    ApiEntry {
        name: "ItemSeparator",
        description: "A rule between two items.",
        props: &[Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Merged over the rule's classes.",
        }],
    },
    ApiEntry {
        name: "Item",
        description: "A row: media on the left, text in the middle, actions on the right.",
        props: &[
            Prop {
                name: "variant",
                ty: "ItemVariant",
                default: "Default",
                description: "One of: Default, Outline, Muted.",
            },
            Prop {
                name: "size",
                ty: "ItemSize",
                default: "Default",
                description: "One of: Default, Sm.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the row's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Media, content, actions, and optionally a header and a footer.",
            },
        ],
    },
    ApiEntry {
        name: "ItemMedia",
        description: "The picture at the left: a tinted tile for an icon, or plain for an avatar.",
        props: &[
            Prop {
                name: "variant",
                ty: "ItemMediaVariant",
                default: "Plain",
                description: "One of: Plain, Icon.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the media's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "An icon, an avatar, a checkbox.",
            },
        ],
    },
    ApiEntry {
        name: "ItemContent",
        description: "The middle: title and description, taking the free space.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the layout classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Usually a title and a description.",
            },
        ],
    },
    ApiEntry {
        name: "ItemTitle",
        description: "The row's name.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the title's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The title text.",
            },
        ],
    },
    ApiEntry {
        name: "ItemDescription",
        description: "The line under the title.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the muted text classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The description text.",
            },
        ],
    },
    ApiEntry {
        name: "ItemActions",
        description: "The right-hand end, for controls.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the layout classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Usually buttons.",
            },
        ],
    },
    ApiEntry {
        name: "ItemHeader",
        description: "A full-width strip above the row's main line.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the strip's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Usually a label or a status.",
            },
        ],
    },
    ApiEntry {
        name: "ItemFooter",
        description: "The same, below.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the strip's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Usually a label or a status.",
            },
        ],
    },
];

#[component]
pub fn Page() -> impl IntoView {
    let edge = RwSignal::new(true);

    view! {
        <DocLayout
            title="Item"
            description="A row of content: media, text, actions — for lists, settings and results."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="ItemContent takes the free space, so the media and the actions stay pinned to the ends."
                    code=DEFAULT
                >
                    <Item variant=ItemVariant::Outline>
                        <ItemMedia>
                            <Avatar>
                                <AvatarImage src=AVATAR_URL alt="Ada" class="object-cover" />
                                <AvatarFallback>"AL"</AvatarFallback>
                            </Avatar>
                        </ItemMedia>
                        <ItemContent>
                            <ItemTitle>"Ada Lovelace"</ItemTitle>
                            <ItemDescription>"ada@analytical.engine"</ItemDescription>
                        </ItemContent>
                        <ItemActions>
                            <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>
                                "Message"
                            </Button>
                        </ItemActions>
                    </Item>
                </DemoSection>

                <DemoSection
                    title="Variants"
                    description="Three weights: transparent, bordered, and filled."
                    code=VARIANTS
                >
                    <div class="flex flex-col gap-3">
                        <Item>"Default — no border, no fill"</Item>
                        <Item variant=ItemVariant::Outline>"Outline"</Item>
                        <Item variant=ItemVariant::Muted>"Muted"</Item>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Grouped"
                    description="ItemGroup stacks rows and ItemSeparator rules between them; the small size suits a dense list."
                    code=GROUP
                >
                    <ItemGroup class="rounded-lg border">
                        <Item size=ItemSize::Sm>
                            <ItemContent>
                                <ItemTitle>"report-q3.pdf"</ItemTitle>
                                <ItemDescription>"2.4 MB · uploaded yesterday"</ItemDescription>
                            </ItemContent>
                            <ItemActions>
                                <Button variant=ButtonVariant::Ghost size=ButtonSize::Sm>
                                    "Download"
                                </Button>
                            </ItemActions>
                        </Item>
                        <ItemSeparator />
                        <Item size=ItemSize::Sm>
                            <ItemContent>
                                <ItemTitle>"budget.xlsx"</ItemTitle>
                                <ItemDescription>"812 KB · uploaded last week"</ItemDescription>
                            </ItemContent>
                            <ItemActions>
                                <Button variant=ButtonVariant::Ghost size=ButtonSize::Sm>
                                    "Download"
                                </Button>
                            </ItemActions>
                        </Item>
                        <ItemSeparator />
                        <Item size=ItemSize::Sm>
                            <ItemContent>
                                <ItemTitle>"notes.md"</ItemTitle>
                                <ItemDescription>"4 KB · uploaded last week"</ItemDescription>
                            </ItemContent>
                            <ItemActions>
                                <Button variant=ButtonVariant::Ghost size=ButtonSize::Sm>
                                    "Download"
                                </Button>
                            </ItemActions>
                        </Item>
                    </ItemGroup>
                </DemoSection>

                <DemoSection
                    title="Icon media"
                    description="The Icon variant boxes the media in a tile, for a settings row rather than a person."
                    code=ICON_MEDIA
                >
                    <Item variant=ItemVariant::Outline>
                        <ItemMedia variant=ItemMediaVariant::Icon>"⚡"</ItemMedia>
                        <ItemContent>
                            <ItemTitle>"Edge functions"</ItemTitle>
                            <ItemDescription>"Run code close to your users."</ItemDescription>
                        </ItemContent>
                        <ItemActions>
                            <Switch checked=edge />
                        </ItemActions>
                    </Item>
                </DemoSection>

                <DemoSection
                    title="With a header"
                    description="ItemHeader spans the full width above the main line, for a label or a status."
                    code=HEADER
                >
                    <Item variant=ItemVariant::Outline>
                        <ItemHeader>
                            <span class="text-xs text-muted-foreground">"Deploy 4821"</span>
                            <Badge variant=BadgeVariant::Secondary>"Live"</Badge>
                        </ItemHeader>
                        <ItemContent>
                            <ItemTitle>"main@8f2c1d0"</ItemTitle>
                            <ItemDescription>"Deployed 12 minutes ago by grace."</ItemDescription>
                        </ItemContent>
                    </Item>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
