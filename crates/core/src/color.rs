//! Colour, in the four spaces a picker has to speak.
//!
//! No crate for this, for the same reason there is no `chrono`: a UI library should not hand its
//! consumers a colour library's version to agree with, and what a picker actually needs is four
//! conversions, a parser and a formatter. All of it is arithmetic, so all of it is tested.
//!
//! **[`Hsva`] is the canonical value, not [`Rgba`].** A saturation/value square with a hue rail
//! beside it *is* HSV, and the round trip through RGB is lossy in exactly the place it hurts:
//! black and white have no hue to recover, so a picker that stores RGB watches its hue rail jump
//! back to red the moment the pointer reaches a corner. Convert outward for display and keep the
//! HSVA.
//!
//! The Oklab matrices are Björn Ottosson's. Oklch is here because the theme's own tokens are
//! written in it — `--chart-1: oklch(0.646 0.222 41.116)` — so a picker that could not read one
//! back would be unable to edit the palette it is themed by.

use std::fmt::Write;

/// 8 bits per channel, and alpha as a fraction. What hex and `rgb()` are made of, and what the
/// screen ultimately gets.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f64,
}

/// Hue in degrees, the rest as fractions. The shape of the picker itself.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Hsva {
    /// 0–360.
    pub h: f64,
    /// 0–1.
    pub s: f64,
    /// 0–1.
    pub v: f64,
    pub a: f64,
}

/// Hue in degrees, the rest as fractions. What CSS's `hsl()` wants.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Hsla {
    pub h: f64,
    pub s: f64,
    pub l: f64,
    pub a: f64,
}

/// Perceptual lightness, chroma, and hue in degrees. Unlike the others this can describe colours
/// no screen can show, so converting to [`Rgba`] clips.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Oklch {
    /// 0–1.
    pub l: f64,
    /// 0 upwards; about 0.37 is as far as sRGB reaches.
    pub c: f64,
    /// 0–360.
    pub h: f64,
    pub a: f64,
}

/// How a colour is written out.
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum ColorFormat {
    /// `#3b82f6`, or `#3b82f680` when it is not opaque.
    #[default]
    Hex,
    /// `rgb(59 130 246)`, modern syntax.
    Rgb,
    /// `hsl(217 91% 60%)`.
    Hsl,
    /// `oklch(0.623 0.188 259.8)`.
    Oklch,
    /// `hsv(217 76% 96%)` — the one format here that is **not** CSS. It is the space the picker
    /// actually stores, so it is the only one where editing a channel changes nothing else, and
    /// the only one where dragging into the black corner and back leaves the hue where it was.
    /// [`parse_color`] reads it; a browser will not, so a value in this format belongs in a
    /// picker rather than in a stylesheet.
    Hsv,
}

/// Which component of a colour a numeric field edits. The three saturations and the two hues are
/// separate on purpose: HSL's saturation is not HSV's, and OKLCH's hue is a different angle from
/// the one HSL and HSV share.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ChannelKind {
    Red,
    Green,
    Blue,
    /// The hue HSL and HSV share.
    Hue,
    HslSaturation,
    Lightness,
    HsvSaturation,
    Value,
    OklchLightness,
    Chroma,
    /// OKLCH's hue, which is not [`ChannelKind::Hue`].
    OklchHue,
    Alpha,
}

/// One numeric field of a format: what it edits, what to call it, and the range it moves in.
///
/// A descriptor rather than a method per channel because this is what a field needs to *render* —
/// a label, a step for the arrow keys, and how many decimals to write back when the colour moves
/// underneath it. Hex has none of these, which is why it has none of these.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ColorChannel {
    pub kind: ChannelKind,
    /// The single letter a field is labelled with.
    pub label: &'static str,
    /// The long name, for an `aria-label` and a `data-channel`.
    pub name: &'static str,
    pub min: f64,
    pub max: f64,
    /// What one arrow key press moves. Shift multiplies it by ten.
    pub step: f64,
    /// How many decimals the field is rewritten with when the colour changes elsewhere.
    pub decimals: usize,
}

const fn channel(
    kind: ChannelKind,
    label: &'static str,
    name: &'static str,
    max: f64,
    step: f64,
    decimals: usize,
) -> ColorChannel {
    ColorChannel {
        kind,
        label,
        name,
        min: 0.0,
        max,
        step,
        decimals,
    }
}

const RGB_CHANNELS: [ColorChannel; 3] = [
    channel(ChannelKind::Red, "R", "red", 255.0, 1.0, 0),
    channel(ChannelKind::Green, "G", "green", 255.0, 1.0, 0),
    channel(ChannelKind::Blue, "B", "blue", 255.0, 1.0, 0),
];

const HSL_CHANNELS: [ColorChannel; 3] = [
    channel(ChannelKind::Hue, "H", "hue", 360.0, 1.0, 0),
    channel(ChannelKind::HslSaturation, "S", "saturation", 100.0, 1.0, 0),
    channel(ChannelKind::Lightness, "L", "lightness", 100.0, 1.0, 0),
];

const HSV_CHANNELS: [ColorChannel; 3] = [
    channel(ChannelKind::Hue, "H", "hue", 360.0, 1.0, 0),
    channel(ChannelKind::HsvSaturation, "S", "saturation", 100.0, 1.0, 0),
    channel(ChannelKind::Value, "V", "value", 100.0, 1.0, 0),
];

// Three decimals on L and C because that is what `format` writes, and a field that rounds
// harder than the string it stands for loses a step every time it is read back.
const OKLCH_CHANNELS: [ColorChannel; 3] = [
    channel(ChannelKind::OklchLightness, "L", "lightness", 1.0, 0.005, 3),
    channel(ChannelKind::Chroma, "C", "chroma", 0.4, 0.005, 3),
    channel(ChannelKind::OklchHue, "H", "hue", 360.0, 1.0, 1),
];

