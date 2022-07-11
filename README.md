# Baker

A simple build automation tool like GNU Make.

### Compiling via baker (release)

```
bake setup
bake release
```

A binary will be copied to `./bin/bake`

For building man pages, install [pandoc](https://pandoc.org/) and run:

```
bake docs
```

The exported man page will be in `./docs/baker.1`.

### Usage

```
bake
```

### Configuration

Baker looks for a `recipe.toml` in the root directory. If it doesn't find one, it generates one:

```toml
[build]
cmd = ""
```

`build` is executed when the binary is invoked without any flags.

Custom commands can be set using `custom`.

```toml
[[custom]]
name = "clean" # name of cmd
cmd = "cargo clean" # cmd
run = false # if it should run after build is executed
```

You can also run custom commands directly by invoking baker with the name of the cmd as the argument.

Example:

```
bake clean
```

You can also run commands before build using `pre`.

```toml
[[pre]]
name = "fmt"
cmd = "cargo fmt"
```

An example config can be found in [recipe.toml](./recipe.toml)

### Screenshots

![Alt](https://media.discordapp.net/attachments/985433521084563486/994926621226172467/unknown.png)
