//! A video player: the `<video>` element, and the controls that drive it.
//!
//! **The element is the state, and this mirrors it.** Nothing here decides that the video is
//! playing — the element says so and [`VideoPlayerContext::sync`] reads it back. Pressing play
//! calls `play()` and then waits for the `play` event, which is the only honest way round,
//! because a browser is entitled to refuse: an autoplay policy rejects the promise, and a player
//! that had set `playing` on the press would sit there showing a pause button over a still frame.
//! One `sync` behind every media event rather than a handler per fact, so no two facts can be a
//! frame apart and a new fact costs a line rather than a listener.
//!
//! The modes ([`VideoPlayerMode`]) are **behaviour, not decoration**. Which controls a bar draws
//! is the styled layer's business, but the two modes that draw none differ in something only this
//! layer can express: whether the reader may press the video at all.
//!
//! **The player does not own a playlist.** Previous and Next report [`VideoPlayerStep`] and the
//! caller moves their own list on, the same stance as the kanban board reporting a move rather
//! than applying it. A player that kept a list would disagree with the caller's the moment
//! anything else touched it — and most callers have one video, which is why the skip controls
//! disable themselves when nobody is listening.
//!
//! Dragging goes through [`crate::primitives::drag`], so a pointer that leaves the scrubber
//! mid-drag still belongs to the drag, and the gesture ends on `pointercancel`.

use crate::primitives::drag::{DragPoint, use_drag};
use leptos::{context::Provider, ev, prelude::*, wasm_bindgen, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;
use web_sys::{Element, HtmlMediaElement, TextTrack, TextTrackKind, TextTrackMode};

/// The readable text of a cue, with WebVTT's own markup taken out.
///
/// [`VideoPlayerContext::cues`] draws cues as ordinary elements, and a `VTTCue`'s `text` is the
/// source as it was authored — so a file using the markup the format allows (`<i>`, `<b>`,
/// `<v Roger>`, the `<00:00:01.000>` karaoke timestamps) would otherwise show its own tags to the
/// reader. Tags are dropped and what they wrapped is kept, which is what a player that cannot
/// render them should do with them.
///
/// A `<` is always a tag in WebVTT — a literal one is written `&lt;` — so an unterminated tag is
/// the rest of the line rather than something to put back.
fn cue_text(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut rest = raw;

    while let Some(open) = rest.find('<') {
        out.push_str(&rest[..open]);
        match rest[open..].find('>') {
            Some(close) => rest = &rest[open + close + 1..],
            None => return decode_entities(&out),
        }
    }
    out.push_str(rest);

    decode_entities(&out)
}

/// The six character references WebVTT defines. Anything else is left as it was written, because
/// a cue is not HTML and `&hellip;` in one is three literal words.
fn decode_entities(text: &str) -> String {
    if !text.contains('&') {
        return text.to_string();
    }

    const ENTITIES: &[(&str, &str)] = &[
        ("&amp;", "&"),
        ("&lt;", "<"),
        ("&gt;", ">"),
        ("&nbsp;", "\u{a0}"),
        ("&lrm;", "\u{200e}"),
        ("&rlm;", "\u{200f}"),
    ];

    let mut out = String::with_capacity(text.len());
    let mut rest = text;

    'outer: while let Some(amp) = rest.find('&') {
        out.push_str(&rest[..amp]);
        for (entity, replacement) in ENTITIES {
            if rest[amp..].starts_with(entity) {
                out.push_str(replacement);
                rest = &rest[amp + entity.len()..];
                continue 'outer;
            }
        }
        // `&amp;` is decoded first above, so a bare `&` here is one the file meant literally.
        out.push('&');
        rest = &rest[amp + 1..];
    }
    out.push_str(rest);

    out
}

/// How far a keypress moves the scrubber, in seconds, and the volume, as a fraction.
const SEEK_STEP: f64 = 5.0;
const VOLUME_STEP: f64 = 0.05;

/// How much of the player the reader is given.
///
/// The first three are control bars of decreasing size and the styled layer decides what goes in
/// them. The last two draw nothing, and the difference between them is the whole reason this is a
/// mode rather than a set of flags: one is a video you can press, the other is a video you cannot.
#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
pub enum VideoPlayerMode {
    /// Every control there is.
    #[default]
    Full,
    /// Play/pause, previous and next, volume, and the time bar.
    Reduced,
    /// Play/pause and the time bar.
    Limited,
    /// No controls. The video itself is the play button, which is the only way in.
    None,
    /// No controls and no way in: the video plays or it does not, and the reader has no say.
    Uncontrolled,
}

impl VideoPlayerMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Full => "full",
            Self::Reduced => "reduced",
            Self::Limited => "limited",
            Self::None => "none",
            Self::Uncontrolled => "uncontrolled",
        }
    }

    /// Whether the reader may drive the player at all. The one thing a styled layer cannot decide
    /// for itself, since it is about the keyboard and the video's own click as much as the bar.
    pub fn interactive(&self) -> bool {
        !matches!(self, Self::Uncontrolled)
    }

    /// Whether a control bar is drawn at all.
    pub fn has_controls(&self) -> bool {
        matches!(self, Self::Full | Self::Reduced | Self::Limited)
    }

    /// Volume and the skip pair go together: they are what [`Self::Reduced`] keeps and
    /// [`Self::Limited`] drops.
    pub fn has_volume(&self) -> bool {
        matches!(self, Self::Full | Self::Reduced)
    }

    pub fn has_skip(&self) -> bool {
        matches!(self, Self::Full | Self::Reduced)
    }

    /// The timestamp and fullscreen are what [`Self::Full`] adds over [`Self::Reduced`].
    pub fn has_timestamp(&self) -> bool {
        matches!(self, Self::Full)
    }

    pub fn has_fullscreen(&self) -> bool {
        matches!(self, Self::Full)
    }
}

