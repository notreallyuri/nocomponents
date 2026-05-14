use crate::components::theme_switcher::ThemeSwitcher;
use leptos::prelude::*;
use ui::{
    components::{
        button::{Button, ButtonVariant},
        card::{Card, CardContent, CardFooter, CardHeader, CardTitle},
        checkbox::Checkbox,
        dialog::{
            Dialog, DialogDescription, DialogFooter, DialogHeader, DialogPortal, DialogTitle,
            DialogTrigger,
        },
        dropdown_menu::{
            DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuLabel,
            DropdownMenuPortal, DropdownMenuSeparator, DropdownMenuTrigger,
        },
        input::Input,
        label::Label,
        popover::{
            Popover, PopoverContent, PopoverDescription, PopoverFooter, PopoverHeader,
            PopoverPortal, PopoverTitle, PopoverTrigger,
        },
        select::{Select, SelectContent, SelectItem, SelectTrigger, SelectValue},
        switch::Switch,
        tabs::{Tabs, TabsContent, TabsList, TabsTrigger},
        toast::{use_toast, ToastPosition},
    },
    utils::types::{Align, Side},
};

#[component]
pub fn Home() -> impl IntoView {
    let toaster = use_toast();

    view! {
        <main class="flex flex-col gap-4 bg-background h-full">
            <header class="p-4 w-full items-center h-20 flex justify-end bg-muted">
                <ThemeSwitcher class="ml-auto" />
            </header>
            <div class="p-4 flex flex-col gap-4">
                <h1 class="text-2xl font-semibold">Test project</h1>

                <div class="flex gap-4">
                    <Card class="w-80">
                        <CardHeader>
                            <CardTitle>Buttons</CardTitle>
                        </CardHeader>
                        <CardContent class="flex flex-col gap-y-2">
                            <Button variant=ButtonVariant::Default>Testing</Button>
                            <Button variant=ButtonVariant::Secondary>Testing</Button>
                            <Button variant=ButtonVariant::Outline>Testing</Button>
                            <Button variant=ButtonVariant::Ghost>Testing</Button>
                            <Button variant=ButtonVariant::Link>Testing</Button>
                            <Button variant=ButtonVariant::Destructive>Testing</Button>
                        </CardContent>
                    </Card>

                    <Card class="w-80">
                        <Tabs default_value="login">
                            <CardHeader>
                                <TabsList class="flex w-full">
                                    <TabsTrigger class="w-full" value="login">
                                        Login
                                    </TabsTrigger>
                                    <TabsTrigger class="w-full" value="register">
                                        Register
                                    </TabsTrigger>
                                </TabsList>
                            </CardHeader>
                            <TabsContent value="login">
                                <CardContent class="h-full flex flex-col gap-2">
                                    <Label html_for="email">Email</Label>
                                    <Input id="email" placeholder="example@mail.com" />

                                    <Label html_for="password">Password</Label>
                                    <Input id="password" placeholder="******" />

                                    <Button class="mt-auto w-full">Confirm</Button>
                                </CardContent>
                            </TabsContent>

                            <TabsContent value="register">
                                <CardContent class="flex flex-col gap-2">
                                    <Label html_for="email">Email</Label>
                                    <Input id="email" placeholder="example@mail.com" />

                                    <Label html_for="username">Username</Label>
                                    <Input id="username" />

                                    <Label html_for="password">Password</Label>
                                    <Input id="password" placeholder="******" />

                                    <Button class="w-full">Confirm</Button>
                                </CardContent>
                            </TabsContent>
                        </Tabs>
                        <CardFooter class="justify-center text-muted-foreground text-sm">
                            <span>Forgot your password ?</span>
                            <a class="ml-0.5 text-primary hover:underline underline-offset-4 cursor-pointer">
                                Recover
                            </a>
                        </CardFooter>
                    </Card>

                    <Card class="w-80">
                        <CardHeader>
                            <CardTitle>"Select"</CardTitle>
                        </CardHeader>
                        <CardContent class="flex flex-col gap-4">
                            <div class="flex flex-col gap-1.5">
                                <Label html_for="theme">"Theme"</Label>
                                {
                                    let theme_val = RwSignal::new(Some("System".to_string()));

                                    view! {
                                        <Select value=theme_val>
                                            <SelectTrigger>
                                                <SelectValue placeholder="Select a theme..." />
                                            </SelectTrigger>
                                            <SelectContent>
                                                <SelectItem value="Light">"Light"</SelectItem>
                                                <SelectItem value="Dark">"Dark"</SelectItem>
                                                <SelectItem value="System">"System"</SelectItem>
                                            </SelectContent>
                                        </Select>
                                    }
                                }

                            </div>
                        </CardContent>
                    </Card>

                    <Card class="w-80">
                        <CardHeader>
                            <CardTitle>"Preferences"</CardTitle>
                        </CardHeader>
                        <CardContent class="flex flex-col gap-6">
                            <div class="flex items-center justify-between">
                                <div class="flex flex-col gap-0.5">
                                    <Label html_for="airplane-mode" class="text-base">
                                        "Airplane Mode"
                                    </Label>
                                    <span class="text-sm text-muted-foreground">
                                        "Disable all network connectivity."
                                    </span>
                                </div>
                                {
                                    let airplane_mode = RwSignal::new(false);
                                    view! { <Switch id="airplane-mode" checked=airplane_mode /> }
                                }
                            </div>

                            <div class="flex items-center space-x-2">
                                {
                                    let terms_accepted = RwSignal::new(true);
                                    view! { <Checkbox id="terms" checked=terms_accepted /> }
                                }
                                <Label
                                    html_for="terms"
                                    class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                                >
                                    "Accept terms and conditions"
                                </Label>
                            </div>
                        </CardContent>
                    </Card>
                </div>

                <div class="flex gap-4">
                    <Card class="w-80">
                        <CardHeader>
                            <CardTitle>"Popover"</CardTitle>
                        </CardHeader>
                        <CardContent class="flex flex-col gap-2">
                            <Popover>
                                <PopoverTrigger class="w-full">"Basic Popover"</PopoverTrigger>

                                <PopoverPortal>
                                    <PopoverContent side=Side::Top>
                                        <PopoverHeader>
                                            <PopoverTitle class="text-lg font-bold">
                                                "Hello World"
                                            </PopoverTitle>
                                            <PopoverDescription>
                                                "This is a basic popover with default button styles."
                                            </PopoverDescription>
                                        </PopoverHeader>
                                    </PopoverContent>
                                </PopoverPortal>
                            </Popover>

                            <Popover>
                                <PopoverTrigger class="w-full">"Popover Form"</PopoverTrigger>

                                <PopoverPortal>
                                    <PopoverContent class="w-64" align=Align::Start side=Side::Top>
                                        <PopoverHeader>
                                            <PopoverTitle class="font-bold">"Preferences"</PopoverTitle>
                                            <p class="text-xs text-muted-foreground">
                                                "Manage your account settings."
                                            </p>
                                        </PopoverHeader>

                                        <div class="flex flex-col gap-2">
                                            <div class="flex justify-between gap-4">
                                                <Label
                                                    html_for="popover-w"
                                                    class="text-sm text-muted-foreground"
                                                >
                                                    Width
                                                </Label>
                                                <Input id="popover-w" class="w-40" placeholder="0.0" />
                                            </div>
                                            <div class="flex justify-between gap-4">
                                                <Label
                                                    html_for="popover-h"
                                                    class="text-sm text-muted-foreground"
                                                >
                                                    Height
                                                </Label>
                                                <Input id="popover-h" class="w-40" placeholder="0.0" />
                                            </div>
                                        </div>

                                        <PopoverFooter>
                                            <Button variant=ButtonVariant::Outline class="w-full">
                                                "Save Changes"
                                            </Button>
                                        </PopoverFooter>
                                    </PopoverContent>
                                </PopoverPortal>
                            </Popover>
                        </CardContent>
                    </Card>
                    <Card class="w-80">
                        <CardHeader>
                            <CardTitle>"Dialog"</CardTitle>
                        </CardHeader>
                        <CardContent class="flex flex-col gap-2">
                            <Dialog>
                                <DialogTrigger class="w-full">"Basic Dialog"</DialogTrigger>
                                <DialogPortal>
                                    <DialogHeader>
                                        <DialogTitle>"Information"</DialogTitle>
                                        <DialogDescription>
                                            "This is a simple modal dialog to test the backdrop and centering."
                                        </DialogDescription>
                                    </DialogHeader>
                                    <DialogFooter>
                                        <Button variant=ButtonVariant::Outline>
                                            "Acknowledge"
                                        </Button>
                                    </DialogFooter>
                                </DialogPortal>
                            </Dialog>
                            <Dialog>
                                <DialogTrigger class="w-full" variant=ButtonVariant::Secondary>
                                    "Edit Profile"
                                </DialogTrigger>
                                <DialogPortal>
                                    <DialogHeader>
                                        <DialogTitle>"Edit profile"</DialogTitle>
                                        <DialogDescription>
                                            "Make changes to your profile here. Click save when you're done."
                                        </DialogDescription>
                                    </DialogHeader>

                                    <div class="grid gap-4 py-4">
                                        <div class="grid grid-cols-4 items-center gap-4">
                                            <Label html_for="name" class="text-right">
                                                "Name"
                                            </Label>
                                            <Input
                                                id="name"
                                                class="col-span-3"
                                                placeholder="Yuri Ribeiro"
                                            />
                                        </div>
                                        <div class="grid grid-cols-4 items-center gap-4">
                                            <Label html_for="username" class="text-right">
                                                "Username"
                                            </Label>
                                            <Input
                                                id="username"
                                                class="col-span-3"
                                                placeholder="@notreallyuri"
                                            />
                                        </div>
                                    </div>

                                    <DialogFooter>
                                        <Button>"Save changes"</Button>
                                    </DialogFooter>
                                </DialogPortal>
                            </Dialog>
                        </CardContent>
                    </Card>

                    <Card class="w-80">
                        <CardHeader>
                            <CardTitle>"Dropdown Menu"</CardTitle>
                        </CardHeader>
                        <CardContent class="flex gap-2 items-start">
                            <DropdownMenu>
                                <DropdownMenuTrigger class="w-18">"Start"</DropdownMenuTrigger>

                                <DropdownMenuPortal>
                                    <DropdownMenuContent align=Align::Start side=Side::Bottom>
                                        <DropdownMenuLabel>"Start"</DropdownMenuLabel>
                                        <DropdownMenuSeparator />
                                        <DropdownMenuItem on_click=move |_| {
                                            leptos::logging::log!("Profile clicked!")
                                        }>"Profile"</DropdownMenuItem>
                                        <DropdownMenuItem on_click=move |_| {
                                            leptos::logging::log!("Settings clicked!")
                                        }>"Settings"</DropdownMenuItem>

                                        <DropdownMenuSeparator />
                                        <DropdownMenuItem
                                            class="text-destructive focus:text-destructive hover:bg-destructive/10 hover:text-destructive"
                                            on_click=move |_| leptos::logging::log!("Logout clicked!")
                                        >
                                            "Log out"
                                        </DropdownMenuItem>
                                    </DropdownMenuContent>
                                </DropdownMenuPortal>
                            </DropdownMenu>
                            <DropdownMenu>
                                <DropdownMenuTrigger class="w-18">"Center"</DropdownMenuTrigger>
                                <DropdownMenuPortal>
                                    <DropdownMenuContent align=Align::Center side=Side::Bottom>
                                        <DropdownMenuLabel>"My Account"</DropdownMenuLabel>
                                        <DropdownMenuSeparator />
                                        <DropdownMenuItem on_click=move |_| {
                                            leptos::logging::log!("Profile clicked!")
                                        }>"Profile"</DropdownMenuItem>

                                        <DropdownMenuItem on_click=move |_| {
                                            leptos::logging::log!("Settings clicked!")
                                        }>"Settings"</DropdownMenuItem>

                                        <DropdownMenuSeparator />

                                        <DropdownMenuItem
                                            class="text-destructive focus:text-destructive hover:bg-destructive/10 hover:text-destructive"
                                            on_click=move |_| leptos::logging::log!("Logout clicked!")
                                        >
                                            "Log out"
                                        </DropdownMenuItem>
                                    </DropdownMenuContent>
                                </DropdownMenuPortal>
                            </DropdownMenu>
                            <DropdownMenu>
                                <DropdownMenuTrigger class="w-18">"End"</DropdownMenuTrigger>
                                <DropdownMenuPortal>
                                    <DropdownMenuContent align=Align::End side=Side::Bottom>
                                        <DropdownMenuLabel>"My Account"</DropdownMenuLabel>
                                        <DropdownMenuSeparator />

                                        <DropdownMenuItem on_click=move |_| {
                                            leptos::logging::log!("Profile clicked!")
                                        }>"Profile"</DropdownMenuItem>

                                        <DropdownMenuItem on_click=move |_| {
                                            leptos::logging::log!("Settings clicked!")
                                        }>"Settings"</DropdownMenuItem>

                                        <DropdownMenuSeparator />

                                        <DropdownMenuItem
                                            class="text-destructive focus:text-destructive hover:bg-destructive/10 hover:text-destructive"
                                            on_click=move |_| leptos::logging::log!("Logout clicked!")
                                        >
                                            "Log out"
                                        </DropdownMenuItem>
                                    </DropdownMenuContent>
                                </DropdownMenuPortal>
                            </DropdownMenu>
                        </CardContent>
                    </Card>
                </div>
                <Card class="w-164">
                    <CardHeader>
                        <CardTitle>"Toast"</CardTitle>
                    </CardHeader>
                    <CardContent>
                        <div class="w-1/2 grid grid-cols-3 gap-2">
                            <div class="flex flex-col gap-2">

                                <Button on_click=move |_| {
                                    toaster
                                        .toast("Success!")
                                        .description("Your profile has been saved.")
                                        .position(ToastPosition::TopCenter)
                                        .show();
                                }>"Top center"</Button>

                                <Button on_click=move |_| {
                                    toaster
                                        .toast("Success!")
                                        .description("Your profile has been saved.")
                                        .position(ToastPosition::BottomLeft)
                                        .show();
                                }>"Bottom left"</Button>

                            </div>

                            <div class="flex flex-col gap-2">
                                <Button on_click=move |_| {
                                    toaster
                                        .toast("Success!")
                                        .description("Your profile has been saved.")
                                        .position(ToastPosition::TopLeft)
                                        .show();
                                }>"Top left"</Button>

                                <Button on_click=move |_| {
                                    toaster
                                        .toast("Success!")
                                        .description("Your profile has been saved.")
                                        .position(ToastPosition::BottomCenter)
                                        .show();
                                }>"Bottom center"</Button>

                            </div>

                            <div class="flex flex-col gap-2">

                                <Button on_click=move |_| {
                                    toaster
                                        .toast("Success!")
                                        .description("Your profile has been saved.")
                                        .position(ToastPosition::TopRight)
                                        .show();
                                }>"Top right"</Button>

                                <Button on_click=move |_| {
                                    toaster
                                        .toast("Success!")
                                        .description("Your profile has been saved.")
                                        .position(ToastPosition::BottomRight)
                                        .show();
                                }>"Bottom right"</Button>
                            </div>
                        </div>

                    </CardContent>
                </Card>
            </div>

        </main>
    }
}
