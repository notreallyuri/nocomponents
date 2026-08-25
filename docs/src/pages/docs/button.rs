use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::{
    components::{
        button::{Button, ButtonSize, ButtonVariant},
        field::Field,
    },
    icons::x::X,
    utils::types::StyledRender,
};

const VARIANTS: &str = r#"<Button variant=ButtonVariant::Default>"Default"</Button>
<Button variant=ButtonVariant::Secondary>"Secondary"</Button>
<Button variant=ButtonVariant::Outline>"Outline"</Button>
<Button variant=ButtonVariant::Ghost>"Ghost"</Button>
<Button variant=ButtonVariant::Link>"Link"</Button>
<Button variant=ButtonVariant::Destructive>"Destructive"</Button>"#;

const SIZES: &str = r#"<Button size=ButtonSize::Xs>"Extra Small"</Button>
<Button size=ButtonSize::Sm>"Small"</Button>
<Button>"Default"</Button>
<Button size=ButtonSize::Lg>"Large"</Button>"#;

const ICON_SIZES: &str = r#"<Button size=ButtonSize::IconXs>"×"</Button>
<Button size=ButtonSize::IconSm>"×"</Button>
<Button size=ButtonSize::Icon>"×"</Button>
<Button size=ButtonSize::IconLg>"×"</Button>"#;

const DISABLED: &str = r#"<Button disabled=true>"Default"</Button>
<Button variant=ButtonVariant::Secondary disabled=true>
    "Secondary"
</Button>
<Button variant=ButtonVariant::Outline disabled=true>
    "Outline"
</Button>
<Button variant=ButtonVariant::Destructive disabled=true>
    "Destructive"
</Button>"#;

const IN_A_FIELD: &str = r#"<Field disabled=true class="max-w-sm">
    <span class="text-sm font-medium">"Notifications"</span>
    <Button variant=ButtonVariant::Outline>"Send a test"</Button>
</Field>"#;

const AS_A_LINK: &str = r##"<Button render=Callback::new(move |styled: StyledRender| {
    view! {
        // An `<a>` has no `disabled` attribute, so the root writes `aria-disabled`
        // onto the node instead — the class and the node ref are all that has to
        // be forwarded. What the link does about it is this callback's to decide.
        <a
            href=move || (!styled.disabled.get()).then_some("/docs/button")
            class=styled.class
            node_ref=styled.node_ref
        >
            "Read the docs"
        </a>
    }
        .into_any()
}) />"##;

