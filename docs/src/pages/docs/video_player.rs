use crate::{
    app::href,
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::{
    components::{
        dropdown_menu::{DropdownMenuItem, DropdownMenuLabel},
        toggle_group::{ToggleGroup, ToggleGroupItem},
        video_player::{
            VideoPlayer, VideoPlayerCaptionItems, VideoPlayerFullscreenButton, VideoPlayerMenu,
            VideoPlayerPlayButton, VideoPlayerSeek, VideoPlayerSpeedItems, VideoPlayerTime,
            VideoPlayerVolumeControl,
        },
    },
    icons::check::Check,
    primitives::video_player::{VideoPlayerMode, VideoPlayerStep},
};

const DEFAULT: &str =
    r#"<VideoPlayer src="/tears-of-steel.mp4" poster="/tears-of-steel.jpg" class="max-w-xl" />"#;

const MODES: &str = r#"// The mode decides which controls the bar draws — and, for the last two,
// whether the reader may drive the player at all.
<VideoPlayer src="/tears-of-steel.mp4" mode=VideoPlayerMode::Reduced />
<VideoPlayer src="/tears-of-steel.mp4" mode=VideoPlayerMode::Limited />
<VideoPlayer src="/tears-of-steel.mp4" mode=VideoPlayerMode::None />
<VideoPlayer src="/tears-of-steel.mp4" mode=VideoPlayerMode::Uncontrolled />"#;

const PLAYLIST: &str = r#"// The player does not own the playlist. Previous and Next report a step and
// the caller moves their own list on; without `on_skip` both controls disable.
let track = RwSignal::new(0usize);

view! {
    <VideoPlayer
        src=Signal::derive(move || CLIPS[track.get()].to_string())
        on_skip=Callback::new(move |step| {
            track
                .update(|track| {
                    *track = match step {
                        VideoPlayerStep::Previous => track.saturating_sub(1),
                        VideoPlayerStep::Next => (*track + 1).min(CLIPS.len() - 1),
                    };
                })
        })
    />
}"#;

const FORMATS: &str = r#"// `<video>`'s own children are how one video is offered in several formats:
// the browser takes the first it can play.
<VideoPlayer poster="/tears-of-steel.jpg">
    <source src="/tears-of-steel.webm" type="video/webm" />
    <source src="/tears-of-steel.mp4" type="video/mp4" />
</VideoPlayer>"#;

const SUBTITLES: &str = r#"// A <track> is a child of the <video>, so it is a child of the player.
// Nothing else is needed: the player reads the track list off the element,
// and draws the cues itself so they can be positioned and sized.
<VideoPlayer src="/tears-of-steel.mp4">
    <track
        kind="subtitles"
        src="/tears-of-steel.en.vtt"
        srclang="en"
        label="English"
        default=true
    />
    <track kind="subtitles" src="/tears-of-steel.pt.vtt" srclang="pt" label="Português" />
</VideoPlayer>"#;

const MENU: &str = r#"// The menu takes whatever you put in it. These two groups are provided,
// but a caller with quality levels or an audio-track picker writes their own.
<VideoPlayerMenu>
    <VideoPlayerSpeedItems />
    <VideoPlayerCaptionItems />
</VideoPlayerMenu>

// `submenu` folds a group into one row that opens beside the menu, which is
// what the default player does: fourteen rows become three.
<VideoPlayerMenu>
    <VideoPlayerSpeedItems submenu=true />
    <VideoPlayerCaptionItems submenu=true />
    <VideoPlayerCaptionSizeItems submenu=true />
</VideoPlayerMenu>"#;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "VideoPlayer",
        description: "The whole player: the video, the surface that makes the frame a play button, and whichever controls the mode calls for. Compose the parts yourself when the bar wants to be something else.",
        props: &[
            Prop {
                name: "src",
                ty: "Signal<String>",
                default: "\"\"",
                description: "The video. Leave it empty and pass `<source>` children instead to offer several formats.",
            },
            Prop {
                name: "poster",
                ty: "Signal<String>",
                default: "\"\"",
                description: "The still shown before playback starts.",
            },
            Prop {
                name: "mode",
                ty: "VideoPlayerMode",
                default: "Full",
                description: "How much of the player the reader is given. A value rather than a signal: changing it remounts the player.",
            },
            Prop {
                name: "autoplay",
                ty: "bool",
                default: "false",
                description: "Browsers refuse this unless the video is also muted, and the player reports what actually happened rather than what was asked for.",
            },
            Prop {
                name: "loop",
                ty: "bool",
                default: "false",
                description: "Written `r#loop=true`, `loop` being a Rust keyword.",
            },
            Prop {
                name: "muted",
                ty: "bool",
                default: "false",
                description: "The element's starting mute. The mute control writes to the element from then on.",
            },
            Prop {
                name: "on_skip",
                ty: "Callback<VideoPlayerStep>",
                default: "None",
                description: "Where Previous and Next report to. Without it there is no playlist and both controls disable themselves.",
            },
            Prop {
                name: "controls",
                ty: "Callback<(), AnyView>",
                default: "None",
                description: "Your own control bar, in place of the one the mode would have drawn. The shell — the video, the surface, the keys, fullscreen — stays, so replacing the bar does not mean rebuilding the player.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the player's classes. It is `aspect-video` by default, which a class can override.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Placed inside the `<video>` itself: `<source>` and `<track>` elements.",
            },
        ],
    },
    ApiEntry {
        name: "VideoPlayerVideo",
        description: "The `<video>` element, and its own root — so `attr:` and any event land on the control rather than on a wrapper. It wires every media event to one `sync`, which reads the whole of the player's state back off the element.",
        props: &[
            Prop {
                name: "src",
                ty: "Signal<String>",
                default: "\"\"",
                description: "The video.",
            },
            Prop {
                name: "poster",
                ty: "Signal<String>",
                default: "\"\"",
                description: "The still shown before playback starts.",
            },
            Prop {
                name: "autoplay",
                ty: "bool",
                default: "false",
                description: "Passed to the element.",
            },
            Prop {
                name: "loop",
                ty: "bool",
                default: "false",
                description: "Passed to the element, written `r#loop=true`.",
            },
            Prop {
                name: "muted",
                ty: "bool",
                default: "false",
                description: "The element's starting mute.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the video's classes — `object-cover` where the frame should fill rather than fit.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The `<source>` and `<track>` elements.",
            },
        ],
    },
    ApiEntry {
        name: "VideoPlayerSurface",
        description: "The layer over the video that takes a press, which is what makes the frame itself a play button. Inert in the uncontrolled mode, so it never stands in the way of something that cannot be pressed.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the surface's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Usually a `VideoPlayerBadge`.",
            },
        ],
    },
    ApiEntry {
        name: "VideoPlayerBadge",
        description: "The centred play badge, shown while the video is paused. It reads the surface's own `data-state`, so it needs nothing passed to it.",
        props: &[Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Merged over the badge's classes.",
        }],
    },
    ApiEntry {
        name: "VideoPlayerControls",
        description: "The bar. It fades out a couple of seconds after the reader last did anything — but never over a paused video, and never mid-scrub. Hidden it is `pointer-events-none`, so nothing in it can be clicked or tabbed to while it is not there, and the pointer goes with it: the player wears `cursor-none` until `data-active` comes back, so a still cursor does not sit over the picture. Only where there is a bar to lose — the modes that draw none keep their cursor.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the bar's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The controls.",
            },
        ],
    },
    ApiEntry {
        name: "VideoPlayerSeek",
        description: "The time bar: a `role=\"slider\"` that drags, takes arrows, Home and End, and shows what the browser has buffered behind what has played. It disables itself until the metadata says how long the video is, there being nothing to scrub along before that.",
        props: &[Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Merged over the rail's classes.",
        }],
    },
    ApiEntry {
        name: "VideoPlayerVolume",
        description: "The volume rail, the same component as the scrubber with a different notion of what a fraction along means. Muted reads as silent rather than as wherever the slider was left, because that is what the reader is hearing.",
        props: &[Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Merged over the rail's classes.",
        }],
    },
    ApiEntry {
        name: "VideoPlayerPlayButton",
        description: "Play and pause, which is one button because it is one state. It never assumes: pressing it calls `play()` and the icon changes when the element says it is playing, so a refused autoplay does not leave a pause icon over a still frame.",
        props: &[Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Merged over the button's classes.",
        }],
    },
    ApiEntry {
        name: "VideoPlayerSkipButton",
        description: "Previous or Next. Disabled whenever the player has no `on_skip`, since a control that does nothing should say so rather than swallow the press.",
        props: &[
            Prop {
                name: "step",
                ty: "VideoPlayerStep",
                default: "",
                description: "`Previous` or `Next`, which decides both the icon and what is reported.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the button's classes.",
            },
        ],
    },
    ApiEntry {
        name: "VideoPlayerMuteButton",
        description: "Mute and unmute. Coming off mute at zero volume brings the volume back somewhere audible, since silence with the control showing sound is a control that appears broken.",
        props: &[Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Merged over the button's classes.",
        }],
    },
    ApiEntry {
        name: "VideoPlayerTime",
        description: "The elapsed and total times. A length the player does not know yet — before the metadata lands, or a live stream, whose duration is infinite — reads `--:--` rather than a confident `0:00`.",
        props: &[
            Prop {
                name: "elapsed_only",
                ty: "bool",
                default: "false",
                description: "Show only the elapsed time, for a bar with no room for both.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the timestamp's classes.",
            },
        ],
    },
    ApiEntry {
        name: "VideoPlayerVolumeControl",
        description: "The mute button with the volume rail beside it, collapsed to just the icon until the group is hovered or something inside it takes focus. Focus counts as well as hover, or the rail would be a control a keyboard could reach and never see.",
        props: &[
            Prop {
                name: "collapsible",
                ty: "bool",
                default: "true",
                description: "Set false to leave the rail out permanently, for a bar with room to spare.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the group's classes.",
            },
        ],
    },
    ApiEntry {
        name: "VideoPlayerMenu",
        description: "A dropdown in the control bar, holding whatever you put in it. Not portalled, on purpose: a menu portalled to `document.body` disappears when the player goes fullscreen, since only the fullscreen element's own subtree is drawn. While it is open the bar refuses to fade out from under it.",
        props: &[
            Prop {
                name: "label",
                ty: "String",
                default: "\"Settings\"",
                description: "The trigger's `aria-label`, since the trigger is an icon.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the trigger's classes.",
            },
            Prop {
                name: "children",
                ty: "ChildrenFn",
                default: "",
                description: "The menu's contents — `DropdownMenuItem` and friends, the two provided groups, or your own.",
            },
        ],
    },
    ApiEntry {
        name: "VideoPlayerSpeedItems",
        description: "A labelled group of playback speeds, ticking whichever is in force. A group rather than a whole menu, so it can sit beside a caller's own items instead of replacing them, and `submenu` folds it into one row when the menu is getting long.",
        props: &[Prop {
            name: "submenu",
            ty: "bool",
            default: "false",
            description: "Fold the group into a single row that opens its items beside the menu, instead of listing them inline under a label. The default player uses this; a caller assembling a short menu of their own usually does not.",
        }],
    },
    ApiEntry {
        name: "VideoPlayerCaptionItems",
        description: "A labelled group of the subtitle tracks the element is carrying, plus Off. Renders nothing at all when there are no tracks, so it costs nothing to leave in a menu for a video that has none.",
        props: &[Prop {
            name: "submenu",
            ty: "bool",
            default: "false",
            description: "Fold the group into a single row that opens its items beside the menu, instead of listing them inline under a label. The default player uses this; a caller assembling a short menu of their own usually does not.",
        }],
    },
    ApiEntry {
        name: "VideoPlayerCaptionSizeItems",
        description: "A labelled group of caption sizes, ticking the one in force. Sizes are written against the player's own width rather than in pixels, so one choice holds from a thumbnail to fullscreen. Renders nothing while no track is showing, since there would be nothing to size.",
        props: &[Prop {
            name: "submenu",
            ty: "bool",
            default: "false",
            description: "Fold the group into a single row that opens its items beside the menu, instead of listing them inline under a label. The default player uses this; a caller assembling a short menu of their own usually does not.",
        }],
    },
    ApiEntry {
        name: "VideoPlayerCaptions",
        description: "The cues on screen, drawn as ordinary elements. Included in the default player, so it is only worth naming when composing the parts by hand. It steps above the control bar while the bar is up and drops back when it goes, and renders nothing between two cues.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the box holding the cues.",
            },
            Prop {
                name: "cue_class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over each cue in turn. A cue is a line, and two active at once are two elements.",
            },
        ],
    },
    ApiEntry {
        name: "VideoPlayerCaptionsButton",
        description: "Toggles the first subtitle track on and off, with an underline marking it on — the one-press control for the common case where there is only one. Renders nothing when the video carries no tracks.",
        props: &[Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Merged over the button's classes.",
        }],
    },
    ApiEntry {
        name: "VideoPlayerFullscreenButton",
        description: "Fullscreen, taken on the whole player rather than on the `<video>` — a video element in fullscreen draws the browser's own controls over these. What the document is actually doing is read back from `fullscreenchange`, so leaving with Escape is not missed.",
        props: &[Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Merged over the button's classes.",
        }],
    },
];

