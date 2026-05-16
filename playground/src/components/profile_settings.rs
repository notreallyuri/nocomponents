use leptos::{ev, prelude::*};
use nocomponents::components::{
    avatar::{Avatar, AvatarFallback, AvatarImage, AvatarSize},
    button::{Button, ButtonSize, ButtonVariant},
    dialog::{Dialog, DialogHeader, DialogPortal, DialogTitle, DialogTrigger},
    input::{Input, InputType},
    label::Label,
    tabs::{Tabs, TabsContent, TabsList, TabsTrigger},
};

#[derive(Clone, PartialEq)]
pub struct User {
    pub name: String,
    pub email: Option<String>,
    pub profile_picture: String,
}

#[component]
pub fn SettingsDialog(
    user: User,
    #[prop(default = RwSignal::new(false))] open: RwSignal<bool>,
    #[prop(optional)] render: Option<Callback<Callback<ev::MouseEvent>, AnyView>>,
) -> impl IntoView {
    let user = StoredValue::new(user);

    view! {
        <Dialog open=open>
            <Show when=move || render.is_some()>
                <DialogTrigger render=render.unwrap() />
            </Show>

            <DialogPortal class="flex flex-col min-h-102 sm:max-w-106">
                <DialogHeader>
                    <DialogTitle>"Settings"</DialogTitle>
                </DialogHeader>

                <Tabs default_value="profile" class="h-full mt-4">
                    <TabsList class="w-full justify-start">
                        <TabsTrigger value="profile">"Profile"</TabsTrigger>
                        <TabsTrigger value="account">"Account"</TabsTrigger>
                    </TabsList>

                    <TabsContent value="account" class="flex h-full flex-col gap-4 mt-4">
                        <div class="flex flex-col gap-3">
                            <div class="space-y-1">
                                <h3 class="text-sm font-bold font-montserrat text-foreground">
                                    "Access"
                                </h3>
                                <p class="text-xs text-muted-foreground font-manrope">
                                    "This is the email linked to your account"
                                </p>
                            </div>
                            <div class="space-y-1.5">
                                <Label>"Main email"</Label>
                                <Input
                                    input_type=InputType::Email
                                    value=user.with_value(|u| u.email.clone().unwrap_or_default())
                                    class="bg-muted/30 font-medium"
                                />
                            </div>
                        </div>

                        <div class="my-2 h-px w-full bg-border" />

                        <div class="flex flex-col gap-3">
                            <div class="space-y-1">
                                <h3 class="text-sm font-bold font-montserrat text-foreground">
                                    "Security"
                                </h3>
                                <p class="text-xs text-muted-foreground font-manrope">
                                    "Change your password"
                                </p>
                            </div>

                            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                                <div class="space-y-1.5">
                                    <Label>"New password"</Label>
                                    <Input
                                        input_type=InputType::Password
                                        attr:placeholder="••••••••"
                                    />
                                </div>
                                <div class="space-y-1.5">
                                    <Label>"Confirm password"</Label>
                                    <Input
                                        input_type=InputType::Password
                                        attr:placeholder="••••••••"
                                    />
                                </div>
                            </div>
                        </div>

                        <Button class="mt-4 w-full sm:w-auto sm:ml-auto cursor-pointer shadow-sm">
                            "Save changes"
                        </Button>
                    </TabsContent>

                    <TabsContent value="profile" class="flex h-full flex-col gap-4 mt-4">
                        <div class="flex items-center gap-4 py-2">
                            <Avatar size=AvatarSize::Lg class="size-20 border-2 border-muted">
                                <AvatarImage
                                    src=user.with_value(|u| u.profile_picture.clone())
                                    alt=user.with_value(|u| u.name.clone())
                                    class="object-cover"
                                />
                                <AvatarFallback
                                    delay_ms=150
                                    class="text-xl font-bold font-montserrat"
                                >
                                    {move || {
                                        user.with_value(|u| {
                                            u.name
                                                .chars()
                                                .next()
                                                .unwrap_or('?')
                                                .to_string()
                                                .to_uppercase()
                                        })
                                    }}
                                </AvatarFallback>
                            </Avatar>
                            <div class="flex flex-col gap-1.5">
                                <Button
                                    variant=ButtonVariant::Outline
                                    size=ButtonSize::Sm
                                    class="w-fit cursor-pointer"
                                >
                                    "Change picture"
                                </Button>
                                <span class="text-[10px] text-muted-foreground uppercase tracking-widest">
                                    "JPG or PNG. Max 2MB."
                                </span>
                            </div>
                        </div>

                        <div class="space-y-1.5">
                            <Label>"Public Name"</Label>
                            <Input attr:placeholder="Your name or nickname" />
                        </div>

                        <div class="space-y-1.5">
                            <Label>"Bio"</Label>
                            <Input attr:placeholder="Write about you..." />
                        </div>

                        <Button class="mt-4 w-full sm:w-auto sm:ml-auto cursor-pointer shadow-sm">
                            "Save changes"
                        </Button>
                    </TabsContent>
                </Tabs>
            </DialogPortal>
        </Dialog>
    }
}
