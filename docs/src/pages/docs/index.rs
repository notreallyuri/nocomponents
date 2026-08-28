use crate::layout::doc_layout::DocLayout;
use leptos::prelude::*;
use leptos_router::components::A;
use nocomponents::components::button::{Button, ButtonSize, ButtonVariant};
use nocomponents::utils::types::StyledRender;

const COMPONENTS: &[(&str, &str)] = &[
    ("Alert", "/docs/alert"),
    ("Accordion", "/docs/accordion"),
    ("Alert Dialog", "/docs/alert-dialog"),
    ("Aspect Ratio", "/docs/aspect-ratio"),
    ("Avatar", "/docs/avatar"),
    ("Badge", "/docs/badge"),
    ("Breadcrumb", "/docs/breadcrumb"),
    ("Button", "/docs/button"),
    ("Button Group", "/docs/button-group"),
    ("Canvas", "/docs/canvas"),
    ("Card", "/docs/card"),
    ("Checkbox", "/docs/checkbox"),
    ("Code", "/docs/code"),
    ("Collapsible", "/docs/collapsible"),
    ("Combobox", "/docs/combobox"),
    ("Command", "/docs/command"),
    ("Context Menu", "/docs/context-menu"),
    ("Dialog", "/docs/dialog"),
    ("Drawer", "/docs/drawer"),
    ("Dropzone", "/docs/dropzone"),
    ("Dropdown Menu", "/docs/dropdown-menu"),
    ("Empty", "/docs/empty"),
    ("Field", "/docs/field"),
    ("Floating Menu", "/docs/floating-menu"),
    ("Hover Card", "/docs/hover-card"),
    ("Input", "/docs/input"),
    ("Input Group", "/docs/input-group"),
    ("Input OTP", "/docs/input-otp"),
    ("Item", "/docs/item"),
    ("Kanban", "/docs/kanban"),
    ("Kbd", "/docs/kbd"),
    ("Label", "/docs/label"),
    ("Menubar", "/docs/menubar"),
    ("Native Select", "/docs/native-select"),
    ("Navigation Menu", "/docs/navigation-menu"),
    ("Resizable", "/docs/resizable"),
    ("Carousel", "/docs/carousel"),
    ("Calendar", "/docs/calendar"),
    ("Date Picker", "/docs/date-picker"),
    ("Data Table", "/docs/data-table"),
    ("Chart", "/docs/chart"),
    ("Color Picker", "/docs/color-picker"),
    ("Image Cropper", "/docs/image-cropper"),
    ("Pagination", "/docs/pagination"),
    ("Popover", "/docs/popover"),
    ("Progress", "/docs/progress"),
    ("Radio Group", "/docs/radio-group"),
    ("Scroll Area", "/docs/scroll-area"),
    ("Select", "/docs/select"),
    ("Separator", "/docs/separator"),
    ("Sheet", "/docs/sheet"),
    ("Sidebar", "/docs/sidebar"),
    ("Skeleton", "/docs/skeleton"),
    ("Slider", "/docs/slider"),
    ("Spinner", "/docs/spinner"),
    ("Switch", "/docs/switch"),
    ("Table", "/docs/table"),
    ("Tabs", "/docs/tabs"),
    ("Textarea", "/docs/textarea"),
    ("Toast", "/docs/toast"),
    ("Toggle", "/docs/toggle"),
    ("Tooltip", "/docs/tooltip"),
    ("Timeline", "/docs/timeline"),
    ("Tree", "/docs/tree"),
    ("Video Player", "/docs/video-player"),
    ("Toggle Group", "/docs/toggle-group"),
];

#[component]
pub fn Page() -> impl IntoView {
    let mut sorted_components = COMPONENTS.to_vec();

    sorted_components.sort_by_key(|&(label, _)| label.to_lowercase());

    view! {
        <DocLayout
            title="Components"
            description="All the components available in the library are presented here."
        >
            <h1 class="text-2xl mb-6 font-semibold">Component List</h1>
            <div class="grid grid-cols-2 sm:grid-cols-3 gap-2">
                {sorted_components
                    .into_iter()
                    .map(|(label, href)| {
                        view! {
                            <Button
                                variant=ButtonVariant::Link
                                size=ButtonSize::Lg
                                class="text-left justify-start"
                                render=Callback::new(move |styled: StyledRender| {
                                    view! {
                                        <A href=crate::app::href(href) attr:class=styled.class>
                                            {label}
                                        </A>
                                    }
                                        .into_any()
                                })
                            />
                        }
                    })
                    .collect_view()}
            </div>
        </DocLayout>
    }
}
