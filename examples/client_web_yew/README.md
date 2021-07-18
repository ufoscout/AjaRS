# web_yew_web_sys

A simple [Yew](https://yew.rs) based WASM application that uses `ajars_web_sys` to call the backend.

## Requirements

To build this application you need to have [Trunk](https://trunkrs.dev/) and [wasm-bindgen-cli](https://rustwasm.github.io/wasm-bindgen/) on your system.
You need also the rustc `wasm-unkknwon-unknowkn` target to compile it to WASM.

To install them:
```bash
cargo install trunk wasm-bindgen-cli

rustup target add wasm32-unknown-unknown
```

## Start the frontend localy

Start the frontend using `Trunk`:

```bash
trunk serve
```

This starts a local http server at port 3000. To use the application, open your browser and navigate to [http://127.0.0.1:3000](http://127.0.0.1:3000)


