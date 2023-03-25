
version="$1"
runnerTemp="$2"

# if version has already suffix (e.g. -rc1), leave as-is, but replace filePath
if [[ $version =~ "-" ]]; then
    filePath=$(echo "$version" | sed "s!-!/!")
else
    filePath="$version"
    version="${version}-stable"
fi

filename=Godot_v${version}_linux.x86_64

echo "GODOT4_FILE_URL=$filePath/$filename.zip" >> $GITHUB_ENV
echo "GODOT4_DIR=$runnerTemp/godot_bin" >> $GITHUB_ENV
echo "GODOT4_BIN=$runnerTemp/godot_bin/$filename" >> $GITHUB_ENV
echo "GODOT4_VER=$version" >> $GITHUB_ENV