/// Read off the `#[component]` signature in `src/components/button.rs`, in declaration order.
const API: &[ApiEntry] = &[ApiEntry {
    name: "Button",
    description: "A button, or whatever `render` turns it into.",
    props: &[
        Prop {
            name: "variant",
            ty: "ButtonVariant",
            default: "Default",
            description: "Default, Secondary, Outline, Ghost, Link or Destructive.",
        },
        Prop {
            name: "size",
            ty: "ButtonSize",
            default: "Default",
            description: "Xs, Sm, Default, Lg, or the four Icon sizes for a square button.",
        },
        Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Merged over the variant and size classes, so the caller's wins.",
        },
        Prop {
            name: "disabled",
            ty: "Signal<bool>",
            default: "false",
            description: "Resolved against any Field or FieldSet around the button, so a disabled field disables it without being told. The click is refused here rather than only painted away, so a button restyled by the caller refuses it too.",
        },
        Prop {
            name: "type",
            ty: "ButtonType",
            default: "Button",
            description: "Button, Submit or Reset. Written as `r#type`. Not HTML's default: a button that means nothing in particular does not submit the form it happens to be inside.",
        },
        Prop {
            name: "on_click",
            ty: "Option<Callback<MouseEvent>>",
            default: "None",
            description: "Ignored when `render` is given: the rendered element takes the click.",
        },
        Prop {
            name: "node_ref",
            ty: "Option<AnyNodeRef>",
            default: "None",
            description: "Pass one to measure or focus the button from outside.",
        },
        Prop {
            name: "render",
            ty: "Option<Callback<StyledRender, AnyView>>",
            default: "None",
            description: "Render as something else — a link, usually. Handed the merged class, the node ref and the resolved disabled; the class and the node ref must both reach the element, or the state attributes have nowhere to land.",
        },
        Prop {
            name: "children",
            ty: "Option<ChildrenFn>",
            default: "None",
            description: "The label. Optional, since an icon button may have none.",
        },
    ],
}];

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout title="Button" description="Triggers an action or event.">
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Variants"
                    description="Six variants, from the filled default to a link that only underlines on hover."
                    code=VARIANTS
                >
                    <div class="flex flex-wrap gap-3">
                        <Button variant=ButtonVariant::Default>"Default"</Button>
                        <Button variant=ButtonVariant::Secondary>"Secondary"</Button>
                        <Button variant=ButtonVariant::Outline>"Outline"</Button>
                        <Button variant=ButtonVariant::Ghost>"Ghost"</Button>
                        <Button variant=ButtonVariant::Link>"Link"</Button>
                        <Button variant=ButtonVariant::Destructive>"Destructive"</Button>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Sizes"
                    description="Four heights from Xs to Lg; the smaller ones also step down the text size and radius."
                    code=SIZES
                >
                    <div class="flex flex-wrap items-center gap-3">
                        <Button size=ButtonSize::Xs>"Extra Small"</Button>
                        <Button size=ButtonSize::Sm>"Small"</Button>
                        <Button>"Default"</Button>
                        <Button size=ButtonSize::Lg>"Large"</Button>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Icon Sizes"
                    description="Square sizes for a button whose entire content is one glyph."
                    code=ICON_SIZES
                >
                    <div class="flex flex-wrap items-center gap-3">
                        <Button size=ButtonSize::IconXs>
                            <X />
                        </Button>
                        <Button size=ButtonSize::IconSm>
                            <X />
                        </Button>
                        <Button size=ButtonSize::Icon>
                            <X />
                        </Button>
                        <Button size=ButtonSize::IconLg>
                            <X />
                        </Button>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Disabled"
                    description="A disabled button drops to half opacity and stops taking pointer events."
                    code=DISABLED
                >
                    <div class="flex flex-wrap gap-3">
                        <Button disabled=true>"Default"</Button>
                        <Button variant=ButtonVariant::Secondary disabled=true>
                            "Secondary"
                        </Button>
                        <Button variant=ButtonVariant::Outline disabled=true>
                            "Outline"
                        </Button>
                        <Button variant=ButtonVariant::Destructive disabled=true>
                            "Destructive"
                        </Button>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Inside a field"
                    description="A field that is disabled disables the buttons in it. Nothing is passed down by the caller — the button resolves its own `disabled` against the field it finds itself in, the way every other control does."
                    code=IN_A_FIELD
                >
                    <Field disabled=true class="max-w-sm">
                        <span class="text-sm font-medium">"Notifications"</span>
                        <Button variant=ButtonVariant::Outline>"Send a test"</Button>
                    </Field>
                </DemoSection>

                <DemoSection
                    title="As a link"
                    description="`render` swaps the element for whatever the callback returns, usually an `<a>` so middle-click and copy-address keep working. It is handed the merged class, the node ref, and the resolved `disabled`."
                    code=AS_A_LINK
                >
                    <div class="flex flex-wrap items-center gap-3">
                        <Button render=Callback::new(move |styled: StyledRender| {
                            view! {
                                <a
                                    href="https://github.com/notreallyuri/nocomponents"
                                    class=styled.class
                                    node_ref=styled.node_ref
                                    rel="noreferrer"
                                    target="_blank"
                                >
                                    "Source"
                                </a>
                            }
                                .into_any()
                        }) />
                        <Button
                            variant=ButtonVariant::Outline
                            disabled=true
                            render=Callback::new(move |styled: StyledRender| {
                                view! {
                                    <a
                                        href=move || {
                                            (!styled.disabled.get()).then_some("/docs/button")
                                        }
                                        class=styled.class
                                        node_ref=styled.node_ref
                                    >
                                        "Read the docs"
                                    </a>
                                }
                                    .into_any()
                            })
                        />
                    </div>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
