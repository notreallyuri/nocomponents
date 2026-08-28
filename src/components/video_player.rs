use crate::utils::types::{Align, Side};
use crate::{
    cn,
    components::dropdown_menu::{
        DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuLabel,
        DropdownMenuSeparator, DropdownMenuSub, DropdownMenuSubContent, DropdownMenuSubTrigger,
    },
    icons::{
        check::Check,
        media::{
            Captions, Maximize, Minimize, Pause, Play, Settings, SkipBack, SkipForward, Volume,
            VolumeOff,
        },
    },
    primitives::{
        dropdown_menu::{DropdownMenuTriggerRoot, use_dropdown},
        video_player::{
            VideoPlayerCaptionSize, VideoPlayerCaptionsRoot, VideoPlayerControlsRoot,
            VideoPlayerFullscreenRoot, VideoPlayerMode, VideoPlayerMuteRoot, VideoPlayerPlayRoot,
            VideoPlayerRail, VideoPlayerRailFillRoot, VideoPlayerRailRoot, VideoPlayerRoot,
            VideoPlayerSkipRoot, VideoPlayerStep, VideoPlayerSurfaceRoot, VideoPlayerTimeRoot,
            VideoPlayerVideoRoot, use_video_player,
        },
    },
};
use leptos::{either::Either, prelude::*};

const CONTROL: &str = "inline-flex size-8 shrink-0 items-center justify-center rounded-md text-white outline-none transition-colors hover:bg-white/20 focus-visible:ring-2 focus-visible:ring-white/70 disabled:pointer-events-none disabled:opacity-40";
const MENU_ITEM: &str =
    "justify-between gap-4 focus:bg-white/15 focus:text-white hover:bg-white/15 hover:text-white";
const SUBMENU_TRIGGER: &str = "focus:bg-white/15 focus:text-white hover:bg-white/15 hover:text-white data-[state=open]:bg-white/15 data-[state=open]:text-white";
const SUBMENU_CONTENT: &str = "min-w-40 border-white/15 bg-black/85 text-white backdrop-blur";

#[component]
pub fn VideoPlayerVideo(
    #[prop(optional, into)] src: Signal<String>,
    #[prop(optional, into)] poster: Signal<String>,
    #[prop(default = false)] autoplay: bool,
    #[prop(default = false)] r#loop: bool,
    #[prop(default = false)] muted: bool,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <VideoPlayerVideoRoot
            src=src
            poster=poster
            autoplay=autoplay
            r#loop=r#loop
            muted=muted
            class=move || { cn!("size-full object-contain", class.get()) }
        >
            {children.map(|children| children())}
        </VideoPlayerVideoRoot>
    }
}

#[component]
pub fn VideoPlayerSurface(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <VideoPlayerSurfaceRoot class=move || {
            cn!("group/surface absolute inset-0 flex items-center justify-center", class.get())
        }>{children.map(|children| children())}</VideoPlayerSurfaceRoot>
    }
}

#[component]
pub fn VideoPlayerBadge(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <span class=move || {
            cn!(
                "pointer-events-none flex size-16 items-center justify-center rounded-full bg-black/55 text-white opacity-0 transition-opacity group-data-[state=paused]/surface:opacity-100",
                class.get()
            )
        }>
            <Play class="size-7" />
        </span>
    }
}

pub fn caption_size_class(size: VideoPlayerCaptionSize) -> &'static str {
    match size {
        VideoPlayerCaptionSize::Small => "text-[clamp(0.625rem,2.2cqw,1.875rem)]",
        VideoPlayerCaptionSize::Medium => "text-[clamp(0.75rem,3.0cqw,2.5rem)]",
        VideoPlayerCaptionSize::Large => "text-[clamp(0.9rem,3.8cqw,3.25rem)]",
        VideoPlayerCaptionSize::Huge => "text-[clamp(1.05rem,4.9cqw,4.25rem)]",
    }
}

