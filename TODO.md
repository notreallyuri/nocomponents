# TODO

Working list for `nocomponents`. Status reflects the tree as of 2026-08-20; check items off as
they land and add new ones rather than keeping a parallel list elsewhere.

## Bugs

- [x] **Dialog content rendered with a stale `data-state`, causing a flicker on open.** Fixed in
      `src/primitives/dialog.rs` by scoping the context with `<Provider value=context>` instead of a
      bare `provide_context`.
      Root cause: `DialogPortalRoot` mounts children through leptos' `Portal`, which renders under an
      owner of its own (`leptos-0.8.19/src/portal.rs:68` calls `mount_to`). A context provided on the
      component's own owner is not reachable from there, so `use_dialog()` inside the portal resolved
      to a *different* dialog's context — in practice the last `DialogContext` provided on the page.
      With three dialogs on `/docs/dialog`, the first dialog's content read the third's `is_open`
      (still `false`) and rendered `data-state="closed"` while open, so `animate-out` ran on mount:
      the content faded to 0 over ~100ms, then snapped back to visible when the animation ended.
      It was never controlled-vs-uncontrolled — every dialog *except the last one on the page* was
      broken, and the controlled demo simply happens to be last. `alert_dialog` shares the primitive
      and is fixed by the same change. The floating primitives were never affected because
      `FloatingRoot` already wraps its children in `<Provider>` (`src/primitives/floating.rs:99`).
      Verified over CDP: content now resolves its own signal and plays a single `enter` animation
      (opacity 0 -> 1, scale .95 -> 1) for all three dialogs on the page.

- [x] **`fill-mode` fallbacks in `animate.css` were dead code.** Fixed in
      `playground/styles/animate.css` by dropping `initial-value: none` from the
      `@property --tw-animation-fill-mode` declaration.
      Root cause: a *registered* custom property resolves to its initial value rather than to the
      `var()` fallback, so declaring one made every `var(--tw-animation-fill-mode, …)` below it
      unreachable — `--animate-in` / `--animate-out` computed `none` instead of `forwards`, and any
      element without an explicit `fill-mode-*` class snapped back to its natural style the moment
      an exit animation ended. Left unset (the same trick `--tw-animation-duration` already used two
      declarations above), each usage site gets the fallback it asks for: `forwards` for
      `enter`/`exit`, `none` for the accordion and collapsible animations. Tailwind still emits the
      `@property` rule, and the legacy `@layer properties` block now sets `initial` rather than
      `none`, which is guaranteed-invalid and so also falls through to the fallback.
      Verified over CDP: dialog overlay and content both compute `animation-fill-mode: forwards`,
      and popover/dropdown content (previously `forwards` only because `PopoverContent` set the
      class by hand) is unchanged.

- [x] **The dialog now plays its exit animation.** `DialogPortalRoot`'s `Show` gates on
      `is_mounted` instead of `is_open` (`src/primitives/dialog.rs`), so the content survives the
      150ms `unmount_delay` that `DialogRoot` was already computing and nothing read. Two things
      came with it:
      - the pending unmount is held as a `TimeoutHandle` and cancelled when the layer reopens
        inside the delay, so the timer from the previous close cannot undo it. `FloatingRoot`
        (`src/primitives/floating.rs`) had the same race and took the same treatment; both also
        cancel on `on_cleanup` (see the disposal panic below).
      - the overlay and content take `data-[state=closed]:pointer-events-none` in
        `src/components/dialog.rs` and `alert_dialog.rs`; without it the fully transparent
        full-screen overlay swallowed every click for those 150ms. `popover`, `select` and
        `dropdown_menu` have the same hole and always have — see below.
      Verified over CDP: opacity runs 1 → 0 over the 100ms `duration-100`, holds at 0 (thanks to the
      fill-mode fix above), unmounts at ~150ms, and a click on the trigger during the exit window
      hits the trigger and reopens the dialog.

