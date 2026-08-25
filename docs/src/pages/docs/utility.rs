//! The utilities page: everything the library ships that is not a component.

use crate::{app::href, components::demo_section::DemoSection, layout::doc_layout::DocLayout};
use leptos::prelude::*;
use leptos_router::components::A;
use nocomponents::{
    components::{
        code::CodeBlock,
        separator::Separator,
        table::{Table, TableBody, TableCell, TableHead, TableHeader, TableRow},
    },
    utils::types::Language,
};

const CN: &str = r#"use nocomponents::cn;

// Joins the classes, then runs tw_merge over the result: px-3 loses to px-6,
// and everything that does not collide is left where it was.
let class = cn!("px-3 py-1.5 text-sm", "px-6");

// The idiom every styled component is written in. The caller's class comes last,
// so passing one overrides rather than fights.
view! {
    <div class=move || cn!("rounded-lg border bg-card p-4", class.get())>
        {children()}
    </div>
}"#;

const TYPES: &str = r#"use nocomponents::utils::types::{Align, Orientation, Side};

// Rendered into the attribute, never into a class name.
view! {
    <div
        data-side=side.get().as_str()
        data-align=align.get().as_str()
        data-orientation=orientation.get().as_str()
        class="data-[side=top]:mb-2 data-[orientation=vertical]:flex-col"
    />
}"#;

const AS_CLASS: &str = r#"use nocomponents::utils::types::AsClass;

#[derive(Default, Clone, Copy, PartialEq)]
pub enum CalloutVariant {
    #[default]
    Note,
    Warning,
}

impl AsClass for CalloutVariant {
    fn as_class(&self) -> &'static str {
        match self {
            CalloutVariant::Note => "border-border bg-muted/40",
            CalloutVariant::Warning => "border-destructive/40 bg-destructive/10",
        }
    }
}

// Which is all a variant prop ever has to do:
class=move || cn!("rounded-lg border p-4", variant.get().as_class(), class.get())"#;

const IDS: &str = r#"use nocomponents::utils::next_id;

// "nc-tooltip-7" — unique for the life of the page.
let id = next_id("tooltip");

view! {
    <button aria-describedby=id.clone()>"Save"</button>
    <div role="tooltip" id=id>"Writes the file to disk"</div>
}"#;

const MEDIA_QUERY: &str = r#"use nocomponents::utils::use_media_query;

let is_phone = use_media_query("(max-width: 767px)");

// A dialog that becomes a drawer is two components, not one component with a
// different width — so this is a decision CSS cannot make for you.
view! {
    <Show
        when=move || is_phone.get()
        fallback=|| view! { <Dialog>{...}</Dialog> }
    >
        <Drawer>{...}</Drawer>
    </Show>
}"#;

const PLACEMENT: &str = r#"use nocomponents::utils::{get_placement, types::{Align, Side}};

// Placement::BottomStart, ready for floating-ui's middleware.
let placement = get_placement(Side::Bottom, Align::Start);"#;

const DATE: &str = r#"use nocomponents::utils::date::{month_grid, Date, DateNames};

let today = Date::today();

today.to_iso();                    // "2026-08-25"
today.weekday();                   // 0 is Sunday
Date::new(2026, 1, 31).add_months(1);   // 2026-02-28, clamped, not 03-03
Date::days_in_month(2026, 2);      // 28

// Always 42 days: six weeks, so paging through the year never changes the
// grid's height. week_starts_on is 0 for Sunday, 1 for Monday.
let grid = month_grid(today, 1);

// Resolved once out of Intl, then kept — a grid reads 42 weekday names.
let names = DateNames::new("pt-BR");
names.month(2);                    // "fevereiro", 1-12 like Date::month
names.weekday_abbreviation(1);     // "seg"
names.is_localized();              // false when there was no Intl to ask"#;

const COLOR: &str = r##"use nocomponents::utils::color::{parse_color, ColorFormat, Hsva, Rgba};

// Reads hex, rgb(), hsl(), oklch(), hsv() and the CSS colour names.
let blue = parse_color("oklch(0.623 0.188 259.8)").unwrap();

ColorFormat::Hex.format(blue);     // "#3b82f6"
ColorFormat::Hsl.format(blue);     // "hsl(217 91% 60%)"

// Hold HSVA, convert outward to display. Going the other way loses the hue in
// the corners: black has no hue to recover, so a picker that stores RGB watches
// its rail snap back to red.
let picked: Hsva = blue.into();
let back: Rgba = picked.into();"##;