/// Appended to any format's channels when the picker offers an alpha at all. As a percentage,
/// because `0.5` and `50` in the same row of boxes is a trap.
pub const ALPHA_CHANNEL: ColorChannel = channel(ChannelKind::Alpha, "A", "alpha", 100.0, 1.0, 0);

impl ColorFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            ColorFormat::Hex => "hex",
            ColorFormat::Rgb => "rgb",
            ColorFormat::Hsl => "hsl",
            ColorFormat::Oklch => "oklch",
            ColorFormat::Hsv => "hsv",
        }
    }

    /// What a format picker shows. Upper case because these are initialisms, not words.
    pub fn label(&self) -> &'static str {
        match self {
            ColorFormat::Hex => "HEX",
            ColorFormat::Rgb => "RGB",
            ColorFormat::Hsl => "HSL",
            ColorFormat::Oklch => "OKLCH",
            ColorFormat::Hsv => "HSV",
        }
    }

    /// The inverse of [`ColorFormat::as_str`], for reading a choice back out of a select.
    pub fn from_name(name: &str) -> Option<Self> {
        ColorFormat::ALL
            .into_iter()
            .find(|format| format.as_str() == name)
    }

    /// The numeric fields this format is edited as, before any alpha. Hex has none: it is one
    /// string, and three boxes of two hex digits is a worse field than the string already is.
    pub fn channels(&self) -> &'static [ColorChannel] {
        match self {
            ColorFormat::Hex => &[],
            ColorFormat::Rgb => &RGB_CHANNELS,
            ColorFormat::Hsl => &HSL_CHANNELS,
            ColorFormat::Oklch => &OKLCH_CHANNELS,
            ColorFormat::Hsv => &HSV_CHANNELS,
        }
    }

    /// Every format in the order a picker offers them.
    pub const ALL: [ColorFormat; 5] = [
        ColorFormat::Hex,
        ColorFormat::Rgb,
        ColorFormat::Hsl,
        ColorFormat::Hsv,
        ColorFormat::Oklch,
    ];

    /// Writes `color` out. Alpha is only mentioned when there is some to mention, so an opaque
    /// colour never comes back as `#ff0000ff`.
    pub fn format(&self, color: Rgba) -> String {
        let opaque = color.a >= 1.0;

        match self {
            ColorFormat::Hex => color.to_hex(),
            ColorFormat::Rgb => {
                let Rgba { r, g, b, .. } = color;
                if opaque {
                    format!("rgb({r} {g} {b})")
                } else {
                    format!("rgb({r} {g} {b} / {})", format_alpha(color.a))
                }
            }
            ColorFormat::Hsl => {
                let hsl = Hsla::from(color);
                let (h, s, l) = (
                    hsl.h.round(),
                    (hsl.s * 100.0).round(),
                    (hsl.l * 100.0).round(),
                );
                if opaque {
                    format!("hsl({h} {s}% {l}%)")
                } else {
                    format!("hsl({h} {s}% {l}% / {})", format_alpha(color.a))
                }
            }
            ColorFormat::Oklch => {
                let oklch = Oklch::from(color);
                let (l, c, h) = (
                    round_to(oklch.l, 3),
                    round_to(oklch.c, 3),
                    round_to(oklch.h, 1),
                );
                if opaque {
                    format!("oklch({l} {c} {h})")
                } else {
                    format!("oklch({l} {c} {h} / {})", format_alpha(color.a))
                }
            }
            ColorFormat::Hsv => {
                let hsv = Hsva::from(color);
                let (h, s, v) = (
                    hsv.h.round(),
                    (hsv.s * 100.0).round(),
                    (hsv.v * 100.0).round(),
                );
                if opaque {
                    format!("hsv({h} {s}% {v}%)")
                } else {
                    format!("hsv({h} {s}% {v}% / {})", format_alpha(color.a))
                }
            }
        }
    }
}

/// Reads any colour this module can *write*, which is one function more than CSS can read:
/// `hsv()` is the picker's own space, not a CSS colour function. Everything else is
/// [`Rgba::parse`], which stays strictly what a browser would accept.
pub fn parse_color(text: &str) -> Option<Rgba> {
    let trimmed = text.trim();

    let Some(body) = trimmed
        .strip_prefix("hsv(")
        .or_else(|| trimmed.strip_prefix("hsva("))
    else {
        return Rgba::parse(trimmed);
    };

    let body = body.strip_suffix(')')?;
    let (values, alpha) = split_components(body);
    let [h, s, v] = take_three(&values)?;

    Some(Rgba::from(Hsva {
        h: wrap_hue(parse_number(h, 1.0)?),
        s: parse_number(s, 1.0).map(percentage)?,
        v: parse_number(v, 1.0).map(percentage)?,
        a: match alpha {
            Some(alpha) => parse_number(alpha, 1.0)?.clamp(0.0, 1.0),
            None => fourth_alpha(&values, 1.0)?,
        },
    }))
}

/// Trims an alpha to two decimals without leaving `0.50` behind.
fn format_alpha(alpha: f64) -> String {
    let mut text = format!("{:.2}", alpha.clamp(0.0, 1.0));
    while text.ends_with('0') {
        text.pop();
    }
    if text.ends_with('.') {
        text.pop();
    }
    text
}

fn round_to(value: f64, decimals: i32) -> f64 {
    let factor = 10f64.powi(decimals);
    (value * factor).round() / factor
}

