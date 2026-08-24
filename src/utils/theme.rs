//! Four independent axes, one provider.
//!
//! Light and dark is a *mode*: whether the same palette is read light-side-up or dark-side-up, and
//! the one axis the operating system has an opinion about. A colour theme is a different palette
//! entirely — a different `--primary`, a different `--radius` — and it has a light and a dark of
//! its own. Folding them into one three-valued `Theme` meant a consumer with two brands could not
//! say "brand two, dark" at all.
//!
//! An accent is narrower than either: one hue picked *inside* whatever palette is in force —
//! the thing a settings screen offers as fourteen dots, where every one of them leaves the rest
//! of the palette alone. It is a third name rather than fourteen more palettes because that is
//! what it costs in CSS: a palette's stylesheet declares a ramp, and the accent says which rung
//! of it `--primary` reads.
//!
//! The corner radius is the odd one out: it is not a name a stylesheet defines but a length the
//! reader picks, so there is nothing to enumerate and no `data-` value that would mean anything.
//! It is written as `--radius` in `<html>`'s own `style`, which beats every rule that set it and
//! is the one place a value with no name belongs. A palette still declares its own; a custom one
//! overrides it, and clearing it hands the palette its radius back.
//!
//! So the mode stays a [`Theme`], the palette and the accent become names, and the radius is a
//! length. The mode is a `dark` class on
//! `<html>`, which is what Tailwind's `dark:` variant reads; the palette is `data-theme` on the
//! same element, which CSS matches with `[data-theme="brand"]` and a `@custom-variant` if it wants
//! a utility for it; the accent is `data-accent` beside it. Nothing here knows what themes or
//! accents exist: a name is whatever the stylesheet defines, and the empty name is the absence of
//! the attribute — the base palette in `:root`, and no accent at all.

use leptos::{
    prelude::*,
    wasm_bindgen::{JsCast, prelude::Closure},
};
use std::str::FromStr;

/// Which way round the palette is read. Not the palette itself — that is [`ThemeContext::color`].
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum Theme {
    Light,
    Dark,
    #[default]
    System,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
            Theme::System => "system",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Theme::Light => "Light",
            Theme::Dark => "Dark",
            Theme::System => "System",
        }
    }

    /// Every mode, in the order a switcher offers them.
    pub const ALL: [Theme; 3] = [Theme::Light, Theme::Dark, Theme::System];
}

impl FromStr for Theme {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "light" => Theme::Light,
            "dark" => Theme::Dark,
            _ => Theme::System,
        })
    }
}

/// Where the mode is kept between visits.
const THEME_KEY: &str = "ui-theme";
/// And the palette, under a key of its own so that changing one does not rewrite the other.
const COLOR_KEY: &str = "ui-color-theme";
/// And the accent, for the same reason.
const ACCENT_KEY: &str = "ui-accent";
/// And the corner radius, which is a length rather than a name.
const RADIUS_KEY: &str = "ui-radius";

#[derive(Copy, Clone)]
pub struct ThemeContext {
    /// Light, dark, or whatever the system says.
    pub current: RwSignal<Theme>,
    /// The palette's name, written to `data-theme`. Empty is the base palette, and is the one
    /// name that removes the attribute rather than setting it.
    pub color: RwSignal<String>,
    /// The accent's name, written to `data-accent`. Empty is the palette's own `--primary`,
    /// whatever that is, and removes the attribute.
    pub accent: RwSignal<String>,
    /// A custom corner radius as a CSS length — `"0.5rem"`, `"4px"`. Written to `--radius` in
    /// `<html>`'s inline style, which outranks whatever the palette said. Empty gives the
    /// palette its own back.
    pub radius: RwSignal<String>,
    /// What `current` comes out as once `System` has been resolved — never `Theme::System`, so a
    /// switcher can show which of the two the system actually chose.
    pub resolved: Signal<Theme>,
}

impl ThemeContext {
    pub fn set_theme(&self, theme: Theme) {
        self.current.set(theme);
    }

    /// Switches palette. A name that is not a plain identifier is ignored rather than written
    /// into an attribute selector-shaped hole.
    pub fn set_color(&self, color: impl Into<String>) {
        let color = color.into();
        if is_theme_name(&color) {
            self.color.set(color);
        }
    }

    /// Switches accent, validated the same way a palette name is.
    pub fn set_accent(&self, accent: impl Into<String>) {
        let accent = accent.into();
        if is_theme_name(&accent) {
            self.accent.set(accent);
        }
    }

    /// Sets a custom corner radius. A value that is not a plain CSS length is ignored rather
    /// than written into a property that would silently drop it anyway.
    pub fn set_radius(&self, radius: impl Into<String>) {
        let radius = radius.into();
        if is_length(&radius) {
            self.radius.set(radius);
        }
    }

    pub fn is_dark(&self) -> bool {
        self.resolved.get() == Theme::Dark
    }
}

pub fn use_theme() -> ThemeContext {
    expect_context::<ThemeContext>()
}

