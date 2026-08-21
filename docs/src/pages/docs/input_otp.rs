use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
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
            </div>
        </DocLayout>
    }
}