/// Wraps a hue into 0–360, so arithmetic on it never has to check.
fn wrap_hue(hue: f64) -> f64 {
    if hue.is_finite() {
        hue.rem_euclid(360.0)
    } else {
        0.0
    }
}

fn to_byte(channel: f64) -> u8 {
    (channel.clamp(0.0, 1.0) * 255.0).round() as u8
}

impl Rgba {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub const fn with_alpha(r: u8, g: u8, b: u8, a: f64) -> Self {
        Self { r, g, b, a }
    }

    /// The channels as fractions, which is what every conversion below works in.
    pub fn fractions(&self) -> (f64, f64, f64) {
        (
            self.r as f64 / 255.0,
            self.g as f64 / 255.0,
            self.b as f64 / 255.0,
        )
    }

    /// `#rrggbb`, or `#rrggbbaa` when it is not opaque.
    pub fn to_hex(&self) -> String {
        let mut hex = String::with_capacity(9);
        let _ = write!(hex, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b);
        if self.a < 1.0 {
            let _ = write!(hex, "{:02x}", to_byte(self.a));
        }
        hex
    }

    /// Reads any of the CSS forms this library writes, plus the comma-separated legacy ones,
    /// percentages, and the 148 named colours.
    ///
    /// A name is tried only once the shapes with punctuation in them have been ruled out, so the
    /// table is never searched for something that cannot be a name.
    pub fn parse(text: &str) -> Option<Self> {
        let text = text.trim();

        if let Some(hex) = text.strip_prefix('#') {
            return Self::parse_hex(hex);
        }

        let Some((function, body)) = text.split_once('(') else {
            return Self::from_name(text);
        };
        let body = body.strip_suffix(')')?;
        let (values, alpha) = split_components(body);
        let alpha = match alpha {
            Some(alpha) => parse_number(alpha, 1.0)?.clamp(0.0, 1.0),
            None => 1.0,
        };

        match function.trim().to_ascii_lowercase().as_str() {
            "rgb" | "rgba" => {
                let [r, g, b] = take_three(&values)?;
                Some(Self {
                    r: to_byte(parse_number(r, 255.0)? / 255.0),
                    g: to_byte(parse_number(g, 255.0)? / 255.0),
                    b: to_byte(parse_number(b, 255.0)? / 255.0),
                    a: fourth_alpha(&values, alpha)?,
                })
            }
            "hsl" | "hsla" => {
                let [h, s, l] = take_three(&values)?;
                Some(Rgba::from(Hsla {
                    h: wrap_hue(parse_number(h, 1.0)?),
                    s: parse_number(s, 1.0).map(percentage)?,
                    l: parse_number(l, 1.0).map(percentage)?,
                    a: fourth_alpha(&values, alpha)?,
                }))
            }
            "oklch" => {
                let [l, c, h] = take_three(&values)?;
                Some(Rgba::from(Oklch {
                    // `oklch(70% …)` and `oklch(0.7 …)` are the same colour.
                    l: parse_number(l, 1.0).map(|l| if l > 1.0 { l / 100.0 } else { l })?,
                    c: parse_number(c, 0.4)?,
                    h: wrap_hue(parse_number(h, 1.0)?),
                    a: alpha,
                }))
            }
            _ => None,
        }
    }

    fn parse_hex(hex: &str) -> Option<Self> {
        let digits: Vec<u8> = hex
            .chars()
            .map(|digit| digit.to_digit(16).map(|digit| digit as u8))
            .collect::<Option<Vec<_>>>()?;

        // The short forms double each digit: `#f0a` is `#ff00aa`.
        let (r, g, b, a) = match digits.as_slice() {
            [r, g, b] => (r * 17, g * 17, b * 17, 255),
            [r, g, b, a] => (r * 17, g * 17, b * 17, a * 17),
            [r1, r2, g1, g2, b1, b2] => (r1 * 16 + r2, g1 * 16 + g2, b1 * 16 + b2, 255),
            [r1, r2, g1, g2, b1, b2, a1, a2] => {
                (r1 * 16 + r2, g1 * 16 + g2, b1 * 16 + b2, a1 * 16 + a2)
            }
            _ => return None,
        };

        Some(Self {
            r,
            g,
            b,
            a: a as f64 / 255.0,
        })
    }

    /// A CSS colour keyword, or `None` for anything that is not one. Case-insensitive, since CSS
    /// is: `RebeccaPurple` is the same keyword as `rebeccapurple`.
    ///
    /// `transparent` is a name like the rest as far as a caller is concerned, and the only one
    /// that carries an alpha.
    pub fn from_name(name: &str) -> Option<Self> {
        let name = name.trim();

        // Only ASCII letters can be a keyword, and checking that first is what keeps a typo'd
        // `rgb 1 2 3` out of the search.
        if name.is_empty() || !name.bytes().all(|b| b.is_ascii_alphabetic()) {
            return None;
        }

        let name = name.to_ascii_lowercase();
        if name == "transparent" {
            return Some(Self::with_alpha(0, 0, 0, 0.0));
        }

        NAMED_COLORS
            .binary_search_by(|(known, _)| (*known).cmp(name.as_str()))
            .ok()
            .map(|at| {
                let packed = NAMED_COLORS[at].1;
                Self::new((packed >> 16) as u8, (packed >> 8) as u8, packed as u8)
            })
    }

    /// The CSS keyword for this colour, where there is one — for a swatch that would rather say
    /// "rebeccapurple" than "#663399".
    ///
    /// Where two keywords name one colour the first alphabetically wins, which is the spelling
    /// CSS itself lists first: `aqua` over `cyan`, `gray` over `grey`.
    pub fn name(&self) -> Option<&'static str> {
        if self.a <= 0.0 {
            return Some("transparent");
        }
        if self.a < 1.0 {
            return None;
        }

