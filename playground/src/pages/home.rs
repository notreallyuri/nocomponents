use crate::components::{
    profile_settings::{SettingsDialog, User},
    theme_switcher::ThemeSwitcher,
};
use leptos::{ev, prelude::*};
use leptos_node_ref::AnyNodeRef;
use nocomponents::{
    components::{
        avatar::{Avatar, AvatarFallback, AvatarImage},
        badge::{Badge, BadgeVariant},
        button::{Button, ButtonSize, ButtonVariant},
        card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle},
        checkbox::Checkbox,
        dialog::{
            Dialog, DialogDescription, DialogFooter, DialogHeader, DialogPortal, DialogTitle,
            DialogTrigger,
        },
        dropdown_menu::{
            DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuLabel,
            DropdownMenuPortal, DropdownMenuSeparator, DropdownMenuSub, DropdownMenuSubContent,
            DropdownMenuSubTrigger, DropdownMenuTrigger,
        },
        input::{Input, InputType},
        label::Label,
        popover::{
            Popover, PopoverContent, PopoverDescription, PopoverHeader, PopoverPortal,
            PopoverTitle, PopoverTrigger,
        },
        select::{Select, SelectContent, SelectItem, SelectPortal, SelectTrigger, SelectValue},
        switch::Switch,
        tabs::{Tabs, TabsContent, TabsList, TabsTrigger},
        toast::{use_toast, ToastPosition},
    },
    utils::types::{Align, Side},
};

