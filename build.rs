/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/// Build script to expose the crate's manifest directory to dependent crates.
///
/// This allows dependent crates to locate platform-specific prebuilt bindings
/// during cross-compilation, where cfg attributes are evaluated for the host
/// platform instead of the target platform.
///
/// Dependent crates can access this via the `DEP_GDEXTENSION_API_ROOT` environment variable.
fn main() {
    // Export the manifest directory so dependent crates can find our files
    let manifest_dir =
        std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR should be set by Cargo");

    println!("cargo:root={}", manifest_dir);
}
