# Changelog

The styled layer is installed, not depended on, so a release has two halves: the crate, which this
file is about, and the files `cargo nocli` copied into your project, which are yours and which an
upgrade does not touch. After bumping the dependency, run `cargo nocli components status` to see
which installed components are `unchanged`, `edited` or `not in lock`, then `cargo nocli components
add <name> --force` for each unchanged one, and diff the rest. A component installed before the
lock existed reads `not in lock`: the tool cannot tell whether it was edited, so diff it too.

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
