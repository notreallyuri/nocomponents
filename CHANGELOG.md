# Changelog

The styled layer is installed, not depended on, so a release has two halves: the crate, which this
file is about, and the files `cargo nocli` copied into your project, which are yours and which an
upgrade does not touch. After bumping the dependency and running `cargo update -p nocomponents`,
run `cargo nocli components update` to see which installed components and icons are outdated, then
`cargo nocli components update --apply` to rewrite the unchanged ones. Edited files are never
touched; `cargo nocli components status` and a diff are how you deal with those. A component
installed before the lock existed reads `not in lock`: the tool cannot tell whether it was edited,
so diff it too and `components add <name> --force` to record it.

## 0.6.1

Phone sheets: fixes for what a real iPhone showed. The fixes themselves are verified in Chromium
with touch emulation only; the iOS Safari behaviour they target is unverified. `cargo-nocli` is
unchanged (0.4.0), and `cargo nocli components update --apply` brings the styled fixes into an
installed project.

### Added

- `utils::scroll_lock` (`use_scroll_lock`): the counted page lock the sheets and modal dialogs use.
- `DropdownMenuSubPortalRoot`, the portal of a submenu's content.
- `data-presentation` (`"sheet"` or `"floating"`) on submenu triggers, and `"sheet"` on inline
  submenu content.

### Fixed

- Dragging over a phone sheet scrolled the page behind it. Every sheet (dropdown menu, context menu,
  select, popover, and a dialog or drawer) now freezes the page while it is open, through one
  reference-counted lock in `utils::scroll_lock`: `overflow: hidden` on `<html>` and `<body>`,
  restored on close and unmount, and nested layers do not unlock early. iOS Safari ignores that for
  touch, so a non-passive `touchmove` listener also cancels any vertical move that does not start
  in something that can still scroll that way; the scrim is `touch-action: none` and the sheet
  scrollers are `overscroll-behavior: contain`. A floating menu or select still does not lock the
  page. The dialog's own body lock is replaced by the same one.
- Submenus did not work on touch. A finger or pen sends `pointerenter` and `pointerleave` around
  every tap, and the leave closed the submenu before the tap's click could keep it open; only a
  mouse hovers now, and a tap toggles. In a sheet a submenu opens in place, under its trigger, as an
  indented group (`aria-expanded` and a turned chevron on the trigger; ArrowRight or Enter expands,
  ArrowLeft folds back to the trigger) rather than as a floating panel beside the sheet. New
  `DropdownMenuSubPortalRoot` is the portal that makes that choice; `DropdownMenuSubContent` uses it.
- `Input` with `InputType::Date` was taller than its neighbours on iOS, which draws date fields
  with its own appearance and ignores `h-8`.
- The Select and Context Menu pages had no phone-sheet demo; both have one now.

### Repo

- The smoke runner waits longer for a cold Chrome on CI, retries a launch once, and passes
  `--disable-dev-shm-usage`, `--disable-gpu` and `--headless=new`.

## 0.6.0

Forms and focus behave on the sign-in and account screens a port starts with, and the phone sheet
can be closed and dragged like one. One rename breaks callers, and phone sheets change by default;
both are below. `cargo-nocli` is unchanged (0.4.0).

### Upgrading from 0.5

Bump to `0.6`, `cargo update -p nocomponents`, then `cargo nocli components update` and
`--apply`. By hand:

- `Calendar` and `DatePicker`: rename `disabled=` (the per-date callback) to `disabled_dates=`.
  `disabled` on `DatePicker` is now a `Signal<bool>` that disables the whole picker.
- Phone sheets now show a header close button and drag to dismiss; `DialogPortal`'s corner close
  hides in a sheet. Opt out per layer with `sheet_show_close=false` / `sheet_draggable=false`.
- A dropdown menu, context menu or select open as a sheet closes when the window crosses `md`.
- A popover moves focus into itself on open; `auto_focus=false` keeps it on the trigger.
- A form reset now writes into a `model`: pass `default_value` to say what it resets to.

### Breaking

- **`Calendar` and `DatePicker`: the per-date callback is `disabled_dates`.** `disabled` was a
  `Callback<Date, bool>` here and a `Signal<bool>` everywhere else; it is now the signal on
  `DatePicker`. Migration: rename `disabled=|date| …` to `disabled_dates=|date| …`. The primitive
  `CalendarRoot` keeps `disabled`.

### Changed

