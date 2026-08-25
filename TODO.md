# TODO

Working list for `nocomponents`. Status reflects the tree as of 2026-08-25; check items off as they
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

- [x] The ⌘K palette navigated to `/base/base/docs/…` on the Pages build: `use_navigate` resolves
      against the router's base and was handed a path `href()` had already prefixed. `A` is the
      other way round, which is why only the palette broke — and why `trunk serve` never showed it.
- [x] Switching pages reset the sidebar's scroll to the top. Every page rendered its own
      `DocLayout`, so navigating rebuilt the whole sidebar; the chrome is now `DocShell` on a
      `ParentRoute` at `/docs` and only the `Outlet` swaps.
- [x] The sidebar is on `utils::use_media_query`, and `SidebarContext::is_mobile` is a `Signal`
      rather than an `RwSignal` — derived from the viewport, so nothing but the window decides it.
      Not the drop-in it looked: the old `resize` handler did two things, and closing the mobile
      overlay when the breakpoint is crossed is now its own effect, keyed on the *change* rather
      than the value so it cannot shut an overlay the reader just opened.
- [x] A floating trigger overwrote a caller's own `id`. It now adopts an id already on the node as
      `trigger_id` instead, so a `<label for>` still names the trigger and everything downstream —
      the field's `control_id`, the content's `aria-labelledby` — follows the caller's id rather
      than a minted one. A trigger with no id of its own still gets one.
- [x] A menu item can be disabled. `DropdownMenuItemRoot` takes `disabled`, writes `data-disabled`
      and `aria-disabled`, and refuses the click in the handler rather than trusting the styled
      layer's `pointer-events-none` — so an item styled by someone else refuses it too. One root, so
      the dropdown, the context menu and the menubar all got it; `DropdownMenuItem` and
      `MenubarItem` pass it through.
- [x] The `<select>` is `NativeSelect`'s own root, so `attr:id`, `attr:name` and any event land on
      the control instead of on a wrapper. The chevron is a background image, since a pseudo-element
      does not render on a `<select>` and there is no way to mask one to a token: two baked greys,
      light and dark, which are the values `--muted-foreground` has in both default palettes. A
      custom palette's own muted-foreground will not reach it — the price of the wrapper going.
      `on_blur` stays as a convenience, but its reason for existing is gone and its doc says so.
- [x] `DropdownMenuTrigger` and `PopoverTrigger` build their `Button` as the `render` the root
      already takes, instead of around `ctx.trigger_ref` behind the root's back. They were doing
      what `ComboboxTrigger` does — it was never the only one, and `SelectTrigger` was the only one
      doing it right. Found by the `data-slot` work: a trigger that skips its root gets nothing the
      root writes onto the node.
- [x] `ComboboxTrigger` goes through its root too, and the duplicated `field_control_for_trigger`
      is gone with it — the root does the field wiring once and hands the resolved `disabled` back.
      What made that possible: the trigger callback is a **`TriggerRender` struct** now, not a
      tuple. `(AnyNodeRef, Callback<MouseEvent>)` had to grow a third element, and a tuple has to
      be re-broken every time it does; a struct takes a new field and leaves every caller alone.
      Verified where it matters — a combobox inside a `<Field disabled=true>` still comes out
      disabled, which is exactly what routing it through the root would otherwise have dropped.
- [x] `FloatingTrigger` takes a `data_slot`, written onto the node rather than into the markup —
      the trigger is its own `<button>` in one place and whatever `render` returned in another, and
      the node ref is all the shapes have in common, so one path covers both. `dropdown_menu` and
      `select`, which had no `data-slot` anywhere, now carry one on every part they own: the
      trigger, the content, the item, and the submenu's trigger and panel.
- [x] Every component carries a `data-slot` now. The audit found twelve with none at all — badge,
      button, checkbox, data_table, date_picker, input, label, progress, spinner, switch, textarea,
      toast — which was most of the ones people reach for first. The rule: the primitive writes it
      where there is one, the styled layer writes it for the style-only components.
