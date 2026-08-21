# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

`nocomponents` is a Leptos 0.8 (CSR-only) UI library for the browser: unstyled behavior primitives in the
spirit of Radix, plus a styled layer that mirrors shadcn/ui. `docs/` is a Trunk + Tailwind v4 docs
site that is both the demo and the only way to exercise the library in a browser.

Cargo workspace: root crate `nocomponents`, member `docs`.

## Commands

```bash
cargo check --features full          # type-check the library (default features compile almost nothing)
cargo check -p docs                  # type-check the docs app
cargo fmt && cargo clippy --features full

cd docs && trunk serve               # dev server on :3000, watches ../src too (see Trunk.toml)
cd docs && trunk build --release
```

Trunk downloads and runs the Tailwind CLI itself (pinned in `docs/Trunk.toml`); there is no
`tailwind.config.js` and no npm build step. There is no test suite — verify changes in `trunk serve`.

`leptosfmt` is what formats `view!` blocks, and `cargo fmt` leaves them alone. Run it on the files
you changed, never over `src/` or `docs/src/`: 0.1.33 silently **deletes** Rust comments that sit
inside a `view!` — e.g. on a `match` arm in an `on:keydown` handler — so a whole-tree run strips
commentary out of files you never meant to touch.

## Feature flags

`primitives`, `icons`, `components` (= primitives + icons), `theme`, `full`. In `src/lib.rs` **both**
`components` and `primitives` modules are gated on the `primitives` feature, and `theme` gates
`utils::theme`. With no features only `utils` and `middleware` exist, so always check/build with
`--features full` (which is what `docs` depends on).

## Architecture: the two layers

`src/primitives/<name>.rs` — behavior, ARIA roles, state, `data-*` attributes, **no Tailwind classes**.
Components are named `<Name>Root`, `<Name>TriggerRoot`, `<Name>ContentRoot`, … and take
`class: Signal<String>` that they render verbatim.

`src/components/<name>.rs` — the shadcn-styled wrapper: same tree, named `<Name>`, `<Name>Trigger`, …,
passing a `cn!(...)` class down into the matching `*Root`. Some components are style-only and have no
primitive (`skeleton`, `spinner`, `badge`, `card`, `input`, `label`); `alert_dialog`, `date_picker`
and `combobox` are built on other primitives rather than one of their own. Both modules re-export
everything through a `prelude`.

Three components ship as **one component instead of a tree of parts** — `Calendar`, `Chart`,
`DataTable` — because nobody rearranges a month grid or an axis, and the parts would be a worse API
than the props. Their primitives are still parts (`CalendarGridRoot`, `ChartPlotRoot`, …) for anyone
who disagrees, and `DataTable` takes its columns as a `Vec<DataTableColumn<T>>` value rather than as
children.

When adding a component, put behavior in the primitive and only classes in the component; if you find
yourself writing `if is_open` styling logic in `src/components/`, it belongs in a `data-*` attribute
on the primitive instead.

## Cross-cutting patterns

**Context**: each stateful primitive defines a `Copy + Clone` `<Name>Context` holding `RwSignal`s, a
`provide_context` in its `*Root`, and a `use_<name>()` that `expect_context`s it (`use_tabs`,
`use_dialog`, `use_toast`, `use_theme`, …). Call these only inside the corresponding root.

**Floating layer**: `popover`, `select`, and `dropdown_menu` all alias the shared `FloatingContext`
from `src/primitives/floating.rs` (`pub type PopoverContext = FloatingContext;`) and reuse
`FloatingRoot` / `FloatingTrigger` / `FloatingProvider`. Positioning goes through `floating-ui-leptos`
with Offset/Flip/Shift middleware; `utils::get_placement(side, align)` maps `Side` + `Align` to a
`Placement`, and `src/middleware/mod.rs` adds a `TransformOrigin` middleware for animation origins.
`FloatingContext` carries both `is_open` and `is_mounted`: `is_mounted` stays true for
`unmount_delay` (150ms) after close so exit animations can run — never key content on `is_open` alone.

**Styling**: the `cn!` macro (`src/utils/cn.rs`, exported at crate root, `use crate::cn`) joins classes
and runs `tw_merge`, which is coarser than the JS `tailwind-merge` in places — `overflow-x-*` and
`overflow-y-*` are one group, so writing both keeps only the last. The consistent idiom is a derived
signal so the caller's `class` wins:

```rust
class=move || cn!("base classes here", class.get())
```

A bare `duration-*` sets `transition-duration` as well as `--tw-duration`, and `transition-property`
defaults to `all` — so on a floating layer it transitions the shell's `visibility: hidden` away over
that duration, and anything focused inside it in the meantime silently stays unfocused. Use
`animation-duration-*` (from `animate.css`) when only the enter/exit animation is meant.

**Variants** are plain enums implementing `AsClass` (`src/utils/types.rs`) returning a `&'static str`
of Tailwind classes — e.g. `ButtonVariant`, `ButtonSize`, `BadgeVariant`, `ToastVariant`. Shared enums
`Side`, `Align`, `Orientation`, `State`, `Direction`, `SideOffset` also live in `utils/types.rs` and
expose `as_str()` for rendering into `data-*` attributes.

