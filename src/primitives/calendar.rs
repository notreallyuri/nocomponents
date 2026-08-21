//! A month grid you can pick dates in.
//!
//! Focus follows a *date*, not a DOM node: the keyboard moves `focused`, the grid re-renders
//! around it, and an effect puts the browser's focus on whichever cell now holds that date. That
//! is what makes stepping off the end of a month page to the next one for free, and it is why
//! this cannot reuse [`crate::primitives::roving_focus`], which walks siblings in the DOM and has
//! no idea that a grid is two-dimensional.
//!
//! The selection is one `Vec<Date>` in every mode, which keeps the arithmetic in one place: a
//! single date is a vector of one, and a range is two — start and end, in order — with the
//! in-between days derived rather than stored.

use crate::utils::{
    date::{Date, month_grid},
    next_id,
};
use leptos::{context::Provider, ev, prelude::*, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;
use web_sys::HtmlElement;

/// How many dates a click may leave selected.
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum CalendarMode {
    /// One date, and clicking it again clears it.
    #[default]
    Single,
    /// As many as are clicked, each click toggling one.
    Multiple,
    /// Two dates and everything between them. The first click starts a range, the second closes
    /// it, and a third starts over.
    Range,
}

/// Where a date sits in a selected range, which is what the styled layer needs to round the ends
/// and fill the middle.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RangePosition {
    Outside,
    Start,
    Middle,
    End,
    /// A range of one day: both ends at once.
    Only,
}

impl RangePosition {
    pub fn as_str(&self) -> &'static str {
        match self {
            RangePosition::Outside => "outside",
            RangePosition::Start => "start",
            RangePosition::Middle => "middle",
            RangePosition::End => "end",
            RangePosition::Only => "only",
        }
    }
}

#[derive(Copy, Clone)]
pub struct CalendarContext {
    /// The month on show. Only the year and month are read; the day is ignored.
    pub month: RwSignal<Date>,
    /// The date the keyboard is on. Moving it is what moves focus.
    pub focused: RwSignal<Date>,
    /// Every selected date, in order. One in `Single`, two in `Range`.
    pub selected: RwSignal<Vec<Date>>,

    pub mode: CalendarMode,
    /// 0 is Sunday.
    pub week_starts_on: u32,
    pub min: Option<Date>,
    pub max: Option<Date>,
    /// Anything else that cannot be picked — weekends, days that are already booked.
    pub disabled: Option<Callback<Date, bool>>,

    /// The day the pointer is over while a range is half-made, so the grid can show what would be
    /// selected before it is.
    pub preview: RwSignal<Option<Date>>,

    pub grid_ref: AnyNodeRef,
    pub label_id: RwSignal<String>,
}

impl CalendarContext {
    pub fn is_disabled(&self, date: Date) -> bool {
        if self.min.is_some_and(|min| date < min) || self.max.is_some_and(|max| date > max) {
            return true;
        }

        self.disabled.is_some_and(|disabled| disabled.run(date))
    }

    pub fn is_selected(&self, date: Date) -> bool {
        match self.mode {
            CalendarMode::Range => self.range_position(date) != RangePosition::Outside,
            _ => self.selected.with(|selected| selected.contains(&date)),
        }
    }

    /// The range as it stands, including the day the pointer is on while it is half-made.
    fn range_bounds(&self) -> Option<(Date, Date)> {
        self.selected.with(|selected| match selected.as_slice() {
            [start, end] => Some((*start.min(end), *start.max(end))),
            [start] => match self.preview.get() {
                Some(preview) => Some((*start.min(&preview), *start.max(&preview))),
                None => Some((*start, *start)),
            },
            _ => None,
        })
    }

    pub fn range_position(&self, date: Date) -> RangePosition {
        let Some((start, end)) = self.range_bounds() else {
            return RangePosition::Outside;
        };

        match (date == start, date == end) {
            (true, true) => RangePosition::Only,
            (true, false) => RangePosition::Start,
            (false, true) => RangePosition::End,
            _ if date > start && date < end => RangePosition::Middle,
            _ => RangePosition::Outside,
        }
    }

    /// Picks a date, in whichever way the mode means.
    pub fn select(&self, date: Date) {
        if self.is_disabled(date) {
            return;
        }

        match self.mode {
            CalendarMode::Single => self.selected.update(|selected| {
                if selected.first() == Some(&date) {
                    selected.clear();
                } else {
                    *selected = vec![date];
                }
            }),
            CalendarMode::Multiple => self.selected.update(|selected| {
                match selected.iter().position(|selected| *selected == date) {
                    Some(index) => {
                        selected.remove(index);
                    }
                    None => {
                        selected.push(date);
                        selected.sort();
                    }
                }
            }),
            CalendarMode::Range => {
                self.preview.set(None);
                self.selected.update(|selected| match selected.len() {
                    // A finished range, or none at all: start a new one.
                    1 => {
                        let start = selected[0];
                        *selected = if date < start {
                            vec![date, start]
                        } else {
                            vec![start, date]
                        };
                    }
                    _ => *selected = vec![date],
                });
            }
        }

        self.focused.set(date);
    }

