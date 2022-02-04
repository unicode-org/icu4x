# icu_list [![crates.io](https://img.shields.io/crates/v/icu_list)](https://crates.io/crates/icu_list)

[`icu_list`](crate) provides the [`ListFormatter`] which renders sequences of [`Writeable`](
writeable::Writeable)s as lists in a locale-sensitive way.

## Examples

### Format a list of strings in Spanish

```rust
use icu_list::{ListFormatter, markers::And, ListStyle};
use icu_locid::Locale;
use icu_locid_macros::langid;
use writeable::Writeable;

let locale: Locale = langid!("es").into();
let provider = icu_testdata::get_provider();
let list_formatter = ListFormatter::<And>::try_new(locale, &provider, ListStyle::Wide)
    .expect("Data should load successfully");

assert_eq!(
    list_formatter.format(["España", "Suiza"].iter())
        .writeable_to_string(),
    "España y Suiza"
);

// The Spanish 'y' sometimes becomes an 'e':
assert_eq!(
    list_formatter.format(["España", "Suiza", "Italia"].iter())
        .writeable_to_string(),
    "España, Suiza e Italia"
);

// We can use any Writeables as inputs:
assert_eq!(
    list_formatter.format(1..=10).writeable_to_string(),
    "1, 2, 3, 4, 5, 6, 7, 8, 9 y 10"
);
```

[`ListFormatter`]: ListFormatter

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
