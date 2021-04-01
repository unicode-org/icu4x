# icu_datetime [![crates.io](http://meritbadge.herokuapp.com/icu_datetime)](https://crates.io/crates/icu_datetime)

`icu_datetime` is one of the [`ICU4X`] components.

This API provides necessary functionality for formatting date and time to user readable textual representation.

[`DateTimeFormat`] is the main structure of the component. It accepts a set of arguments which
allow it to collect necessary data from the [`DataProvider`], and once instantiated, can be
used to quickly format any date and time provided.

## Examples

```rust
use icu_locid::Locale;
use icu_locid_macros::langid;
use icu_datetime::{DateTimeFormat, date::MockDateTime, options::length};

let provider = icu_testdata::get_provider();

let locale: Locale = langid!("en").into();

let options = length::Bag {
    date: Some(length::Date::Medium),
    time: Some(length::Time::Short),
    ..Default::default()
});

let dtf = DateTimeFormat::try_new(locale, &provider, &options)
    .expect("Failed to create DateTimeFormat instance.");


let date: MockDateTime = "2020-09-12T12:35:00".parse()
    .expect("Failed to parse date.");

let formatted_date = dtf.format(&date);
assert_eq!(formatted_date.to_string(), "Sep 12, 2020, 12:35 PM");
```

At the moment, the crate provides only options using the `Length` bag, but in the future,
we expect to add more ways to customize the output, like skeletons, and components.

*Notice:* Rust at the moment does not have a canonical way to represent date and time. We are introducing
[`MockDateTime`] as an example of the data necessary for ICU [`DateTimeFormat`] to work, and
[we hope to work with the community](https://github.com/unicode-org/icu4x/blob/main/docs/research/date_time.md)
to develop core date and time APIs that will work as an input for this component.

[`DataProvider`]: icu_provider::DataProvider
[`ICU4X`]: ../icu/index.html
[`Style`]: options::style
[`MockDateTime`]: mock::MockDateTime
