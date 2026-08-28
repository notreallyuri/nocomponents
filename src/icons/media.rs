use crate::{cn, icons::IconRoot};
use leptos::prelude::*;

#[component]
pub fn Play(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <IconRoot class=move || cn!("size-4", class.get())>
            <polygon points="6 3 20 12 6 21 6 3" />
        </IconRoot>
    }
}

#[component]
pub fn Pause(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <IconRoot class=move || cn!("size-4", class.get())>
            <rect x="14" y="4" width="4" height="16" rx="1" />
            <rect x="6" y="4" width="4" height="16" rx="1" />
        </IconRoot>
    }
}

#[component]
pub fn SkipBack(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <IconRoot class=move || cn!("size-4", class.get())>
            <polygon points="19 20 9 12 19 4 19 20" />
            <line x1="5" x2="5" y1="19" y2="5" />
        </IconRoot>
    }
}

#[component]
pub fn SkipForward(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <IconRoot class=move || cn!("size-4", class.get())>
            <polygon points="5 4 15 12 5 20 5 4" />
            <line x1="19" x2="19" y1="5" y2="19" />
        </IconRoot>
    }
}

#[component]
pub fn Volume(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <IconRoot class=move || cn!("size-4", class.get())>
            <path d="M11 5 6 9H2v6h4l5 4z" />
            <path d="M15.54 8.46a5 5 0 0 1 0 7.07" />
            <path d="M19.07 4.93a10 10 0 0 1 0 14.14" />
        </IconRoot>
    }
}

#[component]
pub fn VolumeOff(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <IconRoot class=move || cn!("size-4", class.get())>
            <path d="M11 5 6 9H2v6h4l5 4z" />
            <line x1="22" x2="16" y1="9" y2="15" />
            <line x1="16" x2="22" y1="9" y2="15" />
        </IconRoot>
    }
}

#[component]
pub fn Maximize(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <IconRoot class=move || cn!("size-4", class.get())>
            <path d="M8 3H5a2 2 0 0 0-2 2v3" />
            <path d="M21 8V5a2 2 0 0 0-2-2h-3" />
            <path d="M3 16v3a2 2 0 0 0 2 2h3" />
            <path d="M16 21h3a2 2 0 0 0 2-2v-3" />
        </IconRoot>
    }
}

#[component]
pub fn Minimize(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <IconRoot class=move || cn!("size-4", class.get())>
            <path d="M8 3v3a2 2 0 0 1-2 2H3" />
            <path d="M21 8h-3a2 2 0 0 1-2-2V3" />
            <path d="M3 16h3a2 2 0 0 1 2 2v3" />
            <path d="M16 21v-3a2 2 0 0 1 2-2h3" />
        </IconRoot>
    }
}

#[component]
pub fn Settings(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <IconRoot class=move || cn!("size-4", class.get())>
            <path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" />
            <circle cx="12" cy="12" r="3" />
        </IconRoot>
    }
}

#[component]
pub fn Captions(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <IconRoot class=move || cn!("size-4", class.get())>
            <rect width="18" height="14" x="3" y="5" rx="2" ry="2" />
            <path d="M7 15h4M15 15h2M7 11h2M13 11h4" />
        </IconRoot>
    }
}
