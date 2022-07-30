# icu_datetime [![crates.io](https://img.shields.io/crates/v/icu_datetime)](https://crates.io/crates/icu_datetime)

Formatting date and time.

This module is published as its own crate ([`icu_datetime`](https://docs.rs/icu_datetime/latest/icu_datetime/))
and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.

[`DateTimeFormatter`] is the main structure of the component. It accepts a set of arguments which
allow it to collect necessary data from the [data provider], and once instantiated, can be
used to quickly format any date and time provided.

## Examples

```rust
use icu::calendar::Gregorian;
use icu::datetime::{
    mock::parse_gregorian_from_str, options::length, DateTimeFormatter, DateTimeFormatterOptions,
};
use icu::locid::locale;

let provider = icu_testdata::get_provider();

// See the next code example for a more ergonomic example with .into().
let options = DateTimeFormatterOptions::Length(length::Bag::from_date_time_style(
    length::Date::Medium,
    length::Time::Short,
));

let dtf = DateTimeFormatter::<Gregorian>::try_new(&provider, &locale!("en").into(), &options)
    .expect("Failed to create DateTimeFormatter instance.");

let date = parse_gregorian_from_str("2020-09-12T12:35:00").expect("Failed to parse date.");

let formatted_date = dtf.format(&date);
assert_eq!(formatted_date.to_string(), "Sep 12, 2020, 12:35 PM");
```

The options can be created more ergonomically using the `Into` trait to automatically
convert a [`options::length::Bag`] into a [`DateTimeFormatterOptions::Length`].

```rust
use icu::calendar::Gregorian;
use icu::datetime::{options::length, DateTimeFormatter, DateTimeFormatterOptions};
let options =
    length::Bag::from_date_time_style(length::Date::Medium, length::Time::Short).into();

let dtf = DateTimeFormatter::<Gregorian>::try_new(&provider, &locale.into(), &options);
```

At the moment, the crate provides only options using the [`Length`] bag, but in the future,
we expect to add more ways to customize the output, like skeletons, and components.

*Notice:* Rust at the moment does not have a canonical way to represent date and time. We use
[`DateTime`] as an example of the data necessary for ICU [`DateTimeFormatter`] to work, and
[we hope to work with the community](https://github.com/unicode-org/icu4x/blob/main/docs/research/datetime.md)
to develop core date and time APIs that will work as an input for this component. [`DateTime`] additionally
has support for non-Gregorian calendars, which this module will eventually be able to format.

[data provider]: icu_provider
[`ICU4X`]: ../icu/index.html
[`Length`]: options::length
[`DateTime`]: icu_calendar::DateTime

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
