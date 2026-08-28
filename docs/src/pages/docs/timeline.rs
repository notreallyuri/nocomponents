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
        scroll_area::ScrollArea,
        timeline::{
            Timeline, TimelineConnector, TimelineContent, TimelineDescription, TimelineItem,
            TimelineMarker, TimelineTime, TimelineTitle,
        },
    },
    icons::check::Check,
    primitives::timeline::{TimelineSide, TimelineState},
    utils::types::Orientation,
};

const DEFAULT: &str = r#"<Timeline>
    <TimelineItem>
        <TimelineMarker />
        <TimelineConnector />
        <TimelineContent>
            <TimelineTime datetime="2026-03-14">"14 March"</TimelineTime>
            <TimelineTitle>"Repository created"</TimelineTitle>
            <TimelineDescription>"The first commit, and nothing else."</TimelineDescription>
        </TimelineContent>
    </TimelineItem>
    …
</Timeline>"#;

const STEPS: &str = r#"// The state is what turns a record of the past into a thing in progress.
// `Current` is the only one that writes `aria-current="step"`.
<TimelineItem state=TimelineState::Complete>…</TimelineItem>
<TimelineItem state=TimelineState::Current>…</TimelineItem>
<TimelineItem state=TimelineState::Upcoming>…</TimelineItem>"#;

const MARKERS: &str = r#"// A marker with children is not decorative, so it keeps its own place in the
// reading order. Without them it is a dot, and hidden from assistive tech.
<TimelineMarker class="size-7">"1"</TimelineMarker>
<TimelineMarker class="size-7"><Check class="size-3.5" /></TimelineMarker>"#;

struct Event {
    time: &'static str,
    datetime: &'static str,
    title: &'static str,
    description: &'static str,
}

const HISTORY: &[Event] = &[
    Event {
        time: "14 March",
        datetime: "2026-03-14",
        title: "Repository created",
        description: "The first commit, and nothing else in it.",
    },
    Event {
        time: "2 April",
        datetime: "2026-04-02",
        title: "The primitives land",
        description: "Behaviour, ARIA and data attributes, with no classes anywhere near them.",
    },
    Event {
        time: "19 May",
        datetime: "2026-05-19",
        title: "A styled layer over the top",
        description: "The same tree again, carrying nothing but Tailwind.",
    },
    Event {
        time: "8 August",
        datetime: "2026-08-08",
        title: "Installed rather than depended on",
        description: "A Tailwind build cannot see inside another crate, so the CLI copies the files in.",
    },
];

const STEP_LIST: &[(&str, &str, TimelineState)] = &[
    (
        "Order placed",
        "Paid for and confirmed.",
        TimelineState::Complete,
    ),
    ("Packed", "Everything in one box.", TimelineState::Complete),
    (
        "In transit",
        "Somewhere between the warehouse and you.",
        TimelineState::Current,
    ),
    ("Delivered", "Not yet.", TimelineState::Upcoming),
];

const ALTERNATE: &str = r#"// Which side an event falls on is derived from its index, not set per item:
// a zig-zag a caller keeps in step by hand stops zig-zagging the moment one
// event is inserted.
<Timeline alternate=true>…</Timeline>"#;

const HORIZONTAL: &str = r#"// The axis runs left to right and the content drops below it. Items share the
// width evenly; wrap it in a ScrollArea if there are more than fit.
<Timeline orientation=Orientation::Horizontal>…</Timeline>"#;

const OVERFLOW_VERTICAL: &str = r#"// A timeline does not scroll itself — that is one component's job each.
<ScrollArea class="h-72 rounded-lg border">
    <Timeline class="p-4">…</Timeline>
</ScrollArea>"#;

const OVERFLOW_HORIZONTAL: &str = r#"// Horizontally the items need a width of their own, or they share the
// viewport's and never overflow at all.
<ScrollArea horizontal=true class="rounded-lg border">
    <Timeline orientation=Orientation::Horizontal class="min-w-max p-4">
        <TimelineItem class="w-56 flex-none">…</TimelineItem>
    </Timeline>
</ScrollArea>"#;

const SIDE: &str = r#"// Which side content sits on, over either axis: left/right when the
// timeline runs down, above/below when it runs across.
<Timeline side=TimelineSide::Start>…</Timeline>
<Timeline orientation=Orientation::Horizontal side=TimelineSide::Start>…</Timeline>"#;

