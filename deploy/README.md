# Production

Aliyun runs Caddy. The local machine runs the homepage and translator.

```text
Internet → Cloudflare → Aliyun :443
                           ├─ 127.0.0.1:8082 → SSH → local :8082
                           └─ 127.0.0.1:7860 → SSH → local :7860
```

The separate machine's 8081/9000 services are unchanged.

## Daily use

On Aliyun, Caddy is enabled at boot. To start it manually:

```sh
ssh -o ClearAllForwardings=yes aliyun
sudo systemctl start caddy
```

On the local machine:

```sh
cd /mnt/netshare2/liujiajun/jiajun_homepage
pixi run start
pixi run status
```

Publish code or post changes with `pixi run deploy`. Stop the local apps and tunnel with `pixi run stop`.

## First setup

Install Pixi, Rust, and the sibling `pdf2zh-next` environment, then run locally:

```sh
pixi install --locked
pixi run install-web-tools
pixi run web-build
bash deploy/setup-tunnel.sh
pixi run install-services
pixi run start
```

The tunnel setup creates a restricted SSH account on Aliyun using this machine's public key. OpenSSH binds only Aliyun's loopback ports 8082 and 7860. No separate SSH config, remote commands, socket directories, or reload hooks are used during operation.

The three unit files contain this installation's paths. If moving the checkout, edit those paths once before `install-services`.

On a new Aliyun server, install Caddy, `deploy/Caddyfile`, and the origin certificates. Validate as `caddy`, then enable the service. The current self-signed origin certificates require Cloudflare **Full** mode.

## Translator configuration

The service reads `~/.config/pdf2zh/config.v3.toml` and uses only the `OpenAICompatible` engine for translation and term extraction. The Fengwind endpoint and credentials stay in that file.

Login credentials stay in `~/.config/pdf2zh/access.csv`, mode `0600`, as one `username,password` line. Restart `pdf2zh.service` after editing either file. Users must log in again.

Translation responses are private and uncached. Never commit credentials, provider keys, or uploaded files.

## Checks

```sh
curl -I http://127.0.0.1:8082/
curl -I https://jiajun0805.cc.cd/
curl -i https://jiajun0805.cc.cd/translate/config
```

Expect homepage `200` and anonymous translator config `401`.

Local logs: `journalctl --user -u <unit>`. Aliyun logs: `sudo journalctl -u caddy`.
