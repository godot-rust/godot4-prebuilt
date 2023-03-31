/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

// Output of generated code. Mimics the file structure, symbols are re-exported.
#[rustfmt::skip]
#[allow(
dead_code,
    deref_nullptr,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::redundant_static_lifetimes,
)]
#[path = "../../.generated/gdextension_interface.rs"]
mod gdextension_interface;


// Sanity check, ensures that symbols are available
trait Distinct {}

// This only compiles if those are different types -- ensures type safety through patch
impl Distinct for gdextension_interface::GDExtensionVariantPtr {}
impl Distinct for gdextension_interface::GDExtensionTypePtr {}
impl Distinct for gdextension_interface::GDExtensionConstTypePtr {}
