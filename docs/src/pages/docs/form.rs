//! `noform` against these components. The two crates never mention each other: what they share is
//! a `RwSignal<T>` for the value and a `Signal<bool>` for whether to say something about it, which
//! is exactly what `Input` takes as `model` and what `Field` takes as `invalid`.

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
        button::{Button, ButtonType, ButtonVariant},
        checkbox::Checkbox,
        field::{Field, FieldDescription, FieldError, FieldGroup, FieldLabel},
        input::Input,
        native_select::NativeSelect,
    },
    utils::types::Orientation,
};
use noform::{Form, FormConfig, FormHandle, RevalidateOn, ValidateOn, use_form, use_form_with};

// ---------------------------------------------------------------------------------------------
// The data. A form is a plain struct; the derive turns it into one handle per field.
// ---------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Default, PartialEq, Form)]
struct Signup {
    #[validate(required, email)]
    email: String,
    #[validate(required, length(min = 8, max = 64))]
    password: String,
    #[validate(required)]
    plan: String,
    #[validate(required)]
    terms: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Form)]
struct Nickname {
    #[validate(required, length(min = 3, message = "Three characters or more"))]
    nickname: String,
}

#[derive(Clone, Debug, Default, PartialEq, Form)]
struct Account {
    #[validate(required, email)]
    email: String,
}

#[derive(Clone, Debug, Default, PartialEq, Form)]
struct Profile {
    #[validate(required)]
    display_name: String,
    bio: String,
}

// ---------------------------------------------------------------------------------------------
// The snippets, in module-level consts like every other page here.
// ---------------------------------------------------------------------------------------------

const WHOLE: &str = r##"#[derive(Form, Clone, Default)]
struct Signup {
    #[validate(required, email)]
    email: String,
    #[validate(required, length(min = 8, max = 64))]
    password: String,
    #[validate(required)]
    plan: String,
    #[validate(required)]
    terms: bool,
}

let form = use_form(Signup::default());

view! {
    // `novalidate`: the rules are the form's, not the browser's.
    <form novalidate on:submit=form.on_submit(move |values: Signup| {
        submitted.set(Some(values));
    })>
        <Field invalid=form.email.invalid>
            <FieldLabel>"Email"</FieldLabel>
            <Input
                model=form.email.value
                on:blur=move |_| form.email.blur()
            />
            <FieldError>{move || form.email.message.get()}</FieldError>
        </Field>

        <Button r#type=ButtonType::Submit>"Create account"</Button>
    </form>
}"##;

const WHEN: &str = r##"// The same field three times, differing only in when it speaks.
let on_submit = use_form(Nickname::default());                    // the default
let on_blur = use_form_with(
    Nickname::default(),
    FormConfig::new().validate_on(ValidateOn::Blur),
);
let on_change = use_form_with(
    Nickname::default(),
    FormConfig::new().validate_on(ValidateOn::Change),
);

// `Blur` is reachable because leptos forwards `on:` written on a component
// to the root element of the view it returns — here, the `<input>` itself.
view! {
    <Input model=on_blur.nickname.value on:blur=move |_| on_blur.nickname.blur() />
}"##;

const AFTER: &str = r##"// Once something has objected, when does it stop objecting?
let corrects = use_form_with(
    Nickname::default(),
    FormConfig::new().revalidate_on(RevalidateOn::Change),   // the default
);
let stays = use_form_with(
    Nickname::default(),
    FormConfig::new().revalidate_on(RevalidateOn::Never),
);"##;

const SERVER: &str = r##"// What the rules cannot know. Errors arrive keyed by field name, and
// `set_field_errors` hands back the names it could not place.
let submit = move |_| {
    if let Some(values) = form.try_submit() {
        // ... the request that comes back with this:
        let unplaced = form.core().set_field_errors([
            ("email", "That address is already registered"),
        ]);
        debug_assert!(unplaced.is_empty());
    }
};

// Nothing retracts it but the value it was about changing: an external error
// is stored with the value it applied to, because nothing here can re-derive it."##;

const DIRTY: &str = r##"// `is_dirty` is measured against the values the form was built with, so a
// round trip back to them is not dirty.
<Button disabled=Signal::derive(move || !form.is_dirty().get()) r#type=ButtonType::Submit>
    "Save"
</Button>

<Button on_click=Callback::new(move |_| form.reset())>"Discard"</Button>

