#!/usr/bin/env bash
# `dx bundle` writes dist/public incrementally; stale hashed assets (including CSS) pile up.
# Keep only the closure reachable from index.html: html → js → wasm → other referenced hashed assets.
# CSS is compiled with tailwindcss before dx bundle; index.html references its hashed name.
set -euo pipefail
cd "$(dirname "$0")/dist/public"

if [ ! -f index.html ]; then
    echo "prune-dist: no index.html, nothing to prune" >&2
    exit 1
fi

haystack=$(mktemp)
trap 'rm -f "$haystack"' EXIT
cat index.html > "$haystack"

js=$(grep -ao 'assets/[^"]*\.js' index.html | head -n 1 || true)
if [ -n "$js" ] && [ -f "$js" ]; then
    # -a: the glue js embeds binary data, so plain grep treats it as binary
    cat "$js" >> "$haystack"
    wasm=$(grep -ao 'assets/jiajun-homepage_bg-[A-Za-z0-9_-]*\.wasm' "$js" | head -n 1 || true)
    if [ -n "$wasm" ] && [ -f "$wasm" ]; then
        cat "$wasm" >> "$haystack"
    fi
fi

shopt -s nullglob
for f in assets/*; do
    [ -f "$f" ] || continue
    # -a: the haystack contains wasm binary fragments
    grep -aqF "$(basename "$f")" "$haystack" || rm -f "$f"
done