/// Which way a skip control goes. The player reports it; the caller owns the list.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VideoPlayerStep {
    Previous,
    Next,
}

impl VideoPlayerStep {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Previous => "previous",
            Self::Next => "next",
        }
    }
}

/// The two things in a player chosen by dragging along a line. One component, twice — they differ
/// in what a fraction of the way along means and in nothing else.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VideoPlayerRail {
    Seek,
    Volume,
}

impl VideoPlayerRail {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Seek => "seek",
            Self::Volume => "volume",
        }
    }
}

/// How big the cues are drawn, as a reader preference rather than a caller's styling choice.
///
/// Here rather than in the styled layer because the reader picks it from a menu at runtime, which
/// makes it state the player has to hold — the same argument as [`VideoPlayerRail`]. What each
/// one *measures* is the styled layer's business, and it writes the sizes against the player's own
/// width so that going fullscreen makes the captions bigger along with everything else.
#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
pub enum VideoPlayerCaptionSize {
    Small,
    #[default]
    Medium,
    Large,
    Huge,
}

impl VideoPlayerCaptionSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::Huge => "huge",
        }
    }

    /// In the order a menu lists them, smallest first.
    pub const ALL: &'static [Self] = &[Self::Small, Self::Medium, Self::Large, Self::Huge];

    /// What a menu calls it.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Small => "Small",
            Self::Medium => "Medium",
            Self::Large => "Large",
            Self::Huge => "Huge",
        }
    }
}

/// A subtitle or caption track the `<video>` is carrying.
///
/// Read back off the element rather than taken as a prop, because `<track>` elements are the
/// caller's own children and the browser is what turns them into tracks — a list passed in
/// alongside them would be a second copy to keep in step.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VideoPlayerTrack {
    /// Its position in the element's own track list, which is what selects it.
    pub index: usize,
    /// What to call it in a menu. Falls back to the language tag, then to a number, since
    /// `label` is optional on a `<track>` and an unlabelled entry is unpickable.
    pub label: String,
    pub language: String,
}

#[derive(Copy, Clone)]
pub struct VideoPlayerContext {
    /// Mirrored from the element's `play` and `pause`, never set by pressing a button.
    pub playing: RwSignal<bool>,
    pub current_time: RwSignal<f64>,
    /// Zero until the metadata arrives, and stays zero for a live stream, whose duration is
    /// infinite — a scrubber over an unknown length is a scrubber that lies.
    pub duration: RwSignal<f64>,
    /// How far ahead of the playhead the browser has buffered, in seconds.
    pub buffered: RwSignal<f64>,
    pub volume: RwSignal<f64>,
    pub muted: RwSignal<bool>,
    pub rate: RwSignal<f64>,
    /// Whether playback is stalled waiting for data, for a spinner over the frame.
    pub waiting: RwSignal<bool>,
    pub ended: RwSignal<bool>,
    pub fullscreen: RwSignal<bool>,
    /// Whether the reader has touched the player lately. See [`Self::controls_visible`].
    pub active: RwSignal<bool>,
    /// True for as long as a rail is being dragged, so a thumb can stay out while it is.
    pub scrubbing: RwSignal<bool>,
    /// Subtitle and caption tracks the element is carrying, read off it once it has them.
    pub tracks: RwSignal<Vec<VideoPlayerTrack>>,
    /// Which of [`Self::tracks`] is showing, or `None` for captions off.
    pub active_track: RwSignal<Option<usize>>,
    /// The text of the cues on screen right now, one entry per active cue.
    ///
    /// **The player draws these itself rather than letting the browser draw them**, which is why
    /// the active track is set to [`TextTrackMode::Hidden`] rather than `Showing`: hidden keeps
    /// the cue list current and `cuechange` firing, and only stops the painting. The browser's
    /// own captions sit at the bottom of the video frame, which is exactly where a control bar
    /// goes, and there is no portable way to move them — Chrome offers
    /// `::-webkit-media-text-track-container`, which honours `transform` and ignores the
    /// `translate` property, and Firefox has no such pseudo-element at all. Drawn as ordinary
    /// elements they are positioned, themed and animated like anything else.
    ///
    /// The cost, and it is a real one: the browser's VTT layout goes with its rendering, so cue
    /// settings that place a line somewhere other than the bottom (`line`, `position`, `align`,
    /// `size`, `vertical`) are not honoured, and inline cue markup (`<b>`, `<v Speaker>`) arrives
    /// as the text it is written as. Captions that are a line of dialogue at the bottom — which
    /// is nearly all of them — are unaffected.
    pub cues: RwSignal<Vec<String>>,
    /// How big the reader has asked for the cues to be drawn. See [`VideoPlayerCaptionSize`].
    pub caption_size: RwSignal<VideoPlayerCaptionSize>,
    /// Whether a menu is open over the controls. The idle timer has to know: a bar that fades
    /// out from under an open menu leaves the menu hanging over nothing.
    pub menu_open: RwSignal<bool>,
    pub mode: VideoPlayerMode,
    pub video_ref: AnyNodeRef,
    /// The element that goes fullscreen, which is the whole player rather than the video: a
    /// `<video>` in fullscreen draws the browser's own controls over ours.
    pub root_ref: AnyNodeRef,
    on_skip: Option<Callback<VideoPlayerStep>>,
}

