use crate::{
    cn,
    primitives::color_picker::{
        ColorPickerAreaRoot, ColorPickerFormatRoot, ColorPickerInputRoot, ColorPickerPresetRoot,
        ColorPickerRail, ColorPickerRailRoot, ColorPickerRoot, ColorPickerSwatchRoot,
    },
    utils::color::ColorFormat,
};
use leptos::prelude::*;

/// The chequer that shows through a partly transparent colour. Two gradients rather than an
/// image, so it costs nothing and follows the theme's border colour.
const CHEQUER: &str = "background-image: linear-gradient(45deg, var(--color-border) 25%, transparent 25%, transparent 75%, var(--color-border) 75%), linear-gradient(45deg, var(--color-border) 25%, transparent 25%, transparent 75%, var(--color-border) 75%); background-size: 8px 8px; background-position: 0 0, 4px 4px;";

/// A colour picker. One component rather than a set of parts, like `Calendar`: the square, the
/// rails and the field only ever go together. `primitives::color_picker` has them separately.
#[component]
pub fn ColorPicker(
    /// The colour as text, in the current format. Read it for a form; write it to set the picker.
    #[prop(optional, into)]
    value: RwSignal<String>,
    #[prop(optional)] format: ColorFormat,
    /// Whether to offer an opacity rail.
    #[prop(default = false)]
    with_alpha: bool,
    /// A row of colours under the picker. CSS colours, in any form the parser reads.
    #[prop(optional, into)]
    presets: Vec<&'static str>,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    view! {
        <ColorPickerRoot
            value=value
            format=format
            with_alpha=with_alpha
            class=move || {
                cn!(
                    "flex w-64 flex-col gap-3 rounded-lg border bg-popover p-3 text-popover-foreground",
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
                // The swatch sits over the chequer, so a transparent colour reads as transparent
                // rather than as the panel's own background.
                <div
                    class="size-9 shrink-0 overflow-hidden rounded-md border"
                    style=CHEQUER
                >
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

            <div class="flex items-center gap-2">
                <ColorPickerInputRoot class="h-8 w-full min-w-0 flex-1 rounded-md border border-input bg-transparent px-2 font-mono text-xs outline-none focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50 data-invalid:border-destructive data-invalid:ring-3 data-invalid:ring-destructive/20" />
                <ColorPickerFormatRoot class="h-8 shrink-0 rounded-md border px-2 font-mono text-xs uppercase transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:ring-3 focus-visible:ring-ring/50 focus-visible:outline-none" />
            </div>

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
