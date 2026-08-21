use crate::{
    cn,
    components::{
        button::{Button, ButtonVariant},
        calendar::Calendar,
    },
    icons::calendar::Calendar as CalendarIcon,
    primitives::{
        calendar::CalendarMode,
        floating::{FloatingContext, FloatingRoot, TriggerAria},
        popover::{PopoverContentRoot, PopoverPortalRoot, use_popover},
    },
    utils::{
        date::Date,
        types::{Align, Side, SideOffset},
    },
};
use leptos::{ev, prelude::*};

/// A button that opens a calendar and comes back with a date.
///
/// Composition rather than machinery, the way `AlertDialog` is: a popover holding a `Calendar`,
/// plus the one rule that makes it a picker — the layer closes once the selection is finished,
/// which for a range means when its second end lands.
#[component]
pub fn DatePicker(
    #[prop(optional)] mode: CalendarMode,
    /// The selected dates, in the shape the mode implies.
    #[prop(optional, into)]
    selected: RwSignal<Vec<Date>>,
    #[prop(optional, into)] placeholder: Signal<String>,
    #[prop(default = None, into)] month: Option<RwSignal<Date>>,
    #[prop(default = 0)] week_starts_on: u32,
    #[prop(default = None, into)] min: Option<Date>,
    #[prop(default = None, into)] max: Option<Date>,
    #[prop(default = None, into)] disabled: Option<Callback<Date, bool>>,
    #[prop(optional)] side: Side,
    #[prop(optional)] align: Align,
    #[prop(optional)] side_offset: SideOffset,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    let ctx = FloatingContext::default();

    view! {
        <FloatingRoot
            class="relative inline-block text-left"
            context=ctx
            trigger_aria=TriggerAria::Popup("dialog")
        >
            <DatePickerTrigger
                mode=mode
                selected=selected
                placeholder=placeholder
                class=class
            />
            <DatePickerContent
                mode=mode
                selected=selected
                month=month
                week_starts_on=week_starts_on
                min=min
                max=max
                disabled=disabled
                side=side
                align=align
                side_offset=side_offset
            />
        </FloatingRoot>
    }
}

/// What the button reads: the date, the range, or the placeholder.
fn label(mode: CalendarMode, selected: &[Date], placeholder: &str) -> String {
    let placeholder = if placeholder.is_empty() {
        "Pick a date"
    } else {
        placeholder
    };

    match (mode, selected) {
        (_, []) => placeholder.to_string(),
        (CalendarMode::Range, [start, end]) => {
            format!("{} – {}", start.day_month_year(), end.day_month_year())
        }
        (CalendarMode::Range, [start]) => format!("{} – …", start.day_month_year()),
        (CalendarMode::Multiple, dates) if dates.len() > 1 => {
            format!("{} dates", dates.len())
        }
        (_, [date, ..]) => date.day_month_year(),
    }
}

#[component]
fn DatePickerTrigger(
    mode: CalendarMode,
    selected: RwSignal<Vec<Date>>,
    placeholder: Signal<String>,
    class: Signal<String>,
) -> impl IntoView {
    let ctx = use_popover();

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
            <CalendarIcon class="text-muted-foreground" />
            <span class="truncate">
                {move || {
                    selected.with(|selected| label(mode, selected, &placeholder.get()))
                }}
            </span>
        </Button>
    }
}

#[component]
fn DatePickerContent(
    mode: CalendarMode,
    selected: RwSignal<Vec<Date>>,
    month: Option<RwSignal<Date>>,
    week_starts_on: u32,
    min: Option<Date>,
    max: Option<Date>,
    disabled: Option<Callback<Date, bool>>,
    side: Side,
    align: Align,
    side_offset: SideOffset,
) -> impl IntoView {
    let ctx = use_popover();

    // Closing is the picker's whole contribution: a finished selection means the layer has done
    // its job. `Multiple` never finishes, so it stays open until dismissed.
    Effect::new(move |previous: Option<usize>| {
        let count = selected.with(|selected| selected.len());
        let finished = match mode {
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
                    week_starts_on=week_starts_on
                    min=min
                    max=max
                    disabled=disabled
                    class="border-0 bg-transparent"
                />
            </PopoverContentRoot>
        </PopoverPortalRoot>
    }
}
