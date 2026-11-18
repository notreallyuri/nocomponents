# TODO

Working list for `nocomponents`. Unfinished tasks are preserved from the 2026-11-13 list.
Completed entries were consolidated on 2026-11-14; detailed reasoning remains in git history.

## Completed work summary

The 154 completed tasks removed from this list covered:

- **Components and shared utilities:** completed the component catalogue and added drag, colour,
  image/GIF, date and locale utilities; shipped the colour picker, cropper, kanban, floating menu,
  canvas, timeline, tree and video player, with further interaction and layout improvements.
- **Bug fixes and accessibility:** corrected portal context, exit animations, dismissal, focus,
  pointer handling, scroll compensation, field wiring and ARIA attributes. Shared dismissal,
  roving-focus and typeahead primitives now support keyboard interaction across controls.
- **Canvas editing:** added move/resize callbacks, committed gesture reporting, surface gestures,
  marquee and group selection, group transforms, z-order, world-coordinate helpers and editable
  multiline labels. Reserved rotation in the data model with `WorldBox::angle`.
- **Component APIs, icons and themes:** reviewed the styled layer, standardised controlled and
  uncontrolled state and render callbacks, improved native forms and reset handling, added consumer
  icon slots and localisable labels, and expanded palette, accent and radius configuration.
- **Phone sheets:** added sheet presentations, header close controls, drag-to-dismiss, snap points
  and portal containers; fixed background scroll locking and touch submenu behaviour. Real iOS
  Safari touch behaviour remains unverified by the Chromium smoke suite.
- **Docs:** added component demos, snippets and API tables, composed blocks, utility and theme
  pages, search and page navigation. Persistent docs shells preserve sidebar scroll within a
  section; blocks and primitives have their own searchable sections, with five primitive pages
  written. Consolidating the five-file component registration process was folded into the open
  generator task below.
- **CLI, releases and repository checks:** installed component/icon sources and CSS, resolved
  sources from the consumer's library release, added lock hashes, safe removal, managed module
  declarations and `components update`. Published both crates, improved README, package metadata,
  licences and notices, configured docs.rs, and added CI checks and browser smoke coverage.

## Priority

- [ ] Three-way merge for files edited here and changed upstream (`update` lists them; today the
      project diffs by hand).
