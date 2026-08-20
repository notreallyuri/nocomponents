use crate::layout::doc_layout::DocLayout;
use leptos::prelude::*;
use leptos_router::components::A;
use nocomponents::components::button::{Button, ButtonSize, ButtonVariant};

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
    ("Card", "/docs/card"),
    ("Checkbox", "/docs/checkbox"),
    ("Code", "/docs/code"),
    ("Collapsible", "/docs/collapsible"),
    ("Dialog", "/docs/dialog"),
    ("Dropdown Menu", "/docs/dropdown-menu"),
    ("Empty", "/docs/empty"),
    ("Field", "/docs/field"),
    ("Hover Card", "/docs/hover-card"),
    ("Input", "/docs/input"),
    ("Item", "/docs/item"),
    ("Kbd", "/docs/kbd"),
    ("Label", "/docs/label"),
    ("Native Select", "/docs/native-select"),
    ("Pagination", "/docs/pagination"),
    ("Popover", "/docs/popover"),
    ("Progress", "/docs/progress"),
    ("Radio Group", "/docs/radio-group"),
    ("Select", "/docs/select"),
    ("Separator", "/docs/separator"),
    ("Sheet", "/docs/sheet"),
    ("Skeleton", "/docs/skeleton"),
    ("Spinner", "/docs/spinner"),
    ("Switch", "/docs/switch"),
    ("Table", "/docs/table"),
    ("Tabs", "/docs/tabs"),
    ("Textarea", "/docs/textarea"),
    ("Toast", "/docs/toast"),
    ("Toggle", "/docs/toggle"),
    ("Tooltip", "/docs/tooltip"),
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
                                render=Callback::new(move |(class, _node_ref)| {
                                    view! {
                                        <A href=href attr:class=class>
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
