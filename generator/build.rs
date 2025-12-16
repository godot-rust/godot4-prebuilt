/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

// This crate is just a shim around godot-bindings.
// See also gdext's godot-ffi/build.rs which is similar in nature.

// If you want to build this crate locally, you need to do two things from the repo root prior to build:
//   cd .generated
//   godot4 --headless --dump-gdextension-interface

use std::path::Path;

use godot_bindings::TargetPointerWidth;

fn main() {
    let gen_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../.generated"));
    let header_c_path = gen_path.join("gdextension_interface.h");

    // Generate bindings for both 64-bit and 32-bit targets.
    // This allows proper layout tests on both architectures without runtime stripping.
    // See: https://github.com/godot-rust/gdext/issues/347

    // Output paths - CI workflow will rename these with platform prefix (e.g., linux_64, linux_32).
    // Windows and macOS only need 64-bit; Linux/wasm needs both.
    let header_rs_64_path = gen_path.join("gdextension_interface_64.rs");
    let header_rs_32_path = gen_path.join("gdextension_interface_32.rs");

    // Note: do not call clear_dir(), as we pass in the C header into the gen directory.

    // Generate 64-bit bindings.
    godot_bindings::write_gdextension_headers_for_target(
        &header_c_path,
        &header_rs_64_path,
        TargetPointerWidth::Bits64,
    );
    println!("Generated 64-bit bindings: {}", header_rs_64_path.display());

    // Generate 32-bit bindings.
    godot_bindings::write_gdextension_headers_for_target(
        &header_c_path,
        &header_rs_32_path,
        TargetPointerWidth::Bits32,
    );
    println!("Generated 32-bit bindings: {}", header_rs_32_path.display());
}
