#!/usr/bin/env bash
# One-off remote provisioning for the loopback TCP tunnel. Idempotent.
# Run with admin SSH access: ./deploy/setup-tunnel.sh [host] [admin-user]
set -euo pipefail

host=${1:-47.116.43.212}
admin=${2:-admin}
user=homepage-tunnel
key=$HOME/.ssh/id_rsa

pub=$(ssh-keygen -y -f "$key")
ssh-keygen -F "$host" >/dev/null || { echo "no known_hosts entry for $host" >&2; exit 1; }

ssh -T -o BatchMode=yes -o ConnectTimeout=10 "$admin@$host" \
    "sudo -n bash -s -- $(printf '%q' "$user") $(printf '%q' "$pub") $(printf '%q' "$admin")" <<'REMOTE'
set -euo pipefail
user=$1
pub=$2
admin_user=$3
dropin=/etc/ssh/sshd_config.d/60-homepage-tunnel.conf

home=$(getent passwd "$user" | cut -d: -f6 || true)
if [[ -z $home ]]; then
    sudo -n useradd --system --create-home --shell /usr/sbin/nologin "$user"
    home=$(getent passwd "$user" | cut -d: -f6)
fi
sudo -n install -d -m 0700 -o "$user" -g "$user" "$home/.ssh"
ak=$home/.ssh/authorized_keys
sudo -n touch "$ak"
while IFS= read -r line; do
    [[ -z $line || $line == "$pub" ]] || { echo "unexpected key in $ak" >&2; exit 1; }
done < <(sudo -n cat "$ak")
sudo -n grep -qxF -- "$pub" "$ak" || printf '%s\n' "$pub" | sudo -n tee -a "$ak" >/dev/null
sudo -n chown "$user:$user" "$home/.ssh" "$ak"
sudo -n chmod 0700 "$home/.ssh"
sudo -n chmod 0600 "$ak"

conf="Match User $user
    GatewayPorts no
    AllowTcpForwarding remote
    PermitListen 127.0.0.1:8082 127.0.0.1:7860
    AllowStreamLocalForwarding no
    AuthenticationMethods publickey
    AuthorizedKeysCommand none
    MaxSessions 0
    ClientAliveInterval 30
    ClientAliveCountMax 3"
if [[ -e $dropin ]] && [[ $(sudo -n cat "$dropin") != "$conf" ]]; then
    echo "unexpected existing drop-in: $dropin" >&2
    exit 1
fi
printf '%s\n' "$conf" | sudo -n tee "$dropin" >/dev/null
sudo -n chmod 0644 "$dropin"

if ! sudo -n /usr/sbin/sshd -t; then
    sudo -n rm -f "$dropin"
    echo "sshd -t failed; removed $dropin" >&2
    exit 1
fi
field() { awk -v k="$1" '$1 == k { sub(/^[^ ]+ /, ""); print }'; }
t=$(sudo -n /usr/sbin/sshd -T -C "user=$user,host=localhost,addr=127.0.0.1")
check() { [[ $(field "$1" <<<"$t") == "$2" ]] || { echo "$1: expected '$2', got '$(field "$1" <<<"$t")'" >&2; exit 1; }; }
check gatewayports no
check allowtcpforwarding remote
check 'permitlisten' '127.0.0.1:8082 127.0.0.1:7860'
check allowstreamlocalforwarding no
check authenticationmethods publickey
check authorizedkeyscommand none
check maxsessions 0
check clientaliveinterval 30
check clientalivecountmax 3
a=$(sudo -n /usr/sbin/sshd -T -C "user=$admin_user,host=localhost,addr=127.0.0.1")
[[ $(field gatewayports <<<"$a") == yes ]] || { echo "admin gatewayports changed" >&2; exit 1; }
[[ $(field permitlisten <<<"$a") == any ]] || { echo "admin permitlisten changed" >&2; exit 1; }
sudo -n systemctl reload sshd
echo "provisioned $user: pubkey-only, loopback 8082/7860"
REMOTE
