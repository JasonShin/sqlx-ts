#!/usr/bin/env bash
# Usage: build-npm-platform-packages.sh <tag> <zip-dir> <out-dir>
# Builds one npm package per release zip into <out-dir> and adds them to node/package.json as optionalDependencies
set -euo pipefail

tag=$1
zip_dir=$2
out_dir=$3
version="${tag#v}"
node_dir="$(cd "$(dirname "$0")/../node" && pwd)"

for entry in macos-arm:darwin:arm64 macos-64-bit:darwin:x64 linux-64-bit:linux:x64 linux-arm:linux:arm64 \
  linux-32-bit:linux:ia32 windows-64-bit:win32:x64 windows-32-bit:win32:ia32; do
  IFS=: read -r build os cpu <<< "$entry"
  name="sqlx-ts-$os-$cpu"
  dir="$out_dir/$name"
  mkdir -p "$dir"
  unzip -oqj "$zip_dir/sqlx-ts-$tag-$build.zip" -d "$dir"
  chmod 755 "$dir"/sqlx-ts*
  cat > "$dir/package.json" <<EOF
{
  "name": "$name",
  "version": "$version",
  "description": "The $os $cpu binary of sqlx-ts",
  "repository": {
    "type": "git",
    "url": "git+https://github.com/JasonShin/sqlx-ts.git"
  },
  "license": "MIT",
  "os": ["$os"],
  "cpu": ["$cpu"],
  "preferUnplugged": true
}
EOF
  (cd "$node_dir" && npm pkg set "optionalDependencies.$name=$version")
done
