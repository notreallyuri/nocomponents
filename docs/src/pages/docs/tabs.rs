use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::components::{
    button::Button,
    input::{Input, InputType},
    label::Label,
    tabs::{Tabs, TabsContent, TabsList, TabsTrigger},
};
use nocomponents::primitives::tabs::TabsListVariants;
use nocomponents::utils::types::Orientation;

const DEFAULT: &str = r#"<Tabs default_value="account" class="w-full max-w-md">
    <TabsList>
        <TabsTrigger value="account">"Account"</TabsTrigger>
        <TabsTrigger value="password">"Password"</TabsTrigger>
    </TabsList>
    <TabsContent value="account" class="mt-4 flex flex-col gap-3">
        <div class="flex flex-col gap-1.5">
            <Label>"Display name"</Label>
            <Input attr:placeholder="Your name" />
        </div>
        <div class="flex flex-col gap-1.5">
            <Label>"Email"</Label>
            <Input attr:placeholder="you@example.com" />
        </div>
        <Button class="w-fit mt-2">"Save"</Button>
    </TabsContent>
    <TabsContent value="password" class="mt-4 flex flex-col gap-3">
        <div class="flex flex-col gap-1.5">
            <Label>"Current password"</Label>
            <Input
                input_type=InputType::Password
                attr:placeholder="••••••••"
            />
        </div>
        <div class="flex flex-col gap-1.5">
            <Label>"New password"</Label>
            <Input
                input_type=InputType::Password
                attr:placeholder="••••••••"
            />
        </div>
        <Button class="w-fit mt-2">"Update password"</Button>
    </TabsContent>
</Tabs>"#;

const LINE_VARIANT: &str = r#"<Tabs default_value="overview" class="w-full max-w-md">
    <TabsList variant=TabsListVariants::Line>
        <TabsTrigger value="overview">"Overview"</TabsTrigger>
        <TabsTrigger value="activity">"Activity"</TabsTrigger>
        <TabsTrigger value="settings">"Settings"</TabsTrigger>
    </TabsList>
    <TabsContent value="overview" class="mt-4">
        <p class="text-sm text-muted-foreground">"Overview content."</p>
    </TabsContent>
    <TabsContent value="activity" class="mt-4">
        <p class="text-sm text-muted-foreground">"Activity feed."</p>
    </TabsContent>
    <TabsContent value="settings" class="mt-4">
        <p class="text-sm text-muted-foreground">"Settings panel."</p>
    </TabsContent>
</Tabs>"#;

const VERTICAL: &str = r#"<Tabs
    default_value="general"
    orientation=Orientation::Vertical
    class="w-full max-w-md"
>
    <TabsList class="w-36">
        <TabsTrigger value="general">"General"</TabsTrigger>
        <TabsTrigger value="privacy">"Privacy"</TabsTrigger>
        <TabsTrigger value="advanced">"Advanced"</TabsTrigger>
    </TabsList>
    <TabsContent value="general" class="ml-4">
        <p class="text-sm text-muted-foreground">"General settings."</p>
    </TabsContent>
    <TabsContent value="privacy" class="ml-4">
        <p class="text-sm text-muted-foreground">"Privacy controls."</p>
    </TabsContent>
    <TabsContent value="advanced" class="ml-4">
        <p class="text-sm text-muted-foreground">"Advanced configuration."</p>
    </TabsContent>
</Tabs>"#;

const DISABLED_TAB: &str = r#"<Tabs default_value="active" class="w-full max-w-md">
    <TabsList>
        <TabsTrigger value="active">"Active"</TabsTrigger>
        <TabsTrigger value="disabled" disabled=true>
            "Disabled"
        </TabsTrigger>
        <TabsTrigger value="other">"Other"</TabsTrigger>
    </TabsList>
    <TabsContent value="active" class="mt-4">
        <p class="text-sm text-muted-foreground">"This tab is active."</p>
    </TabsContent>
    <TabsContent value="other" class="mt-4">
        <p class="text-sm text-muted-foreground">"Another tab."</p>
    </TabsContent>
