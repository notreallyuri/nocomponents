use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::{
    components::input_otp::{InputOtp, InputOtpGroup, InputOtpSeparator, InputOtpSlot},
    primitives::input_otp::OtpPattern,
};

const DEFAULT: &str = r#"<InputOtp length=6 value=code>
    <InputOtpGroup>
        <InputOtpSlot index=0 />
        <InputOtpSlot index=1 />
        <InputOtpSlot index=2 />
        <InputOtpSlot index=3 />
        <InputOtpSlot index=4 />
        <InputOtpSlot index=5 />
    </InputOtpGroup>
</InputOtp>"#;

const SEPARATED: &str = r#"<InputOtp length=6 value=grouped>
    <InputOtpGroup>
        <InputOtpSlot index=0 />
        <InputOtpSlot index=1 />
        <InputOtpSlot index=2 />
    </InputOtpGroup>
    <InputOtpSeparator />
    <InputOtpGroup>
        <InputOtpSlot index=3 />
        <InputOtpSlot index=4 />
        <InputOtpSlot index=5 />
    </InputOtpGroup>
</InputOtp>"#;

const ALPHANUMERIC: &str = r#"<InputOtp length=4 pattern=OtpPattern::Alphanumeric value=licence>
    <InputOtpGroup>
        <InputOtpSlot index=0 />
        <InputOtpSlot index=1 />
        <InputOtpSlot index=2 />
        <InputOtpSlot index=3 />
    </InputOtpGroup>
</InputOtp>"#;

const COMPLETE: &str = r#"<InputOtp
    length=4
    on_complete=Callback::new(move |code: String| submitted.set(code))
>
    <InputOtpGroup>
        <InputOtpSlot index=0 />
        <InputOtpSlot index=1 />
        <InputOtpSlot index=2 />
        <InputOtpSlot index=3 />
    </InputOtpGroup>
</InputOtp>"#;

const DISABLED: &str = r#"<InputOtp length=4 disabled=true value=locked>
    <InputOtpGroup>
        <InputOtpSlot index=0 />
        <InputOtpSlot index=1 />
        <InputOtpSlot index=2 />
        <InputOtpSlot index=3 />
    </InputOtpGroup>
</InputOtp>"#;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "InputOtp",
        description: "A one-time code, typed a character at a time. One real input holds the whole value; the boxes are painted output.",
        props: &[
            Prop {
                name: "length",
                ty: "usize",
                default: "6",
                description: "How many characters the code has. The value is capped at it.",
            },
            Prop {
                name: "pattern",
                ty: "OtpPattern",
                default: "Digits",
                description: "One of: Digits, Alphanumeric, Any.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Stops it taking pointer events and marks it disabled.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the wrapper's classes.",
            },
            Prop {
                name: "value",
                ty: "Option<RwSignal<String>>",
                default: "None",
                description: "Omit for an uncontrolled field that owns its own value.",
            },
            Prop {
                name: "on_complete",
                ty: "Option<Callback<String>>",
                default: "None",
                description: "Called with the finished code once the last slot fills.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Groups, slots and separators.",
            },
        ],
    },
    ApiEntry {
        name: "InputOtpGroup",
        description: "A run of slots, so a code can be drawn as `123 456`.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the group's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Slots.",
            },
        ],
    },
    ApiEntry {
        name: "InputOtpSlot",
        description: "One box. Draws the character at its index, and its own caret while it is the live one.",
        props: &[
            Prop {
                name: "index",
                ty: "usize",
                default: "0",
                description: "Which character of the value this box shows.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the slot's classes.",
            },
        ],
    },
    ApiEntry {
        name: "InputOtpSeparator",
        description: "The mark between two groups. Hidden from assistive tech.",
        props: &[Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Merged over the separator's classes.",
        }],
    },
];

#[component]
pub fn Page() -> impl IntoView {
    let code = RwSignal::new(String::new());
    let grouped = RwSignal::new(String::new());
    let licence = RwSignal::new(String::new());
    let locked = RwSignal::new("2481".to_string());
    let submitted = RwSignal::new(String::new());

    view! {
        <DocLayout title="Input OTP" description="A one-time code, typed a character at a time.">
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="Six boxes over one real input. Paste a whole code into it and every box fills, because the platform is doing the pasting."
                    code=DEFAULT
                >
                    <div class="flex flex-col gap-3">
                        <InputOtp length=6 value=code>
                            <InputOtpGroup>
                                <InputOtpSlot index=0 />
                                <InputOtpSlot index=1 />
                                <InputOtpSlot index=2 />
                                <InputOtpSlot index=3 />
                                <InputOtpSlot index=4 />
                                <InputOtpSlot index=5 />
                            </InputOtpGroup>
                        </InputOtp>
                        <span class="text-sm text-muted-foreground">
                            "Value: " {move || code.get()}
                        </span>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Grouped"
                    description="Groups are presentational only — the value is still one string, so splitting it into 123 456 costs nothing."
                    code=SEPARATED
                >
                    <InputOtp length=6 value=grouped>
                        <InputOtpGroup>
                            <InputOtpSlot index=0 />
                            <InputOtpSlot index=1 />
                            <InputOtpSlot index=2 />
                        </InputOtpGroup>
                        <InputOtpSeparator />
                        <InputOtpGroup>
                            <InputOtpSlot index=3 />
                            <InputOtpSlot index=4 />
                            <InputOtpSlot index=5 />
                        </InputOtpGroup>
                    </InputOtp>
                </DemoSection>

                <DemoSection
                    title="Alphanumeric"
                    description="The pattern decides both what is accepted and which keyboard a phone puts up. Digits get inputmode=numeric; anything else gets a full keyboard."
                    code=ALPHANUMERIC
                >
                    <div class="flex flex-col gap-3">
                        <InputOtp length=4 pattern=OtpPattern::Alphanumeric value=licence>
                            <InputOtpGroup>
                                <InputOtpSlot index=0 />
                                <InputOtpSlot index=1 />
                                <InputOtpSlot index=2 />
                                <InputOtpSlot index=3 />
                            </InputOtpGroup>
                        </InputOtp>
                        <span class="text-sm text-muted-foreground">
                            "Value: " {move || licence.get()}
                        </span>
                    </div>
                </DemoSection>

                <DemoSection
                    title="On complete"
                    description="Fires once, when the last slot fills."
                    code=COMPLETE
                >
                    <div class="flex flex-col gap-3">
                        <InputOtp
                            length=4
                            on_complete=Callback::new(move |code: String| submitted.set(code))
                        >
                            <InputOtpGroup>
                                <InputOtpSlot index=0 />
                                <InputOtpSlot index=1 />
                                <InputOtpSlot index=2 />
                                <InputOtpSlot index=3 />
                            </InputOtpGroup>
                        </InputOtp>
                        <span class="text-sm text-muted-foreground">
                            {move || {
                                let code = submitted.get();
                                if code.is_empty() {
                                    "Waiting for four digits.".to_string()
                                } else {
                                    format!("Submitted {code}")
                                }
                            }}
                        </span>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Disabled"
                    description="The input underneath is disabled, so the whole field stops taking pointer and keyboard alike."
                    code=DISABLED
                >
                    <InputOtp length=4 disabled=true value=locked>
                        <InputOtpGroup>
                            <InputOtpSlot index=0 />
                            <InputOtpSlot index=1 />
                            <InputOtpSlot index=2 />
                            <InputOtpSlot index=3 />
                        </InputOtpGroup>
                    </InputOtp>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
