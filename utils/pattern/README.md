# icu_pattern [![crates.io](https://img.shields.io/crates/v/icu_pattern)](https://crates.io/crates/icu_pattern)

<!-- cargo-rdme start -->

`icu_pattern` is a utility crate of the [`ICU4X`] project.

It includes a [`Pattern`] type which supports patterns with various storage backends.

The types are tightly coupled with the [`writeable`] crate.

## Examples

Parsing and interpolating with a single-placeholder pattern:

```rust
use icu_pattern::SinglePlaceholderPattern;
use writeable::assert_writeable_eq;

// Parse a pattern string:
let pattern = "Hello, {0}!"
    .parse::<SinglePlaceholderPattern<_>>()
    .unwrap();

// Interpolate into the pattern string:
assert_writeable_eq!(pattern.interpolate(["World"]), "Hello, World!");

// Introspect the serialized form of the pattern string:
assert_eq!(pattern.take_store(), "\x08Hello, !");
```

[`ICU4X`]: ../icu/index.html
[`FromStr`]: std::str::FromStr

<!-- cargo-rdme end -->

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
