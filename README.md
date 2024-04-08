# Workflows

A CLI tool for creating a seamless workflow with remote and local
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

Installing latest from source

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
greeted with a `fzf` UI with your list of projects. Local projects
will be displayed instantly, however GitHub projects may take a 
second to load in depending on your connection strength. Select 
the project you'd like to open, and it'll be launched in a tmux 
session. If a tmuxinator config doesn't exist for the project, 
one will be created.

Projects are stored in `~/Projects/` by default, however this can be
changed via the configuration file. This is also where GitHub projects
are cloned to. If a project exists elsewhere on your system it won't be
detected.

What commands are run with `tmuxinator` can be configured using 
the config file.

#### `--open` shortcut

If you're visiting a particular project frequently, it might be helpful
to have the shortcut in your recent terminal history. This isn't possible
through the fzf interface, which is where the `--open` (or `-o`) command
comes in.

You can open a local project directly by supplying the projects name
in full as an argument to the `--open` command:

```console
workflows --open workflows
```

The following also works:

```console
workflows -o workflows
```

### Creating a project

To create a project, run `workflows --new <project-name>`. This creates a
directory with the name of the project in the projects directory. A
`tmuxinator` config is created and run, launching the project in `tmux`.

The command `--new` can be exchanged for the shorthand `-n`.

### Deleting a project

To delete a project, run `workflows --delete`. You'll be greeted 
with a `fzf` UI, but only with local projects. Selecting one will
cause checks to be run on whether the repo has a clean working tree 
and is pushed to main. **NOTE** this only works for remote branches.
With confirmation the project will be deleted. 

Deleting using `workflows --delete` deletes the tmuxinator config 
as well as the project in the projects directory.

### Cloning a project

Workflows can also be used to clone git repos using `workflows --clone`,
or the short flag `-c`. This command is helpful for checking out projects 
that are not your own, and thus do not show up in the default open prompt.

## Configuration

`workflows` is configured by a toml file in either of the following paths

- `~/.config/workflows/config.toml`
- `~/.workflows.toml`

If the first option cannot be found, the second one is looked for. If neither is present then the
default configuration is used

### Default Configuration

```toml
[general]
programs_dir="Projects/"

[fzf]
layout="default"
border="none"
border_label=""
open_prompt="Open: "
delete_prompt="Delete: "
pointer=">"
theme="dark"

[github]
enabled=true
confirm_cloning=true
project_indicator="  "

[git]
check_tree=true
check_push=true

[tmuxinator]
fresh_config=false
window_names=["editor"]
start_commands=["nvim ."]
```

### General configuration

| Option | Purpose | Default Value |
| ------ | ------- | ------------- |
| `projects_dir` | Where `workflows` should look for projects. Path is relative to $HOME | `"Projects/"` |

### fzf configuration

| Option | Purpose | Default Value |
| ------ | ------- | ------------- | 
| `layout` | The layout fzf should use, either `default`, `reverse`, or `reverse-list`| `"default"` |
| `border` | The border fzf should display around the finder. Options include: `rounded`, `sharp`, `none`, etc. For more run `fzf --help`. All fzf border types are supported. | `"none"` |
| `border_label` | The label to show in the border. If blank or no border is selected, the label won't show | `""` |
| `open_prompt` | The prompt fzf should show when opening a project | `"Open: " ` |
| `delete_prompt` | The prompt fzf should show when deleteing a project | `"Delete: " ` |
| `pointer` | The pointer to the current item in fzf | `">"` |
| `theme` | The theme to use with fzf. Options include: `dark`, `light`, `16`, `bw` | `"dark"` |

### github configuration

**NOTE** Both of these options require the `gh` cli tool, and for the user to be logged in.

| Option | Purpose | Default Value |
| ------ | ------- | ------------- | 
| `enabled` | Whether to enable GitHub Intergration  | `true` |
| `confirm_cloning` | Whether to ask for confirmation before cloning a git repo into `~/Projects/` | `true` |
| `project_indicator` | A string that is prepended to a github project in the list view, to disable leave blank. | `  ` |

### git configuration

**NOTE** Both of these options require git, and for the project to be a git repo. If one of these
requirements is not fulfilled then both of these options will always be false when run.

| Option | Purpose | Default Value |
| ------ | ------- | ------------- | 
| `check_tree` | Check if the git repo has a clean working tree before deletion | `true` |
| `check_push` | Check if the git repo has been pushed before deletion | `true` |

### tmuxinator configuration

| Option | Purpose | Default Value |
| ------ | ------- | ------------- | 
| `fresh_config` | If enabled, a tmuxinator configuration will be generated before every project launch. This can be useful if you've updated the other two options below | `false` |
| `window_names` | An array of names of the tmux windows tmuxinator should create | `["editor"]` |
| `start_commands` | An array of the commands to run for each window when tmux is started. If there are not enough commands for the defined windows, then the default option is used. | `["nvim ."]` |

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
