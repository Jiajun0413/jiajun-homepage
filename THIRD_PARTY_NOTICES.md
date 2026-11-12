# 第三方资源

## Noto Serif SC

`web/public/fonts/noto-serif-sc-{400,600}.woff2` 是 Noto Serif SC 的本地子集：GB2312 汉字子集、ASCII 及常用标点。浏览器只向本站请求字体，不依赖外部 CDN。超出子集的字符使用系统字体回退。

- 原字体：Google / Adobe 的 [Noto CJK](https://github.com/notofonts/noto-cjk)
- 获取来源：[`@fontsource/noto-serif-sc` 5.3.1](https://www.npmjs.com/package/@fontsource/noto-serif-sc)
- 处理方式：使用 fontTools `subset`，保留 400 / 600 两档字重与 OpenType 排版功能
- 许可证：SIL Open Font License 1.1，完整文本见 [`web/public/fonts/OFL.txt`](web/public/fonts/OFL.txt)

字体不适用本仓库代码的 MIT 许可证。

## PDFMathTranslate-next

翻译入口连接独立部署的 PDFMathTranslate-next（pdf2zh）服务，并未将它的程序代码或配置打包进本仓库。

- 实际运行版本：PyPI `pdf2zh-next==2.10.2`，由独立项目的 `pixi.lock` 管理；加载的是该环境中的安装包，而不是本地 checkout 的 Python 源码。
- 对应项目源码：[PDFMathTranslate-next/PDFMathTranslate-next](https://github.com/PDFMathTranslate-next/PDFMathTranslate-next)；安装包的源码发行包可从 [PyPI 2.10.2](https://pypi.org/project/pdf2zh-next/2.10.2/#files) 获取。
- 许可证：AGPL-3.0。网络使用者应能获取实际运行版本的对应源码；不要把密钥、用户上传文件或私有配置提交到源码仓库。

## 项目链接与署名

主页上的 oh-my-opencode-slim 是上游项目，展示身份为主要贡献者，不声称拥有或独立创作上游。10k+ stars 是截至 2026-11-12 核实的上游数量级，不是个人 fork 的星数。
