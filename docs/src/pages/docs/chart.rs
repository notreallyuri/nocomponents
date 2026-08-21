use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::{
    components::chart::Chart,
    primitives::chart::{ChartKind, ChartSeries},
};

const MONTHS: [&str; 6] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun"];

const DEFAULT: &str = r#"let labels = Signal::derive(|| MONTHS.iter().map(|m| m.to_string()).collect());
let series = Signal::derive(|| vec![
    ChartSeries::new("desktop", "Desktop", vec![186.0, 305.0, 237.0, 273.0, 209.0, 264.0]),
    ChartSeries::new("mobile", "Mobile", vec![80.0, 200.0, 120.0, 190.0, 130.0, 140.0]),
]);

view! { <Chart labels=labels series=series /> }"#;

const BAR: &str = r#"<Chart labels=labels series=series kind=ChartKind::Bar />"#;

const AREA: &str = r#"// One series needs no key, and a colour can be anything CSS understands.
<Chart
    labels=labels
    series=Signal::derive(|| vec![
        ChartSeries::new("visitors", "Visitors", vec![…]).color("var(--chart-2)"),
    ])
    kind=ChartKind::Area
    legend=false
/>"#;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "Chart",
        description: "A chart: the plot, the key and the tooltip. Drawn as SVG, so it takes the theme's colours, scales with its container without a resize listener, and prints.",
        props: &[
            Prop {
                name: "labels",
                ty: "Signal<Vec<String>>",
                default: "",
                description: "The categories along the bottom. Each series' values line up with these by index.",
            },
            Prop {
                name: "series",
                ty: "Signal<Vec<ChartSeries>>",
                default: "",
                description: "The lines, areas or sets of bars.",
            },
            Prop {
                name: "kind",
                ty: "ChartKind",
                default: "Line",
                description: "`Line`, `Area` or `Bar`. Line and area put their points on the edges of the plot; bars sit in the middle of a band, which is the difference between a value at a moment and a value for a period.",
            },
            Prop {
                name: "tooltip",
                ty: "bool",
                default: "true",
                description: "The card that follows the pointer, with every series' value at the category nearest it.",
            },
            Prop {
                name: "legend",
                ty: "bool",
                default: "true",
                description: "The key beneath the plot. Turn it off for a single series, which needs none.",
            },
            Prop {
                name: "grid",
                ty: "bool",
                default: "true",
                description: "The horizontal grid and its value labels.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the chart's classes; where to change the `aspect-[2/1]` the plot keeps.",
            },
        ],
    },
    ApiEntry {
        name: "ChartSeries",
        description: "One series, built with `ChartSeries::new(key, label, values)` and optionally `.color(…)`. Not a component — a value you put in a `Vec`.",
        props: &[
            Prop {
                name: "key",
                ty: "String",
                default: "",
                description: "What the series is called in the markup, as `data-series`. Unique within the chart.",
            },
            Prop {
                name: "label",
                ty: "String",
                default: "",
                description: "What the key and the tooltip call it.",
            },
            Prop {
                name: "color",
                ty: "String",
                default: "\"\"",
                description: "Any CSS colour. Left empty, the series takes the theme colour for its position — `--chart-1` through `--chart-5`.",
            },
            Prop {
                name: "values",
                ty: "Vec<f64>",
                default: "",
                description: "One value per label, in the same order.",
            },
        ],
    },
];

#[component]
pub fn Page() -> impl IntoView {
    let labels = Signal::derive(|| MONTHS.iter().map(|month| month.to_string()).collect());

    let two = Signal::derive(|| {
        vec![
            ChartSeries::new(
                "desktop",
                "Desktop",
                vec![186.0, 305.0, 237.0, 273.0, 209.0, 264.0],
            ),
            ChartSeries::new(
                "mobile",
                "Mobile",
                vec![80.0, 200.0, 120.0, 190.0, 130.0, 140.0],
            ),
        ]
    });

    let one = Signal::derive(|| {
        vec![
            ChartSeries::new(
                "visitors",
                "Visitors",
                vec![1240.0, 1580.0, 1420.0, 2100.0, 1890.0, 2340.0],
            )
            .color("var(--chart-2)"),
        ]
    });

    view! {
        <DocLayout
            title="Chart"
            description="Lines, areas and bars, drawn as SVG."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Line"
                    description="No plotting library and no canvas: a chart is a handful of paths, and the moment it is SVG it inherits the theme's colours and scales with its container. Move the pointer across it — the tooltip reads the category nearest the pointer, which is the one thing here that is measured."
                    code=DEFAULT
                >
                    <Chart labels=labels series=two />
                </DemoSection>

                <DemoSection
                    title="Bar"
                    description="Bars are grouped inside their category's band, with a gap between bands so the grouping reads. Hovering dims the other categories rather than highlighting one, which keeps the colours honest."
                    code=BAR
                >
                    <Chart labels=labels series=two kind=ChartKind::Bar />
                </DemoSection>

                <DemoSection
                    title="Area"
                    description="A line with the space below it filled. One series needs no key, and the axis rounds itself up to a number a person would have chosen — never to exactly the largest value, which leaves no headroom and reads as an accident."
                    code=AREA
                >
                    <Chart labels=labels series=one kind=ChartKind::Area legend=false />
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
