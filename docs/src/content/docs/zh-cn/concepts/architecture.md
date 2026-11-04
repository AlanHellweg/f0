---
title: 架构
description: f0 CLI、tmux、Neovim 和 treehouse 如何组合在一起。
sidebar:
  order: 1
---

f0 是一个**个人开发环境（Personal Development Environment，PDE）**：一组配置，加上把它们串起来的小型 Rust CLI。CLI 自己不保存任何状态。它读取 treehouse 和 tmux 的状态，并驱动它们。

```
f0 (Rust CLI)
 ├── treehouse  租用、列出和归还 worktree slot
 ├── tmux       每个工作区一个会话，逐个窗格构建
 └── git        找到检出、主仓库和分支
config/
 ├── nvim       编辑器、文件浏览器、diff
 ├── tmux       前缀键、导航、窗格标题
 └── ghostty    字体和键盘
```

## `f0 new` 做了什么

1. 加载 `config.toml` 和 `projects/*.toml`，选出 `repo` 是当前目录主检出的那个项目。
2. 运行 `treehouse get --lease --lease-holder f0:<session> --branch <prefix><ticket>`，配置了的话会加上 `--base` 和 `--include-file`。treehouse 会输出 slot 路径。
3. 把项目转换成一组窗格和分割。这一步是纯函数，并有单元测试。
4. 以你终端的尺寸创建 tmux 会话，逐个窗格分割，为每个窗格命名并输入对应的命令。
5. 把 slot 路径作为 `@f0_path` 选项保存在会话上。`ls`、`open` 和 `done` 通过它找到会话。
6. 连接到会话；如果已经在 tmux 中，则切换客户端。

## 源码结构

| 路径 | 内容 |
|---|---|
| `crates/f0/src/main.rs` | 命令行定义 |
| `crates/f0/src/commands.rs` | 每个命令一个函数 |
| `crates/f0/src/config.rs` | 配置文件、默认值和校验 |
| `crates/f0/src/layout.rs` | 窗口的纯描述 |
| `crates/f0/src/tmux.rs` | 会话的创建和查找 |
| `crates/f0/src/treehouse.rs` | treehouse 调用 |
| `config/` | 内置的 Neovim、tmux 和 Ghostty 配置 |
| `install.sh` | 安装脚本 |
| `docs/` | 本站点 |
