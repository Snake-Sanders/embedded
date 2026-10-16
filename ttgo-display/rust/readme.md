# Development

## Using the right Rust tool

If rust is installed with `mise`, then `rustup` will use the default toolchain
which is the local computer architecture.

`Mise` will ignore your `rust-toolcheain.toml` anyway, so the way
to tell Mise to use Esp toolchain is by adding the file to the root folder
of the project:

1. add **mise.toml**

```toml
[env]
RUSTUP_TOOLCHAIN = "esp"
```
2. trust the directory

Then in this directory run `mise trust`.

3. Verify the toolchain is selected

```sh
rustup show
Default host: aarch64-apple-darwin
rustup home:  /Users/mac/.rustup

installed toolchains
--------------------
stable-aarch64-apple-darwin (default)
nightly-aarch64-apple-darwin
1.67.1-aarch64-apple-darwin
1.89.0-aarch64-apple-darwin
1.96.1-aarch64-apple-darwin
esp (active) //<-- ESP is the active

active toolchain
----------------
name: esp
active because: overridden by environment variable RUSTUP_TOOLCHAIN
installed targets:
  aarch64-apple-darwin
```

## Flashing

```sh
> cargo run
...(building)

> Please select a port
Serial port: '/dev/cu.usbserial-58AB0097191'
```
