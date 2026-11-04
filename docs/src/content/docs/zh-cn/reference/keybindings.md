---
title: 快捷键
description: 内置配置中的 tmux 和 Neovim 快捷键。
sidebar:
  order: 3
---

这些快捷键来自 `config/tmux` 和 `config/nvim` 中的内置配置。`Space` 是 Neovim 的 leader 键。

## tmux

前缀键是 `Ctrl-a`。

| 按键 | 操作 |
|---|---|
| `Ctrl-h/j/k/l` | 在窗格和 Neovim 分屏之间移动 |
| `Ctrl-a z` | 放大当前窗格（切换） |
| `Ctrl-a s` | 选择会话 |
| `Ctrl-a d` | 分离会话；一切继续运行 |
| `Ctrl-a X` | 关闭此会话的 f0 任务：一个菜单，可以保留 slot 以后再用（`f0 stop`），或在询问后归还它（`f0 done`）。由 f0 自己绑定，参见[按键](/zh-cn/reference/configuration/#按键) |
| `Ctrl-a \|` / `Ctrl-a -` | 向右 / 向下分割 |
| `Ctrl-a c` | 新建窗口 |
| `Ctrl-a [` | 向上滚动（`v` 选择，`y` 复制到剪贴板） |
| `Ctrl-a Ctrl-l` | 向窗格发送 `Ctrl-l`（清屏） |
| `Ctrl-a Ctrl-k` | 向窗格发送 `Ctrl-k` |
| `Ctrl-a r` | 重新加载 tmux 配置 |

鼠标已启用：点击获得焦点，拖动边框调整大小，滚动查看历史。

## Neovim

### 文件与搜索

| 按键 | 操作 |
|---|---|
| `Space ff` | 查找文件 |
| `Space fg` | 在项目中搜索文本 |
| `Space fb` | 已打开的缓冲区 |
| `Space fr` | 最近的文件 |
| `Space e` | 显示或隐藏文件浏览器 |
| `Space gm` | 显示或隐藏已修改文件面板 |
| `Space ui` | 恢复三列布局 |
| `Shift-l` / `Shift-h` | 下一个 / 上一个缓冲区 |
| `Space bd` | 关闭缓冲区；如果有未保存的改动，会询问保存、丢弃或取消 |

### Git

| 按键 | 操作 |
|---|---|
| `]c` / `[c` | 下一处 / 上一处改动 |
| `Space gp` | 预览改动 |
| `Space gr` | 撤销改动 |
| `Space gb` | 查看该行的 blame |
| `Space gs` | 已修改文件（选择器） |
| `Space gc` | 提交记录 |
| `Space gd` | 未提交改动的 diff |
| `Space gD` | 分支与其基础分支的 diff |
| `Space gh` | 文件历史 |
| `Space gq` | 关闭 diff |

### 代码

| 按键 | 操作 |
|---|---|
| `gd` / `gD` | 定义 / 声明 |
| `gi` / `gr` | 实现 / 引用 |
| `K` | 悬停文档 |
| `Space rn` | 重命名 |
| `Space ca` | 代码操作 |
| `[d` / `]d` | 上一个 / 下一个诊断 |
| `Space ld` | 显示诊断 |
| `Space cf` | 格式化 |

## 使用你自己的配置

跨窗格使用 `Ctrl-h/j/k/l` 需要 [vim-tmux-navigator](https://github.com/christoomey/vim-tmux-navigator) 的两部分：tmux 配置中的代码片段，以及 Neovim 中的插件。没有它们时，用你的 tmux 前缀键加方向键在窗格之间移动。只有你的 tmux 配置设置了 `pane-border-status`，窗格标题才会显示。

每当 f0 创建或打开会话时，都会在你自己的前缀键之后绑定 `X`，所以它适用于任何 tmux 配置，而且无需重新加载。每个 f0 会话的状态栏都会显示它，比如 `C-b X close task`。可以用 [`[keys] done`](/zh-cn/reference/configuration/#按键) 换一个按键或关闭它。
