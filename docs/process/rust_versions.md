# Rust version policy

Currently ICU4X pins a stable Rust version [in the `rust-toolchain` file](https://github.com/unicode-org/icu4x/blob/main/rust-toolchain), and additionally uses specific nightlies for [WASM tests](https://github.com/unicode-org/icu4x/blob/1f4a9505f21a6d5c9bb4833e0cf3fe969f734c54/Makefile.toml#L158) and [coverage](https://github.com/unicode-org/icu4x/blob/1f4a9505f21a6d5c9bb4833e0cf3fe969f734c54/.github/workflows/build-test.yml#L332).

ICU4X currently does not have an minimum supported Rust version policy. It is acceptable to update these Rust versions as necessary as a part of other PRs.

At 1.0 we will likely start following some kind of MSRV policy.