        let packed = (self.r as u32) << 16 | (self.g as u32) << 8 | self.b as u32;
        NAMED_COLORS
            .iter()
            .find(|(_, known)| *known == packed)
            .map(|(name, _)| *name)
    }
}

/// `50%` is a half whichever slot it is in; a bare number is already one.
fn percentage(value: f64) -> f64 {
    if value > 1.0 { value / 100.0 } else { value }
}

/// Legacy `rgba(r, g, b, a)` and `hsla(h, s, l, a)` carry alpha as a fourth value rather than
/// after a slash; whichever is present wins, and both is not an error worth failing over.
fn fourth_alpha(values: &[&str], slash_alpha: f64) -> Option<f64> {
    match values.get(3) {
        Some(alpha) => Some(parse_number(alpha, 1.0)?.clamp(0.0, 1.0)),
        None => Some(slash_alpha),
    }
}

fn take_three<'a>(values: &[&'a str]) -> Option<[&'a str; 3]> {
    match values {
        [a, b, c, ..] => Some([a, b, c]),
        _ => None,
    }
}

/// Splits a function body into its values and its slash-separated alpha. Commas and whitespace
/// are both separators, since CSS accepts either.
fn split_components(body: &str) -> (Vec<&str>, Option<&str>) {
    let (values, alpha) = match body.split_once('/') {
        Some((values, alpha)) => (values, Some(alpha.trim())),
        None => (body, None),
    };

    let values = values
        .split([',', ' ', '\t', '\n'])
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .collect();

    (values, alpha)
}

/// A number, with `%` read against `full` — 100% of 255 is 255, of 1.0 is 1.0.
fn parse_number(text: &str, full: f64) -> Option<f64> {
    match text.trim().strip_suffix('%') {
        Some(number) => number.trim().parse::<f64>().ok().map(|n| n / 100.0 * full),
        None => text.trim().parse::<f64>().ok(),
    }
}

impl From<Hsva> for Rgba {
    fn from(hsva: Hsva) -> Self {
        let h = wrap_hue(hsva.h) / 60.0;
        let s = hsva.s.clamp(0.0, 1.0);
        let v = hsva.v.clamp(0.0, 1.0);

        let sector = h.floor();
        let fraction = h - sector;

        let p = v * (1.0 - s);
        let q = v * (1.0 - s * fraction);
        let t = v * (1.0 - s * (1.0 - fraction));

        let (r, g, b) = match sector as i64 % 6 {
            0 => (v, t, p),
            1 => (q, v, p),
            2 => (p, v, t),
            3 => (p, q, v),
            4 => (t, p, v),
            _ => (v, p, q),
        };

        Self {
            r: to_byte(r),
            g: to_byte(g),
            b: to_byte(b),
            a: hsva.a,
        }
    }
}

impl From<Rgba> for Hsva {
    fn from(rgba: Rgba) -> Self {
        let (r, g, b) = rgba.fractions();
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let chroma = max - min;

        // A grey has no hue to report. The caller keeps its own, which is the whole argument for
        // storing HSVA rather than deriving it.
        let h = if chroma == 0.0 {
            0.0
        } else if max == r {
            60.0 * (((g - b) / chroma) % 6.0)
        } else if max == g {
            60.0 * ((b - r) / chroma + 2.0)
        } else {
            60.0 * ((r - g) / chroma + 4.0)
        };

        Self {
            h: wrap_hue(h),
            s: if max == 0.0 { 0.0 } else { chroma / max },
            v: max,
            a: rgba.a,
        }
    }
}

impl From<Hsla> for Rgba {
    fn from(hsla: Hsla) -> Self {
        Rgba::from(Hsva::from(hsla))
    }
}

impl From<Rgba> for Hsla {
    fn from(rgba: Rgba) -> Self {
        Hsla::from(Hsva::from(rgba))
    }
}

/// HSV and HSL share a hue and differ only in how they measure the rest, so this is exact in both
/// directions — unlike anything that goes through 8-bit RGB.
impl From<Hsva> for Hsla {
    fn from(hsva: Hsva) -> Self {
        let l = hsva.v * (1.0 - hsva.s / 2.0);
        let s = if l == 0.0 || l == 1.0 {
            0.0
        } else {
            (hsva.v - l) / l.min(1.0 - l)
        };

        Self {
            h: wrap_hue(hsva.h),
            s,
            l,
            a: hsva.a,
        }
    }
}

impl From<Hsla> for Hsva {
    fn from(hsla: Hsla) -> Self {
        let v = hsla.l + hsla.s * hsla.l.min(1.0 - hsla.l);
        let s = if v == 0.0 {
            0.0
        } else {
            2.0 * (1.0 - hsla.l / v)
        };

        Self {
            h: wrap_hue(hsla.h),
            s,
            v,
            a: hsla.a,
        }
    }
}

/// sRGB's transfer function: the curve between what a channel says and how much light it means.
fn to_linear(channel: f64) -> f64 {
    if channel <= 0.04045 {
        channel / 12.92
    } else {
        ((channel + 0.055) / 1.055).powf(2.4)
    }
}

fn from_linear(channel: f64) -> f64 {
    if channel <= 0.003_130_8 {
        12.92 * channel
    } else {
        1.055 * channel.powf(1.0 / 2.4) - 0.055
    }
}