// `reset_with` takes new values and makes them the baseline too, which is
// what a save that succeeded wants.
form.reset_with(saved);"##;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "Field<T>",
        description: "One field's handle, which is what `#[derive(Form)]` builds per struct field. Everything but `value` is derived from it, the rules, and whether anything has checked the field yet — there are no effects, so what is on screen cannot lag a value the user already fixed.",
        props: &[
            Prop {
                name: "value",
                ty: "RwSignal<T>",
                default: "",
                description: "The value itself. Goes straight into `Input`'s `model`.",
            },
            Prop {
                name: "invalid",
                ty: "Signal<bool>",
                default: "",
                description: "Whether to be saying something right now — the rules failing *and* something having asked. Goes into `Field`'s `invalid`.",
            },
            Prop {
                name: "message",
                ty: "Signal<String>",
                default: "",
                description: "The first failing rule's wording, or empty. Goes inside `FieldError`.",
            },
            Prop {
                name: "errors",
                ty: "Signal<Vec<FieldError>>",
                default: "",
                description: "Every failing rule, in the order they were added. Each carries a stable `code` beside the wording.",
            },
            Prop {
                name: "dirty",
                ty: "Signal<bool>",
                default: "",
                description: "Whether the value differs from the one the field was built with.",
            },
            Prop {
                name: "touched",
                ty: "RwSignal<bool>",
                default: "false",
                description: "Set by `blur()`. What `ValidateOn::Blur` waits for.",
            },
            Prop {
                name: "blur()",
                ty: "fn(&self)",
                default: "",
                description: "What a control's `on:blur` calls: marks the field touched, and validates it if that is when this field validates.",
            },
            Prop {
                name: "set_error(..)",
                ty: "fn(&self, impl Into<FieldError>)",
                default: "",
                description: "An error from outside the rules. Retracts itself as soon as the value it was about changes.",
            },
        ],
    },
    ApiEntry {
        name: "FormHandle",
        description: "The generated handle. `use_form(data)` builds one; the methods below are the trait's, not generated.",
        props: &[
            Prop {
                name: "on_submit(f)",
                ty: "fn(..) -> impl Fn(SubmitEvent)",
                default: "",
                description: "The `on:submit` handler: stops the navigation, runs every rule, and calls `f` with the values only if they survived.",
            },
            Prop {
                name: "try_submit()",
                ty: "fn(&self) -> Option<Data>",
                default: "",
                description: "The same gate without the event, for a button that is not in a form.",
            },
            Prop {
                name: "values()",
                ty: "fn(&self) -> Data",
                default: "",
                description: "The struct back, read untracked.",
            },
            Prop {
                name: "is_valid() / is_dirty()",
                ty: "fn(&self) -> Signal<bool>",
                default: "",
                description: "Across every field. `is_valid` is quiet on a form nothing has asked about yet.",
            },
            Prop {
                name: "reset() / reset_with(v)",
                ty: "fn(&self)",
                default: "",
                description: "Back to the baseline, or onto new values that become the baseline.",
            },
            Prop {
                name: "core()",
                ty: "fn(&self) -> FormCore",
                default: "",
                description: "The config, the field register, the submit count, and `set_field_errors` for a server response keyed by field name.",
            },
        ],
    },
];

// ---------------------------------------------------------------------------------------------

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout
            title="Form State"
            description="noform bound to these components: one signal for the value, one for whether to complain."
        >
            <div class="flex flex-col gap-8">
                <p class="text-sm leading-relaxed text-muted-foreground">
                    "Nothing on this page is a component. "
                    <span class="font-medium text-foreground">"noform"</span>
                    " is a separate crate that holds a form's state and runs its rules; it knows
                    nothing about this library, and this library knows nothing about it. They meet
                    at two signals — "
                    <code class="rounded bg-muted px-1 py-0.5 text-xs">"RwSignal<T>"</code>
                    " for the value, which is what "
                    <code class="rounded bg-muted px-1 py-0.5 text-xs">"Input"</code>
                    " takes as its model, and "
                    <code class="rounded bg-muted px-1 py-0.5 text-xs">"Signal<bool>"</code>
                    " for whether to say something, which is what "
                    <code class="rounded bg-muted px-1 py-0.5 text-xs">"Field"</code> " takes as "
                    <code class="rounded bg-muted px-1 py-0.5 text-xs">"invalid"</code>
                    ". Every demo below works the same against a bare "
                    <code class="rounded bg-muted px-1 py-0.5 text-xs">"<input>"</code> "."
                </p>

                <WholeForm />
                <WhenItChecks />
                <AfterACorrection />
                <ServerErrors />
                <DirtyAndReset />
            </div>

            <ApiReference entries=API />
        </DocLayout>
    }
}

