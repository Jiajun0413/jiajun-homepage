---
title: Running this site
date: 2026-11-12
summary: Follow a request from Cloudflare to the local machine, then learn which files to edit and what to restart.
tags: [Engineering, Operations]
---

Changing a translation model takes a restart of pdf2zh. Editing a post takes a rebuild of the homepage. These are separate operations because two programs run on the local machine, and each reads different files.

## Follow a request

```text
Browser -> Cloudflare -> Caddy on Aliyun (:443)
  /translate/* -> Aliyun :7860 -> SSH -> local pdf2zh :7860
  Other paths  -> Aliyun :8082 -> SSH -> local homepage :8082
```

The local machine opens the SSH connection to Aliyun. In `-R 127.0.0.1:7860:127.0.0.1:7860`, the first address and port are the listener on Aliyun; the second are the destination on the local machine. Aliyun exposes neither forwarded port to the internet. Caddy receives the public HTTPS requests and chooses the upstream.

Caddy strips `/translate` before forwarding a request, so `/translate/config` reaches pdf2zh as `/config`. `GRADIO_ROOT_PATH=/translate` tells Gradio what prefix to put on the upload, queue, and download URLs it sends back. The route has a 50 MB request-body limit and sends `Cache-Control: private, no-store`. The separate 8081 and 9000 forwards serve other sites and are not part of this setup.

## Change the translator

The production service explicitly reads `~/.config/pdf2zh/config.v3.toml` on the local machine. Edit the existing keys in these two tables; the values below are placeholders, not a replacement file.

```toml
[openaicompatible_detail]
openai_compatible_base_url = "<Fengwind API base URL>"
openai_compatible_api_key = "<API key>"
openai_compatible_model = "<model ID>"

[term_openaicompatible_detail]
term_openai_compatible_base_url = "<same Fengwind API base URL>"
term_openai_compatible_api_key = "<API key>"
term_openai_compatible_model = "<term extraction model ID>"
```

`OpenAICompatible` is the engine name. Fengwind is the provider behind the configured URL. Both tables should point to Fengwind, but they can name different models. When term extraction follows the main engine, it uses the translation settings instead of the separate term table.

| Table | Keys | What they control |
| --- | --- | --- |
| `translation` | `lang_in`, `lang_out` | Default input and output languages |
| `translation` | `qps`, `pool_max_workers` | Translation request rate and concurrency |
| `translation` | `term_qps`, `term_pool_max_workers` | Request rate and concurrency for term extraction |
| `translation` | `no_auto_extract_glossary` | Disable automatic term extraction by default |
| `pdf` | `no_mono`, `no_dual` | Disable monolingual or bilingual output |
| `gui_settings` | `server_port` | Local port, currently `7860` |

CLI flags take precedence over environment variables and TOML at startup. For example, the unit passes `--ui-lang en`, so editing `ui_lang` in the TOML does not change the production interface language. Settings exposed in the browser can also change individual jobs. Translation jobs do not auto-save; the explicit **Save Settings** button can still write the TOML. Edit the file directly for controlled changes.

The website login is separate from the API key. Edit `~/.config/pdf2zh/access.csv` to change it. The file has one `username,password` row with no header. Keep both files private and outside Git:

```sh
chmod 600 ~/.config/pdf2zh/config.v3.toml ~/.config/pdf2zh/access.csv
systemctl --user restart pdf2zh.service
```

Finish any running translation before restarting. Wait for the login page to return and sign in again. A provider, model, or password change does not need a homepage build or a Caddy reload.

## Find the startup files

Repository paths below are relative to `/mnt/netshare2/liujiajun/jiajun_homepage` on the local machine. `~` is the home directory of the user running the services, currently `/home/liujiajun`.

| File | Role |
| --- | --- |
| `deploy/systemd/pdf2zh.service` | Source unit: working directory, startup flags, credentials path, and public URL prefix |
| `~/.config/systemd/user/pdf2zh.service` | Installed copy read by the user systemd manager |
| `deploy/pdf2zh_gui.py` | Launches pdf2zh with the term selector limited to `OpenAICompatible` or following the main engine |
| `../pdf2zh-next/pixi.toml` and `pixi.lock` | Define and lock the translator's Python environment |
| `deploy/systemd/jiajun-homepage.service` | Starts `web/dist/server` on local `127.0.0.1:8082` |
| `deploy/systemd/jiajun-homepage-tunnel.service` | Opens the two SSH reverse forwards and reconnects on failure |
| `pixi.toml` | Defines the homepage build, install, start, stop, and status tasks |

