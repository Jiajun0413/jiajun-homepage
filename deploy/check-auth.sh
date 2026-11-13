#!/usr/bin/env bash
set -euo pipefail

file=${1:?usage: check-auth.sh <credentials-file>}
[[ -s "$file" ]]

awk -F, '
  NF == 0 { next }
  NF != 2 || $1 ~ /^[[:space:]]*$/ || $2 ~ /^[[:space:]]*$/ { exit 1 }
  { found = 1 }
  END { if (!found) exit 1 }
' "$file"