/// The headline: a whole form, bound and gated, in the default configuration.
#[component]
fn WholeForm() -> impl IntoView {
    let form = use_form(Signup::default());
    let submitted = RwSignal::new(None::<Signup>);

    view! {
        <DemoSection
            title="The whole thing"
            description="Nothing is said until you submit; after that, a correction clears its error as soon as it is a correction. Press Create account on the empty form and then start filling it in."
            code=WHOLE
        >
            <div class="w-full max-w-sm">
                // `novalidate`: the rules belong to the form, not to the browser.
                <form
                    novalidate
                    on:submit=form
                        .on_submit(move |values: Signup| {
                            submitted.set(Some(values));
                        })
                >
                    <FieldGroup>
                        <Field invalid=form.email.invalid>
                            <FieldLabel>"Email"</FieldLabel>
                            <Input model=form.email.value on:blur=move |_| form.email.blur() />
                            <FieldError>{move || form.email.message.get()}</FieldError>
                        </Field>

                        <Field invalid=form.password.invalid>
                            <FieldLabel>"Password"</FieldLabel>
                            <Input
                                input_type=nocomponents::components::input::InputType::Password
                                model=form.password.value
                                on:blur=move |_| form.password.blur()
                            />
                            <FieldDescription>"At least eight characters."</FieldDescription>
                            <FieldError>{move || form.password.message.get()}</FieldError>
                        </Field>

                        <Field invalid=form.plan.invalid>
                            <FieldLabel>"Plan"</FieldLabel>
                            <NativeSelect model=form.plan.value>
                                <option value="">"Choose a plan…"</option>
                                <option value="free">"Free"</option>
                                <option value="pro">"Pro"</option>
                            </NativeSelect>
                            <FieldError>{move || form.plan.message.get()}</FieldError>
                        </Field>

                        <Field orientation=Orientation::Horizontal invalid=form.terms.invalid>
                            <Checkbox checked=form.terms.value />
                            <FieldLabel>"I accept the terms"</FieldLabel>
                        </Field>
                        <Field invalid=form.terms.invalid>
                            <FieldError>{move || form.terms.message.get()}</FieldError>
                        </Field>

                        <div class="flex items-center gap-2">
                            <Button r#type=ButtonType::Submit>"Create account"</Button>
                            <Button
                                variant=ButtonVariant::Outline
                                on_click=Callback::new(move |_| {
                                    form.reset();
                                    submitted.set(None);
                                })
                            >
                                "Reset"
                            </Button>
                        </div>
                    </FieldGroup>
                </form>

                {move || {
                    submitted
                        .get()
                        .map(|values| {
                            view! {
                                <p class="mt-4 rounded-lg border border-dashed px-3 py-2 text-xs text-muted-foreground">
                                    "Submitted: "
                                    <span class="font-medium text-foreground">
                                        {format!("{} on the {} plan", values.email, values.plan)}
                                    </span>
                                </p>
                            }
                        })
                }}
            </div>
        </DemoSection>
    }
}

/// The three `ValidateOn` settings, side by side. `Blur` is the one that needed a control able to
/// tell the field that focus had left it.
#[component]
fn WhenItChecks() -> impl IntoView {
    let on_submit = use_form(Nickname::default());
    let on_blur = use_form_with(
        Nickname::default(),
        FormConfig::new().validate_on(ValidateOn::Blur),
    );
    let on_change = use_form_with(
        Nickname::default(),
        FormConfig::new().validate_on(ValidateOn::Change),
    );

    view! {
        <DemoSection
            title="When it checks"
            description="One field, one rule, three settings. Type two characters in each and then click away: the first says nothing until you submit, the second waits for focus to leave, the third objects as you type."
            code=WHEN
        >
            <div class="flex w-full flex-col gap-6 sm:flex-row">
                <div class="flex-1">
                    <Field invalid=on_submit.nickname.invalid>
                        <FieldLabel>"Submit (default)"</FieldLabel>
                        <Input
                            model=on_submit.nickname.value
                            on:blur=move |_| on_submit.nickname.blur()
                        />
                        <FieldError>{move || on_submit.nickname.message.get()}</FieldError>
                    </Field>
                    <Button
                        class="mt-2"
                        variant=ButtonVariant::Outline
                        on_click=Callback::new(move |_| {
                            on_submit.validate();
                        })
                    >
                        "Submit"
                    </Button>
                </div>

                <div class="flex-1">
                    <Field invalid=on_blur.nickname.invalid>
                        <FieldLabel>"Blur"</FieldLabel>
                        <Input
                            model=on_blur.nickname.value
                            on:blur=move |_| on_blur.nickname.blur()
                        />
                        <FieldError>{move || on_blur.nickname.message.get()}</FieldError>
                    </Field>
                </div>

                <div class="flex-1">
                    <Field invalid=on_change.nickname.invalid>
                        <FieldLabel>"Change"</FieldLabel>
                        <Input
                            model=on_change.nickname.value
                            on:blur=move |_| on_change.nickname.blur()
                        />
                        <FieldError>{move || on_change.nickname.message.get()}</FieldError>
                    </Field>
                </div>
            </div>
        </DemoSection>
    }
}

