---
title: 外观
description: Ghostty、tmux 和 Neovim 共用的浅色与深色主题，以及如何修改它。
sidebar:
  order: 5
---

内置配置共用一套调色板 [Catppuccin](https://catppuccin.com)：浅色模式用 Latte，深色模式用 Mocha。Ghostty、tmux 和 Neovim 都跟随 macOS 的外观，并在会话打开时随之切换。

面板之间靠背景深浅区分，而不是靠线条。代码和终端位于基础色上，侧边栏、状态栏、窗格标题和浮动窗口则位于稍深一点的颜色上。次要文字颜色较淡，一种强调色（peach）用于标出会话名、当前窗格标题和 Neovim 模式。

## 每一层如何跟随系统

| 层 | 方式 |
|---|---|
| Ghostty | `config/ghostty/config` 中的 `theme = light:Catppuccin Latte,dark:Catppuccin Mocha` |
| tmux | `config/tmux/appearance.sh` 读取 macOS 外观，设置 tmux 的 `theme`（它会从 `config/tmux/tmux.conf` 中选择 `dark-theme-*` 或 `light-theme-*` 颜色），并用调色板的背景色、文字色和 ANSI 颜色绘制每个窗格。tmux 会在连接时、获得焦点时以及每 5 秒运行它一次；需要 tmux 3.8 或更高版本 |
| Neovim | `config/nvim/lua/f0/appearance.lua` 在获得焦点时以及每 5 秒读取一次 macOS 外观并设置 `background`；`catppuccin` 配色方案据此选择 Latte 或 Mocha |

外观改变时无需重启任何东西：tmux 和 Neovim 会在几秒内切换。

## 其他终端

由于 tmux 自己绘制窗格，所以在任何支持真彩色的终端中（例如 WezTerm、iTerm2 或 kitty），无论终端本身是什么颜色，工作区看起来都一样。仍有两样东西来自终端：tmux 周围的边距和光标颜色。要让它们也一致，就给终端设置同样的主题。以 WezTerm 为例，在 `~/.wezterm.lua` 中：

```lua
local appearance = wezterm.gui and wezterm.gui.get_appearance() or "Dark"
config.color_scheme = appearance:find("Dark") and "Catppuccin Mocha" or "Catppuccin Latte"
```

## 你自己的 Ghostty 配置

Ghostty 会读取多个配置文件，其中任何一个设置的颜色都会覆盖主题。如果边距或光标保持旧颜色，就从你的其他 Ghostty 配置文件（例如 `~/Library/Application Support/com.mitchellh.ghostty/config`）中删除 `background`、`foreground`、`palette` 之类的行。[`f0 doctor`](/zh-cn/reference/cli/#f0-doctor) 会列出这些行及其文件和行号，并报告低于 3.8 的 tmux 版本。

## 更换主题

各层要一起修改，否则它们就不再匹配：

- **Ghostty：** 在 `config/ghostty/config` 中换一个 `theme`。`ghostty +list-themes` 会列出内置主题。
- **tmux：** `config/tmux/tmux.conf` 中用于状态栏和窗格标题的 `dark-theme-*` 和 `light-theme-*` 颜色，以及 `config/tmux/appearance.sh` 中的窗格颜色。`tmux.conf` 中的每种颜色都有一个角色，写在它们旁边：面板深浅、选区、淡色文字和强调色。
- **Neovim：** `config/nvim/lua/plugins/colorscheme.lua`，其中也包含让面板呈现深浅的覆盖设置。

使用 `install.sh --cli-only`，或者 `--no-nvim`、`--no-tmux-config` 和 `--no-ghostty` 时，你自己的配置和颜色保持不变。