</Tabs>"#;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "Tabs",
        description: "One panel at a time, chosen from a row of triggers.",
        props: &[
            Prop {
                name: "default_value",
                ty: "String",
                default: "",
                description: "Which tab starts selected. Match a trigger's `value`.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the wrapper's classes.",
            },
            Prop {
                name: "orientation",
                ty: "Orientation",
                default: "Horizontal",
                description: "One of: Horizontal, Vertical.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "A list and its panels.",
            },
        ],
    },
    ApiEntry {
        name: "TabsList",
        description: "The row of triggers. One tab stop: the arrows walk it, and selection follows focus.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the list's classes.",
            },
            Prop {
                name: "variant",
                ty: "TabsListVariants",
                default: "Default",
                description: "One of: Default, Line.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Triggers.",
            },
        ],
    },
    ApiEntry {
        name: "TabsTrigger",
        description: "Selects the panel with the same `value`.",
        props: &[
            Prop {
                name: "value",
                ty: "String",
                default: "",
                description: "Which panel this selects.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the trigger's classes.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Skipped by the arrow keys as well as the pointer.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The tab's label.",
            },
        ],
    },
    ApiEntry {
        name: "TabsContent",
        description: "The panel for one trigger. Only the selected one renders.",
        props: &[
            Prop {
                name: "value",
                ty: "String",
                default: "",
                description: "Which trigger this belongs to.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the panel's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The panel's contents.",
            },
        ],
    },
];

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout
            title="Tabs"
            description="Organises content into separate panels accessible by a tab list."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="default_value picks the panel that is open on mount."
                    code=DEFAULT
                >
                    <Tabs default_value="account" class="w-full max-w-md">
                        <TabsList>
                            <TabsTrigger value="account">"Account"</TabsTrigger>
                            <TabsTrigger value="password">"Password"</TabsTrigger>
                        </TabsList>
                        <TabsContent value="account" class="mt-4 flex flex-col gap-3">
                            <div class="flex flex-col gap-1.5">
                                <Label>"Display name"</Label>
                                <Input attr:placeholder="Your name" />
                            </div>
                            <div class="flex flex-col gap-1.5">
                                <Label>"Email"</Label>
                                <Input attr:placeholder="you@example.com" />
                            </div>
                            <Button class="w-fit mt-2">"Save"</Button>
                        </TabsContent>
                        <TabsContent value="password" class="mt-4 flex flex-col gap-3">
                            <div class="flex flex-col gap-1.5">
                                <Label>"Current password"</Label>
                                <Input
                                    input_type=InputType::Password
                                    attr:placeholder="••••••••"
                                />
                            </div>
                            <div class="flex flex-col gap-1.5">
                                <Label>"New password"</Label>
                                <Input
                                    input_type=InputType::Password
                                    attr:placeholder="••••••••"
                                />
                            </div>
                            <Button class="w-fit mt-2">"Update password"</Button>
                        </TabsContent>
                    </Tabs>
                </DemoSection>

                <DemoSection
                    title="Line variant"
                    description="The line variant drops the filled list and underlines the active trigger instead."
                    code=LINE_VARIANT
                >
                    <Tabs default_value="overview" class="w-full max-w-md">
                        <TabsList variant=TabsListVariants::Line>
                            <TabsTrigger value="overview">"Overview"</TabsTrigger>
                            <TabsTrigger value="activity">"Activity"</TabsTrigger>
                            <TabsTrigger value="settings">"Settings"</TabsTrigger>
                        </TabsList>
                        <TabsContent value="overview" class="mt-4">
                            <p class="text-sm text-muted-foreground">"Overview content."</p>
                        </TabsContent>
                        <TabsContent value="activity" class="mt-4">
                            <p class="text-sm text-muted-foreground">"Activity feed."</p>
                        </TabsContent>
                        <TabsContent value="settings" class="mt-4">
                            <p class="text-sm text-muted-foreground">"Settings panel."</p>
                        </TabsContent>
                    </Tabs>
                </DemoSection>

                <DemoSection
                    title="Vertical"
                    description="A vertical orientation stacks the list beside the panels."
                    code=VERTICAL
                >
                    <Tabs
                        default_value="general"
                        orientation=Orientation::Vertical
                        class="w-full max-w-md"
                    >
                        <TabsList class="w-36">
                            <TabsTrigger value="general">"General"</TabsTrigger>
                            <TabsTrigger value="privacy">"Privacy"</TabsTrigger>
                            <TabsTrigger value="advanced">"Advanced"</TabsTrigger>
                        </TabsList>
                        <TabsContent value="general" class="ml-4">
                            <p class="text-sm text-muted-foreground">"General settings."</p>
                        </TabsContent>
                        <TabsContent value="privacy" class="ml-4">
                            <p class="text-sm text-muted-foreground">"Privacy controls."</p>
                        </TabsContent>
                        <TabsContent value="advanced" class="ml-4">
                            <p class="text-sm text-muted-foreground">"Advanced configuration."</p>
                        </TabsContent>
                    </Tabs>
                </DemoSection>

                <DemoSection
                    title="Disabled tab"
                    description="A disabled trigger dims and cannot be selected."
                    code=DISABLED_TAB
                >
                    <Tabs default_value="active" class="w-full max-w-md">
                        <TabsList>
                            <TabsTrigger value="active">"Active"</TabsTrigger>
                            <TabsTrigger value="disabled" disabled=true>
                                "Disabled"
                            </TabsTrigger>
                            <TabsTrigger value="other">"Other"</TabsTrigger>
                        </TabsList>
                        <TabsContent value="active" class="mt-4">
                            <p class="text-sm text-muted-foreground">"This tab is active."</p>
                        </TabsContent>
                        <TabsContent value="other" class="mt-4">
                            <p class="text-sm text-muted-foreground">"Another tab."</p>
                        </TabsContent>
                    </Tabs>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