/// How far ahead of `time` the browser has buffered.
///
/// The ranges are the ones actually held, which after a seek is more than one — so the answer is
/// the end of whichever range the playhead is in, not the end of the last. A player that read the
/// last range would draw a buffered bar over a gap it has not got.
fn buffered_to(media: &HtmlMediaElement, time: f64) -> f64 {
    let ranges = media.buffered();

    for i in 0..ranges.length() {
        let (Ok(start), Ok(end)) = (ranges.start(i), ranges.end(i)) else {
            continue;
        };
        if start <= time && time <= end {
            return end;
        }
    }

    0.0
}

impl VideoPlayerContext {
    fn media(&self) -> Option<HtmlMediaElement> {
        self.video_ref
            .get_untracked()?
            .dyn_into::<HtmlMediaElement>()
            .ok()
    }

    /// Reads the whole of the player's state back off the element.
    ///
    /// Every media event calls this rather than updating the one fact it is named after: they
    /// arrive together anyway, reading a property is free beside the event that carried it, and a
    /// handler per fact is how a player ends up with a pause button over a paused video.
    pub fn sync(&self) {
        let Some(media) = self.media() else {
            return;
        };

        let time = media.current_time();
        let duration = media.duration();
        // NaN before the metadata lands, and infinite for a live stream. Both mean "no length to
        // scrub along", and zero is how the rest of this file spells that.
        let duration = if duration.is_finite() && duration > 0.0 {
            duration
        } else {
            0.0
        };

        set_if_changed(self.playing, !media.paused() && !media.ended());
        set_if_changed(self.current_time, time);
        set_if_changed(self.duration, duration);
        set_if_changed(self.buffered, buffered_to(&media, time));
        set_if_changed(self.volume, media.volume());
        set_if_changed(self.muted, media.muted());
        set_if_changed(self.rate, media.playback_rate());
        set_if_changed(self.ended, media.ended());
    }

    pub fn play(&self) {
        if let Some(media) = self.media() {
            // The promise it returns is allowed to reject — an autoplay policy, a source that
            // will not load — and the `play` event is what says it did not. Nothing to do here.
            let _ = media.play();
        }
    }

    pub fn pause(&self) {
        if let Some(media) = self.media() {
            let _ = media.pause();
        }
    }

    pub fn toggle(&self) {
        if self.playing.get_untracked() {
            self.pause();
        } else {
            self.play();
        }
    }

    /// Moves the playhead, and reads back where it landed rather than assuming: a seek past the
    /// end is clamped by the element, and a seek into an unbuffered stretch may not land at all.
    pub fn seek_to(&self, seconds: f64) {
        let Some(media) = self.media() else {
            return;
        };

        let duration = self.duration.get_untracked();
        let target = if duration > 0.0 {
            seconds.clamp(0.0, duration)
        } else {
            seconds.max(0.0)
        };

        media.set_current_time(target);
        self.sync();
    }

    pub fn seek_by(&self, seconds: f64) {
        self.seek_to(self.current_time.get_untracked() + seconds);
    }

    /// Sets the volume, and takes the mute off when it is raised — a volume control that moves
    /// while the sound stays off is a volume control that appears broken.
    pub fn set_volume(&self, volume: f64) {
        let Some(media) = self.media() else {
            return;
        };

        let volume = volume.clamp(0.0, 1.0);
        media.set_volume(volume);
        media.set_muted(volume <= 0.0);
        self.sync();
    }

    pub fn toggle_muted(&self) {
        let Some(media) = self.media() else {
            return;
        };

        let muted = !media.muted();
        media.set_muted(muted);
        // Coming off mute at zero would be silence with the control showing sound, so it comes
        // back somewhere audible.
        if !muted && media.volume() <= 0.0 {
            media.set_volume(VOLUME_STEP * 4.0);
        }
        self.sync();
    }

    pub fn set_rate(&self, rate: f64) {
        if let Some(media) = self.media() {
            media.set_playback_rate(rate);
            self.sync();
        }
    }

    pub fn toggle_fullscreen(&self) {
        let document = document();

        if document.fullscreen_element().is_some() {
            document.exit_fullscreen();
            return;
        }

        if let Some(root) = self.root_ref.get_untracked() {
            let _ = root.request_fullscreen();
        }
    }

    pub fn skip(&self, step: VideoPlayerStep) {
        if let Some(on_skip) = self.on_skip {
            on_skip.run(step);
        }
    }

    /// Whether there is anywhere to skip to. Nobody listening means no playlist, and a control
    /// that does nothing should say so rather than swallow the press.
    pub fn can_skip(&self) -> bool {
        self.on_skip.is_some()
    }

    /// How far through, from 0 at the start to 1 at the end.
    pub fn progress(&self) -> f64 {
        let duration = self.duration.get();
        if duration <= 0.0 {
            return 0.0;
        }
        (self.current_time.get() / duration).clamp(0.0, 1.0)
    }

    /// How much of the track is buffered, on the same scale as [`Self::progress`].
    pub fn buffered_progress(&self) -> f64 {
        let duration = self.duration.get();
        if duration <= 0.0 {
            return 0.0;
        }
        (self.buffered.get() / duration).clamp(0.0, 1.0)
    }

