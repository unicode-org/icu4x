# icu_list [![crates.io](https://img.shields.io/crates/v/icu_list)](https://crates.io/crates/icu_list)

[`icu_list`](crate) provides the [`ListFormatter`] which renders sequences of [`Writeable`](
writeable::Writeable)s as lists in a locale-sensitive way.

## Examples

### Formatting *and* lists in Spanish

```rust
#
let list_formatter = ListFormatter::try_new_and(
    locale!("es"),
    &icu_testdata::get_provider(),
    ListStyle::Wide,
)
.expect("Data should load successfully");

assert_writeable_eq!(
    list_formatter.format(["España", "Suiza"].iter()),
    "España y Suiza",
);

// The Spanish 'y' sometimes becomes an 'e':
assert_writeable_eq!(
    list_formatter.format(["España", "Suiza", "Italia"].iter()),
    "España, Suiza e Italia",
);
```

### Formatting *or* lists in Thai

```rust
#
let list_formatter = ListFormatter::try_new_or(
    locale!("th"),
    &icu_testdata::get_provider(),
    ListStyle::Short,
)
.expect("Data should load successfully");

// We can use any Writeables as inputs
assert_writeable_eq!(
    list_formatter.format(1..=3),
    "1, 2 หรือ 3",
);
```

### Formatting unit lists in English

```rust
#
let list_formatter = ListFormatter::try_new_unit(
    locale!("en"),
    &icu_testdata::get_provider(),
    ListStyle::Wide,
)
.expect("Data should load successfully");

assert_writeable_eq!(
    list_formatter.format(["1ft", "2in"].iter()),
    "1ft, 2in",
);
```

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
