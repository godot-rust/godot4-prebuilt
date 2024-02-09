/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

// Note: this API is not necessarily forward-compatible. It can be updated on demand (for all published Godot versions).
// Important is that the latest version of gdext can be used together with all latest published artifacts.
// For long-term stability, it may be helpful to have a dynamic query mechanism, such as:
//   fn get_property(key: &str) -> Option<String>

/// Version of the Godot engine that the API JSON and C header mirror.
/// Note that this currently only contains the `major.minor[.patch]` part, so even `4.2-rc1` will be `4.2`.
pub const GODOT_VERSION: &str = "4.3";

/// Version of the Rust compiler used to build this artifact.
/// Is still a number even for nightly versions.
pub const RUST_VERSION: &str = "1.76.0";

/// Version of the `bindgen` crate used to generate the `gdextension_interface.h` Rust binding.
pub const BINDGEN_VERSION: &str = "0.65.1";

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