- [ ] `--backup` for `components update --apply`: copy replaced files to
      `.nocli/backup/<timestamp>/` first (opt-in; not the default — a project under version
      control doesn't need it).

## Accessibility & keyboard

Shared keyboard and focus primitives are covered in the completed work summary; the canvas gap
remains below.

- [ ] Canvas nodes are not reachable by keyboard — see *Canvas: what a collaboration tool needs*.

## Canvas: what a collaboration tool needs

The canvas exists because of `novoice`, a collaborative editor that will be built on it, and this
is the gap between what is in the tree and what that needs. **Ordered, not a bag**: each group is
what has to be true before the next one is worth starting, and the reason is written down for
each — a list of features with no argument for their order is a list somebody will reorder by
whichever is most fun.

What is already right, and should not be redone: the canvas **never owns what is on it**. It
reports where a drag would put a node and the caller decides, so a CRDT can be the single source
of truth without competing with local state, and snapping, grids and constraints are arithmetic
applied to the reported point rather than features the component has to grow. The viewport is a
plain signal, which makes follow-the-presenter three lines. The arithmetic is in `utils::viewport`,
off the DOM and unit-tested. And a resize is measured from the press rather than accumulated,
which matters more under network lag than it ever did locally.

### 2. Settle before there is data, because it is free now and expensive later

Already reserved: `WorldBox::angle` and the canvas node's angle prop carry rotation through
rendering and gesture reports; rotated resize, bounds and the rotation grip remain deferred.

- [ ] **The rotation gesture itself**, when novoice wants it. Three pieces, and the field above is
      not one of them: rotate the pointer delta into the box's own frame before `resized` and back
      after; project the four corners in `around` so fit and cull stop under-covering; and a ninth
      grip to turn it by, which is the only new thing on screen.

### 3. Before it is a whiteboard

- [ ] **Pinch on touch.** `use_drag` follows one pointer and nothing anywhere tracks a
      `pointerId`, so there is no two-finger anything. Zoom works on a trackpad only because a
      browser disguises a pinch as `ctrl+wheel`, and on a tablet there is no zoom at all — the
      styled surface sets `touch-none`, so the browser will not do it either. Two pointers on the
      surface should scale about the midpoint between them and pan by its travel, which is one
      gesture rather than two. Belongs in `drag.rs` if a second primitive wants it, and in the
      canvas if none does.

### 4. Before it is big

- [ ] **Cull what is off screen.** Every node is mounted, always. A board is the one component
      here where the count is the reader's to decide, and thousands of absolutely-positioned
      elements is where this stops being usable. The viewport already knows the visible world
      rectangle, so the test is arithmetic the caller could do — but it needs the rectangle, which
      means publishing it: `CanvasContext::visible()`.
- [ ] **`content_bounds` is coupled to that.** It reads every `[data-slot=canvas-node]` out of the
      DOM and takes each one's rect — a forced layout per call, which is fine for a button press
      and not for a loop, and *silently wrong* the moment culling lands, since it would fit only
      what happens to be mounted. Whichever of the two is done first has to answer for the other:
      either fit takes the caller's own bounds, or culling keeps a way to measure what it hid.

      Reading the DOM has a second cost now that a caller can render a node that is not content:
      the board block's selection frame is a `[data-slot=canvas-node]` like any other, and fit
      counts it. Harmless there — a frame is the box around things already counted — and not
      harmless for a guide or a ruler. A caller's own bounds would answer this too.

### 5. Before it is a product

- [ ] **Nodes are not reachable by keyboard.** The surface is a tab stop and the things on it are
      not: nothing on a board can be selected, moved or renamed without a pointer. Wants a roving
      tab stop over the nodes (`primitives/roving_focus.rs` already does the hard part), arrows to
      nudge, Enter to edit the label, and the same `data-selected` the pointer path writes.

## Components — gap vs the shadcn/ui catalogue

**The catalogue is complete, and this list is the count.** Shipped (66): accordion, alert,
alert-dialog, aspect-ratio, avatar, badge, breadcrumb, button, button-group, calendar, canvas,
card, carousel, chart, checkbox, code, collapsible, color-picker, combobox, command, context-menu,
data-table, date-picker, dialog, drawer, dropdown-menu, dropzone, empty, field, floating-menu,
hover-card, image-cropper, input, input-group, input-otp, item, kanban, kbd, label, menubar,
native-select, navigation-menu, pagination, popover, progress, radio-group, resizable, scroll-area,
select, separator, sheet, sidebar, skeleton, slider, spinner, switch, table, tabs, textarea,
timeline, toast, toggle, toggle-group, tooltip, tree, video-player.

Twelve are not in the shadcn catalogue — canvas, code, color-picker, dropzone, floating-menu,
image-cropper, kanban, native-select, timeline, toast, tree, video-player. `toast` is that
catalogue's `sonner` under the name the rest of the world uses; `code` is ours, for documenting
the rest; the other ten are listed under *Beyond the shadcn catalogue* below.

**Nothing in the shadcn catalogue is missing.** The three names not above are not components:
`sonner` *is* `toast`, `typography` is a stylesheet page, and `form` is deliberately a separate
crate (`noform`) rather than something this one grows.

The remaining count, for whoever comes to check it: 74 element features, 56 of them with a
primitive and 67 with a component. `CLAUDE.md` says the same numbers — when one of these moves,
both do.

**P4 — skipped, and here is why.** Marker, Attachment, Bubble, Message, Message Scroller and
Questionnaire are chat and AI surfaces: markup with no state machine, no ARIA question, no gesture
and no keyboard contract. Six of them would add six docs pages and around ninety prop rows to a
table of 985 already kept in step by hand, and would demonstrate nothing this library does not
already show somewhere better. They are not a gap in the catalogue; they belong to whoever is
building a chat product, on top of `item`, `avatar`, `scroll-area` and `field`, which are here.

- [ ] **Direction (RTL).** The one thing still open here, and it is not a component: it is a
      provider plus every component that lays out along an axis reading it — the floating layer's
      `Side`/`Align`, the sidebar, the slider, the carousel, the resizable handles, the roving
      focus arrows. Cheap to add and worthless alone, which is why it stayed open while the
      catalogue was being filled and is worth doing now that it is not moving. It is the
      difference between a library that is incomplete in RTL and one that is *wrong* in it.

**Beyond the shadcn catalogue.** Ours, and none of them took a dependency.

- [ ] Canvas connectors: edges between nodes, with anchors, arrowheads and re-routing as nodes
      move. The largest of the three and the one that most wants its own primitive, since routing
      is arithmetic and belongs beside `utils::viewport` rather than in a component.

**Unfinished corners of what is shipped.**

- [ ] `ColorPicker` in more shapes than the square-and-rail. A **triangular** picker (saturation
      and value on the corners of a triangle, hue on the ring around it) and a **wheel** are the two
      people ask for; both are the same `Hsva` underneath, so what is new is the geometry — a point
      inside a triangle or a disc mapped to two channels, and back. That is arithmetic with a unit
      test, and the picker already keeps HSVA rather than RGB precisely so a shape like this cannot
      lose the hue in a corner. The rails stay: a shape is a way to pick, not the only way, and the
      text field is what makes any of them exact.

- [ ] `ColorPicker` should use a `Select` instead of a button to switch colors, and instead of
      a unified input, using a divided input (with `InputGroup`) with a `Select` to switch formats
  - 3 for rgb
  - 4 for rgba
  - 3 for hsl
  - 4 for hsla
  - 4 for hsva
  - 4 for oklch
- [x] `ButtonSize` cannot size the icons inside the button, and the rules that were meant to are
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
      Done: containers set `--icon-size`; `Icon` and the glyphs read `size-[var(--icon-size,…)]` (bracket form, `tw_merge` keeps both of `size-(--x)`); guards removed.

- [ ] `highlight` is still a lexer per language: a new one is a new scanner beside `tokenize_rust`,
      and the six that exist share only their helpers. Fine at six; worth a second look at ten.
- [ ] `navigation_menu` has no shared viewport: each panel is absolutely positioned against the
      menu, so they all open in the same place, but the box does not slide or resize between them.
      That needs the active panel measured and the size animated.
- [ ] `chart` needs more of each mode, not only more modes. How a series gets from one point to the
      next is its own choice and is currently always a straight line: an area or a line should be
      able to be **curved** (a monotone spline, which does not overshoot into values the data never
      had) or **stepped** (blocky — which is the honest shape for anything that holds a value until
      it changes, a price or a setting), as well as the sharp one it draws now. That is an
      interpolation prop over the existing path arithmetic rather than a new kind of chart.

- [ ] `chart` needs the shapes that are not a cartesian plot at all: **pie**, **radial** (a bar
      chart bent round a circle) and **radar**. They share an angular scale and a centre instead of
      an x and a y, so they want their own primitive beside `primitives/chart.rs`'s rather than a
      mode inside it — and all three are arithmetic, which means they can be unit-tested the way
      `utils::color` is rather than only eyeballed.

- [ ] `chart` is line, area and bar over a categorical x axis. No stacking, no second axis, no time
      scale, no pie — each is more arithmetic in `primitives/chart.rs`, not new machinery.
- [ ] `chart` draws every label it is handed, and the tooltip reads that same list, so a long series
      cannot be thinned by blanking labels without leaving the tooltip with nothing to say. The
      caller's way out is fewer, wider buckets (the chart block does exactly that). A `labels` /
      `tick_labels` split, or drawing every nth, would make the chart's own answer possible.
- [ ] `data_table` has no pagination of its own; it shows what it is handed. `Pagination` is shipped
      and the two compose — the data-table block demonstrates it, sorting the whole set before
      slicing a page out of it.
- [ ] `utils::gif` re-emits every frame at its clipped size rather than diffing consecutive frames
      for a minimal changed rectangle, so a cropped GIF is a little larger than it needs to be. The
      sample goes 17.5kB → 7.3kB on a crop to a third of the area, which is fine but not optimal.
- [ ] `docs/assets/avatar.jpg` is a 6.9MB PNG with a `.jpg` name. It is `include_bytes!`d, so all
      of it lands in the wasm binary — worth re-encoding to a few hundred kilobytes and renaming.
- [ ] The colour picker's square announces itself as `role="application"` with a description of the
      keys. There is no honest ARIA role for a two-axis drag; the rails and the text field are the
      paths that announce themselves properly.

### Styled-layer API review (2026-11-05)

All 66 components were reviewed with their primitives for class forwarding, API completeness,
reactive props, repeatable children and bugs. The remaining findings follow.

**Per component**

- [ ] `accordion`: `AccordionContent` gives the root a fixed class and puts the caller's `class` on
      the inner padding div (`components/accordion.rs:72-79`), so the animated panel itself
      cannot be styled — `class` should land on the element that carries `data-state`, with the
      padding div a fixed inner.
- [ ] `alert_dialog`: no `AlertDialogOverlay`/`AlertDialogContent` split — the portal's `class`
      styles the panel, the overlay is fixed — and no `modal`, which `DialogRoot` has and the styled
      wrapper hides.
- [ ] `calendar`: no `number_of_months` (`components/calendar.rs:13`).
- [ ] `command`: `CommandInput` wraps the `<input>` in a fixed `div`
      (`components/command.rs:50`), so the caller's `class` reaches the input and nothing reaches
      the row with the icon — you cannot change its height or border.
- [ ] `context_menu`: `ContextMenu` takes no `class` (`components/context_menu.rs:16`) though
      `ContextMenuRoot` is a `FloatingRoot` with one. The content has its own slot now, but the
      re-exported items, labels and sub parts still carry the dropdown's names —
      `data-slot="dropdown-menu-item"` (`primitives/dropdown_menu.rs:310`) inside a context menu —
      so a stylesheet keyed on slot cannot tell the two apart; giving them their own means a slot
      prefix handed down from the root through context. `Group`, `Shortcut`, `CheckboxItem`,
      `RadioGroup` and `RadioItem` are now re-exported under `ContextMenu*` names.
- [ ] `data_table`: `empty` is a string, so "No results" cannot be a node. (The select-all is mixed
      now — landed in 0.6.0.)
- [ ] `dialog`: no `DialogOverlay`/`DialogContent` split — the portal's `class` styles the panel
      and the overlay is fixed. (`modal` and `DialogClose` are not missing; both landed
      by 0.4.)
- [ ] `drawer`: no `DrawerPortal` or `DrawerOverlay` — the overlay is built inside `DrawerContent` and only when `modal`
      (:66-75), so a non-default overlay means rewriting the content.
- [ ] `dropzone`: no `max_size` or maximum count either (`components/dropzone.rs:10`).
- [ ] `field`: no `FieldContent`, `FieldTitle` or `FieldSeparator` (`components/field.rs:12`).
- [ ] `floating_menu`: the styled layer does not expose the primitive's `style`
      (`primitives/floating_menu.rs:86`, `components/floating_menu.rs:11`). `FloatingMenuItem`
      builds its own `<button>`, the `data-compact` rule and the compact-to-tooltip switch
      (:107-165) in the styled layer, and `FloatingMenuContent` (:77) and `FloatingMenuSeparator`
      (:169) render their elements directly with no `*Root` behind them — behaviour that belongs in
      the primitive under the rule in `CLAUDE.md`. The doc comment gives a reason (`ButtonSize` is
      a value, so an item could not resize with the menu), which is a reason for the enum item
      above rather than for the layering.
- [ ] `hover_card`: the open delay is `delay` on `HoverCard` (`components/hover_card.rs:16`) where Radix
      and the primitive call it `open_delay`; `close_delay` is already there. Rename it, or keep it and say so.
- [ ] `image_cropper`: `min_size` is on the primitive (`primitives/image_cropper.rs:219`) and not
      on `ImageCropper` (`components/image_cropper.rs:12`). The resize handles are
      `aria-hidden="true"` (`primitives/image_cropper.rs:386`) with no keyboard path, so the crop
      can be moved and not resized without a pointer.
- [ ] `input_group`: `InputGroupButton` takes `disabled` and `type` now (landed in 0.6.0). Still
      open: `InputGroupInput` and `InputGroupTextarea` forward only
      the subset of `Input`'s and `Textarea`'s props they list (:49), so whatever `Input` gains
      does not reach them until someone remembers.
- [ ] `item`: `ItemMediaVariant` lacks shadcn's `Image`. (`render`, so an `Item` can be a router
      link, landed in 0.4.)
