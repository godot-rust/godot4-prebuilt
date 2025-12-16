/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

// Output of generated code. Mimics the file structure, symbols are re-exported.
// We now generate separate bindings for 64-bit and 32-bit targets (Linux/wasm only for 32-bit).
// See: https://github.com/godot-rust/gdext/issues/347

// The generator runs on 64-bit Linux and produces both _64.rs and _32.rs files.
// For local builds, we select based on host pointer width.
#[rustfmt::skip]
#[allow(
    dead_code,
    deref_nullptr,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::redundant_static_lifetimes,
)]
#[cfg(target_pointer_width = "64")]
#[path = "../../.generated/gdextension_interface_64.rs"]
mod gdextension_interface;

#[rustfmt::skip]
#[allow(
    dead_code,
    deref_nullptr,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::redundant_static_lifetimes,
)]
#[cfg(target_pointer_width = "32")]
#[path = "../../.generated/gdextension_interface_32.rs"]
mod gdextension_interface;


// Sanity check, ensures that symbols are available.
#[allow(dead_code)]
trait Distinct {}

// This only compiles if those are different types -- ensures type safety through patch.
impl Distinct for gdextension_interface::GDExtensionVariantPtr {}
impl Distinct for gdextension_interface::GDExtensionTypePtr {}
impl Distinct for gdextension_interface::GDExtensionConstTypePtr {}

#[cfg(test)]
mod tests {
    use super::gdextension_interface::*;

    /// Verify that key FFI types are exported and have the expected properties.
    #[test]
    fn test_ffi_types_exist() {
        // These types should exist in the generated bindings
        let _: GDExtensionVariantPtr;
        let _: GDExtensionTypePtr;
        let _: GDExtensionConstTypePtr;
        let _: GDExtensionBool;
        let _: GDExtensionInt;
    }

    /// Verify pointer types are distinct (type safety through patch).
    #[test]
    fn test_pointer_types_are_distinct() {
        use std::any::TypeId;

        // These should be different types (ensured by the Distinct trait above,
        // but we also verify with TypeId for good measure)
        let variant_ptr_id = TypeId::of::<GDExtensionVariantPtr>();
        let type_ptr_id = TypeId::of::<GDExtensionTypePtr>();
        let const_type_ptr_id = TypeId::of::<GDExtensionConstTypePtr>();

        assert_ne!(variant_ptr_id, type_ptr_id, "VariantPtr and TypePtr should be distinct");
        assert_ne!(variant_ptr_id, const_type_ptr_id, "VariantPtr and ConstTypePtr should be distinct");
        assert_ne!(type_ptr_id, const_type_ptr_id, "TypePtr and ConstTypePtr should be distinct");
    }

    /// Verify that GDExtensionBool has the expected size (1 byte / u8).
    #[test]
    fn test_gdextension_bool_size() {
        assert_eq!(
            std::mem::size_of::<GDExtensionBool>(),
            1,
            "GDExtensionBool should be 1 byte"
        );
    }

    /// Verify that GDExtensionInt has pointer-width size (platform-dependent).
    #[test]
    fn test_gdextension_int_size() {
        // GDExtensionInt is int64_t on 64-bit and int32_t on 32-bit (pointer-sized)
        let expected_size = std::mem::size_of::<isize>();
        assert_eq!(
            std::mem::size_of::<GDExtensionInt>(),
            expected_size,
            "GDExtensionInt should be pointer-sized"
        );
    }
}