const MODE_CHOICES: &[(&str, &str, VideoPlayerMode)] = &[
    ("full", "Full", VideoPlayerMode::Full),
    ("reduced", "Reduced", VideoPlayerMode::Reduced),
    ("limited", "Limited", VideoPlayerMode::Limited),
    ("none", "None", VideoPlayerMode::None),
    (
        "uncontrolled",
        "Uncontrolled",
        VideoPlayerMode::Uncontrolled,
    ),
];

fn mode_note(mode: VideoPlayerMode) -> &'static str {
    match mode {
        VideoPlayerMode::Full => {
            "Everything: play, the skip pair, volume, the timestamp and fullscreen."
        }
        VideoPlayerMode::Reduced => {
            "Play, the skip pair, volume and the time bar. No timestamp, no fullscreen."
        }
        VideoPlayerMode::Limited => "Play and the time bar, and nothing else.",
        VideoPlayerMode::None => {
            "No controls at all — the frame itself is the play button, and the keys still work."
        }
        VideoPlayerMode::Uncontrolled => {
            "No controls and no way in: the surface is inert and the player is not a tab stop."
        }
    }
}

#[component]
pub fn Page() -> impl IntoView {
    let source = href("/tears-of-steel.mp4");
    let chosen = RwSignal::new(vec!["reduced".to_string()]);

    let mode = Signal::derive(move || {
        chosen.with(|chosen| {
            chosen
                .first()
                .and_then(|name| MODE_CHOICES.iter().find(|(value, _, _)| value == name))
                .map(|&(_, _, mode)| mode)
                .unwrap_or_default()
        })
    });

    let track = RwSignal::new(0usize);
    let quality = RwSignal::new("Auto".to_string());
    let playlist_source = source.clone();
    let default_source = source.clone();
    let modes_source = source.clone();
    let formats_source = source.clone();
    let poster_source = href("/tears-of-steel.jpg");
    let default_poster = poster_source.clone();
    let subtitles_source = source.clone();
    let subtitles_poster = poster_source.clone();
    let menu_source = source.clone();
    let menu_poster = poster_source.clone();
    let en_track = href("/tears-of-steel.en.vtt");
    let pt_track = href("/tears-of-steel.pt.vtt");

    view! {
        <DocLayout
            title="Video Player"
            description="The video element, and the controls that drive it."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="The element is the state and the player mirrors it: nothing here decides that the video is playing, the element says so and every media event reads the whole lot back. Press it, or use the keys — space, the arrows, m and f — from anywhere inside. The paler stretch behind the played part of the bar is what the browser has actually buffered, read out of the element's own time ranges rather than guessed from how long the file has been downloading."
                    code=DEFAULT
                >
                    <div class="flex flex-col items-center gap-3">
                        <VideoPlayer src=default_source poster=default_poster class="max-w-xl" />
                        <p class="text-xs text-muted-foreground">
                            "\"Tears of Steel\" — (CC) Blender Foundation, "
                            <a
                                href="https://mango.blender.org"
                                target="_blank"
                                rel="noreferrer"
                                class="underline underline-offset-2 hover:text-foreground"
                            >
                                mango.blender.org
                            </a>
                            ", used under CC BY 3.0. A 40-second excerpt, and the clip every demo
                            on this page runs on."
                        </p>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Modes"
                    description="Five of them, and the last two are the reason this is a mode rather than a set of flags: one is a video you can press, the other is a video you cannot. Switching remounts the player, since the mode is a value rather than a signal."
                    code=MODES
                >
                    <div class="flex flex-col items-center gap-4">
                        <ToggleGroup value=chosen>
                            {MODE_CHOICES
                                .iter()
                                .map(|&(value, label, _)| {
                                    view! { <ToggleGroupItem value=value>{label}</ToggleGroupItem> }
                                })
                                .collect_view()}
                        </ToggleGroup>

                        <p class="text-center text-sm text-muted-foreground">
                            {move || mode_note(mode.get())}
                        </p>

                        {move || {
                            view! {
                                <VideoPlayer
                                    src=modes_source.clone()
                                    mode=mode.get()
                                    class="max-w-xl"
                                />
                            }
                        }}
                    </div>
                </DemoSection>

                <DemoSection
                    title="A playlist it does not own"
                    description="Previous and Next report a step and the caller moves their own list on, the same stance as the kanban board reporting a move rather than applying it. Without an `on_skip` there is no playlist, and both controls disable themselves rather than swallowing the press."
                    code=PLAYLIST
                >
                    <div class="flex flex-col items-center gap-3">
                        <VideoPlayer
                            src=playlist_source
                            mode=VideoPlayerMode::Reduced
                            on_skip=Callback::new(move |step| {
                                track
                                    .update(|track| {
                                        *track = match step {
                                            VideoPlayerStep::Previous => track.saturating_sub(1),
                                            VideoPlayerStep::Next => (*track + 1).min(3),
                                        };
                                    })
                            })
                            class="max-w-xl"
                        />
                        <p class="text-sm text-muted-foreground">
                            "Clip "
                            <span class="font-mono text-foreground">{move || track.get() + 1}</span>
                            " of 4 — the same file four times, since what moves is the caller's list."
                        </p>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Several formats"
                    description="`<video>`'s own children are how one video is offered in more than one encoding, and they are what the player's children become — the browser takes the first source it can play. What that does not stretch to is adaptive streaming: this is a plain `<video>`, so HLS and DASH need hls.js or dash.js layered on, and only Safari plays an `.m3u8` on its own."
                    code=FORMATS
                >
                    <div class="flex justify-center">
                        <VideoPlayer mode=VideoPlayerMode::Limited class="max-w-xl">
                            <source src=formats_source type="video/mp4" />
                        </VideoPlayer>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Subtitles"
                    description="A `<track>` is a child of the `<video>`, and the player's children are the video's children — so subtitles work with nothing added. What the player does on top is read the track list back off the element, which is why a menu can offer them: a list passed in beside the `<track>` elements would be a second copy to keep in step. **The cues are drawn by the player, not by the browser**: the track is set to `hidden`, which keeps the cue list current while stopping the browser painting it, and each cue becomes an ordinary element. The browser's own captions sit at the bottom of the frame — exactly where a control bar goes — and there is no portable way to move them: Chrome offers `::-webkit-media-text-track-container` and honours only `transform` on it, and Firefox has no such pseudo-element at all. Drawn as elements they step above the bar when it is up, and are sized against the player's own width so fullscreen makes them bigger."
                    code=SUBTITLES
                >
                    <div class="flex justify-center">
                        <VideoPlayer src=subtitles_source poster=subtitles_poster class="max-w-xl">
                            <track
                                kind="subtitles"
                                src=en_track
                                srclang="en"
                                label="English"
                                default=true
                            />
                            <track kind="subtitles" src=pt_track srclang="pt" label="Português" />
                        </VideoPlayer>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Menus"
                    description="`VideoPlayerMenu` is a dropdown in the control bar, and it takes whatever you put in it — speed and subtitles are provided because every player has them, but quality levels or an audio-track picker are the same shape. It is not portalled, deliberately: a menu portalled to `document.body` vanishes the moment the player goes fullscreen, since only the fullscreen element's own subtree is drawn. While one is open the bar refuses to fade, which the idle timer has to be told about."
                    code=MENU
                >
                    <div class="flex justify-center">
                        <VideoPlayer
                            src=menu_source
                            poster=menu_poster
                            class="max-w-xl"
                            controls=Callback::new(move |_| {
                                view! {
                                    <VideoPlayerSeek />
                                    <div class="flex items-center gap-1">
                                        <VideoPlayerPlayButton />
                                        <VideoPlayerVolumeControl />
                                        <VideoPlayerTime />
                                        <div class="flex-1" />
                                        <VideoPlayerMenu>
                                            <DropdownMenuLabel>"Quality"</DropdownMenuLabel>
                                            {["1080p", "720p", "480p", "Auto"]
                                                .into_iter()
                                                .map(|level| {
                                                    view! {
                                                        <DropdownMenuItem
                                                            class="justify-between gap-4 focus:bg-white/15 focus:text-white hover:bg-white/15 hover:text-white"
                                                            on_click=Callback::new(move |_| {
                                                                quality.set(level.to_string())
                                                            })
                                                        >
                                                            <span>{level}</span>
                                                            <Check class=move || {
                                                                if quality.get() == level {
                                                                    "size-3.5 opacity-100"
                                                                } else {
                                                                    "size-3.5 opacity-0"
                                                                }
                                                            } />
                                                        </DropdownMenuItem>
                                                    }
                                                })
                                                .collect_view()}
                                            <VideoPlayerSpeedItems />
                                            <VideoPlayerCaptionItems />
                                        </VideoPlayerMenu>
                                        <VideoPlayerFullscreenButton />
                                    </div>
                                }
                                    .into_any()
                            })
                        />
                    </div>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
