#!/usr/bin/env bash
# Rebuild the SSR bundle and hand 8082 to the systemd --user unit.
# Stops only jiajun-homepage.service and a manually started server whose
# executable is this repo's web/dist/server. Run as the owning login user.
set -euo pipefail
[[ $# == 0 ]] || { echo "usage: $0" >&2; exit 2; }

root=$(cd "$(dirname "$0")/.." && pwd)

systemctl --user stop jiajun-homepage.service

# Match the exact executable and UID; never stop unrelated server processes.
manual_pids=()
for process in /proc/[0-9]*; do
    [[ -O "$process" ]] || continue
    if [[ -e "$process/exe" && "$process/exe" -ef "$root/web/dist/server" ]]; then
        pid=${process##*/}
        kill -TERM "$pid"
        manual_pids+=("$pid")
    fi
done
for pid in "${manual_pids[@]}"; do
    for _ in {1..100}; do
        [[ -e "/proc/$pid/exe" && "/proc/$pid/exe" -ef "$root/web/dist/server" ]] || break
        sleep 0.1
    done
    if [[ -e "/proc/$pid/exe" && "/proc/$pid/exe" -ef "$root/web/dist/server" ]]; then
        echo "error: SSR process $pid did not stop; build not started" >&2
        exit 1
    fi
done

cd "$root"
"$HOME/.pixi/bin/pixi" run web-build

systemctl --user restart jiajun-homepage.service