- **A `mobile_sheet` layer's header now has defaults you did not have in 0.5.** On
  `DropdownMenuContent`, `ContextMenuContent`, `SelectContent`, `PopoverContent` and `DialogPortal`
  (and their `*Root`s), below `md`:
  - a close button is shown in the header row — opt out with `sheet_show_close=false`;
  - dragging the header (grabber and title) moves and dismisses the sheet, and the header is
    `position: sticky` with `touch-action: none` — opt out with `sheet_draggable=false`, which also
    drops the `touch-action`;
  - `DialogPortal` hides its corner close button in a sheet, the header's taking its place (it
    follows `show_close`, so `show_close=false` removes both) — there is no separate switch;
  - the title is `font-semibold text-foreground` rather than `font-medium`.
- **In a sheet, a menu, context menu or select is built as a frame around the role element.** The
  header sits beside the `role="menu"` / `role="listbox"` element, not inside it, and the classes,
  the drag transform and `data-dragging`, `data-snap` and `data-state` are on the frame
  (`data-slot="menu-sheet"`). Floating markup is unchanged. Such a layer closes if the window
  crosses `md` while it is open.

### Added

- **`Checkbox` `indeterminate`** (and `indeterminate_icon`, default `Minus`): `aria-checked="mixed"`,
  `data-state="indeterminate"`; a click checks the box and calls `on_checked_change(true)`. On
  `CheckboxRoot` too. `DataTable`'s select-all uses it for a partly selected page.
- **`PopoverContentRoot` / `PopoverContent` `auto_focus`** (default true).
- **`DatePicker` `disabled` and `id`**, so the picker can be switched off and a `Field` label can
  point at it.
- **`InputType::Tel`, `Url`, `Search`, `Date`.**
- **`NativeSelect` `on_change`, `NativeSelectOption` and `NativeSelectOptGroup`** (with
  `*Root` primitives).
- **`SelectTrigger` `disabled`** and **`InputGroupButton` `disabled` and `type`**.
- **`SliderThumb` `aria_label`**, `FieldContext::label_id` and `FieldControl::labelled_by`.
- **`default_value`** on `Input`, `Textarea` and `NativeSelect`: what a form reset puts back, read
  when the reset happens (so data that arrived after mount is honoured), else the current `value`.
  A control's `form` prop is honoured by the reset listener. `utils::form::on_native_reset`.
- **`PopoverContentRoot` `initial_focus`**, a selector `auto_focus` tries first.
- **The phone sheet can be closed and dragged.** On `DropdownMenuContent`, `ContextMenuContent`,
  `SelectContent`, `PopoverContent` and `DialogPortal` (and their `*Root`s) a sheet now has a close
  button in its header row — `sheet_show_close` (default true), `sheet_close_label` (default
  "Close"), `sheet_close_icon` — and dragging the header down moves the panel with the pointer and,
  on release, springs back or dismisses it by the drawer's own rule (`data-dragging`, no transition
  while dragging). The body is not a handle, so scrolling a list never drags the sheet.
  `sheet_draggable` turns the drag off. `sheet_snap_points` (fractions of the viewport height) gives it several resting heights:
  it opens at the first, settles on the nearest, a flick moves one, and a release below the
  lowest dismisses; the active index is `data-snap`. `DialogPortal` hides its corner close button
  in a sheet, since the header carries one. New primitive `sheet_drag` (`use_sheet_grab`,
  `SheetGrab`), and `utils::settle` (`settle`, `Settle`, `settle_drag`), which the drawer now
  re-exports. The sheet title reads as a heading (`font-semibold text-foreground`).
- **`container` on every portalled floating layer**: `DropdownMenuPortal` (and `ContextMenuPortal`),
  `SelectPortal`, `MenubarPortal`, and `HoverCardContent`, `TooltipContent`, `ComboboxContent` (with
  the matching `*PortalRoot`s), as `PopoverPortal` already had.
- **A floating layer presented as a sheet no longer attaches `use_floating`'s auto-update**
  (`floating_options_unless_sheet`).

### Fixed

- **A form reset now reaches a `model`.** `Input`, `Textarea` and `NativeSelect` went back to their
  starting value on screen and left the caller's `model` holding the old one; they now write their
  `default_value` into both.
- **A popover moves focus into its content when it opens**, as Radix does (`auto_focus=false`
  opts out); before, a keyboard user had to Tab through the page to reach it. A `DatePicker`
  focuses the selected day, or the calendar's current day stop.
- **`DataTable`'s select-all shows mixed** when some but not all rows are selected.
- **A slider inside a `Field` has an accessible name**: the thumb points `aria-labelledby` at the
  field's label.

