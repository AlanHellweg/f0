---
title: 服务器
description: 在终端行中运行、重启和组织开发服务器。
sidebar:
  order: 3
---

项目文件中的每个 `[[servers]]` 条目都会按顺序在底部一行获得自己的窗格。直到 `layout.terminals` 为止的其余窗格是普通 shell。

```toml
[[servers]]
name = "api"
cmd = "pnpm nx serve @org/api"

[[servers]]
name = "mobile"
cwd = "apps/mobile"
cmd = "npx expo start -c"
```

- `name` 显示在窗格边框上。
- `cwd` 相对于 slot。
- 命令是输入到 shell 中的，而不是作为窗格的进程运行。`Ctrl-C` 会停止服务器并保留 shell，所以按 `Up` 再按 `Enter` 就能重启它。

## 选择要启动的服务器

这个列表是任务*可以*运行的内容，而不是它实际运行的内容。默认情况下不会启动任何服务器：每个窗格打开时命令已输入在提示符处，等待 Enter。大多数任务只涉及 monorepo 中的一两个应用，全部启动会浪费内存，还会争抢端口。

| 目的 | 做法 |
|---|---|
| 立即启动某个服务器 | 在它的窗格中按 Enter |
| 随任务启动某些服务器 | `f0 new APP-123 --start api,web` |
| 每个任务都启动某个服务器 | 在它的 `[[servers]]` 块中设置 `autostart = true` |
| 重建会话时启动服务器 | `f0 open APP-123 --start api` |

```toml
[[servers]]
name = "api"
cmd = "pnpm nx serve @org/api"
autostart = true
```

`--start` 会在租用 slot 之前检查名称，所以拼写错误会尽早失败，并列出项目的服务器。

## 注意事项

- 服务器运行的是它启动时所在目录的代码。其他任务的服务器继续提供那个任务的代码。
- 两个任务运行同一个服务器时会争抢同一个端口。停掉其中一个，或者为每个任务配置不同的端口。
- 用 `Ctrl-a z` 放大窗格来阅读长日志，用 `Ctrl-a [` 向上滚动。
- `layout.terminals` 必须至少等于服务器的数量（最多 6 个）。
