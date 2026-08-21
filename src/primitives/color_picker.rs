//! A colour picker: a saturation/brightness square, a hue rail, and a text field.
//!
//! The value it holds is [`Hsva`], and everything else is derived from it. That is not a detail —
//! it is the difference between a picker that works and one that fights you. Drag into the black
//! corner of the square with an RGB-backed picker and the hue has nowhere to live, so the rail
//! springs back to red; come out of the corner and you are somewhere you did not choose. Keeping
//! HSVA means the hue you set stays set no matter where the square goes.
//!
//! The caller sees a string, in whichever [`ColorFormat`] is current, and the two are kept in step
//! in both directions — with the one guard that makes that safe: text that parses to the colour we
//! are already showing is ignored rather than converted back, so writing our own output out never
//! walks the hue.
//!
//! The square is two CSS gradients over a pure hue, so there is no canvas here and nothing to
//! rasterise. The rails are the same component twice.

use crate::{
    primitives::drag::{DragPoint, use_drag},
    utils::color::{ColorFormat, Hsva, Rgba},
};
use leptos::{context::Provider, ev, prelude::*};
use leptos_node_ref::AnyNodeRef;

/// How far an arrow key moves, as a fraction of the whole range. Shift multiplies it by ten, the
/// way every other slider in the library does.
const KEY_STEP: f64 = 0.01;

#[derive(Copy, Clone)]
pub struct ColorPickerContext {
    /// The colour, and the only thing that is stored. Everything else is a view of it.
    pub hsva: RwSignal<Hsva>,
    /// The format the text field reads and writes.
    pub format: RwSignal<ColorFormat>,
    /// Whether the picker offers an alpha channel at all.
    pub with_alpha: bool,
}

impl ColorPickerContext {
    pub fn rgba(&self) -> Rgba {
        Rgba::from(self.hsva.get())
    }

    /// The colour written out in the current format — what the caller's signal holds.
    pub fn text(&self) -> String {
        self.format.get().format(self.rgba())
    }

    /// A CSS colour for the current hue at full saturation and brightness: the square's backdrop,
    /// and the colour the hue rail's thumb sits on.
    pub fn hue_color(&self) -> String {
        format!("hsl({} 100% 50%)", self.hsva.with(|hsva| hsva.h.round()))
    }

    /// Reads text into the picker. Returns whether it was a colour at all, so a text field can
    /// show that it was not.
    pub fn set_text(&self, text: &str) -> bool {
        let Some(rgba) = Rgba::parse(text) else {
            return false;
        };

        self.set_rgba(rgba);
        true
    }

    /// Takes a colour in, keeping the hue the picker already has when the new one cannot supply
    /// its own — black and grey have no hue, and losing it would move the rail for no reason.
    pub fn set_rgba(&self, rgba: Rgba) {
        if self.rgba() == rgba {
            return;
        }

        let incoming = Hsva::from(rgba);
        self.hsva.update(|hsva| {
            let hue = if incoming.s == 0.0 {
                hsva.h
            } else {
                incoming.h
            };
            *hsva = Hsva { h: hue, ..incoming };
        });
    }

    pub fn set_saturation_value(&self, saturation: f64, value: f64) {
        self.hsva.update(|hsva| {
            hsva.s = saturation.clamp(0.0, 1.0);
            hsva.v = value.clamp(0.0, 1.0);
        });
    }

    pub fn set_hue(&self, hue: f64) {
        self.hsva.update(|hsva| hsva.h = hue.clamp(0.0, 360.0));
    }

    pub fn set_alpha(&self, alpha: f64) {
        self.hsva.update(|hsva| hsva.a = alpha.clamp(0.0, 1.0));
    }

    /// Moves to the next format, which is what a picker's format button does.
    pub fn cycle_format(&self) {
        self.format.update(|format| {
            let next = ColorFormat::ALL
                .iter()
                .position(|candidate| candidate == format)
                .map(|index| (index + 1) % ColorFormat::ALL.len())
                .unwrap_or(0);
            *format = ColorFormat::ALL[next];
        });
    }
}

pub fn use_color_picker() -> ColorPickerContext {
    expect_context::<ColorPickerContext>()
}

#[component]
pub fn ColorPickerRoot(
    /// The colour as text, in the current format. Read it for a form; write it to set the picker.
    #[prop(optional, into)]
    value: RwSignal<String>,
    #[prop(optional)] format: ColorFormat,
    /// Whether to offer an alpha channel.
    #[prop(default = false)]
    with_alpha: bool,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let opening = Rgba::parse(&value.get_untracked()).unwrap_or(Rgba::new(0, 0, 0));

    let ctx = ColorPickerContext {
        hsva: RwSignal::new(Hsva::from(opening)),
        format: RwSignal::new(format),
        with_alpha,
    };

    // Out: the caller's signal always holds what the picker is showing.
    Effect::new(move |_| {
        let text = ctx.text();
        if value.get_untracked() != text {
            value.set(text);
        }
    });

    // And in. The guard inside `set_rgba` is what keeps this from being a loop: our own output,
    // read back, is the colour we already have, so nothing happens.
    Effect::new(move |_| {
        let text = value.get();
        if let Some(rgba) = Rgba::parse(&text) {
            ctx.set_rgba(rgba);
        }
    });

    view! {
        <Provider value=ctx>
            <div
                data-slot="color-picker"
                data-format=move || ctx.format.get().as_str()
                class=class
            >
                {children()}
            </div>
        </Provider>
    }
}

