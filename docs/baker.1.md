% Baker(1) Baker 1.0.0
% rv178
% July 2022

# NAME

Baker - A simple build automation tool.

# SYNOPSIS

**bake** [*OPTIONS*]

# DESCRIPTION

Baker is a simple build automation tool configured via TOML.

# OPTIONS

**-h**, **--help**
: Help command

**-v**, **--version**
: Check version

**-c**, **--commands**
: List commands

**[command]**
: Run a command

# CONFIGURATION

Baker looks for a **recipe.toml** in the root directory. If it doesn't find one, it generates one:

```toml
[build]
cmd = ""
```

**build** is executed when the binary is invoked without any flags.

Custom commands can be set using **custom**.

```toml
[[custom]]
name = "clean" # name of cmd
cmd = "cargo clean" # cmd
run = false # if it should run after build is executed
```

You can also run custom commands directly by invoking baker with the name of the command as the argument.

Example:

```
bake clean
```

You can also run commands before build using **pre**.

```
[[pre]]
name = "fmt"
cmd = "cargo fmt"
```
