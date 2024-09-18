# icu_datetime [![crates.io](https://img.shields.io/crates/v/icu_datetime)](https://crates.io/crates/icu_datetime)

<!-- cargo-rdme start -->

Formatting date and time.

This module is published as its own crate ([`icu_datetime`](https://docs.rs/icu_datetime/latest/icu_datetime/))
and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.

[`TypedNeoFormatter`] and [`NeoFormatter`] are the main types of the component. They accepts a set of arguments which
allow it to collect necessary data from the [data provider], and once instantiated, can be
used to quickly format any date and time provided. There are variants of these types that can format greater or fewer components.

These formatters work with types from the [`calendar`] module, like [`Date`], [`DateTime`], and [`Time`],
and [`timezone::CustomTimeZone`], however other types may be used provided they implement the traits from the [`input`] module.

Each instance of a date-related formatter is associated with a particular [`Calendar`].
The "Typed" vs untyped formatter distinction is to help with this. For example, if you know at compile time that you
will only be formatting Gregorian dates, you can use [`TypedNeoFormatter<Gregorian>`](TypedNeoFormatter) and the
APIs will make sure that only Gregorian [`DateTime`]s are used with the calendar. On the other hand, if you want to be able to select
the calendar at runtime, you can use [`neo::NeoFormatter`] with the calendar specified in the locale, and use it with
[`DateTime<AnyCalendar>`](icu_calendar::DateTime) (see [`AnyCalendar`]). These formatters still require dates associated
with the appropriate calendar (though they will convert ISO dates to the calendar if provided), they just do not force the
programmer to pick the calendar at compile time.


## Examples

```rust
use icu::calendar::{DateTime, Gregorian};
use icu::datetime::neo::{TypedNeoFormatter, NeoFormatter, NeoOptions};
use icu::datetime::neo_skeleton::NeoSkeletonLength;
use icu::datetime::neo_marker::NeoYearMonthDayHourMinuteMarker;
use icu::locale::{locale, Locale};
use writeable::assert_try_writeable_eq;

// You can work with a formatter that can select the calendar at runtime:
let locale = Locale::try_from_str("en-u-ca-gregory").unwrap();
let dtf = NeoFormatter::<NeoYearMonthDayHourMinuteMarker>::try_new(
    &locale.into(),
    NeoSkeletonLength::Medium.into()
)
.expect("should successfully create NeoFormatter instance");

// Or one that selects a calendar at compile time:
let typed_dtf = TypedNeoFormatter::<Gregorian, NeoYearMonthDayHourMinuteMarker>::try_new(
    &locale!("en").into(),
    NeoSkeletonLength::Medium.into(),
)
.expect("should successfully create TypedNeoFormatter instance");

let typed_date =
    DateTime::try_new_gregorian_datetime(2020, 9, 12, 12, 34, 28).unwrap();
// prefer using ISO dates with DateTimeFormatter
let date = typed_date.to_iso();

let formatted_date = dtf.convert_and_format(&date);
let typed_formatted_date = typed_dtf.format(&typed_date);

assert_try_writeable_eq!(formatted_date, "Sep 12, 2020, 12:34 PM");
assert_try_writeable_eq!(typed_formatted_date, "Sep 12, 2020, 12:34 PM");
```

[data provider]: icu_provider
[`ICU4X`]: ../icu/index.html
[`Length`]: options::length
[`DateTime`]: calendar::{DateTime}
[`Date`]: calendar::{Date}
[`Time`]: calendar::types::{Time}
[`Calendar`]: calendar::{Calendar}
[`AnyCalendar`]: calendar::any_calendar::{AnyCalendar}
[`timezone::CustomTimeZone`]: icu::timezone::{CustomTimeZone}

<!-- cargo-rdme end -->

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
