#!/usr/bin/env bash
# `dx bundle` 把 dist/public 增量写入，旧构建的哈希资源（含 CSS）会堆积。
# 只保留 index.html 可达的闭包：html → js → wasm → 其它被引用的哈希资源。
# CSS 在 dx bundle 之前已用 tailwindcss 显式编译，index.html 会引用其哈希名。
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
    # -a：胶水 js 内嵌二进制数据，普通 grep 会判定为二进制
    cat "$js" >> "$haystack"
    wasm=$(grep -ao 'assets/jiajun-homepage_bg-[A-Za-z0-9_-]*\.wasm' "$js" | head -n 1 || true)
    if [ -n "$wasm" ] && [ -f "$wasm" ]; then
        cat "$wasm" >> "$haystack"
    fi
fi

shopt -s nullglob
for f in assets/*; do
    [ -f "$f" ] || continue
    # -a：haystack 含 wasm 二进制片段
    grep -aqF "$(basename "$f")" "$haystack" || rm -f "$f"
done
