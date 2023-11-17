# wasm_pi

Rust WebAssembly example

## Build & Run

- Install `wasm-pack`, the high-level build tool for Rust WebAssembly projects

```sh
cargo install wasm-pack
```

- Build the project

```sh
wasm-pack build --target web
```

- Install a simple web server (Host These Things Please)

```sh
cargo install https
```

- Start the web server in the project root

```sh
http
```

- Open the application in your browser: `http://localhost:8000`
