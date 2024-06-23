#!/bin/bash
# Copyright (c) godot-rust; Bromeon and contributors.
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

version="$1"
runnerTemp="$2"

# if version has already suffix (e.g. -rc1), leave as-is, but replace filePath
if [[ $version =~ "-" ]]; then
    filePath=$(echo "$version" | sed "s!-!/!")
    wordyVersion="$version"
else
    filePath="$version"
    wordyVersion="${version}-stable"
fi

filename=Godot_v${wordyVersion}_linux.x86_64

echo "GODOT4_FILE_URL=$filePath/$filename.zip" >> $GITHUB_ENV
echo "GODOT4_DIR=$runnerTemp/godot_bin" >> $GITHUB_ENV
echo "GODOT4_BIN=$runnerTemp/godot_bin/$filename" >> $GITHUB_ENV
echo "GODOT4_VER=$version" >> $GITHUB_ENV
echo "GODOT4_UPSTREAM_VER=$wordyVersion" >> $GITHUB_ENV

