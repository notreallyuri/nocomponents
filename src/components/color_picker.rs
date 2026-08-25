use crate::{
    cn,
    components::{
        input_group::{InputGroup, InputGroupAddon},
        select::{Select, SelectContent, SelectItem, SelectPortal, SelectTrigger},
        separator::Separator,
    },
    primitives::color_picker::{
        ColorPickerAreaRoot, ColorPickerChannelRoot, ColorPickerInputRoot, ColorPickerPresetRoot,
        ColorPickerRail, ColorPickerRailRoot, ColorPickerRoot, ColorPickerSwatchRoot,
        use_color_picker,
    },
    utils::{color::ColorFormat, types::Orientation},
};
use leptos::{either::Either, prelude::*};

const CHEQUER: &str = "background-image: linear-gradient(45deg, var(--color-border) 25%, transparent 25%, transparent 75%, var(--color-border) 75%), linear-gradient(45deg, var(--color-border) 25%, transparent 25%, transparent 75%, var(--color-border) 75%); background-size: 8px 8px; background-position: 0 0, 4px 4px;";

#[component]
fn ColorPickerFields() -> impl IntoView {
    let ctx = use_color_picker();
    let selection = ctx.format_selection();

    view! {
        <InputGroup class="h-9 flex-nowrap gap-1 px-1.5">
            <InputGroupAddon>
                <Select value=selection class="w-auto">
                    <SelectTrigger class="h-7 w-19 gap-1 border-0 bg-transparent px-1.5 font-mono text-xs uppercase focus:ring-0 focus:ring-offset-0">
                        <span class="truncate">{move || ctx.format.get().label()}</span>
                    </SelectTrigger>
                    <SelectPortal>
                        <SelectContent class="min-w-24">
                            {ColorFormat::ALL
                                .into_iter()
                                .map(|format| {
                                    view! {
                                        <SelectItem value=format.as_str() class="font-mono text-xs">
                                            {format.label()}
                                        </SelectItem>
                                    }
                                })
                                .collect_view()}
                        </SelectContent>
                    </SelectPortal>
                </Select>
            </InputGroupAddon>

            <Separator orientation=Orientation::Vertical class="h-5" />

            {move || {
                let channels = ctx.channels();
                if channels.is_empty() {
                    Either::Left(

                        view! {
                            <ColorPickerInputRoot class="h-7 w-full min-w-0 flex-1 bg-transparent font-mono text-xs outline-none data-invalid:text-destructive" />
                        },
                    )
                } else {
                    Either::Right(
                        channels
                            .into_iter()
                            .map(|channel| {
                                view! {
                                    <div class="flex min-w-0 flex-1 items-center gap-px">
                                        <span class="shrink-0 font-mono text-[0.65rem] text-muted-foreground select-none">
                                            {channel.label}
                                        </span>
                                        <ColorPickerChannelRoot
                                            channel=channel
                                            class="h-7 w-full min-w-0 bg-transparent text-center font-mono text-xs tabular-nums outline-none"
                                        />
                                    </div>
                                }
                            })
                            .collect_view(),
                    )
                }
            }}
        </InputGroup>
    }
}

#[component]
pub fn ColorPicker(
    #[prop(optional, into)] value: RwSignal<String>,
    #[prop(optional)] format: ColorFormat,
    #[prop(default = false)] with_alpha: bool,
    #[prop(optional, into)] presets: Vec<&'static str>,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    view! {
        <ColorPickerRoot
            value=value
            format=format
            with_alpha=with_alpha
            class=move || {
                cn!(
                    "flex w-72 flex-col gap-3 rounded-lg border bg-popover p-3 text-popover-foreground",
                    class.get()
                )
            }
        >
            <ColorPickerAreaRoot
                class="relative h-40 w-full cursor-crosshair rounded-md outline-none focus-visible:ring-3 focus-visible:ring-ring/50"
                thumb=Callback::new(|(x, y): (f64, f64)| {
                    view! {
                        <span
                            class="pointer-events-none absolute size-4 -translate-x-1/2 -translate-y-1/2 rounded-full border-2 border-white shadow-[0_0_0_1px_rgba(0,0,0,0.4)]"
                            style=format!("left: {x}%; top: {y}%;")
                        />
                    }
                        .into_any()
                })
            />

            <div class="flex items-center gap-3">
                <div class="size-9 shrink-0 overflow-hidden rounded-md border" style=CHEQUER>
                    <ColorPickerSwatchRoot class="size-full" />
                </div>

                <div class="flex min-w-0 flex-1 flex-col gap-2">
                    <ColorPickerRailRoot
                        rail=ColorPickerRail::Hue
                        class="relative h-3 w-full cursor-ew-resize rounded-full outline-none focus-visible:ring-3 focus-visible:ring-ring/50"
                        thumb=Callback::new(|x: f64| {
                            view! {
                                <span
                                    class="pointer-events-none absolute top-1/2 size-4 -translate-x-1/2 -translate-y-1/2 rounded-full border-2 border-white bg-transparent shadow-[0_0_0_1px_rgba(0,0,0,0.4)]"
                                    style=format!("left: {x}%;")
                                />
                            }
                                .into_any()
                        })
                    />

                    {with_alpha
                        .then(|| {
                            view! {
                                <div class="overflow-hidden rounded-full" style=CHEQUER>
                                    <ColorPickerRailRoot
                                        rail=ColorPickerRail::Alpha
                                        class="relative h-3 w-full cursor-ew-resize rounded-full outline-none focus-visible:ring-3 focus-visible:ring-ring/50"
                                        thumb=Callback::new(|x: f64| {
                                            view! {
                                                <span
                                                    class="pointer-events-none absolute top-1/2 size-4 -translate-x-1/2 -translate-y-1/2 rounded-full border-2 border-white shadow-[0_0_0_1px_rgba(0,0,0,0.4)]"
                                                    style=format!("left: {x}%;")
                                                />
                                            }
                                                .into_any()
                                        })
                                    />
                                </div>
                            }
                        })}
                </div>
            </div>

            <ColorPickerFields />

            {(!presets.is_empty())
                .then(|| {
                    view! {
                        <div class="flex flex-wrap gap-1.5">
                            {presets
                                .into_iter()
                                .map(|preset| {
                                    view! {
                                        <ColorPickerPresetRoot
                                            value=preset
                                            class="size-6 rounded-md border transition-transform hover:scale-110 focus-visible:ring-3 focus-visible:ring-ring/50 focus-visible:outline-none data-selected:ring-2 data-selected:ring-ring data-selected:ring-offset-2 data-selected:ring-offset-popover"
                                        />
                                    }
                                })
                                .collect_view()}
                        </div>
                    }
                })}
        </ColorPickerRoot>
    }
}
