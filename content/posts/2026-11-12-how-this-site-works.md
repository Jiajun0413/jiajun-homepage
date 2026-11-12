---
title: 这页主页的搭建笔记
date: 2026-11-12
summary: 把文章、开源项目和论文翻译放在同一个入口。记录本站的内容结构、工具链与部署方式。
tags: [工程, Rust]
---

这里主要用来放文章和阅读笔记。项目列表留在首页下方，论文翻译则从导航里的「翻译」进入。

## 沿用一套工具链

本站沿用 MoM-ISEP 的 `beautification/deep-refinement` 分支所使用的 Web 工具链：Rust 2024、Dioxus 0.7 fullstack、Axum、Tailwind CSS 4 和 Pixi。

Dioxus 在服务器上渲染首屏 HTML，随后由 WebAssembly 接管站内导航。Tailwind 负责构建样式，具体的版式仍然是手写 CSS。这里没有评论系统、访问统计或外部字体请求。

```sh
pixi install --locked
pixi run install-web-tools
pixi run web-build
IP=127.0.0.1 PORT=8082 pixi run web-serve
```

完整的安装说明和代码在 [GitHub 仓库](https://github.com/Jiajun0413/jiajun-homepage) 中。

## 文章就是 Markdown 文件

每篇文章放在 `content/posts/` 中。文件名决定地址，开头的元信息记录标题、日期、摘要与标签。

```text
---
title: 一篇新笔记
date: 2026-11-12
summary: 用一句话说明这篇文章。
tags: [笔记]
---

正文从这里开始。
```

构建时，Markdown 被转换为 HTML，文章元信息编译进程序。首页、按年归档和 RSS 使用同一份文章列表；发布文章只需要添加文件、构建并重新部署，不需要数据库或后台管理页面。

## 同一个域名，两条路径

- `/` 与 `/blog`：个人主页与文章。
- `/translate/`：独立运行的 pdf2zh 论文翻译页面。

Cloudflare 负责域名代理，Aliyun 上的 Caddy 终止 TLS，再把请求送往对应的程序。翻译页使用 `GRADIO_ROOT_PATH=/translate`，确保上传、任务进度和结果下载的地址都保留这个路径前缀。

[PDFMathTranslate-next](https://github.com/PDFMathTranslate-next/PDFMathTranslate-next) 基于 BabelDOC，能保留论文中的公式与排版。它是独立的上游开源项目；这里只提供部署和使用入口，不把它当作本站自行开发的翻译引擎。不要上传敏感论文，翻译结果也需要对照原文检查。

## 保持简单

先保留文章、项目、翻译入口和 RSS。不加登录、标签筛选或数据库；等内容真的需要时再决定。首页的设计也一样：让文字容易读，让工具容易找到。
