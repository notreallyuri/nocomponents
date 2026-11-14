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
afterwards. So are the icons the components fall back on. What stays an ordinary dependency is
everything with no classes in it — the behaviour primitives, `utils`, `middleware`.

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
`Trunk.toml`, adds the `nocomponents` dependency, and writes a minimal `index.html` (a `rust` link
and a `tailwind-css` link to the installed stylesheet) so `trunk build` works on a fresh project.
It never overwrites an `index.html`, a stylesheet or a `Trunk.toml` that is already there.

The installed `globals.css` is the docs' stylesheet without its font import, so `--font-sans` is
set to `ui-sans-serif, system-ui, sans-serif` rather than the docs' Figtree, which nothing in your
project loads. Import a font and point `--font-sans` at it when you want one.

`components add` copies each component into the install directory, together with anything it is
built on — `sidebar` brings `sheet`, `skeleton`, `tooltip`, `separator` and `input` with it — and
turns on the matching Cargo features. The glyphs those components fall back on (a chevron, a check,
a close cross) and the `Icon` wrapper they are drawn with go into the icon directory beside them. `components list` shows everything available, marking what is
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

Pass `--force` when you do want files replaced. Each one prints `replaced`, and it names anything
of yours it overwrote. `add` touches `Cargo.toml` only when the dependency or its features have to
change.

```bash
cargo nocli components status
```

`status` lists every installed component as `unchanged` (still what was installed), `edited` (yours
now) or `not in lock` (installed before the lock existed, or copied in by hand, so there is nothing
to compare against). After upgrading the `nocomponents` dependency it is the list of what is safe
to refresh with `add --force` and what to diff first.

`status` lists the installed icons too, in their own section.

## Updating

```bash
cargo update -p nocomponents
cargo nocli components update
cargo nocli components update --apply
```

`update` compares every installed component and icon with the release your `Cargo.lock` names, the
same one `add` reads. So the flow is: bump the dependency, `cargo update -p nocomponents`, run
`update` to see what changed, then `--apply`. Each file is fetched and rewritten exactly as `add`
would write it, and then sorted into one row:

```
components in src/components/nc:
  outdated       button
  current        dialog
  edited         select (edited here, and changed upstream — diff it)
  not in lock    tabs (not in lock, cannot tell if edited; `components add <name> --force` records it)
  not in release ours (yours, or dropped by the release)

0 outdated, 1 current, 2 ignored (edited or not in lock), 1 not in the release
run with --apply to update the 1 unchanged ones
```

`outdated` is a file still exactly as it was installed, for which the release has something
different; `current` is one that already matches. Without `--apply` nothing is written.

With `--apply`, only the `outdated` files are rewritten, and the lock records the new hash. Everything
else is left alone and listed at the end, with its reason: `ignored:` for the first two below, and a
`not in the release:` heading for the last.

- `edited` is yours now, so it is never overwritten. When the release also changed it, the row says so and
  you diff it by hand; `status` and `add --force` remain for that.
- `not in lock` has nothing to compare against, so the tool cannot tell whether you edited it.
  `add <name> --force` replaces it and records it.
- `not in release` is a file the release does not have: one you added to the directory yourself, or
  one the release dropped. It is reported and never deleted.

An updated component can now be built on something that is not installed — a new component, or a
new glyph. `--apply` installs those too (printed as `needed by`), turns on their Cargo features, and
rewrites each directory's `mod.rs`.

Before anything is fetched, `update` names the release it is comparing against and where that came
from, because a `Cargo.toml` requirement such as `"0.3.0"` keeps `cargo update` on that release and
the comparison with it:

```
comparing against nocomponents v0.3.0 (from Cargo.lock)
```

`add`, `icons add`, `list` and `init` print the same line (`installing from …`, `listing …`). With
`--from` it reads `… the checkout at <path>`, and with no lock entry yet, `(latest on crates.io)`.

When the release came from `Cargo.lock`, `update` also asks crates.io for the latest stable one
and, if it is newer, ends with:

```
nocomponents 0.6.1 is available; this project is on 0.3.0. To move to it: cargo add nocomponents@0.6 && cargo nocli components update
```

The requirement is `major.minor` for a 0.x release and `major` from 1.0. This is a courtesy: a
request that fails or times out (5 seconds) prints nothing, and `--from` never asks.

The files are fetched up to eight at a time over one HTTP client, and results are sorted before
anything is printed, so the output is the same as a sequential run. On a terminal, stderr shows a
single `checking 12/49…` line that is erased when the fetching ends; when stderr is piped nothing
extra is written.

`update` first checks that the release exists, and stops with an error if it cannot be reached or
is not tagged, rather than reporting every file as missing from it.

Because the comparison is against the release in `Cargo.lock`, a project that depends on
`nocomponents` by path gets the same refusal as `add`; use `--from <path>`.

## Icons

```bash
cargo nocli icons list        # every glyph, * marks the installed ones
cargo nocli icons add x plus  # install more, for your own use
```

Every component takes its glyphs through an `icon` prop, so you can hand it anything. The installed
icons are what it draws when you do not: redraw `src/icons/nc/chevron.rs` and every component that
falls back on a chevron draws yours. Because they are yours to redraw, a component's `--force`
refreshes an icon that is unchanged but never one you have edited; `icons add <name> --force`
replaces it.

```bash
cargo nocli icons remove x
```

`icons remove` mirrors `components remove`: it deletes the glyph, forgets it in the lock and
rewrites `mod.rs`. It refuses for an icon that an installed component or another icon still
draws (`kept ... (used by dialog)`) and for one you have edited; `--force` overrides both. The
`icon` wrapper is used by every glyph, so it is refused until they are gone. Names are lowercase
letters, digits and `_`; anything that could be a path is rejected.

A project's own icon set belongs beside `nc`, not inside it (`src/icons/app.rs`, declared in
`src/icons/mod.rs` outside the nocli block), so nothing the CLI writes ever touches it.

Installing from a release before 0.4, which ships no icon sources, keeps the old behaviour: the
components draw `nocomponents::icons::…` and the `icons` feature is turned on.

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
icons = "src/icons/nc"
```

Both paths have to be under `src/` — an installed component needs to be able to name what it is
built on and the icons it draws, and only a directory under `src/` has a module path. A config
written before `icons` existed gets the default.

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

`add` also declares the install directory in the module above it: `src/components/mod.rs` is
created with `pub mod nc;` when there is none, and when there is one (or a `src/components.rs`)
the line is added inside a `// nocli:begin` / `// nocli:end` block, never over what you wrote. A
deeper directory is declared level by level. The crate root (`main.rs` / `lib.rs`) is yours and is
not edited: if it does not declare the first segment, `add` prints what to put there
(`mod components;`).

## What gets rewritten

An installed file is compiled by your crate, so every path in it is redirected:

| in the library | as installed |
| --- | --- |
| `crate::components::button` | `crate::components::nc::button` |
| `crate::icons::chevron` | `crate::icons::nc::chevron` |
| `crate::primitives`, `crate::utils`, `crate::cn` | `nocomponents::…` |
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
