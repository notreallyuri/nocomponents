# nocomponents

Unstyled behaviour primitives for [Leptos](https://leptos.dev) 0.8, in the spirit of Radix, plus a
styled layer that mirrors [shadcn/ui](https://ui.shadcn.com). 66 components, no npm, CSR.

**[Documentation and live demos →](https://notreallyuri.github.io/nocomponents/)**

> Early. The API moves between 0.1.x releases, and nothing here has been exercised under SSR or
> hydration — the primitives reach for `document()` directly, so an SSR consumer is unexplored
> rather than supported.

## Two layers

`primitives` is behaviour: ARIA roles, state, focus management, keyboard handling, and `data-*`
attributes to style from. No Tailwind classes anywhere in it. Parts are named `<Name>Root`,
`<Name>TriggerRoot`, `<Name>ContentRoot`, and each takes a `class` it renders verbatim.

`components` is the shadcn-styled layer over exactly that tree — same parts, named `<Name>`,
`<Name>Trigger` — passing classes down into the matching `*Root`.

Take the first if you have your own design system. Take the second if you want shadcn's, and edit
it, because you will own the file.

## Install

The two layers install differently, and the reason is Tailwind.

A Tailwind build only emits a class if it can see the file that class is written in, and it does
not look inside other crates. A project that merely *depends* on the styled layer gets its markup
with none of its CSS, and nothing on the project's side fixes that — `@source` would have to point
into `~/.cargo/registry`, and the tokens the classes are written against would still be missing.

So the styled layer is **installed, not depended on**:

```bash
cargo install nocomponents-cli

cd my-leptos-app
cargo nocli init
cargo nocli components add button dialog
```

`init` writes `nocomponents.toml`, installs `globals.css` and `animate.css`, points `Trunk.toml` at
a Tailwind, and adds the dependency. `components add` copies each component into your own `src/`,
where Tailwind already scans it, together with anything it is built on — `sidebar` brings `sheet`,
`skeleton`, `tooltip`, `separator` and `input` — and turns on the matching Cargo features. It never
overwrites a file you have edited unless you pass `--force`.

Everything with no classes in it stays an ordinary dependency: the primitives, the icons, `utils`,
`middleware`. If you only want those, skip the CLI:

```toml
[dependencies]
nocomponents = { version = "0.1", features = ["primitives", "dialog", "tooltip"] }
```

See [the CLI's README](https://github.com/notreallyuri/nocomponents/blob/main/cli/README.md) for
the rest — `components list`, `components remove`, the
lockfile, and what gets rewritten in an installed file.

## A styled component

```rust
use leptos::prelude::*;
use nocomponents::components::{
    button::{Button, ButtonVariant},
    dialog::{Dialog, DialogDescription, DialogFooter, DialogHeader, DialogPortal, DialogTitle,
             DialogTrigger},
};

view! {
    <Dialog>
        <DialogTrigger variant=ButtonVariant::Destructive>"Delete account"</DialogTrigger>
        <DialogPortal>
            <DialogHeader>
                <DialogTitle>"Are you absolutely sure?"</DialogTitle>
                <DialogDescription>
                    "This action cannot be undone."
                </DialogDescription>
            </DialogHeader>
            <DialogFooter class="mt-4">
                <Button variant=ButtonVariant::Outline>"Cancel"</Button>
                <Button variant=ButtonVariant::Destructive>"Delete account"</Button>
            </DialogFooter>
        </DialogPortal>
    </Dialog>
}
```

## The same thing unstyled

The primitive is the same tree with the classes left to you. State is on the element as `data-*`,
never as a class the component picked, so `data-[state=open]:` is the whole styling contract:

```rust
use nocomponents::primitives::collapsible::{
    CollapsibleContentRoot, CollapsibleRoot, CollapsibleTriggerRoot,
};

view! {
    <CollapsibleRoot>
        <CollapsibleTriggerRoot class="my-trigger">"Details"</CollapsibleTriggerRoot>
        <CollapsibleContentRoot class="my-panel">"…"</CollapsibleContentRoot>
    </CollapsibleRoot>
}
```

## Features

Two kinds, and you almost always want some of each.

**Layers** gate the modules:

| feature | what it turns on |
| --- | --- |
| `primitives` | behaviour, ARIA, `data-*` — no classes |
| `icons` | the icon set the styled layer draws |
| `components` | the styled layer (implies `primitives` + `icons`) |
| `theme` | `ThemeProvider`, light/dark/system over `localStorage` |
| `highlight` | syntax highlighting for the `code` component |
| `full` | all of the above |

**Elements** are one feature per component name — 72 of them, and a single name gates both layers,
because `accordion` is one thing whichever layer you take it from. Each lists what its own source
imports, so cargo works out the closure:

```toml
# exactly the dialog primitive and what it needs, and nothing else
nocomponents = { version = "0.1", features = ["primitives", "dialog"] }
```

With no features at all only `utils`, `middleware` and `deps` exist, so `--features full` is what
you want when checking the whole thing. `cargo nocli` sets these for you.

## Beyond the shadcn catalogue

Twelve components are ours, and none of them took a dependency: `canvas` (an infinite pan-and-zoom
surface), `kanban`, `timeline`, `color-picker`, `image-cropper` (which crops animated GIFs at the
LZW level, so palettes and timing survive), `dropzone`, `tree`, `video-player`, `floating-menu`,
`code`, `native-select`, and `toast`.

So are the utilities they are built on — `utils::color`, `utils::image`, `utils::gif`,
`utils::date` (a 12-byte civil date, deliberately not `chrono`, so this crate hands you no date
library to agree with), `utils::viewport`, and `primitives::drag`.

## Contributing and development

```bash
cargo check --features full          # the library — default features compile almost nothing
cargo test --features full           # unit tests, and the feature-table check
cd docs && trunk serve               # the docs site on :3000, which is how UI is verified
```

`leptosfmt` formats `view!` blocks; run it only on files you changed, since 0.1.33 deletes Rust
comments inside a `view!`.

## Licence and credit

Apache-2.0. The styled layer's design, class choices and component names come from
[shadcn/ui](https://ui.shadcn.com), and the primitives' behaviour follows
[Radix UI](https://www.radix-ui.com); both are MIT, and their notices are reproduced in
[`NOTICE`](https://github.com/notreallyuri/nocomponents/blob/main/NOTICE) as that licence
requires. The icons are [Lucide](https://lucide.dev), ISC.

This is a re-implementation for Leptos, not a port of their code.
