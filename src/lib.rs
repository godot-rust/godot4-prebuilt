/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

// Since this lib.rs is repurposed as a module file, avoid crate-level attributes such as
// #![doc(html_logo_url)] here.

//! # GDExtension API for Godot 4.2

use std::borrow::Cow;

/// Abstracts from borrow/owned and allows to change implementation without affecting API.
pub type CowStr = Cow<'static, str>;

/// Version of the Godot engine that the API JSON and C header mirror.
///
/// Note that this currently only contains the `major.minor[.patch]` part, so even `4.2-rc1` would be `4.2` (although pre-releases are currently
/// not published).
pub const GODOT_VERSION_STRING: &str = "4.2";

/// Target platform for which the GDExtension interface is generated.
///
/// Since the `gdextension-api` crate is typically used as a build dependency, it won't have compile-time access to the build target
/// (only runtime via `CARGO_CFG_TARGET_*` env vars). For more flexibility and testing, this choice is delegated to the caller.
///
/// See [`load_gdextension_header_rs_for_platform()`] for usage.
#[non_exhaustive]
pub enum TargetPlatform {
    Windows,
    /// Includes iOS.
    MacOS,
    /// Includes other Unix-like systems such as BSD and Android.
    Linux,
    Wasm,
}

/// Returns the contents of the header file `gdextension_interface.h`.
pub const fn load_gdextension_header_h() -> CowStr {
    CowStr::Borrowed(include_str!("../../4.2/res/gdextension_interface.h"))
}

/// Returns the contents of the header file `gdextension_interface.rs`, generated for the corresponding platform.
#[deprecated = "Wrongly dispatches based on host and doesn't support Wasm. Use load_gdextension_header_rs_for_platform() instead."]
pub const fn load_gdextension_header_rs() -> CowStr {
    #[cfg(windows)]
    let s = include_str!("../../4.2/res/gdextension_interface_windows.rs");

    #[cfg(target_os = "macos")]
    let s = include_str!("../../4.2/res/gdextension_interface_macos.rs");

    #[cfg(all(unix, not(target_os = "macos")))]
    let s = include_str!("../../4.2/res/gdextension_interface_linux.rs");

    CowStr::Borrowed(s)
}

/// Returns the contents of the header file `gdextension_interface.rs`, generated for the corresponding platform.
pub fn load_gdextension_header_rs_for_platform(platform: TargetPlatform) -> CowStr {
    let s = match platform {
        TargetPlatform::Windows => include_str!("../../4.2/res/gdextension_interface_windows.rs"),
        TargetPlatform::MacOS => include_str!("../../4.2/res/gdextension_interface_macos.rs"),
        TargetPlatform::Linux => include_str!("../../4.2/res/gdextension_interface_linux.rs"),
        TargetPlatform::Wasm => include_str!("../../4.2/res/gdextension_interface_wasm.rs"),
    };

    CowStr::Borrowed(s)
}

/// Returns the contents of the JSON API file `extension_api.json`.
pub const fn load_gdextension_json() -> CowStr {
    Cow::Borrowed(include_str!("../res/extension_api.json"))
}

/// Dynamically fetch a property of this crate.
pub fn get_package_property(key: &str) -> Option<CowStr> {
    let value = match key {
        "godot_version_string" => Cow::Borrowed(GODOT_VERSION_STRING),
        "rust_version_string" => Cow::Borrowed("1.92.0"),
        "bindgen_version_string" => Cow::Borrowed("0.72.1"),
        _ => return None,
    };

    Some(value)
}
