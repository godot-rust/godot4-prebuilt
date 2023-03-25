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
#[path = "gen/gdextension_interface.rs"]
mod sys;


// Sanity check, ensures that symbols are available
trait Distinct {}

// This only compiles if those are different types -- ensures type safety through patch
impl Distinct for sys::GDExtensionVariantPtr {}
impl Distinct for sys::GDExtensionTypePtr {}
impl Distinct for sys::GDExtensionConstTypePtr {}