## 0.5.0

Icon slots reach through the composite components, the close and loading labels can be translated,
and the CLI learns to bring unchanged installed files up to date on its own. Purely additive.

### Upgrading from 0.4

Bump to `0.5`, `cargo install nocomponents-cli` (0.4.0), `cargo update -p nocomponents`, then
`cargo nocli components update` and `--apply`. A refreshed `code`, `date_picker`, `data_table`,
`color_picker` or `video_player` names `nocomponents::utils::icon::icon_or`, so it needs the 0.5
crate; one you leave as it is keeps compiling against 0.5 unchanged.

### Added

- **Composite icon pass-through.** The components built out of other parts now take the glyphs
  those parts draw: `CodeBlock` `copy_icon` and `copied_icon`, `DatePicker` `prev_icon` and
  `next_icon`, `DataTable` `check_icon`, `ColorPicker` `select_icon` and `check_icon`, and
  `VideoPlayer` `icons: VideoPlayerIcons`, one struct for the player's glyphs, written with
  `..Default::default()` so whatever is left out keeps its built-in one. Only the default controls
  use it; a `controls` callback composes the parts, and each passes its own.
- **`VideoPlayerVolumeControl` `volume_icon` and `muted_icon`**, and `submenu_icon` on the
  `*Items` parts of the video player's menu.
- **`close_label`** (default "Close") on DialogPortal, SheetContent and DrawerContent, and
  **`label`** (default "Loading") on VideoPlayerBuffering, so the screen-reader text can be
  translated.
- **`utils::icon::icon_or`**, which composites use to forward an optional slot with the inner
  part's fallback glyph.

### cargo-nocli 0.4.0

- `components update [--apply]` compares every installed component and icon with the release in
  `Cargo.lock`, and with `--apply` rewrites the ones still unchanged, installing any new dependency
  they gained. Files edited here, not in the lock, or not in the release are listed and left alone.
- `icons remove <names> [--force]`, the counterpart of `components remove`: refuses an icon that
  something installed still uses, or one that has been edited.

## 0.4.0

Icons come from the consumer. Every component that draws a glyph takes it through an `icon` slot,
and the glyphs it falls back on are installed into the project as source, like the components, so
there is no icon set to depend on and no icon crate to agree with. Plus the form and link gaps the
first screens of a port run into.

### Upgrading from 0.3

Bump the dependency to `0.4`, update the CLI (`cargo install nocomponents-cli`, 0.3.0), then:

```bash
cargo nocli components status          # what is unchanged, edited, not in lock
cargo nocli components add <name> --force   # for each unchanged one; diff the edited ones
```

`add` now also writes the glyphs each component falls back on, and the `Icon` wrapper, into
`paths.icons` (default `src/icons/nc`; a 0.3 `nocomponents.toml` gets the default without being
edited). Declare `mod icons;` in your crate root if `add` says so. Components you do not refresh
keep compiling: they still name `nocomponents::icons::…`, which 0.4 keeps behind the `icons`
feature. Once every installed component is refreshed, drop `icons` from your features.

Nothing below removes a prop; every change is additive except the two marked **breaking**.