#[component]
pub fn VideoPlayerCaptions(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] cue_class: Signal<String>,
) -> impl IntoView {
    let ctx = use_video_player();

    view! {
        <VideoPlayerCaptionsRoot
            class=move || {
                cn!(
                    "@container/captions pointer-events-none absolute inset-x-0 bottom-3 z-10 flex flex-col items-center gap-[0.35em] px-4 text-center transition-transform duration-200 data-[active=true]:-translate-y-14",
                    class.get()
                )
            }
            cue_class=move || {
                cn!(
                    "max-w-[92%] rounded-[0.2em] bg-black/75 px-[0.45em] py-[0.12em] leading-snug text-balance text-white",
                    caption_size_class(ctx.caption_size.get()),
                    cue_class.get()
                )
            }
        />
    }
}

#[component]
pub fn VideoPlayerControls(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <VideoPlayerControlsRoot class=move || {
            cn!(
                "pointer-events-none absolute inset-x-0 bottom-0 flex flex-col gap-2 bg-gradient-to-t from-black/80 via-black/40 to-transparent px-3 pt-8 pb-2.5 opacity-0 transition-opacity duration-200 data-[active=true]:pointer-events-auto data-[active=true]:opacity-100",
                class.get()
            )
        }>{children()}</VideoPlayerControlsRoot>
    }
}

#[component]
pub fn VideoPlayerPlayButton(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    let ctx = use_video_player();

    view! {
        <VideoPlayerPlayRoot class=move || {
            cn!(CONTROL, class.get())
        }>
            {move || {
                if ctx.playing.get() {
                    Either::Left(view! { <Pause /> })
                } else {
                    Either::Right(view! { <Play /> })
                }
            }}
        </VideoPlayerPlayRoot>
    }
}

#[component]
pub fn VideoPlayerSkipButton(
    step: VideoPlayerStep,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    view! {
        <VideoPlayerSkipRoot step=step class=move || cn!(CONTROL, class.get())>
            {match step {
                VideoPlayerStep::Previous => Either::Left(view! { <SkipBack /> }),
                VideoPlayerStep::Next => Either::Right(view! { <SkipForward /> }),
            }}
        </VideoPlayerSkipRoot>
    }
}

#[component]
pub fn VideoPlayerMuteButton(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    let ctx = use_video_player();

    view! {
        <VideoPlayerMuteRoot class=move || {
            cn!(CONTROL, class.get())
        }>
            {move || {
                if ctx.muted.get() || ctx.volume.get() <= 0.0 {
                    Either::Left(view! { <VolumeOff /> })
                } else {
                    Either::Right(view! { <Volume /> })
                }
            }}
        </VideoPlayerMuteRoot>
    }
}

#[component]
pub fn VideoPlayerSeek(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <VideoPlayerRailRoot
            rail=VideoPlayerRail::Seek
            class=move || {
                cn!(
                    "group/rail relative h-1.5 w-full cursor-pointer touch-none rounded-full bg-white/25 outline-none focus-visible:ring-2 focus-visible:ring-white/70 data-[disabled=true]:cursor-default data-[disabled=true]:opacity-50",
                    class.get()
                )
            }
            thumb=Callback::new(|x: f64| {
                view! {
                    <span
                        class="pointer-events-none absolute top-1/2 size-3 -translate-x-1/2 -translate-y-1/2 rounded-full bg-white opacity-0 shadow transition-opacity group-hover/rail:opacity-100 group-focus-visible/rail:opacity-100 group-data-[dragging=true]/rail:opacity-100"
                        style=format!("left: {x}%;")
                    />
                }
                    .into_any()
            })
        >
            <VideoPlayerRailFillRoot
                rail=VideoPlayerRail::Seek
                buffered=true
                class="absolute inset-y-0 left-0 rounded-full bg-white/35"
            />
            <VideoPlayerRailFillRoot
                rail=VideoPlayerRail::Seek
                class="absolute inset-y-0 left-0 rounded-full bg-primary"
            />
        </VideoPlayerRailRoot>
    }
}

