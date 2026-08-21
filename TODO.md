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

Shipped (47): accordion, alert, alert-dialog, aspect-ratio, avatar, badge, breadcrumb, button,
button-group, card, checkbox, code, collapsible, context-menu, dialog, drawer, dropdown-menu,
empty, field, hover-card, input, input-group, input-otp, item, kbd, label, native-select,
pagination, popover, progress, radio-group, scroll-area, select, separator, sheet, sidebar,
skeleton, slider, spinner, switch, table, tabs, textarea, toast, toggle, toggle-group, tooltip. `code` is
not in the shadcn catalogue — it is ours, for documenting the rest.

Missing (17 of the catalogue), ordered by priority. Priority = how often it is reached for,
weighted down by how much machinery it needs that this library does not have yet.

**P3 — substantial state machines.** Each needs filtering, list navigation or measurement work
beyond what the shared helpers cover. Sidebar is done — it is what the docs site's own nav is
built from, which is the only honest way to find out what living with it is like.

- [ ] Command (filtering + keyboard list navigation)
- [ ] Combobox (Command + Popover; `FloatingContext.value` / `display_value` were added for it)
- [ ] Navigation Menu
- [ ] Menubar
- [ ] Resizable
- [ ] Carousel
- [ ] Calendar
- [ ] Date Picker (Calendar + Popover)
- [ ] Table sorting/selection helpers, then Data Table
- [ ] Chart

**P4 — app-specific pieces.** Useful only once the core set is solid; several are chat/AI surfaces
rather than general UI, so they are the easiest to defer or skip.

- [ ] Direction (RTL provider; cheap, but only pays off once enough components read it)
- [ ] Marker
- [ ] Attachment
- [ ] Bubble
- [ ] Message
- [ ] Message Scroller
- [ ] Questionnaire

**Beyond the shadcn catalogue.** Ours, and both need dependencies the library does not have, so
both go behind their own feature flag rather than into `full`.

- [ ] Color Picker — colour-space conversion (hex/rgb/hsl/oklch) and a saturation/value canvas.
- [ ] Image Cropper — pointer-drag geometry, a canvas for the output, an image encoder.

**Unfinished corners of what is shipped.**

- [ ] `field` has no `<fieldset>` / `<legend>` pair for grouping a radio group or a set of
      checkboxes under one label — that needs `aria-labelledby`, not `for`.
- [ ] `highlight` is a lexer, not a parser: a call and a plain identifier are the same token,
      capitalisation is what makes a "type", JS regex literals are unrecognised, and `${…}` is not
      broken out of a template literal. A new language means a new scanner beside `tokenize_rust`.

## Docs site

- [x] Docs pages for alert-dialog, label and spinner.
- [x] A `code` snippet under every demo — 89 blocks across 26 pages, extracted from the demo markup
      so the two cannot drift.
- [ ] Snippets are extracted, not curated. A few still carry the demo's layout wrapper (`<div
      class="flex gap-3">`). Worth a pass to trim the ones where it adds nothing.
- [ ] Dead nav links in `docs/src/layout/doc_layout.rs`: `/docs/introduction` and
      `/docs/installation` have no route and fall through to "Not Found."
- [ ] Adding a component means editing five files, two of which (`NAV`, `COMPONENTS`) are
      hand-maintained copies of the same list — which is how `/docs` came to be missing progress,
      skeleton and textarea. Drive routes, nav and the index from one const.
- [ ] The primitives have no docs pages at all: every page documents the styled layer, and the
      behaviour layer — the half a Radix-style library is sold on — is undocumented. Needs its own
      section in the nav, and the same table treatment.
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
