//! The docs site's theme control.
//!
//! `utils::theme` drives four independent things — a mode, a palette, an accent inside it, and a
//! corner radius — and the whole point of them being independent is that you can see them at once
//! and move one without disturbing the others. So the header's control is a popover with three
//! columns and a slider rather than a menu you have to reopen twice. [`ThemeSwitcher`] is the
//! compact one for pages that only want the first
use leptos::prelude::*;
use nocomponents::{
    cn,
    components::{
        button::ButtonVariant,
        dropdown_menu::{
            DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuLabel,
            DropdownMenuPortal, DropdownMenuSeparator, DropdownMenuTrigger,
        },
        popover::{Popover, PopoverContent, PopoverPortal, PopoverTrigger},
        separator::Separator,
        slider::Slider,
    },
    icons::check::Check,
    utils::{
        theme::{Theme, ThemeContext, use_theme},
        types::{Align, Orientation, Side},
    },
};

/// One row of the palette column.
#[derive(Clone, Copy)]
struct Palette {
    /// What `data-theme` is set to when the row is picked and nothing it stands for is already
    /// in force. The empty name is the base palette, which is the absence of the attribute.
    name: &'static str,
    label: &'static str,
    /// The other names this row also *is*. Latte is the light half of all three Catppuccin
    /// flavours, so in light mode one row stands for the three of them — picking it keeps
    /// whichever is already set, because there is nothing to see in the difference.
    aliases: &'static [&'static str],
}

impl Palette {
    fn holds(&self, color: &str) -> bool {
        color == self.name || self.aliases.contains(&color)
    }

    /// Picking a row that stands for several names keeps whichever of them is already in force:
    /// choosing "Latte" while Frappé is set would otherwise silently pick a different dark half
    /// of a palette that looks, right now, exactly the same.
    fn select(&self, theme_ctx: &ThemeContext) {
        if !theme_ctx.color.with_untracked(|color| self.holds(color)) {
            theme_ctx.set_color(self.name);
        }
    }
}

const SHARED: [Palette; 3] = [
    Palette {
        name: "",
        label: "Default",
        aliases: &[],
    },
    Palette {
        name: "grove",
        label: "Grove",
        aliases: &[],
    },
    Palette {
        name: "ember",
        label: "Ember",
        aliases: &[],
    },
];

/// Light: the three Catppuccin flavours are indistinguishable, all being Latte, so they are one
/// row that says what you are actually looking at.
const LIGHT_PALETTES: [Palette; 4] = [
    SHARED[0],
    SHARED[1],
    SHARED[2],
    Palette {
        name: "ctp-mocha",
        label: "Latte",
        aliases: &["ctp-frappe", "ctp-macchiato"],
    },
];

/// Dark: the three of them are three different palettes, so they are three rows.
const DARK_PALETTES: [Palette; 6] = [
    SHARED[0],
    SHARED[1],
    SHARED[2],
    Palette {
        name: "ctp-frappe",
        label: "Frappé",
        aliases: &[],
    },
    Palette {
        name: "ctp-macchiato",
        label: "Macchiato",
        aliases: &[],
    },
    Palette {
        name: "ctp-mocha",
        label: "Mocha",
        aliases: &[],
    },
];

/// The rows to offer, which is a question about the mode: a palette that looks like another one
/// under the mode in force is not a choice, it is two names for what is on screen.
fn palettes(dark: bool) -> &'static [Palette] {
    if dark {
        &DARK_PALETTES
    } else {
        &LIGHT_PALETTES
    }
}

/// The accent ramp, in Catppuccin's own order — which runs round the hue circle rather than
/// alphabetically, so the grid reads as a spectrum.
const ACCENTS: [(&str, &str); 14] = [
    ("rosewater", "Rosewater"),
    ("flamingo", "Flamingo"),
    ("pink", "Pink"),
    ("mauve", "Mauve"),
    ("red", "Red"),
    ("maroon", "Maroon"),
    ("peach", "Peach"),
    ("yellow", "Yellow"),
    ("green", "Green"),
    ("teal", "Teal"),
    ("sky", "Sky"),
    ("sapphire", "Sapphire"),
    ("blue", "Blue"),
    ("lavender", "Lavender"),
];

