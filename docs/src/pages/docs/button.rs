use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::components::button::{Button, ButtonSize, ButtonVariant};

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

const DISABLED: &str = r#"<Button attr:disabled=true>"Default"</Button>
<Button variant=ButtonVariant::Secondary attr:disabled=true>
    "Secondary"
</Button>
<Button variant=ButtonVariant::Outline attr:disabled=true>
    "Outline"
</Button>
<Button variant=ButtonVariant::Destructive attr:disabled=true>
    "Destructive"
</Button>"#;

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
            ty: "Option<Callback<(Signal<String>, AnyNodeRef), AnyView>>",
            default: "None",
            description: "Render as something else — a link, usually. Handed the merged class and the node ref, both of which must reach the element.",
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
                        <Button size=ButtonSize::IconXs>"×"</Button>
                        <Button size=ButtonSize::IconSm>"×"</Button>
                        <Button size=ButtonSize::Icon>"×"</Button>
                        <Button size=ButtonSize::IconLg>"×"</Button>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Disabled"
                    description="A disabled button drops to half opacity and stops taking pointer events."
                    code=DISABLED
                >
                    <div class="flex flex-wrap gap-3">
                        <Button attr:disabled=true>"Default"</Button>
                        <Button variant=ButtonVariant::Secondary attr:disabled=true>
                            "Secondary"
                        </Button>
                        <Button variant=ButtonVariant::Outline attr:disabled=true>
                            "Outline"
                        </Button>
                        <Button variant=ButtonVariant::Destructive attr:disabled=true>
                            "Destructive"
                        </Button>
                    </div>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