impl From<Rgba> for Oklch {
    fn from(rgba: Rgba) -> Self {
        let (r, g, b) = rgba.fractions();
        let (r, g, b) = (to_linear(r), to_linear(g), to_linear(b));

        let l = 0.412_221_470_8 * r + 0.536_332_536_3 * g + 0.051_445_992_9 * b;
        let m = 0.211_903_498_2 * r + 0.680_699_545_1 * g + 0.107_396_956_6 * b;
        let s = 0.088_302_461_9 * r + 0.281_718_837_6 * g + 0.629_978_700_5 * b;

        let (l, m, s) = (l.cbrt(), m.cbrt(), s.cbrt());

        let lightness = 0.210_454_255_3 * l + 0.793_617_785_0 * m - 0.004_072_046_8 * s;
        let a = 1.977_998_495_1 * l - 2.428_592_205_0 * m + 0.450_593_709_9 * s;
        let b = 0.025_904_037_1 * l + 0.782_771_766_2 * m - 0.808_675_766_0 * s;

        let chroma = (a * a + b * b).sqrt();
        // A neutral has no hue; reporting one would make the rail jump, as with HSV.
        let hue = if chroma < 1e-7 {
            0.0
        } else {
            wrap_hue(b.atan2(a).to_degrees())
        };

        Self {
            l: lightness,
            c: chroma,
            h: hue,
            a: rgba.a,
        }
    }
}

impl From<Oklch> for Rgba {
    fn from(oklch: Oklch) -> Self {
        let hue = wrap_hue(oklch.h).to_radians();
        let a = oklch.c * hue.cos();
        let b = oklch.c * hue.sin();

        let l = oklch.l + 0.396_337_777_4 * a + 0.215_803_757_3 * b;
        let m = oklch.l - 0.105_561_345_8 * a - 0.063_854_172_8 * b;
        let s = oklch.l - 0.089_484_177_5 * a - 1.291_485_548_0 * b;

        let (l, m, s) = (l * l * l, m * m * m, s * s * s);

        let r = 4.076_741_662_1 * l - 3.307_711_591_3 * m + 0.230_969_929_2 * s;
        let g = -1.268_438_004_6 * l + 2.609_757_401_1 * m - 0.341_319_396_5 * s;
        let blue = -0.004_196_086_3 * l - 0.703_418_614_7 * m + 1.707_614_701_0 * s;

        // Clipped, not gamut-mapped: an oklch outside sRGB has no honest answer here, and the
        // nearest showable colour is what a picker wants to land on.
        Self {
            r: to_byte(from_linear(r)),
            g: to_byte(from_linear(g)),
            b: to_byte(from_linear(blue)),
            a: oklch.a,
        }
    }
}

impl From<Hsva> for Oklch {
    fn from(hsva: Hsva) -> Self {
        Oklch::from(Rgba::from(hsva))
    }
}

impl From<Oklch> for Hsva {
    fn from(oklch: Oklch) -> Self {
        Hsva::from(Rgba::from(oklch))
    }
}

