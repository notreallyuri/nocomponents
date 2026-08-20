use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
use leptos::prelude::*;
use nocomponents::components::{
    label::Label,
    native_select::{NativeSelect, NativeSelectSize},
};

const DEFAULT: &str = r#"<NativeSelect class="max-w-56">
    <option value="apple">"Apple"</option>
    <option value="banana">"Banana"</option>
    <option value="cherry">"Cherry"</option>
</NativeSelect>"#;

const WITH_LABEL: &str = r#"<Label attr:r#for="timezone">"Timezone"</Label>
<NativeSelect attr:id="timezone" class="max-w-56">
    <option value="utc">"UTC"</option>
    <option value="cet">"Central European"</option>
    <option value="jst">"Japan Standard"</option>
</NativeSelect>"#;

const GROUPED: &str = r#"<NativeSelect class="max-w-56">
    <optgroup label="Fruit">
        <option value="apple">"Apple"</option>
        <option value="pear">"Pear"</option>
    </optgroup>
    <optgroup label="Vegetables">
        <option value="leek">"Leek"</option>
        <option value="fennel">"Fennel"</option>
    </optgroup>
</NativeSelect>"#;

const SIZES: &str = r#"<NativeSelect size=NativeSelectSize::Sm class="max-w-56">
    <option>"Small"</option>
</NativeSelect>
<NativeSelect class="max-w-56">
    <option>"Default"</option>
</NativeSelect>"#;

const DISABLED: &str = r#"<NativeSelect attr:disabled=true class="max-w-56">
    <option>"Locked"</option>
</NativeSelect>"#;

const CONTROLLED: &str = r#"<NativeSelect model=role class="max-w-56">
    <option value="admin">"Admin"</option>
    <option value="editor">"Editor"</option>
    <option value="viewer">"Viewer"</option>
</NativeSelect>
<p class="text-sm text-muted-foreground">"Selected: " {move || role.get()}</p>"#;

#[component]
pub fn Page() -> impl IntoView {
    let role = RwSignal::new("editor".to_string());

    view! {
        <DocLayout
            title="Native Select"
            description="The browser's own select element, styled to match the rest."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="A real <select>: it costs nothing, works everywhere, and opens the platform picker on a phone. Reach for Select instead when the options need to look like anything other than a native menu."
                    code=DEFAULT
                >
                    <NativeSelect class="max-w-56">
                        <option value="apple">"Apple"</option>
                        <option value="banana">"Banana"</option>
                        <option value="cherry">"Cherry"</option>
                    </NativeSelect>
                </DemoSection>

                <DemoSection
                    title="With a label"
                    description="Point for at the control's id, as with any other field."
                    code=WITH_LABEL
                >
                    <div class="flex max-w-56 flex-col gap-1.5">
                        <Label attr:r#for="timezone">"Timezone"</Label>
                        <NativeSelect attr:id="timezone">
                            <option value="utc">"UTC"</option>
                            <option value="cet">"Central European"</option>
                            <option value="jst">"Japan Standard"</option>
                        </NativeSelect>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Grouped options"
                    description="optgroup works, which is most of the reason to use the native control."
                    code=GROUPED
                >
                    <NativeSelect class="max-w-56">
                        <optgroup label="Fruit">
                            <option value="apple">"Apple"</option>
                            <option value="pear">"Pear"</option>
                        </optgroup>
                        <optgroup label="Vegetables">
                            <option value="leek">"Leek"</option>
                            <option value="fennel">"Fennel"</option>
                        </optgroup>
                    </NativeSelect>
                </DemoSection>

                <DemoSection
                    title="Sizes"
                    description="Two heights, matching the small and default inputs."
                    code=SIZES
                >
                    <div class="flex flex-col gap-3">
                        <NativeSelect size=NativeSelectSize::Sm class="max-w-56">
                            <option>"Small"</option>
                        </NativeSelect>
                        <NativeSelect class="max-w-56">
                            <option>"Default"</option>
                        </NativeSelect>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Disabled"
                    description="A disabled select dims and fills its background, like a disabled input."
                    code=DISABLED
                >
                    <NativeSelect attr:disabled=true class="max-w-56">
                        <option>"Locked"</option>
                    </NativeSelect>
                </DemoSection>

                <DemoSection
                    title="Controlled"
                    description="model binds the value both ways; pass value on its own for a one-way binding."
                    code=CONTROLLED
                >
                    <div class="flex max-w-56 flex-col gap-3">
                        <NativeSelect model=role>
                            <option value="admin">"Admin"</option>
                            <option value="editor">"Editor"</option>
                            <option value="viewer">"Viewer"</option>
                        </NativeSelect>
                        <p class="text-sm text-muted-foreground">
                            "Selected: " {move || role.get()}
                        </p>
                    </div>
                </DemoSection>
            </div>
        </DocLayout>
    }
}
