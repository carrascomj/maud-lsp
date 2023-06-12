# maud-lsp

A Language Server for [Maud](https://github.com/biosustain/Maud).

Supports `Hover` and `GotoDefinition`, of **Metabolites**, **Reactions** and **Enzymes**.

![Maud screenshot](assets/maud_screen.png "Maud screenshot") 

## Installation

There are x84 binaries available for Linux, Mac and Windows at the
[release page](https://github.com/carrascomj/maud-lsp/releases/latest):

1. Download one of the compressed files (the one for your OS).
2. Decompress it. For instance, if you are running Linux:
```bash
tar xf maud-lsp-x86_64-unknown-linux-gnu.tar.gz 
```
3. Put it in your path. For instance:
```bash
# if $HOME/.local/bin is in your $PATH
mv maud-lsp ~/.local/bin
```

### Neovim setup

Copy-paste [`maud-ls.lua`](./assets/maud-ls.lua) in your `init.lua` config.

### VScode setup

It's on the marketplace! Go to the extension tab (probably `Ctrl-Shift-X`) and look for _maud_.

### Helix setup

Copy-paste [`maud-ls.toml`](./assets/maud-ls.toml) in your `languages.toml`.

### Building from source

Install [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) and run

```bash
git clone https://github.com/carrascomj/maud-lsp.git
cd maud-lsp
cargo install --path .
```

## Acknowledgments

Everything was more or less stolen from [`rust-analyzer`](https://github.com/rust-lang/rust-analyzer/).

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be licensed as above, without any additional terms or conditions.