const IMAGE: &str = r#"use nocomponents::utils::image::{crop_image, Crop};

// Fractions of the image, never pixels: the <img> on screen is whatever width
// the layout says this second, and the output wants the natural size.
let crop = Crop::centered(16.0 / 9.0).moved_by(0.0, -0.05);

crop.to_pixels(natural_width, natural_height);   // when you do need pixels

// `source` is the original bytes, and is what makes an animated GIF come back
// animated. Without them a canvas copies one frame and calls it a crop.
let out = crop_image(&img, Some(&bytes), crop, "image/png", 0.92)?;
out.mime;    // read off the output, not echoed back from the argument"#;

const GIF: &str = r#"use nocomponents::utils::gif::{crop_gif, frame_count, is_gif, size};

if is_gif(&bytes) {
    size(&bytes);           // Some((width, height)), straight out of the header
    frame_count(&bytes)?;   // whether it is animated at all
    crop_gif(&bytes, crop)?;
}"#;

const HIGHLIGHT: &str = r#"use nocomponents::utils::{highlight::tokenize, types::Language};

// Tokens cover the input exactly: concatenating every token's text gives the
// source back byte for byte, so a rendered block cannot drop a character.
for token in tokenize(source, Language::Rust) {
    let class = token.kind.as_str();   // "keyword", "string", "comment", ...
    let text = token.text;
}"#;

const FEATURES: &str = r#"[dependencies]
# cn!, the common types, next_id, use_media_query, dates, colour,
# crops and GIFs — all of it, with no features at all.
nocomponents = "0.1"

# tokenize() and the theme provider are the two that are gated.
nocomponents = { version = "0.1", features = ["highlight", "theme"] }"#;

struct TypeRow {
    name: &'static str,
    values: &'static str,
    note: &'static str,
}