/// One row of a column: says what it is, and wears a tick when it is the one in force.
#[component]
fn Choice(selected: Signal<bool>, on_select: Callback<()>, children: Children) -> impl IntoView {
    view! {
        <button
            type="button"
            role="radio"
            aria-checked=move || selected.get().to_string()
            class="flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-left text-sm transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:ring-2 focus-visible:ring-ring focus-visible:outline-none"
            on:click=move |_| on_select.run(())
        >
            {children()}
            <span class="ml-auto flex size-3.5 shrink-0 items-center justify-center">
                <Show when=move || selected.get()>
                    <Check class="size-3.5" />
                </Show>
            </span>
        </button>
    }
}

/// A column of the popover: a heading nothing else uses, and the group it labels.
#[component]
fn Column(
    label: &'static str,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || cn!("flex flex-col gap-1", class.get())>
            <span class="px-2 text-xs font-semibold text-muted-foreground">{label}</span>
            <div role="radiogroup" aria-label=label class="flex flex-col gap-0.5">
                {children()}
            </div>
        </div>
    }
}

/// A dot in the accent grid. The colour is `var(--ac-<name>)`, read through `style` rather than a
/// class: a Tailwind class name has to exist as a literal somewhere for the scanner to find it,
/// and fourteen of them assembled at runtime would not.
#[component]
fn Swatch(
    name: &'static str,
    label: &'static str,
    selected: Signal<bool>,
    on_select: Callback<()>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            role="radio"
            aria-checked=move || selected.get().to_string()
            title=label
            aria-label=label
            style=format!("background-color: var(--ac-{name})")
            on:click=move |_| on_select.run(())
            class=move || {
                cn!(
                    "size-5 rounded-full ring-1 ring-inset ring-foreground/15 transition-transform hover:scale-110 focus-visible:ring-2 focus-visible:ring-ring focus-visible:outline-none",
                    if selected.get() { "ring-2 ring-foreground ring-offset-2 ring-offset-popover" } else { "" },
                )
            }
        />
    }
}

/// A palette's dot. The swatch borrows the palette it names: the theme rules are attribute
/// selectors rather than `:root` ones, so a `data-theme` on the dot makes `--primary` and
/// `--background` inside it that palette's own — which is what a palette differs by. Split
/// diagonally because three of them share a primary and differ only in their surfaces.
#[component]
fn PaletteDot(name: &'static str, #[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <span
            data-theme=(!name.is_empty()).then_some(name)
            style="background: linear-gradient(135deg, var(--primary) 0 50%, var(--background) 50% 100%)"
            class=move || {
                cn!(
                    "size-3.5 shrink-0 rounded-full ring-1 ring-inset ring-foreground/20",
                    class.get(),
                )
            }
        />
    }
}

/// How far the slider goes. Past about 1.5rem a "rounded" component is a lozenge, which is a
/// different design rather than a rounder one.
const MAX_RADIUS_REM: f64 = 1.5;

/// The radius in force, in rem: the custom one if there is one, and otherwise whatever the
/// palette's `--radius` computes to. Read once, when the panel mounts — which the panel does
/// every time it is opened, being portalled and unmounted on close.
fn radius_in_force(theme_ctx: &ThemeContext) -> f64 {
    if let Some(rem) = theme_ctx.radius.with_untracked(|radius| parse_rem(radius)) {
        return rem;
    }

    document()
        .document_element()
        .and_then(|html| window().get_computed_style(&html).ok().flatten())
        .and_then(|style| style.get_property_value("--radius").ok())
        .and_then(|value| parse_rem(&value))
        .unwrap_or(0.0)
}

/// `"0.5rem"` or `"8px"` as a number of rem. Anything else is not a radius this control set.
fn parse_rem(value: &str) -> Option<f64> {
    let value = value.trim();
    if let Some(rem) = value.strip_suffix("rem") {
        rem.parse().ok()
    } else {
        value
            .strip_suffix("px")
            .and_then(|px| px.parse::<f64>().ok())
            .map(|px| px / 16.0)
    }
}