- [ ] `menubar`: `MenubarSub` takes no `class` (`components/menubar.rs:157`) and
      `MenubarSubTrigger` no `disabled` (:162) — see `dropdown_menu`, which this is built on.
      None of `MenubarGroup`, `MenubarCheckboxItem`, `MenubarRadioGroup`, `MenubarRadioItem` or
      `inset`, though the dropdown primitive has all of them now and `menubar.rs` only has to wrap
      them. (`MenubarSubContent` used to mount into `document.body`; it
      goes through `FloatingPortal` now, like `DropdownMenuSubContent`.)
- [ ] `navigation_menu`: `NavigationMenu` exposes `value` and not the primitive's
      `open_delay`/`close_delay` (`components/navigation_menu.rs:13`,
      `primitives/navigation_menu.rs:127-128`), so the hover timing is fixed.
- [ ] `pagination`: `PaginationItem` takes no `class` (`components/pagination.rs:56`).
      `PaginationPrevious` and `PaginationNext` (:82) have no `disabled` or `aria-disabled`, so
      the first page's "Previous" is a live link to nowhere. `PaginationLink` is a plain `<a>` with
      no `render`, so a router link has to be wrapped; and an unset `href` writes `href=""`.
- [ ] `popover`: `Popover` takes no `class` (`components/popover.rs:11`), no `PopoverAnchor`.
      (Opening now moves focus into the content, `auto_focus` to opt out — landed in 0.6.0.)
