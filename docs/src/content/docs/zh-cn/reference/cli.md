---
title: CLI
description: f0 的每个命令和参数。
sidebar:
  order: 1
---

```
f0 <command> [options]
```

运行 `f0 <command> --help` 可以在终端中看到相同的信息。

## f0 init

为你所在的仓库创建项目文件，过程中会询问相关信息。

```bash
f0 init [--name <name>] [--template]
```

| 参数 | 含义 |
|---|---|
| `--name` | 项目名称的默认答案。否则使用仓库目录名 |
| `--template` | 不询问，直接写入带有 `<...>` 占位符的文件 |

它会询问项目名称、基础分支（默认是 `origin/HEAD` 指向的分支，在没有远程仓库的 repo 中则是当前分支）、分支前缀、[AI 智能体](/zh-cn/reference/configuration/#ai-智能体)、env 文件和开发服务器。按 Enter 采用方括号中的值。

```
Project name [react-native-bottom-sheet]: bottom-sheet
Base branch [main]:
Branch prefixes, comma separated, the first is the default [feature/, fix/, refactor/, chore/]:
AI agent command (claude, codex, gemini, opencode...) [claude]:
Other files to copy, comma separated (empty for none):
Add a dev server? [y/N]: y
  Name: example
  Command: bun run example start
  Folder, relative to the repo (empty for the root):
  Start it with every task? [y/N]:
Add another server? [y/N]:
```

当仓库没有 `.worktreeinclude` 时，f0 会列出它找到的被忽略的 `.env*` 文件，把你选中的写入配置目录中的 `<name>.worktreeinclude`，并让 `include_file` 指向它。如果仓库有自己的 `.worktreeinclude`，就继续使用它，f0 会跳过这个问题。当服务器多于终端时，文件中也会调高 `layout.terminals`。

在没有终端的环境中（例如脚本里）或使用 `--template` 时，它会写入占位符版本；只要还剩有占位符，`f0 new`、`f0 open` 和 `f0 doctor` 就会拒绝该项目。`Ctrl-D` 会在写入任何内容之前取消，而且它从不覆盖：如果仓库已经有项目或文件已存在，它会失败。

## f0 edit

逐个问题更新项目文件，并在保存前显示改动。

```bash
f0 edit [project]
```

| 参数 | 含义 |
|---|---|
| `[project]` | 要编辑的项目。默认是你所在仓库的项目 |

它会询问与 `f0 init` 相同的问题，并以当前值作为默认值，所以按 Enter 就会保留。对于每个服务器，你可以选择保留、编辑或删除，之后还可以添加更多。

```
Base branch [develop]:
Branch prefixes, comma separated, the first is the default [feature/, fix/, refactor/, chore/]:
AI agent command (claude, codex, gemini, opencode...) [claude]:
Env files: using the repo's .worktreeinclude
Server api: pnpm nx serve @org/api
  Keep, edit or remove? [K/e/r]: e
  Name [api]:
  Command [pnpm nx serve @org/api]:
  Folder, relative to the repo [.]:
  Start it with every task? [y/N]: y
Server admin: pnpm nx dev @org/admin
  Keep, edit or remove? [K/e/r]: r
Add another server? [y/N]:
```

如果没有任何改动，它会告诉你并保持文件不变。否则它会显示文件的 `git diff`，并在保存前询问；新文件会一步替换旧文件。`editor`、`agent` 和 `layout` 会保留，你自己写的注释则不会。`f0 init --template` 留下的占位符会作为问题让你填写。它需要终端。

## f0 new

租用一个带有新分支的 treehouse slot，并打开它的工作区。

```bash
f0 new <ticket> [--project <name>] [--base <branch>] [--type <type>] [--start <servers>] [--agent <command>] [--no-attach]
```

| 参数 | 含义 |
|---|---|
| `<ticket>` | 任务 id，追加在项目 `branch_prefixes` 的第一个前缀之后（`APP-123` 变成 `feature/APP-123`） |
| `-p`、`--project` | 项目名称。默认是 `repo` 包含当前目录的那个项目 |
| `--base` | 从哪个分支切出，覆盖项目中的 `base` |
| `-t`、`--type` | 分支类型：项目 `branch_prefixes` 中的一个，带不带 `/` 都可以（`--type fix` 会创建 `fix/APP-123`）。会话名仍然是 `my-app/APP-123` |
| `--start` | 立即启动的服务器，用逗号分隔（`--start api,web`）。其他服务器等待 Enter，除非它们设置了 `autostart = true` |
| `--agent` | 仅用于此任务的 AI 智能体，比如 `codex` 或 `opencode`，覆盖项目中的设置 |
| `--no-attach` | 创建会话但不连接，并输出会话名 |

需要 treehouse、项目文件及其 [worktree include 文件](/zh-cn/guides/worktrees/#worktreeinclude)。如果缺少其中任何一个，或者已存在同名会话，会在租用 slot 之前失败。

当分支已经存在时（本地或 `origin` 上），比如一个在合并前就被归还的任务，slot 会检出该分支而不是创建它，`--base` 不起作用。如果另一个检出正在使用该分支，它会失败：如果那是一个 slot，`f0 open` 可以重新打开它。

## f0 open

重新打开一个 treehouse slot 的工作区；如果它的会话正在运行，就连接到该会话。

```bash
f0 open [slot] [--start <servers>] [--agent <command>] [--no-attach]
```

| 参数 | 含义 |
|---|---|
| `[slot]` | 要打开的 slot：它的 id（`my-app/4`）、任务（`APP-123`）、会话名或路径（其中任意目录都可以），与 `f0 ls` 显示的一致。默认是当前目录 |
| `--start` | 重建会话时要启动的服务器，与 `f0 new` 相同。会话已在运行时不起作用 |
| `--agent` | 重建会话使用的 AI 智能体，与 `f0 new` 相同 |
| `--no-attach` | 创建会话但不连接 |

重建的会话会恢复智能体在该任务中的最后一次对话。参见[恢复对话](/zh-cn/reference/configuration/#恢复对话)。

当任务的会话被关闭后（例如 tmux 或电脑重启后），用它把任务找回来。slot 必须属于一个已配置的项目；其他目录会被拒绝。

已存在的目录总是优先于名称。单独的 slot 编号（`4`）也可以，但它会在多个项目中重复：这时会使用当前目录所在的项目；不在任何项目中时，f0 会列出可选的 id。

## f0 ls

列出正在使用的 slot，以及它们的任务和会话。

```bash
f0 ls [--all] [--paths]
```

```
ID         TASK        STATUS  SESSION
website/1  (detached)  in-use  website/slot-1
my-app/4   APP-123     leased  my-app/APP-123
Free slots: my-app 2 (`f0 ls --all` lists them)
```

| 参数 | 含义 |
|---|---|
| `--all` | 同时列出空闲的 slot |
| `--paths` | 添加一列，显示每个 slot 的目录 |

treehouse 按仓库为 slot 编号，所以同一个编号会出现在多个项目中。**ID** 列 `<project>/<slot>` 是唯一的，`f0 open` 和 `f0 done` 都接受它。**TASK** 是去掉项目 `branch_prefixes` 中匹配前缀之后的分支名。

## f0 switch

显示一个带编号的 f0 会话列表，并切换到你选中的那个。在 tmux 中会切换当前客户端；在 tmux 外则连接到该会话。

## f0 stop

关闭 slot 的会话，但保留 slot 及其分支和未提交的改动，以便之后回来。

```bash
f0 stop [slot]
```

接受的 slot 写法与 `f0 done` 相同，默认是当前目录。智能体和服务器会停止；`f0 ls` 仍会列出该 slot，`f0 open APP-123` 会重建它的会话。在要关闭的会话中输入时，会话会立即关闭。

## f0 done

关闭 slot 的会话，并把 slot 归还到池中。

```bash
f0 done [slot] [--force]
```

可以用 `f0 ls` 显示的任意形式指定 slot：

```bash
f0 done my-app/4                             # ID 列：在所有项目中唯一
f0 done APP-123                              # TASK 列
f0 done my-app/APP-123                       # SESSION 列
f0 done ~/.treehouse/my-app-3f9a2c/4/my-app  # 路径，来自 `f0 ls --paths`
```

不指定 `[slot]` 时使用当前目录。先关闭会话，然后 `treehouse return` 清理 slot，在丢弃未提交的改动之前会询问；`--force` 会不经询问直接丢弃。不是 slot 的目录会被拒绝。

在它要关闭的会话中，关闭会话会让 `f0 done` 自己中途停止，所以它会改为在 tmux 状态栏中询问（`Close my-app/APP-123 and return its slot? (y/n)`，如果有会被丢弃的未提交改动，还会显示数量），然后你的终端离开 tmux，在那里关闭会话并归还 slot，让你看到结果。`Ctrl-a X` 在任何窗格中都提供这个操作，旁边还有关闭会话但保留 slot 的选项（`f0 stop`），会话的状态栏会显示这个按键。如果归还 slot 失败，终端会显示原因并等待 Enter，slot 仍会留在 `f0 ls` 中。

## f0 keys

在正在运行的 tmux 服务器上绑定关闭任务的按键，并在每个 f0 会话的状态栏中显示它。

```bash
f0 keys
```

`f0 new` 和 `f0 open` 会为它们的会话执行这一步，`install.sh` 在更新后也会运行它，所以已经打开的会话无需重新加载 tmux 就能获得新的绑定。按键来自 [`[keys] done`](/zh-cn/reference/configuration/#按键)；修改后，运行 `f0 keys` 解除旧按键的绑定。

## f0 doctor

检查工具、全局默认值、项目配置、tmux 按键，以及阻止内置主题生效的因素。缺少必需项时以错误退出。

| 检查项 | 必需 |
|---|---|
| `tmux`、`git`、`treehouse` | 是 |
| 至少一个项目 | 是 |
| 默认值和每个项目中的编辑器与智能体命令 | 是 |
| 每个项目的 `repo` 和 worktree include 文件 | 是 |
| 正在运行的 tmux 服务器绑定了关闭任务的按键（没有服务器或设置了 `[keys] done = false` 时跳过） | 是 |
| `gh`、`rg`、Ghostty | 否 |
| 链接了内置 tmux 配置时：tmux 3.8 或更高版本，主题需要它 | 是 |
| 链接了内置 Ghostty 配置时：其他 Ghostty 配置文件中没有 `background`、`foreground`、`palette`、`cursor-*` 或 `selection-*` 行，有的话会列出其文件和行号 | 否 |

## 环境变量

| 变量 | 读取方 | 含义 |
|---|---|---|
| `F0_CONFIG_DIR` | f0 | 配置目录，覆盖默认值 |
| `XDG_CONFIG_HOME` | f0 | 配置目录变为 `$XDG_CONFIG_HOME/f0` |
| `F0_PROJECT` | 窗格 | 在会话的每个窗格中设置：项目名称 |
| `F0_BASE` | 窗格 | 项目设置了 `base` 时设置；Neovim 中的 `Space gD` 会用到 |