/// The fourth axis, and the one that is a length rather than a name — so a slider, not a list.
/// The popover's own corners are drawn from `--radius`, which makes the panel its own preview.
#[component]
fn RadiusControl() -> impl IntoView {
    let theme_ctx = use_theme();
    let slider = RwSignal::new(vec![radius_in_force(&theme_ctx)]);

    // Only a *change* writes, and only one that is not the panel catching up with the page: the
    // first run is the value just read out of it, and the reset puts the thumb back on the
    // palette's own. Storing either would turn "whatever the palette says" into a custom radius
    // that merely matches it today, and stop it moving when the palette changes.
    Effect::new(move |previous: Option<f64>| {
        let rem = slider.with(|value| value[0]);

        if previous.is_some_and(|previous| previous != rem) && rem != radius_in_force(&theme_ctx) {
            theme_ctx.set_radius(format!("{rem}rem"));
        }

        rem
    });

    view! {
        <div class="flex flex-wrap items-center gap-x-3 gap-y-2 px-4">
            <span class="text-xs font-semibold text-muted-foreground">"Radius"</span>
            <Slider min=0.0 max=MAX_RADIUS_REM step=0.05 value=slider class="min-w-32 flex-1" />
            <span class="w-14 shrink-0 text-right font-mono text-xs text-foreground">
                {move || format!("{:.2}rem", slider.with(|value| value[0]))}
            </span>
            <button
                type="button"
                class="rounded-md px-2 py-1 text-xs text-muted-foreground transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:ring-2 focus-visible:ring-ring focus-visible:outline-none disabled:pointer-events-none disabled:opacity-40"
                disabled=move || theme_ctx.radius.with(String::is_empty)
                on:click=move |_| {
                    theme_ctx.set_radius("");
                    request_animation_frame(move || slider.set(vec![radius_in_force(&theme_ctx)]));
                }
            >
                "Reset"
            </button>
        </div>
    }
}

/// Both switchers open on the same thing: a dot in the colour the palette and accent currently
/// agree on, and the mode's name.
#[component]
fn TriggerFace() -> impl IntoView {
    let theme_ctx = use_theme();

    view! {
        <span class="size-3.5 shrink-0 rounded-full bg-primary ring-1 ring-inset ring-foreground/15" />
        {move || theme_ctx.current.get().label()}
    }
}

