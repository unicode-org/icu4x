# ICU4X Boilerplate Documentation

This file contains explanations of ICU4X-specific conventions on boilerplate (extra code/annotations) in source files.

## Library annotations

To ensure the ICU4X library crates conform to the project's style guide, every `lib.rs` file must include the standard boilerplate annotations.

These annotations enforce the following rules:

1. Configure the crate as `no_std`.
2. Enforce annotating any panicking behavior, except during tests.
3. Require every exported item to be documented, implement `Debug`, and where appropriate marked as `non_exhaustive` (_unless annotated to permit as exhaustive_).

> [!NOTE]
> While the majority of lints are configured globally via the workspace `Cargo.toml`, some [boilerplate-level overrides remain necessary][gh-issues::boilerplate-lints] due to upstream tooling limitations.

Include the following at the top of each `lib.rs` file:

```rust
// https://github.com/unicode-org/icu4x/blob/main/documents/process/boilerplate.md#library-annotations
#![no_std]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
    )
)]
#![warn(missing_docs)]

#[cfg(test)]
extern crate std;
```

By convention, `extern crate std;` should be placed immediately following the crate-level attributes. If the file also requires `extern crate alloc;`, place `extern crate std;` after the `alloc` declaration.

Not all crates are compatible with the full boilerplate. Incompatible annotations should be included but commented out (_if that is `#![no_std]`, the associated `extern crate std` lines should be excluded from the file_).

Mandating `#![no_std]` ensures that the [`core` prelude is always used instead][prelude-nostd] of the [`std` prelude][prelude-std]. This provides a consistent prelude across environments, ensuring that `std` is only explicitly referenced where required.

> [!NOTE]
> When a crate optionally supports `std`, it should be gated via a `std` feature and additionally enabled when generating documentation:
> 
> ```rust
> #[cfg(any(test, doc, feature = "std"))]
> extern crate std;
> ```
> 
> - Some crates may **only support `std`**, in which case `extern crate std;` is enforced without any conditional constraints.
> - Any crate missing this extern presently relies upon an implicit `std` prelude, but it's boilerplate should have `#![no_std]` commented out, until that crate has been migrated to explicitly import from `std`.

[gh-issues::boilerplate-lints]: https://github.com/unicode-org/icu4x/issues/5974#issuecomment-3775926163
[prelude-nostd]: https://doc.rust-lang.org/reference/names/preludes.html#the-no_std-attribute
[prelude-std]: https://doc.rust-lang.org/std/prelude/index.html
