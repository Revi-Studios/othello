# Othello written in _Rust_

A simple othello game built with [The Rust Programming Language](https://rust-lang.org/) using the [Macroquad](https://macroquad.rs/) immediate mode game engine.

|                                                           |                                                               |
| --------------------------------------------------------- | ------------------------------------------------------------- |
| ![Game play](docs/screenshots/medium-early-medium.png)    | ![Intro](docs/screenshots/medium-medium-intro.png)            |
| ![Game Paused](docs/screenshots/medium-medium-paused.png) | ![Game Settings](docs/screenshots/medium-medium-settings.png) |

# Build

Wasm:

1. Build the binary

```
cargo build --release --target wasm32-unknown-unknown && cp ~/.cargo/shared_target/wasm32-unknown-unknown/release/othello.wasm docs/web-demo/
```

2. Host the web page

```
% basic-http-server docs/web-demo
	 INFO basic-http-server 0.11.0
	 INFO addr: http://127.0.0.1:4000
	 INFO root dir: docs/wasm-build
	 INFO extensions: false
	 INFO listening on 127.0.0.1:4000
listening on 127.0.0.1:4000
```
