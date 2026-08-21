//! A small chart: lines, areas and bars, drawn as SVG.
//!
//! Everything here is arithmetic and one `<svg>`. There is no plotting library behind it and no
//! canvas — a chart in a UI library is a handful of paths, and the moment it is SVG it inherits
//! the theme's colours, scales with the container, prints, and can be read by a screen reader
//! through the table it is described by.
//!
//! The geometry is computed in viewBox units and the SVG scales as a whole, so nothing has to be
//! measured and there is no resize listener: the same numbers describe the chart at any size.
//! What *is* measured is the pointer, once, to turn an x position into the index it is nearest —
//! which is all a tooltip needs.

use leptos::{context::Provider, ev, prelude::*};
use leptos_node_ref::AnyNodeRef;

/// The drawing area, in viewBox units. Uniform scaling means these are the only numbers in the
/// file that are arbitrary.
pub const VIEW_WIDTH: f64 = 600.0;
pub const VIEW_HEIGHT: f64 = 300.0;
const PAD_LEFT: f64 = 44.0;
const PAD_RIGHT: f64 = 12.0;
const PAD_TOP: f64 = 12.0;
const PAD_BOTTOM: f64 = 28.0;

/// How many grid lines the value axis is divided into.
const TICKS: usize = 4;

/// The five theme colours a series falls back to, in order.
pub const SERIES_COLORS: [&str; 5] = [
    "var(--chart-1)",
    "var(--chart-2)",
    "var(--chart-3)",
    "var(--chart-4)",
    "var(--chart-5)",
];

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum ChartKind {
    #[default]
    Line,
    /// A line with the space below it filled.
    Area,
    /// One bar per series per category, grouped.
    Bar,
}

/// One line, area or set of bars. `values` lines up with the chart's labels by index.
#[derive(Clone, PartialEq)]
pub struct ChartSeries {
    pub key: String,
    pub label: String,
    /// Any CSS colour. Empty takes the theme colour for this series' position.
    pub color: String,
    pub values: Vec<f64>,
}

impl ChartSeries {
    pub fn new(key: impl Into<String>, label: impl Into<String>, values: Vec<f64>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            color: String::new(),
            values,
        }
    }

    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = color.into();
        self
    }

    /// The colour to draw with: the one given, or this position's theme colour.
    pub fn resolved_color(&self, index: usize) -> String {
        if self.color.is_empty() {
            SERIES_COLORS[index % SERIES_COLORS.len()].to_string()
        } else {
            self.color.clone()
        }
    }
}

#[derive(Copy, Clone)]
pub struct ChartContext {
    /// The categories along the bottom.
    pub labels: Signal<Vec<String>>,
    pub series: Signal<Vec<ChartSeries>>,
    pub kind: ChartKind,
    /// The category the pointer is nearest, if it is over the plot.
    pub hovered: RwSignal<Option<usize>>,
    pub plot_ref: AnyNodeRef,
}

/// Rounds a maximum up to something a person would choose for the top of an axis: 1, 2, 2.5 or 5
/// times a power of ten. An axis topped at exactly the data's maximum has no headroom and its
/// labels are unreadable numbers.
pub fn nice_ceiling(max: f64) -> f64 {
    if max <= 0.0 {
        return 1.0;
    }

    let magnitude = 10f64.powf(max.log10().floor());
    for step in [1.0, 2.0, 2.5, 5.0, 10.0] {
        let candidate = step * magnitude;
        if candidate >= max {
            return candidate;
        }
    }

    max
}

impl ChartContext {
    /// The largest value across every series, rounded up to a readable axis top.
    pub fn max(&self) -> f64 {
        let max = self.series.with(|series| {
            series
                .iter()
                .flat_map(|series| series.values.iter().copied())
                .fold(0.0f64, f64::max)
        });

        nice_ceiling(max)
    }

    pub fn count(&self) -> usize {
        self.labels.with(|labels| labels.len())
    }

    pub fn plot_width(&self) -> f64 {
        VIEW_WIDTH - PAD_LEFT - PAD_RIGHT
    }

