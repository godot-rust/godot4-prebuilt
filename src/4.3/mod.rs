/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! # GDExtension API for Godot 4.3

use std::borrow::Cow;

/// Abstracts from borrow/owned and allows to change implementation without affecting API.
pub type CowStr = Cow<'static, str>;

/// Version of the Godot engine that the API JSON and C header mirror.
///
/// Note that this currently only contains the `major.minor[.patch]` part, so even `4.2-rc1` would be `4.2` (although pre-releases are currently
/// not published).
pub const GODOT_VERSION_STRING: &str = "4.3";

/// Returns the contents of the JSON API file `extension_api.json`.
pub const fn load_gdextension_json() -> CowStr {
    Cow::Borrowed(include_str!("extension_api.json"))
}

/// Dynamically fetch a property of this crate.
pub fn get_package_property(key: &str) -> Option<CowStr> {
    let value = match key {
        "godot_version_string" => Cow::Borrowed(GODOT_VERSION_STRING),
        _ => return None,
    };

    Some(value)
}