**State is expressed as data attributes**, not conditional classes: `data-state` (`open`/`closed`,
`checked`/`unchecked`, `active`/`inactive`), `data-side`, `data-align`, `data-orientation`,
`data-slot`, `data-variant`. Styling then targets them via `data-[state=open]:…` or the
`@custom-variant`s declared in `docs/styles/globals.css`.

**Polymorphic children (`asChild`)**: components that may render as something else take an optional
`render` / `as_child: Callback<(AnyNodeRef, Callback<MouseEvent>), AnyView>` and branch with
`Either::Left/Right`. The node ref (`leptos_node_ref::AnyNodeRef`) must be forwarded so floating
positioning can measure the trigger.

**Prop conventions**: `#[prop(optional, into)] class: Signal<String>`, `#[prop(optional, into)] disabled: Signal<bool>`,
`children: Option<ChildrenFn>` when children may be re-rendered inside a `Portal` or `Either` branch.
An optional callback or value prop is `#[prop(default = None, into)] x: Option<T>` — `optional` on an
`Option<T>` makes the setter take `T`, and `&str` will not coerce into `Option<String>`; where the
empty string can stand in for "unset" (a label, a heading), prefer `Signal<String>`.

**Pointer gestures** go through `use_drag` (`src/primitives/drag.rs`), never through hand-rolled
window listeners: it owns the listener lifecycle (including `pointercancel`, which every hand-rolled
copy missed) and hands each move back as a `DragPoint` carrying the client position, the delta since
the press, and the position as a fraction of a reference box. `.against(node_ref)` names that box
when the press lands on a wrapper rather than the surface; `.on_start` is where a click and a drag
become one gesture; `.on_end` gets the release velocity and how long the pointer had been still, so
a flick can be told from a placement. Guard the press in your own `on:pointerdown` and then call
`drag.start(&e)`.

**Not everything is a context.** `TableSort` and `TableSelection` (`primitives/table.rs`) are plain
`Copy` handles the caller passes where they are needed: a table already has its rows and columns in
one place, so a context would only hide the wiring. Use a context when parts have to find each other
across a portal or an unknown depth, and a handle when they do not.

**A line's width belongs in pixels, not percent.** The cropper's rule-of-thirds was drawn with
gradient stops 0.17% apart, which is a fraction of a device pixel on a small crop: browsers round
it away and bring it back on zoom, so the lines flickered. Anything hairline-thin — a grid, a
divider, a focus ring — takes `w-px` / `h-px` and is *positioned* in percent.

**Colour and images**: `utils::color` (`Rgba`/`Hsva`/`Hsla`/`Oklch`, parsing, formatting),
`utils::image` (a normalised crop rectangle, and a canvas for the output) and `utils::gif`
(cropping an animated GIF at the LZW level, so palettes and timing survive). All three are
arithmetic with unit tests, and none of them is a dependency — the same call as `utils::date`. The
one thing a unit test cannot check is a *format* we emit: an encoder and a decoder written together
will agree on a stream nobody else can read, so `utils::gif` is verified by handing its output to a
real browser (`dump_for_a_real_decoder`, ignored by default).

**Dates**: `utils::date::Date` is a 12-byte civil date (`year`, `month`, `day`) with Hinnant's
day-count conversions under `to_days` / `from_days`, so month ends and leap days are never special
cases. Deliberately not a date-time library and deliberately not `chrono` — a UI library should not
hand its consumers a version to agree with. English month and weekday names live there too.

## Theming & CSS

All Tailwind v4 configuration is CSS-side in `docs/styles/globals.css`: `@theme inline` token
definitions, `@custom-variant dark (&:is(.dark *))`, radius scale derived from `--radius`, and
crucially `@source "../../src"` so Tailwind scans the library's Rust sources for class names. Classes
that only exist in `src/` and are never referenced from the docs app still get generated — but only
as literal strings; do not build class names dynamically. `styles/animate.css` is a v4-compatible
port of `tailwindcss-animate` (`animate-in`, `fade-in-0`, `slide-in-from-top-2`, …).

`ThemeProvider` (`src/utils/theme.rs`, `theme` feature) reads/writes the `ui-theme` localStorage key,
resolves `Theme::System` via `prefers-color-scheme`, and toggles the `dark` class on `<html>`.

## Docs site

`docs/src/app.rs` wraps everything in `ThemeProvider` → `ToastProvider` → `Router`. Adding a docs
page means touching five places: the new `docs/src/pages/docs/<name>.rs` (a `Page` component
using `DocLayout` + `DemoSection`), `pages/docs/mod.rs`, the route in `app.rs`, the `NAV` const in
`docs/src/layout/doc_layout.rs`, and the `COMPONENTS` const in `pages/docs/index.rs`. The header's
⌘K palette (`docs/src/components/doc_search.rs`) reads `NAV`, so it is not a sixth place.

`DemoSection` takes an optional `code` prop that renders the snippet below the demo. **Put snippets in
module-level consts, never inline in `view!`** — leptosfmt reformats raw strings inside the macro,
flattening their indentation, and the damage lands in what the copy button gives the reader. Snippets
are normally the demo's own markup, dedented; keeping them identical is what stops them drifting.

## Repo notes

Rust edition 2024 (let-chains are used, e.g. in `theme.rs`). The working copy is a colocated
jujutsu repo (`.jj/`) alongside `.git/`; normal git commands work.
