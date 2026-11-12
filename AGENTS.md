# Jiajun homepage

Rust 2024 + Dioxus 0.7 fullstack + Axum + Tailwind 4. Pixi manages build dependencies. Keep the Web toolchain aligned with MoM-ISEP's `beautification/deep-refinement` branch.

## Rules

- Write concise English throughout the repository. Remove filler and duplicate explanations.
- Use `codebase-design`: content parsing and validation stay in one Module; pages consume its small `Post` Interface. Add a Seam only for a real variation.
- Keep the blog layout restrained. UI design belongs to the designer; mechanical copy edits need no redesign.
- Use verified names, links, and attribution. Jiajun0413 is an upstream contributor to oh-my-opencode-slim.
- Do not add a CMS, database, or speculative abstractions.

## Files

- `content/posts/`: Markdown articles.
- `web/build.rs`, `web/src/content.rs`: content compilation and validation.
- `web/src/site.rs`: profile and project facts.
- `web/src/app.rs`, `web/tailwind.css`: pages and styles.
- `web/src/main.rs`: SSR process and RSS.
- `deploy/`: Caddy, SSH, and systemd configuration.

## Checks and deployment

Run `pixi run web-check`, `pixi run test`, then `pixi run web-build`. Check both server and wasm targets. Compile CSS before bundling; verify shipped assets and changed content. Reconcile writers before final checks and reuse valid evidence.

Use `bash deploy/deploy.sh` to stop the exact server before replacing its executable. Never use broad `pkill` patterns or change the user's 8081/9000 routes. Validate Caddy as `caddy` before reload.

The translator uses native Gradio login. Credentials live in `~/.config/pdf2zh/access.csv`, outside Git. Missing credentials must prevent startup. Keep config, uploads, keys, session files, logs, generated bundles, and environments out of commits.

Code is MIT. Preserve font OFL notices and pdf2zh's separate AGPL source access.
