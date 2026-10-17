# Install

use mise to pin the dependencies for building for Arduino Mega

## Toolchain

- make sure you have `xcode` CLI tool installed

```sh
xcode-select --install
```

Install the toolchain and then the rust-src compoment from that version 
of the toolchain.

```sh
rustup toolchain install nightly-2026-10-16
rustup component add rust-src --toolchain nightly-2026-10-16
```

## cargo-generate

https://github.com/cargo-generate/cargo-generate

```sh
mise use --pin cargo:cargo-generate
```

## Ravedude

### dependencies

```sh
brew tap osx-cross/avr
brew trust osx-cross/avr
brew install avr-gcc avrdude
```
### install

```sh
mise use --pin cargo:ravedude
```

## Select the toolchaing

in `./mise.toml` add:

```toml
[env]
RUSTUP_TOOLCHAIN = "nightly-2026-10-16"
```
- verify the env variable is taking effect:

```sh
echo $RUSTUP_TOOLCHAIN
nightly-2026-10-16
```
- verify the toolchain is active:

```sh
rustup show
Default host: aarch64-apple-darwin
rustup home:  /Users/mac/.rustup

installed toolchains
--------------------
stable-aarch64-apple-darwin (default)
nightly-aarch64-apple-darwin
nightly-2025-04-27-aarch64-apple-darwin
nightly-2026-10-16-aarch64-apple-darwin (active)
1.67.1-aarch64-apple-darwin
1.89.0-aarch64-apple-darwin
1.96.1-aarch64-apple-darwin
esp

active toolchain
----------------
name: nightly-2026-10-16-aarch64-apple-darwin
active because: overridden by environment variable RUSTUP_TOOLCHAIN
installed targets:
  aarch64-apple-darwin
```

## Generate a new the project from template

Select the version: `Arduino Mega 2560`

```sh
cargo generate --git https://github.com/Rahix/avr-hal-template.git
```


