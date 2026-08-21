# TODO

Working list for `nocomponents`. Status reflects the tree as of 2026-08-21; check items off as they
land and add new ones rather than keeping a parallel list elsewhere. Closed items are one line —
the reasoning is in the commit that closed them.

## Bugs

- [x] Dialog content resolved a *different* dialog's context: leptos' `Portal` mounts under an
      owner of its own, which a bare `provide_context` does not reach. Scoped with `<Provider>`.
- [x] `animate.css` fill-mode fallbacks were dead code — a registered `@property` resolves to its
      initial value rather than to the `var()` fallback. Dropped `initial-value: none`.
- [x] The dialog plays its exit animation: the portal's `Show` gates on `is_mounted`, not
      `is_open`.
- [x] Closing floating content swallowed clicks for `unmount_delay`. The positioning shells are
      permanently `pointer-events-none` and the styled content re-enables itself.
- [x] Selecting a submenu item left the parent menu open — a submenu's own `FloatingRoot` shadows
      the context, so `MenuRoot` publishes the outermost one alongside it.
- [x] A delayed unmount could fire into a disposed owner and panic. Both roots keep the
      `TimeoutHandle` and clear it in `on_cleanup`.

## Accessibility & keyboard

All closed; this no longer blocks a "stable" tag. What exists, and where to reuse it:

- [x] `src/primitives/dismiss.rs` — one layer stack, ordered by the moment each layer opened.
      Escape dismisses only the topmost; an outside `pointerdown` walks down until it reaches a
      layer that contains the target, or a `LayerBounds::modal()` one, which stops the walk.
- [x] `src/primitives/roving_focus.rs` — a group is one tab stop and the arrows move inside it.
      Items are found in the DOM by `data-roving-item`, so it does not care how they are rendered;
      disabled ones are skipped and the tab stop starts on whatever the group has selected. Used by
      the dropdown menu, select, tabs, toggle group, radio group and accordion.
- [x] `src/primitives/typeahead.rs` — keystrokes accumulate for a second, a repeated letter cycles.
      In the select and, on the shared `handle_menu_keys`, in all three menu surfaces.
- [x] Arrow keys open a closed trigger: `FloatingRoot` takes an `ArrowOpen`, and the intent reaches
      the content as `FloatingContext.open_focus`. A select opens on its value; a menu opens onto
      its first item for Down and its last for Up.
- [x] Dialog focus: the Tab trap and the focus-in were already there, and closing now returns focus
      to whatever had it. Content points `aria-labelledby` / `aria-describedby` at the parts that
      actually rendered.
- [x] Floating triggers carry `aria-haspopup` / `aria-expanded` / `aria-controls`, written onto
      `trigger_ref` from an effect — the trigger is rendered a layer up and in three shapes, and
      the node ref is the only thing all three share. The arrow keys are bound the same way.

## Components — gap vs the shadcn/ui catalogue

Shipped (59): accordion, alert, alert-dialog, aspect-ratio, avatar, badge, breadcrumb, button,
button-group, calendar, card, carousel, chart, checkbox, code, collapsible, color-picker,
combobox, command, context-menu, data-table, date-picker, dialog, drawer, dropdown-menu, empty,
field, hover-card, image-cropper, input, input-group, input-otp, item, kbd, label, menubar,
native-select, navigation-menu,
pagination, popover, progress, radio-group, resizable, scroll-area, select, separator, sheet,
sidebar, skeleton, slider, spinner, switch, table, tabs, textarea, toast, toggle, toggle-group,
tooltip. `code` is not in the shadcn catalogue — it is ours, for documenting the rest.

Missing (7 of the catalogue), ordered by priority. Priority = how often it is reached for,
weighted down by how much machinery it needs that this library does not have yet.

**P3 — substantial state machines.** All closed. Each needed filtering, list navigation or
measurement work beyond what the shared helpers cover, and three of them are what the docs site
itself now runs on: the sidebar is the nav, the command palette is ⌘K, and the data table is the
only honest way to find out what living with a generic row type is like.

- [x] Command — items are not rendered when they do not match, so document order *is* the order
      the arrows walk. The highlight moves through `aria-activedescendant`, which is why it could
      not reuse `roving_focus`.
- [x] Combobox — the command palette inside a floating layer, and nothing else. `ComboboxItemRoot`
      is the whole bridge: it writes the value and closes the layer.
- [x] Navigation Menu — links rather than commands, so every one is tabbable and the panels open
      on a delay. Leaving is decided at the menu, not per trigger; per trigger, a stale close from
      the one you left takes down the panel the one you arrived at just opened.
- [x] Menubar — a row of dropdown menus with one bar over them: one open at a time, and hovering
      or arrowing along it swaps menus. Reuses the dropdown's surface, items and submenus whole.
- [x] Resizable — percentages, never pixels, with the panels' `flex-grow` *being* the percentage,
      which keeps the dividers' own widths out of the arithmetic.
- [x] Carousel — scroll snapping rather than a transform, so a trackpad and a touch drag come free
      and the current slide is read back out of the scroll position rather than assumed.