- [ ] `resizable`: no `collapsible` or `collapsed_size` (`components/resizable.rs:32`).
- [ ] `scroll_area`: the styled component always renders a vertical scrollbar and takes only
      `horizontal` for the other (`components/scroll_area.rs:11-30`) — no `ScrollBar` part and no
      `orientation`, so a horizontal-only area has an inert vertical track.
- [ ] `select`: `SelectTrigger` takes `disabled` now (landed in 0.6.0); still no `size`, no `SelectGroup`, `SelectLabel`,
      `SelectSeparator` or the scroll buttons, and `SelectValue` takes no `class`.
- [ ] `slider`: no `inverted` (`components/slider.rs:8`).
- [ ] `toast`: `duration` is hardcoded `5000.0` in the provider's defaults
      (`primitives/toast.rs:223`) with no `default_duration`, and there are no
      `ToastTitle`/`ToastDescription`/`ToastAction`/`ToastClose` parts: a toast can only be built
      through the builder, never composed.
- [ ] `tooltip`: no `TooltipProvider` — shadcn's shared `delayDuration`/`skipDelayDuration`, which
      is what stops every tooltip in a toolbar waiting out its own delay. `TooltipContent` (:45)
      has no arrow.
- [ ] `video_player`: the three menu parts take no `class` and hardcode the item classes
      (`VideoPlayerSpeedItems` `components/video_player.rs:437`, `VideoPlayerCaptionItems` :479,
      `VideoPlayerCaptionSizeItems` :556), and `VideoPlayerMenu`'s `class` goes to the trigger
      (:419) while the content's is fixed (:428). (Buffering — `VideoPlayerBuffering`, from
      `data-waiting` — the replay glyph once `data-ended`, and `idle_delay`, `preload` and
      `playsinline` on the styled player landed in 0.4.)

