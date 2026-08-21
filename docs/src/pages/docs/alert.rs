use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
use leptos::prelude::*;
use nocomponents::{
    components::alert::{Alert, AlertDescription, AlertTitle, AlertVariant},
    icons::check::Check,
};

const DEFAULT: &str = r#"<Alert>
    <AlertTitle>"Heads up!"</AlertTitle>
    <AlertDescription>
        "You can add components to your app using the CLI."
    </AlertDescription>
</Alert>"#;

const WITH_ICON: &str = r#"<Alert>
    <Check />
    <AlertTitle>"Changes saved"</AlertTitle>
    <AlertDescription>"Your profile is up to date."</AlertDescription>
</Alert>"#;

const DESTRUCTIVE: &str = r#"<Alert variant=AlertVariant::Destructive>
    <AlertTitle>"Payment failed"</AlertTitle>
    <AlertDescription>
        "Your card was declined. Try a different payment method."
    </AlertDescription>
</Alert>"#;

const TITLE_ONLY: &str = r#"<Alert>
    <AlertTitle>"A short, self-contained message."</AlertTitle>
</Alert>"#;

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout
            title="Alert"
            description="Callout for user attention, in flow with the surrounding content."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="Neutral styling for an informational message."
                    code=DEFAULT
                >
                    <Alert>
                        <AlertTitle>"Heads up!"</AlertTitle>
                        <AlertDescription>
                            "You can add components to your app using the CLI."
                        </AlertDescription>
                    </Alert>
                </DemoSection>

                <DemoSection
                    title="Destructive"
                    description="For errors and failed actions; the whole callout takes the destructive colour."
                    code=DESTRUCTIVE
                >
                    <Alert variant=AlertVariant::Destructive>
                        <AlertTitle>"Payment failed"</AlertTitle>
                        <AlertDescription>
                            "Your card was declined. Try a different payment method."
                        </AlertDescription>
                    </Alert>
                </DemoSection>

                <DemoSection
                    title="With icon"
                    description="An svg child shifts the grid into its icon layout automatically."
                    code=WITH_ICON
                >
                    <Alert>
                        <Check />
                        <AlertTitle>"Changes saved"</AlertTitle>
                        <AlertDescription>"Your profile is up to date."</AlertDescription>
                    </Alert>
                </DemoSection>

                <DemoSection
                    title="Title only"
                    description="The description is optional — a lone title renders on one line."
                    code=TITLE_ONLY
                >
                    <Alert>
                        <AlertTitle>"A short, self-contained message."</AlertTitle>
                    </Alert>
                </DemoSection>
            </div>
        </DocLayout>
    }
}
