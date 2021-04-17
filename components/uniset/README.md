# icu_uniset [![crates.io](http://meritbadge.herokuapp.com/icu_uniset)](https://crates.io/crates/icu_uniset)

`icu_uniset` is one of the [`ICU4X`] components.

This API provides necessary functionality for highly efficient querying of sets of Unicode characters.

It is an implementation of the existing [ICU4C UnicodeSet API](https://unicode-org.github.io/icu-docs/apidoc/released/icu4c/classicu_1_1UnicodeSet.html).

## Architecture
ICU4X [`UnicodeSet`] is split up into independent levels, with [`UnicodeSet`] representing the membership/query API,
and [`UnicodeSetBuilder`] representing the builder API. A [Properties API](http://userguide.icu-project.org/strings/properties)
is in future works.

## Examples:

### Creating a `UnicodeSet`

UnicodeSets are created from either serialized [`UnicodeSets`](UnicodeSet),
represented by [inversion lists](http://userguide.icu-project.org/strings/properties),
the [`UnicodeSetBuilder`], or from the TBA Properties API.

```rust
use icu::uniset::{UnicodeSet, UnicodeSetBuilder};

let mut builder = UnicodeSetBuilder::new();
builder.add_range(&('A'..'Z'));
let set: UnicodeSet = builder.build();

assert!(set.contains('A'));
```

### Querying a `UnicodeSet`

Currently, you can check if a character/range of characters exists in the [`UnicodeSet`], or iterate through the characters.

```rust
use icu::uniset::{UnicodeSet, UnicodeSetBuilder};

let mut builder = UnicodeSetBuilder::new();
builder.add_range(&('A'..'Z'));
let set: UnicodeSet = builder.build();

assert!(set.contains('A'));
assert!(set.contains_range(&('A'..='C')));
assert_eq!(set.iter_chars().next(), Some('A'));
```

[`ICU4X`]: ../icu/index.html

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
