use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::{
    components::{calendar::Calendar, native_select::NativeSelect},
    primitives::calendar::CalendarMode,
    utils::date::Date,
};

/// A few tags with visibly different answers: a Latin script that renames the months, one that
/// writes the year first, and one that does not separate its weekday names into letters at all.
const LOCALES: [(&str, &str); 5] = [
    ("pt-BR", "Português (Brasil)"),
    ("fr-FR", "Français"),
    ("de-DE", "Deutsch"),
    ("ja-JP", "日本語"),
    ("", "English (default)"),
];

const DEFAULT: &str = r#"{
    let selected = RwSignal::new(vec![Date::today()]);

    view! {
        <Calendar selected=selected />
        <p>{move || selected.get().first().map(|d| d.day_month_year()).unwrap_or_default()}</p>
    }
}"#;

const RANGE: &str = r#"// Two clicks make a range; the days between them fill in as the pointer moves.
<Calendar mode=CalendarMode::Range selected=selected week_starts_on=1 />"#;

const LOCALE: &str = r#"let locale = RwSignal::new("pt-BR".to_string());

view! {
    // Month names, weekday headers and each day's aria-label follow the tag.
    <Calendar locale=locale week_starts_on=1 />
}"#;

const BOUNDS: &str = r#"// `min` and `max` fence the grid — and disable the paging buttons at the ends.
// `disabled` rules out anything else: here, weekends.
<Calendar
    min=Date::today()
    max=Date::today().add_months(2)
    disabled=Callback::new(|date: Date| matches!(date.weekday(), 0 | 6))
/>"#;

const MULTIPLE: &str = r#"<Calendar mode=CalendarMode::Multiple selected=selected />"#;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "Calendar",
        description: "A month grid. One component rather than a set of parts, since the caption, the paging and the grid only ever go together; `primitives::calendar` has the parts for anyone who disagrees.",
        props: &[
            Prop {
                name: "mode",
                ty: "CalendarMode",
                default: "Single",
                description: "`Single` picks one date and clicking it again clears it; `Multiple` toggles each; `Range` takes two clicks and fills in between.",
            },
            Prop {
                name: "selected",
                ty: "RwSignal<Vec<Date>>",
                default: "[]",
                description: "The selection, in every mode: one date, many, or a start and an end in order.",
            },
            Prop {
                name: "month",
                ty: "Option<RwSignal<Date>>",
                default: "None",
                description: "The month on show. Pass one to drive the paging from outside; omitted, it starts on the first selected date's month, or today's.",
            },
            Prop {
                name: "week_starts_on",
                ty: "u32",
                default: "0",
                description: "0 is Sunday, 1 Monday. The weekday header follows.",
            },
            Prop {
                name: "min",
                ty: "Option<Date>",
                default: "None",
                description: "The earliest date that can be picked. Also disables the paging button once a month is entirely below it.",
            },
            Prop {
                name: "max",
                ty: "Option<Date>",
                default: "None",
                description: "The latest date that can be picked.",
            },
            Prop {
                name: "disabled",
                ty: "Option<Callback<Date, bool>>",
                default: "None",
                description: "Anything else that cannot be picked — weekends, days already taken. Called for each of the 42 cells on show.",
            },
            Prop {
                name: "locale",
                ty: "Signal<String>",
                default: "\"\"",
                description: "A BCP 47 tag — \"pt-BR\", \"ja\" — for the month caption, the weekday headers and every day's accessible name, resolved once through `Intl.DateTimeFormat`. Empty is English, and so is a tag the browser cannot read.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the calendar's classes.",
            },
        ],
    },
    ApiEntry {
        name: "CalendarDay",
        description: "One day, for a caller rendering their own grid with `CalendarGridRoot`. Every state it can be in — outside the month, today, selected, where it sits in a range — is a data attribute the primitive has already worked out.",
        props: &[
            Prop {
                name: "date",
                ty: "Date",
                default: "",
                description: "Which day this is.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the day's classes.",
            },
        ],
    },
];

#[component]
pub fn Page() -> impl IntoView {
    let single = RwSignal::new(vec![Date::today()]);
    let range = RwSignal::new(Vec::<Date>::new());
    let multiple = RwSignal::new(Vec::<Date>::new());
    let locale = RwSignal::new("pt-BR".to_string());

    view! {
        <DocLayout title="Calendar" description="A month grid you can pick dates in.">
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="The arrows move the keyboard by a day and a week, Home and End go to the ends of the week, and Page Up and Page Down change month — stepping off the edge of a month pages to the next one, because focus follows a date rather than a cell. Today is marked with a dot so it survives being selected."
                    code=DEFAULT
                >
                    <div class="flex flex-col items-center gap-3">
                        <Calendar selected=single />
                        <p class="text-sm text-muted-foreground">
                            {move || match single.get().first() {
                                Some(date) => date.long_form(),
                                None => "Nothing selected.".to_string(),
                            }}
                        </p>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Range"
                    description="The first click starts a range and the second closes it; while it is half-made, moving the pointer shows what would be selected. A third click starts over. This one starts its weeks on Monday."
                    code=RANGE
                >
                    <div class="flex flex-col items-center gap-3">
                        <Calendar mode=CalendarMode::Range selected=range week_starts_on=1 />
                        <p class="text-sm text-muted-foreground">
                            {move || {
                                let range = range.get();
                                match range.as_slice() {
                                    [start, end] => {
                                        format!(
                                            "{} → {} ({} nights)",
                                            start.day_month_year(),
                                            end.day_month_year(),
                                            end.to_days() - start.to_days(),
                                        )
                                    }
                                    [start] => format!("{} → …", start.day_month_year()),
                                    _ => "No range yet.".to_string(),
                                }
                            }}
                        </p>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Multiple"
                    description="Each click toggles one date, and the selection stays sorted."
                    code=MULTIPLE
                >
                    <div class="flex flex-col items-center gap-3">
                        <Calendar mode=CalendarMode::Multiple selected=multiple />
                        <p class="text-sm text-muted-foreground">
                            {move || {
                                let dates = multiple.get();
                                if dates.is_empty() {
                                    "Nothing selected.".to_string()
                                } else {
                                    dates
                                        .iter()
                                        .map(|date| date.to_iso())
                                        .collect::<Vec<_>>()
                                        .join(", ")
                                }
                            }}
                        </p>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Locale"
                    description="One tag decides the month caption, the weekday headers and every day's accessible name — the caption is formatted rather than assembled, so its parts come out in the order the locale writes them. Nothing else in the library asks for Intl, and a tag it cannot read falls back to English rather than failing."
                    code=LOCALE
                >
                    <div class="flex flex-col items-center gap-3">
                        <div class="w-56">
                            <NativeSelect model=locale>
                                {LOCALES
                                    .iter()
                                    .map(|(tag, label)| {
                                        view! { <option value=*tag>{*label}</option> }
                                    })
                                    .collect_view()}
                            </NativeSelect>
                        </div>
                        <Calendar locale=locale week_starts_on=1 />
                    </div>
                </DemoSection>

                <DemoSection
                    title="Bounds"
                    description="`min` and `max` fence the grid and stop the paging buttons at the ends; `disabled` rules out anything else — here, every weekend."
                    code=BOUNDS
                >
                    <div class="flex justify-center">
                        <Calendar
                            min=Date::today()
                            max=Date::today().add_months(2)
                            disabled=Callback::new(|date: Date| matches!(date.weekday(), 0 | 6))
                        />
                    </div>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
