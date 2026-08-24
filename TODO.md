e TODO

Working list for `nocomponents`. Status reflects the tree as of 2026-08-22; check items off as they
land and add new ones rather than keeping a parallel list elsewhere. Closed items are one line —
the reasoning is in the commit that closed them, and anything still to do about one is its own open
item rather than a sentence buried in a closed one.

## Bugs

- [x] Dialog content resolved a *different* dialog's context: leptos' `Portal` mounts under an
      owner of its own, which a bare `provide_context` does not reach. Scoped with `<Provider>`.
- [x] `animate.css` fill-mode fallbacks were dead code — a registered `@property` resolves to its
      initial value, not to the `var()` fallback.
- [x] The dialog plays its exit animation: the portal's `Show` gates on `is_mounted`, not `is_open`.
- [x] Closing floating content swallowed clicks for `unmount_delay`; the positioning shells are
      permanently `pointer-events-none` and the styled content re-enables itself.
- [x] Selecting a submenu item left the parent menu open — `MenuRoot` publishes the outermost
      context alongside the one the submenu's own `FloatingRoot` shadows.
- [x] A delayed unmount could fire into a disposed owner and panic; both roots clear the
      `TimeoutHandle` in `on_cleanup`.
- [x] Colour picker rails took the press and went dead for the rest of the drag: `set_rgba` read
      the colour *tracked*, so a lossy round trip undid every move as a stale write. Reads
      untracked now.
- [x] One `crop_image` over bytes rather than two functions over two inputs — every caller was
      sniffing the type and branching, and the avatar block silently flattened picked GIFs.
- [x] Cropper drag modifiers: shift holds the grabbed shape from the far corner, alt holds it about
      the centre, and shift on a move snaps to eight directions. `DragPoint` carries the modifiers,
      so they can be taken up or let go mid-drag.
- [x] `utils::use_media_query` — one `match_media`, re-read on `resize`, so what it says and what
      CSS is doing cannot drift.
- [x] `Select` wears the `Field` around it like every other control; the field is handed the
      trigger's own id rather than minting a second one no element wears.
- [x] `SidebarMenuButton` takes a `node_ref`, so a dropdown can anchor to a sidebar row.
- [x] A toast raised from a view that disposes itself in the same handler was a panic:
      `show()` minted `is_open` and `height` under the caller's owner. `ToastContext` carries the
      provider's `Owner` and `show()` runs inside it.
- [x] A `FieldLabel` beside a `Combobox` pointed `for` at an id nothing wore. The wiring the select
      already had is now `field_control_for_trigger`, shared by both.

- [ ] The sidebar still hand-rolls `match_media` rather than using `utils::use_media_query`. Its
      adoption turns `SidebarContext::is_mobile` from an `RwSignal` into a `Signal` — a public
      field, so a deliberate change rather than a tidy-up. The theme is no longer a candidate:
      `use_media_query` re-reads on `resize`, which a colour-scheme change does not fire, so
      `theme.rs` listens to the query itself.
- [ ] A floating trigger overwrites a caller's own `id`: `FloatingRoot` writes `trigger_id` onto the
      node unconditionally, so `<SelectTrigger attr:id="email">` silently becomes `nc-trigger-4`. It
      should keep an id the caller set and adopt it as `trigger_id`, so `aria-controls` still points
      somewhere real.
- [ ] A menu item cannot be disabled. `DropdownMenuItemRoot` — which the context menu and the
      menubar reuse whole — takes no `disabled` and never writes `data-disabled`, while the styled
      classes (`data-disabled:opacity-50`), the roving focus and the "focus the first item" query
      all key off exactly that attribute. So the styling and the skipping are written for a state
      nothing can put an item into.
- [ ] `NativeSelect`'s wrapper swallows everything a caller writes on it — `attr:id`, `attr:name`,
      any event. `on_blur` is a prop because that one was needed; the general fix is for the
      `<select>` to be the component's own root, which means drawing the chevron without a
      positioned sibling, which means a background image and a colour that is not a token. Worth a
      decision, not a patch.
