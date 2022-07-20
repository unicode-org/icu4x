# icu_uniset [![crates.io](https://img.shields.io/crates/v/icu_uniset)](https://crates.io/crates/icu_uniset)

`icu_uniset` is a utility crate of the [`ICU4X`] project.

This API provides necessary functionality for highly efficient querying of sets of Unicode characters.

It is an implementation of the existing [ICU4C UnicodeSet API](https://unicode-org.github.io/icu-docs/apidoc/released/icu4c/classicu_1_1UnicodeSet.html).

## Architecture
ICU4X [`CodePointSet`] is split up into independent levels, with [`CodePointSet`] representing the membership/query API,
and [`CodePointSetBuilder`] representing the builder API.
is in future works.

## Examples:

### Creating a `CodePointSet`

CodePointSets are created from either serialized [`CodePointSets`](CodePointSet),
represented by [inversion lists](http://userguide.icu-project.org/strings/properties),
the [`CodePointSetBuilder`], or from the TBA Properties API.

```rust
use icu_uniset::{CodePointSet, CodePointSetBuilder};

let mut builder = CodePointSetBuilder::new();
builder.add_range(&('A'..'Z'));
let set: CodePointSet = builder.build();

assert!(set.contains('A'));
```

### Querying a `CodePointSet`

Currently, you can check if a character/range of characters exists in the [`CodePointSet`], or iterate through the characters.

```rust
use icu_uniset::{CodePointSet, CodePointSetBuilder};

let mut builder = CodePointSetBuilder::new();
builder.add_range(&('A'..'Z'));
let set: CodePointSet = builder.build();

assert!(set.contains('A'));
assert!(set.contains_range(&('A'..='C')));
assert_eq!(set.iter_chars().next(), Some('A'));
```

[`ICU4X`]: ../icu/index.html

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