    /// Where a rail's thumb sits, from 0 to 1.
    pub fn fraction_of(&self, rail: VideoPlayerRail) -> f64 {
        match rail {
            VideoPlayerRail::Seek => self.progress(),
            // Muted reads as silent rather than as wherever the slider was left, because that is
            // what the reader is hearing.
            VideoPlayerRail::Volume => {
                if self.muted.get() {
                    0.0
                } else {
                    self.volume.get().clamp(0.0, 1.0)
                }
            }
        }
    }

    fn set_fraction(&self, rail: VideoPlayerRail, fraction: f64) {
        let fraction = fraction.clamp(0.0, 1.0);
        match rail {
            VideoPlayerRail::Seek => {
                let duration = self.duration.get_untracked();
                if duration > 0.0 {
                    self.seek_to(fraction * duration);
                }
            }
            VideoPlayerRail::Volume => self.set_volume(fraction),
        }
    }

    /// Reads the element's own subtitle and caption tracks, and which of them is showing.
    ///
    /// Not part of [`Self::sync`]: the list only changes when the source does, and walking it on
    /// every `timeupdate` would be four DOM reads a second for an answer that does not move.
    pub fn sync_tracks(&self) {
        let Some(media) = self.media() else {
            return;
        };
        let Some(list) = media.text_tracks() else {
            return;
        };

        let mut tracks = Vec::new();
        let mut active = None;

        for index in 0..list.length() {
            let Some(track) = list.get(index) else {
                continue;
            };
            // Chapters and descriptions are tracks too, and neither belongs in a captions menu.
            if !matches!(
                track.kind(),
                TextTrackKind::Subtitles | TextTrackKind::Captions
            ) {
                continue;
            }

            let index = index as usize;
            // `Hidden` is what this player calls showing — see [`VideoPlayerContext::cues`]. A
            // `<track default>` comes up `Showing`, though, so the browser is already painting it
            // by the time this runs: demote it, or the reader gets both sets of captions at once.
            match track.mode() {
                TextTrackMode::Showing => {
                    track.set_mode(TextTrackMode::Hidden);
                    active = Some(index);
                }
                TextTrackMode::Hidden => active = Some(index),
                _ => {}
            }

            let language = track.language();
            let label = match (track.label(), language.clone()) {
                (label, _) if !label.is_empty() => label,
                (_, language) if !language.is_empty() => language,
                _ => format!("Track {}", index + 1),
            };

            tracks.push(VideoPlayerTrack {
                index,
                label,
                language,
            });
        }

        if self.tracks.with_untracked(|current| *current != tracks) {
            self.tracks.set(tracks);
        }
        set_if_changed(self.active_track, active);
    }

    /// Shows one track and hides the rest, or hides them all for `None`.
    ///
    /// The chosen one is set [`TextTrackMode::Hidden`] rather than `Showing`, which is this
    /// player's "on": the cues stay current and the browser stops painting them, because
    /// [`VideoPlayerContext::cues`] is what draws them. Every other track is explicitly disabled
    /// rather than left alone: a `<track default>` comes up showing, and two at once paints one
    /// caption over another.
    pub fn set_text_track(&self, index: Option<usize>) {
        let Some(media) = self.media() else {
            return;
        };
        let Some(list) = media.text_tracks() else {
            return;
        };

        for i in 0..list.length() {
            let Some(track) = list.get(i) else {
                continue;
            };
            track.set_mode(if Some(i as usize) == index {
                TextTrackMode::Hidden
            } else {
                TextTrackMode::Disabled
            });
        }

        set_if_changed(self.active_track, index);
        self.sync_cues();
    }

