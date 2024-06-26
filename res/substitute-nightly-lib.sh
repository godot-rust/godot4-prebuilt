#!/bin/bash
# Copyright (c) godot-rust; Bromeon and contributors.
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

file=$1

if [[ -z $file ]]; then
    echo "Usage: $0 <libRsFile>"
    exit 1
fi


# Extract "major.minor[.patch]" from full version, write into GODOT4_VER (which is 'nightly', but lib.rs needs number)
export GODOT4_VER_NUMERIC="$(echo $GODOT4_FULL_VER | sed -E 's!([0-9]+\.[0-9]+(\.[0-9]+)?).*!\1!')"

sed -i "s/%GODOT4_VER%/${GODOT4_VER_NUMERIC}/" $file
sed -i "s/%RUST_VER%/${RUST_VER}/" $file
sed -i "s/%BINDGEN_VER%/${BINDGEN_VER}/" $file



# Addendum for nightly prebuilt -- must be importable as gdextension_api::version_4_x, so we re-export those modules.
# List multiple versions to not maintain this all the time.
cat <<EOF >> $file
pub mod version_4_2 {
    pub use super::*;
}
pub mod version_4_3 {
    pub use super::*;
}
pub mod version_4_4 {
    pub use super::*;
}
pub mod version_4_5 {
    pub use super::*;
}
pub mod version_4_6 {
    pub use super::*;
}
EOF