#[component]
pub fn Home() -> impl IntoView {
    let settings_open = RwSignal::new(false);

    let current_user = User {
        name: "Yuri VGR".to_string(),
        email: Some("yuriguimaraes@proton.me".to_string()),
        profile_picture: "https://w.wallhaven.cc/full/5d/wallhaven-5d23j9.png".to_string(),
    };

    let toaster = use_toast();

    view! {
        <SettingsDialog user=current_user.clone() open=settings_open />

        <main class="flex flex-col min-h-screen bg-background text-foreground font-sans selection:bg-primary/20">
            <header class="sticky top-0 z-40 w-full border-b bg-background/95 backdrop-blur supports-backdrop-filter:bg-background/60">
                <div class="container flex h-16 items-center justify-between px-8 max-w-7xl mx-auto">
                    <div class="flex gap-2 items-center font-bold tracking-tight">
                        <div class="size-6 rounded-md bg-primary text-primary-foreground flex items-center justify-center">
                            <span class="text-xs">NC</span>
                        </div>
                        "nocomponents"
                    </div>

                    <div class="flex items-center gap-4">
                        <ThemeSwitcher />
                        <DropdownMenu>
                            <DropdownMenuTrigger render=Callback::new(move |
                                (trigger_ref, toggle): (AnyNodeRef, Callback<ev::MouseEvent>)|
                            {
                                view! {
                                    <button
                                        type="button"
                                        node_ref=trigger_ref
                                        on:click=move |e| toggle.run(e)
                                        class="group/avatar relative rounded-full outline-none cursor-pointer focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background"
                                    >
                                        <div class="hidden absolute inset-0 z-10 group-hover/avatar:block bg-black/20 transition-colors rounded-full" />
                                        <Avatar>
                                            <AvatarImage
                                                src="https://w.wallhaven.cc/full/5d/wallhaven-5d23j9.png"
                                                alt="Avatar"
                                                class="object-cover"
                                            />
                                            <AvatarFallback>"YR"</AvatarFallback>
                                        </Avatar>
                                    </button>
                                }
                                    .into_any()
                            }) />
                            <DropdownMenuPortal>
                                <DropdownMenuContent
                                    class="w-48"
                                    align=Align::End
                                    side=Side::Bottom
                                >
                                    <DropdownMenuLabel>"My Account"</DropdownMenuLabel>
                                    <DropdownMenuSeparator />
                                    <DropdownMenuItem>"Profile"</DropdownMenuItem>
                                    <DropdownMenuSub>
                                        <DropdownMenuSubTrigger>
                                            "More Options"
                                        </DropdownMenuSubTrigger>
                                        <DropdownMenuSubContent>
                                            <DropdownMenuItem>"Keyboard Shortcuts"</DropdownMenuItem>
                                            <DropdownMenuItem>"API Tokens"</DropdownMenuItem>
                                        </DropdownMenuSubContent>
                                    </DropdownMenuSub>

                                    <DropdownMenuItem on:click=move |_| {
                                        settings_open.set(true)
                                    }>"Settings"</DropdownMenuItem>
                                    <DropdownMenuSeparator />
                                    <DropdownMenuItem class="text-destructive focus:text-destructive hover:bg-destructive/10 hover:text-destructive">
                                        "Log out"
                                    </DropdownMenuItem>
                                </DropdownMenuContent>
                            </DropdownMenuPortal>
                        </DropdownMenu>
                    </div>
                </div>
            </header>

            <div class="w-full border-b bg-muted/30">
                <div class="container max-w-7xl mx-auto px-8 py-16 flex flex-col gap-4">
                    <h1 class="text-4xl font-bold tracking-tight lg:text-5xl">
                        "Component Showcase"
                    </h1>
                    <p class="text-lg text-muted-foreground max-w-2xl">
                        "A playground demonstrating the capabilities, variants, and interactive states of the nocomponents headless UI library."
                    </p>
                </div>
            </div>

            <div class="container max-w-7xl mx-auto px-8 py-12">
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 items-start">
                    <div class="flex flex-col gap-6">
                        <Card>
                            <CardHeader>
                                <CardTitle>"Buttons"</CardTitle>
                                <CardDescription>"Standard variants and states."</CardDescription>
                            </CardHeader>
                            <CardContent class="flex flex-wrap items-center gap-3">
                                <Button variant=ButtonVariant::Default>"Default"</Button>
                                <Button variant=ButtonVariant::Secondary>"Secondary"</Button>
                                <Button variant=ButtonVariant::Outline>"Outline"</Button>
                                <Button variant=ButtonVariant::Ghost>"Ghost"</Button>
                                <Button variant=ButtonVariant::Link>"Link"</Button>
                                <Button variant=ButtonVariant::Destructive>"Destructive"</Button>
                                <div class="w-full h-px bg-border my-2" />

                                <Button size=ButtonSize::Xs>"Extra Small"</Button>
                                <Button size=ButtonSize::Sm>"Small"</Button>
                                <Button size=ButtonSize::Lg>"Large Button"</Button>
                                <Button attr:disabled=true>"Disabled"</Button>
                            </CardContent>
                        </Card>

                        <Card>
                            <CardHeader>
                                <CardTitle>"Badges"</CardTitle>
                                <CardDescription>"Small status indicators."</CardDescription>
                            </CardHeader>
                            <CardContent class="flex flex-wrap items-center gap-3">
                                <Badge variant=BadgeVariant::Default>"Default"</Badge>
                                <Badge variant=BadgeVariant::Secondary>"Secondary"</Badge>
                                <Badge variant=BadgeVariant::Outline>"Outline"</Badge>
                                <Badge variant=BadgeVariant::Ghost>"Ghost"</Badge>
                                <Badge variant=BadgeVariant::Link>"Link"</Badge>
                                <Badge variant=BadgeVariant::Destructive>"Destructive"</Badge>
                            </CardContent>
                        </Card>

                        <Card>
                            <CardHeader>
                                <CardTitle>"Preferences"</CardTitle>
                                <CardDescription>"Form controls and inputs."</CardDescription>
                            </CardHeader>
                            <CardContent class="flex flex-col gap-6">
                                <div class="flex items-center justify-between">
                                    <div class="flex flex-col gap-1">
                                        <Label prop:htmlFor="airplane-mode" class="text-base">
                                            "Airplane Mode"
                                        </Label>
                                        <span class="text-xs text-muted-foreground">
                                            "Disable all network connectivity."
                                        </span>
                                    </div>
                                    {
                                        let airplane_mode = RwSignal::new(false);
                                        view! {
                                            <Switch id="airplane-mode" checked=airplane_mode />
                                        }
                                    }
                                </div>
                                <div class="w-full h-px bg-border" />
                                <div class="flex items-center space-x-3">
                                    {
                                        let terms_accepted = RwSignal::new(true);
                                        view! { <Checkbox id="terms" checked=terms_accepted /> }
                                    }
                                    <Label
                                        prop:htmlFor="terms"
                                        class="text-sm font-medium leading-none cursor-pointer"
                                    >
                                        "Accept terms and conditions"
                                    </Label>
                                </div>
                            </CardContent>
                        </Card>
                    </div>

                    <div class="flex flex-col gap-6">
                        <Card>
                            <Tabs default_value="login">
                                <CardHeader>
                                    <TabsList class="flex w-full">
                                        <TabsTrigger class="w-full" value="login">
                                            "Login"
                                        </TabsTrigger>
                                        <TabsTrigger class="w-full" value="register">
                                            "Register"
                                        </TabsTrigger>
                                    </TabsList>
                                </CardHeader>
                                <TabsContent value="login">
                                    <CardContent class="flex flex-col gap-4">
                                        <div class="space-y-1.5">
                                            <Label prop:htmlFor="email-lo">"Email"</Label>
                                            <Input
                                                attr:id="email-lo"
                                                attr:placeholder="example@mail.com"
                                            />
                                        </div>
                                        <div class="space-y-1.5">
                                            <Label prop:htmlFor="password-lo">"Password"</Label>
                                            <Input
                                                attr:id="password-lo"
                                                input_type=InputType::Password
                                                attr:placeholder="••••••••"
                                            />
                                        </div>
                                        <Button class="mt-2 w-full">"Confirm"</Button>
                                    </CardContent>
                                </TabsContent>

                                <TabsContent value="register">
                                    <CardContent class="flex flex-col gap-4">
                                        <div class="space-y-1.5">
                                            <Label prop:htmlFor="email-re">"Email"</Label>
                                            <Input
                                                attr:id="email-re"
                                                attr:placeholder="example@mail.com"
                                            />
                                        </div>
                                        <div class="space-y-1.5">
                                            <Label prop:htmlFor="username-re">"Username"</Label>
                                            <Input attr:id="username-re" attr:placeholder="@username" />
                                        </div>
                                        <div class="space-y-1.5">
                                            <Label prop:htmlFor="password-re">"Password"</Label>
                                            <Input
                                                attr:id="password-re"
                                                input_type=InputType::Password
                                                attr:placeholder="••••••••"
                                            />
                                        </div>
                                        <Button class="mt-2 w-full">"Create Account"</Button>
                                    </CardContent>
                                </TabsContent>
                            </Tabs>
                            <CardFooter class="justify-center border-t bg-muted/20 py-3 text-muted-foreground text-xs">
                                <span>"Forgot your password?"</span>
                                <a class="ml-1 text-primary hover:underline underline-offset-4 cursor-pointer font-medium">
                                    "Recover"
                                </a>
                            </CardFooter>
                        </Card>

                        <Card>
                            <CardHeader>
                                <CardTitle>"Select"</CardTitle>
                                <CardDescription>"Accessible dropdown selection."</CardDescription>
                            </CardHeader>
                            <CardContent>
                                <div class="flex flex-col gap-2">
                                    <Label prop:htmlFor="theme">"Theme"</Label>
                                    {
                                        let theme_val = RwSignal::new(Some("System".to_string()));
                                        view! {
                                            <Select value=theme_val>
                                                <SelectTrigger>
                                                    <SelectValue placeholder="Select a theme..." />
                                                </SelectTrigger>
                                                <SelectPortal>
                                                    <SelectContent>
                                                        <SelectItem value="Light">"Light"</SelectItem>
                                                        <SelectItem value="Dark">"Dark"</SelectItem>
                                                        <SelectItem value="System">"System"</SelectItem>
                                                    </SelectContent>
                                                </SelectPortal>
                                            </Select>
                                        }
                                    }
                                </div>
                            </CardContent>
                        </Card>
                    </div>

                    <div class="flex flex-col gap-6">
                        <Card>
                            <CardHeader>
                                <CardTitle>"Popover"</CardTitle>
                                <CardDescription>"Contextual floating content."</CardDescription>
                            </CardHeader>
                            <CardContent class="flex flex-col gap-3">
                                <Popover>
                                    <PopoverTrigger
                                        variant=ButtonVariant::Outline
                                        class="w-full justify-between"
                                    >
                                        "Basic Popover"
                                        <span class="text-muted-foreground">"→"</span>
                                    </PopoverTrigger>
                                    <PopoverPortal>
                                        <PopoverContent side=Side::Top class="w-64">
                                            <PopoverHeader>
                                                <PopoverTitle class="text-base font-bold">
                                                    "Quick Info"
                                                </PopoverTitle>
                                                <PopoverDescription class="mt-1">
                                                    "Popovers are perfect for rich tooltips or small inline forms that don't require a full dialog."
                                                </PopoverDescription>
                                            </PopoverHeader>
                                        </PopoverContent>
                                    </PopoverPortal>
                                </Popover>

                                <Popover>
                                    <PopoverTrigger
                                        variant=ButtonVariant::Secondary
                                        class="w-full justify-between"
                                    >
                                        "Popover Form"
                                        <span class="text-muted-foreground">"→"</span>
                                    </PopoverTrigger>
                                    <PopoverPortal>
                                        <PopoverContent
                                            class="w-72"
                                            align=Align::Start
                                            side=Side::Top
                                        >
                                            <PopoverHeader>
                                                <PopoverTitle class="font-bold">"Dimensions"</PopoverTitle>
                                                <p class="text-xs text-muted-foreground mt-1">
                                                    "Adjust the component size."
                                                </p>
                                            </PopoverHeader>
                                            <div class="px-4 py-3 flex flex-col gap-3">
                                                <div class="flex items-center justify-between gap-4">
                                                    <Label class="text-sm text-muted-foreground">"Width"</Label>
                                                    <Input class="w-40 h-7" attr:placeholder="100%" />
                                                </div>
                                                <div class="flex items-center justify-between gap-4">
                                                    <Label class="text-sm text-muted-foreground">
                                                        "Height"
                                                    </Label>
                                                    <Input class="w-40 h-7" attr:placeholder="auto" />
                                                </div>
                                            </div>
                                        </PopoverContent>
                                    </PopoverPortal>
                                </Popover>
                            </CardContent>
                        </Card>

                        <Card>
                            <CardHeader>
                                <CardTitle>"Dialog"</CardTitle>
                                <CardDescription>
                                    "Modal windows requiring attention."
                                </CardDescription>
                            </CardHeader>
                            <CardContent class="flex flex-col gap-3">
                                <Dialog>
                                    <DialogTrigger class="w-full">"Alert Dialog"</DialogTrigger>
                                    <DialogPortal>
                                        <DialogHeader>
                                            <DialogTitle>"Are you sure?"</DialogTitle>
                                            <DialogDescription class="mt-2">
                                                "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                                            </DialogDescription>
                                        </DialogHeader>
                                        <DialogFooter class="mt-4">
                                            <Button variant=ButtonVariant::Outline>"Cancel"</Button>
                                            <Button variant=ButtonVariant::Destructive>
                                                "Continue"
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
                                            <DialogDescription class="mt-1">
                                                "Make changes to your profile here. Click save when you're done."
                                            </DialogDescription>
                                        </DialogHeader>
                                        <div class="grid gap-4 py-6">
                                            <div class="grid grid-cols-4 items-center gap-4">
                                                <Label prop:htmlFor="name" class="text-right">
                                                    "Name"
                                                </Label>
                                                <Input
                                                    attr:id="name"
                                                    class="col-span-3"
                                                    attr:placeholder="Yuri Ribeiro"
                                                />
                                            </div>
                                            <div class="grid grid-cols-4 items-center gap-4">
                                                <Label prop:htmlFor="username" class="text-right">
                                                    "Username"
                                                </Label>
                                                <Input
                                                    attr:id="username"
                                                    class="col-span-3"
                                                    attr:placeholder="@notreallyuri"
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
                    </div>

                    <div class="md:col-span-2 lg:col-span-3 mt-4">
                        <Card>
                            <CardHeader>
                                <CardTitle>"Toasts & Notifications"</CardTitle>
                                <CardDescription>
                                    "Snackbar notifications that stack dynamically from any corner."
                                </CardDescription>
                            </CardHeader>
                            <CardContent>
                                <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                                    <div class="flex flex-col gap-3">
                                        <Button
                                            variant=ButtonVariant::Outline
                                            on:click=move |_| {
                                                toaster
                                                    .toast("Message Sent")
                                                    .description("We'll get back to you soon.")
                                                    .position(ToastPosition::TopLeft)
                                                    .show();
                                            }
                                        >
                                            "Top Left"
                                        </Button>
                                        <Button
                                            variant=ButtonVariant::Outline
                                            on:click=move |_| {
                                                toaster
                                                    .toast("Archived")
                                                    .description("Conversation moved to archive.")
                                                    .position(ToastPosition::BottomLeft)
                                                    .show();
                                            }
                                        >
                                            "Bottom Left"
                                        </Button>
                                    </div>
                                    <div class="flex flex-col gap-3">
                                        <Button
                                            variant=ButtonVariant::Secondary
                                            on:click=move |_| {
                                                toaster
                                                    .toast("Update Available")
                                                    .description("A new version is ready to install.")
                                                    .position(ToastPosition::TopCenter)
                                                    .show();
                                            }
                                        >
                                            "Top Center"
                                        </Button>
                                        <Button
                                            variant=ButtonVariant::Secondary
                                            on:click=move |_| {
                                                toaster
                                                    .toast("Warning")
                                                    .description("Your session is about to expire.")
                                                    .position(ToastPosition::BottomCenter)
                                                    .show();
                                            }
                                        >
                                            "Bottom Center"
                                        </Button>
                                    </div>
                                    <div class="flex flex-col gap-3">
                                        <Button on:click=move |_| {
                                            toaster
                                                .toast("Success!")
                                                .description("Your profile has been saved.")
                                                .position(ToastPosition::TopRight)
                                                .show();
                                        }>"Top Right"</Button>
                                        <Button on:click=move |_| {
                                            toaster
                                                .toast("Error")
                                                .description("Could not connect to the server.")
                                                .position(ToastPosition::BottomRight)
                                                .show();
                                        }>"Bottom Right"</Button>
                                    </div>
                                </div>
                            </CardContent>
                        </Card>
                    </div>
                </div>
            </div>
        </main>
    }
}
