//! Chart blocks.

use super::Block;
use leptos::prelude::*;
use nocomponents::{
    components::{
        card::{Card, CardAction, CardContent, CardDescription, CardHeader, CardTitle},
        chart::Chart,
        toggle::{ToggleSize, ToggleVariant},
        toggle_group::{ToggleGroup, ToggleGroupItem},
    },
    primitives::{
        chart::{ChartKind, ChartSeries},
        toggle_group::ToggleGroupType,
    },
    utils::date::{Date, MONTH_NAMES, WEEKDAY_ABBREVIATIONS},
};

/// Twelve weeks of daily visits, one week per line. Weekdays are busy, weekends are not, and the
/// whole thing drifts upwards — which is the point of the block: the rhythm is only visible a day
/// at a time, the drift only a week at a time, and it is one list of numbers either way.
#[rustfmt::skip]
const VISITS: &[f64] = &[
    210.0, 224.0, 232.0, 219.0, 240.0, 138.0, 121.0,
    226.0, 241.0, 238.0, 250.0, 244.0, 142.0, 130.0,
    233.0, 248.0, 262.0, 255.0, 268.0, 151.0, 136.0,
    247.0, 259.0, 271.0, 266.0, 275.0, 158.0, 143.0,
    252.0, 274.0, 283.0, 279.0, 290.0, 166.0, 149.0,
    268.0, 286.0, 297.0, 288.0, 301.0, 172.0, 155.0,
    279.0, 298.0, 309.0, 305.0, 316.0, 181.0, 162.0,
    291.0, 312.0, 324.0, 318.0, 330.0, 188.0, 168.0,
    304.0, 327.0, 338.0, 331.0, 345.0, 196.0, 176.0,
    318.0, 341.0, 355.0, 348.0, 359.0, 204.0, 183.0,
    332.0, 356.0, 370.0, 363.0, 377.0, 213.0, 190.0,
    347.0, 372.0, 386.0, 380.0, 394.0, 221.0, 198.0,
];

/// How much of [`VISITS`] is shown, and how many days go into one column.
#[derive(Clone, Copy, PartialEq)]
struct Range {
    slug: &'static str,
    label: &'static str,
    days: usize,
    bucket: usize,
    caption: &'static str,
}

#[rustfmt::skip]
const RANGES: &[Range] = &[
    Range { slug: "7d",  label: "7 days",   days: 7,  bucket: 1, caption: "One column a day" },
    Range { slug: "6w",  label: "6 weeks",  days: 42, bucket: 7, caption: "One column a week" },
    Range { slug: "12w", label: "12 weeks", days: 84, bucket: 7, caption: "One column a week" },
];

const RANGED: &str = r##"// Twelve weeks of daily numbers, and three ways of looking at them. The
// range does not fetch anything and does not rebuild the chart: it decides
// how much of one list is read and how many days go into a column.
const VISITS: &[f64] = &[…];

// Every label is drawn — the chart has no thinning of its own, and the
// tooltip reads the same list the axis does. So eighty-four daily labels
// are not made readable by blanking every seventh one, which would leave
// the tooltip with nothing to say. Fewer, wider columns is the answer, and
// summing into them is the caller's job, not the chart's.
fn bucketed(range: Range) -> (Vec<String>, Vec<f64>) {
    // Weeks end on the most recent Sunday, not on today: run a weekly
    // column to today and the current week is a short bar beside eleven
    // whole ones, which reads as a collapse rather than as a Tuesday.
    let today = Date::today();
    let first = today.add_days(-(today.weekday() as i64) + 1 - range.days as i64);

    VISITS[VISITS.len() - range.days..]
        .chunks(range.bucket)
        .enumerate()
        .map(|(column, days)| {
            let date = first.add_days((column * range.bucket) as i64);
            (label(date, range.bucket), days.iter().sum())
        })
        .unzip()
}

// One memo, read three times: the axis, the bars, and the total over the
// card. A second pass for the total would be a second chance to disagree.
let shown = Memo::new(move |_| bucketed(range.get()));
let labels = Signal::derive(move || shown.get().0);
let series = Signal::derive(move || vec![ChartSeries::new("visits", "Visits", shown.get().1)]);
let total = Signal::derive(move || shown.get().1.iter().sum::<f64>());

view! {
    <Card class="w-full max-w-2xl">
        <CardHeader>
            <CardTitle>"Visitors"</CardTitle>
            <CardDescription>{move || range.get().caption}</CardDescription>
            <CardAction>
                // A toggle group is a toggle group: pressing the pressed item empties it.
                // A range always has one, so `pressed` is put back the way it was and the
                // control cannot land on nothing. `RadioGroup` is the one that means
                // "exactly one" — it just does not look like a segmented control.
                <ToggleGroup
                    kind=ToggleGroupType::Single
                    variant=ToggleVariant::Outline
                    size=ToggleSize::Sm
                    value=pressed
                >
                    <ToggleGroupItem value="7d">"7 days"</ToggleGroupItem>
                    <ToggleGroupItem value="6w">"6 weeks"</ToggleGroupItem>
                    <ToggleGroupItem value="12w">"12 weeks"</ToggleGroupItem>
                </ToggleGroup>
            </CardAction>
        </CardHeader>

        <CardContent class="flex flex-col gap-4">
            <div>
                <div class="text-2xl font-semibold tabular-nums">
                    {move || thousands(total.get())}
                </div>
                <p class="text-xs text-muted-foreground">
                    {move || format!("visits in the last {}", range.get().label)}
                </p>
            </div>

            // Bars rather than a line: a column is a *total* over its days, and a line
            // between two totals draws values that were never counted.
            <Chart labels=labels series=series kind=ChartKind::Bar legend=false />
        </CardContent>
    </Card>
}"##;