#[component]
pub fn VideoPlayerVolume(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <VideoPlayerRailRoot
            rail=VideoPlayerRail::Volume
            class=move || {
                cn!(
                    "group/rail relative h-1 w-16 cursor-pointer touch-none rounded-full bg-white/25 outline-none focus-visible:ring-2 focus-visible:ring-white/70",
                    class.get()
                )
            }
            thumb=Callback::new(|x: f64| {
                view! {
                    <span
                        class="pointer-events-none absolute top-1/2 size-2.5 -translate-x-1/2 -translate-y-1/2 rounded-full bg-white shadow"
                        style=format!("left: {x}%;")
                    />
                }
                    .into_any()
            })
        >
            <VideoPlayerRailFillRoot
                rail=VideoPlayerRail::Volume
                class="absolute inset-y-0 left-0 rounded-full bg-white"
            />
        </VideoPlayerRailRoot>
    }
}

#[component]
pub fn VideoPlayerTime(
    #[prop(default = false)] elapsed_only: bool,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    view! {
        <VideoPlayerTimeRoot
            elapsed_only=elapsed_only
            class=move || {
                cn!("px-1 font-mono text-xs tabular-nums text-white/90 select-none", class.get())
            }
        />
    }
}

#[component]
pub fn VideoPlayerFullscreenButton(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    let ctx = use_video_player();

    view! {
        <VideoPlayerFullscreenRoot class=move || {
            cn!(CONTROL, class.get())
        }>
            {move || {
                if ctx.fullscreen.get() {
                    Either::Left(view! { <Minimize /> })
                } else {
                    Either::Right(view! { <Maximize /> })
                }
            }}
        </VideoPlayerFullscreenRoot>
    }
}

#[component]
pub fn VideoPlayer(
    #[prop(optional, into)] src: Signal<String>,
    #[prop(optional, into)] poster: Signal<String>,
    #[prop(optional)] mode: VideoPlayerMode,
    #[prop(default = false)] autoplay: bool,
    #[prop(default = false)] r#loop: bool,
    #[prop(default = false)] muted: bool,
    #[prop(default = None, into)] on_skip: Option<Callback<VideoPlayerStep>>,
    #[prop(default = None, into)] controls: Option<Callback<(), AnyView>>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let controls = StoredValue::new(controls);
    view! {
        <VideoPlayerRoot
            mode=mode
            on_skip=on_skip
            class=move || {
                cn!(
                    "relative isolate aspect-video w-full overflow-hidden rounded-xl bg-black outline-none focus-visible:ring-2 focus-visible:ring-ring/60",
                    if mode.has_controls() {
                        "cursor-none data-[active=true]:cursor-default"
                    } else {
                        ""
                    },
                    class.get()
                )
            }
        >
            <VideoPlayerVideo src=src poster=poster autoplay=autoplay r#loop=r#loop muted=muted>
                {children.map(|children| children())}
            </VideoPlayerVideo>

            <VideoPlayerSurface>
                {mode.interactive().then(|| view! { <VideoPlayerBadge /> })}
            </VideoPlayerSurface>

            <VideoPlayerCaptions />

            {(mode.has_controls() && controls.with_value(|controls| controls.is_some()))
                .then(|| {
                    view! {
                        <VideoPlayerControls>
                            {move || controls.with_value(|controls| controls.map(|c| c.run(())))}
                        </VideoPlayerControls>
                    }
                })}

            {(mode.has_controls() && controls.with_value(|controls| controls.is_none()))
                .then(|| {
                    view! {
                        <VideoPlayerControls>
                            <VideoPlayerSeek />
                            <div class="flex items-center gap-1">
                                <VideoPlayerPlayButton />
                                {mode
                                    .has_skip()
                                    .then(|| {
                                        view! {
                                            <VideoPlayerSkipButton step=VideoPlayerStep::Previous />
                                            <VideoPlayerSkipButton step=VideoPlayerStep::Next />
                                        }
                                    })}
                                {mode.has_volume().then(|| view! { <VideoPlayerVolumeControl /> })}
                                {mode.has_timestamp().then(|| view! { <VideoPlayerTime /> })}
                                <div class="flex-1" />
                                {mode
                                    .has_timestamp()
                                    .then(|| view! { <VideoPlayerCaptionsButton /> })}
                                {mode
                                    .has_timestamp()
                                    .then(|| {
                                        view! {
                                            <VideoPlayerMenu>
                                                <VideoPlayerSpeedItems submenu=true />
                                                <VideoPlayerCaptionItems submenu=true />
                                                <VideoPlayerCaptionSizeItems submenu=true />
                                            </VideoPlayerMenu>
                                        }
                                    })}
                                {mode
                                    .has_fullscreen()
                                    .then(|| view! { <VideoPlayerFullscreenButton /> })}
                            </div>
                        </VideoPlayerControls>
                    }
                })}
        </VideoPlayerRoot>
    }
}

