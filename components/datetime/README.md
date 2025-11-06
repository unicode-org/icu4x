# icu_datetime [![crates.io](https://img.shields.io/crates/v/icu_datetime)](https://crates.io/crates/icu_datetime)

<!-- cargo-rdme start -->

Localized formatting of dates, times, and time zones.

This module is published as its own crate ([`icu_datetime`](https://docs.rs/icu_datetime/latest/icu_datetime/))
and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.

ICU4X datetime formatting follows the Unicode UTS 35 standard for [Semantic Skeletons](https://unicode.org/reports/tr35/tr35-dates.html#Semantic_Skeletons).
First you choose a _field set_, then you configure the formatting _options_ to your desired context.

1. Field Sets: [`icu::datetime::fieldsets`](fieldsets)
2. Options: [`icu::datetime::options`](options)

ICU4X supports formatting in over one dozen _calendar systems_, including Gregorian, Buddhist,
Hijri, and more. The calendar system is usually derived from the locale, but it can also be
specified explicitly.

The main formatter in this crate is [`DateTimeFormatter`], which supports all field sets,
options, and calendar systems. Additional formatter types are available to developers in
resource-constrained environments.

The formatters accept input types from the [`calendar`](icu_calendar) and
[`timezone`](icu_time) crates (Also reexported from the [`input`] module of this crate):

1. [`Date`](icu_calendar::Date)
2. [`DateTime`](icu_time::DateTime)
3. [`Time`](icu_time::Time)
4. [`UtcOffset`](icu_time::zone::UtcOffset)
5. [`TimeZoneInfo`](icu_time::TimeZoneInfo)
6. [`ZonedDateTime`](icu_time::ZonedDateTime)

Not all inputs are valid for all field sets.

## Examples

```rust
use icu::datetime::fieldsets;
use icu::datetime::input::Date;
use icu::datetime::input::{DateTime, Time};
use icu::datetime::DateTimeFormatter;
use icu::locale::{locale, Locale};
use writeable::assert_writeable_eq;

// Field set for year, month, day, hour, and minute with a medium length:
let field_set_with_options = fieldsets::YMD::medium().with_time_hm();

// Create a formatter for Argentinian Spanish:
let locale = locale!("es-AR");
let dtf = DateTimeFormatter::try_new(locale.into(), field_set_with_options)
    .unwrap();

// Format something:
let datetime = DateTime {
    date: Date::try_new_iso(2025, 1, 15).unwrap(),
    time: Time::try_new(16, 9, 35, 0).unwrap(),
};
let formatted_date = dtf.format(&datetime);

assert_writeable_eq!(formatted_date, "15 de ene de 2025, 4:09 p. m.");
```

## Binary Size Considerations

### Avoid linking unnecessary field sets data

There are two APIs for fieldsets:
* "static" field sets, like [`fieldsets::YMD`], where each field set is a *type*.
* "dynamic" field sets, like [`fieldsets::enums::CompositeFieldSet`], where each field set is a *value*.

While dynamic fields sets may offer a more powerful API, using them in constructors links data for all
possible values, i.e. all patterns, that the dynamic field set can represent, even if they are
unreachable in code.

Static field sets on the other hand leverage the type system to let the compiler drop unneeded data.

#### Example

```rust
use icu::datetime::DateTimeFormatter;
use icu::datetime::fieldsets::YMD;
use icu::datetime::fieldsets::enums::{CompositeFieldSet, DateFieldSet};

// This constructor only links data required for YMD
let a: DateTimeFormatter<YMD> =
    DateTimeFormatter::try_new(Default::default(), YMD::medium()).unwrap();

// This constructor links data for *all possible field sets*, even though we only use YMD
let b: DateTimeFormatter<CompositeFieldSet> =
    DateTimeFormatter::try_new(Default::default(), CompositeFieldSet::Date(DateFieldSet::YMD(YMD::medium()))).unwrap();

// If a DateTimeFormatter<CompositeFieldSet> is required, cast after construction instead:
let c: DateTimeFormatter<CompositeFieldSet> =  a.cast_into_fset::<CompositeFieldSet>();
```

### Avoid linking unnecessary calendar data

All field sets that contain dates use different data for each calendar system when used with [`DateTimeFormatter`].
This is good i18n practice, as in general the calendar system should be derived from the user locale,
not fixed in code. However, there are legitimate use cases where only one calendar system is supported,
in which case [`DateTimeFormatter`] would link unused data. In this case [`FixedCalendarDateTimeFormatter`]
can be used, which is generic in a calendar type and only links the data for that calendar.

Using [`FixedCalendarDateTimeFormatter`] also avoids linking code that converts inputs to the user's calendar.
For field sets that don't contain dates, this can also be achieved using [`NoCalendarFormatter`].

<!-- cargo-rdme end -->

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
