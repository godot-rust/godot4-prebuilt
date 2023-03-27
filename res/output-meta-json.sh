#!/bin/bash
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

cat <<HEREDOC
{
    "protocol": 1,
    "versions": {
        "godot": "$GODOT4_VER",
        "godot_full": "$GODOT4_FULL_VER",
        "rust": "$RUST_VER",
        "bindgen": "$BINDGEN_VER"
    },
    "created_ts": $(date +%s)
}
HEREDOC