const RATES: &[f64] = &[0.5, 0.75, 1.0, 1.25, 1.5, 2.0];

#[component]
pub fn VideoPlayerVolumeControl(
    #[prop(default = true)] collapsible: bool,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    view! {
        <div class=move || cn!("group/volume flex items-center gap-1", class.get())>
            <VideoPlayerMuteButton />
            <VideoPlayerVolume class=if collapsible {
                "w-0 opacity-0 transition-[width,opacity] duration-200 group-hover/volume:w-16 group-hover/volume:opacity-100 group-focus-within/volume:w-16 group-focus-within/volume:opacity-100 [@media(hover:none)]:w-16 [@media(hover:none)]:opacity-100"
            } else {
                ""
            } />
        </div>
    }
}

#[component]
fn MenuOpenGuard() -> impl IntoView {
    let player = use_video_player();
    let menu = use_dropdown();

    Effect::new(move |_| player.menu_open.set(menu.is_open.get()));
    on_cleanup(move || {
        let _ = player.menu_open.try_set(false);
    });
}

#[component]
pub fn VideoPlayerMenu(
    #[prop(default = "Settings".to_string(), into)] label: String,
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let label = StoredValue::new(label);

    view! {
        <DropdownMenu>
            <MenuOpenGuard />
            <DropdownMenuTriggerRoot
                class=move || cn!(CONTROL, class.get())
                attr:aria-label=move || label.get_value()
                attr:data-video-player-menu=""
            >
                <Settings />
            </DropdownMenuTriggerRoot>
            <DropdownMenuContent
                side=Side::Top
                align=Align::End
                class="mb-1 min-w-44 border-white/15 bg-black/85 text-white backdrop-blur"
            >
                {children.with_value(|children| children())}
            </DropdownMenuContent>
        </DropdownMenu>
    }
}

#[component]
pub fn VideoPlayerSpeedItems(#[prop(default = false)] submenu: bool) -> impl IntoView {
    let ctx = use_video_player();

    let rows = move || {
        RATES
            .iter()
            .map(|&rate| {
                let chosen = Signal::derive(move || (ctx.rate.get() - rate).abs() < f64::EPSILON);
                view! {
                    <DropdownMenuItem
                        class=MENU_ITEM
                        on_click=Callback::new(move |_| ctx.set_rate(rate))
                    >
                        <span>
                            {if rate == 1.0 { "Normal".to_string() } else { format!("{rate}×") }}
                        </span>
                        <Check class=move || {
                            cn!("size-3.5", if chosen.get() { "opacity-100" } else { "opacity-0" })
                        } />
                    </DropdownMenuItem>
                }
            })
            .collect_view()
            .into_any()
    };

    if submenu {
        Either::Left(view! {
            <DropdownMenuSub>
                <DropdownMenuSubTrigger class=SUBMENU_TRIGGER>"Speed"</DropdownMenuSubTrigger>
                <DropdownMenuSubContent class=SUBMENU_CONTENT>{rows()}</DropdownMenuSubContent>
            </DropdownMenuSub>
        })
    } else {
        Either::Right(view! {
            <DropdownMenuLabel>"Speed"</DropdownMenuLabel>
            {rows()}
        })
    }
}

