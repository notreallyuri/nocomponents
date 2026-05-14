use std::str::FromStr;

use leptos::prelude::*;

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

#[derive(Copy, Clone)]
pub struct ThemeContext {
    pub current: RwSignal<Theme>,
}

impl ThemeContext {
    pub fn set_theme(&self, theme: Theme) {
        self.current.set(theme);
    }
}

pub fn use_theme() -> ThemeContext {
    expect_context::<ThemeContext>()
}

#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    let initial_theme = window()
        .local_storage()
        .ok()
        .flatten()
        .and_then(|storage| storage.get_item("ui-theme").ok().flatten())
        .and_then(|s| s.parse::<Theme>().ok())
        .unwrap_or_default();

    let current = RwSignal::new(initial_theme);

    provide_context(ThemeContext { current });

    Effect::new(move |_| {
        let theme = current.get();
        let window = window();

        if let Some(document) = window.document() && let Some(html) = document.document_element() {
                let is_dark = match theme {
                    Theme::Dark => true,
                    Theme::Light => false,
                    Theme::System => window
                        .match_media("(prefers-color-scheme: dark)")
                        .ok()
                        .flatten()
                        .map(|m| m.matches())
                        .unwrap_or(false),
                };

                let class_list = html.class_list();
                if is_dark {
                    let _ = class_list.add_1("dark");
                } else {
                    let _ = class_list.remove_1("dark");
                }

                if let Some(storage) = window.local_storage().ok().flatten() {
                    let _ = storage.set_item("ui-theme", theme.as_str());
            }
        }
    });

    view! { {children()} }
}
