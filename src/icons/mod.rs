//! The glyphs the styled layer falls back on, and the [`Icon`] wrapper they are drawn with.
//!
//! **Not a dependency.** Like `src/components/`, this directory is installed: `cargo nocli`
//! copies the glyphs a component needs into the project (`paths.icons`, `src/icons/nc` by
//! default) beside the components, and the installed components name them there. The project
//! owns them from then on — edit `chevron.rs` and every component that falls back on a chevron
//! draws the new one, or ignore them and hand each component its own icon through its `icon`
//! slot. Nothing here is ever a reason for a project to depend on an icon crate.
//!
//! The path data is [Lucide](https://lucide.dev)'s, under the ISC licence.

pub mod calendar;
pub mod check;
pub mod chevron;
pub mod circle;
pub mod copy;
pub mod ellipsis;
pub mod grip;
pub mod icon;
pub mod loader;
pub mod media;
pub mod message;
pub mod minus;
pub mod panel;
pub mod paperclip;
pub mod plus;
pub mod search;
pub mod x;

pub use icon::Icon;
