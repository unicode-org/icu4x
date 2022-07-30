# icu_plurals [![crates.io](https://img.shields.io/crates/v/icu_plurals)](https://crates.io/crates/icu_plurals)

Determine the plural category appropriate for a given number in a given language.

This module is published as its own crate ([`icu_plural`](https://docs.rs/icu_plural/latest/icu_plural/))
and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.

For example in English language, when constructing a message
such as `{ num } items`, the user has to prepare
two variants of the message:

* `1 item`
* `0 items`, `2 items`, `5 items`, `0.5 items` etc.

The former variant is used when the placeholder variable has value `1`,
while the latter is used for all other values of the variable.

Unicode defines [`Language Plural Rules`] as a mechanism to codify those
variants and provides data and algorithms to calculate
appropriate [`Plural Category`].

## Examples

```rust
use icu::locid::locale;
use icu::plurals::{PluralCategory, PluralRuleType, PluralRules};

let provider = icu_testdata::get_provider();

let pr = PluralRules::try_new(&provider, &locale!("en").into(), PluralRuleType::Cardinal)
    .expect("Failed to construct a PluralRules struct.");

assert_eq!(pr.select(5_usize), PluralCategory::Other);
```

### Plural Rules

The crate provides the main struct [`PluralRules`] which handles selection
of the correct [`Plural Category`] for a given language and [`Plural Type`].

### Plural Category

Every number in every language belongs to a certain [`Plural Category`].
For example, Polish language uses four:

* [`One`](PluralCategory::One): `1 miesiąc`
* [`Few`](PluralCategory::Few): `2 miesiące`
* [`Many`](PluralCategory::Many): `5 miesięcy`
* [`Other`](PluralCategory::Other): `1.5 miesiąca`

### Plural Rule Type

Plural rules depend on the use case. This crate supports two types of plural rules:

* [`Cardinal`](PluralRuleType::Cardinal): `3 doors`, `1 month`, `10 dollars`
* [`Ordinal`](PluralRuleType::Ordinal): `1st place`, `10th day`, `11th floor`

[`ICU4X`]: ../icu/index.html
[`Plural Type`]: PluralRuleType
[`Plural Category`]: PluralCategory
[`Language Plural Rules`]: https://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules
[`CLDR`]: http://cldr.unicode.org/

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
