//! # Godot 4 GDExtension API, mapped to Rust.
//!
//! Used internally by the [godot-rust](https://godot-rust.github.io) project.
//! We do not offer any guarantees for the provided API.

#![doc(html_logo_url = "https://godotengine.org/assets/press/icon_color.svg")]

use std::borrow::Cow;

/// Returns the contents of `gdextension_interface.json`.
///
/// Describes the GDExtension C API. Backwards-compatible with all supported Godot versions.
pub const fn load_gdextension_interface_json() -> Cow<'static, str> {
    Cow::Borrowed(include_str!("gdextension_interface.json"))
}

#[path = "4.2/mod.rs"]
pub mod version_4_2;
#[path = "4.3/mod.rs"]
pub mod version_4_3;
#[path = "4.4/mod.rs"]
pub mod version_4_4;
#[path = "4.5/mod.rs"]
pub mod version_4_5;
#[path = "4.6/mod.rs"]
pub mod version_4_6;