    /// Reads the active track's cues into [`Self::cues`].
    ///
    /// Driven by the track's own `cuechange` rather than by [`Self::sync`]: `timeupdate` fires
    /// about four times a second, which would put a subtitle on screen up to a quarter of a
    /// second after the word was said and take it off just as late.
    pub fn sync_cues(&self) {
        let cues = self
            .active_text_track()
            .and_then(|track| track.active_cues())
            .map(|list| {
                (0..list.length())
                    .filter_map(|i| list.get(i))
                    .map(|cue| cue_text(&cue.text()))
                    .filter(|text| !text.trim().is_empty())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        if self.cues.with_untracked(|current| *current != cues) {
            self.cues.set(cues);
        }
    }

    /// The element's own [`TextTrack`] for [`Self::active_track`], which is an index into the
    /// element's list rather than into [`Self::tracks`].
    pub fn active_text_track(&self) -> Option<TextTrack> {
        let index = self.active_track.get_untracked()?;
        self.media()?.text_tracks()?.get(index as u32)
    }

    /// Whether the controls should be on screen: always while paused, and for a moment after the
    /// reader last did anything while playing.
    ///
    /// A bar that hides over a paused video hides it from someone who has stopped to read it.
    pub fn controls_visible(&self) -> bool {
        !self.playing.get() || self.active.get() || self.scrubbing.get() || self.menu_open.get()
    }
}

/// Signals notify whether or not the value changed, and [`VideoPlayerContext::sync`] writes eight
/// of them several times a second. Every one of those is a render if it is not guarded here.
fn set_if_changed<T: PartialEq + Send + Sync + 'static>(signal: RwSignal<T>, value: T) {
    if signal.with_untracked(|current| *current != value) {
        signal.set(value);
    }
}

pub fn use_video_player() -> VideoPlayerContext {
    expect_context::<VideoPlayerContext>()
}

/// A number of seconds as a player writes it: `0:07`, `12:34`, `1:02:03`.
///
/// Hours appear only when there are any, since a two-minute clip reading `0:01:23` is a clip
/// pretending to be a film. Anything that is not a finite length — before the metadata lands, or
/// a live stream — is `--:--`, which is a player admitting it does not know yet rather than
/// showing a confident `0:00`.
pub fn format_timestamp(seconds: f64) -> String {
    if !seconds.is_finite() || seconds < 0.0 {
        return "--:--".to_string();
    }

    let total = seconds.floor() as u64;
    let (hours, minutes, seconds) = (total / 3600, (total % 3600) / 60, total % 60);

    if hours > 0 {
        format!("{hours}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes}:{seconds:02}")
    }
}

#[component]
pub fn VideoPlayerRoot(
    #[prop(optional)] mode: VideoPlayerMode,
    /// Where Previous and Next report to. Without it there is no playlist and both disable.
    #[prop(default = None, into)]
    on_skip: Option<Callback<VideoPlayerStep>>,
    /// How long the controls stay up after the reader last did anything, in milliseconds.
    #[prop(default = 2500)]
    idle_delay: u64,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let root_ref = AnyNodeRef::new();
    let ctx = VideoPlayerContext {
        playing: RwSignal::new(false),
        current_time: RwSignal::new(0.0),
        duration: RwSignal::new(0.0),
        buffered: RwSignal::new(0.0),
        volume: RwSignal::new(1.0),
        muted: RwSignal::new(false),
        rate: RwSignal::new(1.0),
        waiting: RwSignal::new(false),
        ended: RwSignal::new(false),
        fullscreen: RwSignal::new(false),
        active: RwSignal::new(true),
        scrubbing: RwSignal::new(false),
        tracks: RwSignal::new(Vec::new()),
        active_track: RwSignal::new(None),
        cues: RwSignal::new(Vec::new()),
        caption_size: RwSignal::new(VideoPlayerCaptionSize::default()),
        menu_open: RwSignal::new(false),
        mode,
        video_ref: AnyNodeRef::new(),
        root_ref,
        on_skip,
    };

    provide_context(ctx);

    // The countdown that puts the controls away again. Held so it can be restarted rather than
    // stacked, and cleared on cleanup: a timeout that fires into a disposed owner is a panic with
    // no message and no line.
    let idle = StoredValue::new_local(None::<TimeoutHandle>);
    let wake = move || {
        idle.update_value(|handle| {
            if let Some(handle) = handle.take() {
                handle.clear();
            }
        });
        ctx.active.set(true);

        let handle = set_timeout_with_handle(
            move || {
                // Never over a paused video, never out from under a pointer mid-scrub, and
                // never out from under an open menu.
                if ctx.playing.get_untracked()
                    && !ctx.scrubbing.get_untracked()
                    && !ctx.menu_open.get_untracked()
                {
                    ctx.active.set(false);
                }
            },
            std::time::Duration::from_millis(idle_delay),
        );
        idle.set_value(handle.ok());
    };

    on_cleanup(move || {
        idle.update_value(|handle| {
            if let Some(handle) = handle.take() {
                handle.clear();
            }
        });
    });

    // Fullscreen is the document's to know, and it changes without us — Escape leaves it, and so
    // does the browser's own chrome.
    let listener = window_event_listener(ev::fullscreenchange, move |_| {
        let inside = document()
            .fullscreen_element()
            .zip(root_ref.get_untracked())
            .is_some_and(|(current, root)| current == *root.unchecked_ref::<Element>());
        set_if_changed(ctx.fullscreen, inside);
    });
    on_cleanup(move || listener.remove());

    // The active track's own `cuechange` is what keeps `cues` current — see
    // [`VideoPlayerContext::cues`]. Both the handler and the track it was put on are held: the
    // handler because a `Closure` that is dropped leaves the browser calling into freed memory,
    // and the track because it is the only way to take the handler off again when the reader
    // switches language.
    let cue_handler = StoredValue::new_local(None::<wasm_bindgen::closure::Closure<dyn FnMut()>>);
    let wired = StoredValue::new_local(None::<TextTrack>);

    let unwire = move || {
        wired.update_value(|track| {
            if let Some(track) = track.take() {
                track.set_oncuechange(None);
            }
        });
        cue_handler.set_value(None);
    };

    Effect::new(move |_| {
        // Tracked, so switching language rewires; `active_text_track` reads untracked.
        let _ = ctx.active_track.get();
        unwire();

        if let Some(track) = ctx.active_text_track() {
            let handler = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
                ctx.sync_cues();
            });
            track.set_oncuechange(Some(handler.as_ref().unchecked_ref()));
            wired.set_value(Some(track));
            cue_handler.set_value(Some(handler));
        }

        // The switch itself changes what is on screen, and `cuechange` will not fire until the
        // next cue boundary — which on a paused video is never.
        ctx.sync_cues();
    });

    on_cleanup(unwire);

    let on_keydown = move |e: ev::KeyboardEvent| {
        // A rail handles its own arrows and says so by preventing the default; without this the
        // root would seek again on top of it.
        if !mode.interactive() || e.default_prevented() {
            return;
        }

        match e.key().as_str() {
            " " | "k" => ctx.toggle(),
            "ArrowLeft" | "j" => ctx.seek_by(-SEEK_STEP),
            "ArrowRight" | "l" => ctx.seek_by(SEEK_STEP),
            "ArrowUp" => ctx.set_volume(ctx.volume.get_untracked() + VOLUME_STEP),
            "ArrowDown" => ctx.set_volume(ctx.volume.get_untracked() - VOLUME_STEP),
            "m" => ctx.toggle_muted(),
            "f" => ctx.toggle_fullscreen(),
            "Home" => ctx.seek_to(0.0),
            "End" => ctx.seek_to(ctx.duration.get_untracked()),
            _ => return,
        }

        // Only once something was actually done with it, or this would eat Tab and every shortcut
        // the browser owns.
        e.prevent_default();
        wake();
    };

    view! {
        <Provider value=ctx>
            <div
                node_ref=root_ref
                data-slot="video-player"
                data-mode=mode.as_str()
                data-state=move || if ctx.playing.get() { "playing" } else { "paused" }
                data-active=move || ctx.controls_visible().then_some("true")
                data-fullscreen=move || ctx.fullscreen.get().then_some("true")
                // A player is one tab stop and the keys work from anywhere inside it, which is
                // what makes it operable without tabbing through six buttons first.
                tabindex=move || mode.interactive().then_some("0")
                on:keydown=on_keydown
                on:pointermove=move |_| wake()
                on:pointerdown=move |_| wake()
                on:focusin=move |_| wake()
                on:pointerleave=move |_| {
                    if ctx.playing.get_untracked() && !ctx.scrubbing.get_untracked()
                        && !ctx.menu_open.get_untracked()
                    {
                        ctx.active.set(false);
                    }
                }
                class=class
            >
                {children()}
            </div>
        </Provider>
    }
}

