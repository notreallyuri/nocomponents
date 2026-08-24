//! A colour picker: a saturation/brightness square, a hue rail, and the colour as numbers.
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
//! Which numbers a reader edits is a question about the format rather than about the picker, so
//! [`ColorPickerContext::channels`] answers it and [`ColorPickerChannelRoot`] is one field of the
//! answer. Hex has no channels — it is one string, and three boxes of two hex digits would be a
//! worse field than the string already is — so it keeps [`ColorPickerInputRoot`], which is also
//! the only field anything can be *pasted* into.
//!
//! The square is two CSS gradients over a pure hue, so there is no canvas here and nothing to
//! rasterise. The rails are the same component twice.

use crate::{
    primitives::drag::{DragPoint, use_drag},
    utils::color::{
        ALPHA_CHANNEL, ChannelKind, ColorChannel, ColorFormat, Hsla, Hsva, Oklch, Rgba, parse_color,
    },
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
        let Some(rgba) = parse_color(text) else {
            return false;
        };

        self.set_rgba(rgba);
        true
    }

    /// Takes a colour in, keeping the hue the picker already has when the new one cannot supply
    /// its own — black and grey have no hue, and losing it would move the rail for no reason.
    ///
    /// The guard reads the colour **untracked**, and that is not an optimisation. This runs inside
    /// the effect that watches the caller's text signal, so a tracked read here would subscribe
    /// that effect to the picker's own state: the next drag would write a colour, wake the effect
    /// with the caller's text still one step behind, and have the old colour written straight back
    /// over the gesture. See the note on the two effects in [`ColorPickerRoot`].
    pub fn set_rgba(&self, rgba: Rgba) {
        if self.hsva.with_untracked(|hsva| Rgba::from(*hsva)) == rgba {
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

    /// The numeric fields the current format is edited as, in order — the format's own, plus an
    /// alpha when the picker has one to offer. Empty for hex, which is one string rather than a
    /// row of boxes.
    pub fn channels(&self) -> Vec<ColorChannel> {
        let mut channels = self.format.get().channels().to_vec();

        if self.with_alpha && !channels.is_empty() {
            channels.push(ALPHA_CHANNEL);
        }

        channels
    }

    /// What one channel currently reads.
    ///
    /// The two hue channels come off the stored HSVA rather than out of a round trip, because a
    /// grey has no hue to convert back: read through RGB, the field would say 0 for every
    /// unsaturated colour and take the rail with it the moment anyone typed into it.
    pub fn channel(&self, channel: ColorChannel) -> f64 {
        let hsva = self.hsva.get();

        match channel.kind {
            ChannelKind::Red => f64::from(self.rgba().r),
            ChannelKind::Green => f64::from(self.rgba().g),
            ChannelKind::Blue => f64::from(self.rgba().b),
            ChannelKind::Hue => hsva.h,
            ChannelKind::HslSaturation => Hsla::from(self.rgba()).s * 100.0,
            ChannelKind::Lightness => Hsla::from(self.rgba()).l * 100.0,
            ChannelKind::HsvSaturation => hsva.s * 100.0,
            ChannelKind::Value => hsva.v * 100.0,
            ChannelKind::OklchLightness => Oklch::from(self.rgba()).l,
            ChannelKind::Chroma => Oklch::from(self.rgba()).c,
            ChannelKind::OklchHue => Oklch::from(self.rgba()).h,
            ChannelKind::Alpha => hsva.a * 100.0,
        }
    }

    /// Writes one channel, leaving the others where they are.
    ///
    /// HSV's three go straight into the store, which is the whole reason the picker holds HSVA:
    /// no conversion, so nothing else drifts. The others are a round trip through RGB, which is
    /// lossy in the last digit and unavoidable — HSL's lightness and OKLCH's chroma are not
    /// independent of anything the store keeps.
    pub fn set_channel(&self, channel: ColorChannel, value: f64) {
        let value = value.clamp(channel.min, channel.max);

        match channel.kind {
            ChannelKind::Hue => self.set_hue(value),
            ChannelKind::HsvSaturation => {
                self.hsva.update(|hsva| hsva.s = value / 100.0);
            }
            ChannelKind::Value => {
                self.hsva.update(|hsva| hsva.v = value / 100.0);
            }
            ChannelKind::Alpha => self.set_alpha(value / 100.0),
            ChannelKind::Red => self.set_rgba(Rgba {
                r: value as u8,
                ..self.rgba()
            }),
            ChannelKind::Green => self.set_rgba(Rgba {
                g: value as u8,
                ..self.rgba()
            }),
            ChannelKind::Blue => self.set_rgba(Rgba {
                b: value as u8,
                ..self.rgba()
            }),
            ChannelKind::HslSaturation => {
                let hsla = Hsla {
                    s: value / 100.0,
                    ..self.hsla()
                };
                self.set_rgba(Rgba::from(hsla));
            }
            ChannelKind::Lightness => {
                let hsla = Hsla {
                    l: value / 100.0,
                    ..self.hsla()
                };
                self.set_rgba(Rgba::from(hsla));
            }
            ChannelKind::OklchLightness => {
                let oklch = Oklch {
                    l: value,
                    ..self.oklch()
                };
                self.set_rgba(Rgba::from(oklch));
            }
            ChannelKind::Chroma => {
                let oklch = Oklch {
                    c: value,
                    ..self.oklch()
                };
                self.set_rgba(Rgba::from(oklch));
            }
            ChannelKind::OklchHue => {
                let oklch = Oklch {
                    h: value,
                    ..self.oklch()
                };
                self.set_rgba(Rgba::from(oklch));
            }
        }
    }

    /// The colour as HSL, with the hue taken from the store rather than the round trip — same
    /// reason as [`ColorPickerContext::channel`].
    fn hsla(&self) -> Hsla {
        Hsla {
            h: self.hsva.with(|hsva| hsva.h),
            ..Hsla::from(self.rgba())
        }
    }

    fn oklch(&self) -> Oklch {
        Oklch::from(self.rgba())
    }

    /// The format as the string a select binds to, kept in step in both directions.
    ///
    /// Minted per call and owned by whoever calls it, so call it once per picker. It exists
    /// because a `Select` speaks `RwSignal<Option<String>>` and the picker speaks `ColorFormat`,
    /// and the two effects that bridge them are wiring rather than styling — which puts them
    /// here rather than in the component that renders the select.
    pub fn format_selection(&self) -> RwSignal<Option<String>> {
        let format = self.format;
        let selection = RwSignal::new(Some(format.get_untracked().as_str().to_string()));

        Effect::new(move |_| {
            let name = format.get().as_str();
            if selection.with_untracked(|current| current.as_deref() != Some(name)) {
                selection.set(Some(name.to_string()));
            }
        });

        Effect::new(move |_| {
            if let Some(chosen) = selection.get().as_deref().and_then(ColorFormat::from_name)
                && format.get_untracked() != chosen
            {
                format.set(chosen);
            }
        });

        selection
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
    let opening = parse_color(&value.get_untracked()).unwrap_or(Rgba::new(0, 0, 0));

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

    // And in. This one must depend on `value` and nothing else — `set_rgba` reads the colour
    // untracked for that reason.
    //
    // The two effects are a loop with one exit: writing text out is lossy (two decimals of alpha,
    // three of oklch's lightness), so what comes back is rarely bit-for-bit what went out, and the
    // guard inside `set_rgba` catches the case where it is. What used to break was the ordering.
    // Once a round trip revised the colour — which is every drag on an alpha rail, and every drag
    // on anything in oklch — this effect had been woken by that revision and ran *ahead* of the
    // one above, so each pointer move was read back as a stale caller write and undone before it
    // was ever published. The rail took the press and then went dead for the rest of the gesture.
    Effect::new(move |_| {
        let text = value.get();
        if let Some(rgba) = parse_color(&text) {
            ctx.set_rgba(rgba);
        }
    });

    view! {
        <Provider value=ctx>
            <div data-slot="color-picker" data-format=move || ctx.format.get().as_str() class=class>
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
                ColorPickerRail::Hue => format!("{} degrees", ctx.hsva.with(|hsva| hsva.h.round())),
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
                    ColorPickerRail::Hue => {
                        "linear-gradient(to right, #f00 0%, #ff0 17%, #0f0 33%, #0ff 50%, #00f 67%, #f0f 83%, #f00 100%)"
                            .to_string()
                    }
                    // Transparent to the colour itself, over whatever chequer the styled layer
                    // put behind it.
                    ColorPickerRail::Alpha => {
                        let Rgba { r, g, b, .. } = ctx.rgba();
                        format!("linear-gradient(to right, rgb({r} {g} {b} / 0), rgb({r} {g} {b}))")
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

/// One channel's numeric field.
///
/// Commits on every keystroke that reads as a number, unlike the text field beside it: a channel
/// is one number, so there is no half-typed state that means something else, and a picker whose
/// square does not move while you type into R is a picker that feels broken. What the draft is
/// for is the states that are not numbers yet — an empty box, a lone minus, `0.` on the way to
/// `0.5` — which must be allowed to sit there rather than being snapped to zero.
///
/// Arrow keys step by the channel's own step, shift by ten of them, which is what the rails and
/// every slider in the library already do.
#[component]
pub fn ColorPickerChannelRoot(
    channel: ColorChannel,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    let ctx = use_color_picker();

    let written = move || format!("{:.*}", channel.decimals, ctx.channel(channel));
    let draft = RwSignal::new(written());
    // While the box has focus its own text wins: rewriting it under the caret would renumber
    // what is being typed, and `0.5` would become `0.500` between the point and the five.
    let editing = RwSignal::new(false);

    Effect::new(move |_| {
        let text = written();
        if !editing.get_untracked() {
            draft.set(text);
        }
    });

    let step_by = move |steps: f64| {
        let next = ctx.channel(channel) + steps * channel.step;
        ctx.set_channel(channel, next);
        draft.set(format!("{:.*}", channel.decimals, ctx.channel(channel)));
    };

    view! {
        <input
            data-slot="color-picker-channel"
            data-channel=channel.name
            type="text"
            inputmode="decimal"
            autocomplete="off"
            spellcheck="false"
            aria-label=channel.name
            role="spinbutton"
            aria-valuemin=channel.min
            aria-valuemax=channel.max
            aria-valuenow=move || ctx.channel(channel)
            prop:value=move || draft.get()
            on:focus=move |_| editing.set(true)
            on:input=move |e| {
                let text = event_target_value(&e);

                match text.trim().parse::<f64>() {
                    // Out of range is not half-typed, it is wrong: the colour clamps, and a box
                    // still reading 999 next to a swatch that went to 255 is the field lying
                    // about what it did. Anything inside the range keeps the text as typed, so
                    // `0.` on the way to `0.5` survives.
                    Ok(value) if value < channel.min || value > channel.max => {
                        ctx.set_channel(channel, value);
                        draft.set(written());
                    }
                    Ok(value) => {
                        ctx.set_channel(channel, value);
                        draft.set(text);
                    }
                    Err(_) => draft.set(text),
                }
            }
            on:keydown=move |e| {
                let multiplier = if e.shift_key() { 10.0 } else { 1.0 };
                match e.key().as_str() {
                    "ArrowUp" => step_by(multiplier),
                    "ArrowDown" => step_by(-multiplier),
                    "Enter" => {}
                    _ => return,
                }
                e.prevent_default();
                draft.set(format!("{:.*}", channel.decimals, ctx.channel(channel)));
            }
            on:blur=move |_| {
                editing.set(false);
                draft.set(format!("{:.*}", channel.decimals, ctx.channel(channel)));
            }
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
                None => leptos::either::Either::Right(view! { {move || ctx.format.get().as_str()} }),
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
        Signal::derive(move || value.with_value(|value| parse_color(value)) == Some(ctx.rgba()));

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
