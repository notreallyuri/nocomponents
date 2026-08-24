#[cfg(feature = "components")]
pub mod components;

#[cfg(feature = "primitives")]
pub mod primitives;

#[cfg(feature = "icons")]
pub mod icons;

pub mod middleware;
pub mod utils;

/// The crates the styled layer's own source names, re-exported.
///
/// `src/components/` is not only compiled from here — the CLI copies it into a consumer's project,
/// where `use leptos_node_ref::…` resolves against *their* crate graph. Re-exporting the three
/// means an installed component needs one dependency instead of four kept in version lockstep with
/// this one. `leptos` is deliberately not among them: `#[component]` expands to `::leptos::…`
/// paths, so it has to be a real dependency of whoever compiles the file.
pub mod deps {
    pub use js_sys;
    pub use leptos_node_ref;
    pub use tw_merge;
}
