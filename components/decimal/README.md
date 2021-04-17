# icu_decimal [![crates.io](http://meritbadge.herokuapp.com/icu_decimal)](https://crates.io/crates/icu_decimal)

[`icu_decimal`](crate) offers localized decimal number formatting.

Currently, [`icu_decimal`](crate) provides [`FixedDecimalFormat`], which renders basic decimal numbers
in a locale-sensitive way.

Support for currencies, measurement units, and compact notation is planned. To track progress,
follow this issue:

https://github.com/unicode-org/icu4x/issues/275

## Example

```rust
use icu_decimal::FixedDecimalFormat;
use icu_decimal::FormattedFixedDecimal;
use icu_locid::Locale;
use icu_locid_macros::langid;
use writeable::Writeable;

let locale: Locale = langid!("bn").into();
let provider = icu_testdata::get_provider();
let fdf = FixedDecimalFormat::try_new(locale, &provider, Default::default())
    .expect("Data should load successfully");

let fixed_decimal = 1000007.into();
let formatted_value = fdf.format(&fixed_decimal);
let formatted_str = formatted_value.writeable_to_string();

assert_eq!("১০,০০,০০৭", formatted_str);
```

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