/// Lowercase letters, digits and hyphens: what a class or an attribute selector can name without
/// quoting, and what keeps a value read back out of `localStorage` from being anything else.
fn is_theme_name(name: &str) -> bool {
    name.is_empty()
        || name
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

/// A number and a unit, and nothing else. Not a full length grammar — `calc()` and the rest are
/// deliberately out, because what writes this is a slider and what reads it is one declaration.
fn is_length(value: &str) -> bool {
    if value.is_empty() {
        return true;
    }

    let Some(number) = ["rem", "px", "em"]
        .iter()
        .find_map(|unit| value.strip_suffix(unit))
    else {
        return false;
    };

    !number.is_empty()
        && number.bytes().all(|b| b.is_ascii_digit() || b == b'.')
        && number.bytes().filter(|b| *b == b'.').count() <= 1
        && number != "."
}

fn stored(key: &str) -> Option<String> {
    window()
        .local_storage()
        .ok()
        .flatten()
        .and_then(|storage| storage.get_item(key).ok().flatten())
}

fn store(key: &str, value: &str) {
    if let Some(storage) = window().local_storage().ok().flatten() {
        let _ = storage.set_item(key, value);
    }
}

fn prefers_dark() -> bool {
    window()
        .match_media("(prefers-color-scheme: dark)")
        .ok()
        .flatten()
        .is_some_and(|query| query.matches())
}

#[component]
pub fn ThemeProvider(
    /// The palette to open with, for a first visit with nothing stored. Empty is the base one.
    #[prop(optional, into)]
    default_color: String,
    /// And the accent. Empty leaves the palette's own `--primary` alone.
    #[prop(optional, into)]
    default_accent: String,
    /// And a corner radius, as a CSS length. Empty leaves the palette's own alone.
    #[prop(optional, into)]
    default_radius: String,
    children: Children,
) -> impl IntoView {
    let current = RwSignal::new(
        stored(THEME_KEY)
            .and_then(|theme| theme.parse::<Theme>().ok())
            .unwrap_or_default(),
    );

    // A name out of storage is checked the same way one out of `set_color` is: it was a plain
    // identifier when it was written, but nothing guarantees it still is when it is read.
    let color = RwSignal::new(
        stored(COLOR_KEY)
            .filter(|color| is_theme_name(color))
            .unwrap_or(default_color)
            .to_string(),
    );

    let accent = RwSignal::new(
        stored(ACCENT_KEY)
            .filter(|accent| is_theme_name(accent))
            .unwrap_or(default_accent)
            .to_string(),
    );

    let radius = RwSignal::new(
        stored(RADIUS_KEY)
            .filter(|radius| is_length(radius))
            .unwrap_or(default_radius)
            .to_string(),
    );

    // Resolved here rather than at each place that wants to know: `System` is a question for the
    // browser, and asking it in a signal keeps the answer in one place.
    let system_dark = RwSignal::new(prefers_dark());

    // And kept up to date, because the system's answer changes under a running page — a laptop
    // that goes dark at sunset, or a switch flipped in another window. `utils::use_media_query`
    // re-reads on `resize`, which is exactly the event this never fires.
    if let Some(query) = window()
        .match_media("(prefers-color-scheme: dark)")
        .ok()
        .flatten()
    {
        let on_change = Closure::<dyn FnMut()>::new(move || system_dark.set(prefers_dark()));
        let _ =
            query.add_event_listener_with_callback("change", on_change.as_ref().unchecked_ref());
        // Handed to JS and not taken back. `on_cleanup` wants a `Send + Sync` closure, which a
        // `Closure` is not, and dropping this one while the listener still holds it is a call
        // into freed memory — so the listener lives as long as the page, which is how long a
        // provider wrapping the whole app lives anyway.
        on_change.forget();
    }
    let resolved = Signal::derive(move || match current.get() {
        Theme::System if system_dark.get() => Theme::Dark,
        Theme::System => Theme::Light,
        theme => theme,
    });

    provide_context(ThemeContext {
        current,
        color,
        accent,
        radius,
        resolved,
    });

    Effect::new(move |_| {
        let Some(html) = document().document_element() else {
            return;
        };

        let class_list = html.class_list();
        if resolved.get() == Theme::Dark {
            let _ = class_list.add_1("dark");
        } else {
            let _ = class_list.remove_1("dark");
        }

        color.with(|color| {
            // The base palette is the absence of the attribute, not `data-theme=""`: a stylesheet
            // written against `:root` should not have to also match an empty string.
            if color.is_empty() {
                let _ = html.remove_attribute("data-theme");
            } else {
                let _ = html.set_attribute("data-theme", color);
            }

            store(COLOR_KEY, color);
        });

        accent.with(|accent| {
            if accent.is_empty() {
                let _ = html.remove_attribute("data-accent");
            } else {
                let _ = html.set_attribute("data-accent", accent);
            }

            store(ACCENT_KEY, accent);
        });

        radius.with(|radius| {
            // On `<html>`'s own style rather than in a rule: there is no name to write, and an
            // inline custom property is the one thing every `[data-theme]` block loses to.
            if let Some(style) = html.dyn_ref::<web_sys::HtmlElement>().map(|el| el.style()) {
                if radius.is_empty() {
                    let _ = style.remove_property("--radius");
                } else {
                    let _ = style.set_property("--radius", radius);
                }
            }

            store(RADIUS_KEY, radius);
        });

        store(THEME_KEY, current.get().as_str());
    });

    view! { {children()} }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_names_are_plain_identifiers() {
        assert!(is_theme_name(""));
        assert!(is_theme_name("ctp-mocha"));
        assert!(is_theme_name("brand2"));
        assert!(!is_theme_name("Grove"));
        assert!(!is_theme_name("a\"] , [b"));
    }

    #[test]
    fn lengths_are_a_number_and_a_unit() {
        assert!(is_length(""));
        assert!(is_length("0rem"));
        assert!(is_length("0.75rem"));
        assert!(is_length("8px"));
        assert!(is_length("1.5em"));

        assert!(!is_length("1"), "a bare number has no unit");
        assert!(!is_length("rem"), "a bare unit has no number");
        assert!(!is_length("."), "a lone point is not a number");
        assert!(!is_length("1..5rem"), "two points is not a number");
        assert!(!is_length("calc(1rem + 2px)"), "deliberately not a grammar");
        assert!(!is_length("1rem; color: red"), "nor an escape hatch");
    }
}
