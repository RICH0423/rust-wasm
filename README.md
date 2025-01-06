# rust-wasm

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
