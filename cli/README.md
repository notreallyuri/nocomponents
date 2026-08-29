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

An `add` never overwrites a file that is already there, and it says *which* kind of file it left
alone — a component still exactly as it was installed reads differently from one you have edited:

```
  kept      src/components/nc/button.rs (edited here)
  kept      src/components/nc/input.rs (unchanged, --force refreshes it)
```

It can tell them apart because `nocomponents.lock`, written beside the config, records a hash of
each file as it was installed. That file is generated — check it in if you want the distinction to
survive a fresh clone, and delete it if you would rather every installed component read as yours.

Pass `--force` when you do want files replaced. It names anything of yours it overwrote.

```bash
cargo nocli components remove button
```

`remove` deletes installed components and refuses twice before it does: once for a component
another installed one is built on, which would leave the project not compiling, and once for a file
you have edited. `--force` overrides both. Cargo features are left alone, since the feature also
switches on the primitive underneath, which something else may still be using.

## Configuration

`nocomponents.toml`, written by `init`:

```toml
[style]
css = "styles/globals.css"

[paths]
components = "src/components/nc"
```

`paths.components` has to be under `src/` — an installed component that is built on another one
needs to be able to name it, and only a directory under `src/` has a module path.

The `mod.rs` in there is written from the directory on every `add` and `remove`, but only between
its two markers:

```rust
// nocli:begin
pub mod button;
// nocli:end

pub mod our_own;   // anything out here is left alone
```

Every `.rs` beside it is declared inside the block, your own files included — so a module you
declare yourself outside it is not declared twice. The components are yours to edit; the list
between the markers is not.

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

## Which version it installs

An installed component is compiled by *your* crate against the `nocomponents` your project depends
on, and it names that crate's internals directly — `nocomponents::primitives::button::ButtonRoot`,
`nocomponents::utils::types::StyledRender`. So the source and the library have to be the same
release, and this tool reads a git tag rather than a branch to make sure of it.

**The tag comes from your project, not from this tool's version.** In order:

1. the `nocomponents` in your `Cargo.lock`, which is the version you actually compile against;
2. the latest release on crates.io, when nothing is resolved yet — every first `init`;
3. a refusal, if your `nocomponents` is a `path` dependency. That is a working tree, and the
   release tagged with the same number is not it — use `--from` to install from that same tree.

So `cargo-nocli` and `nocomponents` version independently: a newer CLI installs correctly for an
older library you have pinned, and a CLI bugfix does not need a library release to go with it.

## Working against a checkout

`--from` reads the components from a local clone instead of the repository, and points the
dependency it writes at that clone. No tag is resolved and nothing is fetched — `--from` already
says which tree you mean. It is how the CLI is tested, and how you try a component you have not
pushed yet.

```bash
cargo nocli components add sidebar --from ../nocomponents
```
