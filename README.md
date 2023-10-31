# Workflows

A cli tool for creating a seemless workflow with remote and local
git repos.

`workflows` allows users to view projects on their GitHub and their
local machine. The selected project is then opened in a tmux session.

If the selected project is not already present on the user's machine,
it is cloned from GitHub and then opened in a tmux session. 

Projects are selected using `fzf` for a fluent keyboard driven workflow.

## Requirements

`workflows` makes use of the following programs

| Program | Purpose | Required |
| ------- | ------- | -------- |
| `fzf` | Selecting Projects | Yes |
| `gh` | Viewing and Cloning github projects | Optional |
| `git` | Checking the status of local repos | Optional |
| `tmux` | Terminal Multliplexer that projects are launched in | Yes |
| `tmuxinator` | Session manager for `tmux` | Yes |

Both `gh` and `git` integration can be disabled via configuration,
however they are both enabled by default.

### Checking Requirements

To check if the needed programs are installed, run workflows health check

```console
workflows --health
```

## Installation

Installing from cargo

```console
cargo install workflows
```

Installing from source

```console
cargo install --git https://github.com/danielronalds/workflows
```

Building from source

```console
git clone https://github.com/danielronalds/workflows
cd workflows
cargo build --release
```

## Usage

### Opening a project

To open a project, open a terminal and run `workflows`. You'll be
greeted with a `fzf` ui with your list of projects. Local projects
will be displayed instantly, however GitHub projects may take a 
second to load in depending on your connection strength. Select 
the project you'd like to open, and it'll be launched in a tmux 
session. If a tmuxinator config doesn't exist for the project, 
one will be created.

Projects are stored in `~/Projects/`. This is also where GitHub 
projects are cloned to. If a project exists elsewhere on your 
system it won't be detected.

What commands are run with `tmuxinator` can be configured using 
the config file.

### Deleting a project

To delete a project, run `workflows --delete`. You'll be greeted 
with a `fzf` ui, but only with local projects. Selecting one will
cause checks to be run on whether the repo has a clean working tree 
and is pushed to main. **NOTE** this only works for remote branches.
With confirmation the project will be deleted. 

Deleting using `workflows --delete` deletes the tmuxinator config 
as well as the project in `~/Projects/`.

## Configuration

`workflows` is configured by a toml file in either of the following paths

- `~/.config/workflows/config.toml`
- `~/.workflows.toml`

If the first option cannot be found, the second one is looked for. If neither is present then the
default configuration is used

### Default Configuration

```toml
[fzf]
reverse_layout=true

[github]
enabled=true
confirm_cloning=true

[git]
check_tree=true
check_push=true

[tmuxinator]
fresh_config=false
on_open="nvim ."
window_name="editor"
```

### fzf configuration

| Option | Purpose | Default Value |
| ------ | ------- | ------------- | 
| `reverse_layout` | Whether the reverse layout should be used with fzf or not | `true` |

### github configuration

**NOTE:** Both of these options require the `gh` cli tool, and for the user to be logged in.

| Option | Purpose | Default Value |
| ------ | ------- | ------------- | 
| `enabled` | Whether to enable GitHub Intergration  | `true` |
| `confirm_cloning` | Whether to ask for confirmation before cloning a git repo into `~/Projects/` | `true` |

### git configuration

**NOTE:** Both of these options require git, and for the project to be a git repo. If one of these
requirements is not fulfilled then both of these options will always be false when run.

| Option | Purpose | Default Value |
| ------ | ------- | ------------- | 
| `check_tree` | Check if the git repo has a clean working tree before deletion | `true` |
| `check_push` | Check if the git repo has been pushed before deletion | `true` |

### tmuxinator configuration

| Option | Purpose | Default Value |
| ------ | ------- | ------------- | 
| `fresh_config` | If enabled, a tmuxinator configuration will be generated before every project launch. This can be useful if you've updated the other two options below | `false` |
| `on_open` | The command to run when the tmux session is opened. Typically your terminal text editor. | `"nvim ."` |
| `window_name` | The name of the tmux window | `"editor"` |

## Window Manager Integration

I wrote this program to be used with a window manager in mind, as it encourages even less interaction 
with a mouse. It has only been tested on Hyprland using kitty.

### Hyprland Keybinds

This snippet below adds a bind for launching `workflows` with kitty in Hyprland

```
bind=SUPER_SHIFT,W,exec,kitty workflows
```

And this one is for running delete mode

```
bind=SUPER_SHIFT_CTRL,W,exec,kitty workflows --delete
```
