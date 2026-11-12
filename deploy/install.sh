#!/usr/bin/env bash
# Install local systemd --user units + private SSH config for jiajun-homepage.
# Run as the owning login user (no sudo). Does NOT start anything.
set -euo pipefail
[[ $# == 0 ]] || { echo "usage: $0" >&2; exit 2; }

root=$(cd "$(dirname "$0")/.." && pwd)
pdf2zh=$(cd "$root/.." && pwd)/pdf2zh-next
units="$HOME/.config/systemd/user"

[[ -x "$root/web/dist/server" ]] || echo "warning: $root/web/dist/server not built yet" >&2
[[ -d "$pdf2zh" ]] || { echo "error: pdf2zh project not found: $pdf2zh" >&2; exit 1; }

# --- private ssh config derived from the user's existing `aliyun` host ---
sshg=$(/usr/bin/ssh -G -T aliyun)
get() { awk -v k="$1" '$1==k {print $2; exit}' <<<"$sshg"; }
host=$(get hostname); port=$(get port); user=$(get user); ident=$(get identityfile)
ident=${ident/#\~/$HOME}
[[ -n "$host" && -n "$user" && -n "$ident" ]] || { echo "error: cannot derive aliyun host/user/identity" >&2; exit 1; }

# The generated SSH config is private from its first write.
umask 077
sed -e "s|@HOSTNAME@|$host|" -e "s|@PORT@|$port|" -e "s|@USER@|$user|" \
    -e "s|@IDENTITY@|$ident|" "$root/deploy/ssh.conf.example" > "$root/deploy/ssh.conf"
chmod 600 "$root/deploy/ssh.conf"

# --- units (templates -> installed, machine paths substituted) ---
install -d "$units"
sed -e "s|@ROOT@|$root|g"   "$root/deploy/systemd/jiajun-homepage.service"        > "$units/jiajun-homepage.service"
sed -e "s|@ROOT@|$root|g"   "$root/deploy/systemd/jiajun-homepage-tunnel.service" > "$units/jiajun-homepage-tunnel.service"
sed -e "s|@PDF2ZH@|$pdf2zh|g" "$root/deploy/systemd/pdf2zh.service"               > "$units/pdf2zh.service"

systemctl --user daemon-reload
systemctl --user enable jiajun-homepage.service jiajun-homepage-tunnel.service pdf2zh.service

echo "installed: $root/deploy/ssh.conf"
echo "installed units in $units"