| component | change |
| --- | --- |
| `icons::IconRoot` | **breaking**: renamed `Icon` (`icons::icon::Icon`), and `view_box` is required. It no longer strokes by default: pass `stroke_width=2.0` for the old look. Decorative by default (`aria-hidden`); `label` names it. |
| every glyph (`ChevronDown`, `X`, `Check`, …) | now `aria-hidden="true"`. A glyph you gave `attr:aria-label` should use `Icon`'s `label` instead. |
| `Spinner` | **breaking (DOM)**: the root is a `<span role="status">` holding the glyph, not the `<svg>` itself — CSS aimed at `svg[data-slot=spinner]` needs `[data-slot=spinner]`. New `icon`. |
| `Accordion` | `AccordionTrigger`: `icon`. The open rotation now targets `[data-slot=accordion-icon]`, not a child `svg`. |
| `Badge` | `render` (the `StyledRender` shape); `children` optional. Renders a `<span>` rather than a `<div>`. |
| `Breadcrumb` | `BreadcrumbLink`: `render`, `children` optional. `BreadcrumbSeparator`: `icon`. |
| `Button` | `aria_label`, `id`, `name`, `value` (also on `ButtonRoot`; `aria_label` and `id` reach a `render` element). |
| `Calendar` | `prev_icon`, `next_icon`. |
| `Canvas` | `CanvasControls`: `zoom_out_icon`, `zoom_in_icon`, `fit_icon`. |
| `Carousel` | `CarouselPrevious`, `CarouselNext`: `icon`. |
| `Checkbox` | `icon`. |
| `Code` | `CodeCopyButton`: `copy_icon`, `copied_icon`. |
| `Combobox` | `ComboboxTrigger`, `ComboboxItem`: `icon`. |
| `Command` | `CommandInput`: `icon`. |
| `DataTable` | `sort_ascending_icon`, `sort_descending_icon`. |
| `DatePicker` | `icon`. |
| `Dialog` | `DialogPortal`: `close_icon`. New `DialogClose` (a button that closes, like `SheetClose`). |
| `Drawer` | `DrawerContent`: `show_close`, `close_icon`. |
| `DropdownMenu` / `ContextMenu` | `*CheckboxItem`, `*RadioItem`, `*SubTrigger`: `icon`. |
| `FloatingMenu` | `FloatingMenuHandle`: `icon`. |
| `Input` | `on_input`, `on_change` (`Callback<String>`). |
| `Item` | `render`; `children` optional. |
| `Label` | `html_for`. |
| `Menubar` | `MenubarSubTrigger`: `icon`. |
| `NavigationMenu` | `NavigationMenuTrigger`: `icon`. |
| `Pagination` | `PaginationPrevious`, `PaginationNext`: `icon`. |
| `Select` | `SelectTrigger`, `SelectItem`: `icon`. |
| `Sheet` | `SheetContent`: `show_close`, `close_icon`. |
| `Sidebar` | `SidebarTrigger`, `SidebarTreeRow`: `icon`. The mobile sheet uses `show_close=false` rather than hiding the button with a class. |
| `Sortable` | `SortableHandle`: `icon`. |
| `Textarea` | `on_input` (also on `TextareaRoot`). |
| `Tree` | `TreeRow`: `icon`. |
| `VideoPlayer` | `playsinline`, `preload`, `idle_delay`; draws `VideoPlayerBuffering` and a replay glyph once ended. `VideoPlayerVideo`: `playsinline`, `preload`. `VideoPlayerBadge`: `play_icon`, `replay_icon`. `VideoPlayerPlayButton`: `play_icon`, `pause_icon`, `replay_icon`. `VideoPlayerMuteButton`: `volume_icon`, `muted_icon`. `VideoPlayerFullscreenButton`: `enter_icon`, `exit_icon`. `VideoPlayerSkipButton`, `VideoPlayerMenu`, `VideoPlayerCaptionsButton`: `icon`. The three `*Items`: `check_icon`. |

### Added

- **Icon slots.** One shape everywhere: `icon: Option<ViewFn>` on a part that draws one glyph, and
  `<what>_icon` for each glyph of a part that swaps them (`play_icon` / `pause_icon`). Pass any
  view, `icon=|| view! { <MyIcon /> }`; unset draws the installed fallback. The component draws
  the slot through `utils::icon::IconSlot`, an `aria-hidden` span it sizes, colours, rotates and
  hides, with the `<svg>` filling it — so the caller's icon needs no classes, and one from any set,
  at any viewBox, sits exactly where the fallback did.
