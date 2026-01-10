/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

fn main() {
    let manifest_dir =
        std::env::var("CARGO_MANIFEST_DIR").expect("Cargo should define CARGO_MANIFEST_DIR");

    // Define a variable that can be transported to dependent build scripts.
    // This can be accessed as std::env::var("DEP_GDEXTENSION_API_ROOT") in a build.rs using this crate as a dependency.
    println!("cargo:root={manifest_dir}");
}
