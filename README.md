# Jiajun's homepage

A small blog with project links and a password-protected paper translator.

[Website](https://jiajun0805.cc.cd/) · [Blog](https://jiajun0805.cc.cd/blog) · [RSS](https://jiajun0805.cc.cd/feed.xml) · [Translator](https://jiajun0805.cc.cd/translate/) · [ORCID](https://orcid.org/0009-0004-4258-2237)

## Stack

Rust 2024, Dioxus 0.7 fullstack, Axum, Tailwind 4, and Pixi. This matches the Web toolchain on MoM-ISEP's `beautification/deep-refinement` branch.

Markdown becomes HTML at build time. Pages and RSS share one article list. No database, CMS, analytics, or external font CDN.

## Run

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

Pixi, Cargo, and npm lockfiles are committed. Tailwind runs before bundling; stale hashed assets are removed afterward.

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

Profile and projects: `web/src/site.rs`. Layout and styles: `web/src/app.rs` and `web/tailwind.css`.

## Deploy

See [deploy/README.md](deploy/README.md).

Cloudflare routes to Caddy on Aliyun. SSH Unix sockets connect Caddy to the homepage on `127.0.0.1:8082` and pdf2zh on port `7860`. Three systemd user units keep the apps and tunnel running. `deploy/Caddyfile` records all three sites with explicit routes and Caddy defaults.

The translator uses pdf2zh's native login, including direct access to port 7860. Credentials stay outside this repository. The current runtime is the locked PyPI package `pdf2zh-next==2.10.2`, not the sibling checkout's Python source. Translation provider credentials and uploaded files also remain outside this repository.

## License

Code: [MIT](LICENSE). Fonts: SIL OFL 1.1. pdf2zh is a separate AGPL-3.0 application. See [third-party notices](THIRD_PARTY_NOTICES.md).
