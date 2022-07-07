# Baker

A simple build system

### Compiling via make

```
make setup
make release
```

A binary will be copied to `./bin/baker`

### Usage

```
bake
```

### Configuration

```toml
[build]
cmd = "cargo build --release" # cmd to run on build
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
