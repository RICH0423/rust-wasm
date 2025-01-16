# Rust 🦀 and WebAssembly 🕸

## WASM-PACK Template

[**📚 Read this template tutorial! 📚**](https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html)

This template is designed for compiling Rust libraries into WebAssembly andpublishing the resulting package to NPM.

- [tutorials](https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html)
- [template-docs](https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html)

- using **strip = true** to fix Wasm module parse error!
```
ERROR in ../pkg/wasm_game_of_life_bg.wasm
Module parse failed: Unknown element type in table: 0xNaN
You may need an appropriate loader to handle this file type, currently no loaders are configured to process this file. See https://webpack.js.org/concepts#loaders
Error: Unknown element type in table: 0xNaN
```

```toml
[profile.release]
# Tell `rustc` to optimize for small code size.
opt-level = "s"
strip = true

[profile.dev]
strip = true
```

## [Tutorial: Conway's Game of Life](https://rustwasm.github.io/docs/book/game-of-life/introduction.html#tutorial-conways-game-of-life)
This is a tutorial that implements Conway's Game of Life in Rust and WebAssembly.

## [WebAssembly Linear Memory](webassembly-linear-memory)
Linear Memory is a vector of contiguous bytes in memory. The WebAssembly specification allows for modules to have more than one instance of a linear memory, but at the time this course was written, all practical implementations assume that memory operations implicitly operate on the memory instance at index 0.

Linear memory is allocated in multiples of the page size (65535 bytes or 64KiB), and WebAssembly modules can manipulate that memory without fear of interfering with any other memory used by the host or other modules. We have seen a few of these memory instructions in the WebAssembly text format before:

```
(type).load
For example, i32.load.

(type).store 
For example, f32.store.

size
Query the size of the memory instance.

grow 
Request an expansion of the memory instance. The new maximum size must be specified in multiples of the page size.
```
