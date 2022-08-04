# Wasm FFI for ICU4X

This folder contains the WebAssembly (Wasm) FFI for ICU4X. There are two halves: the ICU4X NPM package, and a demo application.

## ICU4X NPM Package
The `icu4x/` folder contains the contents for the ICU4X NPM package. Most importantly, it contains the Diplomat-generated JavaScript bindings, TypeScript header files, and documentation.

To run the package's tests, run:
```sh
# ffi/diplomat/wasm/icu4x/
npm run test
```

To re-generate, navigate to the repository root and run:
```sh
cargo make diplomat-gen-js
```
This command clears out the existing bindings, and invokes `diplomat-tool` to create new ones. **You shouldn't have to run this unless you change the Diplomat bindings on the Rust side.**

Usually getting the compiled `.wasm` binary is unnecessary because CI can automatically generate it since it's not saved in the repo. Regardless, to generate the `.wasm` binary that the bindings use, run:
```sh
cargo make wasm-copy-to-diplomat
```
This compiles ICU4X into WebAssembly in release mode, and copies the resulting binary into `ffi/diplomat/wasm/icu4x/lib/icu_capi.wasm`. This step is performed automatically when running `cargo make diplomat-build-webpack-demo`, which is part of building the demo described below.

## Wasm Demo Application
The `wasm-demo/` folder contains a demo application that using Webpack and the wasm bindings in `icu4x/` folder. It serves as a baseline demo to show off some of the features that ICU4X offers.

To build the demo, run:
```sh
cargo make diplomat-build-webpack-demo
```
This gets the latest bindings and `.wasm` binary from the `icu4x/` folder and invokes Webpack to bundle everything together. The resulting files are placed in `wasm-demo/dist/`.

At this point, the demo can be viewed from the browser:
```sh
# ffi/diplomat/wasm/wasm-demo
python3 -m http.server
```

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
