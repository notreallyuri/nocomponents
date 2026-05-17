use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
use leptos::prelude::*;
use nocomponents::components::{
    badge::{Badge, BadgeVariant},
    button::{Button, ButtonVariant},
    card::{
        Card, CardAction, CardContent, CardDescription, CardFooter, CardHeader, CardSize, CardTitle,
    },
};

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout title="Card" description="A container for grouping related content and actions.">
            <div class="flex flex-col gap-8">
                <DemoSection title="Default">
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
                            <Button
                                variant=ButtonVariant::Outline
                                size=nocomponents::components::button::ButtonSize::Sm
                            >
                                "View details"
                            </Button>
                        </CardFooter>
                    </Card>
                </DemoSection>

                <DemoSection title="Small" description="Compact variant for denser layouts.">
                    <Card size=CardSize::Sm class="max-w-sm">
                        <CardHeader>
                            <CardTitle>"Storage"</CardTitle>
                            <CardDescription>"72% of 100 GB used"</CardDescription>
                        </CardHeader>
                        <CardContent>
                            <div class="h-1.5 w-full rounded-full bg-muted overflow-hidden">
                                <div class="h-full w-[72%] rounded-full bg-primary" />
                            </div>
                        </CardContent>
                    </Card>
                </DemoSection>

                <DemoSection
                    title="With Action"
                    description="CardAction anchors to the top-right of the header."
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