- [ ] `primitives::button::ButtonRoot` is rendered by nothing, `Button` included — the styled one
      writes its own `<button>`. Found because a `data-slot` put on the root never appeared. It is
      the same shape as the trigger bypass, and the same question: make `Button` render it, or
      admit the primitive is dead and delete it.
- [x] All three render callbacks are named structs: `TriggerRender` for the floating triggers,
      `StyledRender` (class + node ref) for `Button` and `SidebarMenuButton`, `AnchorRender` (node
      ref alone) for the tooltip and hover card. Kept separate rather than merged into one: a
      tooltip trigger has no click to run and no classes to wear, and handing it fields that mean
      nothing where it stands is worse than three honest shapes. Each can now grow on its own.

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
- [x] `kanban` — a board of columns of cards, on `use_drag` like every other gesture here, so it
      ends on `pointercancel` and no card is left stuck to the cursor when the browser takes the
      pointer away.

      **The board does not own the cards.** It reports `on_move` — the card, the column it left,
      the column and index it landed at — and the caller applies that to their own list. Same
      stance as `DataTable` sorting what it is handed: a board that reordered a list it did not own
      would disagree with the caller's the moment anything else touched it.

      Where a card would land is read out of the DOM (`data-kanban-column`, `data-kanban-card`)
      rather than from a registry the parts keep in step, so columns and cards can come and go
      without telling anybody. The card in hand is skipped while counting, or the index it came
      from reads one too far down. A card let go where it started reports nothing.

      The card lifts and follows the pointer, by `transform` rather than by leaving the list: the
      gap it came from stays open, so no column reflows mid-gesture and the slot under the pointer
      does not shift as the card moves. It goes `pointer-events: none` while it travels, so what
      is under the cursor is whatever it is over rather than itself. The transform is never
      transitioned — easing a thing that is following a cursor makes it lag by the duration. The
      line showing where it will land is a `KanbanSlot` the caller renders between its own cards,
      because only the caller knows where "between" is.
- [ ] Floating Menu — a draggable menu that can be moved around the screen. It is a `use_drag` target, so it can be moved
      around in a larger layout; it is a `use_drag` source, so it can be moved within or between
      columns; It may also support fixed positioning, so it can be anchored to a specific point
      on the screen, controlled and uncontrolled.
- [ ] Timeline — Vertical/Horizontal events
- [ ] Form — A context that holds form state, made to work with `noform`
- [x] `dropzone` — a drop target and the picker behind it, arriving as one `Vec<File>` either way;
      a caller who had to tell them apart would be a caller writing two paths for one thing.
      `accept` uses the attribute's own grammar and is checked on **both** ways in, which is not
      what it shipped as: the pick path trusted the file dialog, and a dialog's `accept` only
      decides what it *offers* — "All files" walks a .md straight past it. A reader picked one and
      it was taken. Both paths partition now, and both report on `on_rejected`. The matcher is pure and has five tests, including the
      case that motivates it — a dropped file often carries no MIME type at all, so `.pdf` has to
      match on the name.

      Two things the browser makes you know: `dragover` must be prevented or the drop is refused
      *silently*, and `dragleave` fires when the pointer crosses onto a child — so leaving is a
      counter reaching zero rather than one event, or a zone with a label in it flickers.

      It also turned up the only `use web_sys::` in the styled layer, which would have broken an
      install: `src/components/` may name leptos and `deps`, nothing else. `leptos::web_sys::File`
      instead.
- [x] `tree` — its own element rather than a mode on the sidebar, because a tree is not a sidebar
      feature: it is a thing a sidebar can contain, and a file picker or a settings pane wants the
      same one. `role="tree"`, one tab stop, `aria-level` per depth, and `aria-expanded` only on
      the items that have children — which is not knowable until the children have rendered, so
      `TreeGroupRoot` is what tells its item it is a branch. A closed group renders nothing rather
      than hiding, so "the next visible item" is just the next one in the DOM and the roving focus
      needs no notion of visibility. Enter and Space are routed through the row's own click, so the
      pointer and the keyboard take one path.

      Four things it shipped wrong first, all invisible to the compiler: the keys were on the row,
      which is a *child* of the focused item and so never saw them; `attr:aria-level` on a native
      element renders nothing, so every item was depthless; nothing called `sync_tab_stop`, so the
      tree could not be tabbed into at all; and — the one a reader spotted before I did — **a
      named Tailwind group matches any ancestor that carries it**, so `group/tree-item` on an item
      that nests inside another item lit up every descendant. The chevron of a closed folder inside
      an open one was permanently rotated, and a focused branch ringed every row beneath it.
      Both now key off the row, which does not nest, and the focus ring is a direct-child variant
      (`[[data-slot=tree-item]:focus-visible>&]`) rather than a group at all.

      **Nesting a component inside itself makes `group/name` unusable for its own state.** Worth
      remembering for the kanban board and anything else recursive.
