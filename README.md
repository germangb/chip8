# chip8 interpreter

A [**chip8**](https://en.wikipedia.org/wiki/CHIP-8) interpreter in Rust, with some debug features.

## Native

In order to run the interpreter using SDL, run the following on your shell from the root of the repository:

```bash
$ RUST_LOG=trace cargo run --package chip8-sdl2 -- --rom "<rom_path>"
```

## WebAssembly

Directory `wasm/` contains a WASM implementation of the interpreter, which can be run from the **[website of this repository](#)**.

Most of the files were generated using the *Rust and WebAssembly* book, and the web app limited to only a single ROM.

## Other

* The [Rust 🦀 and WebAssembly 🕸](https://rustwasm.github.io/docs/book/) book.
