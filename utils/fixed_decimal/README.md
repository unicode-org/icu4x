# fixed_decimal [![crates.io](http://meritbadge.herokuapp.com/fixed_decimal)](https://crates.io/crates/fixed_decimal)

`fixed_decimal` is a utility crate of the [`ICU4X`] project.

It includes [`FixedDecimal`], a core API for representing numbers in a human-readable form
appropriate for formatting and plural rule selection. It is optimized for operations involving
the individual digits of a number.

## Examples

```rust
use fixed_decimal::FixedDecimal;

let dec = FixedDecimal::from(250)
    .multiplied_pow10(-2)
    .expect("Bounds are small");
assert_eq!("2.50", format!("{}", dec));

#[derive(Debug, PartialEq)]
struct MagnitudeAndDigit(i16, u8);

let digits: Vec<MagnitudeAndDigit> = dec
    .magnitude_range()
    .map(|m| MagnitudeAndDigit(m, dec.digit_at(m)))
    .collect();

assert_eq!(
    vec![
        MagnitudeAndDigit(-2, 0),
        MagnitudeAndDigit(-1, 5),
        MagnitudeAndDigit(0, 2)
    ],
    digits
);
```

[`ICU4X`]: ../icu/index.html

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