- [ ] `ComboboxTrigger` is the one styled trigger that does not render its own `*Root` — it builds
      a `Button` around `ctx.trigger_ref` directly, which is why the field wiring has to be asked
      for twice. `SelectTrigger` shows the shape it should have.
- [ ] `dropdown_menu` carries no `data-slot` on any of its parts, alone among the components.
      Nothing keys off it today, so nothing is broken, but it is the convention everything else
      follows and the thing anyone styling the unstyled layer reaches for — and its absence reads
      as a broken menu when a selector quietly matches nothing.

## Accessibility & keyboard

All closed; this no longer blocks a "stable" tag. What exists, and where to reuse it:

- [x] `primitives/dismiss.rs` — one layer stack ordered by opening time. Escape dismisses the
      topmost; an outside `pointerdown` walks down until a layer contains the target or is modal.
- [x] `primitives/roving_focus.rs` — a group is one tab stop and the arrows move inside it. Items
      are found by `data-roving-item`, so it does not care how they are rendered. Used by the
      dropdown menu, select, tabs, toggle group, radio group and accordion.
- [x] `primitives/typeahead.rs` — keystrokes accumulate for a second, a repeated letter cycles. In
      the select and, via `handle_menu_keys`, in all three menu surfaces.
- [x] Arrow keys open a closed trigger; the intent reaches the content as
      `FloatingContext.open_focus`.
- [x] Dialog focus: the Tab trap, the focus-in, and focus returned to whatever had it on close.
- [x] Floating triggers carry `aria-haspopup` / `aria-expanded` / `aria-controls`, written onto
      `trigger_ref` from an effect — the trigger comes in three shapes and the node ref is all they
      share.

## Components — gap vs the shadcn/ui catalogue

Shipped (59): accordion, alert, alert-dialog, aspect-ratio, avatar, badge, breadcrumb, button,
button-group, calendar, card, carousel, chart, checkbox, code, collapsible, color-picker,
combobox, command, context-menu, data-table, date-picker, dialog, drawer, dropdown-menu, empty,
field, hover-card, image-cropper, input, input-group, input-otp, item, kbd, label, menubar,
native-select, navigation-menu, pagination, popover, progress, radio-group, resizable, scroll-area,
select, separator, sheet, sidebar, skeleton, slider, spinner, switch, table, tabs, textarea, toast,
toggle, toggle-group, tooltip. `code` is not in the shadcn catalogue — it is ours, for documenting
the rest.

**P3 — substantial state machines.** All closed, and three of them are what the docs site itself
runs on: the sidebar is the nav, the command palette is ⌘K, and the data table is the only honest
way to find out what living with a generic row type is like.

- [x] Command — unmatched items are not rendered, so document order *is* arrow order. The highlight
      moves through `aria-activedescendant`, which is why it could not reuse `roving_focus`.
- [x] Combobox — the command palette inside a floating layer. `ComboboxItemRoot` is the whole
      bridge: it writes the value and closes the layer.
- [x] Navigation Menu — links rather than commands, so leaving is decided at the menu, not per
      trigger.
- [x] Menubar — a row of dropdown menus with one bar over them, reusing the dropdown's surface,
      items and submenus whole.
- [x] Resizable — percentages, never pixels, with `flex-grow` *being* the percentage.
- [x] Carousel — scroll snapping rather than a transform, so a trackpad and a touch drag come free.
- [x] Calendar — over `utils::date`, so there is no `chrono` for consumers to agree with. Focus
      follows a *date*, not a cell, which is what steps off the end of a month page.
- [x] Date Picker — a popover holding a calendar, plus the one rule that makes it a picker: the
      layer closes when the selection is finished.
- [x] Table sort/selection handles, then Data Table — plain `Copy` handles rather than contexts, so
      they work over a hand-laid-out table too.
- [x] Chart — line, area and bar as one `<svg>`, geometry in viewBox units, so nothing is measured
      and there is no resize listener.

