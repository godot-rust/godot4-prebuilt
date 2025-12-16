/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

// Since this lib.rs is repurposed as a module file, avoid crate-level attributes such as
// #![doc(html_logo_url)] here.

//! # GDExtension API for Godot %GODOT4_VER%

use std::borrow::Cow;

/// Abstracts from borrow/owned and allows to change implementation without affecting API.
pub type CowStr = Cow<'static, str>;

/// Version of the Godot engine that the API JSON and C header mirror.
///
/// Note that this currently only contains the `major.minor[.patch]` part, so even `4.2-rc1` would be `4.2` (although pre-releases are currently
/// not published).
pub const GODOT_VERSION_STRING: &str = "%GODOT4_VER%";

/// Returns the contents of the header file `gdextension_interface.h`.
pub const fn load_gdextension_header_h() -> CowStr {
    CowStr::Borrowed(include_str!("../res/gdextension_interface.h"))
}

/// Returns the contents of the header file `gdextension_interface.rs`, generated for the corresponding platform and pointer width.
///
/// The bindings are now generated separately for 32-bit and 64-bit targets to ensure
/// correct layout tests on both architectures. See: https://github.com/godot-rust/gdext/issues/347
///
/// **Important**: This function uses compile-time `#[cfg]` to select bindings, which means it
/// evaluates based on the *current compilation target*, not the cross-compilation target.
/// For cross-compilation scenarios (e.g., build scripts), use [`load_gdextension_header_rs_32`]
/// or [`load_gdextension_header_rs_64`] with `CARGO_CFG_TARGET_POINTER_WIDTH` instead.
// Kept for backward compatibility. New code should use load_gdextension_header_rs_32() or _64().
#[deprecated(
    since = "0.3.1",
    note = "Use load_gdextension_header_rs_32() or load_gdextension_header_rs_64() with CARGO_CFG_TARGET_POINTER_WIDTH for cross-compilation support"
)]
pub const fn load_gdextension_header_rs() -> CowStr {
    // 64-bit platforms: use platform-specific bindings
    #[cfg(all(windows, target_pointer_width = "64"))]
    let s = include_str!("../res/gdextension_interface_windows.rs");

    #[cfg(all(target_os = "macos", target_pointer_width = "64"))]
    let s = include_str!("../res/gdextension_interface_macos.rs");

    #[cfg(all(unix, not(target_os = "macos"), target_pointer_width = "64"))]
    let s = include_str!("../res/gdextension_interface_linux_64.rs");

    // 32-bit platforms: all use the same bindings (Linux, wasm32, etc.)
    //
    // Note: with this, in practce, we only support wasm32 and linux i686.
    // Godot *has* support for other 32-bit platforms. However
    // maintaining and offering *tested* exports for corner cases such
    // as win32 is quite costly. Wasm32 is a real-world use case though.
    #[cfg(target_pointer_width = "32")]
    let s = include_str!("../res/gdextension_interface_linux_32.rs");

    CowStr::Borrowed(s)
}

/// Returns the 32-bit bindings, regardless of current compilation target.
///
/// Use this in build scripts with `CARGO_CFG_TARGET_POINTER_WIDTH` to select
/// the correct bindings for cross-compilation.
pub const fn load_gdextension_header_rs_32() -> CowStr {
    CowStr::Borrowed(include_str!("../res/gdextension_interface_linux_32.rs"))
}

/// Returns the 64-bit bindings for the current platform.
///
/// Use this in build scripts with `CARGO_CFG_TARGET_POINTER_WIDTH` to select
/// the correct bindings for cross-compilation.
///
/// Note: Returns platform-specific bindings (Windows/macOS/Linux) based on compile-time cfg.
/// For cross-compilation to a different OS, this may not be accurate, but struct layouts
/// are identical across 64-bit platforms.
pub const fn load_gdextension_header_rs_64() -> CowStr {
    #[cfg(windows)]
    let s = include_str!("../res/gdextension_interface_windows.rs");

    #[cfg(target_os = "macos")]
    let s = include_str!("../res/gdextension_interface_macos.rs");

    #[cfg(all(unix, not(target_os = "macos")))]
    let s = include_str!("../res/gdextension_interface_linux_64.rs");

    // Fallback for non-unix, non-windows, non-macos (unlikely but safe)
    #[cfg(not(any(windows, unix)))]
    let s = include_str!("../res/gdextension_interface_linux_64.rs");

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
        "rust_version_string" => Cow::Borrowed("%RUST_VER%"),
        "bindgen_version_string" => Cow::Borrowed("%BINDGEN_VER%"),
        _ => return None,
    };

    Some(value)
}