### From the noaweb port (2026-11-05)

The rest of what noaweb asked for landed (sortable, drawer snap points, long-press, mobile sheets,
the small items and the CLI friction); what follows is what those left behind.

- [ ] Still open on the sheets: `SheetScrim`'s and the grabber's classes live in `utils/sheet.rs`,
      which a Tailwind build in a consumer's project does not scan (the same trap `src/components`
      exists to avoid) — they should move to the styled layer. `use_floating` still computes one
      position when a sheet mounts and discards it (its auto-update is detached and re-attached
      live as the presentation flips). A menu, context menu or select that crosses `md` while open
      is closed rather than re-shaped, since a sheet's markup differs (`SheetFrame`). Submenus
      (`DropdownMenuSubContent`) take no `container`. Sheet heights are `dvh` and the settle
      fraction uses `visualViewport.height`, which differs from `dvh` while a keyboard is up.

### From noaweb, second batch (2026-11-18)

What noaweb's reports in `~/projects/noatelier/noaweb/docs/` found after the first batch landed.

Bugs (`nc-patch-dialog-escape.md`, except the third):

- [x] `DialogContentRoot` registers a window `keydown` focus-trap listener and drops the handle
      (`src/primitives/dialog.rs:370`, `let _ = window_event_listener(ev::keydown, focus_trap);`),
      so it is never removed. The closure reads `ctx.is_open`, so once the dialog's owner is
      disposed the next keydown anywhere panics on a disposed signal (Escape first, since it is
      what closes it). Keep the handle and remove it in `on_cleanup`; `try_get` in the closure.
      noaweb's `ui/detached.rs` workaround can go once this is fixed.
- [x] `Field`'s `focusout` handler (`src/primitives/field.rs:221-233`) calls `field_ref.get()` and
      `context.touched.set(true)`. A focused control removed along with its dialog fires `focusout`
      after the field's owner is gone, and it panics. `try_get` / `try_set`. Same report.
- [x] Two `DropdownMenuRadioGroup`s in one menu share a context owner: `DropdownMenuRadioGroupRoot`
      (`src/primitives/dropdown_menu.rs:575`) uses a bare `provide_context(RadioGroupContext {..})`,
      so the later group's context reaches the earlier group's lazily rendered items. noaweb's sort
      menu swapped its second group for plain items and a `•` text bullet, which is the "radio dot
      size differs between groups" it reported (`noaweb/src/ui/artist_toolbar.rs:215`). Use
      `<Provider>`, as the dialog portal fix did, and audit the other parts that bare
      `provide_context` and can sit beside themselves (`select.rs:227`, `tabs.rs:74`,
      `collapsible.rs:121`, `carousel.rs:280`, `progress.rs:23`, `avatar.rs:23`).

