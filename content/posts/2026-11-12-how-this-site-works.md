---
title: Building this site
date: 2026-11-12
summary: A small blog, a few project links, and a paper translator.
tags: [Engineering, Rust]
---

This site keeps notes, project links, and a paper translator in one place.

## Stack

The Web toolchain matches MoM-ISEP's `beautification/deep-refinement` branch: Rust 2024, Dioxus 0.7 fullstack, Axum, Tailwind 4, and Pixi.

Dioxus renders the first page on the server. WebAssembly handles later navigation. Tailwind builds the stylesheet; the layout is plain CSS.

```sh
pixi run web-check
pixi run test
pixi run web-build
IP=127.0.0.1 PORT=8082 pixi run web-serve
```

## Content

Posts live in `content/posts/`. The filename sets the URL; front matter supplies the title, date, summary, and tags. Markdown is rendered at build time and compiled into the program. The homepage, archive, and RSS use the same list.

There is no database or admin panel. Publishing a post means adding a file and rebuilding.

## Routing

Cloudflare sends requests to Caddy on Aliyun. SSH Unix sockets connect it to the local apps:

- `/` and `/blog`: the homepage and posts.
- `/translate/`: pdf2zh, with its own login.

The translator uses `GRADIO_ROOT_PATH=/translate` so upload, progress, and download URLs keep the prefix. [PDFMathTranslate-next](https://github.com/PDFMathTranslate-next/PDFMathTranslate-next) supplies the translation engine and preserves formulas and page layout.

Source and deployment instructions are on [GitHub](https://github.com/Jiajun0413/jiajun-homepage).
