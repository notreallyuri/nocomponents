use leptos::prelude::*;
use ui::prelude::{
    Button, ButtonSize, ButtonVariant, Card, CardContent, CardFooter, CardHeader, CardTitle, Input,
    Label, Popover, PopoverContent, PopoverTrigger,
};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <main class="p-2 flex flex-col gap-2 bg-background h-full text-primary">
            <h1 class="text-2xl font-semibold">Test project</h1>
            <div class="flex gap-4">
                <Card class="w-48">
                    <CardHeader>
                        <CardTitle>Buttons</CardTitle>
                    </CardHeader>

                    <CardContent class="flex flex-col gap-2">
                        <Button variant=ButtonVariant::Default>Testing</Button>
                        <Button variant=ButtonVariant::Secondary>Testing</Button>
                        <Button variant=ButtonVariant::Outline>Testing</Button>
                        <Button variant=ButtonVariant::Ghost>Testing</Button>
                        <Button variant=ButtonVariant::Link>Testing</Button>
                    </CardContent>
                </Card>

                <Card class="w-48">
                    <CardHeader>
                        <CardTitle>Login Form</CardTitle>
                    </CardHeader>
                    <CardContent class="flex flex-col gap-2">
                        <Label html_for="email">Email</Label>
                        <Input id="email" />

                        <Label html_for="password">Password</Label>
                        <Input id="password" />
                    </CardContent>
                    <CardFooter>
                        <Button class="w-full">Confirm</Button>
                    </CardFooter>
                </Card>
            </div>

            <Popover>
                <PopoverTrigger>"Quick Info"</PopoverTrigger>

                <PopoverContent>
                    <h3 class="font-bold text-lg">"Hello World"</h3>
                    <p class="text-sm text-muted-foreground">
                        "This is a basic popover with default button styles."
                    </p>
                </PopoverContent>
            </Popover>

            <Input class="w-64" placeholder="Testing" />

            <Popover>
                <PopoverTrigger variant=ButtonVariant::Secondary size=ButtonSize::Sm>
                    "Settings"
                </PopoverTrigger>

                <PopoverContent class="w-64">
                    <div class="flex flex-col gap-4">
                        <div class="border-b border-border pb-2">
                            <h3 class="font-bold">"Preferences"</h3>
                            <p class="text-xs text-muted-foreground">
                                "Manage your account settings."
                            </p>
                        </div>

                        <div class="flex items-center justify-between">
                            <span class="text-sm">"Email Notifications"</span>
                            <input type="checkbox" class="size-4 rounded border-border" />
                        </div>

                        <Button
                            variant=ButtonVariant::Outline
                            size=ButtonSize::Sm
                            class="w-full mt-2"
                        >
                            "Save Changes"
                        </Button>
                    </div>
                </PopoverContent>
            </Popover>
        </main>
    }
}
