# CHIP-8 interpreter

![](assets/chip8.png)

A [CHIP-8](https://en.wikipedia.org/wiki/CHIP-8) interpreter in rust, with some debug features.

## Native

Running the SDL-based interpreter:

```bash
$ RUST_LOG=trace cargo run --package chip8-sdl -- --rom "roms/Trip8 Demo (2008) [Revival Studios].ch8"
```

## WebAssembly

The WebAssembly version (located under `wasm`) is limited to a limited number of ROMS.

In order to run the development server:

```bash
$ cd wasm
$ wasm-pack build   # this will create a pkg directory with the npm module
$ cd www            # navigate to the webapp
$ npm run start     # start serving on http://localhost:8080/
```

To deploy the web app, run `npm run build` from the `www` directory and deploy the contents of the generated `dist` directory.

## Links

* https://germangb.github.io/chip8/
* https://rustwasm.github.io/docs/book/
* https://github.com/rustwasm/wasm-pack