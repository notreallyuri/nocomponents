//! The "On this page" rail.
//!
//! Reads its entries out of the rendered page rather than being handed a list. A page already
//! declares its sections — every `DemoSection` is one, every `ApiEntry` is one — and a second list
//! written beside them would be the one that goes stale. The headings mark themselves with
//! `data-toc` so a heading that happens to be *inside* a demo is not mistaken for a section of the
//! page; several demos render one.

use leptos::{ev, html::Main, prelude::*, wasm_bindgen::JsCast, web_sys::Element};

/// The id a heading is given, derived from its own text so the rail and the heading agree on it
/// without either being told what the other used.
pub fn slug(text: &str) -> String {
    let mut out = String::with_capacity(text.len());

    for c in text.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
        } else if !out.is_empty() && !out.ends_with('-') {
            out.push('-');
        }
    }

    out.trim_end_matches('-').to_string()
}

#[derive(Clone)]
struct Heading {
    id: String,
    label: String,
    /// An `ApiReference` entry rather than a section of its own, and indented to say so.
    nested: bool,
}

/// Below this the rail is noise: a reader can see the whole page without it.
const MIN_HEADINGS: usize = 3;

/// Clears the sticky header, plus enough air that the heading does not sit against it.
const SCROLL_OFFSET: f64 = 80.0;

fn scroll_to(id: &str) {
    let Some(el) = document().get_element_by_id(id) else {
        return;
    };

    let top =
        el.get_bounding_client_rect().top() + window().scroll_y().unwrap_or(0.0) - SCROLL_OFFSET;
    window().scroll_to_with_x_and_y(0.0, top.max(0.0));
}

#[component]
pub fn PageNav(
    /// The `<main>` whose headings to read. Scoped rather than looked up in the document: leaving
    /// a page mounts the next one before unmounting this one, so for a moment there are two
    /// `<main>`s and a document-wide query answers with the one being left.
    content: NodeRef<Main>,
) -> impl IntoView {
    let headings = RwSignal::new(Vec::<Heading>::new());
    let active = RwSignal::new(String::new());

    // `content` is a signal, so this re-runs when the element it names arrives — which is the
    // earliest there is anything to read.
    Effect::new(move |_| {
        let Some(main) = content.get() else {
            return;
        };

        let Ok(found) = main.query_selector_all("[data-toc]") else {
            return;
        };

        let mut list = Vec::new();
        for i in 0..found.length() {
            let Some(el) = found
                .get(i)
                .and_then(|node| node.dyn_into::<Element>().ok())
            else {
                continue;
            };

            let id = el.id();
            let label = el.text_content().unwrap_or_default().trim().to_string();
            if id.is_empty() || label.is_empty() {
                continue;
            }

            list.push(Heading {
                id,
                label,
                nested: el.get_attribute("data-toc").as_deref() == Some("item"),
            });
        }

        if list.len() < MIN_HEADINGS {
            return;
        }

        active.set(list[0].id.clone());
        headings.set(list);
    });

    let listener = window_event_listener(ev::scroll, move |_| {
        let list = headings.get_untracked();
        let Some(first) = list.first() else {
            return;
        };

        // The last heading that has crossed the line, rather than the nearest one: a section is
        // current from where it starts until the next one starts, however long it runs.
        let mut current = first.id.clone();
        for heading in &list {
            let Some(el) = document().get_element_by_id(&heading.id) else {
                continue;
            };

            if el.get_bounding_client_rect().top() <= SCROLL_OFFSET + 8.0 {
                current = heading.id.clone();
            }
        }

        // A short last section never reaches the line, so the rail would go dead exactly where the
        // reader has finished scrolling.
        let scrolled = window().scroll_y().unwrap_or(0.0);
        let viewport = window()
            .inner_height()
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        let page = document()
            .document_element()
            .map(|el| el.scroll_height() as f64)
            .unwrap_or(0.0);

        if page > 0.0
            && scrolled + viewport >= page - 2.0
            && let Some(last) = list.last()
        {
            current = last.id.clone();
        }

        if active.get_untracked() != current {
            active.set(current);
        }
    });

    on_cleanup(move || listener.remove());

    view! {
        // `xl:` only: this is the third column, and a narrow window has room for two.
        <aside class="sticky top-14 hidden h-fit w-56 shrink-0 self-start py-8 xl:block">
            <Show when=move || !headings.get().is_empty()>
                <nav aria-label="On this page" class="flex flex-col gap-3">
                    <p class="text-xs font-medium tracking-tight text-foreground">"On this page"</p>

                    <ul class="flex flex-col gap-0.5 border-l border-border">
                        {move || {
                            headings
                                .get()
                                .into_iter()
                                .map(|heading| {
                                    let id = heading.id.clone();
                                    let target = heading.id.clone();
                                    let label = heading.label;
                                    let indent = if heading.nested { "pl-6" } else { "pl-3" };
                                    let is_active = Signal::derive(move || active.get() == id);

                                    view! {
                                        <li>
                                            <a
                                                href=format!("#{target}")
                                                // The offset is ours to apply, or the sticky
                                                // header lands on top of what was jumped to.
                                                on:click=move |e| {
                                                    e.prevent_default();
                                                    scroll_to(&target);
                                                }
                                                data-active=move || is_active.get().then_some("true")
                                                class=format!(
                                                    "-ml-px block border-l border-transparent py-1 {indent} text-xs leading-snug text-muted-foreground transition-colors hover:text-foreground data-[active=true]:border-foreground data-[active=true]:font-medium data-[active=true]:text-foreground",
                                                )
                                            >
                                                {label}
                                            </a>
                                        </li>
                                    }
                                })
                                .collect_view()
                        }}
                    </ul>
                </nav>
            </Show>
        </aside>
    }
}
