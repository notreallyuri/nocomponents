# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

`nocomponents` is a Leptos 0.8 (CSR-only) UI library for the browser: unstyled behavior primitives in the
spirit of Radix, plus a styled layer that mirrors shadcn/ui. `playground/` is a Trunk + Tailwind v4 docs
site that is both the demo and the only way to exercise the library in a browser.

Cargo workspace: root crate `nocomponents`, member `playground`.

## Commands

```bash
cargo check --features full          # type-check the library (default features compile almost nothing)
cargo check -p playground            # type-check the docs app
cargo fmt && cargo clippy --features full

cd playground && trunk serve         # dev server on :3000, watches ../src too (see Trunk.toml)
cd playground && trunk build --release
```

Trunk downloads and runs the Tailwind CLI itself (pinned in `playground/Trunk.toml`); there is no
`tailwind.config.js` and no npm build step. There is no test suite — verify changes in `trunk serve`.

## Feature flags

`primitives`, `icons`, `components` (= primitives + icons), `theme`, `full`. In `src/lib.rs` **both**
`components` and `primitives` modules are gated on the `primitives` feature, and `theme` gates
`utils::theme`. With no features only `utils` and `middleware` exist, so always check/build with
`--features full` (which is what `playground` depends on).

## Architecture: the two layers

`src/primitives/<name>.rs` — behavior, ARIA roles, state, `data-*` attributes, **no Tailwind classes**.
Components are named `<Name>Root`, `<Name>TriggerRoot`, `<Name>ContentRoot`, … and take
`class: Signal<String>` that they render verbatim.

`src/components/<name>.rs` — the shadcn-styled wrapper: same tree, named `<Name>`, `<Name>Trigger`, …,
passing a `cn!(...)` class down into the matching `*Root`. Some components are style-only and have no
primitive (`skeleton`, `spinner`, `badge`, `card`, `input`, `label`); `alert_dialog` is built on the
dialog primitive. Both modules re-export everything through a `prelude`.

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
and runs `tw_merge`. The consistent idiom is a derived signal so the caller's `class` wins:

```rust
class=move || cn!("base classes here", class.get())
```

**Variants** are plain enums implementing `AsClass` (`src/utils/types.rs`) returning a `&'static str`
of Tailwind classes — e.g. `ButtonVariant`, `ButtonSize`, `BadgeVariant`, `ToastVariant`. Shared enums
`Side`, `Align`, `Orientation`, `State`, `Direction`, `SideOffset` also live in `utils/types.rs` and
expose `as_str()` for rendering into `data-*` attributes.

**State is expressed as data attributes**, not conditional classes: `data-state` (`open`/`closed`,
`checked`/`unchecked`, `active`/`inactive`), `data-side`, `data-align`, `data-orientation`,
`data-slot`, `data-variant`. Styling then targets them via `data-[state=open]:…` or the
`@custom-variant`s declared in `playground/styles/globals.css`.

**Polymorphic children (`asChild`)**: components that may render as something else take an optional
`render` / `as_child: Callback<(AnyNodeRef, Callback<MouseEvent>), AnyView>` and branch with
`Either::Left/Right`. The node ref (`leptos_node_ref::AnyNodeRef`) must be forwarded so floating
positioning can measure the trigger.

**Prop conventions**: `#[prop(optional, into)] class: Signal<String>`, `#[prop(optional, into)] disabled: Signal<bool>`,
`children: Option<ChildrenFn>` when children may be re-rendered inside a `Portal` or `Either` branch.

## Theming & CSS

All Tailwind v4 configuration is CSS-side in `playground/styles/globals.css`: `@theme inline` token
definitions, `@custom-variant dark (&:is(.dark *))`, radius scale derived from `--radius`, and
crucially `@source "../../src"` so Tailwind scans the library's Rust sources for class names. Classes
that only exist in `src/` and are never referenced from the playground still get generated — but only
as literal strings; do not build class names dynamically. `styles/animate.css` is a v4-compatible
port of `tailwindcss-animate` (`animate-in`, `fade-in-0`, `slide-in-from-top-2`, …).

`ThemeProvider` (`src/utils/theme.rs`, `theme` feature) reads/writes the `ui-theme` localStorage key,
resolves `Theme::System` via `prefers-color-scheme`, and toggles the `dark` class on `<html>`.

## Playground

`playground/src/app.rs` wraps everything in `ThemeProvider` → `ToastProvider` → `Router`. Adding a docs
page means touching five places: the new `playground/src/pages/docs/<name>.rs` (a `Page` component
using `DocLayout` + `DemoSection`), `pages/docs/mod.rs`, the route in `app.rs`, the `NAV` const in
`playground/src/layout/doc_layout.rs`, and the `COMPONENTS` const in `pages/docs/index.rs`.

`DemoSection` takes an optional `code` prop that renders the snippet below the demo. **Put snippets in
module-level consts, never inline in `view!`** — leptosfmt reformats raw strings inside the macro,
flattening their indentation, and the damage lands in what the copy button gives the reader. Snippets
are normally the demo's own markup, dedented; keeping them identical is what stops them drifting.

## Repo notes

Rust edition 2024 (let-chains are used, e.g. in `theme.rs`). The working copy is a colocated
jujutsu repo (`.jj/`) alongside `.git/`; normal git commands work.
