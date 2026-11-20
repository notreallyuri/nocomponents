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

pub use nocomponents_core::chart::{
    ChartKind, ChartLayout, ChartSeries, SERIES_COLORS, VIEW_HEIGHT, VIEW_WIDTH, format_tick,
    nice_ceiling,
};

#[derive(Copy, Clone)]
pub struct ChartContext {
    /// The categories along the bottom.
    pub labels: Signal<Vec<String>>,
    pub series: Signal<Vec<ChartSeries>>,
    pub kind: Signal<ChartKind>,
    /// The category the pointer is nearest, if it is over the plot.
    pub hovered: RwSignal<Option<usize>>,
    pub plot_ref: AnyNodeRef,
}

impl ChartContext {
    /// The largest value across every series, rounded up to a readable axis top.
    pub fn max(&self) -> f64 {
        self.series
            .with(|series| ChartLayout::default().max(series))
    }

    pub fn count(&self) -> usize {
        self.labels.with(|labels| labels.len())
    }

    pub fn plot_width(&self) -> f64 {
        ChartLayout::default().plot_width()
    }

    pub fn plot_height(&self) -> f64 {
        ChartLayout::default().plot_height()
    }

    pub fn bottom(&self) -> f64 {
        ChartLayout::default().bottom()
    }

    /// Where a category sits along the bottom. Points sit on the edges for a line and in the
    /// middle of a band for bars, which is the difference between "a value at this moment" and "a
    /// value for this period".
    pub fn x(&self, index: usize) -> f64 {
        ChartLayout::default().x(index, self.count(), self.kind.get())
    }

    pub fn y(&self, value: f64) -> f64 {
        ChartLayout::default().y(value, self.max())
    }

    /// The value each grid line stands for, from the top down.
    pub fn ticks(&self) -> Vec<f64> {
        self.series
            .with(|series| ChartLayout::default().ticks_for(series))
    }

    /// `M … L …` through a series' points.
    pub fn line_path(&self, series: &ChartSeries) -> String {
        ChartLayout::default().line_path(series, self.count(), self.max(), self.kind.get())
    }

    /// The same path, closed along the bottom.
    pub fn area_path(&self, series: &ChartSeries) -> String {
        ChartLayout::default().area_path(series, self.count(), self.max(), self.kind.get())
    }

    /// One bar: `(x, y, width, height)`. Bars are grouped inside their category's band, with a
    /// gap between bands so the grouping reads.
    pub fn bar(&self, index: usize, series_index: usize, value: f64) -> (f64, f64, f64, f64) {
        let series_count = self.series.with(|series| series.len());
        ChartLayout::default().bar(
            index,
            series_index,
            value,
            self.count(),
            series_count,
            self.max(),
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

        // Back out of the browser's pixels into viewBox units, which is where every other number
        // in this file lives.
        let layout = ChartLayout::default();
        let x = (client_x - rect.left()) / rect.width() * layout.width;
        layout.index_at(x, self.count(), self.kind.get_untracked())
    }

    /// Where a category sits across the plot, as a percentage — what an HTML tooltip needs, since
    /// it is positioned in the container rather than in the SVG.
    pub fn x_percent(&self, index: usize) -> f64 {
        ChartLayout::default().x_percent(index, self.count(), self.kind.get())
    }
}

pub fn use_chart() -> ChartContext {
    expect_context::<ChartContext>()
}

#[component]
pub fn ChartRoot(
    #[prop(into)] labels: Signal<Vec<String>>,
    #[prop(into)] series: Signal<Vec<ChartSeries>>,
    #[prop(optional, into)] kind: Signal<ChartKind>,
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
                data-kind=move || match kind.get() {
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
    #[prop(into, default = true.into())]
    grid: Signal<bool>,
    /// The accessible name of the plot, written as `aria-label`. Left empty, the plot has none.
    #[prop(optional, into)]
    label: Signal<String>,
) -> impl IntoView {
    let ctx = use_chart();
    let layout = ChartLayout::default();

    view! {
        <svg
            node_ref=ctx.plot_ref
            data-slot="chart-plot"
            viewBox=format!("0 0 {VIEW_WIDTH} {VIEW_HEIGHT}")
            role="img"
            aria-label=move || Some(label.get()).filter(|label| !label.is_empty())
            on:pointermove=move |e: ev::PointerEvent| {
                ctx.hovered.set(ctx.index_at(e.client_x() as f64));
            }
            on:pointerleave=move |_| ctx.hovered.set(None)
            class=class
        >
            {move || {
                grid.get()
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
                                                    x1=layout.pad_left
                                                    x2=layout.width - layout.pad_right
                                                    y1=y
                                                    y2=y
                                                    class="stroke-border"
                                                    stroke-width="1"
                                                />
                                                <text
                                                    x=layout.pad_left - 8.0
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
                    })
            }}

            <g data-slot="chart-series">
                {move || {
                    ctx.series
                        .get()
                        .into_iter()
                        .enumerate()
                        .map(|(index, series)| {
                            let color = series.resolved_color(index);
                            match ctx.kind.get() {
                                ChartKind::Bar => {
                                    series
                                        .values
                                        .iter()
                                        .enumerate()
                                        .map(|(category, value)| {
                                            let (x, y, width, height) = ctx
                                                .bar(category, index, *value);
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
                                        (0, _) if ctx.kind.get() != ChartKind::Bar => "start",
                                        (
                                            index,
                                            count,
                                        ) if index + 1 == count
                                            && ctx.kind.get() != ChartKind::Bar => "end",
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
                            y1=layout.pad_top
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
        <Show when=move || {
            ctx.hovered.get().is_some()
        }>
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
