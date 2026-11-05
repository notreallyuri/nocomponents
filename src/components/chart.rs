use crate::{
    cn,
    primitives::chart::{
        ChartKind, ChartLegendRoot, ChartPlotRoot, ChartRoot, ChartSeries, ChartTooltipRoot,
    },
};
use leptos::prelude::*;

#[component]
pub fn Chart(
    #[prop(into)] labels: Signal<Vec<String>>,
    #[prop(into)] series: Signal<Vec<ChartSeries>>,
    #[prop(optional, into)] kind: Signal<ChartKind>,
    #[prop(into, default = true.into())] tooltip: Signal<bool>,
    #[prop(into, default = true.into())] legend: Signal<bool>,
    #[prop(into, default = true.into())] grid: Signal<bool>,
    #[prop(optional, into)] label: Signal<String>,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    view! {
        <ChartRoot
            labels=labels
            series=series
            kind=kind
            class=move || cn!("flex w-full flex-col gap-2", class.get())
        >
            <div class="relative w-full">
                <ChartPlotRoot
                    grid=grid
                    label=label
                    class="aspect-[2/1] w-full overflow-visible [&_text]:select-none"
                />

                {move || {
                    tooltip
                        .get()
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
                        })
                }}
            </div>

            {move || {
                legend
                    .get()
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
                    })
            }}
        </ChartRoot>
    }
}

fn format_value(value: f64) -> String {
    if value.fract().abs() < f64::EPSILON {
        format!("{value:.0}")
    } else {
        format!("{value:.1}")
    }
}