- [x] The sidebar half: `SidebarTree` / `SidebarTreeItem` / `SidebarTreeRow` / `SidebarTreeGroup`,
      the same primitives with the sidebar's classes rather than the tree's own — one behaviour,
      two styled layers, which is the split the library is built on. The rows take
      `SidebarMenuButton`'s look and the group takes `SidebarMenuSub`'s rule, and the whole thing
      folds away with `group-data-[collapsible=icon]:hidden`, a tree of labels having nothing to
      show at that width. `sidebar` gained `tree` as a feature edge.
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
- [ ] `ButtonSize` cannot size the icons inside the button, and the rules that were meant to are
      dead code. Every size writes `[&_svg:not([class*='size-'])]:size-3` or similar, and the base
      writes `size-4` — but **every icon in this library renders with `size-4` already**
      (`IconRoot` is `cn!("size-4", class.get())`), so `:not([class*='size-'])` never matches and
      an icon in an `Xs` button stays 16px in a 24px button. The rules are inherited from shadcn,
      where a lucide icon carries no default size and the guard does bite.

      Dropping the guard is not the fix: `[&_svg]:size-3` is a descendant rule and would outrank a
      caller's own `size-6` on the icon. Dropping the icons' default leaves a bare icon with no
      size at all. The shape that works is a variable — the button sets `--icon-size` per size,
      `IconRoot` defaults to `size-(--icon-size,1rem)` — so the button decides, and a caller who
      writes a size on the icon still wins on the icon's own class.

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

- [x] `/docs/utility`, filling the second dead nav link: `cn!`, the common types, `next_id`,
      `use_media_query`, `get_placement`, and the arithmetic modules — dates, colour, crops, GIFs,
      the tokenizer — each with the reasoning the module's own `//!` header carries.
- [x] An "On this page" rail (`docs/src/components/page_nav.rs`) on every page wide enough for it.
      It reads headings marked `data-toc` out of its own `<main>`, so `DemoSection` and
      `ApiReference` opt in once and no page has to list anything.
- [x] A kanban block: columns you add, cards you add, and two ways to move one — drag it, or
      right-click and pick a column. Both end in the same closure, `on_move` handing back a
      `KanbanMove` and the menu building one by hand, so the two cannot come to disagree about what
      moving means. The card is the context menu's trigger, which works only because the drag
      guards on the primary button. Cards are keyed by an id rather than by their title: two cards
      called the same thing would be one card that teleports.
- [ ] Blocks for the rest, in rough order of what would teach the most: **select** — a dependent
      pair where the second is filtered by the first; **resizable** — a two-pane layout that
      remembers its split; **menubar** — an application menu with the keyboard doing the work;
      **carousel** — slides that are links, so tab order and scroll snapping have to agree;
      **date-picker** — a range across two months, which is the one the calendar's API has not been
      asked for yet.
