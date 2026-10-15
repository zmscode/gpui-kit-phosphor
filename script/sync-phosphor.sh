#!/usr/bin/env bash
# Refresh assets/ from a phosphor-icons/core release.
#
#   script/sync-phosphor.sh            # latest main
#   script/sync-phosphor.sh v2.0.8     # or a tag / branch
#
# Upstream ships `assets/<weight>/<name>-<weight>.svg` (regular has no suffix);
# this crate stores them as `assets/<weight>/<name>.svg`.
set -euo pipefail

REF="${1:-main}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

git clone --quiet --depth 1 --branch "$REF" https://github.com/phosphor-icons/core "$TMP/core"

rm -rf "$ROOT/assets"
for weight in thin light regular bold fill duotone; do
    mkdir -p "$ROOT/assets/$weight"
    for file in "$TMP/core/assets/$weight"/*.svg; do
        name="$(basename "$file" .svg)"
        [ "$weight" = regular ] || name="${name%-$weight}"
        cp "$file" "$ROOT/assets/$weight/$name.svg"
    done
    echo "$weight: $(ls "$ROOT/assets/$weight" | wc -l | tr -d ' ') icons"
done
cp "$TMP/core/LICENSE" "$ROOT/LICENSE-PHOSPHOR"
echo "Synced Phosphor $REF ($(git -C "$TMP/core" rev-parse --short HEAD))"