/// `RevalidateOn`: having objected once, what makes it stop.
#[component]
fn AfterACorrection() -> impl IntoView {
    let corrects = use_form_with(
        Nickname::default(),
        FormConfig::new().validate_on(ValidateOn::Change),
    );
    let stays = use_form_with(
        Nickname::default(),
        FormConfig::new()
            .validate_on(ValidateOn::Change)
            .revalidate_on(RevalidateOn::Never),
    );

    view! {
        <DemoSection
            title="And after a correction"
            description="Type one character in each, then keep typing until both are long enough. The first clears itself; the second was told never to re-check on its own and waits to be asked."
            code=AFTER
        >
            <div class="flex w-full flex-col gap-6 sm:flex-row">
                <div class="flex-1">
                    <Field invalid=corrects.nickname.invalid>
                        <FieldLabel>"Change (default)"</FieldLabel>
                        <Input model=corrects.nickname.value />
                        <FieldError>{move || corrects.nickname.message.get()}</FieldError>
                    </Field>
                </div>

                <div class="flex-1">
                    <Field invalid=stays.nickname.invalid>
                        <FieldLabel>"Never"</FieldLabel>
                        <Input model=stays.nickname.value />
                        <FieldError>{move || stays.nickname.message.get()}</FieldError>
                    </Field>
                    <Button
                        class="mt-2"
                        variant=ButtonVariant::Outline
                        on_click=Callback::new(move |_| {
                            stays.validate();
                        })
                    >
                        "Ask again"
                    </Button>
                </div>
            </div>
        </DemoSection>
    }
}

/// The half of validation the rules cannot do: what only the server knows.
#[component]
fn ServerErrors() -> impl IntoView {
    let form = use_form(Account {
        email: "ada@example.com".to_owned(),
    });

    view! {
        <DemoSection
            title="Errors from the server"
            description="Press Save: the rules pass, and the answer comes back anyway. Then change one character — the error retracts itself, because it was stored with the value it was about and nothing here can re-derive it."
            code=SERVER
        >
            <div class="w-full max-w-sm">
                <FieldGroup>
                    <Field invalid=form.email.invalid>
                        <FieldLabel>"Email"</FieldLabel>
                        <Input model=form.email.value on:blur=move |_| form.email.blur() />
                        <FieldError>{move || form.email.message.get()}</FieldError>
                    </Field>

                    <Button on_click=Callback::new(move |_| {
                        if let Some(values) = form.try_submit() && values.email == "ada@example.com"
                        {
                            let unplaced = form
                                .core()
                                .set_field_errors([
                                    ("email", "That address is already registered"),
                                ]);
                            debug_assert!(unplaced.is_empty(), "every name was a field");
                        }
                    })>"Save"</Button>
                </FieldGroup>
            </div>
        </DemoSection>
    }
}

/// `dirty` across the form, and the two ways of putting it back.
#[component]
fn DirtyAndReset() -> impl IntoView {
    let saved = Profile {
        display_name: "Ada Lovelace".to_owned(),
        bio: "Writes notes on the Analytical Engine.".to_owned(),
    };
    let form = use_form(saved);

    view! {
        <DemoSection
            title="Dirty, and putting it back"
            description="Save is disabled until something actually differs from what the form was built with — type a character and undo it, and it goes quiet again. Discard restores; Save makes the current values the new baseline."
            code=DIRTY
        >
            <div class="w-full max-w-sm">
                <form
                    novalidate
                    on:submit=form
                        .on_submit(move |values: Profile| {
                            form.reset_with(values);
                        })
                >
                    <FieldGroup>
                        <Field invalid=form.display_name.invalid>
                            <FieldLabel>"Display name"</FieldLabel>
                            <Input
                                model=form.display_name.value
                                on:blur=move |_| form.display_name.blur()
                            />
                            <FieldError>{move || form.display_name.message.get()}</FieldError>
                        </Field>

                        <Field>
                            <FieldLabel>"Bio"</FieldLabel>
                            <Input model=form.bio.value />
                        </Field>

                        <div class="flex items-center gap-2">
                            <Button
                                r#type=ButtonType::Submit
                                disabled=move || !form.is_dirty().get()
                            >
                                "Save"
                            </Button>
                            <Button
                                variant=ButtonVariant::Outline
                                disabled=move || !form.is_dirty().get()
                                on_click=Callback::new(move |_| form.reset())
                            >
                                "Discard"
                            </Button>
                            <span class="text-xs text-muted-foreground">
                                {move || match form.is_dirty().get() {
                                    true => "unsaved changes",
                                    false => "no changes",
                                }}
                            </span>
                        </div>
                    </FieldGroup>
                </form>
            </div>
        </DemoSection>
    }
}
