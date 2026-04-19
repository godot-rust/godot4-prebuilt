//! # Godot 4 GDExtension API, mapped to Rust.
//!
//! Used internally by the [godot-rust](https://godot-rust.github.io) project.
//! We do not offer any guarantees for the provided API.

#![doc(html_logo_url = "https://godotengine.org/assets/press/icon_color.svg")]
#[path = "../versions/4.2/src/lib.rs"]
pub mod version_4_2;
#[path = "../versions/4.3/src/lib.rs"]
pub mod version_4_3;
#[path = "../versions/4.4/src/lib.rs"]
pub mod version_4_4;
#[path = "../versions/4.5/src/lib.rs"]
pub mod version_4_5;
#[path = "../versions/4.6/src/lib.rs"]
pub mod version_4_6;
