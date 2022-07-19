# Baker

A simple build automation tool like GNU Make.

### Dependencies

Baker calls `sh -c <cmd>` to execute your commands. I originally wanted to directly execute the commands, but parsing variables
and other shell syntax added a lot of extra code which could have been easily prevented with `sh`.

### Installing

#### Arch Linux

Baker is available in the [AUR](https://aur.archlinux.org/packages/baker-git) (`baker-git`).

### Compiling via baker

```
bake setup
bake
```

A binary will be copied to `./bin/bake`

**Install**
`sudo bake install`

**Uninstall**
`sudo bake uninstall`

For building man pages, install [pandoc](https://pandoc.org/) and run:

```
bake docs
```

The exported man page will be in `./docs/baker.1`.

### Usage

-   Execute command in build: `bake`

-   List all commands: `bake [-c, --commands]`

-   Show help message: `bake [-h, --help]`

-   Run a custom command: `bake [command]`

### Configuration

Baker looks for a `recipe.toml` in the root directory. If it doesn't find one, it generates one:

```toml
[build]
cmd = ""
```

`build` is executed when the binary is invoked without any flags.

#### Custom commands

Custom commands can be set using `custom`.

```toml
# clean is taken as the name of the custom command

[custom.clean]
cmd = "cargo clean" # cmd
run = false # if it should run after build is executed
```

You can also run custom commands directly by invoking baker with the name of the cmd as the argument.

Example:

```
bake clean
```

#### Running commands before build

You can also run commands before build using `pre`.

```toml
# fmt is taken as the name of the pre command

[pre.fmt]
cmd = "cargo fmt"
```

#### Environment variables

You can set env vars using `env`.

```toml
[env]
TEST_ENV="foo"
TEST_ENV_2="bar"
```

#### Recursion

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

Running `bake view-docs` will run `bake docs` and view the man page.

An example config can be found in [recipe.toml](./recipe.toml)
