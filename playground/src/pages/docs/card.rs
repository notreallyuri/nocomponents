use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
use leptos::prelude::*;
use nocomponents::components::{
    badge::{Badge, BadgeVariant},
    button::{Button, ButtonSize, ButtonVariant},
    card::{
        Card, CardAction, CardContent, CardDescription, CardFooter, CardHeader, CardSize, CardTitle,
    },
    prelude::Progress,
};

const DEFAULT: &str = r#"<Card class="max-w-sm">
    <CardHeader>
        <CardTitle>"Project Alpha"</CardTitle>
        <CardDescription>
            "A long-running initiative tracking Q3 milestones."
        </CardDescription>
    </CardHeader>
    <CardContent>
        <p class="text-muted-foreground text-sm">
            "Last updated 3 hours ago. 4 open tasks remain."
        </p>
    </CardContent>
    <CardFooter>
        <Button
            variant=ButtonVariant::Outline
            size=ButtonSize::Sm
        >
            "View details"
        </Button>
    </CardFooter>
</Card>"#;

const SMALL: &str = r#"<Card size=CardSize::Sm class="max-w-sm">
    <CardHeader>
        <CardTitle>"Storage"</CardTitle>
        <CardDescription>"72% of 100 GB used"</CardDescription>
    </CardHeader>
    <CardContent>
        <Progress value=72.0 class="h-1.5" />
    </CardContent>
</Card>"#;

const WITH_ACTION: &str = r#"<Card class="max-w-sm">
    <CardHeader>
        <CardTitle>"Notifications"</CardTitle>
        <CardDescription>"Manage how you receive alerts."</CardDescription>
        <CardAction>
            <Badge variant=BadgeVariant::Secondary>"3 new"</Badge>
        </CardAction>
    </CardHeader>
    <CardContent>
        <p class="text-sm text-muted-foreground">
            "Email and push notifications are currently enabled."
        </p>
    </CardContent>
</Card>"#;

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout title="Card" description="A container for grouping related content and actions.">
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="Header, content and footer are separate slots — use only the ones you need."
                    code=DEFAULT
                >
                    <Card class="max-w-sm">
                        <CardHeader>
                            <CardTitle>"Project Alpha"</CardTitle>
                            <CardDescription>
                                "A long-running initiative tracking Q3 milestones."
                            </CardDescription>
                        </CardHeader>
                        <CardContent>
                            <p class="text-muted-foreground text-sm">
                                "Last updated 3 hours ago. 4 open tasks remain."
                            </p>
                        </CardContent>
                        <CardFooter>
                            <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>
                                "View details"
                            </Button>
                        </CardFooter>
                    </Card>
                </DemoSection>

                <DemoSection
                    title="Small"
                    description="Compact variant for denser layouts."
                    code=SMALL
                >
                    <Card size=CardSize::Sm class="max-w-sm">
                        <CardHeader>
                            <CardTitle>"Storage"</CardTitle>
                            <CardDescription>"72% of 100 GB used"</CardDescription>
                        </CardHeader>
                        <CardContent>
                            <Progress value=72.0 class="h-1.5" />
                        </CardContent>
                    </Card>
                </DemoSection>

                <DemoSection
                    title="With Action"
                    description="CardAction anchors to the top-right of the header."
                    code=WITH_ACTION
                >
                    <Card class="max-w-sm">
                        <CardHeader>
                            <CardTitle>"Notifications"</CardTitle>
                            <CardDescription>"Manage how you receive alerts."</CardDescription>
                            <CardAction>
                                <Badge variant=BadgeVariant::Secondary>"3 new"</Badge>
                            </CardAction>
                        </CardHeader>
                        <CardContent>
                            <p class="text-sm text-muted-foreground">
                                "Email and push notifications are currently enabled."
                            </p>
                        </CardContent>
                    </Card>
                </DemoSection>
            </div>
        </DocLayout>
    }
}
