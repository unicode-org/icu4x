# icu_collator [![crates.io](https://img.shields.io/crates/v/icu_collator)](https://crates.io/crates/icu_collator)

`icu_collator` is one of the ICU4X components.

This API provides necessary functionality for comparing strings according to
language-dependent conventions.

`Collator` is the main structure of the component. It accepts a set of arguments
which allow it to collect necessary data from the data provider, and once
instantiated, can be used to compare strings.

## Development environment (on Linux) for fuzzing and generating data

These notes assume that ICU4X itself has been cloned to `$PROJECTS/icu4x`.

Clone ICU4C from https://github.com/hsivonen/icu to `$PROJECTS/icu` and switch
to the branch `icu4x-collator`.

Create a directory `$PROJECTS/localicu`

Create a directory `$PROJECTS/icu-build` and `cd` into it.

Run `../icu/icu4c/source/runConfigureICU --enable-debug Linux --prefix /opt/Projects/localicu --enable-static`

Run `make`

### Generating data



### Testing

`cargo test --features serde`

### Fuzzing

`cargo install cargo-fuzz`

Clone `rust_icu` from https://github.com/google/rust_icu to `$PROJECTS/rust_icu`.

In `$PROJECTS/icu-build` run `make install`.

`cd $PROJECTS/icu4x/experimental/collator`

Run the fuzzer until a panic:

`PKG_CONFIG_PATH="$PROJECTS/localicu/lib/pkgconfig" PATH="$PROJECTS/localicu/bin:$PATH" LD_LIBRARY_PATH="/$PROJECTS/localicu/lib" RUSTC_BOOTSTRAP=1 cargo +stable fuzz run compare_utf16`

Once there is a panic, recompile with debug symbols by adding `--dev`:

`PKG_CONFIG_PATH="$PROJECTS/localicu/lib/pkgconfig" PATH="$PROJECTS/localicu/bin:$PATH" LD_LIBRARY_PATH="$PROJECTS/localicu/lib" RUSTC_BOOTSTRAP=1 cargo +stable fuzz run --dev compare_utf16 fuzz/artifacts/compare_utf16/crash-$ARTIFACTHASH`

Record with

`LD_LIBRARY_PATH="$PROJECTS/localicu/lib" rr fuzz/target/x86_64-unknown-linux-gnu/debug/compare_utf16 -artifact_prefix=$PROJECTS/icu4x/experimental/collator/fuzz/artifacts/compare_utf16/ fuzz/artifacts/compare_utf16/crash-$ARTIFACTHASH`
