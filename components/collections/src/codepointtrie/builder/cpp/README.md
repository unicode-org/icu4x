# CodePointTrie Builder Tool

This directory contains C++ bindings to the ICU4C CodePointTrie builder in the form of an executable named `list_to_ucptrie`. There are two ways to run this tool:

1. As a WebAssembly module
2. As a native executable

The WebAssembly module is checked into tree and made available to Rust library clients.

## Usage

`list_to_ucptrie` takes line-separated integers in STDIN, corresponding to the trie values for code points 0, 1, 2, and so on. When it receives an EOF (`^D`), it prints helpful information to STDERR and dumps the CodePointTrie TOML to STDOUT.

It takes three positional arguments:

1. Default value for code points (integer)
2. Error value for out-of-range code points (integer)
3. TrieType: `fast` or `small`

## WebAssembly Module

To build the WebAssembly module, you need:

- Local copy of the ICU4C sources (May 6, 2022 or later: [eea7985](https://github.com/unicode-org/icu/commit/eea7985e5a7108d00f1224ed36f0220fa9441cdc))
- [wasienv](https://github.com/wasienv/wasienv)

Once you have these two tools installed, from this directory, simply run:

```bash
$ make ICU4C_SOURCE=/path/to/icu4c/source list_to_ucptrie.wasm
```

To test the WASM file, you can use `wasirun`:

```bash
$ wasirun list_to_ucptrie.wasm
Takes 3 positional arguments: default value, error value, and trie type
```

You can then copy the wasm file up one directory in order to update the version shipped with ICU4X.

## Native Executable

To build the native executable, you need:

- Local copy of the ICU4C sources
- ICU4C built as a static library

For example, to build ICU:

```bash
$ cd icu4c/source
$ ./runConfigureICU Linux --enable-static
$ make -j6
```

Then, from this directory, simply run:

```bash
$ make ICU4C_SOURCE=/path/to/icu4c/source list_to_ucptrie
```

If you have an out-of-source build of ICU4C, set both `ICU4C_SOURCE` (for header file includes) and `ICU4C_BUILD` (for linking with .a files).

You now have a binary `list_to_ucptrie` with ICU4C built-in that you can run:

```bash
$ ./list_to_ucptrie 0 0 small
```

Type the following in STDIN:

```
22
33
44
```

Press `^D` and watch the magic occur:

```
Saved values for code points up to 3
index = [
  0,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,
  0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,
  0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,
  0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40
]
data_8 = [
  0x16,0x21,0x2c,0,0,0,0,0,0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0
]
indexLength = 64
dataLength = 128
highStart = 0x200
shifted12HighStart = 0x1
type = 1
valueWidth = 2
index3NullOffset = 0x7fff
dataNullOffset = 0x40
nullValue = 0x0
```
