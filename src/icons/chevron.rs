use crate::{cn, icons::icon::Icon};
use leptos::prelude::*;

#[component]
pub fn ChevronUp(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon view_box="0 0 24 24" stroke_width=2.0 class=move || cn!("size-4", class.get())>
            <path d="m18 15-6-6-6 6" />
        </Icon>
    }
}

#[component]
pub fn ChevronDown(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon view_box="0 0 24 24" stroke_width=2.0 class=move || cn!("size-4", class.get())>
            <path d="m6 9 6 6 6-6" />
        </Icon>
    }
}
#[component]
pub fn ChevronRight(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon view_box="0 0 24 24" stroke_width=2.0 class=move || cn!("size-4", class.get())>
            <path d="m9 18 6-6-6-6" />
        </Icon>
    }
}

#[component]
pub fn ChevronLeft(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon view_box="0 0 24 24" stroke_width=2.0 class=move || cn!("size-4", class.get())>
            <path d="m15 18-6-6 6-6" />
        </Icon>
    }
}