    pub fn plot_height(&self) -> f64 {
        VIEW_HEIGHT - PAD_TOP - PAD_BOTTOM
    }

    pub fn bottom(&self) -> f64 {
        VIEW_HEIGHT - PAD_BOTTOM
    }

    /// Where a category sits along the bottom. Points sit on the edges for a line and in the
    /// middle of a band for bars, which is the difference between "a value at this moment" and "a
    /// value for this period".
    pub fn x(&self, index: usize) -> f64 {
        let count = self.count();
        match self.kind {
            ChartKind::Bar => {
                let band = self.plot_width() / count.max(1) as f64;
                PAD_LEFT + band * index as f64 + band / 2.0
            }
            _ if count > 1 => PAD_LEFT + self.plot_width() * index as f64 / (count - 1) as f64,
            _ => PAD_LEFT + self.plot_width() / 2.0,
        }
    }

    pub fn y(&self, value: f64) -> f64 {
        let max = self.max();
        PAD_TOP + self.plot_height() * (1.0 - (value / max).clamp(0.0, 1.0))
    }

    /// The value each grid line stands for, from the top down.
    pub fn ticks(&self) -> Vec<f64> {
        let max = self.max();
        (0..=TICKS)
            .map(|tick| max * (TICKS - tick) as f64 / TICKS as f64)
            .collect()
    }

