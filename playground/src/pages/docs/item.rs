use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
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
            </div>
        </DocLayout>
    }
}