- [x] Calendar — `utils::date` is a 12-byte civil date and Hinnant's day-count conversions, so
      there is no `chrono` for consumers to agree with. Focus follows a *date*, not a cell, which
      is what makes stepping off the end of a month page to the next one.
- [x] Date Picker — a popover holding a calendar, plus the one rule that makes it a picker: the
      layer closes when the selection is finished. Composition, like `alert_dialog`.
- [x] Table sorting/selection helpers, then Data Table — `TableSort` and `TableSelection` are
      plain handles rather than context providers, so they work over a hand-laid-out table too.
      `DataTable` is generic over the row type and sorts a copy for display.
- [x] Chart — line, area and bar as one `<svg>`, geometry in viewBox units so nothing is measured
      and there is no resize listener. The pointer is the one exception.

**P4 — app-specific pieces.** Useful only once the core set is solid; several are chat/AI surfaces
rather than general UI, so they are the easiest to defer or skip.

- [ ] Direction (RTL provider; cheap, but only pays off once enough components read it)
- [ ] Marker
- [ ] Attachment
- [ ] Bubble
- [ ] Message
- [ ] Message Scroller
- [ ] Questionnaire

**Beyond the shadcn catalogue.** Ours. They were parked behind "needs dependencies the library
does not have", and after writing `utils::date` by hand that reads as the wrong call: neither needs
a crate, so neither needs a feature flag. What they actually need is written out below.

- [x] **A shared pointer-drag helper.** `primitives/drag.rs`: `use_drag(on_move)`, with
      `.against(node_ref)`, `.on_start` and `.on_end`. Each move arrives as a `DragPoint` — client
      position, delta since the press, and position as a fraction of a reference box — and the
      release as a `DragEnd` carrying the velocity and how long the pointer had been still.
      `slider`, `scroll_area`, `drawer` and `resizable` are folded onto it: 83 lines of listener
      bookkeeping gone, and all four now end on `pointercancel`, which every hand-rolled copy
      missed and which used to leave a cancelled touch drag tracking forever.

- [x] **`utils::color`.** `Rgba` / `Hsva` / `Hsla` / `Oklch`, conversions both ways, a parser for
      every CSS form this library writes plus the legacy comma ones, and a formatter. 9 tests, no
      crate. Two things it turned up: HSV↔HSL is exact while anything through 8-bit RGB is not
      (so `Hsva` is what a picker stores), and **`--chart-1` does not fit in sRGB** — reading one
      of the theme's own tokens back clips it a few degrees round the hue circle. The picker will
      have to show that rather than pretend, or editing the palette will look broken.

- [x] **`utils::image`.** The crop rectangle in normalised coordinates, `moved_by`, `resized`
      anchoring the opposite corner and holding an aspect ratio, and `crop_to_data_url` over a
      canvas. 12 tests.

- [x] **`utils::gif`.** Cropping an animated GIF, frame by frame, at the index level: LZW out,
      take the sub-rectangle, LZW back in. Palettes, delays, disposal, transparency and the loop
      count all survive because none of them are decoded. 10 tests plus a browser check.

- [x] **Color Picker.** `primitives::color_picker` + `components::color_picker`, over
      `utils::color`. Holds HSVA, so the hue survives the black corner — verified by dragging
      there and back and watching the rail stay at 217°. The square is two CSS gradients over a
      pure hue; the two rails are one component twice.

- [x] **Image Cropper.** `primitives::image_cropper` + `components::image_cropper`, over
      `utils::image` and `utils::gif`. A still goes through the canvas; an animated GIF goes
      through `crop_gif` and comes out still animating — 16 frames in, 16 frames out, loop count
      intact, checked with Chrome's own frame decoder.

**Unfinished corners of what is shipped.**

- [ ] `field` has no `<fieldset>` / `<legend>` pair for grouping a radio group or a set of
      checkboxes under one label — that needs `aria-labelledby`, not `for`.
- [ ] `highlight` is a lexer, not a parser: a call and a plain identifier are the same token,
      capitalisation is what makes a "type", JS regex literals are unrecognised, and `${…}` is not
      broken out of a template literal. A new language means a new scanner beside `tokenize_rust`.
- [ ] `utils::date` names its months and weekdays in English only, as two consts. Locale-aware
      names mean `js_sys::Intl`, which is a dependency's worth of API for two arrays — worth it
      once anything else in the library needs a locale.
- [ ] `navigation_menu` has no shared viewport: each panel is absolutely positioned against the
      menu, so they all open in the same place, but the box does not slide or resize between them.
      That needs the active panel measured and the size animated.
- [ ] `chart` is line, area and bar over a categorical x axis. No stacking, no second axis, no
      time scale, no pie — each is more arithmetic in `primitives/chart.rs`, not new machinery.
- [ ] `data_table` has no pagination of its own; it shows what it is handed. `Pagination` is
      shipped and the two compose, but nothing demonstrates them together yet.
- [ ] `utils::color` does not read colour names — 148 of them, and a picker has a swatch row. A
      handful (`transparent`, `white`, `black`) would cover most of what anyone types.
