//! A drop target for files, and the file picker behind it.
//!
//! Two ways in, one way out: dropping files on the zone and picking them through the hidden
//! `<input type="file">` both arrive as the same `Vec<File>` on `on_files`. A caller who wanted to
//! tell them apart would be a caller writing two code paths for one thing.
//!
//! **`accept` is checked here, on both ways in.** The attribute on a file input only decides what
//! the dialog *offers* — every dialog has an "All files" escape — and a drop never consults it at
//! all. Neither way in can be trusted to have filtered itself.
//!
//! **`dragover` must be prevented or nothing can be dropped.** The browser's default for a drag
//! over an element is to refuse it, and the refusal is silent — the cursor says no and `drop`
//! never fires. Preventing it on every `dragover` is what makes the element a target at all.
//!
//! `dragleave` fires when the pointer crosses onto a *child*, which is why leaving is counted
//! rather than believed: a zone with an icon and two lines of text in it would otherwise flicker
//! out of its hover state the moment the pointer touched any of them.

use crate::utils::next_id;
use leptos::{context::Provider, ev, prelude::*, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;
use web_sys::{File, HtmlInputElement};

#[derive(Copy, Clone)]
pub struct DropzoneContext {
    /// Whether a drag is currently over the zone.
    pub is_over: RwSignal<bool>,
    pub disabled: Signal<bool>,
    /// The hidden input, so a trigger anywhere inside can open the picker.
    pub input_ref: AnyNodeRef,
}

impl DropzoneContext {
    /// Opens the file picker, which only a real click gesture is allowed to do.
    pub fn open(&self) {
        if self.disabled.get_untracked() {
            return;
        }
        if let Some(input) = self
            .input_ref
            .get_untracked()
            .and_then(|node| node.dyn_into::<HtmlInputElement>().ok())
        {
            input.click();
        }
    }
}

pub fn use_dropzone() -> DropzoneContext {
    expect_context::<DropzoneContext>()
}

/// Whether a file is one of the kinds `accept` asks for.
///
/// The same grammar the `accept` attribute uses, because a caller should not have to learn a
/// second one: a comma-separated list of `.ext`, `type/subtype`, or `type/*`. An empty list
/// accepts everything, which is what the attribute means by being absent.
///
/// Applied to both ways in. The attribute decides what a file dialog *offers*, not what it hands
/// back — the reader can switch it to "All files" — and a drop does not consult it at all.
/// Checking here is what makes `accept` a rule rather than a suggestion.
pub fn accepts(accept: &str, mime: &str, name: &str) -> bool {
    let accept = accept.trim();
    if accept.is_empty() {
        return true;
    }

    let mime = mime.to_ascii_lowercase();
    let name = name.to_ascii_lowercase();

    accept.split(',').map(str::trim).any(|pattern| {
        let pattern = pattern.to_ascii_lowercase();
        if pattern.is_empty() {
            return false;
        }
        if let Some(extension) = pattern.strip_prefix('.') {
            return name.ends_with(&format!(".{extension}"));
        }
        if let Some(family) = pattern.strip_suffix("/*") {
            return mime.split('/').next() == Some(family);
        }
        mime == pattern
    })
}

fn files_from(list: Option<web_sys::FileList>, accept: &str) -> Vec<File> {
    let Some(list) = list else {
        return Vec::new();
    };

    (0..list.length())
        .filter_map(|i| list.get(i))
        .filter(|file| accepts(accept, &file.type_(), &file.name()))
        .collect()
}

#[component]
pub fn DropzoneRoot(
    #[prop(optional, into)] class: Signal<String>,
    /// The `accept` grammar: `"image/*"`, `".pdf,.docx"`. Empty takes anything.
    #[prop(optional, into)]
    accept: Signal<String>,
    #[prop(default = true)] multiple: bool,
    #[prop(optional, into)] disabled: Signal<bool>,
    /// Every arrival, however it arrived. Never called with an empty list.
    #[prop(into)]
    on_files: Callback<Vec<File>>,
    /// Files that were dropped and turned away by `accept`.
    #[prop(default = None, into)]
    on_rejected: Option<Callback<Vec<File>>>,
    children: Children,
) -> impl IntoView {
    let input_ref = AnyNodeRef::new();
    let context = DropzoneContext {
        is_over: RwSignal::new(false),
        disabled,
        input_ref,
    };

    // `dragleave` fires on the way onto a child as well as on the way out, so leaving is the
    // count reaching zero rather than one event.
    let depth = RwSignal::new(0i32);
    let input_id = StoredValue::new(next_id("dropzone"));

    let deliver = move |files: Vec<File>, rejected: Vec<File>| {
        if !rejected.is_empty()
            && let Some(on_rejected) = on_rejected
        {
            on_rejected.run(rejected);
        }
        if !files.is_empty() {
            on_files.run(files);
        }
    };

    view! {
        <div
            data-slot="dropzone"
            role="button"
            tabindex=move || if disabled.get() { "-1" } else { "0" }
            aria-disabled=move || disabled.get().then_some("true")
            data-over=move || context.is_over.get().then_some("true")
            data-disabled=move || disabled.get().then_some("true")
            on:dragenter=move |e: ev::DragEvent| {
                e.prevent_default();
                if disabled.get_untracked() {
                    return;
                }
                depth.update(|d| *d += 1);
                context.is_over.set(true);
            }
            on:dragover=move |e: ev::DragEvent| {
                // Without this the drop is refused, silently.
                e.prevent_default();
            }
            on:dragleave=move |e: ev::DragEvent| {
                e.prevent_default();
                depth.update(|d| *d -= 1);
                if depth.get_untracked() <= 0 {
                    depth.set(0);
                    context.is_over.set(false);
                }
            }
            on:drop=move |e: ev::DragEvent| {
                e.prevent_default();
                depth.set(0);
                context.is_over.set(false);
                if disabled.get_untracked() {
                    return;
                }
                // One read of the transfer, split in two: what `accept` takes and what it
                // turned away.
                let accept = accept.get_untracked();
                let (taken, rejected): (Vec<_>, Vec<_>) = files_from(
                        e.data_transfer().and_then(|dt| dt.files()),
                        "",
                    )
                    .into_iter()
                    .partition(|file| accepts(&accept, &file.type_(), &file.name()));
                deliver(taken, rejected);
            }
            on:click=move |_| context.open()
            on:keydown=move |e: ev::KeyboardEvent| {
                if matches!(e.key().as_str(), "Enter" | " ") {
                    e.prevent_default();
                    context.open();
                }
            }
            class=class
        >
            <input
                node_ref=input_ref
                id=move || input_id.get_value()
                type="file"
                class="sr-only"
                tabindex="-1"
                accept=move || accept.get()
                multiple=multiple
                disabled=move || disabled.get()
                on:change=move |e: ev::Event| {
                    let input = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
                    let Some(input) = input else {
                        return;
                    };
                    let accept = accept.get_untracked();
                    // `accept` on the input is a filter the dialog *offers*, not one it
                    // enforces: every file dialog has an "All files" escape, and whatever the
                    // reader picks arrives regardless. So a pick is checked exactly like a drop.
                    let (taken, rejected): (Vec<_>, Vec<_>) = files_from(input.files(), "")
                        .into_iter()
                        .partition(|file| accepts(&accept, &file.type_(), &file.name()));
                    // Or picking the same file twice running is one event, not two.
                    input.set_value("");
                    deliver(taken, rejected);
                }
            />
            <Provider value=context>{children()}</Provider>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::accepts;

    #[test]
    fn an_empty_accept_takes_anything() {
        assert!(accepts("", "image/png", "a.png"));
        assert!(accepts("  ", "application/x-thing", "a.bin"));
    }

    #[test]
    fn a_family_matches_its_whole_type() {
        assert!(accepts("image/*", "image/png", "a.png"));
        assert!(accepts("image/*", "image/svg+xml", "a.svg"));
        assert!(!accepts("image/*", "video/mp4", "a.mp4"));
    }

    #[test]
    fn an_exact_type_matches_only_itself() {
        assert!(accepts("application/pdf", "application/pdf", "a.pdf"));
        assert!(!accepts("application/pdf", "application/json", "a.json"));
    }

    #[test]
    fn an_extension_matches_the_name_a_drop_arrives_with() {
        // A dropped file often has no type at all, which is why the name is checked too.
        assert!(accepts(".pdf", "", "report.pdf"));
        assert!(accepts(".PDF", "", "REPORT.pdf"));
        assert!(!accepts(".pdf", "", "report.pdf.exe"));
        assert!(!accepts(".pdf", "", "pdf"));
    }

    #[test]
    fn a_list_takes_any_of_its_entries() {
        assert!(accepts("image/*, .pdf", "image/gif", "a.gif"));
        assert!(accepts("image/*, .pdf", "", "a.pdf"));
        assert!(!accepts("image/*, .pdf", "text/csv", "a.csv"));
    }
}
