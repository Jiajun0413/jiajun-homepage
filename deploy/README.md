# Deployment

Cloudflare → Caddy on Aliyun → SSH Unix sockets → local apps.

| Route | Remote socket | Local app |
| --- | --- | --- |
| `/`, `/blog`, `/feed.xml` | `site.sock` | `127.0.0.1:8082` |
| `/translate/` | `translate.sock` | `127.0.0.1:7860` |

## Install

Requires `ssh aliyun`, remote `sudo -n`, and a configured `pdf2zh-next` Pixi project beside this repository.

```sh
bash deploy/install.sh
loginctl enable-linger "$USER"
```

The installer derives connection details from `ssh -G aliyun`, creates private `deploy/ssh.conf`, and enables three user units. It does not start them or copy private keys.

Install `tmpfiles.d/jiajun-homepage.conf` on Aliyun at `/etc/tmpfiles.d/jiajun-homepage.conf`, then apply that file with `systemd-tmpfiles --create`. The socket directory is `2750 admin:caddy`; the tunnel hooks set only the two app sockets to `0770`. No global sshd changes are needed.

Add `Caddyfile.jiajun` to `/etc/caddy/Caddyfile`. Keep other site blocks intact. Validate as `caddy`, then reload. Access logs must be writable by that user.

The current origin certificate is self-signed and Cloudflare uses **Full**. Full (strict) requires a trusted origin certificate.

## Translator login

Create a private credential file before starting pdf2zh. Each line is `username,password`, as required by pdf2zh:

```sh
mkdir -p "$HOME/.config/pdf2zh"
umask 077
read -rsp 'Translator password: ' translator_password
printf '\n'
printf 'ljj,%s\n' "$translator_password" > "$HOME/.config/pdf2zh/access.csv"
unset translator_password
chmod 600 "$HOME/.config/pdf2zh/access.csv"
```

The unit refuses to start without a nonempty credential file. Native Gradio login protects config, upload, queue, and download routes, including direct access to port 7860. Caddy marks translation responses `private, no-store`. The homepage stays public.

When enabling login on a previously public instance, purge its existing Cloudflare cache once. The new response headers do not remove files already cached at the edge.

The service uses the existing locked `gui` task, disables sensitive settings inputs and autosave, and sets `GRADIO_ROOT_PATH=/translate`. Restart pdf2zh after changing credentials. Keep provider keys and private documents out of Git.

## Publish

```sh
systemctl --user start jiajun-homepage-tunnel.service
bash deploy/deploy.sh
systemctl --user start pdf2zh.service
```

`deploy.sh` stops this repository's server, builds once, and starts the new bundle. The homepage is briefly unavailable during the build. It does not commit or push code.

## Check

```sh
systemctl --user status jiajun-homepage.service jiajun-homepage-tunnel.service pdf2zh.service
curl -I https://jiajun0805.cc.cd/
curl -I https://jiajun0805.cc.cd/translate/config
```

Expect `200` for the homepage and `401` for unauthenticated translator config requests. `/translate/` shows the login page. Set provider quotas and clean old uploads as needed.
