# Jiajun's homepage

A small blog with project links and a password-protected paper translator.

[Website](https://jiajun0805.cc.cd/) · [Blog](https://jiajun0805.cc.cd/blog) · [RSS](https://jiajun0805.cc.cd/feed.xml) · [Translator](https://jiajun0805.cc.cd/translate/) · [ORCID](https://orcid.org/0009-0004-4258-2237)

## Stack

Rust 2024, Dioxus 0.7 fullstack, Axum, plain CSS, and Pixi. No CSS framework or npm build step.

Markdown becomes HTML at build time. Pages and RSS share one article list. No database, CMS, analytics, or external font CDN.

## Development

Install [Pixi](https://pixi.sh/) and [Rust](https://rustup.rs/), then:

```sh
pixi install --locked
pixi run install-web-tools
pixi run web-dev
```

Check and build:

```sh
pixi run web-check
pixi run test
pixi run web-build
IP=127.0.0.1 PORT=8082 pixi run web-serve
```

Pixi and Cargo lockfiles are committed. Dioxus bundles the stylesheet directly; stale hashed assets are removed afterward.

## Write

Add `content/posts/a-note.md`:

```markdown
---
title: A short note
date: 2026-11-12
summary: One sentence about the post.
tags: [Engineering]
---

Post content.
```

The filename becomes `/blog/a-note`. Use lowercase letters, digits, and hyphens. `title` and `date` are required. Front matter supports simple `key: value` fields and comma-separated tags, not full YAML.

Profile and projects: `web/src/site.rs`. Layout and styles: `web/src/app.rs` and `web/styles.css`.

## Production

See [production startup](deploy/README.md) for exact Aliyun and local commands.

- Public traffic enters Aliyun Caddy on HTTPS port 443.
- A dedicated SSH service forwards the Aliyun Unix sockets to local `127.0.0.1:8082` (homepage) and `127.0.0.1:7860` (translator).
- The local machine runs the Dioxus SSR bundle and pdf2zh. Aliyun only terminates TLS and proxies requests; computation and file handling remain local.
- The 8081/9000 services and their tunnels run on another machine and are managed separately.
- User-level systemd units keep the homepage, translator, and forwarding tunnel running. `loginctl enable-linger "$USER"` keeps them active after logout and across reboots.

The translator requires native Gradio login, even on direct port 7860. Credentials and provider keys stay outside Git. Runtime: locked PyPI package `pdf2zh-next==2.10.2` from the sibling Pixi environment.

## License

Code: [MIT](LICENSE). Fonts: SIL OFL 1.1. pdf2zh is a separate AGPL-3.0 application. See [third-party notices](THIRD_PARTY_NOTICES.md).