- **`Icon`** (`icons::icon::Icon`, was `IconRoot`): the wrapper for any SVG. Takes the artwork's
  own `view_box` (required — Font Awesome's are per icon, `0 0 512 512`, `0 0 448 512`), fills with
  `currentColor` unless `stroke_width` is set, accepts arbitrary children (a duotone icon's two
  paths and opacity), sizes to `1em` through the `width`/`height` attributes so any class or
  component rule wins, and is `aria-hidden` unless `label` is set (then `role="img"` with that
  `aria-label`).
- **`/docs/icons`**, the page that documents the slot, the wrapper, the fallbacks and bringing
  your own set, with demos of a filled narrow icon and a duotone one.
- **`VideoPlayerBuffering`**, a spinner over the frame while playback waits for data;
  `VideoPlayerRoot` and `VideoPlayerSurfaceRoot` write `data-waiting` and `data-ended`. A `Replay`
  glyph.
- **`render` on `BreadcrumbLink`, `Badge` and `Item`** — a router `<A>`, as `Button` already
  could. **`DialogClose`.** **`show_close` on `SheetContent` and `DrawerContent`.**

### Fixed

- `TODO.md` said `Dialog` had no `modal`; it has had one on the root since 0.2.0, as has `Sheet`.

### cargo-nocli 0.3.0

- **Icons are installed.** `components add` writes the glyphs each component falls back on, and
  `Icon`, into `paths.icons` (default `src/icons/nc`), rewrites `crate::icons::…` to that module,
  declares it in the module above, and records the files in the lock (apart from components, since
  `calendar` is both). A component's `--force` refreshes an unchanged icon but never one you have
  edited; `icons add <name> --force` does.
- **`icons add`** and **`icons list`**; `components status` lists icons too.
- `init` no longer turns on the `icons` feature.
- Installing from a release before 0.4, which has no icon sources, keeps the old behaviour: icons
  come from `nocomponents::icons` and the feature is turned on.

## 0.3.0

What noaweb needed to port immich-front: a sortable list, drawer snap points, long-press, phone
sheets, and an installer that leaves a fresh project compiling. Source-compatible with 0.2 except
where focus goes after a pointer closes a layer (see Changed).

### Added

- **Bottom sheet on phones.** `DropdownMenuContent`, `ContextMenuContent`, `SelectContent`,
  `PopoverContent` and `DialogContentRoot` (styled: `DialogPortal`) take `mobile_sheet:
  Signal<bool>` (default false) and `sheet_title: Signal<String>`. Below `md` (767px, via
  `use_media_query`) the content is a bottom sheet in the same portal: a scrim, the panel on the
  bottom edge, a grabber and the title row; the floating layers apply no positioning, and focus,
  roving focus, typeahead and dismissal (scrim tap, Escape, choosing an item) are unchanged. The
  content writes `data-presentation="sheet"` or `"floating"` (a dialog only when `mobile_sheet` is
  set), and drops `data-side` and `data-align` in a sheet; the styled layer styles both from one
  class string. The shared pieces are `utils::sheet` (`use_sheet_presentation`, `SheetScrim`,
  `SheetHeader`, `SHEET_QUERY`).
- **`PopoverPortal` / `PopoverPortalRoot` take `container: Option<AnyNodeRef>`**, which mounts the
  panel in that element instead of `<body>` (`FloatingPortal` takes it too). A fullscreen element
  still wins when the container is not inside it.
- **`Skeleton` takes `variant: SkeletonVariant`** (`Default`, `Tile`, `Line`, `Card`), written as
  `data-variant`; the pulse stops under `prefers-reduced-motion`, and the placeholder is
  `aria-hidden`.
- **`ButtonSize::Fit`**: no fixed height or padding, for a button that wraps a layout of its own.
- **`utils::modality`** (`track_input_modality`, `last_input_was_keyboard`): whether the last input
  was a key or a pointer.
- **`Sortable`** (`primitives::sortable`, feature `sortable`; styled `Sortable`, `SortableItem`,
  `SortableHandle`): a list reordered by pointer, touch or keyboard, which reports the move and
  leaves the order to the caller. `on_move` gets `SortableMove { keys, to }`: the moved items in
  their current relative order, and `to`, the index they land at in the list once they have left
  it; `apply_move(&mut items, &moved, key)` applies it. Pointer drag goes through `use_drag` (the
  held item follows the pointer, the others slide aside, no easing under `prefers-reduced-motion`);
  on the handle, or the item if it has none, Space or Enter picks up, the arrows (Left and Right
  when `orientation` is horizontal) and Home/End move, Space or Enter drops, Escape or losing focus
  cancels, and focus stays on the moved item. `selected` makes a block drag: the selection moves
  together, by pointer and keyboard. A visually hidden `aria-live` region announces pick-up, moves,
  drop and cancel, and an instruction paragraph is `aria-describedby` on every handle; both are
  replaceable (`announce`, `instructions`). Held near an edge it scrolls the nearest scrolling
  ancestor, never the page, so it works in a dialog that locks the body. Needs `Grip` icon (new).
- **`primitives::drag`** now also exports `edge_ramp`, `ScrollAnchor` and `scroll_parent`, the
  auto-scroll curve, scroll-compensation and scroller lookup that the kanban board had privately;
  the board uses them, and so does `Sortable`.
- **`use_long_press`** (`primitives::long_press`, feature `long_press`): a held touch or pen press
  that fires once after a delay (450 ms by default), reporting the press point, and is abandoned by
  a move of more than 10 px, a release or a `pointercancel`. A mouse press never fires it. It starts
  from a `pointerdown` that may land on a child, and follows the gesture through `use_drag`.
- **`ContextMenu` opens on a touch long-press**, at the touch point, as `contextmenu` does at the
  cursor. `ContextMenuTrigger` and `ContextMenuTriggerRoot` take `long_press: Signal<bool>`
  (default true) and `long_press_ms: Signal<f64>` (default 450.0), and set
  `-webkit-touch-callout: none` so iOS shows no callout of its own. The `contextmenu` Android sends
  after a long press is swallowed when the press has already opened the menu. Feature
  `context_menu` now lists `long_press`.
- **Drawer snap points.** `Drawer` and `DrawerRoot` take `snap_points: Signal<Vec<f64>>` (fractions
  of the viewport along the side, e.g. `vec![0.4, 0.88]`), `active_snap` / `default_active_snap` /
  `on_active_snap_change` (the active point's value, `Option<f64>`), `dismiss_below` (default 0.22)
  and `flick_velocity` (default 0.6 px/ms). The panel is as tall as the highest point, follows the
  handle while dragged, settles on the nearest point, moves one point on a flick, and is dismissed
  below `dismiss_below`; `data-snap` carries the index. The decision is the pure
  `primitives::drawer::settle`, returning `Settle::Snap(f64)` or `Settle::Dismiss`. Without
  `snap_points` a drawer behaves as before.

### Notes

- **`ChildrenFn` children cannot move an owned value.** `DropdownMenu`, `Popover`, `Dialog` and the
  other parts that take `children: ChildrenFn` may be rendered more than once (inside a `Portal` or
  an `Either` branch), so moving a non-`Clone` value into them fails with E0525 ("expected a closure
  that implements `Fn`, but this closure only implements `FnOnce`"). Clone it before the `view!`, or
  keep it in a `StoredValue` and read it inside.

### Changed

- **Focus returns to the trigger on close only when the keyboard closed the layer.** Floating
  layers (`FloatingRoot`: popover, dropdown menu, select, context menu, tooltip and the rest) and
  `DialogRoot` used to give focus back after any close; after an outside click or a mouse-chosen
  item they now leave it where the pointer put it, which stops the focus ring and scroll jump on
  the trigger. Escape, and Enter or Space on an item, still return it. If your code relied on the
  trigger being focused after a mouse close, focus it from `on_open_change`.
- **Styled modules re-export the types their props name** when a primitive or `utils` owns them:
  `Side`, `Align`, `SideOffset`, `Orientation`, `TriggerRender`, `StyledRender`, `AnchorRender`,
  `Date`, `Language`, `ColorFormat`, `Crop`, `CalendarMode`, `ChartKind`, `TableSort`,
  `DropdownMenuItemVariant`, `TabsListVariants`, `ToastVariant` and so on, from the styled module
  that takes them. Previously `DropdownMenuItem`'s `variant` could only be named through
  `nocomponents::primitives::dropdown_menu`.
- The re-exports in installed files, and the generated `mod.rs` prelude, carry
  `#[allow(unused_imports)]`: in a binary crate every `pub use` a project does not itself use
  warned, and `ContextMenu*` alone produced a dozen.

### cargo-nocli 0.2.0

- **`components status`** lists installed components as `unchanged`, `edited` or `not in lock`.
- **`components add`** no longer runs `cargo add` when `Cargo.toml` already asks for every feature
  the install needs, so the project's formatting is left alone when nothing changes. Files it
  overwrites with `--force` print `replaced`, with a note when they were edited or not in the lock,
  and a file not in the lock is `kept (not in lock, cannot tell if edited)` rather than "already
  here".
- **`init`** writes a minimal Trunk `index.html` (linking the installed stylesheet) when there is
  none, and the installed `globals.css` sets `--font-sans` to the system stack instead of naming a
  font nothing loads.
- **A `use` with attributes keeps them on every line it is flattened to.** `#[allow(unused_imports)]`
  on a re-export tree used to land on the first line only, so an installed `context_menu` warned
  once per remaining name; a `#[cfg]` would have been a miscompile. `pub(crate) use` keeps its
  visibility per line too.
- **`add` declares the install directory** in its parent module (`src/components/mod.rs` with
  `pub mod nc;`, inside a `nocli:begin` / `nocli:end` block when the file is the project's), so a
  fresh project no longer fails with E0583. The crate root is not edited; `add` prints the
  `mod components;` to add when it is missing.

## 0.2.0

Every stateful root is now controlled *or* uncontrolled, plain values take signals, and controls
reach a native `<form>`. Most of it is source-compatible, and the list below is what is not.

### Breaking

- **`Some(callback)` no longer compiles for an optional callback.** `on_click`, `on_select`,
  `on_complete`, `on_rejected`, `on_blur`, `on_skip` and the rest are declared
  `#[prop(optional, into)] x: Option<Callback<T>>`, so the setter takes the callback itself.
  Migration: write `on_click=move |e| …`, or `on_click=Callback::new(…)`, and drop the `Some(…)`.
  `render` and `as_child`, and the canvas, kanban-card and video-player callbacks whose presence
  changes behaviour (`on_move`, `on_resize`, `on_surface_drag`, `on_skip`), still take the `Option`.
- **A trigger's `render` / `as_child` is handed a `TriggerRender`, not a `Callback<MouseEvent>`.**
  This is `DialogTrigger`, `SheetTrigger`, `AlertDialogTrigger` and `DialogTriggerRoot`. Migration:
  take `|t: TriggerRender|` and attach `t.node_ref`, call `t.on_click` and honour `t.disabled`. The
  struct is `nocomponents::utils::types::TriggerRender`, still re-exported from
  `primitives::floating`.
- **`NavigationMenuLink`'s `render` takes a `NavigationMenuLinkRender`**, a struct, instead of the
  bare click callback.
- **`Menubar`: `open` is now `value`.** It is `Option<RwSignal<Option<String>>>`, with
  `default_value` and `on_value_change` beside it. Rename the prop.
- **`Toggle`: `on_change` is now `on_pressed_change`.** `ToggleRoot` also resolves `disabled`
  against a surrounding `Field`.
- **Contexts hold signals where they held plain values.** `use_accordion()`, `use_calendar()`,
  `use_carousel()`, `use_chart()`, `use_code()`, `use_color_picker()`, `use_command()`,
  `use_image_cropper()`, `use_resizable_group()`, `use_sidebar()`, `use_slider()`, `use_tabs()`,
  `use_timeline()` and `use_video_player()` fields such as `kind`, `collapsible`, `mode`,
  `orientation`, `wrap`, `with_alpha`, `should_filter`, `aspect`, `circular`, `direction`, `side`,
  `min`/`max`/`step`, `alternate` and `index` are `Signal<T>` (`MaybeProp<T>` for an optional one).
  Migration: `.get()` them. `CodeContext::code` went from `StoredValue<String>` to `Signal<String>`,
  and `FieldControl::id` from `Signal<String>` to `Signal<Option<String>>`, `None` meaning "write no
  `id`".
- **Props the caller would change at runtime are signals** — `side`, `align`, `side_offset`,
  `variant`, `size`, `orientation`, `kind`, `delay`, `src`, `alt`, `code`, `label`, `columns`,
  `empty`, `show_close` and the like. Passing a plain value or a closure still compiles; only a
  caller that named the type, or that read the prop back, has to change. `KanbanSlot`'s `index` and
  `Code`'s `label: Option<String>` (now an empty `Signal<String>` for "none") are the two that
  change what you write.
- **`checked`, `value` and `open` are `Option<RwSignal<_>>`.** `Checkbox`, `Switch`, `Tabs`,
  `Select`, `Combobox`, `Command`, `NavigationMenu`, `Calendar` and `DatePicker` no longer require
  the signal; `checked=sig` still works. `Tabs`' `default_value` is optional too.
- **`SelectValue`'s `placeholder` is a `Signal<String>`**, not a `String`. Plain strings still
  convert; only a caller that named the type changes.
- **`Field` ids.** A control outside a field and given no `id` renders no `id` attribute rather than
  `id=""`; a `FieldLabel` pointing at it needs one.

### Added

- **Controlled and uncontrolled everywhere.** Every stateful root takes `x`, `default_x` and
  `on_x_change`, resolved by `utils::controllable::{controllable, forward}`. New where there was
  nothing: `Popover`, `ContextMenu`, `Sheet`, `Dialog`, `AlertDialog`, `Drawer`, `Collapsible`,
  `Accordion` (`default_value`, `on_value_change`), `Slider`, `ToggleGroup`, `Toggle` (`pressed`,
  `default_pressed`), `Command`,
  `Calendar` (`default_month`, `on_month_change`) and the rest of that shape.
- **Native forms.** `Checkbox`, `Switch`, `RadioGroup`, `Select`, `Slider` and `Combobox` render
  hidden inputs and take `name`, `value`, `required` and `form`; they reset with the form (`reset`
  event) and are submitted by it. A range slider submits `name[]` once per thumb. `Toggle`,
  `ToggleGroup` and `Slider` resolve `disabled` against a surrounding `Field`. The plumbing is
  `utils::form`.
- **`DropdownMenu`'s missing parts** — `DropdownMenuGroup`, `DropdownMenuShortcut`,
  `DropdownMenuCheckboxItem`, `DropdownMenuRadioGroup`, `DropdownMenuRadioItem`, `inset` on items,
  labels and sub-triggers, and `variant=Destructive` — and `class` on every part. `ContextMenu*`
  re-exports them. Checkbox and radio items stay open (`close_on_select`).
- **`modal`** on `Dialog` and `Sheet`; `aria-haspopup`, `aria-expanded` and `aria-controls` on
  dialog triggers, and a `role` on the dialog panel.
- **`Select`** takes `name`, `required`, `disabled`, `form` and `default_value`; the trigger shows
  a chosen item's label even before the list has first opened.
- **`docs/smoke/`**, a browser suite for the docs site (`node docs/smoke/run.mjs`). Not shipped.

### Fixed

- **Triggers go through their `*Root`.** `DialogTrigger`, `SheetTrigger`, `AlertDialogTrigger` and
  `DrawerTrigger` went around it (the drawer's was a raw button), so `data-slot`, the ARIA and the
  `Field` wiring were missing and the node ref was never set. A non-modal drawer closed on the
  trigger's pointerdown and the click reopened it; `DrawerTrigger` also takes `disabled` now.
- **Dialogs.** `AlertDialogAction` closes the dialog, and the panel is `role="alertdialog"`.
- **`Select`.** The trigger shows the chosen item's label rather than its value, including a
  `default_value` chosen before the list has ever opened. A `SelectItem` can be `disabled`.
- **`DataTable`.** Selected rows highlight, and sortable headers keep `scope="col"` and
  `data-sorted`. `TableRow` no longer writes `data-state=""` on an unselected row.
- **`Toast`.** `data-[size=lg]:text-base` was written `data=[size=lg]`, so a large toast kept small
  text. Toasts are announced: the regions are labelled, and each toast carries a role and live
  region (`status` or `alert` by variant).
- **`FieldLabel`'s `for` follows the control's own `id`**, so `<Input id="email">` inside a field is
  no longer labelled by an id nothing carries.
- **`Code`.** The copy button says "Copied" only once the clipboard write succeeded, and its reset
  timer is cleared; `code` and `label` follow changes.
- **`Dropzone`** with `multiple=false` keeps one file on a drop, not only in the picker.
- **`Carousel`** reacts to writes to `index`, and ignores arrow keys in editable targets.
- **`Progress`** no longer yields `NaN` at `max=0`, and takes a `label`.
- **`Accordion`** no longer logs Leptos's untracked-read warning once per item.
- **Menus.** Clicking a sub-trigger opens its submenu and a hover-opened one stays open; the
  submenu writes `data-align`; the item under the pointer takes focus. `MenubarSubContent` mounts
  through `FloatingPortal`, as the other floating layers do. `ContextMenu` content and `Popover`
  content write their own `data-slot`.
- **`Timeline`** orders events by document position, so inserting or removing one no longer breaks
  the alternating sides.
- **`Avatar`** returns to the fallback when `src` changes, and clears its delay timer on cleanup.
- **`Button`** with `render` runs `on_click` and the disabled refusal.
- **`Sidebar`**'s mobile branch merges the caller's `class` and carries the `id` the trigger's
  `aria-controls` names.
- **A control with no `id`** outside a `Field` no longer renders `id=""`; `InputOtp` takes `id`,
  `label` and the field's `aria-describedby` and `aria-invalid`.
- **Accessibility.**
  - `Tabs`: triggers and panels have ids and point at each other with `aria-controls` and
    `aria-labelledby`.
  - `Calendar`: `aria-selected` sits on the cell, and there is always one tab stop, even when the
    focused day is disabled.
  - `Carousel`: a labelled region, and each slide is named "n of m".
  - `Chart` takes a `label`; `Spinner` is `role="status"` and named (default "Loading"); `Canvas`
    and `ColorPicker` buttons keep their visible text in their names.
  - Video player: Mute, Fullscreen and Subtitles keep a fixed label and write
    `aria-pressed="true"|"false"`.
  - `PaginationEllipsis` and `BreadcrumbEllipsis` hide the `…`, not their `sr-only` text;
    `BreadcrumbLink` omits an empty `href`, and the `<nav>` label can be set.
  - `HoverCard`'s trigger describes rather than claiming a popup; `ScrollArea`'s viewport is
    focusable; a `Dropzone` with a trigger button is no longer a button itself; `Kanban` drops the
    deprecated `aria-grabbed`; `ItemGroup` no longer claims `role="list"` without list items.
