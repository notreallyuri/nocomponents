use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::{
    components::{
        date_picker::DatePicker,
        field::{Field, FieldDescription, FieldLabel},
    },
    primitives::calendar::CalendarMode,
    utils::date::Date,
};

const DEFAULT: &str = r#"{
    let selected = RwSignal::new(Vec::<Date>::new());

    view! {
        <DatePicker selected=selected placeholder="Pick a date" />
    }
}"#;

const RANGE: &str = r#"// The layer closes when the selection is finished — for a range, on its second end.
<DatePicker
    mode=CalendarMode::Range
    selected=selected
    placeholder="Pick a range"
    class="w-72"
/>"#;

const FIELD: &str = r#"<Field>
    <FieldLabel>"Delivery date"</FieldLabel>
    <DatePicker selected=selected min=Date::today() />
    <FieldDescription>"Weekends are not available."</FieldDescription>
</Field>"#;

const API: &[ApiEntry] = &[ApiEntry {
    name: "DatePicker",
    description: "A button that opens a calendar and comes back with a date. Composition rather than machinery: a popover holding a `Calendar`, plus the one rule that makes it a picker — the layer closes once the selection is finished.",
    props: &[
        Prop {
            name: "mode",
            ty: "Signal<CalendarMode>",
            default: "Single",
            description: "As on `Calendar`. `Single` closes on the first click, `Range` on the second, `Multiple` stays open.",
        },
        Prop {
            name: "selected",
            ty: "Option<RwSignal<Vec<Date>>>",
            default: "None",
            description: "The selection, held by the caller, in the shape the mode implies. Omit it and the picker keeps its own.",
        },
        Prop {
            name: "default_selected",
            ty: "Vec<Date>",
            default: "[]",
            description: "The dates an uncontrolled picker starts with. Ignored when `selected` is given.",
        },
        Prop {
            name: "on_selected_change",
            ty: "Option<Callback<Vec<Date>>>",
            default: "None",
            description: "Called with the new selection whenever it changes, including when the signal passed as `selected` is written from outside.",
        },
        Prop {
            name: "placeholder",
            ty: "Signal<String>",
            default: "\"Pick a date\"",
            description: "What the button reads while nothing is selected.",
        },
        Prop {
            name: "month",
            ty: "Option<RwSignal<Date>>",
            default: "None",
            description: "The month on show, if you want to drive the paging.",
        },
        Prop {
            name: "default_month",
            ty: "Option<Date>",
            default: "None",
            description: "The month an uncontrolled calendar opens on. Ignored when `month` is given.",
        },
        Prop {
            name: "on_month_change",
            ty: "Option<Callback<Date>>",
            default: "None",
            description: "Called with the new month whenever the one on show changes.",
        },
        Prop {
            name: "open",
            ty: "Option<RwSignal<bool>>",
            default: "None",
            description: "Whether the calendar is open, held by the caller. Omit it and the picker keeps its own.",
        },
        Prop {
            name: "default_open",
            ty: "bool",
            default: "false",
            description: "Whether the calendar starts open. Ignored when `open` is given.",
        },
        Prop {
            name: "on_open_change",
            ty: "Option<Callback<bool>>",
            default: "None",
            description: "Called with the new value whenever the calendar opens or closes, however that came about.",
        },
        Prop {
            name: "week_starts_on",
            ty: "Signal<u32>",
            default: "0",
            description: "0 is Sunday, 1 Monday.",
        },
        Prop {
            name: "min",
            ty: "MaybeProp<Date>",
            default: "None",
            description: "The earliest date that can be picked.",
        },
        Prop {
            name: "max",
            ty: "MaybeProp<Date>",
            default: "None",
            description: "The latest date that can be picked.",
        },
        Prop {
            name: "disabled",
            ty: "Option<Callback<Date, bool>>",
            default: "None",
            description: "Anything else that cannot be picked.",
        },
        Prop {
            name: "side",
            ty: "Signal<Side>",
            default: "Side::Bottom",
            description: "Which side of the button the calendar opens on.",
        },
        Prop {
            name: "align",
            ty: "Signal<Align>",
            default: "Start",
            description: "How it lines up with the button.",
        },
        Prop {
            name: "side_offset",
            ty: "Signal<SideOffset>",
            default: "SideOffset(4.0)",
            description: "Distance from the button, in pixels.",
        },
        Prop {
            name: "locale",
            ty: "Signal<String>",
            default: "\"\"",
            description: "A BCP 47 tag, passed to the calendar inside and used for the button's own label. Empty is English.",
        },
        Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Merged over the button's classes; the calendar itself is styled by `Calendar`.",
        },
        Prop {
            name: "icon",
            ty: "Option<ViewFn>",
            default: "Calendar",
            description: "The glyph, in place of the calendar glyph on the trigger. Any view — an `Icon`, an SVG of your own; see Icons.",
        },
        Prop {
            name: "prev_icon",
            ty: "Option<ViewFn>",
            default: "ChevronLeft",
            description: "The glyph, in place of the previous-month arrow on the calendar inside. Any view — an `Icon`, an SVG of your own; see Icons.",
        },
        Prop {
            name: "next_icon",
            ty: "Option<ViewFn>",
            default: "ChevronRight",
            description: "The glyph, in place of the next-month arrow on the calendar inside. Any view — an `Icon`, an SVG of your own; see Icons.",
        },
    ],
}];

#[component]
pub fn Page() -> impl IntoView {
    let single = RwSignal::new(Vec::<Date>::new());
    let range = RwSignal::new(Vec::<Date>::new());
    let delivery = RwSignal::new(Vec::<Date>::new());

    view! {
        <DocLayout
            title="Date Picker"
            description="A button that opens a calendar and comes back with a date."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="A popover with a calendar in it. Picking a date finishes the job, so the layer closes itself; the button then reads the date, and focus goes back to it."
                    code=DEFAULT
                >
                    <div class="flex flex-wrap items-center gap-3">
                        <DatePicker selected=single />
                        <span class="font-mono text-sm text-muted-foreground">
                            {move || match single.get().first() {
                                Some(date) => date.to_iso(),
                                None => "—".to_string(),
                            }}
                        </span>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Range"
                    description="A range takes two clicks, so the layer waits for the second one before closing."
                    code=RANGE
                >
                    <div class="flex flex-wrap items-center gap-3">
                        <DatePicker
                            mode=CalendarMode::Range
                            selected=range
                            placeholder="Pick a range"
                            class="w-72"
                        />
                        <span class="font-mono text-sm text-muted-foreground">
                            {move || {
                                let range = range.get();
                                if range.is_empty() {
                                    "—".to_string()
                                } else {
                                    range
                                        .iter()
                                        .map(|date| date.to_iso())
                                        .collect::<Vec<_>>()
                                        .join(" → ")
                                }
                            }}
                        </span>
                    </div>
                </DemoSection>

                <DemoSection
                    title="In a field"
                    description="Nothing special: it is a button, so it sits in a `Field` like any other control, and the calendar's own bounds do the validating."
                    code=FIELD
                >
                    <Field class="max-w-sm">
                        <FieldLabel>"Delivery date"</FieldLabel>
                        <DatePicker
                            selected=delivery
                            min=Date::today()
                            disabled=|date: Date| matches!(date.weekday(), 0 | 6)
                        />
                        <FieldDescription>"Weekdays only, from today onwards."</FieldDescription>
                    </Field>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