pub const BLOCKS: &[Block] = &[Block {
    slug: "a-range-over-one-set-of-numbers",
    title: "A range over one set of numbers",
    uses: &["card", "chart", "toggle", "toggle-group"],
    description: "A dashboard card, and the seam is what the range control does *not* do: it \
                  fetches nothing and rebuilds nothing. Twelve weeks of daily numbers sit in one \
                  const, and the range decides how much of it is read and how many days go into a \
                  column — so the weekday rhythm is what you see a day at a time, the climb is \
                  what you see a week at a time, and both are the same list. The bucketing is the \
                  caller's because it has to be: the chart draws every label it is handed, and \
                  the tooltip reads that same list, so eighty-four daily labels cannot be thinned \
                  by blanking every seventh one — that would only leave the tooltip with nothing \
                  to say. Fewer, wider columns is the honest fix, and a column that is a total \
                  over its days is a bar, not a point on a line, which would draw values nobody \
                  counted in between. The axis rescales itself as the columns grow, since the \
                  ceiling is read off the data rather than fixed, and the window ends on the \
                  most recent Sunday rather than on today, so a week's column is always a whole \
                  week — run it to today and the current week is a short bar beside eleven full \
                  ones, which reads as a collapse in traffic rather than as a Tuesday. One memo feeds the axis, the \
                  bars and the figure printed above them, so the headline and the plot cannot \
                  drift. The one wrinkle is the control: a toggle group can be *emptied* by \
                  pressing what is already pressed, and a range always has a value, so the choice \
                  is written back rather than left on nothing.",
    code: RANGED,
    view: || view! { <VisitorsCard /> }.into_any(),
}];

fn range_for(slug: &str) -> Range {
    RANGES
        .iter()
        .copied()
        .find(|range| range.slug == slug)
        .unwrap_or(RANGES[2])
}

/// A day at a time the weekday is what tells one column from the next; a week at a time it is the
/// date the week starts on.
fn label(date: Date, bucket: usize) -> String {
    if bucket == 1 {
        format!(
            "{} {}",
            WEEKDAY_ABBREVIATIONS[date.weekday() as usize],
            date.day
        )
    } else {
        format!(
            "{} {}",
            date.day,
            &MONTH_NAMES[date.month as usize - 1][..3]
        )
    }
}

/// 2 794 rather than 2794 — a headline figure is read, not parsed.
fn thousands(value: f64) -> String {
    let digits = format!("{}", value.round() as i64);

    digits
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(|chunk| String::from_utf8_lossy(chunk).into_owned())
        .collect::<Vec<_>>()
        .join(",")
}

/// The last day the card counts: the most recent Sunday, so a week's column is a whole week. Run
/// to today instead and the current week is a short bar next to eleven full ones, which reads as a
/// collapse in traffic rather than as a Tuesday.
fn window_end() -> Date {
    let today = Date::today();

    today.add_days(-(today.weekday() as i64))
}

/// The labels and the column totals for a range, in one pass over the days it covers.
fn bucketed(range: Range) -> (Vec<String>, Vec<f64>) {
    let first = window_end().add_days(1 - range.days as i64);

    VISITS[VISITS.len() - range.days..]
        .chunks(range.bucket)
        .enumerate()
        .map(|(column, days)| {
            let date = first.add_days((column * range.bucket) as i64);
            (label(date, range.bucket), days.iter().sum::<f64>())
        })
        .unzip()
}

#[component]
fn VisitorsCard() -> impl IntoView {
    // What the control shows, and what the card is drawn from. They are not the same signal
    // because a toggle group can be emptied — pressing the pressed item clears it — and a range
    // always has a value.
    let pressed = RwSignal::new(vec![RANGES[2].slug.to_string()]);
    let range = RwSignal::new(RANGES[2]);

    Effect::new(move |_| match pressed.get().first() {
        Some(slug) => range.set(range_for(slug)),
        None => pressed.set(vec![range.get_untracked().slug.to_string()]),
    });

    // One memo, read three times over: the axis, the bars, and the figure above them.
    let shown = Memo::new(move |_| bucketed(range.get()));
    let labels = Signal::derive(move || shown.get().0);
    let series = Signal::derive(move || vec![ChartSeries::new("visits", "Visits", shown.get().1)]);
    let total = Signal::derive(move || shown.get().1.iter().sum::<f64>());

    view! {
        <Card class="w-full max-w-2xl">
            <CardHeader>
                <CardTitle>"Visitors"</CardTitle>
                <CardDescription>{move || range.get().caption}</CardDescription>
                <CardAction>
                    <ToggleGroup
                        kind=ToggleGroupType::Single
                        variant=ToggleVariant::Outline
                        size=ToggleSize::Sm
                        value=pressed
                    >
                        {RANGES
                            .iter()
                            .map(|range| {
                                let Range { slug, label, .. } = *range;

                                view! { <ToggleGroupItem value=slug>{label}</ToggleGroupItem> }
                            })
                            .collect_view()}
                    </ToggleGroup>
                </CardAction>
            </CardHeader>

            <CardContent class="flex flex-col gap-4">
                <div>
                    <div class="text-2xl font-semibold tabular-nums">
                        {move || thousands(total.get())}
                    </div>
                    <p class="text-xs text-muted-foreground">
                        {move || format!("visits in the last {}", range.get().label)}
                    </p>
                </div>

                // Bars rather than a line: a column is a total over its days, and a line between
                // two totals draws values that were never counted.
                <Chart labels=labels series=series kind=ChartKind::Bar legend=false />
            </CardContent>
        </Card>
    }
}