/// The saturation/brightness square. Saturation runs left to right, brightness bottom to top,
/// which is the arrangement every picker uses and the one the gradients draw for free.
#[component]
pub fn ColorPickerAreaRoot(
    #[prop(optional, into)] class: Signal<String>,
    /// The dot marking the current colour, handed its position as a percentage.
    thumb: Callback<(f64, f64), AnyView>,
) -> impl IntoView {
    let ctx = use_color_picker();
    let area_ref = AnyNodeRef::new();

    let drag = use_drag(move |point: DragPoint| {
        ctx.set_saturation_value(point.fraction_x, 1.0 - point.fraction_y);
    })
    .against(area_ref)
    // The press itself picks a colour, so a click and a drag are one gesture.
    .on_start(move |point: DragPoint| {
        ctx.set_saturation_value(point.fraction_x, 1.0 - point.fraction_y);
    });

    let on_keydown = move |e: ev::KeyboardEvent| {
        let step = if e.shift_key() {
            KEY_STEP * 10.0
        } else {
            KEY_STEP
        };
        let hsva = ctx.hsva.get_untracked();

        let (saturation, value) = match e.key().as_str() {
            "ArrowLeft" => (hsva.s - step, hsva.v),
            "ArrowRight" => (hsva.s + step, hsva.v),
            "ArrowUp" => (hsva.s, hsva.v + step),
            "ArrowDown" => (hsva.s, hsva.v - step),
            "Home" => (0.0, hsva.v),
            "End" => (1.0, hsva.v),
            _ => return,
        };

        e.prevent_default();
        ctx.set_saturation_value(saturation, value);
    };

    view! {
        <div
            node_ref=area_ref
            data-slot="color-picker-area"
            data-dragging=move || drag.active.get().then_some("")
            // A two-axis drag has no honest ARIA role — `slider` is one-dimensional and a
            // grid this is not. The rails and the text field are the paths that announce
            // themselves properly; this one says what it is and what the keys do.
            role="application"
            aria-label="Saturation and brightness. Arrow keys adjust, shift for larger steps."
            tabindex="0"
            on:pointerdown=move |e| drag.start(&e)
            on:keydown=on_keydown
            // White to the right, black to the bottom, the hue underneath: the square, drawn
            // by the browser rather than by us.
            style=move || {
                format!(
                    "background: linear-gradient(to top, #000, transparent), linear-gradient(to right, #fff, transparent), {}; touch-action: none;",
                    ctx.hue_color(),
                )
            }
            class=class
        >
            {move || {
                let hsva = ctx.hsva.get();
                thumb.run((hsva.s * 100.0, (1.0 - hsva.v) * 100.0))
            }}
        </div>
    }
}

/// Which rail: the two differ only in what they set and what they are drawn with.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ColorPickerRail {
    Hue,
    Alpha,
}

impl ColorPickerRail {
    pub fn as_str(&self) -> &'static str {
        match self {
            ColorPickerRail::Hue => "hue",
            ColorPickerRail::Alpha => "alpha",
        }
    }
}

