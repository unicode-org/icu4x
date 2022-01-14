# icu_list [![crates.io](https://img.shields.io/crates/v/icu_list)](https://crates.io/crates/icu_list)

[`icu_list`](crate) provides the [`ListFormatter`] which renders sequences of [`Writeable`](
writeable::Writeable)s as lists in a locale-sensitive way.

## Examples

### Format a list of strings in Spanish

```rust
use icu_list::{ListFormatter, ListStyle};
use icu_locid::locale;
use writeable::Writeable;

let list_formatter = ListFormatter::try_new_and(
    locale!("es"),
    &icu_testdata::get_static_provider(),
    ListStyle::Wide,
)
.expect("Data should load successfully");

assert_eq!(
    list_formatter.format(["España", "Suiza"].iter())
        .write_to_string(),
    "España y Suiza"
);

// The Spanish 'y' sometimes becomes an 'e':
assert_eq!(
    list_formatter.format(["España", "Suiza", "Italia"].iter())
        .write_to_string(),
    "España, Suiza e Italia"
);

// We can use any Writeables as inputs:
assert_eq!(
    list_formatter.format(1..=10).write_to_string(),
    "1, 2, 3, 4, 5, 6, 7, 8, 9 y 10"
);
```

[`ListFormatter`]: ListFormatter

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
