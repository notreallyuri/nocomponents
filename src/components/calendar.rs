use crate::{
    cn,
    icons::chevron::{ChevronLeft, ChevronRight},
    primitives::calendar::{
        CalendarCaptionRoot, CalendarDayRoot, CalendarGridRoot, CalendarNavRoot, CalendarRoot,
    },
    utils::{controllable::forward, icon::IconSlot},
};
use leptos::prelude::*;

#[allow(unused_imports)]
pub use crate::primitives::calendar::CalendarMode;
#[allow(unused_imports)]
pub use crate::utils::date::Date;

#[component]
pub fn Calendar(
    #[prop(optional, into)] mode: Signal<CalendarMode>,
    #[prop(default = None, into)] selected: Option<RwSignal<Vec<Date>>>,
    #[prop(optional, into)] default_selected: Vec<Date>,
    #[prop(optional, into)] on_selected_change: Option<Callback<Vec<Date>>>,
    #[prop(default = None, into)] month: Option<RwSignal<Date>>,
    #[prop(default = None, into)] default_month: Option<Date>,
    #[prop(optional, into)] on_month_change: Option<Callback<Date>>,
    #[prop(into, default = 0.into())] week_starts_on: Signal<u32>,
    #[prop(optional, into)] min: MaybeProp<Date>,
    #[prop(optional, into)] max: MaybeProp<Date>,
    #[prop(optional, into)] disabled: Option<Callback<Date, bool>>,
    #[prop(optional, into)] locale: Signal<String>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] prev_icon: Option<ViewFn>,
    #[prop(optional, into)] next_icon: Option<ViewFn>,
) -> impl IntoView {
    view! {
        <CalendarRoot
            mode=mode
            selected=selected
            default_selected=default_selected
            on_selected_change=forward(on_selected_change)
            month=month
            default_month=default_month
            on_month_change=forward(on_month_change)
            week_starts_on=week_starts_on
            min=min
            max=max
            disabled=forward(disabled)
            locale=locale
            class=move || {
                cn!("w-fit rounded-lg border bg-background p-3 text-foreground", class.get())
            }
        >
            <div class="mb-2 flex items-center justify-between gap-2">
                <CalendarNavRoot months=-1 class=NAV_CLASS>
                    <IconSlot
                        icon=prev_icon
                        fallback=|| view! { <ChevronLeft /> }
                        data_slot="calendar-icon"
                        class=ICON_CLASS
                    />
                    <span class="sr-only">"Previous month"</span>
                </CalendarNavRoot>

                <CalendarCaptionRoot class="text-sm font-medium" />

                <CalendarNavRoot months=1 class=NAV_CLASS>
                    <IconSlot
                        icon=next_icon
                        fallback=|| view! { <ChevronRight /> }
                        data_slot="calendar-icon"
                        class=ICON_CLASS
                    />
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

const ICON_CLASS: &str = "flex size-4 shrink-0 items-center justify-center [&>svg]:size-full";

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
