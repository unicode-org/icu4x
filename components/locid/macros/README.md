# icu_locid_macros [![crates.io](http://meritbadge.herokuapp.com/icu_locid_macros)](https://crates.io/crates/icu_locid_macros)

[`icu_locid_macros`](crate) is one of the ICU4X components.

This API provides convenience macros for [`icu_locid`].

## Examples

```rust
use icu_locid_macros::{language, region, langid};

let lid = langid!("EN_US");

assert_eq!(lid.language, language!("en"));
assert_eq!(lid.region, Some(region!("US")));
```

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