**P4 — app-specific pieces.** Useful only once the core set is solid; several are chat/AI surfaces
rather than general UI, so they are the easiest to defer or skip.

- [ ] Direction (RTL provider; cheap, but only pays off once enough components read it)
- [ ] Marker
- [ ] Attachment
- [ ] Bubble
- [ ] Message
- [ ] Message Scroller
- [ ] Questionnaire

**Beyond the shadcn catalogue.** Ours, and none of them took a dependency.

- [x] `primitives/drag.rs` — `use_drag`, with `.against()`, `.on_start` and `.on_end`. `slider`,
      `scroll_area`, `drawer` and `resizable` are folded onto it: 83 lines of listener bookkeeping
      gone, and all four now end on `pointercancel`, which every hand-rolled copy missed.
- [x] `utils::color` — `Rgba` / `Hsva` / `Hsla` / `Oklch`, a parser for every CSS form this library
      writes, a formatter, 9 tests. HSV↔HSL is exact where anything through 8-bit RGB is not, which
      is why a picker stores `Hsva`; and `--chart-1` does not fit in sRGB, so reading one of the
      theme's own tokens back clips it a few degrees round the hue circle.
- [x] `utils::image` — a normalised crop rectangle, `moved_by`, `resized` holding an aspect ratio,
      and a canvas for the output. 12 tests.
- [x] `utils::gif` — cropping an animated GIF at the LZW level, so palettes, delays, disposal,
      transparency and the loop count survive by never being decoded. 10 tests plus a browser check.
- [x] Color Picker — holds HSVA, so the hue survives the black corner. The square is two gradients
      over a pure hue; the two rails are one component twice.
- [x] Image Cropper — a still through the canvas, an animated GIF through `crop_gif` and still
      animating afterwards: 16 frames in, 16 frames out, checked with Chrome's own decoder.
- [ ] Kanban Board — consists of 3 components: a `Board` that lays out columns, a `Column` that lays out cards, and a `Card` that
      is a draggable item. The board is a `use_drag` target, so the whole thing can be moved around
      in a larger layout; the column is a `use_drag` target, so cards can be moved between columns;
      and the card is a `use_drag` source, so it can be moved within or between columns. The board
      and column are both `use_drag` targets, so they can be moved around in a larger layout.
- [ ] Floating Menu — a draggable menu that can be moved around the screen. It is a `use_drag` target, so it can be moved
      around in a larger layout; it is a `use_drag` source, so it can be moved within or between
      columns; It may also support fixed positioning, so it can be anchored to a specific point
      on the screen, controlled and uncontrolled.
- [ ] Timeline — Vertical/Horizontal events
- [ ] Form — A context that holds form state, made to work with `noform`
- [ ] Dropzone - A component that allows users to drag and drop files onto it, with support for multiple
      files and file type validation.
- [ ] Add a `tree-view-mode` to `sidebar`, allowing for a tree view of the sidebar items, with support for expanding
      and collapsing items, and keyboard navigation.
- [ ] Video player - A component that allows users to play videos, with support for multiple video formats and playback controls.
  - [ ] View Modes:
    - [ ] Full: All components are visible and inline
    - [ ] Reduced: Only Play/Pause, Next/Prevous, Volume and Time bar are visible
    - [ ] Limited: Only Play/Pause and Time bar are visible
    - [ ] None: Only the video is visible, with no controls, clicking on the video toggles play/pause.
    - [ ] Uncontrolled: The video is visible, but no controls are visible, and the user cannot interact with the video.

**Unfinished corners of what is shipped.**

- [x] `ToastBuilder::action(label, on_press)` — "deleted · undo". The callback is a plain
      `Arc<dyn Fn>` rather than a `Callback`, because what raises such a toast is on its way out.
- [x] A `Field` knows when it has been left: `FieldContext.touched`, `data-touched` on the field,
      `touched` on `FieldControl`, set from a `focusout` with the related target deciding — `blur`
      does not bubble and a styled control may wrap itself in a box. `NativeSelect` also takes an
      `on_blur` for a caller who is not using a field.