Video player on touch (`video-touch.md`):

- [x] `VideoPlayerRoot` wakes on every `pointermove` / `pointerdown` and hides on any
      `pointerleave` (`src/primitives/video_player.rs:836-845`). A finger lifting is a
      `pointerleave`, so the bar vanishes under the tap. Gate hover wake/hide on
      `pointer_type == "mouse"`; a touch tap should toggle the overlay. Done: hover wake/hide are mouse-only, and a touch
      tap on a playing video toggles the overlay instead of playback.
- [x] The controls go `pointer-events-none` the instant `data-active` drops
      (`src/components/video_player.rs:215`), so a tap that lands as the bar fades falls through to
      the surface's play/pause click (`primitives/video_player.rs:961`). Done: a press on the controls
      restarts the idle countdown, so the bar is still up for the click.
- [x] `active` starts `true` (`primitives/video_player.rs:695`) and the idle countdown starts only
      from `wake()` (`:714`), so the overlay never auto-hides when playback starts without a
      pointer. Done: `play`/`playing` start the countdown.
- [x] `toggle_fullscreen` (`primitives/video_player.rs:437-447`) drops `request_fullscreen`'s error
      and has no `webkitEnterFullscreen` fallback on the `<video>`, so fullscreen does nothing on
      iPhone. Done: falls back to the video's `webkitEnterFullscreen`/`webkitExitFullscreen`, and
      `webkitbegin/endfullscreen` keep the flag in step.
- [x] Touch targets are small: `CONTROL` is `size-8` (`components/video_player.rs:37`), the rail
      `h-1.5` (`:313`), and nothing is sized with `pointer-coarse:`. Done: controls `pointer-coarse:size-11`, the rail
      gets a taller transparent hit area.

APIs:

- [x] `CheckboxRoot` discards the click event (`src/primitives/checkbox.rs:71`,
      `move |_: ev::MouseEvent|`), so shift-click range selection cannot be built;
      noaweb hand-rolled a `role="checkbox"` button. Wants
      `on_click: Option<Callback<ev::MouseEvent>>` run before the toggle, as on `ButtonRoot`
      (`primitives/button.rs:56`), and the styled `Checkbox` forwarding it. Report:
      `release-order.md`. Done: `on_click` on `CheckboxRoot` and `Checkbox`, run before the toggle.
- [x] `Input` has no `node_ref` (`src/components/input.rs:58`, an internal `AnyNodeRef::new()`);
      `Textarea` already takes one. `on:keydown` already reaches the `<input>`, so no typed keydown
      prop is needed. Done: `node_ref` prop, internal ref kept as the default.
- [x] A windowed sortable (`release-order.md`). Done: `SortableWindow` (`primitives::sortable::window`), fixed row height, O(1) drop and shift, held and focused rows pinned, `/docs/sortable` "Windowed". `SortableRoot` finds its items in the DOM and
      measures every one at pick-up, and every item's `item_style` reads the whole drag state, so
      it is quadratic: noaweb needed 5,000 rows (route cap; 20,000 planned) and wrote its own on
      `use_drag` + `edge_ramp` (`noaweb/src/ui/release_order/list.rs`,
      `noaweb/src/model/order_window.rs`: fixed 64px rows, overscan 6, O(log n) `drop_index`).
      Wants a root taking a count and a row height, rendering a visible range, with index-based
      drop and preview, focus and reveal that work for unmounted rows, labels from data, and
      `on_move` carrying indices. A larger design item.

From `shell-report.md`:

- [x] `DropdownMenuItem` has no `render` (`src/components/dropdown_menu.rs:139`), so a menu row
      cannot be a router link. Done: `render` (`StyledRender`) through `DropdownMenuItemRoot`, on `MenubarItem` too.
- [x] `SidebarMenuAction` (`src/components/sidebar.rs:504-531`) is a bare `<button>` with no
      `render`, `node_ref` or `on_click`, so it cannot be a dropdown trigger. Done: new `SidebarMenuActionRoot` with `render`, `node_ref`, `on_click`, `disabled`.
- [x] `SidebarMenuButton`'s `[&>svg]:size-4` (`src/components/sidebar.rs:479`) beats a caller's own
      size on a direct child svg; noaweb wraps its brand mark in a span. Same shape as the
      `ButtonSize` icon-size item above — fix them together. Done with it: the button sets `--icon-size`.
- [x] `SortableItem`'s bordered-card classes are fixed (`src/components/sortable.rs:63`); there is
      no plain or menu-row variant. Done: `SortableItemVariant::{Card, Plain}`, plus a docs demo.