#[component]
pub fn VideoPlayerVideoRoot(
    /// Omit and give the element `<source>` children instead, which is how one video is offered
    /// in several formats.
    #[prop(optional, into)]
    src: Signal<String>,
    #[prop(optional, into)] poster: Signal<String>,
    #[prop(default = false)] autoplay: bool,
    #[prop(default = false)] r#loop: bool,
    /// The element's own starting mute. Autoplay is refused unless the video is muted, so the two
    /// almost always go together.
    #[prop(default = false)]
    muted: bool,
    /// Inline rather than the phone's own fullscreen player, which is what a styled player wants
    /// and is not the platform default.
    #[prop(default = true)]
    playsinline: bool,
    #[prop(default = "metadata")] preload: &'static str,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_video_player();
    let sync = move |_| ctx.sync();

    view! {
        <video
            node_ref=ctx.video_ref
            data-slot="video-player-video"
            src=move || {
                let src = src.get();
                (!src.is_empty()).then_some(src)
            }
            poster=move || {
                let poster = poster.get();
                (!poster.is_empty()).then_some(poster)
            }
            autoplay=autoplay
            loop=r#loop
            muted=muted
            playsinline=playsinline
            preload=preload
            // Never the browser's own bar: it would sit on top of ours, and in fullscreen it is
            // the only one that shows.
            controls=false
            on:play=sync
            on:pause=sync
            on:ended=sync
            on:timeupdate=sync
            on:durationchange=sync
            on:loadedmetadata=move |_| {
                ctx.sync();
                ctx.sync_tracks();
            }
            on:volumechange=sync
            on:ratechange=sync
            on:seeked=sync
            on:progress=move |_| ctx.sync()
            on:emptied=sync
            on:waiting=move |_| {
                ctx.waiting.set(true);
                ctx.sync();
            }
            on:playing=move |_| {
                ctx.waiting.set(false);
                ctx.sync();
            }
            on:canplay=move |_| {
                ctx.waiting.set(false);
                ctx.sync();
            }
            class=class
        >
            {children.map(|children| children())}
        </video>
    }
}

/// The surface over the video that takes a press: what makes the frame itself a play button.
///
/// Its own element rather than a handler on the video, because a `<video>` with controls off
/// still swallows some presses of its own, and because the styled layer wants somewhere to put a
/// centred play badge.
#[component]
pub fn VideoPlayerSurfaceRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_video_player();
    let interactive = ctx.mode.interactive();

    view! {
        <div
            data-slot="video-player-surface"
            data-state=move || if ctx.playing.get() { "playing" } else { "paused" }
            // Nothing to press in the uncontrolled mode, so nothing to stand in the way either.
            style=(!interactive).then_some("pointer-events: none;")
            on:click=move |_| {
                if interactive {
                    ctx.toggle();
                }
            }
            class=class
        >
            {children.map(|children| children())}
        </div>
    }
}

#[component]
pub fn VideoPlayerControlsRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_video_player();

    view! {
        <div
            data-slot="video-player-controls"
            data-mode=ctx.mode.as_str()
            data-active=move || ctx.controls_visible().then_some("true")
            data-state=move || if ctx.playing.get() { "playing" } else { "paused" }
            class=class
        >
            {children()}
        </div>
    }
}

/// The cues on screen, drawn as ordinary elements rather than by the browser.
///
/// See [`VideoPlayerContext::cues`] for why the player draws them at all. Renders nothing when
/// there is no track showing or the moment between two cues, so the styled layer can put a
/// background on it without a bar appearing over silence.
///
/// Writes `data-active` alongside the controls' own, which is what lets the caption sit above a
/// bar that is up and drop back down when it goes away. Not interactive and not announced: a cue
/// is a transcription of audio the reader can already hear, and a live region would read every
/// line of dialogue over whatever else the reader is doing.
///
/// Each cue is its own `[data-slot=video-player-cue]`, because a cue is a line and two active at
/// once are two lines. A caller who wants different markup than a span per line reads
/// [`VideoPlayerContext::cues`] and writes their own; this is the common shape, not the only one.
#[component]
pub fn VideoPlayerCaptionsRoot(
    #[prop(optional, into)] class: Signal<String>,
    /// Worn by each cue, since the cues are this part's own markup rather than the caller's.
    #[prop(optional, into)]
    cue_class: Signal<String>,
) -> impl IntoView {
    let ctx = use_video_player();

    view! {
        <Show when=move || !ctx.cues.with(Vec::is_empty)>
            <div
                data-slot="video-player-captions-display"
                data-active=move || ctx.controls_visible().then_some("true")
                data-size=move || ctx.caption_size.get().as_str()
                aria-hidden="true"
                class=class
            >
                {move || {
                    ctx.cues
                        .get()
                        .into_iter()
                        .map(|cue| {
                            view! {
                                <span data-slot="video-player-cue" class=cue_class>
                                    {cue}
                                </span>
                            }
                        })
                        .collect_view()
                }}
            </div>
        </Show>
    }
}

