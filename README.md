# Jiajun 的主页

极简、以文章为主的个人主页。

- 主页：[jiajun0805.cc.cd](https://jiajun0805.cc.cd/)
- 文章：[归档](https://jiajun0805.cc.cd/blog) · [RSS](https://jiajun0805.cc.cd/feed.xml)
- 论文翻译：[pdf2zh](https://jiajun0805.cc.cd/translate/)
- GitHub：[@Jiajun0413](https://github.com/Jiajun0413)

文章在前，项目在后。保留明暗主题、键盘焦点、移动端和打印排版；没有广告、访问统计、评论系统或外部字体 CDN。

## 工具链

沿用 `MoM-ISEP` 的 `beautification/deep-refinement` 分支的 Web 工具链：

| 用途 | 工具 |
| --- | --- |
| 环境与任务 | Pixi |
| 程序 | Rust stable，edition 2024 |
| 页面与路由 | Dioxus 0.7 fullstack / WebAssembly |
| 服务端渲染 | Axum + Dioxus Server |
| 样式构建 | Tailwind CSS 4 |
| 发布产物 | `dx bundle --release` → `web/dist/{server,public/}` |
| HTTPS 与反代 | Caddy + Cloudflare |

只复用 Web 工具链，不引入原项目的 PyTorch、模型权重、数据库或 PyO3。

## 本地运行

需要安装 [Pixi](https://pixi.sh/) 与 [Rust / rustup](https://rustup.rs/)。仓库的 `rust-toolchain.toml` 声明 stable 与 `wasm32-unknown-unknown` 目标。

```sh
git clone https://github.com/Jiajun0413/jiajun-homepage.git
cd jiajun-homepage
pixi install --locked
pixi run install-web-tools
pixi run web-dev
```

正式构建与运行：

```sh
pixi run web-check
pixi run test
pixi run web-build
IP=127.0.0.1 PORT=8082 pixi run web-serve
```

`pixi.lock`、`web/Cargo.lock`、`web/package-lock.json` 都提交到仓库。CSS 在打包前先生成，构建结束后清理不再被引用的旧哈希资源。`NO_DOWNLOADS=1` 让 Dioxus CLI 使用 PATH 中的构建工具，不在打包期间另行下载它们。

## 写文章

在 `content/posts/` 增加一个 Markdown 文件，文件名只用小写字母、数字和短横线。例如 `2026-11-12-a-note.md`：

```markdown
---
title: 一篇新笔记
date: 2026-11-12
summary: 用一句话说明这篇文章。
tags: [笔记, 工程]
---

## 一个小节

正文从这里开始。
```

- 文件名决定 `/blog/2026-11-12-a-note` 这个地址。
- `title`、`date` 必填，日期为 `YYYY-MM-DD`；摘要和标签可省略。
- 元信息只支持这里展示的简单 `key: value` 和逗号分隔标签，不是完整 YAML。
- 构建时转换 Markdown、排序并编译到程序；首页、归档和 RSS 共用同一份文章列表。
- 支持表格、引用、代码块与脚注。仓库作者可以编辑文章，站点没有接受不可信用户 HTML 的入口。

修改姓名、GitHub、邮箱或项目条目：编辑 `web/src/site.rs`。修改视觉：编辑 `web/tailwind.css` 与 `web/src/app.rs`。

## 部署

当前部署路径是 Cloudflare → Aliyun Caddy → SSH Unix socket → 本机程序。主页与翻译各有独立 socket，不在 Aliyun 上额外开放 8082 / 7860 公网端口。

```text
jiajun0805.cc.cd
  ├─ /、/blog、/feed.xml → site.sock → 127.0.0.1:8082
  └─ /translate/        → translate.sock → 127.0.0.1:7860
```

- `deploy/` 提供用户级 systemd 单元、安装与发布脚本及 Caddy 片段。
- 翻译程序是独立的 `pdf2zh-next` Pixi 项目；当前使用锁定的 PyPI `pdf2zh-next==2.10.2` 安装包，不直接加载该目录的 checkout 源码。本站不复制它的密钥、上传文件或配置。
- Gradio 使用 `GRADIO_ROOT_PATH=/translate`。Caddy 去除这个路径前缀后回源，支持上传、队列进度和下载。
- 现有 Cloudflare 使用 **Full** 模式与自签源站证书；这不是 Full (strict)。不要把 Full (strict) 配合自签证书使用。更换 TLS 策略前应替换为可信源站证书。
- 原有 `pisd.cc.cd` 与 `sbdssp.cc.cd` 的 Caddy 路由不由本站接管。

部署脚本只发布已审核的本地代码，不自动提交或推送 GitHub；更新文章后需要自行提交。

## 公开翻译的注意事项

该页面是公开翻译服务，不是保密文档处理平台。不要上传敏感论文；上游翻译提供商可能接收文档文本。部署者承担算力与翻译 API 用量，请按资源情况设置提供商额度并定期清理上传及输出文件。翻译结果需对照原文校核。

服务启动时关闭敏感配置输入与自动保存；密钥只留在独立的 pdf2zh 配置中，不进入页面源码或本仓库。部署前后应检查实际运行版本与文件访问权限。

## 模块设计

遵循 `codebase-design` 的深模块原则：

1. **内容 Module**：Markdown 的读取、校验、转换与排序集中在构建期；运行时只消费 `Post` 列表。其 Interface 是文章数据，不是解析器细节。
2. **页面 Module**：只负责将文章与项目事实渲染成页面。内容数据与呈现之间的 Seam 使改文字不需要改布局。
3. **部署 Adapter**：Caddy、SSH、systemd 承担运行环境，不进入页面逻辑；主页不重新实现翻译引擎。

没有通用 CMS、插件注册系统、数据库层或假想的可替换适配器。

## 许可证

代码使用 [MIT](LICENSE)。字体使用 SIL OFL 1.1，pdf2zh 保持其独立 AGPL-3.0 许可；详见 [第三方说明](THIRD_PARTY_NOTICES.md)。
