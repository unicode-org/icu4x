Crate Features and Dependencies
===============================

This page is part of the [ICU4X Style Guide](README.md). Rules are **required** or **suggested**; see the [Preamble](README.md#preamble).

## Crate Features

### Use no_std :: suggested

Main issues: [#77](https://github.com/unicode-org/icu4x/issues/77), [#151](https://github.com/unicode-org/icu4x/issues/151)

Most ICU4X code will not work in [no_std](https://rust-embedded.github.io/book/intro/no-std.html), since memory allocation is very often required to handle edge cases.  Even our most fundamental type, Locale, requires memory allocation.

However, when designing traits and interfaces, we should make them `no_std`-friendly, such that we can more easily expand in this direction more easily in the future.

### When to add crate [features][features] :: suggested

When adding enhancements to an ICU4X component, introduce features as a way for
the end user to control the code size of their compilation as follows:

1. If the enhancement adds a new crate dependency, it should be behind a
   feature.
2. If the enhancement contains code that is not considered best practice, for
   example if it is mainly used for debugging diagnostics or development, then
   it should be behind a feature.

[features]: https://doc.rust-lang.org/cargo/reference/features.html

## Crate Dependencies

### Avoid heavy dependencies :: suggested

Code size is an important factor for portability.  One of the easiest ways to accidentally bloat your code size is to pull in a heavy dependency.

When possible, write your code in such a way as to reduce dependencies, especially dependencies on heavier libraries.  If you need to add a dependency, consider putting it behind a feature flag.

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
