# icu_capi_gn [![crates.io](https://img.shields.io/crates/v/icu_capi_gn)](https://crates.io/crates/icu_capi_gn)

<!-- cargo-rdme start -->

This directory contains ICU4X build rules for the
[GN meta-build system](https://gn.googlesource.com/gn/).

Operations on the files in this directory can be performed by running `cargo make`
commands having the "gn" prefix from the top level, including:

- `cargo make gn-gen`: uses
  [Cargo GNaw](https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/tools/cargo-gnaw/)
  to re-generate the GN build rules from Cargo build rules.
- `cargo make gn-build`: Builds ICU4X using `gn` and `ninja`.
- `cargo make gn-run`: Runs ICU4X from the binaries built with `gn` and `ninja`.

This actual Rust crate is only a placeholder for input to Cargo GNaw.

<!-- cargo-rdme end -->

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
