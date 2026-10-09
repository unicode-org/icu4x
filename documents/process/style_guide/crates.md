Crate Features and Dependencies
===============================

This page is part of the [ICU4X Style Guide](README.md). Rules are **required** or **suggested**; see the [Preamble](README.md#preamble).

## Crate Features

### Use no_std :: suggested

Main issues: [#77](https://github.com/unicode-org/icu4x/issues/77), [#151](https://github.com/unicode-org/icu4x/issues/151)

Library crates are `no_std`: they use `core` and `alloc` instead of `std`. Allocating is fine. [principles.md](../../design/principles.md#no-standard-library-dependencies-in-the-core-library) says that the `icu` crate and all of its dependencies "should be `#[no_std]`, but may use the `alloc` crate".

**Why:** ICU4X runs in resource-constrained environments that don't have a standard library.

**❌ Don't:**

```rust
use std::collections::BTreeMap;
use std::string::String;
```

**✅ Do:** Start `lib.rs` with the [library annotations](../boilerplate.md#library-annotations), and import from `alloc`:

```rust
#![cfg_attr(not(any(test, doc)), no_std)]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
```

Add an `std` feature only for code that needs `std`, such as file I/O.

**Enforcement:** `cargo make ci-job-nostd` builds `icu_capi`, which depends on all components, for a target without `std` (`thumbv7m-none-eabi`).

### When to add crate [features][features] :: suggested

When adding enhancements to an ICU4X component, introduce features as a way for
the end user to control the code size of their compilation as follows:

1. If the enhancement adds a new crate dependency, it should be behind a
   feature.
2. If the enhancement contains code that is not considered best practice, for
   example if it is mainly used for debugging diagnostics or development, then
   it should be behind a feature.

**❌ Don't:** Make a dependency that only some users need a required one:

```toml
[dependencies]
serde = { workspace = true }
```

**✅ Do:** Make it optional, and turn it on from a feature with `dep:`. From `components/plurals/Cargo.toml`:

```toml
[dependencies]
serde = { workspace = true, features = ["derive", "alloc"], optional = true }

[features]
serde = ["dep:serde", "zerovec/serde", "icu_locale_core/serde", "icu_provider/serde", "dep:displaydoc"]
```

[features]: https://doc.rust-lang.org/cargo/reference/features.html

## Crate Dependencies

### Avoid heavy dependencies :: suggested

Code size is an important factor for portability.  One of the easiest ways to accidentally bloat your code size is to pull in a heavy dependency.

When possible, write your code in such a way as to reduce dependencies, especially dependencies on heavier libraries.  If you need to add a dependency, consider putting it behind a feature flag.

**Enforcement:** Every dependency must be on one of the allowlists in `tools/make/depcheck/src/allowlist.rs`. There are separate lists for runtime dependencies, build dependencies, and opt-in features such as `serde`. ICU4X components and utils can be added there; for other crates, get approval from the ICU4X owners (`@unicode-org/icu4x-owners`) first, as the file says. `cargo make depcheck`, part of `ci-job-tidy`, checks the lists and also fails on unused dependencies.

### Avoid `std::collections::HashMap` :: suggested

The standard library HashMap makes bloated binaries.  In order to reduce code size, consider one of these options:

1. If the keys are known ahead of time, use a `struct` or `enum_map`.
1. Otherwise, use a sorted vector.

If using a vector, please note that sorting algorithms are also bloated.  Better practice to reduce code size is to either perform sorting offline (e.g., by requiring that data returned by the data provider is already sorted), or to perform binary searches when inserting new elements.  Example:

```rust
fn insert_sorted<A>(vec: &mut Vec<A>, item: A) {
  if let Err(idx) = vec.binary_search(&item) {
    vec.insert(idx, item);
  }
}
```

**Why:** Besides code size, `std::collections::HashMap` isn't available in `no_std` crates (see [Use no_std](#use-no_std--suggested)). In data structs, use `ZeroMap` (see [Zero-copy in DataProvider structs](data_types.md#zero-copy-in-dataprovider-structs--required)).

**Enforcement:** Review only. Clippy doesn't check this.