- [x] `ComboboxItem` takes `selected` and `close_on_select`: a multi-select is a list the caller
      keeps, not a mode the combobox has.
- [x] `utils::theme` is three axes rather than one: light/dark stays a `dark` class, the palette
      is a name on `data-theme`, so a theme has a light and a dark of its own, and an accent is a
      name on `data-accent` — one hue picked *inside* whichever palette is in force, which is
      fourteen names in a stylesheet rather than fourteen more palettes. `ThemeContext` gained
      `color`, `accent` and a `resolved` that is never `System` — which had to become live, so the
      media query is listened to rather than read: `use_media_query` re-reads on `resize`, the
      one event a colour-scheme change never fires.
- [x] A fourth axis, and the one that is not a name: a custom corner radius. There is nothing to
      enumerate, so `ThemeContext::radius` is a CSS length written into `--radius` on `<html>`'s
      own `style` — the one place a `[data-theme]` rule cannot outrank — and the switcher offers
      it as a slider over the palette's own. Clearing it hands the radius back to the palette.
- [x] An accent moves the *tinted surface* too — `--accent` / `--sidebar-accent`, which is hover,
      a focused item and the active sidebar row, and is unfortunately not the accent hue. Derived
      from the accent at a fixed lightness and a low chroma, which is what Grove and Ember were
      writing out by hand. Without it a palette with a green hover kept it while the brand went
      orange, which is the half-changed look the axis exists to avoid.
- [x] The palette rows depend on the mode: Latte is the light half of all three Catppuccin
      flavours, so light offers one row saying Latte and dark offers three. A row that stands for
      several names keeps whichever is in force rather than silently choosing a dark half.
- [x] Which way the text on an accent goes is the accent's own lightness, not the mode's — Latte's
      yellow is lighter than Mocha's blue. `--primary-foreground` reads the L back out with
      relative colour syntax and clamps it to a black or a white, so it is one line rather than
      fourteen judgements per flavour.
- [x] The docs' theme switcher is one popover with the four axes in it — three columns and a
      slider — and the choice in force actually marked, rather than a menu you reopen twice. Writing it turned up that
      `attr:data-theme` on a *native* `<span>` renders nothing — the palette dots had all been
      showing the same colour.
- [x] `FieldSet` / `FieldLegend`, and `FieldSetControl` beside `FieldControl`: a field labels one
      control with `for`, a set is labelled the other way round with the group pointing
      `aria-labelledby` at the legend. A real `<fieldset>` for the one thing a `role="group"`
      could not borrow — `disabled` on it disables everything inside. `RadioGroup` reads it, and
      a description or an error under a fieldset registers on the set rather than on a field.
- [x] `RadioGroupItem` was the only control in its family that never built a `FieldControl`, so a
      `FieldLabel` beside one pointed `for` at an id nothing wore. Found writing the fieldset.
- [x] `highlight` reads the three shapes it was missing: a call is `TokenKind::Function`, a JS
      regex is a literal rather than punctuation plus contents — decided by whether a value could
      begin there, which is why the loop remembers its last token — and `${…}` is cut out of a
      template literal and tokenized as the code it is. Capitalisation still makes a type, which
      in both languages is the convention rather than a guess.
- [x] `utils::date::DateNames` reads a locale's names out of `Intl.DateTimeFormat` — `js_sys`,
      which `Date::today` was already reaching into. Resolved once per calendar, since a grid
      reads a weekday name 42 times, and the captions are *formatted* rather than assembled,
      because the order of the parts belongs to the locale too (`2026年8月`). No `Intl` and an
      unreadable tag both fall back to English. `Calendar` and `DatePicker` take a `locale`.
- [x] `utils::color` reads all 148 colour names, and `Rgba::name` reads one back for a swatch that
      would rather say `rebeccapurple`. A sorted table of packed `u32`s, searched only once the
      shapes with punctuation in them are ruled out; `transparent` is matched ahead of it, being
      the one keyword with an alpha.
