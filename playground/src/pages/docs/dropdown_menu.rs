use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
use leptos::prelude::*;
use nocomponents::components::{
    button::ButtonVariant,
    dropdown_menu::{
        DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuLabel, DropdownMenuPortal,
        DropdownMenuSeparator, DropdownMenuSub, DropdownMenuSubContent, DropdownMenuSubTrigger,
        DropdownMenuTrigger,
    },
};

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout
            title="Dropdown Menu"
            description="Displays a list of actions triggered by a button."
        >
            <div class="flex flex-col gap-8">
                <DemoSection title="Default">
                    <DropdownMenu>
                        <DropdownMenuTrigger variant=ButtonVariant::Outline>
                            "Options"
                        </DropdownMenuTrigger>
                        <DropdownMenuPortal>
                            <DropdownMenuContent class="w-48">
                                <DropdownMenuLabel>"My Account"</DropdownMenuLabel>
                                <DropdownMenuSeparator />
                                <DropdownMenuItem>"Profile"</DropdownMenuItem>
                                <DropdownMenuItem>"Settings"</DropdownMenuItem>
                                <DropdownMenuItem>"Billing"</DropdownMenuItem>
                                <DropdownMenuSeparator />
                                <DropdownMenuItem class="text-destructive hover:bg-destructive/10 hover:text-destructive">
                                    "Log out"
                                </DropdownMenuItem>
                            </DropdownMenuContent>
                        </DropdownMenuPortal>
                    </DropdownMenu>
                </DemoSection>

                <DemoSection title="With submenu">
                    <DropdownMenu>
                        <DropdownMenuTrigger variant=ButtonVariant::Outline>
                            "More"
                        </DropdownMenuTrigger>
                        <DropdownMenuPortal>
                            <DropdownMenuContent class="w-48">
                                <DropdownMenuItem>"Dashboard"</DropdownMenuItem>
                                <DropdownMenuSub>
                                    <DropdownMenuSubTrigger>"Team"</DropdownMenuSubTrigger>
                                    <DropdownMenuSubContent>
                                        <DropdownMenuItem>"Invite members"</DropdownMenuItem>
                                        <DropdownMenuItem>"Manage roles"</DropdownMenuItem>
                                        <DropdownMenuSeparator />
                                        <DropdownMenuItem>"Audit log"</DropdownMenuItem>
                                    </DropdownMenuSubContent>
                                </DropdownMenuSub>
                                <DropdownMenuSeparator />
                                <DropdownMenuItem>"Support"</DropdownMenuItem>
                            </DropdownMenuContent>
                        </DropdownMenuPortal>
                    </DropdownMenu>
                </DemoSection>
            </div>
        </DocLayout>
    }
}
