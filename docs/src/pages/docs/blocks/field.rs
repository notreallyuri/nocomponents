//! Field blocks.

use super::Block;
use leptos::prelude::*;
use nocomponents::{
    components::{
        button::{Button, ButtonVariant},
        checkbox::Checkbox,
        field::{Field, FieldDescription, FieldError, FieldGroup, FieldLabel},
        input::{Input, InputType},
        select::{Select, SelectContent, SelectItem, SelectPortal, SelectTrigger, SelectValue},
        spinner::Spinner,
        textarea::Textarea,
    },
    primitives::toast::use_toast,
    utils::types::Orientation,
};

/// What is wrong with the form, worked out in one place. Empty means it can be sent.
#[derive(Clone, Copy, Default, PartialEq)]
struct Problems {
    name: bool,
    email: bool,
    summary: bool,
}

impl Problems {
    fn any(&self) -> bool {
        self.name || self.email || self.summary
    }
}

const FORM: &str = r#"// Errors are worked out from the values, but only *shown* once a submit
// has been attempted: a form that goes red while you are still filling in
// the first box is telling you off for not having finished yet.
let submitted = RwSignal::new(false);

let problems = Signal::derive(move || Problems {
    name: name.get().trim().is_empty(),
    email: !email.get().contains('@'),
    summary: summary.get().trim().len() < 10,
});

// One derived signal per field: `invalid` on the field paints the label,
// the border and the ring, and is also what makes `FieldError` render and
// the control's `aria-describedby` point at it.
let bad = move |which: fn(&Problems) -> bool| {
    Signal::derive(move || submitted.get() && problems.with(|p| which(p)))
};

view! {
    // `novalidate` because the validation is ours: without it, `type=email` gets the
    // browser's own bubble in first and the submit never reaches this code at all.
    <form novalidate on:submit=move |e| {
        e.prevent_default();
        submitted.set(true);
        if !problems.get_untracked().any() {
            sending.set(true);
            // …and on the way back, a toast, because the form is about to
            // be somewhere else on the page or gone altogether.
            toast.toast("Report filed").description("We will be in touch.").show();
        }
    }>
        <FieldGroup>
            <Field invalid=bad(|p| p.name)>
                <FieldLabel>"Your name"</FieldLabel>
                <Input model=name />
                <FieldError>"We need something to call you."</FieldError>
            </Field>

            <Field invalid=bad(|p| p.email)>
                <FieldLabel>"Email"</FieldLabel>
                <Input model=email input_type=InputType::Email />
                <FieldDescription>"Only used to reply to this report."</FieldDescription>
                <FieldError>"That does not look like an email address."</FieldError>
            </Field>

            // `Select` is a listbox in a portal rather than a native control, and it still
            // wears the field: the label's `for` names the trigger, and `aria-describedby`
            // and `aria-invalid` land on it the same way they land on the `Input` above.
            <Field>
                <FieldLabel>"Where did it happen?"</FieldLabel>
                <Select value=area>
                    <SelectTrigger>
                        <SelectValue placeholder="Pick an area" />
                    </SelectTrigger>
                    <SelectPortal>
                        <SelectContent>
                            <SelectItem value="Documentation site">"Documentation site"</SelectItem>
                            <SelectItem value="The library itself">"The library itself"</SelectItem>
                        </SelectContent>
                    </SelectPortal>
                </Select>
            </Field>

            <Field orientation=Orientation::Horizontal>
                <Checkbox checked=urgent />
                <FieldLabel>"This is blocking my work"</FieldLabel>
            </Field>

            <Button attr:r#type="submit" attr:disabled=sending>"Send report"</Button>
        </FieldGroup>
    </form>
}"#;

pub const BLOCKS: &[Block] = &[Block {
    title: "A form that waits before it complains",
    description: "Four controls, one submit, and the seam is *when* the form is allowed to say \
                  no. The errors are derived from the values and so are always up to date, but \
                  they are gated behind a submit having been attempted — a form that turns red \
                  while you are still typing the first box is telling you off for not having \
                  finished. Send it empty to see all of them at once, then watch them go one at a \
                  time as you fix them, without another submit. Under that, `invalid` on a \
                  `Field` is the only thing any of these controls are told: it paints the label, \
                  the border and the ring, renders the `FieldError`, and adds that error to the \
                  control's `aria-describedby` — *beside* the description rather than in place of \
                  it, so a screen reader still gets both the rule and the complaint. Which is why \
                  `Input`, `Textarea`, `Select` and `Checkbox` need no validation props of \
                  their own — including `Select`, which is a listbox in a portal rather than a \
                  form control, and still ends up named by the label above it. The success path \
                  leaves the form entirely and becomes a toast, since by then the thing you were \
                  looking at is gone.",
    code: FORM,
    view: || view! { <BugReport /> }.into_any(),
}];

