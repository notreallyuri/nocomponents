//! Dialog blocks.

use super::Block;
use leptos::{either::Either, prelude::*};
use nocomponents::{
    components::{
        button::{Button, ButtonVariant},
        dialog::{
            Dialog, DialogDescription, DialogFooter, DialogHeader, DialogPortal, DialogTitle,
        },
        drawer::{Drawer, DrawerContent},
        field::{Field, FieldDescription, FieldGroup, FieldLabel},
        input::Input,
    },
    utils::{types::Side, use_media_query},
};

/// Where a dialog stops being the right shape. The same breakpoint Tailwind calls `sm`, so the
/// classes inside the body and the choice of shell change at the same width.
const PHONE: &str = "(max-width: 639px)";

const RESPONSIVE: &str = r#"// A dialog on a desktop and a drawer on a phone. Not two components with
// one hidden — that leaves two of everything in the DOM and two of
// everything focusable — but one branch over one piece of state.
let open = RwSignal::new(false);
let phone = use_media_query("(max-width: 639px)");

// The fields are their own component, so the two branches share a body
// rather than a copy of one. The state they edit lives out here, above
// both shells, and so survives the swap.
view! {
    <Button on_click=Callback::new(move |_| open.set(true))>"Edit profile"</Button>

    {move || match phone.get() {
        false => Either::Left(view! {
            <Dialog open=open>
                <DialogPortal class="sm:max-w-md">
                    <DialogHeader>
                        <DialogTitle>"Edit profile"</DialogTitle>
                        <DialogDescription>"Saved to your account."</DialogDescription>
                    </DialogHeader>
                    <ProfileFields name=name handle=handle />
                    <DialogFooter>
                        <Button on_click=save>"Save"</Button>
                    </DialogFooter>
                </DialogPortal>
            </Dialog>
        }),
        true => Either::Right(view! {
            <Drawer open=open side=Side::Bottom>
                <DrawerContent class="px-4 pb-6">
                    <DialogHeader class="text-left">
                        <DialogTitle>"Edit profile"</DialogTitle>
                        <DialogDescription>"Saved to your account."</DialogDescription>
                    </DialogHeader>
                    <ProfileFields name=name handle=handle />
                    <Button class="mt-4 w-full" on_click=save>"Save"</Button>
                </DrawerContent>
            </Drawer>
        }),
    }}
}"#;

pub const BLOCKS: &[Block] = &[Block {
    slug: "a-dialog-that-becomes-a-drawer",
    title: "A dialog that becomes a drawer",
    uses: &["button", "dialog", "drawer", "field", "input"],
    description: "Narrow the window past 640px and the same edit lands as a sheet from the \
                  bottom instead of a box in the middle. The seam is that this is a change of \
                  *component*, not of class: a drawer is dragged shut, a dialog is not, and no \
                  amount of `sm:` on a dialog will give it a handle. So the choice is made in \
                  Rust — `use_media_query` — and only one of the two is ever in the DOM. \
                  Rendering both and hiding one with `hidden sm:block` is the usual shortcut and \
                  it leaves two copies of every field on the page, two tab stops for each, and a \
                  screen reader reading the form twice. The fields are their own component so \
                  the two branches share a body rather than a copy of one, and everything they \
                  edit — the values, and whether the panel is open at all — lives above both \
                  shells. Crossing the breakpoint really does tear one down and build the other, \
                  so anything held inside would go with it; held outside, the panel that arrives \
                  is already open and still has what you typed. `DrawerContent` reuses \
                  `DialogHeader` and `DialogTitle` rather than growing its own: a drawer *is* a \
                  dialog here, right down to the context, and only its shell and its drag differ.",
    code: RESPONSIVE,
    view: || view! { <ResponsiveEdit /> }.into_any(),
}];

/// The form, written once. Rendered inside whichever shell is showing.
#[component]
fn ProfileFields(name: RwSignal<String>, handle: RwSignal<String>) -> impl IntoView {
    view! {
        <FieldGroup class="py-2">
            <Field>
                <FieldLabel>"Display name"</FieldLabel>
                <Input model=name />
            </Field>
            <Field>
                <FieldLabel>"Handle"</FieldLabel>
                <Input model=handle />
                <FieldDescription>"How people find you."</FieldDescription>
            </Field>
        </FieldGroup>
    }
}

#[component]
fn ResponsiveEdit() -> impl IntoView {
    let open = RwSignal::new(false);
    let phone = use_media_query(PHONE);

    let name = RwSignal::new("Yuri".to_string());
    let handle = RwSignal::new("notreallyuri".to_string());
    let saved = RwSignal::new(None::<String>);

    let save = Callback::new(move |_| {
        saved.set(Some(format!(
            "{} (@{})",
            name.get_untracked(),
            handle.get_untracked()
        )));
        open.set(false);
    });

    view! {
        <div class="flex flex-col items-start gap-3">
            <div class="flex flex-wrap items-center gap-3">
                <Button on_click=Callback::new(move |_| open.set(true))>"Edit profile"</Button>
                <span class="rounded-md border px-2 py-1 font-mono text-xs text-muted-foreground">
                    {move || match phone.get() {
                        true => "under 640px → drawer",
                        false => "640px and up → dialog",
                    }}
                </span>
            </div>

            <p class="text-sm text-muted-foreground">
                {move || match saved.get() {
                    Some(saved) => format!("Saved: {saved}"),
                    None => "Nothing saved yet.".to_string(),
                }}
            </p>

            {move || match phone.get() {
                false => {
                    Either::Left(
                        view! {
                            <Dialog open=open>
                                <DialogPortal class="sm:max-w-md">
                                    <DialogHeader>
                                        <DialogTitle>"Edit profile"</DialogTitle>
                                        <DialogDescription>
                                            "Two fields, one shell — this one is the dialog."
                                        </DialogDescription>
                                    </DialogHeader>
                                    <ProfileFields name=name handle=handle />
                                    <DialogFooter>
                                        <Button
                                            variant=ButtonVariant::Outline
                                            on_click=Callback::new(move |_| open.set(false))
                                        >
                                            "Cancel"
                                        </Button>
                                        <Button on_click=save>"Save"</Button>
                                    </DialogFooter>
                                </DialogPortal>
                            </Dialog>
                        },
                    )
                }
                true => {
                    Either::Right(
                        view! {
                            <Drawer open=open side=Side::Bottom>
                                <DrawerContent class="px-4 pb-6">
                                    <DialogHeader class="text-left">
                                        <DialogTitle>"Edit profile"</DialogTitle>
                                        <DialogDescription>
                                            "Two fields, one shell — this one is the drawer.
                                            Drag it down to dismiss."
                                        </DialogDescription>
                                    </DialogHeader>
                                    <ProfileFields name=name handle=handle />
                                    <Button class="mt-4 w-full" on_click=save>
                                        "Save"
                                    </Button>
                                </DrawerContent>
                            </Drawer>
                        },
                    )
                }
            }}
        </div>
    }
}