    /// `M … L …` through a series' points.
    pub fn line_path(&self, series: &ChartSeries) -> String {
        series
            .values
            .iter()
            .enumerate()
            .map(|(index, value)| {
                let command = if index == 0 { 'M' } else { 'L' };
                format!("{command}{:.2} {:.2}", self.x(index), self.y(*value))
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// The same path, closed along the bottom.
    pub fn area_path(&self, series: &ChartSeries) -> String {
        if series.values.is_empty() {
            return String::new();
        }

        let last = series.values.len() - 1;
        format!(
            "{} L{:.2} {:.2} L{:.2} {:.2} Z",
            self.line_path(series),
            self.x(last),
            self.bottom(),
            self.x(0),
            self.bottom()
        )
    }

    /// One bar: `(x, y, width, height)`. Bars are grouped inside their category's band, with a
    /// gap between bands so the grouping reads.
    pub fn bar(&self, index: usize, series_index: usize, value: f64) -> (f64, f64, f64, f64) {
        let count = self.count().max(1);
        let series_count = self.series.with(|series| series.len()).max(1);

        let band = self.plot_width() / count as f64;
        let group = band * 0.72;
        let width = group / series_count as f64;
        let left = PAD_LEFT + band * index as f64 + (band - group) / 2.0;

        let y = self.y(value);

        (
            left + width * series_index as f64,
            y,
            width.max(1.0),
            (self.bottom() - y).max(0.0),
        )
    }

    /// The category nearest a pointer, from its position in the SVG's own box. The one place
    /// anything is measured.
    pub fn index_at(&self, client_x: f64) -> Option<usize> {
        let plot = self.plot_ref.get_untracked()?;
        let rect = plot.get_bounding_client_rect();
        if rect.width() <= 0.0 {
            return None;
        }

        let count = self.count();
        if count == 0 {
            return None;
        }

        // Back out of the browser's pixels into viewBox units, which is where every other number
        // in this file lives.
        let x = (client_x - rect.left()) / rect.width() * VIEW_WIDTH;

        let index = match self.kind {
            ChartKind::Bar => {
                let band = self.plot_width() / count as f64;
                ((x - PAD_LEFT) / band).floor()
            }
            _ if count > 1 => {
                let step = self.plot_width() / (count - 1) as f64;
                ((x - PAD_LEFT) / step).round()
            }
            _ => 0.0,
        };

        Some((index.max(0.0) as usize).min(count - 1))
    }

    /// Where a category sits across the plot, as a percentage — what an HTML tooltip needs, since
    /// it is positioned in the container rather than in the SVG.
    pub fn x_percent(&self, index: usize) -> f64 {
        self.x(index) / VIEW_WIDTH * 100.0
    }
}

pub fn use_chart() -> ChartContext {
    expect_context::<ChartContext>()
}

#[component]
pub fn ChartRoot(
    #[prop(into)] labels: Signal<Vec<String>>,
    #[prop(into)] series: Signal<Vec<ChartSeries>>,
    #[prop(optional)] kind: ChartKind,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = ChartContext {
        labels,
        series,
        kind,
        hovered: RwSignal::new(None),
        plot_ref: AnyNodeRef::new(),
    };

    view! {
        <Provider value=ctx>
            <div
                data-slot="chart"
                data-kind=match kind {
                    ChartKind::Line => "line",
                    ChartKind::Area => "area",
                    ChartKind::Bar => "bar",
                }
                class=class
            >
                {children()}
            </div>
        </Provider>
    }
}

/// The drawing itself.
#[component]
pub fn ChartPlotRoot(
    #[prop(optional, into)] class: Signal<String>,
    /// Whether to draw the horizontal grid and its value labels.
    #[prop(default = true)]
    grid: bool,
) -> impl IntoView {
    let ctx = use_chart();

    view! {
        <svg
            node_ref=ctx.plot_ref
            data-slot="chart-plot"
            viewBox=format!("0 0 {VIEW_WIDTH} {VIEW_HEIGHT}")
            role="img"
            on:pointermove=move |e: ev::PointerEvent| {
                ctx.hovered.set(ctx.index_at(e.client_x() as f64));
            }
            on:pointerleave=move |_| ctx.hovered.set(None)
            class=class
        >
            {grid
                .then(|| {
                    view! {
                        <g data-slot="chart-grid">
                            {move || {
                                ctx.ticks()
                                    .into_iter()
                                    .map(|value| {
                                        let y = ctx.y(value);
                                        view! {
                                            <line
                                                x1=PAD_LEFT
                                                x2=VIEW_WIDTH - PAD_RIGHT
                                                y1=y
                                                y2=y
                                                class="stroke-border"
                                                stroke-width="1"
                                            />
                                            <text
                                                x=PAD_LEFT - 8.0
                                                y=y + 4.0
                                                text-anchor="end"
                                                class="fill-muted-foreground text-[10px]"
                                            >
                                                {format_tick(value)}
                                            </text>
                                        }
                                    })
                                    .collect_view()
                            }}
                        </g>
                    }
                })}

            <g data-slot="chart-series">
                {move || {
                    ctx.series
                        .get()
                        .into_iter()
                        .enumerate()
                        .map(|(index, series)| {
                            let color = series.resolved_color(index);
                            match ctx.kind {
                                ChartKind::Bar => {
                                    series
                                        .values
                                        .iter()
                                        .enumerate()
                                        .map(|(category, value)| {
                                            let (x, y, width, height) = ctx.bar(category, index, *value);
                                            view! {
                                                <rect
                                                    x=x
                                                    y=y
                                                    width=width
                                                    height=height
                                                    rx="2"
                                                    fill=color.clone()
                                                    data-series=series.key.clone()
                                                    class="transition-opacity"
                                                    opacity=move || {
                                                        match ctx.hovered.get() {
                                                            Some(hovered) if hovered != category => "0.5",
                                                            _ => "1",
                                                        }
                                                    }
                                                />
                                            }
                                        })
                                        .collect_view()
                                        .into_any()
                                }
                                kind => {
                                    let area = (kind == ChartKind::Area)
                                        .then(|| {
                                            view! {
                                                <path
                                                    d=ctx.area_path(&series)
                                                    fill=color.clone()
                                                    opacity="0.15"
                                                />
                                            }
                                        });

                                    view! {
                                        {area}
                                        <path
                                            d=ctx.line_path(&series)
                                            fill="none"
                                            stroke=color.clone()
                                            stroke-width="2"
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            data-series=series.key.clone()
                                        />
                                        // The point under the pointer, and only that one: a dot on
                                        // every value turns a line chart into a bead necklace.
                                        {move || {
                                            let hovered = ctx.hovered.get()?;
                                            let value = *series.values.get(hovered)?;
                                            Some(
                                                view! {
                                                    <circle
                                                        cx=ctx.x(hovered)
                                                        cy=ctx.y(value)
                                                        r="4"
                                                        fill=color.clone()
                                                        class="stroke-background"
                                                        stroke-width="2"
                                                    />
                                                },
                                            )
                                        }}
                                    }
                                        .into_any()
                                }
                            }
                        })
                        .collect_view()
                }}
            </g>

            <g data-slot="chart-axis">
                {move || {
                    let count = ctx.count();
                    ctx.labels
                        .get()
                        .into_iter()
                        .enumerate()
                        .map(|(index, label)| {
                            view! {
                                <text
                                    x=ctx.x(index)
                                    y=VIEW_HEIGHT - 8.0
                                    text-anchor=match (index, count) {
                                        (0, _) if ctx.kind != ChartKind::Bar => "start",
                                        (index, count) if index + 1 == count
                                            && ctx.kind != ChartKind::Bar => "end",
                                        _ => "middle",
                                    }
                                    class="fill-muted-foreground text-[10px]"
                                >
                                    {label}
                                </text>
                            }
                        })
                        .collect_view()
                }}
            </g>

            // The line that follows the pointer, drawn last so it sits over the series.
            {move || {
                let hovered = ctx.hovered.get()?;
                Some(
                    view! {
                        <line
                            x1=ctx.x(hovered)
                            x2=ctx.x(hovered)
                            y1=PAD_TOP
                            y2=ctx.bottom()
                            class="stroke-muted-foreground"
                            stroke-width="1"
                            stroke-dasharray="3 3"
                        />
                    },
                )
            }}
        </svg>
    }
}

/// Trims a tick's decimals without turning 1500 into "2k" — an axis that rounds its own labels
/// lies about the grid line beside it.
fn format_tick(value: f64) -> String {
    if value.fract().abs() < f64::EPSILON {
        format!("{value:.0}")
    } else {
        format!("{value:.1}")
    }
}

/// The card that follows the pointer. Positioned as a percentage across the chart, so it needs no
/// measuring of its own.
#[component]
pub fn ChartTooltipRoot(
    #[prop(optional, into)] class: Signal<String>,
    /// One row of the card, handed the series and its value at the hovered category.
    row: Callback<(ChartSeries, f64), AnyView>,
) -> impl IntoView {
    let ctx = use_chart();

    view! {
        <Show when=move || ctx.hovered.get().is_some()>
            {move || {
                let hovered = ctx.hovered.get()?;
                let label = ctx.labels.with(|labels| labels.get(hovered).cloned())?;

                Some(
                    view! {
                        <div
                            data-slot="chart-tooltip"
                            role="tooltip"
                            style=format!(
                                "left: {:.2}%; transform: translateX(-50%);",
                                ctx.x_percent(hovered),
                            )
                            class=class
                        >
                            <div data-slot="chart-tooltip-label">{label}</div>
                            {ctx
                                .series
                                .get()
                                .into_iter()
                                .enumerate()
                                .filter_map(|(index, series)| {
                                    let value = *series.values.get(hovered)?;
                                    let mut series = series;
                                    series.color = series.resolved_color(index);
                                    Some(row.run((series, value)))
                                })
                                .collect_view()}
                        </div>
                    },
                )
            }}
        </Show>
    }
}

/// The key: one swatch per series.
#[component]
pub fn ChartLegendRoot(
    #[prop(optional, into)] class: Signal<String>,
    /// One entry, handed the series with its colour already resolved.
    item: Callback<ChartSeries, AnyView>,
) -> impl IntoView {
    let ctx = use_chart();

    view! {
        <div data-slot="chart-legend" class=class>
            {move || {
                ctx.series
                    .get()
                    .into_iter()
                    .enumerate()
                    .map(|(index, mut series)| {
                        series.color = series.resolved_color(index);
                        item.run(series)
                    })
                    .collect_view()
            }}
        </div>
    }
}
