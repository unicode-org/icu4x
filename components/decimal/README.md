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
use writeable::assert_writeable_eq;

let fdf = FixedDecimalFormatter::try_new_unstable(
    &icu_testdata::unstable(),
    &locale!("bn").into(),
    Default::default(),
)
.expect("Data should load successfully");

let fixed_decimal = 1000007.into();

assert_writeable_eq!(fdf.format(&fixed_decimal), "১০,০০,০০৭");
```

### Format a number with digits after the decimal separator

```rust
use fixed_decimal::FixedDecimal;
use icu::decimal::FixedDecimalFormatter;
use icu::locid::Locale;
use writeable::assert_writeable_eq;

let fdf = FixedDecimalFormatter::try_new_unstable(
    &icu_testdata::unstable(),
    &Locale::UND.into(),
    Default::default(),
)
.expect("Data should load successfully");

let fixed_decimal = FixedDecimal::from(200050).multiplied_pow10(-2);

assert_writeable_eq!(fdf.format(&fixed_decimal), "2,000.50");
```

#### Format a number using an alternative numbering system

Numbering systems specified in the `-u-nu` subtag will be followed as long as the locale has
symbols for that numbering system.

```rust
use icu::decimal::FixedDecimalFormatter;
use icu::locid::locale;
use writeable::assert_writeable_eq;

let fdf = FixedDecimalFormatter::try_new_unstable(
    &icu_testdata::unstable(),
    &locale!("th-u-nu-thai").into(),
    Default::default(),
)
.expect("Data should load successfully");

let fixed_decimal = 1000007.into();

assert_writeable_eq!(fdf.format(&fixed_decimal), "๑,๐๐๐,๐๐๗");
```

[`FixedDecimalFormatter`]: FixedDecimalFormatter

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