/// The CSS named colours, sorted so [`Rgba::parse`] can binary-search them, and packed one to a
/// `u32` so the whole table is 148 names and 148 words rather than 148 structs.
///
/// `transparent` is not in here: it is the one name that is not opaque, so it cannot be packed
/// beside the others and is matched before the search.
const NAMED_COLORS: &[(&str, u32); 148] = &[
    ("aliceblue", 0xf0f8ff),
    ("antiquewhite", 0xfaebd7),
    ("aqua", 0x00ffff),
    ("aquamarine", 0x7fffd4),
    ("azure", 0xf0ffff),
    ("beige", 0xf5f5dc),
    ("bisque", 0xffe4c4),
    ("black", 0x000000),
    ("blanchedalmond", 0xffebcd),
    ("blue", 0x0000ff),
    ("blueviolet", 0x8a2be2),
    ("brown", 0xa52a2a),
    ("burlywood", 0xdeb887),
    ("cadetblue", 0x5f9ea0),
    ("chartreuse", 0x7fff00),
    ("chocolate", 0xd2691e),
    ("coral", 0xff7f50),
    ("cornflowerblue", 0x6495ed),
    ("cornsilk", 0xfff8dc),
    ("crimson", 0xdc143c),
    ("cyan", 0x00ffff),
    ("darkblue", 0x00008b),
    ("darkcyan", 0x008b8b),
    ("darkgoldenrod", 0xb8860b),
    ("darkgray", 0xa9a9a9),
    ("darkgreen", 0x006400),
    ("darkgrey", 0xa9a9a9),
    ("darkkhaki", 0xbdb76b),
    ("darkmagenta", 0x8b008b),
    ("darkolivegreen", 0x556b2f),
    ("darkorange", 0xff8c00),
    ("darkorchid", 0x9932cc),
    ("darkred", 0x8b0000),
    ("darksalmon", 0xe9967a),
    ("darkseagreen", 0x8fbc8f),
    ("darkslateblue", 0x483d8b),
    ("darkslategray", 0x2f4f4f),
    ("darkslategrey", 0x2f4f4f),
    ("darkturquoise", 0x00ced1),
    ("darkviolet", 0x9400d3),
    ("deeppink", 0xff1493),
    ("deepskyblue", 0x00bfff),
    ("dimgray", 0x696969),
    ("dimgrey", 0x696969),
    ("dodgerblue", 0x1e90ff),
    ("firebrick", 0xb22222),
    ("floralwhite", 0xfffaf0),
    ("forestgreen", 0x228b22),
    ("fuchsia", 0xff00ff),
    ("gainsboro", 0xdcdcdc),
    ("ghostwhite", 0xf8f8ff),
    ("gold", 0xffd700),
    ("goldenrod", 0xdaa520),
    ("gray", 0x808080),
    ("green", 0x008000),
    ("greenyellow", 0xadff2f),
    ("grey", 0x808080),
    ("honeydew", 0xf0fff0),
    ("hotpink", 0xff69b4),
    ("indianred", 0xcd5c5c),
    ("indigo", 0x4b0082),
    ("ivory", 0xfffff0),
    ("khaki", 0xf0e68c),
    ("lavender", 0xe6e6fa),
    ("lavenderblush", 0xfff0f5),
    ("lawngreen", 0x7cfc00),
    ("lemonchiffon", 0xfffacd),
    ("lightblue", 0xadd8e6),
    ("lightcoral", 0xf08080),
    ("lightcyan", 0xe0ffff),
    ("lightgoldenrodyellow", 0xfafad2),
    ("lightgray", 0xd3d3d3),
    ("lightgreen", 0x90ee90),
    ("lightgrey", 0xd3d3d3),
    ("lightpink", 0xffb6c1),
    ("lightsalmon", 0xffa07a),
    ("lightseagreen", 0x20b2aa),
    ("lightskyblue", 0x87cefa),
    ("lightslategray", 0x778899),
    ("lightslategrey", 0x778899),
    ("lightsteelblue", 0xb0c4de),
    ("lightyellow", 0xffffe0),
    ("lime", 0x00ff00),
    ("limegreen", 0x32cd32),
    ("linen", 0xfaf0e6),
    ("magenta", 0xff00ff),
    ("maroon", 0x800000),
    ("mediumaquamarine", 0x66cdaa),
    ("mediumblue", 0x0000cd),
    ("mediumorchid", 0xba55d3),
    ("mediumpurple", 0x9370db),
    ("mediumseagreen", 0x3cb371),
    ("mediumslateblue", 0x7b68ee),
    ("mediumspringgreen", 0x00fa9a),
    ("mediumturquoise", 0x48d1cc),
    ("mediumvioletred", 0xc71585),
    ("midnightblue", 0x191970),
    ("mintcream", 0xf5fffa),
    ("mistyrose", 0xffe4e1),
    ("moccasin", 0xffe4b5),
    ("navajowhite", 0xffdead),
    ("navy", 0x000080),
    ("oldlace", 0xfdf5e6),
    ("olive", 0x808000),
    ("olivedrab", 0x6b8e23),
    ("orange", 0xffa500),
    ("orangered", 0xff4500),
    ("orchid", 0xda70d6),
    ("palegoldenrod", 0xeee8aa),
    ("palegreen", 0x98fb98),
    ("paleturquoise", 0xafeeee),
    ("palevioletred", 0xdb7093),
    ("papayawhip", 0xffefd5),
    ("peachpuff", 0xffdab9),
    ("peru", 0xcd853f),
    ("pink", 0xffc0cb),
    ("plum", 0xdda0dd),
    ("powderblue", 0xb0e0e6),
    ("purple", 0x800080),
    ("rebeccapurple", 0x663399),
    ("red", 0xff0000),
    ("rosybrown", 0xbc8f8f),
    ("royalblue", 0x4169e1),
    ("saddlebrown", 0x8b4513),
    ("salmon", 0xfa8072),
    ("sandybrown", 0xf4a460),
    ("seagreen", 0x2e8b57),
    ("seashell", 0xfff5ee),
    ("sienna", 0xa0522d),
    ("silver", 0xc0c0c0),
    ("skyblue", 0x87ceeb),
    ("slateblue", 0x6a5acd),
    ("slategray", 0x708090),
    ("slategrey", 0x708090),
    ("snow", 0xfffafa),
    ("springgreen", 0x00ff7f),
    ("steelblue", 0x4682b4),
    ("tan", 0xd2b48c),
    ("teal", 0x008080),
    ("thistle", 0xd8bfd8),
    ("tomato", 0xff6347),
    ("turquoise", 0x40e0d0),
    ("violet", 0xee82ee),
    ("wheat", 0xf5deb3),
    ("white", 0xffffff),
    ("whitesmoke", 0xf5f5f5),
    ("yellow", 0xffff00),
    ("yellowgreen", 0x9acd32),
];

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64, tolerance: f64) -> bool {
        (a - b).abs() <= tolerance
    }

    #[test]
    fn reads_every_hex_form() {
        assert_eq!(Rgba::parse("#f00"), Some(Rgba::new(255, 0, 0)));
        assert_eq!(Rgba::parse("#ff0000"), Some(Rgba::new(255, 0, 0)));
        assert_eq!(Rgba::parse("  #3B82F6 "), Some(Rgba::new(59, 130, 246)));

        let half = Rgba::parse("#ff000080").unwrap();
        assert_eq!((half.r, half.g, half.b), (255, 0, 0));
        assert!(close(half.a, 0.502, 0.001));

        // A short form doubles its digits rather than padding them.
        assert_eq!(Rgba::parse("#f0a"), Some(Rgba::new(255, 0, 170)));

        assert_eq!(Rgba::parse("#12345"), None);
        assert_eq!(Rgba::parse("#gg0000"), None);
    }

    #[test]
    fn reads_colour_keywords() {
        assert_eq!(Rgba::parse("rebeccapurple"), Some(Rgba::new(102, 51, 153)));
        assert_eq!(
            Rgba::parse("  RebeccaPurple "),
            Some(Rgba::new(102, 51, 153))
        );
        assert_eq!(Rgba::parse("white"), Some(Rgba::new(255, 255, 255)));

        // The two spellings of the same colour agree, as do the two names for one hue.
        assert_eq!(Rgba::parse("gray"), Rgba::parse("grey"));
        assert_eq!(Rgba::parse("aqua"), Rgba::parse("cyan"));

        // The one keyword with an alpha.
        let clear = Rgba::parse("transparent").unwrap();
        assert_eq!(clear.a, 0.0);

        assert_eq!(Rgba::parse("burntsienna"), None);
        assert_eq!(Rgba::parse(""), None);
        assert_eq!(Rgba::parse("rgb 255 0 0"), None);
    }

    #[test]
    fn names_a_colour_back() {
        assert_eq!(Rgba::new(102, 51, 153).name(), Some("rebeccapurple"));
        assert_eq!(Rgba::new(128, 128, 128).name(), Some("gray"));
        assert_eq!(Rgba::new(0, 255, 255).name(), Some("aqua"));
        assert_eq!(Rgba::with_alpha(0, 0, 0, 0.0).name(), Some("transparent"));

        // A colour between two keywords has no name, and neither does a translucent one.
        assert_eq!(Rgba::new(59, 130, 246).name(), None);
        assert_eq!(Rgba::with_alpha(255, 0, 0, 0.5).name(), None);
    }

    #[test]
    fn the_keyword_table_can_be_searched() {
        // `from_name` binary-searches it, which is only correct while it is sorted.
        assert!(NAMED_COLORS.windows(2).all(|pair| pair[0].0 < pair[1].0));
        assert_eq!(NAMED_COLORS.len(), 148);

        // Every name round-trips through the parser, whatever the reverse lookup calls it.
        for (name, packed) in NAMED_COLORS {
            let parsed = Rgba::parse(name).unwrap_or_else(|| panic!("{name} did not parse"));
            let expected = Rgba::new((packed >> 16) as u8, (packed >> 8) as u8, *packed as u8);
            assert_eq!(parsed, expected, "{name}");
        }
    }

    #[test]
    fn reads_functional_forms_old_and_new() {
        let red = Rgba::new(255, 0, 0);
        assert_eq!(Rgba::parse("rgb(255 0 0)"), Some(red));
        assert_eq!(Rgba::parse("rgb(255, 0, 0)"), Some(red));
        assert_eq!(Rgba::parse("rgb(100% 0% 0%)"), Some(red));
        assert_eq!(Rgba::parse("hsl(0 100% 50%)"), Some(red));
        assert_eq!(Rgba::parse("hsl(0, 100%, 50%)"), Some(red));

        // Alpha, after a slash or in the fourth slot.
        assert_eq!(Rgba::parse("rgb(255 0 0 / 0.5)").unwrap().a, 0.5);
        assert_eq!(Rgba::parse("rgba(255, 0, 0, 0.5)").unwrap().a, 0.5);
        assert_eq!(Rgba::parse("hsl(0 100% 50% / 50%)").unwrap().a, 0.5);
        assert_eq!(Rgba::parse("rgb(255 0 0)").unwrap().a, 1.0);

        assert_eq!(Rgba::parse("rgb(255 0)"), None);
        assert_eq!(Rgba::parse("cmyk(1 0 0 0)"), None);
    }

    #[test]
    fn hsv_and_rgb_agree_on_the_corners() {
        let red = Hsva::from(Rgba::new(255, 0, 0));
        assert!(close(red.h, 0.0, 1e-9) && close(red.s, 1.0, 1e-9) && close(red.v, 1.0, 1e-9));

        let teal = Hsva::from(Rgba::new(0, 128, 128));
        assert!(close(teal.h, 180.0, 0.5));

        // Black and white have no hue to recover — the reason a picker keeps its own.
        assert_eq!(Hsva::from(Rgba::new(0, 0, 0)).h, 0.0);
        assert_eq!(Hsva::from(Rgba::new(255, 255, 255)).s, 0.0);

        for color in [
            Rgba::new(255, 0, 0),
            Rgba::new(59, 130, 246),
            Rgba::new(17, 17, 17),
            Rgba::new(255, 255, 255),
        ] {
            assert_eq!(Rgba::from(Hsva::from(color)), color);
        }
    }

    #[test]
    fn hsv_and_hsl_round_trip_exactly() {
        for hsva in [
            Hsva {
                h: 217.0,
                s: 0.76,
                v: 0.96,
                a: 1.0,
            },
            Hsva {
                h: 0.0,
                s: 0.0,
                v: 0.5,
                a: 0.5,
            },
            Hsva {
                h: 359.9,
                s: 1.0,
                v: 1.0,
                a: 1.0,
            },
        ] {
            let back = Hsva::from(Hsla::from(hsva));
            assert!(close(back.h, hsva.h, 1e-9), "{back:?} vs {hsva:?}");
            assert!(close(back.s, hsva.s, 1e-9), "{back:?} vs {hsva:?}");
            assert!(close(back.v, hsva.v, 1e-9), "{back:?} vs {hsva:?}");
        }

        // Mid-grey: HSL says half lightness, HSV says full value and no saturation.
        let hsl = Hsla::from(Hsva {
            h: 0.0,
            s: 0.0,
            v: 0.5,
            a: 1.0,
        });
        assert!(close(hsl.l, 0.5, 1e-9) && close(hsl.s, 0.0, 1e-9));
    }

    #[test]
    fn oklch_matches_the_published_values() {
        // Ottosson's own worked example, and what CSS Color 4 quotes for sRGB red.
        let red = Oklch::from(Rgba::new(255, 0, 0));
        assert!(close(red.l, 0.6280, 0.001), "{red:?}");
        assert!(close(red.c, 0.2577, 0.001), "{red:?}");
        assert!(close(red.h, 29.23, 0.05), "{red:?}");

        let white = Oklch::from(Rgba::new(255, 255, 255));
        assert!(close(white.l, 1.0, 0.001) && close(white.c, 0.0, 0.001));

        let black = Oklch::from(Rgba::new(0, 0, 0));
        assert!(close(black.l, 0.0, 0.001) && black.h == 0.0);
    }

    #[test]
    fn oklch_round_trips_through_srgb() {
        for color in [
            Rgba::new(255, 0, 0),
            Rgba::new(59, 130, 246),
            Rgba::new(120, 200, 40),
            Rgba::new(7, 7, 7),
        ] {
            assert_eq!(Rgba::from(Oklch::from(color)), color);
        }

        // One of the theme's own tokens. It is written in a space wider than sRGB and this one
        // does not fit: reading it lands on the nearest showable colour, which is a little less
        // saturated and a few degrees round the hue circle. The string does not survive that, and
        // no sRGB picker could make it — but the *colour* is stable, so editing a palette does not
        // walk it further away with every pass.
        let token = "oklch(0.646 0.222 41.116)";
        let once = ColorFormat::Oklch.format(Rgba::parse(token).unwrap());
        let twice = ColorFormat::Oklch.format(Rgba::parse(&once).unwrap());
        assert_ne!(once, token);
        assert_eq!(once, twice, "clipping has to settle, not drift");
    }

    #[test]
    fn clips_colours_no_screen_can_show() {
        // Far outside sRGB. A wrapping conversion would come back with a channel at the wrong
        // end of its range; this lands on a real green with its alpha intact.
        let clipped = Rgba::from(Oklch {
            l: 0.7,
            c: 0.9,
            h: 150.0,
            a: 0.5,
        });
        assert_eq!(clipped, Rgba::with_alpha(0, 251, 0, 0.5));

        // And what comes back is inside the gamut, rather than still out of it.
        assert!(Oklch::from(clipped).c < 0.4);
    }

    #[test]
    fn writes_each_format_the_way_css_wants_it() {
        let blue = Rgba::new(59, 130, 246);
        assert_eq!(ColorFormat::Hex.format(blue), "#3b82f6");
        assert_eq!(ColorFormat::Rgb.format(blue), "rgb(59 130 246)");
        assert_eq!(ColorFormat::Hsl.format(blue), "hsl(217 91% 60%)");
        assert_eq!(ColorFormat::Oklch.format(blue), "oklch(0.623 0.188 259.8)");

        // Alpha appears only when there is some.
        let faded = Rgba::with_alpha(59, 130, 246, 0.5);
        assert_eq!(ColorFormat::Hex.format(faded), "#3b82f680");
        assert_eq!(ColorFormat::Rgb.format(faded), "rgb(59 130 246 / 0.5)");
        assert!(ColorFormat::Oklch.format(faded).ends_with("/ 0.5)"));
    }

    #[test]
    fn every_format_reads_back_what_it_wrote() {
        for color in [
            Rgba::new(255, 0, 0),
            Rgba::new(59, 130, 246),
            Rgba::new(0, 0, 0),
        ] {
            for format in ColorFormat::ALL {
                let text = format.format(color);
                // `parse_color`, not `Rgba::parse`: `hsv()` is ours and not CSS, which is the
                // whole distinction between the two readers.
                let parsed = parse_color(&text).unwrap_or_else(|| panic!("could not read {text}"));

                // Hex and rgb are exact; the rest are written to a readable number of decimals,
                // so they come back within a byte or two.
                let tolerance = match format {
                    ColorFormat::Hex | ColorFormat::Rgb => 0,
                    _ => 2,
                };

                assert!(
                    parsed.r.abs_diff(color.r) <= tolerance
                        && parsed.g.abs_diff(color.g) <= tolerance
                        && parsed.b.abs_diff(color.b) <= tolerance,
                    "{text} read back as {parsed:?}, not {color:?}"
                );
            }
        }
    }

    #[test]
    fn hsv_is_written_and_read_back() {
        let blue = Rgba::new(59, 130, 246);
        let text = ColorFormat::Hsv.format(blue);
        assert!(text.starts_with("hsv("), "{text}");
        // Through 8-bit RGB and back, so a channel or two may land next door.
        let back = parse_color(&text).expect("hsv round trip");
        assert!(
            (i16::from(back.r) - 59).abs() <= 1
                && (i16::from(back.g) - 130).abs() <= 1
                && (i16::from(back.b) - 246).abs() <= 1,
            "{back:?}"
        );
    }

    #[test]
    fn hsv_carries_an_alpha_either_way_round() {
        let half = Rgba {
            a: 0.5,
            ..Rgba::new(255, 0, 0)
        };
        assert_eq!(
            parse_color(&ColorFormat::Hsv.format(half)).map(|c| c.a),
            Some(0.5)
        );
        assert_eq!(
            parse_color("hsva(0 100% 100% / 0.25)").map(|c| c.a),
            Some(0.25)
        );
        assert_eq!(
            parse_color("hsv(0, 100%, 100%, 0.25)").map(|c| c.a),
            Some(0.25)
        );
    }

    #[test]
    fn hsv_is_the_one_thing_rgba_parse_will_not_take() {
        assert!(Rgba::parse("hsv(0 100% 100%)").is_none());
        assert!(parse_color("hsv(0 100% 100%)").is_some());
        // And everything else goes straight through it.
        assert_eq!(parse_color("#3b82f6"), Rgba::parse("#3b82f6"));
        assert_eq!(parse_color("rebeccapurple"), Rgba::parse("rebeccapurple"));
    }

    #[test]
    fn a_format_name_round_trips() {
        for format in ColorFormat::ALL {
            assert_eq!(ColorFormat::from_name(format.as_str()), Some(format));
        }
        assert_eq!(ColorFormat::from_name("cmyk"), None);
    }

    #[test]
    fn every_format_but_hex_is_edited_as_three_channels() {
        for format in ColorFormat::ALL {
            let expected = usize::from(format != ColorFormat::Hex) * 3;
            assert_eq!(format.channels().len(), expected, "{format:?}");
        }
    }
}