#[component]
pub fn ColorPickerRailRoot(
    rail: ColorPickerRail,
    #[prop(optional, into)] class: Signal<String>,
    /// The thumb, handed its position along the rail as a percentage.
    thumb: Callback<f64, AnyView>,
) -> impl IntoView {
    let ctx = use_color_picker();
    let rail_ref = AnyNodeRef::new();

    let set = move |fraction: f64| match rail {
        ColorPickerRail::Hue => ctx.set_hue(fraction.clamp(0.0, 1.0) * 360.0),
        ColorPickerRail::Alpha => ctx.set_alpha(fraction),
    };

    let drag = use_drag(move |point: DragPoint| set(point.fraction_x))
        .against(rail_ref)
        .on_start(move |point: DragPoint| set(point.fraction_x));

    let fraction = Signal::derive(move || match rail {
        ColorPickerRail::Hue => ctx.hsva.with(|hsva| hsva.h / 360.0),
        ColorPickerRail::Alpha => ctx.hsva.with(|hsva| hsva.a),
    });

    let on_keydown = move |e: ev::KeyboardEvent| {
        let step = if e.shift_key() {
            KEY_STEP * 10.0
        } else {
            KEY_STEP
        };

        let next = match e.key().as_str() {
            "ArrowLeft" | "ArrowDown" => fraction.get_untracked() - step,
            "ArrowRight" | "ArrowUp" => fraction.get_untracked() + step,
            "Home" => 0.0,
            "End" => 1.0,
            _ => return,
        };

        e.prevent_default();
        set(next);
    };

    view! {
        <div
            node_ref=rail_ref
            data-slot="color-picker-rail"
            data-rail=rail.as_str()
            data-dragging=move || drag.active.get().then_some("")
            role="slider"
            aria-label=match rail {
                ColorPickerRail::Hue => "Hue",
                ColorPickerRail::Alpha => "Opacity",
            }
            aria-valuemin="0"
            aria-valuemax=match rail {
                ColorPickerRail::Hue => "360",
                ColorPickerRail::Alpha => "100",
            }
            aria-valuenow=move || match rail {
                ColorPickerRail::Hue => ctx.hsva.with(|hsva| hsva.h.round()),
                ColorPickerRail::Alpha => ctx.hsva.with(|hsva| (hsva.a * 100.0).round()),
            }
            aria-valuetext=move || match rail {
                ColorPickerRail::Hue => {
                    format!("{} degrees", ctx.hsva.with(|hsva| hsva.h.round()))
                }
                ColorPickerRail::Alpha => {
                    format!("{}% opaque", ctx.hsva.with(|hsva| (hsva.a * 100.0).round()))
                }
            }
            tabindex="0"
            on:pointerdown=move |e| drag.start(&e)
            on:keydown=on_keydown
            style=move || {
                let background = match rail {
                    // Every hue, in order. Written out rather than generated so it is one string.
                    ColorPickerRail::Hue => "linear-gradient(to right, #f00 0%, #ff0 17%, #0f0 33%, #0ff 50%, #00f 67%, #f0f 83%, #f00 100%)".to_string(),
                    // Transparent to the colour itself, over whatever chequer the styled layer
                    // put behind it.
                    ColorPickerRail::Alpha => {
                        let Rgba { r, g, b, .. } = ctx.rgba();
                        format!(
                            "linear-gradient(to right, rgb({r} {g} {b} / 0), rgb({r} {g} {b}))",
                        )
                    }
                };

                format!("background-image: {background}; touch-action: none;")
            }
            class=class
        >
            {move || thumb.run(fraction.get() * 100.0)}
        </div>
    }
}

/// The current colour, as a filled box.
#[component]
pub fn ColorPickerSwatchRoot(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    let ctx = use_color_picker();

    view! {
        <div
            data-slot="color-picker-swatch"
            aria-hidden="true"
            style=move || format!("background-color: {}", ColorFormat::Rgb.format(ctx.rgba()))
            class=class
        />
    }
}

/// The text field. Commits on Enter and on leaving, not on every keystroke: half-typed text is
/// not a colour, and reformatting it as it is typed takes the caret with it.
#[component]
pub fn ColorPickerInputRoot(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    let ctx = use_color_picker();
    let draft = RwSignal::new(ctx.text());
    let invalid = RwSignal::new(false);

    // Anything that changes the colour elsewhere — the square, a preset — rewrites the field,
    // unless the reader is in the middle of typing into it.
    Effect::new(move |_| {
        let text = ctx.text();
        if !invalid.get_untracked() {
            draft.set(text);
        }
    });

    let commit = move || {
        if ctx.set_text(&draft.get_untracked()) {
            invalid.set(false);
            draft.set(ctx.text());
        } else {
            invalid.set(true);
        }
    };

    view! {
        <input
            data-slot="color-picker-input"
            type="text"
            autocomplete="off"
            spellcheck="false"
            aria-label="Colour value"
            aria-invalid=move || invalid.get().then_some("true")
            data-invalid=move || invalid.get().then_some("")
            prop:value=move || draft.get()
            on:input=move |e| {
                draft.set(event_target_value(&e));
                invalid.set(false);
            }
            on:keydown=move |e| {
                if e.key() == "Enter" {
                    e.prevent_default();
                    commit();
                }
            }
            on:blur=move |_| commit()
            class=class
        />
    }
}

/// Cycles the format the text field is written in.
#[component]
pub fn ColorPickerFormatRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_color_picker();

    view! {
        <button
            type="button"
            data-slot="color-picker-format"
            aria-label="Change colour format"
            on:click=move |_| ctx.cycle_format()
            class=class
        >
            {match children {
                Some(children) => leptos::either::Either::Left(children()),
                None => {
                    leptos::either::Either::Right(view! { {move || ctx.format.get().as_str()} })
                }
            }}
        </button>
    }
}

/// One colour off a preset row.
#[component]
pub fn ColorPickerPresetRoot(
    #[prop(into)] value: String,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    let ctx = use_color_picker();
    let value = StoredValue::new(value);

    let selected =
        Signal::derive(move || value.with_value(|value| Rgba::parse(value)) == Some(ctx.rgba()));

    view! {
        <button
            type="button"
            data-slot="color-picker-preset"
            data-selected=move || selected.get().then_some("")
            aria-label=value.get_value()
            aria-pressed=move || selected.get().to_string()
            style=format!("background-color: {}", value.get_value())
            on:click=move |_| {
                value.with_value(|value| ctx.set_text(value));
            }
            class=class
        />
    }
}