- [ ] Snippets are extracted, not curated. A few still carry the demo's layout wrapper (`<div
      class="flex gap-3">`). Worth a pass to trim the ones where it adds nothing.
- [ ] Dead nav link in `docs/src/layout/doc_layout.rs`: `/docs/introduction` has no route and falls
      through to "Not Found." `/docs/installation` is a page now.
- [x] The form-state page is back, on a **git** dependency. It came off main because `docs` named
      `noform` by path (`../../noform`, a sibling clone that exists on one machine): cargo will not
      load a workspace whose member has an unreadable manifest, so every CI job died inside
      `cargo metadata` — `cargo fmt --all` first among them — before a compiler ran. Marking the
      dependency optional does not help; the manifest is read either way. Nothing local catches it
      either, since the path resolves where it was written; the check that does is a copy of the
      tree somewhere without the sibling.

      **A path dependency outside this repo must not land on main.** To work against a local
      `noform`, override it in an untracked `.cargo/config.toml`, not by editing `docs/Cargo.toml`.
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
- [ ] The Leptos-shaped types the prop tables keep naming — `Signal<String>`, `ChildrenFn`, the
      `render` callback shape — still have nowhere to be described once. `Side` / `Align` /
      `Orientation` / `SideOffset` are on `/docs/utility` now, so this is the other half of that
      page rather than a `/docs/types` of its own.
- [ ] Generate the prop-table rows at build time rather than keeping 199 hand-pasted tables in step:
      an `xtask` or a build script over the `#[component]` signatures, emitting the same `ApiEntry`
      shape. It could drive `NAV` and `COMPONENTS` too, closing the five-files item above.

## Publishing

- [x] Consumers have no way to get the CSS. Answered by not shipping the styled layer as a crate at
      all: a Tailwind build cannot see classes inside `~/.cargo/registry`, so `cargo nocli` installs
      `src/components/<name>.rs` into the project's own `src/` and `init` installs the stylesheet
      beside it. The crate keeps everything with no classes in it.
- [x] One feature per element (64 of them), gating both layers under one name, each listing what its
      own source imports so cargo works the closure out. `tests/features.rs` re-derives the table
      from the imports and fails if `Cargo.toml` has drifted — the failure it prevents is silent,
      since `--features full` builds either way.

- [x] Both manifests carry `description`, `license`, `repository`, `homepage`, `keywords` and
      `categories`; `cargo publish --dry-run` is clean for each.
- [x] Apache-2.0, with the shadcn/ui and Radix MIT notices reproduced in `NOTICE` as their licence
      requires. `LICENSE` is also copied into `cli/`, which is packaged separately.
- [x] `leptos` no longer carries `features = ["csr"]`, so the consumer picks the mode. Checked
      against `ssr`, `hydrate` and `csr` from a scratch crate — all three compile. Compiling is not
      running: nothing here has been *exercised* under hydration, and the primitives call
      `document()` directly, so an SSR consumer is unexplored rather than supported.
- [ ] The CLI is not published, and `cargo add nocomponents` inside it assumes the library is. Until
      both are on crates.io the only working path is `--from <checkout>`, which is also how the CLI
      is tested.
- [x] `nocomponents.lock` beside the config records a hash of each file as installed, so `add` now
      says *which* kind of file it left alone — "edited here" against "unchanged, --force refreshes
      it" — and `--force` names what of yours it overwrote. FNV-1a, not a cryptographic hash: the
      question is only whether a file is byte-for-byte what we wrote, and `\r` is skipped so a
      checkout on Windows does not read as edited.
- [x] `components remove`, which refuses twice — once for a component another installed one is
      built on (found by searching the installed files for the rewritten import, so there is no
      list to keep in step), and once for a file the project has edited. `--force` overrides both,
      and doing so is what broke the scratch project during testing, which is the argument for the
      refusal.
- [x] The installed `mod.rs` is written between `// nocli:begin` and `// nocli:end`; anything
      outside them survives. A module the project declares itself out there is skipped inside the
      block rather than declared twice — the block lists every `.rs` in the directory, their own
      files included.
- [ ] `init` installs the whole of `globals.css`, which is four palettes and fourteen accents —
      most of a consumer's stylesheet, for tokens they mostly will not use. Worth splitting the
      base tokens from the palettes so a project takes one and opts into the rest.
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
- [ ] Almost no tests over what renders. `src/utils` is covered, `tests/features.rs` checks the
      feature table, and `cli/` has unit tests over the import rewriting; nothing exercises a
      component. `wasm-bindgen-test` smoke tests over the primitives' `data-state` transitions
      would catch regressions in the floating and toast state machines — the toast one has now been
      wrong twice.
