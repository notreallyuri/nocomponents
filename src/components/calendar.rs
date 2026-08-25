use crate::{
    cn,
    icons::chevron::{ChevronLeft, ChevronRight},
    primitives::calendar::{
        CalendarCaptionRoot, CalendarDayRoot, CalendarGridRoot, CalendarMode, CalendarNavRoot,
        CalendarRoot,
    },
    utils::date::Date,
};
use leptos::prelude::*;

#[component]
pub fn Calendar(
    #[prop(optional)] mode: CalendarMode,
    #[prop(optional, into)] selected: RwSignal<Vec<Date>>,
    #[prop(default = None, into)] month: Option<RwSignal<Date>>,
    #[prop(default = 0)] week_starts_on: u32,
    #[prop(default = None, into)] min: Option<Date>,
    #[prop(default = None, into)] max: Option<Date>,
    #[prop(default = None, into)] disabled: Option<Callback<Date, bool>>,
    #[prop(optional, into)] locale: Signal<String>,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    view! {
        <CalendarRoot
            mode=mode
            selected=selected
            month=month
            week_starts_on=week_starts_on
            min=min
            max=max
            disabled=disabled
            locale=locale
            class=move || {
                cn!("w-fit rounded-lg border bg-background p-3 text-foreground", class.get())
            }
        >
            <div class="mb-2 flex items-center justify-between gap-2">
                <CalendarNavRoot months=-1 class=NAV_CLASS>
                    <ChevronLeft />
                    <span class="sr-only">"Previous month"</span>
                </CalendarNavRoot>

                <CalendarCaptionRoot class="text-sm font-medium" />

                <CalendarNavRoot months=1 class=NAV_CLASS>
                    <ChevronRight />
                    <span class="sr-only">"Next month"</span>
                </CalendarNavRoot>
            </div>

            <CalendarGridRoot
                class="w-full border-collapse"
                head_cell_class="w-9 pb-1 text-[0.75rem] font-normal text-muted-foreground"
                cell_class="p-0"
                day=Callback::new(|date| view! { <CalendarDay date=date /> }.into_any())
            />
        </CalendarRoot>
    }
}

const NAV_CLASS: &str = "inline-flex size-7 items-center justify-center rounded-md border bg-background text-foreground transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:ring-3 focus-visible:ring-ring/50 focus-visible:outline-none disabled:pointer-events-none disabled:opacity-50";

#[component]
pub fn CalendarDay(date: Date, #[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <CalendarDayRoot
            date=date
            class=move || {
                cn!(
                    "relative flex size-9 items-center justify-center rounded-md text-sm font-normal transition-colors",
                    "hover:bg-accent hover:text-accent-foreground focus-visible:ring-3 focus-visible:ring-ring/50 focus-visible:outline-none",
                    "data-outside:text-muted-foreground/50 data-disabled:pointer-events-none data-disabled:opacity-40",
                    "data-today:font-semibold data-today:after:absolute data-today:after:bottom-1 data-today:after:size-1 data-today:after:rounded-full data-today:after:bg-current",
                    "data-selected:bg-primary data-selected:text-primary-foreground data-selected:hover:bg-primary/90",
                    "data-[range=middle]:rounded-none data-[range=middle]:bg-accent data-[range=middle]:text-accent-foreground data-[range=middle]:hover:bg-accent",
                    "data-[range=start]:rounded-r-none data-[range=end]:rounded-l-none",
                    class.get()
                )
            }
        />
    }
}
