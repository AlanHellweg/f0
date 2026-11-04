---
title: 分层与决策
description: f0 为什么分层，以及背后的取舍。
sidebar:
  order: 2
---

## 分层

| 层 | 需要 | 命令 |
|---|---|---|
| 每个任务一个 worktree | treehouse、项目文件和 `.worktreeinclude` | `f0 new`、`f0 open`、`f0 ls`、`f0 done` |
| 窗口布局 | tmux | 由 `f0 new` 和 `f0 open` 为每个 slot 构建 |
| 编辑器、tmux 和终端配置 | 无，按需安装 | `install.sh` 的选项 |

前两层总是一起出现。配置是可选的：使用你自己的编辑器和 tmux 设置，f0 也会构建相同的布局。

## 决策

**每个任务一条命令，每个任务一个 worktree。** 每个工作区都是一个 treehouse slot。一个任务永远不会与另一个任务共享文件、分支或服务器，智能体可以在后台工作，而不碰你的主检出。

**env 文件是任务的一部分。** 没有 `.env` 文件的 slot 无法运行它的服务器，所以项目必须在 `.worktreeinclude` 中列出它们，即使列表为空。

**复用 treehouse，而不是自己管理 worktree。** treehouse 已经处理了 slot 池、租用、进程清理和 `.worktreeinclude`。在 `git worktree` 之上重新实现这些，只会重复逻辑和 bug。

**tmux 管进程，编辑器管文件。** 服务器和智能体运行在 tmux 窗格中，而不是编辑器的终端里。关闭编辑器或终端应用后它们依然存活，每个任务都可以分离和恢复。

**不保存自己的状态。** f0 每次都读取 `treehouse status --json` 和 tmux 会话，所以不会出现状态不一致。

**用租用，而不是子 shell。** `treehouse get --lease` 是非交互式的，并且会一直保留 slot 直到归还，即使会话已分离。

**命令输入到 shell 中。** 用 `Ctrl-C` 停止服务器后，会留下一个 shell 来重启它。

**服务器等你启动。** 项目列出任务可以运行的内容；你用 Enter、`--start` 或 `autostart` 启动需要的部分。每个任务都启动整个 monorepo 会浪费内存，还会让任务之间争抢端口。

**只有个人配置。** 项目文件位于你的配置目录中，所以共享仓库无需任何改动。

**会话以最终尺寸构建。** 窗格大小是创建会话时窗口的百分比，所以 f0 以你终端的尺寸创建会话，而不是之后再调整大小。

**从不强制归还。** treehouse 在丢弃未提交的工作之前会询问，f0 保留这个提示。