const COMMON_TYPES: &[TypeRow] = &[
    TypeRow {
        name: "Side",
        values: "Top, Right, Bottom, Left",
        note: "Which side of its trigger a floating layer sits on. Default Bottom.",
    },
    TypeRow {
        name: "Align",
        values: "Start, Center, End",
        note: "Where along that side it lines up. Default Start.",
    },
    TypeRow {
        name: "Orientation",
        values: "Horizontal, Vertical",
        note: "Which way a group of items runs, and which arrow keys move through it.",
    },
    TypeRow {
        name: "State",
        values: "Open, Closed",
        note: "What data-state carries on anything that opens.",
    },
    TypeRow {
        name: "Direction",
        values: "Left, Right",
        note: "Which way a carousel or a slide moves. Default Right.",
    },
    TypeRow {
        name: "SideOffset",
        values: "SideOffset(f64)",
        note: "The gap between a trigger and its floating layer, in pixels. Default 4.0.",
    },
    TypeRow {
        name: "TriggerRect",
        values: "width, height, x, y",
        note: "A measured trigger, for the layers that size themselves to it.",
    },
    TypeRow {
        name: "Language",
        values: "Rust, Shell, JavaScript, TypeScript, Html, Css, Plain",
        note: "What CodeBlock is told it is rendering. Default Rust.",
    },
];

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout
            title="Utilities"
            description="The parts of the library that are not components: class merging, the shared types, and the arithmetic behind the pickers."
        >
            <div class="flex flex-col gap-8">
                <p class="text-sm leading-relaxed text-muted-foreground">
                    "Everything on this page is an ordinary dependency. The styled components are "
                    <A
                        href=href("/docs/installation")
                        attr:class="font-medium text-foreground underline underline-offset-4"
                    >
                        "copied into your project"
                    </A>
                    " because Tailwind has to see the file a class is written in; none of this has
                    classes in it, so none of it needs copying."
                </p>
                <p class="text-sm leading-relaxed text-muted-foreground">
                    "The dates, the colour conversions, the crop rectangle and the GIF encoder are
                    here rather than pulled from crates for one reason: a UI library should not
                    hand its consumers a "
                    <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">
                        "chrono"
                    </code> " version to agree with. They are all arithmetic, and all covered by "
                    <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">
                        "cargo test --features full"
                    </code> "."
                </p>

                <Separator />

                <DemoSection
                    title="cn!"
                    description="Joins classes and merges the conflicts, so the class a caller passes wins over the one the component wrote."
                >
                    <div class="flex flex-col gap-4">
                        <CodeBlock code=CN language=Language::Rust />
                        <p class="text-sm leading-relaxed text-muted-foreground">
                            "Exported at the crate root — "
                            <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">
                                "use nocomponents::cn"
                            </code>
                            " — and it reaches tw_merge through this crate rather than directly, so an
                            installed component needs one dependency instead of two kept in lockstep."
                        </p>
                        <div class="rounded-lg border bg-muted/40 p-4">
                            <p class="text-xs leading-relaxed text-muted-foreground">
                                <span class="font-medium text-foreground">
                                    "The merger is coarser than the JavaScript one."
                                </span>
                                " Notably "
                                <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">
                                    "overflow-x-*"
                                </code>
                                " and "
                                <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">
                                    "overflow-y-*"
                                </code>
                                " are treated as one group, so writing both keeps only the last. Write
                                the axis you mean, or reach for a single "
                                <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">
                                    "overflow-*"
                                </code>
                                "."
                            </p>
                        </div>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Common types"
                    description="The enums the primitives and the components share. Every one of them has as_str(), because their real destination is a data attribute."
                >
                    <div class="flex flex-col gap-4">
                        <CodeBlock code=TYPES language=Language::Rust />
                        <Table class="table-fixed text-xs">
                            <TableHeader>
                                <TableRow>
                                    <TableHead class="w-[18%] whitespace-normal">"Type"</TableHead>
                                    <TableHead class="w-[32%] whitespace-normal">
                                        "Values"
                                    </TableHead>
                                    <TableHead class="whitespace-normal">"What it says"</TableHead>
                                </TableRow>
                            </TableHeader>
                            <TableBody>
                                {COMMON_TYPES
                                    .iter()
                                    .map(|row| {
                                        view! {
                                            <TableRow>
                                                <TableCell class="align-top font-mono whitespace-normal text-foreground">
                                                    {row.name}
                                                </TableCell>
                                                <TableCell class="align-top font-mono break-words whitespace-normal text-muted-foreground">
                                                    {row.values}
                                                </TableCell>
                                                <TableCell class="align-top whitespace-normal text-muted-foreground">
                                                    {row.note}
                                                </TableCell>
                                            </TableRow>
                                        }
                                    })
                                    .collect_view()}
                            </TableBody>
                        </Table>
                    </div>
                </DemoSection>

                <DemoSection
                    title="AsClass"
                    description="One method returning a &'static str of Tailwind classes. Every variant prop in the styled layer is an enum implementing it."
                >
                    <div class="flex flex-col gap-4">
                        <CodeBlock code=AS_CLASS language=Language::Rust />
                        <p class="text-sm leading-relaxed text-muted-foreground">
                            "The classes have to be literals: Tailwind scans the source for strings and
                            cannot follow a name that is assembled at runtime. A match arm per variant
                            is what makes them literal."
                        </p>
                    </div>
                </DemoSection>

                <Separator />

                <DemoSection
                    title="next_id"
                    description="A document-unique id, for the aria-* attributes that have to point at another element."
                >
                    <div class="flex flex-col gap-4">
                        <CodeBlock code=IDS language=Language::Rust />
                        <p class="text-sm leading-relaxed text-muted-foreground">
                            "Components mint their own rather than asking for one: "
                            <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">
                                "aria-labelledby"
                            </code>
                            " needs an id on the label whether or not anybody wanted to write one."
                        </p>
                    </div>
                </DemoSection>

                <DemoSection
                    title="use_media_query"
                    description="Whether a query matches, as a signal, kept up to date as the window changes."
                >
                    <div class="flex flex-col gap-4">
                        <CodeBlock code=MEDIA_QUERY language=Language::Rust />
                        <p class="text-sm leading-relaxed text-muted-foreground">
                            "For the handful of decisions a class cannot make, where the alternative is
                            rendering both and hiding one — which leaves two of everything in the DOM
                            and two of everything focusable. It re-reads the query on every resize, so
                            what it says and what CSS is doing cannot drift apart."
                        </p>
                    </div>
                </DemoSection>

                <DemoSection
                    title="get_placement"
                    description="Maps a Side and an Align onto floating-ui's twelve placements."
                >
                    <div class="flex flex-col gap-4">
                        <CodeBlock code=PLACEMENT language=Language::Rust />
                        <p class="text-sm leading-relaxed text-muted-foreground">
                            "Every floating component takes the pair rather than the placement, because
                            the pair is what "
                            <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">
                                "data-side"
                            </code> " and "
                            <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">
                                "data-align"
                            </code> " are written from."
                        </p>
                    </div>
                </DemoSection>

                <Separator />

                <DemoSection
                    title="Dates"
                    description="A 12-byte civil date and the arithmetic a calendar needs. No clock, no zones, no dependency."
                >
                    <div class="flex flex-col gap-4">
                        <CodeBlock code=DATE language=Language::Rust />
                        <p class="text-sm leading-relaxed text-muted-foreground">
                            "The conversions to and from a day count are Howard Hinnant's, so stepping
                            over a month end or a leap day is never a special case. "
                            <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">
                                "DateNames"
                            </code>
                            " resolves out of the browser's Intl once and keeps the strings — a month
                            grid reads a weekday name 42 times, and a formatter call each time would be
                            42 trips into JavaScript."
                        </p>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Colour"
                    description="Rgba, Hsva, Hsla and Oklch, with a parser and a formatter between them."
                >
                    <div class="flex flex-col gap-4">
                        <CodeBlock code=COLOR language=Language::Rust />
                        <p class="text-sm leading-relaxed text-muted-foreground">
                            <span class="font-medium text-foreground">
                                "Hsva is the canonical value, not Rgba."
                            </span>
                            " A saturation square with a hue rail beside it is HSV, and the round trip
                            through RGB is lossy in exactly the place it hurts. Oklch is here because
                            the theme's own tokens are written in it, so a picker that could not read "
                            <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">
                                "oklch(0.646 0.222 41.116)"
                            </code>
                            " could not edit the palette it is themed by."
                        </p>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Crops and images"
                    description="A crop rectangle in normalised coordinates, and the one canvas call that turns it into a file."
                >
                    <div class="flex flex-col gap-4">
                        <CodeBlock code=IMAGE language=Language::Rust />
                        <p class="text-sm leading-relaxed text-muted-foreground">
                            "The rectangle is in fractions of the image — 0 at the left or top edge, 1
                            at the right or bottom — and never in pixels. The responsive "
                            <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">
                                "<img>"
                            </code>
                            " on screen, the natural size the output wants, and a value stored in a
                            form all want different pixels; one set of coordinates survives all three."
                        </p>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Animated GIFs"
                    description="Cropping a GIF frame by frame at the LZW level, because a canvas cannot."
                >
                    <div class="flex flex-col gap-4">
                        <CodeBlock code=GIF language=Language::Rust />
                        <p class="text-sm leading-relaxed text-muted-foreground">
                            <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">
                                "drawImage"
                            </code>
                            " copies whichever frame the "
                            <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">
                                "<img>"
                            </code>
                            " is showing, and "
                            <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">
                                "toDataURL(\"image/gif\")"
                            </code>
                            " quietly hands back a PNG. So the frames are cropped as palette indices
                            and never become pixels: no quantisation, and local palettes, transparency,
                            delays and disposal methods all survive untouched. "
                            <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">
                                "crop_image"
                            </code>
                            " routes GIFs here itself, so a caller writes one call."
                        </p>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Syntax highlighting"
                    description="The tokenizer behind CodeBlock. A lexer, not a parser."
                >
                    <div class="flex flex-col gap-4">
                        <CodeBlock code=HIGHLIGHT language=Language::Rust />
                        <p class="text-sm leading-relaxed text-muted-foreground">
                            "Behind the "
                            <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">
                                "highlight"
                            </code>
                            " feature. Without it a CodeBlock still renders, unhighlighted, and nothing
                            else changes."
                        </p>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Theming"
                    description="ThemeProvider, use_theme, and the four axes they carry."
                >
                    <p class="text-sm leading-relaxed text-muted-foreground">
                        "The mode, the palette, the accent and the corner radius are four
                        independent things, and they live behind the "
                        <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">
                            "theme"
                        </code> " feature in "
                        <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">
                            "utils::theme"
                        </code> ". They have "
                        <A
                            href=href("/docs/theme")
                            attr:class="font-medium text-foreground underline underline-offset-4"
                        >
                            "a page of their own"
                        </A> "."
                    </p>
                </DemoSection>

                <Separator />

                <DemoSection
                    title="What ships with what"
                    description="utils is the one module that exists with no features enabled at all."
                >
                    <div class="flex flex-col gap-4">
                        <CodeBlock code=FEATURES language=Language::Rust label="Cargo.toml" />
                        <p class="text-sm leading-relaxed text-muted-foreground">
                            "The layer features — "
                            <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">
                                "primitives"
                            </code> ", "
                            <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">
                                "components"
                            </code> ", "
                            <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">
                                "icons"
                            </code>
                            " — gate the components. They do not gate any of this, with the two
                            exceptions above."
                        </p>
                    </div>
                </DemoSection>
            </div>
        </DocLayout>
    }
}
