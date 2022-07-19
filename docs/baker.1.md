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

## CUSTOM COMMANDS

Custom commands can be set using **custom**.

```toml
# clean is taken as the name of the custom command

[custom.clean]
cmd = "cargo clean" # cmd
run = false # if it should run after build is executed
```

You can also run custom commands directly by invoking baker with the name of the command as the argument.

Example:

```
bake clean
```

## RUNNING COMMANDS BEFORE BUILD

You can also run commands before build using **pre**.

```
# fmt is taken as the name of the pre command

[pre.fmt]
cmd = "cargo fmt"
```

## ENVIRONMENT VARIABLES

You can set env vars using **env**.

```toml
[env]
TEST_ENV="foo"
TEST_ENV_2="bar"
```

## RECURSION

Baker also supports recursion (invoking baker inside baker):

Example:

```toml
[custom.docs]
cmd = "pandoc docs/baker.1.md -s -t man -o docs/baker.1"
run = false

[custom.view-docs]
cmd ="bake docs && man ./docs/baker.1"
run = false
```

Running **bake view-docs** will run **bake docs** and view the man page.