#[component]
fn BugReport() -> impl IntoView {
    let toast = use_toast();

    let name = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let area = RwSignal::new(Some("Documentation site".to_string()));
    let summary = RwSignal::new(String::new());
    let urgent = RwSignal::new(false);

    let submitted = RwSignal::new(false);
    let sending = RwSignal::new(false);

    let problems = Signal::derive(move || Problems {
        name: name.with(|name| name.trim().is_empty()),
        email: !email.with(|email| email.contains('@')),
        summary: summary.with(|summary| summary.trim().chars().count() < 10),
    });

    // Shown only after a submit, and then only while still true.
    let bad = move |which: fn(&Problems) -> bool| {
        Signal::derive(move || submitted.get() && problems.with(|problems| which(problems)))
    };

    let submit = move |event: leptos::ev::SubmitEvent| {
        event.prevent_default();
        submitted.set(true);

        if problems.get_untracked().any() {
            return;
        }

        sending.set(true);
        toast
            .toast("Report filed")
            .description("Thanks — we will be in touch by email.")
            .show();

        // Nothing is actually sent, so the button comes back on its own rather than staying
        // spent for the life of the page.
        set_timeout(
            move || {
                sending.set(false);
                submitted.set(false);
                name.set(String::new());
                email.set(String::new());
                summary.set(String::new());
                urgent.set(false);
            },
            std::time::Duration::from_millis(900),
        );
    };

    view! {
        // `novalidate` because the validation here is ours. Without it `type=email` gets the
        // browser's own bubble in first, the submit never reaches this code, and none of the
        // fields below ever get to say what is wrong with them.
        <form class="w-full max-w-md" novalidate="" on:submit=submit>
            <FieldGroup>
                <Field invalid=bad(|problems| problems.name)>
                    <FieldLabel>"Your name"</FieldLabel>
                    <Input model=name />
                    <FieldError>"We need something to call you."</FieldError>
                </Field>

                <Field invalid=bad(|problems| problems.email)>
                    <FieldLabel>"Email"</FieldLabel>
                    <Input model=email input_type=InputType::Email />
                    <FieldDescription>"Only used to reply to this report."</FieldDescription>
                    <FieldError>"That does not look like an email address."</FieldError>
                </Field>

                <Field>
                    <FieldLabel>"Where did it happen?"</FieldLabel>
                    <Select value=area>
                        <SelectTrigger>
                            <SelectValue placeholder="Pick an area" />
                        </SelectTrigger>
                        <SelectPortal>
                            <SelectContent>
                                <SelectItem value="Documentation site">
                                    "Documentation site"
                                </SelectItem>
                                <SelectItem value="The library itself">
                                    "The library itself"
                                </SelectItem>
                                <SelectItem value="Build or tooling">"Build or tooling"</SelectItem>
                            </SelectContent>
                        </SelectPortal>
                    </Select>
                </Field>

                <Field invalid=bad(|problems| problems.summary)>
                    <FieldLabel>"What happened?"</FieldLabel>
                    <Textarea model=summary auto_resize=true class="min-h-20" />
                    <FieldDescription>
                        {move || {
                            let written = summary.with(|summary| summary.trim().chars().count());
                            match written {
                                0..=9 => format!("{} of 10 characters.", written),
                                _ => "Enough to go on.".to_string(),
                            }
                        }}
                    </FieldDescription>
                    <FieldError>"A sentence or so, so we know what to look at."</FieldError>
                </Field>

                <Field orientation=Orientation::Horizontal>
                    <Checkbox checked=urgent />
                    <FieldLabel>"This is blocking my work"</FieldLabel>
                </Field>

                <div class="flex items-center gap-3">
                    <Button attr:r#type="submit" attr:disabled=sending>
                        {move || match sending.get() {
                            true => {
                                view! {
                                    <Spinner class="size-4" />
                                    "Sending…"
                                }
                                    .into_any()
                            }
                            false => view! { "Send report" }.into_any(),
                        }}
                    </Button>
                    <Button
                        variant=ButtonVariant::Ghost
                        attr:r#type="button"
                        on_click=Callback::new(move |_| {
                            submitted.set(false);
                            name.set(String::new());
                            email.set(String::new());
                            summary.set(String::new());
                            urgent.set(false);
                        })
                    >
                        "Clear"
                    </Button>
                    <span class="ml-auto text-xs text-muted-foreground">
                        {move || match (submitted.get(), problems.get().any()) {
                            (true, true) => "Some fields need another look.",
                            _ => "",
                        }}
                    </span>
                </div>
            </FieldGroup>
        </form>
    }
}