- [ ] `ColorPicker` should use a `Select` instead of a button to switch colors, and instead of
      a unified input, using a divided input (with `InputGroup`) with a `Select` to switch formats
  - 3 for rgb
  - 4 for rgba
  - 3 for hsl
  - 4 for hsla
  - 4 for hsva
  - 4 for oklch
- [ ] `highlight` is still a lexer per language: a new one is a new scanner beside `tokenize_rust`,
      and the six that exist share only their helpers. Fine at six; worth a second look at ten.
- [ ] `navigation_menu` has no shared viewport: each panel is absolutely positioned against the
      menu, so they all open in the same place, but the box does not slide or resize between them.
      That needs the active panel measured and the size animated.
- [ ] `chart` is line, area and bar over a categorical x axis. No stacking, no second axis, no time
      scale, no pie — each is more arithmetic in `primitives/chart.rs`, not new machinery.
- [ ] `chart` draws every label it is handed, and the tooltip reads that same list, so a long series
      cannot be thinned by blanking labels without leaving the tooltip with nothing to say. The
      caller's way out is fewer, wider buckets (the chart block does exactly that). A `labels` /
      `tick_labels` split, or drawing every nth, would make the chart's own answer possible.
- [ ] `data_table` has no pagination of its own; it shows what it is handed. `Pagination` is
      shipped and the two compose — the data-table block demonstrates it, sorting the whole set
      before slicing a page out of it.
- [ ] `utils::gif` re-emits every frame at its clipped size rather than diffing consecutive frames
      for a minimal changed rectangle, so a cropped GIF is a little larger than it needs to be. The
      sample goes 17.5kB → 7.3kB on a crop to a third of the area, which is fine but not optimal.
- [ ] `docs/assets/avatar.jpg` is a 6.9MB PNG with a `.jpg` name. It is `include_bytes!`d, so all
      of it lands in the wasm binary — worth re-encoding to a few hundred kilobytes and renaming.
- [ ] The colour picker's square announces itself as `role="application"` with a description of the
      keys. There is no honest ARIA role for a two-axis drag; the rails and the text field are the
      paths that announce themselves properly.

## Docs site

- [x] Docs pages for alert-dialog, label and spinner; a `code` snippet under every demo (89 blocks
      across 26 pages), extracted from the demo markup so the two cannot drift.
- [x] Prop tables on all 47 component pages — 199 parts, one table each, under one API reference
      heading.
- [x] Blocks. `docs/src/pages/docs/blocks/` is a module per component behind one parameterised route
      and a `REGISTRY` keyed by slug, so a block is a file and a line.
- [x] Twelve blocks: a colour picker in a popover, a palette handing over to an alert dialog, a
      table that sorts before it pages, an avatar cropper that keeps a GIF moving, an application
      shell, a settings dialog with the sidebar inside it, a form that waits before it complains, a
      dialog that becomes a drawer, a range over one set of numbers, a multi-select of people, a
      delete you can take back, and right-click actions on a table row. Three of the last four
      needed library work first, and two of those turned up bugs — which is the argument for
      writing them.

- [ ] Blocks for the rest, in rough order of what would teach the most: **select** — a dependent
      pair where the second is filtered by the first; **resizable** — a two-pane layout that
      remembers its split; **menubar** — an application menu with the keyboard doing the work;
      **carousel** — slides that are links, so tab order and scroll snapping have to agree;
      **date-picker** — a range across two months, which is the one the calendar's API has not been
      asked for yet.