- [ ] `utils::gif` re-emits every frame at its clipped size rather than diffing consecutive frames
      for a minimal changed rectangle, so a cropped GIF is a little larger than it needs to be. The
      sample goes 17.5kB → 7.3kB on a crop to a third of the area, which is fine but not optimal.
- [ ] `docs/assets/avatar.jpg` is a 6.9MB PNG with a `.jpg` name. It is `include_bytes!`d, so all
      of it lands in the wasm binary — worth re-encoding to a few hundred kilobytes and renaming.
      The blob is labelled by sniffing the bytes rather than by the extension, so nothing is
      *broken* by the mismatch.
- [ ] The colour picker's square announces itself as `role="application"` with a description of
      the keys. There is no honest ARIA role for a two-axis drag; the rails and the text field are
      the paths that announce themselves properly.

## Docs site

- [x] Docs pages for alert-dialog, label and spinner.
- [x] A `code` snippet under every demo — 89 blocks across 26 pages, extracted from the demo markup
      so the two cannot drift.
- [ ] Snippets are extracted, not curated. A few still carry the demo's layout wrapper (`<div
      class="flex gap-3">`). Worth a pass to trim the ones where it adds nothing.
- [ ] Dead nav links in `docs/src/layout/doc_layout.rs`: `/docs/introduction` and
      `/docs/installation` have no route and fall through to "Not Found."
- [ ] Blocks pages exist for every component and are all empty. The shape is in place — one
      parameterised route, one `BLOCKS` registry keyed by slug, a link both ways in the header —
      so writing one is a single entry in `docs/src/pages/docs/blocks.rs`. What to write first: a
      colour picker in a popover, an image cropper in a dialog, and either of them inside a
      `Field` with validation, since the seams are what a block is for.
- [ ] That header link is a stopgap. Once blocks exist, a component page should open onto a
      *switch* rather than a link: **Component**, **Blocks**, and later **Primitives** — one page,
      three views of it. The routes are already shaped for it (`/docs/<slug>`,
      `/docs/<slug>/blocks`, `/docs/<slug>/primitives`), so it is a `Tabs` in `DocLayout` driven
      by the path rather than a restructure.
- [ ] Adding a component means editing five files, two of which (`NAV`, `COMPONENTS`) are
      hand-maintained copies of the same list — which is how `/docs` came to be missing progress,
      skeleton and textarea. Drive routes, nav and the index from one const.
- [ ] The primitives have no docs pages at all: every page documents the styled layer, and the
      behaviour layer — the half a Radix-style library is sold on — is undocumented. The same
      table treatment, at `/docs/<slug>/primitives` as the third view of the switch above, rather
      than a second tree in the nav: nobody looks up `DialogContentRoot` except while reading
      about `Dialog`.
- [ ] A `/docs/types` page under Sections: the shared types the prop tables keep naming —
      `Signal<String>`, `ChildrenFn`, the `render` callback shape, `Side` / `Align` /
      `Orientation` / `SideOffset` — described once, so a table can point at them instead of
      re-explaining them per component.
- [x] Prop tables on all 47 component pages — 199 parts, one table each, under one API reference
      heading. The rows were drafted by a script over the `#[component]` signatures (names, types,
      defaults, and any prop that already carries a `///`) and the prose written per page.
- [ ] Generate those rows at build time rather than keeping 199 hand-pasted tables in step. The
      draft script in `tools/api-extract.py` is the prototype; it wants to be an `xtask` or a
      build script emitting the same `ApiEntry` shape, and it could drive `NAV` and `COMPONENTS`
      too, closing the five-files item above.
- [ ] `Cargo.toml` has no `description`, `license`, `repository`, `keywords` or `categories` —
      `cargo publish` will reject it as-is.
- [ ] No LICENSE file, though the README credits shadcn/ui and Radix (both MIT).
- [ ] Consumers have no way to get the CSS: the tokens, `@custom-variant`s and `animate.css` live
      in `docs/styles/` only. Ship them from the crate, or document a copy-paste install.
- [ ] `leptos` is pinned to `features = ["csr"]`, which forces CSR on every consumer.
- [ ] No `///` docs on public components, so docs.rs would be empty.
- [ ] README is a disclaimer — needs install steps, a minimal example and the feature-flag table.

## Repo hygiene

- [ ] Almost no tests. `src/utils/highlight.rs` has 7 (`cargo test --features full`); nothing else
      is covered. `wasm-bindgen-test` smoke tests over the primitives' `data-state` transitions
      would catch regressions in the floating and toast state machines.
- [ ] Nothing installs a panic hook, so a panic in the docs app surfaces as
      `RuntimeError: unreachable` with no message. Three lines in `docs/src/main.rs`.
- [x] CI in `.github/workflows/ci.yml`: `fmt --check`, `clippy --features full -D warnings`,
      `check -p docs`, `test --features full`. `pages.yml` builds the docs site and deploys it to
      GitHub Pages on every push to main — needs Settings → Pages → Source: GitHub Actions once.
