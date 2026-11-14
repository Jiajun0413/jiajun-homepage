# Production startup

The apps run on the local machine. Aliyun runs only Caddy and the SSH forwarding endpoints.

| Public route | Aliyun endpoint | Local process |
| --- | --- | --- |
| `https://jiajun0805.cc.cd/` | `site.sock` | Homepage, port 8082 |
| `https://jiajun0805.cc.cd/translate/` | `translate.sock` | pdf2zh, port 7860 |

Caddy listens on HTTPS port 443. The tunnel forwards Unix sockets, not public TCP ports 8082/7860. Requests still execute on the local machine. The separate machine's 8081/9000 forwards are unchanged.

## 1. Start Aliyun

The server is already configured. From your terminal:

```sh
ssh -o ClearAllForwardings=yes aliyun
sudo -i
systemd-tmpfiles --create /etc/tmpfiles.d/jiajun-homepage.conf
runuser -u caddy -- caddy validate --config /etc/caddy/Caddyfile --adapter caddyfile
systemctl enable --now caddy
systemctl status caddy --no-pager
exit
exit
```

After editing the Caddyfile, run `systemctl reload caddy` as root. Do not restart Caddy for config changes.

## 2. Start the local apps and tunnel

On the machine holding this repository and `pdf2zh-next`:

```sh
cd /mnt/netshare2/liujiajun/jiajun_homepage
systemctl --user start jiajun-homepage.service pdf2zh.service
systemctl --user start jiajun-homepage-tunnel.service
systemctl --user status jiajun-homepage.service pdf2zh.service jiajun-homepage-tunnel.service --no-pager
```

These are production services, not `dx serve`. They stay running after you close the terminal. Linger and the enabled user units start them after reboot.

The tunnel uses its own SSH config. Do not open another tunnel to the same socket paths from another machine.

## One-time local setup

Requires Pixi, Rust, `ssh aliyun`, remote `sudo -n`, and a configured sibling `pdf2zh-next` project:

```sh
pixi install --locked
pixi run install-web-tools
pixi run web-build
bash deploy/install.sh
loginctl enable-linger "$USER"
```

The installer creates private `deploy/ssh.conf` and installs the user units. It does not start them or copy keys. On a new Aliyun server, install `deploy/Caddyfile`, the existing certificates, and `deploy/tmpfiles.d/jiajun-homepage.conf` before starting the tunnel. The current origin certificate is self-signed; Cloudflare must use **Full**, not Full (strict).

The socket directory is `2750 admin:caddy`; the tunnel hooks set socket permissions to `0770`. After rebuilding the sockets, the hook gracefully reloads Caddy to replace its old upstream connections.

## Translator login

The existing credentials are in `~/.config/pdf2zh/access.csv`, outside Git. To create or change them:

```sh
mkdir -p "$HOME/.config/pdf2zh"
umask 077
read -rsp 'Translator password: ' translator_password
printf '\n'
printf 'ljj,%s\n' "$translator_password" > "$HOME/.config/pdf2zh/access.csv"
unset translator_password
chmod 600 "$HOME/.config/pdf2zh/access.csv"
systemctl --user restart pdf2zh.service
```

The unit rejects missing or malformed credentials. Restarting pdf2zh invalidates existing logins; sign in again. Native login protects config, upload, queue, and download routes. Caddy sends `private, no-store` for translation responses. If enabling login on a previously public instance, purge its old Cloudflare cache once.

## Publish a code or post update

```sh
bash deploy/deploy.sh
```

This stops the homepage, builds the production bundle, and starts it again. The translator and tunnel keep running. It does not commit or push Git changes.

## Verify

```sh
curl -I http://127.0.0.1:8082/
curl -I https://jiajun0805.cc.cd/
curl -i https://jiajun0805.cc.cd/translate/config
```

Expect `200` for the homepage and `401` for unauthenticated translator config. `/translate/` shows the login form.

Local logs: `journalctl --user -u <unit>`. Aliyun access logs: `journalctl -u caddy` as root. The three-site Caddyfile keeps proxy defaults, upstream gzip for `pisd.cc.cd`, default zstd/gzip for the other sites, and the Cloudflare client-IP trust list.
