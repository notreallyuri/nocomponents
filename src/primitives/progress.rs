use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct ProgressContext {
    pub value: Signal<Option<f64>>,
    pub max: Signal<f64>,
}

pub fn use_progress() -> ProgressContext {
    expect_context::<ProgressContext>()
}

#[component]
pub fn ProgressRoot(
    #[prop(optional, into)] value: Signal<Option<f64>>,
    #[prop(default = 100.0.into(), into)] max: Signal<f64>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    provide_context(ProgressContext { value, max });

    let data_state = move || match value.get() {
        None => "indeterminate",
        Some(v) if v >= max.get() => "complete",
        _ => "loading",
    };

    view! {
        <div
            data-slot="progress"
            role="progressbar"
            aria-valuemin="0"
            aria-valuemax=move || max.get().to_string()
            aria-valuenow=move || value.get().map(|v| v.to_string())
            data-state=data_state
            data-max=move || max.get().to_string()
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ProgressIndicatorRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] style: Signal<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let ctx = use_progress();

    let data_state = move || match ctx.value.get() {
        None => "indeterminate",
        Some(v) if v >= ctx.max.get() => "complete",
        _ => "loading",
    };

    view! {
        <div data-slot="progress-indicator" data-state=data_state class=class style=style>
            {children.map(|c| c())}
        </div>
    }
}
