# icu_decimal [![crates.io](https://img.shields.io/crates/v/icu_decimal)](https://crates.io/crates/icu_decimal)

[`icu_decimal`](crate) offers localized decimal number formatting.

Currently, [`icu_decimal`](crate) provides [`FixedDecimalFormat`], which renders basic decimal numbers
in a locale-sensitive way.

Support for currencies, measurement units, and compact notation is planned. To track progress,
follow [icu4x#275](https://github.com/unicode-org/icu4x/issues/275).

## Examples

### Format a number with Bengali digits

```rust
use icu::decimal::FixedDecimalFormat;
use icu::locid::Locale;
use icu::locid::macros::langid;
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

### Format a number with digits after the decimal separator

```rust
use fixed_decimal::FixedDecimal;
use icu::decimal::FixedDecimalFormat;
use icu::locid::Locale;
use writeable::Writeable;

let locale = Locale::und();
let provider = icu_provider::inv::InvariantDataProvider;
let fdf = FixedDecimalFormat::try_new(locale, &provider, Default::default())
    .expect("Data should load successfully");

let fixed_decimal = FixedDecimal::from(200050)
    .multiplied_pow10(-2)
    .expect("Operation is fully in range");

assert_eq!("2,000.50", fdf.format(&fixed_decimal).writeable_to_string());
```

[`FixedDecimalFormat`]: FixedDecimalFormat

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