/// The full control: mode, palette and accent side by side.
#[component]
pub fn ThemeSwitcherPopover(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    let theme_ctx = use_theme();

    view! {
        <Popover>
            <PopoverTrigger class=move || cn!("gap-2", class.get()) variant=ButtonVariant::Outline>
                <TriggerFace />
            </PopoverTrigger>

            <PopoverPortal>
                // Stacked on a phone and side by side from `sm` up: three columns is about 26rem,
                // which a narrow viewport does not have to give.
                <PopoverContent align=Align::End side=Side::Bottom class="max-w-[calc(100vw-2rem)]">
                    <div class="flex flex-col gap-3 px-4 sm:flex-row sm:gap-4">
                        <Column label="Mode" class="sm:w-28">
                            {Theme::ALL
                                .into_iter()
                                .map(|theme| {
                                    let selected = Signal::derive(move || {
                                        theme_ctx.current.get() == theme
                                    });
                                    view! {
                                        <Choice
                                            selected=selected
                                            on_select=Callback::new(move |_| {
                                                theme_ctx.set_theme(theme)
                                            })
                                        >
                                            {theme.label()}
                                            <Show when=move || {
                                                theme == Theme::System && selected.get()
                                            }>
                                                <span class="text-xs text-muted-foreground">
                                                    {move || theme_ctx.resolved.get().label()}
                                                </span>
                                            </Show>
                                        </Choice>
                                    }
                                })
                                .collect_view()}
                        </Column>

                        <Separator orientation=Orientation::Vertical class="hidden sm:block" />
                        <Separator class="sm:hidden" />

                        <Column label="Palette" class="sm:w-36">
                            {move || {
                                palettes(theme_ctx.is_dark())
                                    .iter()
                                    .map(|palette| {
                                        let Palette { name, label, .. } = *palette;
                                        view! {
                                            <Choice
                                                selected=Signal::derive(move || {
                                                    theme_ctx.color.with(|color| palette.holds(color))
                                                })
                                                on_select=Callback::new(move |_| {
                                                    palette.select(&theme_ctx)
                                                })
                                            >
                                                <PaletteDot name=name />
                                                {label}
                                            </Choice>
                                        }
                                    })
                                    .collect_view()
                            }}
                        </Column>

                        <Separator orientation=Orientation::Vertical class="hidden sm:block" />
                        <Separator class="sm:hidden" />

                        <Column label="Accent" class="sm:w-38">
                            <div class="grid w-fit grid-cols-5 gap-2 px-2 py-1">
                                {ACCENTS
                                    .into_iter()
                                    .map(|(name, label)| {
                                        view! {
                                            <Swatch
                                                name=name
                                                label=label
                                                selected=Signal::derive(move || {
                                                    theme_ctx.accent.with(|accent| accent == name)
                                                })
                                                on_select=Callback::new(move |_| {
                                                    theme_ctx.set_accent(name)
                                                })
                                            />
                                        }
                                    })
                                    .collect_view()}
                            </div>

                            <Choice
                                selected=Signal::derive(move || {
                                    theme_ctx.accent.with(String::is_empty)
                                })
                                on_select=Callback::new(move |_| theme_ctx.set_accent(""))
                            >
                                <span class="text-muted-foreground">"Palette's own"</span>
                            </Choice>
                        </Column>
                    </div>

                    <Separator />
                    <RadiusControl />
                </PopoverContent>
            </PopoverPortal>
        </Popover>
    }
}

/// Mode and palette only, in a menu — for a page that wants one button rather than a panel.
#[component]
pub fn ThemeSwitcher(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    let theme_ctx = use_theme();

    view! {
        <DropdownMenu>
            <DropdownMenuTrigger
                class=move || cn!("gap-2", class.get())
                variant=ButtonVariant::Outline
            >
                <TriggerFace />
            </DropdownMenuTrigger>

            <DropdownMenuPortal>
                <DropdownMenuContent align=Align::End side=Side::Bottom>
                    <DropdownMenuLabel>"Mode"</DropdownMenuLabel>
                    <DropdownMenuSeparator />

                    {Theme::ALL
                        .into_iter()
                        .map(|theme| {
                            view! {
                                <DropdownMenuItem on_click=move |_| {
                                    theme_ctx.set_theme(theme)
                                }>
                                    {theme.label()}
                                    <Tick selected=Signal::derive(move || {
                                        theme_ctx.current.get() == theme
                                    }) />
                                </DropdownMenuItem>
                            }
                        })
                        .collect_view()}

                    <DropdownMenuSeparator />
                    <DropdownMenuLabel>"Palette"</DropdownMenuLabel>
                    <DropdownMenuSeparator />

                    {move || {
                        palettes(theme_ctx.is_dark())
                            .iter()
                            .map(|palette| {
                                let Palette { name, label, .. } = *palette;
                                view! {
                                    <DropdownMenuItem on_click=move |_| palette.select(&theme_ctx)>
                                        <PaletteDot name=name class="mr-2" />
                                        {label}
                                        <Tick selected=Signal::derive(move || {
                                            theme_ctx.color.with(|color| palette.holds(color))
                                        }) />
                                    </DropdownMenuItem>
                                }
                            })
                            .collect_view()
                    }}
                </DropdownMenuContent>
            </DropdownMenuPortal>
        </DropdownMenu>
    }
}

/// The tick a menu item wears when it is the one in force. Always laid out, so the row does not
/// change width when it becomes the selected one.
#[component]
fn Tick(selected: Signal<bool>) -> impl IntoView {
    view! {
        <span class="ml-auto flex size-3.5 shrink-0 items-center justify-center pl-4">
            <Show when=move || selected.get()>
                <Check class="size-3.5" />
            </Show>
        </span>
    }
}