    /// Moves the keyboard's date, bringing the month with it when it steps off the edge.
    pub fn move_focus(&self, days: i64) {
        let next = self.focused.get_untracked().add_days(days);
        self.focus(next);
    }

    pub fn focus(&self, date: Date) {
        self.focused.set(date);
        if !date.is_same_month(self.month.get_untracked()) {
            self.month.set(date.first_of_month());
        }
    }

    pub fn go_to_month(&self, months: i32) {
        let month = self.month.get_untracked().add_months(months);
        self.month.set(month);
        // The keyboard comes along, landing on the same day of the new month where it exists.
        self.focused
            .set(self.focused.get_untracked().add_months(months));
    }

    /// Whether paging to `months` from here would land outside `min`/`max` entirely.
    pub fn can_go_to_month(&self, months: i32) -> bool {
        let target = self.month.get().add_months(months);
        let last = Date::new(
            target.year,
            target.month,
            Date::days_in_month(target.year, target.month),
        );

        !(self.min.is_some_and(|min| last < min) || self.max.is_some_and(|max| target > max))
    }

    /// The 42 dates on show, always six weeks so the grid never changes height.
    pub fn days(&self) -> Vec<Date> {
        month_grid(self.month.get(), self.week_starts_on)
    }

    /// The weekday indices in column order, so a Monday-first calendar labels its columns right.
    pub fn weekdays(&self) -> Vec<u32> {
        (0..7)
            .map(|column| (self.week_starts_on + column) % 7)
            .collect()
    }
}

pub fn use_calendar() -> CalendarContext {
    expect_context::<CalendarContext>()
}

#[component]
pub fn CalendarRoot(
    #[prop(optional)] mode: CalendarMode,
    /// The selected dates. One for `Single`, two for `Range`, any number for `Multiple`.
    #[prop(optional, into)]
    selected: RwSignal<Vec<Date>>,
    /// The month on show. Defaults to the month of the first selected date, or today's.
    #[prop(default = None, into)]
    month: Option<RwSignal<Date>>,
    /// 0 is Sunday, 1 Monday.
    #[prop(default = 0)]
    week_starts_on: u32,
    #[prop(default = None, into)] min: Option<Date>,
    #[prop(default = None, into)] max: Option<Date>,
    /// Anything else that cannot be picked.
    #[prop(default = None, into)]
    disabled: Option<Callback<Date, bool>>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let today = Date::today();
    let opening = selected
        .with_untracked(|selected| selected.first().copied())
        .unwrap_or(today);

    let ctx = CalendarContext {
        month: month.unwrap_or_else(|| RwSignal::new(opening.first_of_month())),
        focused: RwSignal::new(opening),
        selected,
        mode,
        week_starts_on,
        min,
        max,
        disabled,
        preview: RwSignal::new(None),
        grid_ref: AnyNodeRef::new(),
        label_id: RwSignal::new(next_id("calendar-label")),
    };

    view! {
        <Provider value=ctx>
            <div
                data-slot="calendar"
                data-mode=match mode {
                    CalendarMode::Single => "single",
                    CalendarMode::Multiple => "multiple",
                    CalendarMode::Range => "range",
                }
                class=class
            >
                {children()}
            </div>
        </Provider>
    }
}

/// The month's name, and the id the grid points `aria-labelledby` at.
#[component]
pub fn CalendarCaptionRoot(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    let ctx = use_calendar();

    view! {
        <div
            id=move || ctx.label_id.get()
            data-slot="calendar-caption"
            aria-live="polite"
            class=class
        >
            {move || ctx.month.get().month_caption()}
        </div>
    }
}

