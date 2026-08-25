# `cargo nocli`

Installs [`nocomponents`](https://github.com/notreallyuri/nocomponents)' styled layer into a Leptos
project as source the project owns.

## Why it is not just a dependency

A Tailwind build only emits a class if it can see the file the class is written in, and it does not
look inside other crates. A project that merely depends on `nocomponents` gets the styled layer's
markup with none of its CSS, and there is nothing the project can do about it: `@source` would have
to point into `~/.cargo/registry`, and the tokens the classes are written against would still be
missing.

So the styled layer is copied into your `src/`, where Tailwind already scans it, and you own it
afterwards. What stays an ordinary dependency is everything with no classes in it — the behaviour
primitives, the icons, `utils`, `middleware`.

## Install

```bash
cargo install nocomponents-cli
```

## Use

```bash
cd my-leptos-app
cargo nocli init
cargo nocli components add button dialog
```

`init` writes `nocomponents.toml`, installs `globals.css` and `animate.css`, adds `tailwindcss` to
`Trunk.toml`, and adds the `nocomponents` dependency.

`components add` copies each component into the install directory, together with anything it is
built on — `sidebar` brings `sheet`, `skeleton`, `tooltip`, `separator` and `input` with it — and
turns on the matching Cargo features. `components list` shows everything available, marking what is
already installed.

An `add` never overwrites a file that is already there; it says which ones it left alone. Pass
`--force` when you do want them replaced.

## Configuration

`nocomponents.toml`, written by `init`:

```toml
[style]
css = "styles/globals.css"

[paths]
components = "src/components/nc"
```

`paths.components` has to be under `src/` — an installed component that is built on another one
needs to be able to name it, and only a directory under `src/` has a module path. The `mod.rs` in
there is rewritten from the directory on every `add`; the components beside it are yours.

## What gets rewritten

An installed file is compiled by your crate, so every path in it is redirected:

| in the library | as installed |
| --- | --- |
| `crate::components::button` | `crate::components::nc::button` |
| `crate::primitives`, `crate::icons`, `crate::utils`, `crate::cn` | `nocomponents::…` |
| `leptos_node_ref`, `js_sys` | `nocomponents::deps::…` |
| `leptos` | unchanged |

`leptos` is the one exception because `#[component]` expands to `::leptos::…` paths, so it has to be
a real dependency of whoever compiles the file. Everything else goes through `nocomponents`, which
is why an installed component adds one dependency rather than four to keep in version lockstep.

## Working against a checkout

`--from` reads the components from a local clone instead of the repository, and points the
dependency it writes at that clone. It is how the CLI is tested, and how you try a component you
have not pushed yet.

```bash
cargo nocli components add sidebar --from ../nocomponents
```
