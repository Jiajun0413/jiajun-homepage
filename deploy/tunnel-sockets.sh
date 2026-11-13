#!/usr/bin/env bash
# Remote sshd controls permissions/unlinking for -R Unix sockets. Manage only
# this deployment's two sockets without changing the server's sshd settings.
set -euo pipefail

[[ $# == 1 && ( "$1" == prepare || "$1" == ready ) ]] || {
    echo "usage: $0 {prepare|ready}" >&2
    exit 2
}
root=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)

# These control connections run a command; they must not establish forwards.
exec /usr/bin/ssh -F "$root/deploy/ssh.conf" -o ClearAllForwardings=yes \
    -T homepage-tunnel sh -s -- "$1" <<'REMOTE'
set -eu
site=/run/jiajun-homepage/site.sock
translate=/run/jiajun-homepage/translate.sock
case "$1" in
    prepare)
        for socket in "$site" "$translate"; do
            if [ -e "$socket" ] && [ ! -S "$socket" ]; then
                echo "refusing to remove non-socket: $socket" >&2
                exit 1
            fi
        done
        rm -f -- "$site" "$translate"
        ;;
    ready)
        remaining=100
        while [ "$remaining" -gt 0 ]; do
            if [ -S "$site" ] && [ -S "$translate" ]; then
                chmod 0770 "$site" "$translate"
                stat -c '%a %U %G %n' "$site" "$translate"
                # Replacing the socket inode leaves Caddy's pooled connections on the old inode.
                # A graceful reload drops them so Caddy reconnects to the current inode.
                if ! sudo -n systemctl reload caddy; then
                    echo "caddy reload failed" >&2
                    exit 1
                fi
                exit 0
            fi
            remaining=$((remaining - 1))
            sleep 0.1
        done
        echo "timed out waiting for remote Unix sockets" >&2
        exit 1
        ;;
esac
REMOTE