The translator unit starts `python -I deploy/pdf2zh_gui.py` with the sibling project's locked Pixi environment. The isolated Python invocation uses the installed `pdf2zh-next` package, currently `2.10.2`, not the sibling checkout's Python sources. Do not edit the installed package or the generated files under `~/.config/pdf2zh/default/`.

The unit selects `OpenAICompatible` for both jobs and limits the main selector with `--enabled-services`. The Python launcher also limits the term selector because this pdf2zh version does not apply that flag to it. Changing a Fengwind model, key, or URL does not require editing the launcher. The selector is not an access control; keep the native Gradio login enabled.

Inspect what systemd actually loaded:

```sh
systemctl --user cat pdf2zh.service
```

Edit the repository unit, not the installed copy. From the homepage checkout, install the updated units and restart only the changed service:

```sh
pixi run install-services
systemctl --user restart pdf2zh.service
```

Use the tunnel unit name instead when its `ExecStart` changes. `install-services` copies and enables the units and reloads the user manager; it does not restart running processes.

## Change the route

`deploy/Caddyfile` is the source copy. `/etc/caddy/Caddyfile` on Aliyun is the file Caddy reads. `deploy/setup-tunnel.sh` provisions the dedicated SSH account and its `/etc/ssh/sshd_config.d/60-homepage-tunnel.conf` policy. That policy permits only the two loopback listeners for the tunnel account. The tunnel unit sets its own SSH arguments; the `aliyun` alias in `~/.ssh/config` is for administration, not the running tunnel. Keep the local SSH key and trusted host-key file on the local machine. The remote account needs only the public key in `authorized_keys`.

- **Local port:** change the app listener and the destination in its SSH `-R` mapping. The Aliyun listener can keep its existing port.
- **Aliyun port:** change the listening side of `-R`, the account's `PermitListen` entry, and Caddy's upstream together. Validate the SSH configuration with `sudo /usr/sbin/sshd -t` before reloading `sshd`. Leave the other account's forwards unchanged.
- **URL prefix:** change the Caddy matcher, redirect, and `handle_path` together with `GRADIO_ROOT_PATH` and the links in `web/src/app.rs` and `web/src/site.rs`.
- **Domain or certificate:** check Cloudflare's proxied DNS record, Caddy's site names and certificate paths under `/etc/caddy/certs/`, and the RSS base URL in `web/src/main.rs`. The current self-signed origin certificate uses Cloudflare **Full** mode. Full (strict) requires a trusted origin certificate.

To apply a Caddy change, update the file on Aliyun, then validate it as the account that runs Caddy before reloading:

```sh
sudo -u caddy caddy validate --config /etc/caddy/Caddyfile --adapter caddyfile \
  && sudo systemctl reload caddy
```

## Run and check

For first-time installation, use the [deployment runbook](https://github.com/Jiajun0413/jiajun-homepage/blob/main/deploy/README.md). For day-to-day use, Caddy should already be enabled on Aliyun. If it is stopped, connect with `ssh -o ClearAllForwardings=yes aliyun` and run `sudo systemctl start caddy` there.

On the local machine:

```sh
cd /mnt/netshare2/liujiajun/jiajun_homepage
pixi run start
pixi run status
```

`start` runs the installed units; it does not build them. Use `pixi run stop` to stop the local apps and tunnel together. To publish a post under `content/posts/` or change the homepage, run `pixi run deploy`. The Rust/Dioxus site compiles Markdown into its bundle. The deploy task rebuilds and starts the homepage without restarting pdf2zh or SSH.

When the public translation page fails, check the hops in order:

1. On the local machine, request `http://127.0.0.1:7860/config`.
2. On Aliyun, request the same URL. Here the port is the SSH listener, not pdf2zh itself.
3. Through Cloudflare, request `https://jiajun0805.cc.cd/translate/config`.

Use `curl -i` with each URL. Without a login, `401` is expected at all three hops. If the local app responds but Aliyun does not, inspect `jiajun-homepage-tunnel.service`. If both respond but the public request fails, check Caddy and the Cloudflare origin settings. A `502` alone does not tell you which hop failed.

Use `journalctl --user -u pdf2zh.service -n 50 --no-pager` for local translation logs and `sudo journalctl -u caddy -n 50 --no-pager` on Aliyun. After a routing change, log in, translate a small non-sensitive PDF, and actually download the result. A working login page does not verify the download path.