- [x] **Closing floating content swallowed clicks for `unmount_delay`.** `popover`, `select` and
      `dropdown_menu` keep their content mounted for 150ms after close, and anything under the
      fading panel was unclickable for that window. Their dismissal overlays were never the
      problem — those already switch to `hidden` on close — but the panel itself, and the fixed
      positioning shell around it, both stayed hit-testable.
      Fixed the way Radix does: the positioning shell in each primitive is permanently
      `pointer-events-none`, and the styled content re-enables itself with `pointer-events-auto`
      plus `data-[state=closed]:pointer-events-none`. Verified over CDP that `elementFromPoint`
      over a fading menu now returns the page underneath, that the submenu still opens on hover and
      stays open while the pointer is inside it (`pointerenter`/`pointerleave` still fire on an
      inert ancestor, because boundary events are dispatched along the hit target's chain), that a
      select option still applies, and that a popover's inputs still take clicks and typing.

- [x] **Selecting a submenu item left the parent menu open.** `DropdownMenuSubRoot` wraps its
      children in a `FloatingRoot` of its own, which shadows `DropdownMenuContext`, so
      `DropdownMenuItemRoot`'s `ctx.close()` inside a submenu only ever closed the submenu.
      `DropdownMenuRoot` now also publishes its context as a private `MenuRoot` type that submenus
      do not overwrite, and an item closes both. Found while regression-testing the change above.

## Accessibility & keyboard (blocking a "stable" tag)

- [x] Escape-to-close, via a shared layer stack (`src/primitives/dismiss.rs`). A per-root key
      listener would have closed every open overlay at once — Escape in a popover inside a dialog
      would take the dialog with it. Instead every dismissable root calls `use_dismissable_layer`,
      which keeps it on a thread-local stack ordered by the moment it opened; one document-level
      listener dismisses only the topmost entry. Layers are keyed on the `is_open` signal, so a
      layer leaves the stack when it closes rather than when its exit animation finishes.
      `DialogRoot` dropped its private listener for it and `FloatingRoot` gained it, which covers
      popover, select, dropdown menu and submenus in one place.
      Verified over CDP: Escape closes popover, select, dialog (uncontrolled and controlled) and
      menu; in a menu with an open submenu the first Escape closes only the submenu and the second
      closes the menu; a stray Escape with nothing open is a no-op; and reopening after Escape
      leaves nothing stale on the stack.
- [x] Arrow-key navigation, on a shared roving-focus helper (`src/primitives/roving_focus.rs`)
      plus a typeahead helper beside it (`src/primitives/typeahead.rs`).
      Items are found in the DOM by a `data-roving-item` marker rather than kept in a registry, so
      the helper does not care how a component renders its children — portalled, wrapped,
      conditional or reordered — and document order is what the arrows follow. Disabled items are
      skipped. The tab stop starts on the selected item (`[data-state=active|checked|on]`,
      `[aria-selected=true]`) when the group has one, so Tab lands on what the group currently
      represents, and moves with focus after that: a group is one tab stop, not one per item.
      Wired into four components, each keeping the convention its own control has:
      - **Dropdown menu** — opening focuses the menu container, not the first item, so opening with
        the mouse paints no focus ring; arrows step in from there. Enter/Space activate (items are
        divs, so the key press is turned into the click a button would get for free), Tab closes,
        Right opens a submenu and steps into it, Left closes it and returns to its trigger.
      - **Select** — opens *on* its current value the way a native select does, plus typeahead:
        keystrokes accumulate for a second, a repeated letter cycles through the items starting
        with it, Enter commits.
      - **Tabs** — orientation now lives in `TabsContext` (also rendered as `aria-orientation`), so
        a vertical list takes Up/Down and ignores Left/Right. Selection follows focus.
      - **Toggle group** — roving tabindex, which it shipped without; every item was a tab stop.
      Verified over CDP on all four: movement, wrap-around, Home/End, disabled items skipped,
      orientation, activation, and focus returning to the trigger when a layer closes.
- [x] `FloatingContext.trigger_id` / `content_id` are minted by `FloatingRoot` (`utils::next_id`)
      and read on both ends. The content side is declarative — each content root renders `id` and
      `aria-labelledby` — but the trigger side is written onto `trigger_ref` from an effect,
      because the trigger is rendered a layer up and differently in each case: a plain button here,
      a styled `Button` there, an arbitrary element behind `render`. The node ref is the only thing
      all three share. Triggers now carry `aria-haspopup` (`menu` / `listbox` / `dialog`, declared
      by each component root), `aria-expanded`, and `aria-controls` while open — which also makes
      `Button`'s existing `aria-expanded:` styling do something for the first time.
- [x] `FloatingContext.modal` and `.disabled` are dropped rather than honored: both now have a
      working equivalent. Modality is expressed by `LayerBounds::modal()` on the dismiss stack,
      which is where it actually has an effect, and `disabled` duplicated the prop every trigger
      already takes.
- [x] Dialog focus handling. The Tab trap and the initial focus-into-content were already in
      `DialogContentRoot` (the entry claiming otherwise was stale); what was missing was giving
      focus back. `DialogRoot` now records `document.activeElement` when it opens and refocuses it
      on close. Verified over CDP: Tab and Shift+Tab cycle inside the content, and Escape returns
      focus to the trigger on both `/docs/dialog` and `/docs/alert-dialog`.
- [x] Dialog content points `aria-labelledby` / `aria-describedby` at its title and description.
      `DialogContext` carries both ids, `DialogTitle` / `DialogDescription` claim them through
      `use_dialog_label_id`, and the content only sets an attribute for a part that actually
      rendered — a dialog with no description does not claim to have one. `alert_dialog` shares
      the primitive and got it too.

- [ ] Two keyboard conventions still missing, both small: ArrowDown on a *closed* select or menu
      trigger should open it (today only Enter/Space/click do), and menus have no typeahead —
      `src/primitives/typeahead.rs` is generic, so it is one call in each menu surface.
- [x] Outside-click dismissal, in the layer stack. The three full-screen overlay divs are gone —
      an open popover no longer covers the page — and dismissal runs off one document-level
      `pointerdown` listener in `src/primitives/dismiss.rs`.
      Each layer registers `LayerBounds`: the content node and the trigger node, both of which
      count as "inside". The trigger has to be part of the layer, or clicking it while open would
      dismiss on pointerdown and then reopen on click. A pointerdown walks the stack from the top
      and dismisses every layer until it reaches one that contains the target — so a click in a
      menu closes its submenu but not the menu, and a click on the page closes both.
      `LayerBounds::modal()` opts a layer out: it is never dismissed by an outside pointerdown and
      stops the walk at itself, which keeps `dialog` (dismissed by its own overlay's click) and
      `alert_dialog` (deliberately not dismissable from outside) behaving as before, including a
      popover opened inside a dialog.
      The floating content node comes from a new `FloatingContext.content_ref`, which each content
      root now passes to `use_floating` in place of a local ref, so positioning and hit-testing
      share one node.
      Verified over CDP: outside click closes popover, select and a menu tree; a click inside a
      popover or on its trigger does not (the trigger closes it, once); the page under an open
      popover is hit-testable again; the dialog still closes on its overlay and the alert dialog
      still ignores outside clicks; no page errors.

- [x] **Delayed unmount could fire into a disposed reactive owner.** Found by the above: opening a
      submenu and clicking outside dismisses two layers at once, and 150ms later the menu's
      unmount disposed the submenu's subtree while the submenu's own unmount timer was still
      pending. It fired, read `is_open`, and panicked — as a bare `RuntimeError: unreachable`,
      since nothing installs a panic hook (see Repo hygiene). Both roots now keep the
      `TimeoutHandle` and clear it in `on_cleanup`, so a torn-down layer takes its timer with it.

## Components — gap vs the shadcn/ui catalogue

Shipped (46): accordion, alert, alert-dialog, aspect-ratio, avatar, badge, breadcrumb, button,
button-group, card, checkbox, code, collapsible, context-menu, dialog, drawer, dropdown-menu,
empty, field, hover-card, input, input-group, input-otp, item, kbd, label, native-select,
pagination, popover, progress, radio-group, scroll-area, select, separator, sheet, skeleton,
slider, spinner, switch, table, tabs, textarea, toast, toggle, toggle-group, tooltip. `code` is not in the shadcn catalogue — it is ours, for documenting the rest.

Missing (18), ordered by priority. Priority = how often it is reached for, weighted down by how much
machinery it needs that this library does not have yet.

**P1 — fundamentals, no new machinery.** Static markup or a single bool of state; each is an
afternoon and closes obvious holes in the current set.

- [x] Separator — `src/primitives/separator.rs` + `src/components/separator.rs`
- [x] Alert — style-only, no primitive
- [x] Toggle — `src/primitives/toggle.rs`, controlled or uncontrolled
- [x] Toggle Group — single/multiple, items inherit variant+size from the group. Arrow-key
      roving focus is NOT implemented yet; see the Accessibility section
- [x] Aspect Ratio — style-only, `aspect-ratio` CSS rather than the padding hack
- [x] Kbd — with `KbdGroup`
- [x] Button Group — style-only (`src/components/button_group.rs`), with `ButtonGroupText` and
      `ButtonGroupSeparator`. The group flattens the corners and drops the doubled border where its
      children meet; `button.rs` already normalised its smaller radii inside
      `[data-slot=button-group]`, so the half-written half of the contract just worked.
- [x] Radio Group — `src/primitives/radio_group.rs` + styled layer. First component built on the
      roving-focus helper, and the one that pays for it: the group is a single tab stop, the arrows
      move *and* select (the radio pattern, unlike a menu), disabled options are skipped, and the
      tab stop sits on whatever is checked. Vertical by default, like a native group.
- [x] Native Select — `src/primitives/native_select.rs` binds a real `<select>` to a signal; the
      styled layer draws its own chevron over `appearance-none`. Distinct from `select`, which
      builds a listbox out of divs: this one costs nothing, keeps `optgroup`, and opens the
      platform picker on a phone.
- [x] Table — style-only wrappers over the native elements, so `colspan` and `scope` keep
      working. `Table` puts the `<table>` in its own `overflow-x` container so a wide one scrolls
      itself; `TableRow` takes `selected` and renders `data-state="selected"`. Head and body cells
      allow no children, for checkbox columns and spacers.
- [x] Empty — style-only: media, title, description, then whatever gets the user out of the empty
      state. `EmptyMedia` has an `Icon` variant (tinted tile) and a `Plain` one.
- [x] Item — style-only row: media, content, actions, with optional header and footer strips.
      `ItemGroup` stacks them and `ItemSeparator` rules between. Variants (default/outline/muted)
      and two sizes.

**P2 — builds on primitives that already exist.** Mostly composition over `floating.rs` and
`dialog.rs`; the keyboard gaps listed under Accessibility should land alongside these.

- [x] Tooltip — `src/primitives/tooltip.rs`, on the floating layer. Opens on hover *and* focus,
      with a 400ms open delay (so sweeping across a toolbar does not flash a row of them) and a
      100ms close delay (so crossing the gap to the content does not dismiss it); pressing the
      trigger dismisses it. Its trigger announces `aria-describedby`, not `aria-expanded` — a
      tooltip does not control anything — which is why `FloatingRoot`'s `haspopup` prop became a
      `TriggerAria` enum (`Popup(&str)` / `Describes` / `None`), with menu, select and popover
      updated to the new shape.
- [x] Collapsible — `src/primitives/collapsible.rs`. The hard part is animating to a height
      nobody wrote down: the panel measures itself once it attaches and publishes
      `--nc-content-height` on its own element, which is what the keyframes animate to. Measured
      through a *tracked* node ref, not `get_untracked` — the content mounts a tick after
      `is_open` flips, so measuring eagerly silently does nothing. Content stays mounted for
      `unmount_delay` so the close animation has something to run on.
- [x] Accordion — `src/primitives/accordion.rs`, each item owning a `CollapsibleContext` so the
      panel machinery is not written twice. The accordion adds the selection (`Single` closes its
      sibling, `Multiple` does not, `collapsible=false` keeps one open) and the keyboard contract:
      the headers are one tab stop and the arrows walk between them, on the roving-focus helper.
      Selection flows one way — the group owns it, the trigger toggles the *group*, and the item
      mirrors the answer into its panel — so there is no two-signal feedback loop.
      The orphan keyframes are now wired to a real component and renamed: `--radix-…`,
      `--bits-…`, `--reka-…` and friends collapse to one project-owned `--nc-content-height`,
      shared by the accordion and collapsible pairs since both animate the same thing.
- [x] Sheet — style-only over `src/primitives/dialog.rs`, no primitive of its own. The modality,
      the focus trap, the layer stack and the exit animation are already the dialog's; what a sheet
      adds is one `side` prop, from which both the edge it pins to and the direction it slides from
      are derived.
- [x] Drawer — `src/primitives/drawer.rs`. The dialog's modality, focus trap, layer stack and exit
      animation, the sheet's header/title/description/footer/close re-exported rather than
      restyled, and one gesture on top. It travels outward only, since there is nothing behind the
      edge to reveal; a flick dismisses as surely as a haul, because throwing something away is not
      the same gesture as placing it. Velocity is sampled *between* moves — measured at release it
      always reads zero — and is signed, so a quick shove back toward the edge does not dismiss. A
      press landing in something already scrolled — or on a button, which owns its own press —
      belongs to that, not to the drawer. On dismissal the offset carries on to fully closed rather
      than resetting, which would snap the panel back before sliding it out. Nests: each drawer has
      its own dialog context, so Escape unwinds one layer at a time and dragging the inner panel
      leaves the outer one alone. `modal=false` drops the overlay, the focus trap, the scroll lock
      and `aria-modal`, and lets an outside pointer close it — the flags live on `DialogRoot`, so
      `Dialog`, `Sheet` and `AlertDialog` can take the same treatment when they want it.
- [x] Hover Card — `src/primitives/hover_card.rs`, the tooltip's hover-intent shape with the one
      difference that changes everything: the content is interactive, so the pointer has to be able
      to leave the trigger and enter the card without it closing. Trigger *and* content both cancel
      the pending close, and the card keeps `pointer-events`, which a tooltip deliberately drops.
- [x] Context Menu — `src/primitives/context_menu.rs`. A dropdown whose anchor is a click rather
      than a button, so the items, roving focus and submenus are `dropdown_menu.rs`'s and the
      styled parts are re-exported rather than restyled. The anchor is a zero-size element parked
      at the click point: positioning against a real element keeps flip and shift working, and it
      is what makes dismissal right — with the trigger *region* as the reference, the dismissable
      layer counts it as inside and a left-click there fails to close the menu it just opened.
      Content is the one part not reused: floating-ui's auto-update watches for resizes and
      scrolls, and an anchor that teleports is neither, so a second right-click elsewhere needs an
      explicit `update()`.
- [x] Scroll Area — `src/primitives/scroll_area.rs`. The box still scrolls natively; only the
      *painting* of the scrollbar is replaced, because a native one cannot be styled consistently
      across platforms and takes layout space on some and not others. Measurement is driven by a
      `ResizeObserver` on the viewport *and* its content as well as the scroll event, since content
      that grows after mount changes the thumb without any scroll firing. Hiding the native bar is
      split across the layers of necessity: `scrollbar-width` is inline on the primitive, but
      WebKit needs a `::-webkit-scrollbar` rule that an inline style cannot express.
- [x] Breadcrumb — style-only, but the markup is the point: a `<nav>` labelled "breadcrumb" around
      an ordered list, the current page marked `aria-current="page"` instead of linked, and the
      separators hidden from assistive tech — they are punctuation, not content.
- [x] Pagination — style-only, and links rather than buttons, because a page of results has a URL
      and the reader should be able to middle-click page 3. `PaginationLink` takes `active` for the
      current page, announced as `aria-current="page"`.
- [x] Field — `src/primitives/field.rs` mints one id per control and hands it out, so the label's
      `for`, the control's `aria-describedby` and the error's id all come from one place and a
      caller writes markup instead of wiring. The error registers its id *outside* the `Show` that
      renders it, which keeps the id stable across every toggle. Controls opt in rather than being
      wrapped, since a field cannot reach into arbitrary children to add attributes: `Input`,
      `TextareaRoot`, `NativeSelectRoot`, `CheckboxRoot` and `SwitchRoot` build a `FieldControl`
      and wear what it resolves to; outside a field every signal reads empty. Still to do: the
      `<fieldset>`/`<legend>` pair for grouping a radio group or a set of checkboxes under one
      label, which needs `aria-labelledby` rather than `for`.
- [x] Input Group — the border moves off the `<input>` and onto the box around it, so an icon, a
      unit or a button can sit inside what still reads as one field. Mostly styling; the one piece
      of behaviour, and why there is a primitive, is that clicking the box's own chrome has to land
      in the control, because a real input has no dead zone inside its own border. `mousedown`
      rather than `click`, so focus is redirected before the browser hands it elsewhere, and
      anything that already owns the click — a button in an addon — is left alone.
- [x] Input OTP — one real input holding the whole value, laid transparently over painted boxes.
      One input per box is the obvious build and the wrong one: paste drops everything after the
      first character, phones will not offer the SMS code, and password managers fill one box and
      give up. Two things it got wrong first: collapsing the selection to pin the caret also killed
      select-all, so pasting a fresh code over a full field silently did nothing (the caret is
      pinned by refusing the keys that move it instead); and native `maxlength` truncates the *raw*
      text, so "12-34-56" was cut to "12-34-" and then filtered to four digits — the cap is applied
      after filtering now.
- [x] Slider — `src/primitives/slider.rs`. The value is a `Vec<f64>` and a range slider is the
      two-element case, so the geometry, the keyboard contract and the ARIA are written once
      instead of twice; thumbs clamp to their neighbours and each announces its own bounds rather
      than the track's. Dragging is tracked on the window, since a pointer that leaves the element
      mid-drag still belongs to the drag, with the listeners installed per gesture and removed on
      release. Pressing the track moves the nearest thumb to the press and picks it up, so a click
      and a drag are one gesture. Snapping rounds at the step's own precision, which is what keeps
      `step=0.5` yielding 3.5 rather than 3.5000000000000004.

**Beyond the shadcn catalogue.** Ours, and both need dependencies the library does not have yet,
so both go behind their own feature flag rather than into `full` by default.

- [ ] Color Picker — needs colour-space conversion (hex/rgb/hsl/oklch) and a saturation/value
      canvas. Feature-flagged: the conversion crate is a real dependency, and a consumer who only
      wants a button should not pay for it.
- [ ] Image Cropper — needs pointer-drag geometry, a canvas or `OffscreenCanvas` for the output,
      and probably an image-encoding dependency. Same reasoning: its own flag.

**P3 — substantial state machines.** Each needs typeahead, roving focus, or measurement work that
does not exist yet; do these after the keyboard-navigation gaps are closed.

- [ ] Command (filtering + keyboard list navigation)
- [ ] Combobox (Command + Popover; `FloatingContext.value` / `display_value` were added for it)
- [ ] Navigation Menu
- [ ] Menubar
- [ ] Sidebar
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

- [x] **Docs pages for previously undocumented components:** alert-dialog, label, spinner — all
      three now have pages, routes and nav entries.

- [x] **Snippets across the docs.** `DemoSection` takes an optional `code` prop that renders a
      `CodeBlock` under the demo. All 26 component pages now carry one per section — 89 blocks —
      extracted from the demo markup itself so a snippet cannot drift from what is rendered beside
      it. Keep snippets in module-level consts, never inline in `view!`: leptosfmt reindents raw
      strings inside the macro and silently rewrites what the reader copies.

- [ ] Snippets are extracted, not curated. A few still carry the demo's layout wrapper (`<div
      class="flex gap-3">`) because that is genuinely what the demo is. Worth a pass to trim the
      ones where the wrapper adds nothing.

- [x] **Syntax highlighting for `CodeBlock`** — a hand-written tokenizer in `src/utils/highlight.rs`
      behind the `highlight` feature (in `full`, not in `components`), no dependencies. Handles
      Rust, shell, JavaScript, TypeScript, HTML and CSS; `Language::Plain` opts out. The
      primitive emits `data-token` spans and the styled
      layer colours them, so a consumer can restyle tokens without touching the primitive. The
      `language` prop exists either way, so turning the feature off changes bytes, not API.
      Known limits: it is a lexer, not a parser — no semantic distinction between a function call
      and a plain identifier, capitalisation is what makes something a "type", JS regex literals are
      not recognised (telling `/` apart needs a parser), and `${…}` inside a template literal is not
      broken out of the string. Adding a language means adding a scanner beside `tokenize_rust`.

## Playground

- [ ] Dead nav links in `playground/src/layout/doc_layout.rs`: `/docs/introduction` and
      `/docs/installation` have no route and fall through to "Not Found."
- [ ] Adding a component means editing five files (page, `pages/docs/mod.rs`, route in `app.rs`,
      `NAV` in `doc_layout.rs`, `COMPONENTS` in `pages/docs/index.rs`) — and `NAV` and `COMPONENTS`
      are now two hand-maintained copies of the same list, which is exactly how `/docs` came to be
      missing progress, skeleton and textarea. Drive routes, nav and the index from one const.
- [ ] Docs pages show rendered demos only; no source snippet or prop table per component.

## Library packaging (needed before publishing)

- [ ] `Cargo.toml` has no `description`, `license`, `repository`, `keywords`, or `categories` —
      `cargo publish` will reject it as-is.
- [ ] No LICENSE file, though the README credits shadcn/ui and Radix (both MIT).
- [ ] Consumers have no way to get the CSS: the tokens, `@custom-variant`s, and `animate.css` live in
      `playground/styles/` only. Ship them from the crate (or document a copy-paste install).
- [ ] `leptos` is pinned to `features = ["csr"]` in the library, which forces CSR on every consumer;
      make the renderer feature-selectable if SSR/hydration should ever work.
- [ ] Doc comments: no `//!` module docs or `///` on public components, so docs.rs would be empty.
- [ ] README is a disclaimer — needs install steps, a minimal example, and the feature-flag table.

## Repo hygiene

- [ ] Almost no tests. `src/utils/highlight.rs` has 7 (`cargo test --features full`), including a
      round-trip property that the tokenizer never drops or reorders a character. Nothing else is
      covered — `wasm-bindgen-test` smoke tests over the primitives' `data-state` transitions would
      catch regressions in the floating/toast state machines.
- [ ] Nothing installs a panic hook, so a panic in the playground surfaces as
      `RuntimeError: unreachable` with no message. Three lines in `playground/src/main.rs`
      (`std::panic::set_hook` writing to `console::error_1`) turn that into the real
      `panicked at …` line; it was worth doing temporarily to find the disposal bug above.

- [ ] No CI. A workflow running `cargo fmt --check`, `cargo clippy --features full`, and
      `cargo check -p playground` would cover today's checks (clippy is currently clean).
- [ ] Large uncommitted working tree (new progress/skeleton/spinner/textarea components + toast and
      dialog changes). Land it in a few focused commits; recent history is "oopsie daisy" x3.
