/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! # Internal crate of [**godot-rust**](https://godot-rust.github.io)
//!
//! Do not depend on this crate directly, instead use the `godot` crate.
//! No SemVer or other guarantees are provided.

#![doc(html_logo_url = "https://godotengine.org/assets/press/icon_color.svg")]

// Note: this API is not necessarily forward-compatible. It can be updated on demand (for all published Godot versions).
// Important is that the latest version of gdext can be used together with all latest published artifacts.
// For long-term stability, it may be helpful to have a dynamic query mechanism, such as:
//   fn get_property(key: &str) -> Option<String>

use std::borrow::Cow;

/// Version of the Godot engine that the API JSON and C header mirror.
///
/// Note that this currently only contains the `major.minor[.patch]` part, so even `4.2-rc1` would be `4.2` (although pre-releases are currently
/// not published).
pub const GODOT_VERSION_STRING: &str = "4.3";

/// Returns the contents of the header file `gdextension_interface.h`.
pub const fn load_gdextension_header_h() -> &'static str {
    include_str!("../res/gdextension_interface.h")
}

/// Returns the contents of the header file `gdextension_interface.rs`, generated for the corresponding platform.
pub const fn load_gdextension_header_rs() -> &'static str {
    #[cfg(windows)]
    {
        include_str!("../res/gdextension_interface_windows.rs")
    }
    #[cfg(target_os = "macos")]
    {
        include_str!("../res/gdextension_interface_macos.rs")
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        include_str!("../res/gdextension_interface_linux.rs")
    }
}

/// Returns the contents of the JSON API file `extension_api.json`.
pub const fn load_gdextension_json() -> &'static str {
    include_str!("../res/extension_api.json")
}

/// Dynamically fetch a property of this crate.
pub fn get_package_property(key: &str) -> Option<Cow<'static, str>> {
    let value = match key {
        "godot_version_string" => Cow::Borrowed(GODOT_VERSION_STRING),
        "rust_version_string" => Cow::Borrowed("1.79.0"),
        "bindgen_version_string" => Cow::Borrowed("0.68.1"),
        _ => return None,
    };

    Some(value)
}
