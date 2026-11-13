use crate::{
    cn,
    components::{
        button::{Button, ButtonVariant},
        calendar::Calendar,
    },
    icons::{
        calendar::Calendar as CalendarIcon,
        chevron::{ChevronLeft, ChevronRight},
    },
    primitives::{
        floating::{FloatingContext, FloatingRoot, TriggerAria},
        popover::{PopoverContentRoot, PopoverPortalRoot, use_popover},
    },
    utils::{
        controllable::{controllable, forward},
        date::DateNames,
        icon::{IconSlot, icon_or},
    },
};
use leptos::{ev, prelude::*};

#[allow(unused_imports)]
pub use crate::primitives::calendar::CalendarMode;
#[allow(unused_imports)]
pub use crate::utils::date::Date;
#[allow(unused_imports)]
pub use crate::utils::types::{Align, Side, SideOffset};

#[component]
pub fn DatePicker(
    #[prop(optional, into)] mode: Signal<CalendarMode>,
    #[prop(default = None, into)] selected: Option<RwSignal<Vec<Date>>>,
    #[prop(optional, into)] default_selected: Vec<Date>,
    #[prop(optional, into)] on_selected_change: Option<Callback<Vec<Date>>>,
    #[prop(optional, into)] placeholder: Signal<String>,
    #[prop(default = None, into)] month: Option<RwSignal<Date>>,
    #[prop(default = None, into)] default_month: Option<Date>,
    #[prop(optional, into)] on_month_change: Option<Callback<Date>>,
    #[prop(default = None, into)] open: Option<RwSignal<bool>>,
    #[prop(optional)] default_open: bool,
    #[prop(optional, into)] on_open_change: Option<Callback<bool>>,
    #[prop(into, default = 0.into())] week_starts_on: Signal<u32>,
    #[prop(optional, into)] min: MaybeProp<Date>,
    #[prop(optional, into)] max: MaybeProp<Date>,
    #[prop(optional, into)] disabled: Option<Callback<Date, bool>>,
    #[prop(optional, into)] locale: Signal<String>,
    #[prop(optional, into)] side: Signal<Side>,
    #[prop(optional, into)] align: Signal<Align>,
    #[prop(optional, into)] side_offset: Signal<SideOffset>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] icon: Option<ViewFn>,
    #[prop(optional, into)] prev_icon: Option<ViewFn>,
    #[prop(optional, into)] next_icon: Option<ViewFn>,
) -> impl IntoView {
    let ctx = FloatingContext::default();
    let selected = controllable(selected, default_selected, on_selected_change);
    let nav_icons = StoredValue::new((prev_icon, next_icon));

    view! {
        <FloatingRoot
            attr:data-slot="date-picker"
            class="relative inline-block text-left"
            context=ctx
            open=open
            default_open=default_open
            on_open_change=forward(on_open_change)
            trigger_aria=TriggerAria::Popup("dialog")
        >
            <DatePickerTrigger
                mode=mode
                selected=selected
                placeholder=placeholder
                locale=locale
                class=class
                icon=icon.clone()
            />
            <DatePickerContent
                mode=mode
                selected=selected
                month=month
                default_month=default_month
                on_month_change=on_month_change
                week_starts_on=week_starts_on
                min=min
                max=max
                disabled=disabled
                locale=locale
                side=side
                align=align
                side_offset=side_offset
                nav_icons=nav_icons
            />
        </FloatingRoot>
    }
}

fn label(mode: CalendarMode, selected: &[Date], placeholder: &str, names: &DateNames) -> String {
    let placeholder = if placeholder.is_empty() {
        "Pick a date"
    } else {
        placeholder
    };

    match (mode, selected) {
        (_, []) => placeholder.to_string(),
        (CalendarMode::Range, [start, end]) => {
            format!(
                "{} – {}",
                names.day_month_year(*start),
                names.day_month_year(*end)
            )
        }
        (CalendarMode::Range, [start]) => format!("{} – …", names.day_month_year(*start)),
        (CalendarMode::Multiple, dates) if dates.len() > 1 => {
            format!("{} dates", dates.len())
        }
        (_, [date, ..]) => names.day_month_year(*date),
    }
}

#[component]
fn DatePickerTrigger(
    mode: Signal<CalendarMode>,
    selected: RwSignal<Vec<Date>>,
    placeholder: Signal<String>,
    locale: Signal<String>,
    class: Signal<String>,
    icon: Option<ViewFn>,
) -> impl IntoView {
    let ctx = use_popover();
    let names = Memo::new(move |_| DateNames::new(&locale.get()));

    view! {
        <Button
            variant=ButtonVariant::Outline
            node_ref=ctx.trigger_ref
            on_click=Callback::new(move |_: ev::MouseEvent| ctx.toggle())
            class=move || {
                cn!(
                    "w-56 justify-start gap-2 font-normal data-[empty=true]:text-muted-foreground",
                    class.get()
                )
            }
            attr:data-empty=move || selected.with(|selected| selected.is_empty()).to_string()
        >
            <IconSlot
                icon=icon.clone()
                fallback=|| view! { <CalendarIcon /> }
                data_slot="date-picker-icon"
                class="flex size-4 shrink-0 items-center justify-center text-muted-foreground [&>svg]:size-full"
            />
            <span class="truncate">
                {move || {
                    names
                        .with(|names| {
                            selected
                                .with(|selected| label(
                                    mode.get(),
                                    selected,
                                    &placeholder.get(),
                                    names,
                                ))
                        })
                }}
            </span>
        </Button>
    }
}

#[component]
fn DatePickerContent(
    mode: Signal<CalendarMode>,
    selected: RwSignal<Vec<Date>>,
    month: Option<RwSignal<Date>>,
    default_month: Option<Date>,
    on_month_change: Option<Callback<Date>>,
    week_starts_on: Signal<u32>,
    min: MaybeProp<Date>,
    max: MaybeProp<Date>,
    disabled: Option<Callback<Date, bool>>,
    locale: Signal<String>,
    side: Signal<Side>,
    align: Signal<Align>,
    side_offset: Signal<SideOffset>,
    nav_icons: StoredValue<(Option<ViewFn>, Option<ViewFn>)>,
) -> impl IntoView {
    let ctx = use_popover();

    Effect::new(move |previous: Option<usize>| {
        let count = selected.with(|selected| selected.len());
        let finished = match mode.get() {
            CalendarMode::Single => count == 1,
            CalendarMode::Range => count == 2,
            CalendarMode::Multiple => false,
        };

        if previous.is_some_and(|previous| previous != count) && finished {
            ctx.close();
        }

        count
    });

    view! {
        <PopoverPortalRoot>
            <PopoverContentRoot
                side=side
                align=align
                side_offset=side_offset
                class="rounded-lg bg-popover shadow-md ring-1 ring-foreground/10 outline-none animation-duration-100 pointer-events-auto data-[state=closed]:pointer-events-none data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95"
            >
                <Calendar
                    mode=mode
                    selected=selected
                    month=month
                    default_month=default_month
                    on_month_change=forward(on_month_change)
                    week_starts_on=week_starts_on
                    min=min
                    max=max
                    disabled=forward(disabled)
                    locale=locale
                    prev_icon=icon_or(nav_icons.get_value().0, || view! { <ChevronLeft /> })
                    next_icon=icon_or(nav_icons.get_value().1, || view! { <ChevronRight /> })
                    class="border-0 bg-transparent"
                />
            </PopoverContentRoot>
        </PopoverPortalRoot>
    }
}
