# AV003: No `std` in library crates

- **Status:** Proposed
- **Enforced by:** the compiler (`no_std` in the `lib.rs` header), Clippy
  (`alloc_instead_of_core`), and CI (`ci-job-nostd`)
- **Applies to:** library crates that the `icu` crate depends on. Not test
  code.
- **Source:** [principles.md: No standard library dependencies in the core library](../../design/principles.md#no-standard-library-dependencies-in-the-core-library),
  [boilerplate.md: Library annotations](../../process/boilerplate.md#library-annotations),
  [graduation.md](../../process/graduation.md)
- **Checked against:** ICU4X 2.3 (`main` @ `2fa9afb769`)

## ❌ Don't

```rust,ignore
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;

impl std::error::Error for MyError {}
```

## ✅ Do instead

```rust,ignore
// lib.rs, with the header from boilerplate.md:
#![cfg_attr(not(any(test, doc)), no_std)]

extern crate alloc;

use alloc::borrow::Cow;
use core::fmt;

impl core::error::Error for MyError {}
```

| Instead of | Use |
|---|---|
| `std::fmt`, `std::str`, `std::cmp`, ... | The same path in `core` |
| `std::string::String`, `std::vec::Vec`, `std::borrow::Cow`, `std::boxed::Box` | The same path in `alloc` |
| `std::collections::HashMap` (not in `alloc`) | A `struct` or a sorted `Vec` ([Avoid `std::collections::HashMap`](../../process/style_guide.md#avoid-stdcollectionshashmap--suggested)) |
| `std::error::Error` | `core::error::Error` |

Code that really needs `std` goes behind a `std` Cargo feature. From
`icu_provider` (abridged):

```rust,ignore
// provider/core/src/lib.rs
#![cfg_attr(not(any(test, doc, feature = "std")), no_std)]

// provider/core/src/error.rs
impl core::error::Error for DataError {}

#[cfg(feature = "std")]
impl From<std::io::Error> for DataError { /* ... */ }
```

## Why

- ICU4X runs "in resource-constrained environments that don't have access
  to a standard library" (principles.md). principles.md asks for `no_std` in
  "the `icu` crate and all of its direct and indirect dependencies". So one
  crate that needs `std` makes all of ICU4X need it.
- With `no_std`, the compiler checks the rule: a `std::` path does not
  compile.
- Most of `std` re-exports `core` and `alloc`, so usually only the path
  changes. `core::error::Error` exists since Rust 1.81, so error types don't
  need `std` either.

## Exceptions

- Test code. The header keeps `std` for `test` and `doc` builds.
- Code that needs `std`, such as file I/O, behind a `std` feature.
  graduation.md: "The crate should have an `std` feature if (and only if) it
  contains code that depends on `std`".
- Crates that the `icu` crate does not depend on, and that need `std` by
  design, such as `icu_provider_source`, `icu_provider_export`,
  `icu_provider_fs`, and `databake`. Their `lib.rs` has the `no_std` line
  commented out.

## Automation

- **Today:**
  - The `lib.rs` header makes the crate `no_std` outside tests and docs, so
    `std::` paths fail to compile.
  - `ci-job-nostd` builds `icu_capi` for `thumbv7m-none-eabi`, and the
    crates in `tools/noalloctest` with only `core`. A dependency that needs
    `std` fails there.
  - The workspace enables Clippy's `alloc_instead_of_core`, which flags
    `alloc::` paths for items that are in `core`.
- **Not covered:**
  - A library crate without the header. See the tidy check idea in
    [AV001](AV001_no_panics_in_library_code.md#automation).
  - `ci-job-nostd` builds `icu_capi` with `--no-default-features`, which
    turns off its components. So most components are built only for targets
    with `std`, where a dependency that needs `std` still compiles.
    TODO(verify): whether the job should turn the components on.