- [x] `InputType` has no `Hidden` (`src/components/input.rs:10-21`); noaweb's password-manager
      username field uses `class="hidden"`. Done: `InputType::Hidden`.
- [x] CLI: `manifest::add_dependency` returns whether it changed `Cargo.toml`
      (`cli/src/manifest.rs:15`), and `install.rs:252` discards it, so "0 written" is reported
      when `Cargo.toml` was rewritten. PORT-PLAN §5.4 item 2. Done: the summary line ends "Cargo.toml updated" when it changed.
- [x] Block drag in the windowed sortable (noaweb's release-order list): dragging a selected row
      moves every selected row together. Done: `selected` on `SortableWindow`, and
      `SortableWindowMove` carries the block (`is_block`, `count`, `apply`).
- [x] Native Shift-range selection, so noaweb's hand-made `role="checkbox"` button can go. Done:
      `utils::selection::RangeSelection` with `checked(i)` / `on_click(i)` on the `Checkbox`, which
      also extends a range from the keyboard with Shift+Space.
- [x] `Input` had no typed `keydown` prop. Done: `on_keydown` on `Input`, `Textarea`,
      `InputGroupInput` and `InputGroupTextarea`.

### Icons (0.4)

Components take their glyphs through `icon` slots and fall back on Lucide glyphs installed as
source (`/docs/icons`). What that left:

- [ ] `DataTable`'s "Select all rows" / "Select row" aria-labels are fixed English.
- [ ] Composites repeat their inner part's fallback glyph through `utils::icon::icon_or` (an `optional, into` slot's setter takes `ViewFn`, not `Option<ViewFn>`, so an `Option` can't be forwarded as is); if a part's default glyph changes, its composites must change with it — a slot type that accepts both a closure and an `Option` would remove the duplication.

## Docs site

- [ ] Blocks for the rest, in rough order of what would teach the most: **select** — a dependent
      pair where the second is filtered by the first; **resizable** — a two-pane layout that
      remembers its split; **menubar** — an application menu with the keyboard doing the work;
      **carousel** — slides that are links, so tab order and scroll snapping have to agree;
      **date-picker** — a range across two months, which is the one the calendar's API has not been
      asked for yet.
- [ ] Snippets are extracted, not curated. A few still carry the demo's layout wrapper (`<div
      class="flex gap-3">`). Worth a pass to trim the ones where it adds nothing.
- [ ] The header link between a component and its blocks is a stopgap. A component page should open
      onto a *switch*: **Component**, **Blocks**, and later **Primitives** — one page, three views
      of it. The routes are already shaped for it, so it is a `Tabs` in `DocLayout` driven by the
      path rather than a restructure.

### The other 48 primitives

The ones that do have a styled layer: **48 modules, 180 parts**, against the 5 written. Counted
from the tree rather than remembered — an earlier version of this list said 45 and 157.

**The order below is the plan, and the first item gates the rest.** Writing 180 prop tables by hand
and then generating them is doing the work twice, and 180 is not a number anybody finishes by hand
anyway: the 315 tables and 985 rows already in `docs/src/pages/docs/` are hand-pasted, and they are
the largest single debt in this repo. So the generator comes first, and every page below is then a
short file of prose around a table nobody typed.

- [ ] **1. The generator.** An `xtask` over the `#[component]` signatures emitting the `ApiEntry`
      shape both layers already use — name, type, default, and the doc comment as the description.
      It pays for itself twice over: 180 primitive tables that never get written, and 985 styled
      rows that stop drifting — 315 tables and 985 rows hand-pasted today. An `xtask` or a build
      script both fit. It could drive `NAV` and `COMPONENTS` too, closing the five-files item above.
      This is the one generator item; the later *Generate the prop-table rows* line points here.

      Two things it must handle that the styled layer does not exercise: `#[prop(default = …)]`
      needs rendering as the default column, and `src/primitives/` carries real `///` prose while
      `src/components/` deliberately carries none — so the styled side of a pair should fall back
      to the primitive's description rather than emit a blank.

- [ ] **2. The page shape, settled once.** A generated table is the part a primitive page needs
      *least*. What it is for is the four things a signature cannot carry, and the counts say how
      much of each there is:

      - **the context it publishes** — 33 of the 48 have one, with a `use_<name>()` beside it, and
        the rule that it may only be called inside its own root;
      - **the `data-*` it writes** — every one of the 48 writes some, and they are the entire
        styling contract of the unstyled layer;
      - **the keys it binds** — 21 of the 48;
      - **a demo built from plain classes**, not the styled layer, since that is the claim the
        split exists to make.

      One template with those four sections, proven on two pages before the other 46 are cut to
      fit it.

- [ ] **3. The way in.** The component/blocks/primitives switch, as described under *The header
      link between a component and its blocks is a stopgap* above; `/primitives` keeps its own
      index for the five that have no component page to hang off.

Then the pages, batched by what they share rather than alphabetically, so a shape is argued once
and reused rather than re-decided 48 times. The batches add up to 53 and cover 48: families 4 and 7
overlap by five — `context_menu`, `dropdown_menu`, `menubar`, `navigation_menu`, `select` — which
are the menus that both float and rove, and each belongs to whichever batch it gets written in.

- [ ] **4. The floating family (9).** `popover`, `tooltip`, `hover_card`, `dropdown_menu`,
      `select`, `combobox`, `context_menu`, `menubar`, `navigation_menu`. They alias one
      `FloatingContext` and reuse `FloatingRoot` / `FloatingTrigger`, so their pages are one shape
      nine times over, all linking back to `/primitives/floating`, which is already written. Do
      these first: they settle how a page describes a primitive built on another, which nothing
      else in the docs does yet.

- [ ] **5. The thin ones (9).** `button`, `checkbox`, `input_group`, `native_select`, `separator`,
      `switch`, `table`, `textarea`, `toggle` — one or two parts, no context, no keys. These pages
      are the ARIA and the `data-*` and nothing else, and they are where the template gets proven
      cheap. Two are only thin by the count. `button` is a trigger the whole floating family
      renders through `TriggerRender`, so its page is the other end of family 4. And `table` has
      one part but two `Copy` handles — `TableSort` and `TableSelection`, which are handles rather
      than a context precisely because a table already has its rows in one place; that argument
      *is* the page.

- [ ] **6. The gesture family (10).** `canvas`, `color_picker`, `drawer`, `floating_menu`,
      `image_cropper`, `kanban`, `resizable`, `scroll_area`, `slider`, `video_player` — all on
      `use_drag`, all linking back to `/primitives/drag`. Same trick as the floating family: one
      shape, ten pages. `video_player` is the largest primitive in the tree at 12 parts, so it
      goes last within the batch, once the shape has stopped moving.

- [ ] **7. The roving-focus family (12).** `accordion`, `calendar`, `command`, `context_menu`,
      `dropdown_menu`, `menubar`, `navigation_menu`, `radio_group`, `select`, `tabs`,
      `toggle_group`, `tree`. Overlaps 4 by the five menus that both float and rove — a page
      belongs to whichever family it gets written in, and the other family is a cross-link.
      `command` is the one worth its own page for the opposite reason: it moves the highlight
      through `aria-activedescendant` *because* it could not reuse `roving_focus`, and a family
      page that only says what a primitive shares never says that.

- [ ] **8. The rest, one essay each (13).** `avatar`, `carousel`, `chart`, `code`, `collapsible`,
      `dialog`, `dropzone`, `field`, `input_otp`, `progress`, `sidebar`, `timeline`, `toast`.
      Nothing shared to exploit: `field` at six parts and five hooks, `chart`, `sidebar` and
      `toast` are each their own argument, and the small ones are small for reasons that differ
      every time. Last, because by then the template has been through forty pages and stopped
      changing.

- [ ] The Leptos-shaped types the prop tables keep naming — `Signal<String>`, `ChildrenFn`, the
      `render` callback shape — still have nowhere to be described once. `Side` / `Align` /
      `Orientation` / `SideOffset` are on `/docs/utility` now, so this is the other half of that
      page rather than a `/docs/types` of its own.
- [ ] Generate the prop-table rows at build time — see item 1 of *The other 48 primitives*.
- [ ] `init` installs the whole of `globals.css`, which is four palettes and fourteen accents —
      most of a consumer's stylesheet, for tokens they mostly will not use. Worth splitting the
      base tokens from the palettes so a project takes one and opts into the rest.
- [ ] `cargo doc` over the styled layer is a list of bare signatures. Its prose is the docs site's
      prop tables now, and a description written in both places drifts, so `src/components` carries
      no `///` on purpose — docs.rs stays empty until it can be generated from the same source as
      the tables. `src/primitives` and `src/utils` keep theirs, being undocumented elsewhere.

## Repo hygiene

- [ ] There are no `wasm-bindgen-test` unit tests over the primitives' `data-state`
      transitions, which would catch regressions in the floating and toast state machines without a
      browser session — the toast one has now been wrong twice.