/// A step of `months` — `-1` for the button that goes back.
#[component]
pub fn CalendarNavRoot(
    months: i32,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_calendar();

    view! {
        <button
            type="button"
            data-slot=if months < 0 { "calendar-previous" } else { "calendar-next" }
            disabled=move || !ctx.can_go_to_month(months)
            on:click=move |_| ctx.go_to_month(months)
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn CalendarGridRoot(
    #[prop(optional, into)] class: Signal<String>,
    /// The row of weekday abbreviations.
    #[prop(optional, into)]
    head_class: Signal<String>,
    #[prop(optional, into)] head_cell_class: Signal<String>,
    #[prop(optional, into)] row_class: Signal<String>,
    #[prop(optional, into)] cell_class: Signal<String>,
    /// One day, handed the date it is for; the caller renders the button.
    day: Callback<Date, AnyView>,
) -> impl IntoView {
    let ctx = use_calendar();

    // Focus follows the date, once the cell holding it exists.
    Effect::new(move |_| {
        let focused = ctx.focused.get();
        let Some(grid) = ctx.grid_ref.get() else {
            return;
        };
        // Only while the keyboard is already in the grid: a calendar that steals focus on mount
        // would drag the page down to itself.
        let inside = document()
            .active_element()
            .is_some_and(|active| grid.contains(Some(&active)));
        if !inside {
            return;
        }

        let selector = format!("[data-day=\"{}\"]", focused.to_iso());
        if let Ok(Some(cell)) = grid.query_selector(&selector)
            && let Ok(cell) = cell.dyn_into::<HtmlElement>()
        {
            let _ = cell.focus();
        }
    });

    let on_keydown = move |e: ev::KeyboardEvent| {
        let days = match e.key().as_str() {
            "ArrowLeft" => -1,
            "ArrowRight" => 1,
            "ArrowUp" => -7,
            "ArrowDown" => 7,
            "Home" => {
                let focused = ctx.focused.get_untracked();
                let column = (focused.weekday() + 7 - ctx.week_starts_on % 7) % 7;
                -(column as i64)
            }
            "End" => {
                let focused = ctx.focused.get_untracked();
                let column = (focused.weekday() + 7 - ctx.week_starts_on % 7) % 7;
                6 - column as i64
            }
            "PageUp" => {
                e.prevent_default();
                ctx.go_to_month(-1);
                return;
            }
            "PageDown" => {
                e.prevent_default();
                ctx.go_to_month(1);
                return;
            }
            _ => return,
        };

        e.prevent_default();
        ctx.move_focus(days);
    };

    view! {
        <table
            node_ref=ctx.grid_ref
            data-slot="calendar-grid"
            role="grid"
            aria-labelledby=move || ctx.label_id.get()
            on:keydown=on_keydown
            class=class
        >
            <thead class=move || head_class.get()>
                <tr>
                    {move || {
                        ctx.weekdays()
                            .into_iter()
                            .map(|weekday| {
                                view! {
                                    <th
                                        scope="col"
                                        abbr=crate::utils::date::WEEKDAY_NAMES[weekday as usize]
                                        class=move || head_cell_class.get()
                                    >
                                        {crate::utils::date::WEEKDAY_ABBREVIATIONS[weekday as usize]}
                                    </th>
                                }
                            })
                            .collect_view()
                    }}
                </tr>
            </thead>
            <tbody>
                {move || {
                    ctx.days()
                        .chunks(7)
                        .map(|week| {
                            let week = week.to_vec();
                            view! {
                                <tr class=move || row_class.get()>
                                    {week
                                        .iter()
                                        .map(|date| {
                                            let date = *date;
                                            view! {
                                                <td role="gridcell" class=move || cell_class.get()>
                                                    {day.run(date)}
                                                </td>
                                            }
                                        })
                                        .collect_view()}
                                </tr>
                            }
                        })
                        .collect_view()
                }}
            </tbody>
        </table>
    }
}

/// One day. Everything about it is on the element as data attributes, so the styled layer never
/// has to ask a question the primitive has already answered.
#[component]
pub fn CalendarDayRoot(date: Date, #[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    let ctx = use_calendar();
    let today = Date::today();

    let outside = Signal::derive(move || !date.is_same_month(ctx.month.get()));
    let selected = Signal::derive(move || ctx.is_selected(date));
    let disabled = Signal::derive(move || ctx.is_disabled(date));
    let focused = Signal::derive(move || ctx.focused.get() == date);

    view! {
        <button
            type="button"
            data-slot="calendar-day"
            data-day=date.to_iso()
            data-outside=move || outside.get().then_some("")
            data-today=(date == today).then_some("")
            data-selected=move || selected.get().then_some("")
            data-range=move || {
                (ctx.mode == CalendarMode::Range).then(|| ctx.range_position(date).as_str())
            }
            aria-label=date.long_form()
            aria-selected=move || selected.get().to_string()
            aria-disabled=move || disabled.get().then_some("true")
            disabled=move || disabled.get()
            // One tab stop for the whole grid, on whichever date the keyboard is on.
            tabindex=move || if focused.get() { "0" } else { "-1" }
            on:click=move |_| ctx.select(date)
            on:focus=move |_| ctx.focused.set(date)
            on:pointerenter=move |_| {
                if ctx.mode == CalendarMode::Range
                    && ctx.selected.with_untracked(|selected| selected.len() == 1)
                {
                    ctx.preview.set(Some(date));
                }
            }
            class=class
        >
            {date.day}
        </button>
    }
}