#[component]
pub fn VideoPlayerPlayRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let ctx = use_video_player();

    view! {
        <button
            type="button"
            data-slot="video-player-play"
            data-state=move || if ctx.playing.get() { "playing" } else { "paused" }
            aria-label=move || if ctx.playing.get() { "Pause" } else { "Play" }
            disabled=move || disabled.get()
            on:click=move |_| {
                if !disabled.get_untracked() {
                    ctx.toggle();
                }
            }
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn VideoPlayerSkipRoot(
    step: VideoPlayerStep,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let ctx = use_video_player();
    let unavailable = Signal::derive(move || disabled.get() || !ctx.can_skip());

    view! {
        <button
            type="button"
            data-slot=match step {
                VideoPlayerStep::Previous => "video-player-previous",
                VideoPlayerStep::Next => "video-player-next",
            }
            aria-label=match step {
                VideoPlayerStep::Previous => "Previous",
                VideoPlayerStep::Next => "Next",
            }
            disabled=move || unavailable.get()
            on:click=move |_| {
                if !unavailable.get_untracked() {
                    ctx.skip(step);
                }
            }
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn VideoPlayerMuteRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_video_player();

    view! {
        <button
            type="button"
            data-slot="video-player-mute"
            data-state=move || if ctx.muted.get() { "muted" } else { "unmuted" }
            aria-label=move || if ctx.muted.get() { "Unmute" } else { "Mute" }
            aria-pressed=move || ctx.muted.get().then_some("true")
            on:click=move |_| ctx.toggle_muted()
            class=class
        >
            {children()}
        </button>
    }
}

/// The scrubber and the volume slider, which are the same component twice.
#[component]
pub fn VideoPlayerRailRoot(
    rail: VideoPlayerRail,
    #[prop(optional, into)] class: Signal<String>,
    /// The thumb, handed its position along the rail as a percentage.
    #[prop(default = None, into)]
    thumb: Option<Callback<f64, AnyView>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_video_player();
    let rail_ref = AnyNodeRef::new();

    let set = move |fraction: f64| ctx.set_fraction(rail, fraction);
    let fraction = Signal::derive(move || ctx.fraction_of(rail));

    // A seek bar with nothing to seek along is not a control yet: before the metadata arrives
    // there is no length, and dragging would write a time nothing can be at.
    let unavailable = Signal::derive(move || match rail {
        VideoPlayerRail::Seek => ctx.duration.get() <= 0.0,
        VideoPlayerRail::Volume => false,
    });

    let drag = use_drag(move |point: DragPoint| set(point.fraction_x))
        .against(rail_ref)
        .on_start(move |point: DragPoint| {
            ctx.scrubbing.set(true);
            set(point.fraction_x);
        })
        .on_end(move |_| ctx.scrubbing.set(false));

    let on_keydown = move |e: ev::KeyboardEvent| {
        if unavailable.get_untracked() {
            return;
        }

        let duration = ctx.duration.get_untracked();
        // In whole seconds on the scrubber and in fractions on the volume, because a keypress on
        // a time bar means "five seconds" rather than "five percent of however long this is".
        let step = match rail {
            VideoPlayerRail::Seek if duration > 0.0 => SEEK_STEP / duration,
            VideoPlayerRail::Seek => return,
            VideoPlayerRail::Volume => VOLUME_STEP,
        };

        let next = match e.key().as_str() {
            "ArrowLeft" | "ArrowDown" => fraction.get_untracked() - step,
            "ArrowRight" | "ArrowUp" => fraction.get_untracked() + step,
            "Home" => 0.0,
            "End" => 1.0,
            _ => return,
        };

        // Also what tells the root not to act on the same key.
        e.prevent_default();
        set(next);
    };

    let value_now = move || match rail {
        VideoPlayerRail::Seek => ctx.current_time.get().round(),
        VideoPlayerRail::Volume => (fraction.get() * 100.0).round(),
    };

    view! {
        <div
            node_ref=rail_ref
            data-slot="video-player-rail"
            data-rail=rail.as_str()
            data-dragging=move || drag.active.get().then_some("true")
            data-disabled=move || unavailable.get().then_some("true")
            role="slider"
            aria-label=match rail {
                VideoPlayerRail::Seek => "Seek",
                VideoPlayerRail::Volume => "Volume",
            }
            aria-valuemin="0"
            aria-valuemax=move || match rail {
                VideoPlayerRail::Seek => ctx.duration.get().round(),
                VideoPlayerRail::Volume => 100.0,
            }
            aria-valuenow=value_now
            aria-valuetext=move || match rail {
                VideoPlayerRail::Seek => {
                    format!(
                        "{} of {}",
                        format_timestamp(ctx.current_time.get()),
                        format_timestamp(ctx.duration.get()),
                    )
                }
                VideoPlayerRail::Volume => format!("{}% volume", (fraction.get() * 100.0).round()),
            }
            aria-disabled=move || unavailable.get().then_some("true")
            tabindex="0"
            on:pointerdown=move |e: ev::PointerEvent| {
                if !unavailable.get_untracked() {
                    drag.start(&e);
                }
            }
            on:keydown=on_keydown
            class=class
        >
            {children.map(|children| children())}
            {thumb.map(move |thumb| view! { {move || thumb.run(fraction.get() * 100.0)} })}
        </div>
    }
}

/// The filled part of a rail: how far played, or how loud.
#[component]
pub fn VideoPlayerRailFillRoot(
    rail: VideoPlayerRail,
    /// Draws the buffered stretch rather than the played one. Only means anything on the seek
    /// rail, where it is the browser's answer to "how much of this could I watch right now".
    #[prop(default = false)]
    buffered: bool,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    let ctx = use_video_player();

    let fraction = Signal::derive(move || match (rail, buffered) {
        (VideoPlayerRail::Seek, true) => ctx.buffered_progress(),
        _ => ctx.fraction_of(rail),
    });

    view! {
        <div
            data-slot=if buffered { "video-player-rail-buffered" } else { "video-player-rail-fill" }
            data-rail=rail.as_str()
            // Geometry, so it is written here rather than left to a class that cannot know it.
            style=move || format!("width: {}%;", fraction.get() * 100.0)
            class=class
        />
    }
}

/// The elapsed and total times, as `1:02 / 3:45`.
#[component]
pub fn VideoPlayerTimeRoot(
    /// Show only the elapsed time, for a bar with no room for both.
    #[prop(default = false)]
    elapsed_only: bool,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    let ctx = use_video_player();

    view! {
        <div
            data-slot="video-player-time"
            // Announced as one thing when it changes, rather than a digit at a time.
            aria-live="off"
            class=class
        >
            {move || {
                let elapsed = format_timestamp(ctx.current_time.get());
                let duration = ctx.duration.get();
                if elapsed_only || duration <= 0.0 {
                    elapsed
                } else {
                    format!("{elapsed} / {}", format_timestamp(duration))
                }
            }}
        </div>
    }
}

#[component]
pub fn VideoPlayerFullscreenRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_video_player();

    view! {
        <button
            type="button"
            data-slot="video-player-fullscreen"
            data-state=move || if ctx.fullscreen.get() { "fullscreen" } else { "inline" }
            aria-label=move || {
                if ctx.fullscreen.get() { "Exit fullscreen" } else { "Enter fullscreen" }
            }
            aria-pressed=move || ctx.fullscreen.get().then_some("true")
            on:click=move |_| ctx.toggle_fullscreen()
            class=class
        >
            {children()}
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::{cue_text, format_timestamp};

    #[test]
    fn timestamps_read_the_way_a_player_writes_them() {
        assert_eq!(format_timestamp(0.0), "0:00");
        assert_eq!(format_timestamp(7.0), "0:07");
        assert_eq!(format_timestamp(754.0), "12:34");
        assert_eq!(format_timestamp(3723.0), "1:02:03");
    }

    #[test]
    fn hours_appear_only_when_there_are_any() {
        assert_eq!(format_timestamp(3599.0), "59:59");
        assert_eq!(format_timestamp(3600.0), "1:00:00");
    }

    #[test]
    fn a_part_second_is_the_second_it_is_in() {
        // Rounding up would show 1:00 for a video that has not finished its 59th second.
        assert_eq!(format_timestamp(59.9), "0:59");
        assert_eq!(format_timestamp(0.4), "0:00");
    }

    #[test]
    fn a_length_the_player_does_not_know_says_so() {
        assert_eq!(format_timestamp(f64::NAN), "--:--");
        assert_eq!(format_timestamp(f64::INFINITY), "--:--");
        assert_eq!(format_timestamp(-1.0), "--:--");
    }

    #[test]
    fn cue_markup_is_dropped_and_its_contents_kept() {
        assert_eq!(cue_text("plain line"), "plain line");
        assert_eq!(cue_text("<i>slanted</i>"), "slanted");
        assert_eq!(cue_text("<v Roger>Hello there"), "Hello there");
        assert_eq!(
            cue_text("<c.loud>LOUD</c> and <b>bold</b>"),
            "LOUD and bold"
        );
        // The karaoke timestamps, which sit mid-sentence rather than around anything.
        assert_eq!(
            cue_text("one<00:00:01.000>two<00:00:02.000>three"),
            "onetwothree"
        );
    }

    #[test]
    fn an_unterminated_tag_takes_the_rest_of_the_line() {
        // A `<` is always a tag in WebVTT, so there is nothing here to put back.
        assert_eq!(cue_text("before <i still open"), "before ");
        assert_eq!(cue_text("<"), "");
    }

    #[test]
    fn the_six_character_references_are_decoded() {
        assert_eq!(cue_text("a &amp; b"), "a & b");
        assert_eq!(cue_text("&lt;track&gt; child"), "<track> child");
        assert_eq!(cue_text("wide&nbsp;gap"), "wide\u{a0}gap");
        // Decoded once: the `<` that `&lt;` produces is not then read as a tag.
        assert_eq!(
            cue_text("&lt;i&gt;not italic&lt;/i&gt;"),
            "<i>not italic</i>"
        );
        // Anything else is a cue that happens to contain an ampersand.
        assert_eq!(cue_text("Tom &hellip; Jerry"), "Tom &hellip; Jerry");
        assert_eq!(cue_text("R&D"), "R&D");
    }
}
