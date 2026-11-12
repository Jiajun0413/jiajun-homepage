# 部署

本机运行主页与 pdf2zh，Aliyun 上的 Caddy 接收 Cloudflare 请求。两条 SSH 反向 Unix socket 分别连接本机 `127.0.0.1:8082` 与 `127.0.0.1:7860`，不新增公网 TCP 端口。

## 首次安装

前提：`ssh aliyun` 可连接且远端可 `sudo -n`；`pdf2zh-next` 与本仓库位于同一级目录，已有可用的锁定 Pixi 环境与翻译配置。

```sh
bash deploy/install.sh
loginctl enable-linger "$USER"
```

安装脚本从 `ssh -G aliyun` 获取主机名、用户、端口与第一份私钥路径，生成权限为 0600 的 `deploy/ssh.conf`，再渲染、启用三个用户级 systemd 单元。它不复制私钥，也不启动服务；生成配置已加入 `.gitignore`。

远端创建 socket 目录（此环境 SSH 用户为 `admin`，Caddy 组为 `caddy`）：

```sh
ssh -F deploy/ssh.conf -o ClearAllForwardings=yes -T homepage-tunnel \
  'sudo -n tee /etc/tmpfiles.d/jiajun-homepage.conf >/dev/null' \
  < deploy/tmpfiles.d/jiajun-homepage.conf
ssh -F deploy/ssh.conf -o ClearAllForwardings=yes -T homepage-tunnel \
  'sudo -n systemd-tmpfiles --create /etc/tmpfiles.d/jiajun-homepage.conf'
```

目录为 `/run/jiajun-homepage`，权限 2750，归属 `admin:caddy`。隧道启动后只对 `site.sock` 和 `translate.sock` 设置 0770，Caddy 可连接而其它普通用户不可连接。远端 sshd 决定反向 socket 的初始权限，因此 `tunnel-sockets.sh` 在每次连接前清理这两个旧 socket，在连接后设置组权限；不修改全局 sshd 配置。

`Caddyfile.jiajun` 只包含本站的域名块。将它加入 `/etc/caddy/Caddyfile`，保留其它站点；源站证书路径与该文件一致。访问日志文件需要属于 `caddy:caddy`，请用 Caddy 用户校验，避免 root 创建不可写日志：

```sh
ssh -F deploy/ssh.conf -o ClearAllForwardings=yes -T homepage-tunnel \
  'sudo -n -u caddy /usr/bin/caddy validate --config /etc/caddy/Caddyfile --adapter caddyfile && sudo -n systemctl reload caddy'
```

Cloudflare 当前使用 **Full**，源站为自签证书。切换 **Full (strict)** 前需改用可信的源站证书。

## 发布与启动

```sh
systemctl --user start jiajun-homepage-tunnel.service
bash deploy/deploy.sh
systemctl --user start pdf2zh.service
```

`deploy.sh` 停止本站 SSR 程序，执行一次 `pixi run web-build`，成功后由 systemd 启动新产物。构建失败会停止发布，不会忽略错误；SSH 隧道保持运行。构建期间主页会短暂不可用。

pdf2zh 单元复用独立项目的 `pixi run --locked gui` 任务（关闭敏感配置输入和自动保存），并设置 `GRADIO_ROOT_PATH=/translate`、关闭 Gradio 分析与浏览器自动打开。翻译 API 配置保留在 pdf2zh 的配置目录，不能进入本站仓库。匿名开放会消耗服务资源，请自行设置 API 额度、队列并发和文件保留周期。

## 查看状态

```sh
systemctl --user status jiajun-homepage.service jiajun-homepage-tunnel.service pdf2zh.service
journalctl --user -u pdf2zh.service -n 50 --no-pager
journalctl --user -u jiajun-homepage-tunnel.service -n 30 --no-pager
curl -I https://jiajun0805.cc.cd/
curl -I https://jiajun0805.cc.cd/translate/
```

`/translate` 应以 308 跳转到 `/translate/`；翻译程序未运行时该路径为 502。主页、隧道、翻译三个单元分别重启，互不捆绑。更换机器路径后重新运行安装脚本。

检查模板与 socket：

```sh
bash -n deploy/install.sh deploy/deploy.sh deploy/tunnel-sockets.sh
systemd-analyze --user verify ~/.config/systemd/user/jiajun-homepage.service \
  ~/.config/systemd/user/jiajun-homepage-tunnel.service ~/.config/systemd/user/pdf2zh.service
ssh -F deploy/ssh.conf -o ClearAllForwardings=yes -T homepage-tunnel \
  'stat -c "%a %U %G %n" /run/jiajun-homepage /run/jiajun-homepage/site.sock /run/jiajun-homepage/translate.sock'
```