- [ ] Snippets are extracted, not curated. A few still carry the demo's layout wrapper (`<div
      class="flex gap-3">`). Worth a pass to trim the ones where it adds nothing.
- [ ] Dead nav links in `docs/src/layout/doc_layout.rs`: `/docs/introduction` and
      `/docs/installation` have no route and fall through to "Not Found."
- [x] `/docs/theme`: the three axes, what each lands on `<html>` read back live, the tokens and
      the radius scale in a row of its own, and the CSS a consumer writes to define a palette or
      an accent.
      Catppuccin is three palettes rather than four — Latte is the light half of all three, which
      is how the flavours ship everywhere else and what keeps the mode axis meaning one thing.
      Values generated from `catppuccin/palette`, in oklch, rather than eyeballed.

- [ ] The header link between a component and its blocks is a stopgap. A component page should open
      onto a *switch*: **Component**, **Blocks**, and later **Primitives** — one page, three views
      of it. The routes are already shaped for it, so it is a `Tabs` in `DocLayout` driven by the
      path rather than a restructure.
- [ ] Adding a component means editing five files, two of which (`NAV`, `COMPONENTS`) are
      hand-maintained copies of the same list — which is how `/docs` came to be missing progress,
      skeleton and textarea. Drive routes, nav and the index from one const.
- [ ] The primitives have no docs pages at all: every page documents the styled layer, and the
      behaviour layer — the half a Radix-style library is sold on — is undocumented. The same table
      treatment at `/docs/<slug>/primitives`, as the third view of the switch above, rather than a
      second tree in the nav: nobody looks up `DialogContentRoot` except while reading about
      `Dialog`.
- [ ] A `/docs/types` page under Sections: the shared types the prop tables keep naming —
      `Signal<String>`, `ChildrenFn`, the `render` callback shape, `Side` / `Align` / `Orientation`
      / `SideOffset` — described once, so a table can point at them instead of re-explaining them.
- [ ] Generate the prop-table rows at build time rather than keeping 199 hand-pasted tables in step:
      an `xtask` or a build script over the `#[component]` signatures, emitting the same `ApiEntry`
      shape. It could drive `NAV` and `COMPONENTS` too, closing the five-files item above.

## Publishing

- [x] One feature per element (64 of them), gating both layers under one name, each listing what its
      own source imports so cargo works the closure out. `tests/features.rs` re-derives the table
      from the imports and fails if `Cargo.toml` has drifted — the failure it prevents is silent,
      since `--features full` builds either way.

- [ ] `Cargo.toml` has no `description`, `license`, `repository`, `keywords` or `categories` —
      `cargo publish` will reject it as-is.
- [ ] No LICENSE file, though the README credits shadcn/ui and Radix (both MIT).
- [ ] `leptos` is pinned to `features = ["csr"]`, which forces CSR on every consumer.
- [ ] Consumers still have no way to get the CSS: the tokens, `@custom-variant`s and `animate.css`
      live in `docs/styles/` only. The styled layer is now written to be copied into a project
      rather than depended on — what is missing is the thing that does the copying, and installs
      the stylesheet with it.
- [ ] `cargo doc` over the styled layer is a list of bare signatures. Its prose is the docs site's
      prop tables now, and a description written in both places drifts, so `src/components` carries
      no `///` on purpose — docs.rs stays empty until it can be generated from the same source as
      the tables. `src/primitives` and `src/utils` keep theirs, being undocumented elsewhere.
- [ ] README is a disclaimer — needs install steps, a minimal example and the feature-flag table.

## Repo hygiene

- [x] CI in `.github/workflows/ci.yml`: `fmt --check`, `clippy --features full -D warnings`,
      `check -p docs`, `test --features full`. `pages.yml` deploys the docs site on every push to
      main — needs Settings → Pages → Source: GitHub Actions once.

- [ ] Nothing installs a panic hook, so a panic in the docs app surfaces as
      `RuntimeError: unreachable` with no message and no line. It cost an afternoon on the toast
      disposal bug, where the panic was real and the only way to find it was bisecting the call.
      Three lines in `docs/src/main.rs`.
- [ ] Almost no tests over what renders. `src/utils` is covered and `tests/features.rs` checks the
      feature table; nothing exercises a component. `wasm-bindgen-test` smoke tests over the primitives' `data-state` transitions
      would catch regressions in the floating and toast state machines — the toast one has now been
      wrong twice.
