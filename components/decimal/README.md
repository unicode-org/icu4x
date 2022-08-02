# icu_decimal [![crates.io](https://img.shields.io/crates/v/icu_decimal)](https://crates.io/crates/icu_decimal)

Formatting basic decimal numbers.

This module is published as its own crate ([`icu_decimal`](https://docs.rs/icu_decimal/latest/icu_decimal/))
and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.

Support for currencies, measurement units, and compact notation is planned. To track progress,
follow [icu4x#275](https://github.com/unicode-org/icu4x/issues/275).

## Examples

### Format a number with Bengali digits

```rust
use icu::decimal::FixedDecimalFormatter;
use icu::locid::locale;
use writeable::Writeable;

let provider = icu_testdata::get_provider();
let fdf = FixedDecimalFormatter::try_new(&provider, &locale!("bn").into(), Default::default())
    .expect("Data should load successfully");

let fixed_decimal = 1000007.into();
let formatted_value = fdf.format(&fixed_decimal);
let formatted_str = formatted_value.write_to_string();

assert_eq!("১০,০০,০০৭", formatted_str);
```

### Format a number with digits after the decimal separator

```rust
use fixed_decimal::FixedDecimal;
use icu::decimal::FixedDecimalFormatter;
use icu::locid::Locale;
use writeable::Writeable;

let provider = icu_testdata::get_provider();
let fdf = FixedDecimalFormatter::try_new(&provider, &Locale::UND.into(), Default::default())
    .expect("Data should load successfully");

let fixed_decimal = FixedDecimal::from(200050)
    .multiplied_pow10(-2)
    .expect("Operation is fully in range");

assert_eq!("2,000.50", fdf.format(&fixed_decimal).write_to_string());
```

#### Format a number using an alternative numbering system

Numbering systems specified in the `-u-nu` subtag will be followed as long as the locale has
symbols for that numbering system.

```rust
use icu::decimal::FixedDecimalFormatter;
use icu::locid::Locale;
use writeable::Writeable;

let provider = icu_testdata::get_provider();
let locale = "th-u-nu-thai".parse::<Locale>().unwrap();
let fdf = FixedDecimalFormatter::try_new(&provider, &locale.into(), Default::default())
    .expect("Data should load successfully");

let fixed_decimal = 1000007.into();
let formatted_value = fdf.format(&fixed_decimal);
let formatted_str = formatted_value.write_to_string();

assert_eq!("๑,๐๐๐,๐๐๗", formatted_str);
```

[`FixedDecimalFormatter`]: FixedDecimalFormatter

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
