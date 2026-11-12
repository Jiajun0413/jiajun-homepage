# Jiajun homepage

Minimal, blog-first personal site. Rust 2024 + Dioxus 0.7 fullstack + Axum + Tailwind 4; Pixi manages the build environment. Keep the Web engine/toolchain aligned with MoM-ISEP's `beautification/deep-refinement` branch; do not bring its inference stack here.

## Ownership

- `content/posts/*.md`: blog content, compiled into the binary at build time.
- `web/build.rs`: content discovery, metadata validation and Markdown generation.
- `web/src/model.rs`, `web/src/site.rs`: small content Interface and verified profile/project facts.
- `web/src/app.rs`, `web/tailwind.css`: presentation and routes; generated `web/assets/tailwind.css` is not a source file.
- `web/src/main.rs`: Axum/Dioxus process entry and feeds.
- `deploy/`: Caddy Adapter, SSH tunnel and systemd user units. pdf2zh lives in a sibling repository.

## Design rules

Use `codebase-design`: concentrate parsing and validation in one deep content Module, keep its Interface small, and let pages consume the same article data. Introduce a new Seam only when a real second Adapter exists. No generic CMS, database, DI container or speculative plugins.

The page is a restrained reading surface: fine rules, whitespace, system/local fonts, no cards/gradients/hero slogans/animation. UI changes belong to a designer specialist. Review its copy independently; do not invent biography, Chinese spelling of names, publications, star counts or project ownership. oh-my-opencode-slim is upstream; Jiajun0413 is a major contributor.

## Build and verification

Use Pixi tasks; keep `pixi.lock`, `web/Cargo.lock`, `web/package-lock.json` committed. Do not install Python/Conda packages with pip/conda outside Pixi.

```sh
pixi install --locked
pixi run install-web-tools
pixi run web-check
pixi run test
pixi run web-build
```

Server checks alone do not compile browser-specific branches: require the wasm32 target check. Build Tailwind before `dx bundle` so a clean build ships current CSS. Do not strip wasm-bindgen custom sections before wasm-bindgen runs. Confirm changed content exists in rebuilt `web/dist/server` or `.wasm`, and that the generated HTML references the current CSS/JS/WASM.

After writers reconcile, run the final build once, then check desktop/mobile overflow, hydration navigation, article direct URLs, RSS, and the `/translate/` upload/queue/download workflow. Reuse earlier passing evidence unless changed files make it stale.

## Deployment

`bash deploy/deploy.sh` stops the exact homepage executable before replacing it; building over a running `web/dist/server` fails with `Text file busy`. Never use broad `pkill -f` patterns. Existing 8081/9000 forwards and other sites belong to the user and must remain untouched.

Use dedicated SSH config and remote Unix sockets; existing Aliyun `GatewayPorts yes` can override requested TCP loopback binds. Validate Caddy as user `caddy` before reload so logs remain writable. Generated `deploy/ssh.conf`, private keys/certificates, API secrets, uploaded PDFs, logs, `.pixi`, `dist` and `target` never belong in commits. Local services run under `systemctl --user`; enable linger for persistence.

Code license is MIT. Keep Noto font OFL attribution and upstream pdf2zh AGPL source access intact.
