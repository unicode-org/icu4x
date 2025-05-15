# Size-optimized C++ builds

This folder contains examples for various tactics for reducing the codesize of an ICU4X build when linked with C.

The results of the benchmarks (Rust `nightly-2024-01-01`, Clang/LLD 17`) are:

```
# Fixed Decimal Formatting
-rwxr-x--- 1 manishearth primarygroup 5273536 Dec 27 14:25 1-release.elf
-rwxr-x--- 1 manishearth primarygroup  387664 Dec 27 14:25 2-release-gcc-stripped.elf
-rwxr-x--- 1 manishearth primarygroup 1178184 Dec 27 14:25 3-panic-abort-clang.elf
-rwxr-x--- 1 manishearth primarygroup  133520 Dec 27 14:25 4-panic-abort-lto-clang.elf
-rwxr-x--- 1 manishearth primarygroup   47416 Dec 27 14:25 5-panic-abort-clang-stripped.elf
-rwxr-x--- 1 manishearth primarygroup   34040 Dec 27 14:25 6-panic-abort-lto-clang-stripped.elf
-rwxr-x--- 1 manishearth primarygroup   33296 Dec 27 14:25 7-panic-abort-linker-plugin-lto-clang-stripped.elf


# Segmenter
-rwxr-x--- 1 manishearth primarygroup 9601072 Dec 27 14:25 1-release.elf
-rwxr-x--- 1 manishearth primarygroup  719432 Dec 27 14:25 2-release-gcc-stripped.elf
-rwxr-x--- 1 manishearth primarygroup 5681200 Dec 27 14:25 3-panic-abort-clang.elf
-rwxr-x--- 1 manishearth primarygroup 4318128 Dec 27 14:25 4-panic-abort-lto-clang.elf
-rwxr-x--- 1 manishearth primarygroup  383280 Dec 27 14:25 5-panic-abort-clang-stripped.elf
-rwxr-x--- 1 manishearth primarygroup  372744 Dec 27 14:25 6-panic-abort-lto-clang-stripped.elf
-rwxr-x--- 1 manishearth primarygroup  372136 Dec 27 14:25 7-panic-abort-linker-plugin-lto-clang-stripped.elf
```

The maximally low-size build documented here involves using LTO with `-Clinker-plugin-lto`, `-Os` Rust, `panic=abort`, `panic-immediate-abort` std, `gc-sections`, and `--strip-all`, as seen in the `7-panic-abort-linker-plugin-lto-clang-stripped.elf`. Furthermore, the Fixed Decimal Format test works by baking in a minimal set of locales (in this case, English and Bengali).


```bash
# Pick a toolchain where Rust and Clang use the same underlying LLVM version
CLANG := clang-19
LLD := lld-19
LLVM_COMPATIBLE_NIGHTLY = "nightly-2025-02-17"

# Rust build
# ============


# Enable LTO, -Os, and panic=abort
RUSTFLAGS="-Clto -Cembed-bitcode -Clinker-plugin-lto -Clinker=$(CLANG) -Ccodegen-units=1 -Clink-arg=-flto -Cpanic=abort -Copt-level=s" \
`# Use the optimal baked data including the locales we care about` \
ICU4X_DATA_DIR=$(shell pwd)/baked_data \
`# Build ICU4X with the optimized looping panic handler` \
cargo +${LLVM_COMPATIBLE_NIGHTLY} rustc -p icu_capi --crate-type staticlib --no-default-features --features icu_capi/compiled_data,icu_capi/${CAPI_COMPONENT},icu_capi/looping_panic_handler,icu_capi/libc_alloc \
`# Build the standard library in panic-abort mode with panic-immediate-abort.` \
-Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-unknown-linux-gnu --release  --target-dir target-panic-abort-linker-plugin-lto 


# Clang build
# ===========

$(CLANG) \
`# Use ThinLTO with LLD` \
-flto=thin -fuse-ld=$(LLD) --target=x86_64-unknown-linux-gnu test.c -I${HEADERS} -o 7-panic-abort-linker-plugin-lto-clang-stripped.elf target-panic-abort-linker-plugin-lto/x86_64-unknown-linux-gnu/release/libicu_capi.a\
`# Dead-code eliminate unused sections and strip all symbols` \
 -Wl,--gc-sections -Wl,--strip-all
```


There is a lot of documentation on these strategies in [this guide](https://github.com/johnthagen/min-sized-rust?tab=readme-ov-file), but to give a gist of the different facets and their tradeoffs (tradeoffs impacting compile time are not listed), we have a list here. Note that in the benchmarks above, panic-abort builds always set `-Os`, and "stripped" builds are running both `gc-sections` and stripping.


# Size opt level

`RUSTFLAGS=-Os` or Cargo profile `opt-level=s`

This tunes the optimizer for producing lower codesize.

Tradeoff: This may impact runtime performance.

# panic-abort

`RUSTFLAGS=-Cpanic=abort` or Cargo profile `panic=abort`

`panic=abort` turns all panics into aborts after printing an error message. This is configurable in your Cargo profile with `panic=abort`.

Tradeoff: Unwinding will no longer work in this scenario.


# panic-immediate-abort

`cargo rustc ... -- -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort`

This requires Rust nightly, using `-Zbuild-std`, and setting `RUSTFLAGS`, but will make errors _immediately_ abort instead of attempting to format the panic message. This gets rid of a lot of formatting code in the final binary, which tends to take up a fair amount of space.

Tradeoff: This requires Rust nightly, and removes all panic error messages.

# gc-sections

`clang -fdata-sections -ffunction-sections .... -Wl,--gc-sections`

This performs dead code elimination on the binary at link time. This is a coarser form of link time optimization that is quick and dirty and gets rid of a lot of low hanging fruit.

It's not really necessary to use this when using LTO.

Tradeoff: None

# strip

`clang .... -Wl,--strip`

This removes all symbols, which severely impacts debuggability but greatly reduces codesize. Debuggability can be regained by setting up split debuginfo, which is not documented here but is possible.

Tradeoff: Debugging can be harder

# LTO

`clang ... -flto -fuse-ld=$(LLD)`, `RUSTFLAGS=-Clto -Cembed-bitcode` (or Cargo profile `lto = true` / `lto = thin`)

This performs additional link time optimization based on LLVM IR. LTO can be performed by Rust, Clang, or both. Explicitly setting the linker to LLD helps as well.

Tradeoff: None

## Cross-language LTO

`RUSTFLAGS=-Clinker-plugin-lto`

This is paired with the regular LTO setttings and enables Clang's LTO to optimize within the Rust binary.

Tradeoff: Requires pairing Rust and Clang versions such that they have compatible LLVM bitcode.

