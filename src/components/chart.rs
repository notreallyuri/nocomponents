use crate::{
    cn,
    primitives::chart::{
        ChartKind, ChartLegendRoot, ChartPlotRoot, ChartRoot, ChartSeries, ChartTooltipRoot,
    },
};
use leptos::prelude::*;

/// A chart. One component rather than a set of parts: the plot, the key and the tooltip are the
/// same three pieces every time, and `primitives::chart` has them separately for anyone laying
/// out their own.
#[component]
pub fn Chart(
    /// The categories along the bottom.
    #[prop(into)]
    labels: Signal<Vec<String>>,
    #[prop(into)] series: Signal<Vec<ChartSeries>>,
    #[prop(optional)] kind: ChartKind,
    /// The card that follows the pointer.
    #[prop(default = true)]
    tooltip: bool,
    /// The key beneath the plot. Off for a single series, which needs no key.
    #[prop(default = true)]
    legend: bool,
    #[prop(default = true)] grid: bool,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    view! {
        <ChartRoot
            labels=labels
            series=series
            kind=kind
            class=move || cn!("flex w-full flex-col gap-2", class.get())
        >
            // `relative`, because the tooltip is an HTML card positioned across the plot rather
            // than anything inside the SVG.
            <div class="relative w-full">
                <ChartPlotRoot
                    grid=grid
                    class="aspect-[2/1] w-full overflow-visible [&_text]:select-none"
                />

                {tooltip
                    .then(|| {
                        view! {
                            <ChartTooltipRoot
                                class="pointer-events-none absolute top-2 z-10 min-w-32 rounded-lg border bg-popover px-2.5 py-1.5 text-xs shadow-md [&_[data-slot=chart-tooltip-label]]:mb-1 [&_[data-slot=chart-tooltip-label]]:font-medium"
                                row=Callback::new(|(series, value): (ChartSeries, f64)| {
                                    view! {
                                        <div class="flex items-center gap-1.5 leading-relaxed">
                                            <span
                                                class="size-2 shrink-0 rounded-[2px]"
                                                style=format!("background: {}", series.color)
                                            />
                                            <span class="text-muted-foreground">{series.label}</span>
                                            <span class="ml-auto font-mono font-medium tabular-nums">
                                                {format_value(value)}
                                            </span>
                                        </div>
                                    }
                                        .into_any()
                                })
                            />
                        }
                    })}
            </div>

            {legend
                .then(|| {
                    view! {
                        <ChartLegendRoot
                            class="flex flex-wrap items-center justify-center gap-4"
                            item=Callback::new(|series: ChartSeries| {
                                view! {
                                    <div class="flex items-center gap-1.5 text-xs text-muted-foreground">
                                        <span
                                            class="size-2 shrink-0 rounded-[2px]"
                                            style=format!("background: {}", series.color)
                                        />
                                        {series.label}
                                    </div>
                                }
                                    .into_any()
                            })
                        />
                    }
                })}
        </ChartRoot>
    }
}

/// Whole numbers without a decimal point, everything else to one place — a tooltip that says
/// "12.0" for a count reads as a measurement.
fn format_value(value: f64) -> String {
    if value.fract().abs() < f64::EPSILON {
        format!("{value:.0}")
    } else {
        format!("{value:.1}")
    }
}