#[component]
pub fn VideoPlayerCaptionItems(#[prop(default = false)] submenu: bool) -> impl IntoView {
    let ctx = use_video_player();

    let rows = move || {
        view! {
            <DropdownMenuItem
                class=MENU_ITEM
                on_click=Callback::new(move |_| ctx.set_text_track(None))
            >
                <span>"Off"</span>
                <Check class=move || {
                    cn!(
                        "size-3.5",
                        if ctx.active_track.get().is_none() { "opacity-100" } else { "opacity-0" },
                    )
                } />
            </DropdownMenuItem>
            <For each=move || ctx.tracks.get() key=|track| track.index let:track>
                {
                    let index = track.index;
                    view! {
                        <DropdownMenuItem
                            class=MENU_ITEM
                            on_click=Callback::new(move |_| ctx.set_text_track(Some(index)))
                        >
                            <span>{track.label.clone()}</span>
                            <Check class=move || {
                                cn!(
                                    "size-3.5",
                                    if ctx.active_track.get() == Some(index) {
                                        "opacity-100"
                                    } else {
                                        "opacity-0"
                                    },
                                )
                            } />
                        </DropdownMenuItem>
                    }
                }
            </For>
        }
        .into_any()
    };

    view! {
        <Show when=move || {
            !ctx.tracks.get().is_empty()
        }>
            {move || {
                if submenu {
                    Either::Left(
                        view! {
                            <DropdownMenuSub>
                                <DropdownMenuSubTrigger class=SUBMENU_TRIGGER>
                                    "Subtitles"
                                </DropdownMenuSubTrigger>
                                <DropdownMenuSubContent class=SUBMENU_CONTENT>
                                    {rows()}
                                </DropdownMenuSubContent>
                            </DropdownMenuSub>
                        },
                    )
                } else {
                    Either::Right(
                        view! {
                            <DropdownMenuSeparator />
                            <DropdownMenuLabel>"Subtitles"</DropdownMenuLabel>
                            {rows()}
                        },
                    )
                }
            }}
        </Show>
    }
}

#[component]
pub fn VideoPlayerCaptionSizeItems(#[prop(default = false)] submenu: bool) -> impl IntoView {
    let ctx = use_video_player();

    let rows = move || {
        VideoPlayerCaptionSize::ALL
            .iter()
            .map(|&size| {
                view! {
                    <DropdownMenuItem
                        class=MENU_ITEM
                        on_click=Callback::new(move |_| ctx.caption_size.set(size))
                    >
                        <span>{size.label()}</span>
                        <Check class=move || {
                            cn!(
                                "size-3.5",
                                if ctx.caption_size.get() == size {
                                    "opacity-100"
                                } else {
                                    "opacity-0"
                                },
                            )
                        } />
                    </DropdownMenuItem>
                }
            })
            .collect_view()
            .into_any()
    };

    view! {
        <Show when=move || {
            !ctx.tracks.get().is_empty() && ctx.active_track.get().is_some()
        }>
            {move || {
                if submenu {
                    Either::Left(
                        view! {
                            <DropdownMenuSub>
                                <DropdownMenuSubTrigger class=SUBMENU_TRIGGER>
                                    "Subtitle size"
                                </DropdownMenuSubTrigger>
                                <DropdownMenuSubContent class=SUBMENU_CONTENT>
                                    {rows()}
                                </DropdownMenuSubContent>
                            </DropdownMenuSub>
                        },
                    )
                } else {
                    Either::Right(
                        view! {
                            <DropdownMenuSeparator />
                            <DropdownMenuLabel>"Subtitle size"</DropdownMenuLabel>
                            {rows()}
                        },
                    )
                }
            }}
        </Show>
    }
}

#[component]
pub fn VideoPlayerCaptionsButton(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    let ctx = use_video_player();
    let showing = Signal::derive(move || ctx.active_track.get().is_some());

    view! {
        <Show when=move || !ctx.tracks.get().is_empty()>
            <button
                type="button"
                data-slot="video-player-captions"
                data-state=move || if showing.get() { "on" } else { "off" }
                aria-label=move || {
                    if showing.get() { "Turn off subtitles" } else { "Turn on subtitles" }
                }
                aria-pressed=move || showing.get().then_some("true")
                on:click=move |_| {
                    if showing.get_untracked() {
                        ctx.set_text_track(None);
                    } else {
                        ctx.set_text_track(ctx.tracks.get_untracked().first().map(|t| t.index));
                    }
                }
                class=move || {
                    cn!(
                        CONTROL,
                        "relative after:absolute after:inset-x-1.5 after:bottom-1 after:h-px after:bg-current after:opacity-0 data-[state=on]:after:opacity-100",
                        class.get(),
                    )
                }
            >
                <Captions />
            </button>
        </Show>
    }
}
