#!/bin/bash
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

# Cargo.lock is versioned to have reproducible builds.
# This script just updates to the latest `gdext` version, without updating other dependencies.

cargo update -p godot-bindings