const RELEASES: &[(&str, &str, &str)] = &[
    (
        "v0.1.0",
        "2026-01-08",
        "The first primitives, and nothing styled.",
    ),
    (
        "v0.2.0",
        "2026-02-02",
        "A styled layer, and the floating stack under it.",
    ),
    (
        "v0.3.0",
        "2026-03-14",
        "Dismiss, roving focus and typeahead land together.",
    ),
    (
        "v0.4.0",
        "2026-04-09",
        "The colour utilities, and a picker over them.",
    ),
    ("v0.5.0", "2026-05-19", "Crops, and GIFs that survive one."),
    (
        "v0.6.0",
        "2026-06-11",
        "The data table, and sorting it does not own.",
    ),
    (
        "v0.7.0",
        "2026-07-03",
        "A board of columns, on the same drag as everything else.",
    ),
    (
        "v0.8.0",
        "2026-07-28",
        "Themes on four axes rather than one.",
    ),
    (
        "v0.9.0",
        "2026-08-15",
        "The CLI, and a styled layer you install.",
    ),
    (
        "v0.10.0",
        "2026-08-26",
        "A video player, and a timeline to put it on.",
    ),
];

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "Timeline",
        description: "The sequence, as an `<ol>` — a list of events is what this is, so a screen reader is told how many there are before any styling matters. Events sit evenly spaced in document order and are never positioned by their dates: a timeline where March and December sit proportionally apart is a chart, not this.",
        props: &[
            Prop {
                name: "orientation",
                ty: "Orientation",
                default: "Vertical",
                description: "Which way the axis runs. Horizontal drops the content below the axis and shares the width evenly between events; it overflows rather than wraps.",
            },
            Prop {
                name: "alternate",
                ty: "bool",
                default: "false",
                description: "Whether events fall on alternating sides of the axis — left and right when the timeline runs down, above and below when it runs across. It starts on whichever side `side` names and flips from there.",
            },
            Prop {
                name: "side",
                ty: "TimelineSide",
                default: "End",
                description: "Which side of the axis content sits on: `Start` is left when vertical and above when horizontal, `End` the other way. Also where an alternating timeline begins.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the list's classes. Nothing here scrolls — wrap it in a `ScrollArea` and give that the height.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The items.",
            },
        ],
    },
    ApiEntry {
        name: "TimelineItem",
        description: "One event, as an `<li>`. A two-column grid: the marker and connector in the first, the content spanning both rows of the second — so the caller's markup stays flat instead of needing a wrapper around the axis. Items count themselves, which is how the last one knows it is last.",
        props: &[
            Prop {
                name: "state",
                ty: "TimelineState",
                default: "Complete",
                description: "`Complete`, `Current` or `Upcoming`. Defaults to complete because the common timeline is a record of things that already happened. `Current` writes `aria-current=\"step\"`.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the item's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "A marker, a connector, and the content.",
            },
        ],
    },
    ApiEntry {
        name: "TimelineMarker",
        description: "The dot on the axis, or whatever you put in it — a number, a tick, an avatar. Empty it is `aria-hidden`, since the item already carries the state and a screen reader reading \"bullet\" before every event helps nobody; with children it keeps its place in the reading order.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the marker's classes. `size-7` or so when it holds a number or an icon.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "What sits in the marker. A bare dot without.",
            },
        ],
    },
    ApiEntry {
        name: "TimelineConnector",
        description: "The line from this event to the next. A part you place rather than something the item draws, so a pending stretch can be dashed where a finished one is solid. It knows when it is last and hides itself, because a connector after the final event points at nothing — and `:last-child` stops being true the moment an item is wrapped in anything.",
        props: &[Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Merged over the connector's classes.",
        }],
    },
    ApiEntry {
        name: "TimelineContent",
        description: "The body of the event, spanning both rows beside the axis. It carries the item's state too, which is what dims an upcoming event without the caller styling each part.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the content's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "A time, a title, a description, or anything else.",
            },
        ],
    },
    ApiEntry {
        name: "TimelineTime",
        description: "A real `<time>`, with the machine-readable form in `datetime`. The two are deliberately separate: what a reader wants is \"3 days ago\", what a machine wants is `2026-03-14`, and neither is derivable from the other here.",
        props: &[
            Prop {
                name: "datetime",
                ty: "Signal<String>",
                default: "\"\"",
                description: "The `datetime` attribute. Omitted, the element is still a `<time>`; only the machine-readable half is missing.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the element's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "What the reader sees.",
            },
        ],
    },
    ApiEntry {
        name: "TimelineTitle",
        description: "The event's heading. Style-only, with no primitive behind it.",
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
                description: "The heading.",
            },
        ],
    },
    ApiEntry {
        name: "TimelineDescription",
        description: "The event's supporting line. Style-only, with no primitive behind it.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the description's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The text.",
            },
        ],
    },
];

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout title="Timeline" description="A sequence of events along an axis.">
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="An `<ol>` of events, evenly spaced in document order. The marker and the connector sit in the grid's first column and the content spans both of its rows, so nothing has to be wrapped to make the axis line up."
                    code=DEFAULT
                >
                    <div class="mx-auto w-full max-w-md">
                        <Timeline>
                            {HISTORY
                                .iter()
                                .map(|event| {
                                    view! {
                                        <TimelineItem>
                                            <TimelineMarker />
                                            <TimelineConnector />
                                            <TimelineContent>
                                                <TimelineTime datetime=event
                                                    .datetime>{event.time}</TimelineTime>
                                                <TimelineTitle>{event.title}</TimelineTitle>
                                                <TimelineDescription>
                                                    {event.description}
                                                </TimelineDescription>
                                            </TimelineContent>
                                        </TimelineItem>
                                    }
                                })
                                .collect_view()}
                        </Timeline>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Something in progress"
                    description="The state is what turns a record of the past into a thing still happening. `Current` is the only one that writes `aria-current=\"step\"`, and upcoming events dim themselves — the content reads the state too, so a caller does not style each part."
                    code=STEPS
                >
                    <div class="mx-auto w-full max-w-md">
                        <Timeline>
                            {STEP_LIST
                                .iter()
                                .map(|&(title, description, state)| {
                                    view! {
                                        <TimelineItem state=state>
                                            <TimelineMarker />
                                            <TimelineConnector />
                                            <TimelineContent>
                                                <TimelineTitle>{title}</TimelineTitle>
                                                <TimelineDescription>{description}</TimelineDescription>
                                            </TimelineContent>
                                        </TimelineItem>
                                    }
                                })
                                .collect_view()}
                        </Timeline>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Markers of your own"
                    description="What goes in the marker is content: a number, a tick, an avatar. A marker with children keeps its place in the reading order; an empty one is a dot and hidden from assistive tech, the item having said everything already."
                    code=MARKERS
                >
                    <div class="mx-auto w-full max-w-md">
                        <Timeline>
                            {STEP_LIST
                                .iter()
                                .enumerate()
                                .map(|(index, &(title, description, state))| {
                                    view! {
                                        <TimelineItem state=state>
                                            <TimelineMarker class="mt-0 size-7">
                                                {if state == TimelineState::Complete {
                                                    view! { <Check class="size-3.5" /> }.into_any()
                                                } else {
                                                    view! { {index + 1} }.into_any()
                                                }}
                                            </TimelineMarker>
                                            <TimelineConnector />
                                            <TimelineContent class="pt-1">
                                                <TimelineTitle>{title}</TimelineTitle>
                                                <TimelineDescription>{description}</TimelineDescription>
                                            </TimelineContent>
                                        </TimelineItem>
                                    }
                                })
                                .collect_view()}
                        </Timeline>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Alternating"
                    description="A centre axis with events falling either side of it. Which side is derived from the item's own index rather than set per item — a zig-zag a caller keeps in step by hand stops zig-zagging the moment one event is inserted. The content on the near side is right-aligned so both columns read towards the axis."
                    code=ALTERNATE
                >
                    <div class="mx-auto w-full max-w-xl">
                        <Timeline alternate=true>
                            {HISTORY
                                .iter()
                                .map(|event| {
                                    view! {
                                        <TimelineItem>
                                            <TimelineMarker />
                                            <TimelineConnector />
                                            <TimelineContent>
                                                <TimelineTime datetime=event
                                                    .datetime>{event.time}</TimelineTime>
                                                <TimelineTitle>{event.title}</TimelineTitle>
                                                <TimelineDescription>
                                                    {event.description}
                                                </TimelineDescription>
                                            </TimelineContent>
                                        </TimelineItem>
                                    }
                                })
                                .collect_view()}
                        </Timeline>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Horizontal"
                    description="The axis runs left to right and the content drops below it, with the events sharing the width evenly. It overflows rather than wraps, so anything that will not fit belongs in a `ScrollArea` — that is one component's job each rather than a mode in this one."
                    code=HORIZONTAL
                >
                    <div class="w-full">
                        <Timeline orientation=Orientation::Horizontal>
                            {STEP_LIST
                                .iter()
                                .map(|&(title, description, state)| {
                                    view! {
                                        <TimelineItem state=state>
                                            <TimelineMarker />
                                            <TimelineConnector />
                                            <TimelineContent>
                                                <TimelineTitle>{title}</TimelineTitle>
                                                <TimelineDescription>{description}</TimelineDescription>
                                            </TimelineContent>
                                        </TimelineItem>
                                    }
                                })
                                .collect_view()}
                        </Timeline>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Which side the content takes"
                    description="One idea over both axes: before the axis means left when the timeline runs down and above when it runs across. An alternating timeline starts on this side and flips from there, so `side` and `alternate` compose rather than competing."
                    code=SIDE
                >
                    <div class="mx-auto w-full max-w-md">
                        <Timeline side=TimelineSide::Start>
                            {HISTORY
                                .iter()
                                .take(3)
                                .map(|event| {
                                    view! {
                                        <TimelineItem>
                                            <TimelineMarker />
                                            <TimelineConnector />
                                            <TimelineContent>
                                                <TimelineTime datetime=event
                                                    .datetime>{event.time}</TimelineTime>
                                                <TimelineTitle>{event.title}</TimelineTitle>
                                            </TimelineContent>
                                        </TimelineItem>
                                    }
                                })
                                .collect_view()}
                        </Timeline>
                    </div>
                </DemoSection>

                <DemoSection
                    title="More events than fit"
                    description="The timeline lays out and stops there — it does not scroll itself, because scrolling is `ScrollArea`'s job and doing it in both places would mean two scrollbars disagreeing. Give the area a height and the list overflows into it."
                    code=OVERFLOW_VERTICAL
                >
                    <div class="mx-auto w-full max-w-md">
                        <ScrollArea class="h-72 rounded-lg border">
                            <Timeline class="p-4">
                                {RELEASES
                                    .iter()
                                    .map(|&(version, datetime, note)| {
                                        view! {
                                            <TimelineItem>
                                                <TimelineMarker />
                                                <TimelineConnector />
                                                <TimelineContent>
                                                    <TimelineTime datetime=datetime>{datetime}</TimelineTime>
                                                    <TimelineTitle>{version}</TimelineTitle>
                                                    <TimelineDescription>{note}</TimelineDescription>
                                                </TimelineContent>
                                            </TimelineItem>
                                        }
                                    })
                                    .collect_view()}
                            </Timeline>
                        </ScrollArea>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Wider than it fits"
                    description="The same again on its side, and with one difference that matters: horizontal events share the width evenly by default, so they shrink rather than overflow. Give them a width of their own and the row grows past the viewport, which is what there is to scroll."
                    code=OVERFLOW_HORIZONTAL
                >
                    <ScrollArea horizontal=true class="w-full rounded-lg border">
                        <Timeline orientation=Orientation::Horizontal class="min-w-max p-4">
                            {RELEASES
                                .iter()
                                .map(|&(version, datetime, note)| {
                                    view! {
                                        <TimelineItem class="w-56 flex-none">
                                            <TimelineMarker />
                                            <TimelineConnector />
                                            <TimelineContent>
                                                <TimelineTime datetime=datetime>{datetime}</TimelineTime>
                                                <TimelineTitle>{version}</TimelineTitle>
                                                <TimelineDescription>{note}</TimelineDescription>
                                            </TimelineContent>
                                        </TimelineItem>
                                    }
                                })
                                .collect_view()}
                        </Timeline>
                    </ScrollArea>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
