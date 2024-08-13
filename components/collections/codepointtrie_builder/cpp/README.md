# CodePointTrie Builder Tool

This directory contains C++ bindings to the ICU4C CodePointTrie builder in the form of a wrapper named `ucptrie_wrap`. This tool is intended to be used as a shared library in WebAssembly.

The WebAssembly module is checked into tree and made available to Rust library clients.

Since ICU4X 2.0, we check the WebAssembly Text (.wat) file into source control to make it more easily reviewable.

## WebAssembly Module

To build the WebAssembly module, you need:

- Local copy of the ICU4C sources (May 6, 2022 or later: [eea7985](https://github.com/unicode-org/icu/commit/eea7985e5a7108d00f1224ed36f0220fa9441cdc))
- The following packages which can be fetched from apt-get:
  - `lld`
  - `libc++-dev-wasm32`
  - `libclang-rt-dev-wasm32`
  - `wabt`

Once you have these two tools installed, from this directory, simply run:

```bash
$ make clean
$ make ICU4C_SOURCE=/path/to/icu4c/source ucptrie_wrap.wat
```

You can then copy the wasm file up one directory in order to update the version shipped with ICU4X.
