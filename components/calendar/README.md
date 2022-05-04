# icu_calendar [![crates.io](https://img.shields.io/crates/v/icu_calendar)](https://crates.io/crates/icu_calendar)

The `icu_calendar` crate contains the core types used by ICU4X for dealing
with dates, times, and custom calendars.

The [`types`] module has a lot of common types for dealing with dates and times.

[`Calendar`] is a trait that allows one to define custom calendars, and [`Date`]
can represent dates for arbitrary calendars.

The [`iso`] and [`gregorian`] modules contain implementations for the ISO and
Gregorian calendars respectively.

Some of the algorithms implemented here are based on
Dershowitz, Nachum, and Edward M. Reingold. _Calendrical calculations_. Cambridge University Press, 2008.
with associated Lisp code found at <https://github.com/EdReingold/calendar-code2>.

## Examples

Examples of date manipulation using `Date` object. `Date` objects are useful
for working with dates, encompassing information about the day, month, year,
as well as the calendar type.

```rust
use icu_calendar::{Date,
                   DateDuration,
                   DateDurationUnit,
                   types::IsoWeekday};

// Creating ISO date: 1992-09-02.
let mut date_iso = Date::new_iso_date_from_integers(1992, 9, 2).unwrap();
assert_eq!(date_iso.day_of_week(), IsoWeekday::Wednesday);
assert_eq!(date_iso.year().number, 1992);
assert_eq!(date_iso.month().number, 9);
assert_eq!(date_iso.day_of_month().0, 2);

// Answering questions about days in month and year.
assert_eq!(date_iso.days_in_year(), 366);
assert_eq!(date_iso.days_in_month(), 30);

// Advancing date in-place by 1 year, 2 months, 3 weeks, 4 days.
date_iso.add(DateDuration::new(1, 2, 3, 4));
assert_eq!(date_iso.year().number, 1993);
assert_eq!(date_iso.month().number, 11);
assert_eq!(date_iso.day_of_month().0, 27);

// Reverse date advancement.
date_iso.add(DateDuration::new(-1, -2, -3, -4));
assert_eq!(date_iso.year().number, 1992);
assert_eq!(date_iso.month().number, 9);
assert_eq!(date_iso.day_of_month().0, 2);

// Creating ISO date: 2022-01-30.
let newer_date_iso = Date::new_iso_date_from_integers(2022, 1, 30).unwrap();

// Comparing dates: 2022-01-30 and 1992-09-02.
let duration = newer_date_iso.until(&date_iso, DateDurationUnit::Years, DateDurationUnit::Days);
assert_eq!(duration.years, 30);
assert_eq!(duration.months, -8);
assert_eq!(duration.days, 28);

// Create new date with date advancement. Reassign to new variable.
let mutated_date_iso = date_iso.added(DateDuration::new(1, 2, 3, 4));
assert_eq!(mutated_date_iso.year().number, 1993);
assert_eq!(mutated_date_iso.month().number, 11);
assert_eq!(mutated_date_iso.day_of_month().0, 27);
```

Example of converting an ISO date across Indian and Buddhist calendars.

```rust
use icu_calendar::{Date,
                   buddhist::Buddhist,
                   indian::Indian};

// Creating ISO date: 1992-09-02.
let mut date_iso = Date::new_iso_date_from_integers(1992, 9, 2).unwrap();
assert_eq!(date_iso.year().number, 1992);
assert_eq!(date_iso.month().number, 9);
assert_eq!(date_iso.day_of_month().0, 2);

// Conversion into Indian calendar: 1914-08-02.
let date_indian = date_iso.to_calendar(Indian);
assert_eq!(date_indian.year().number, 1914);
assert_eq!(date_indian.month().number, 8);
assert_eq!(date_indian.day_of_month().0, 30);

// Conversion into Buddhist calendar: 2535-09-02.
let date_buddhist = date_iso.to_calendar(Buddhist);
assert_eq!(date_buddhist.year().number, 2535);
assert_eq!(date_buddhist.month().number, 9);
assert_eq!(date_buddhist.day_of_month().0, 2);
```

Example using `DateTime` object. Similar to `Date` objects, `DateTime` objects
contain an accessible `Date` object containing information about the day, month,
year, and calendar type. Additionally, `DateTime` objects contain an accessible
`Time` object, including granularity of hour, minute, second, and nanosecond.

```rust
use icu_calendar::{DateTime,
                   DateDuration,
                   types::IsoWeekday,
                   types::IsoHour,
                   types::IsoMinute,
                   types::IsoSecond,
                   types::NanoSecond,
                   types::Time};

// Creating ISO date: 1992-09-02 8:59
let mut datetime_iso = DateTime::new_iso_datetime_from_integers(1992, 9, 2, 8, 59, 0).unwrap();
assert_eq!(datetime_iso.date.day_of_week(), IsoWeekday::Wednesday);
assert_eq!(datetime_iso.date.year().number, 1992);
assert_eq!(datetime_iso.date.month().number, 9);
assert_eq!(datetime_iso.date.day_of_month().0, 2);
assert_eq!(datetime_iso.time.hour, IsoHour::new_unchecked(8));
assert_eq!(datetime_iso.time.minute, IsoMinute::new_unchecked(59));
assert_eq!(datetime_iso.time.second, IsoSecond::new_unchecked(0));
assert_eq!(datetime_iso.time.nanosecond, NanoSecond::new_unchecked(0));

// Advancing date by 1 year, 2 months, 3 weeks, 4 days.
datetime_iso.date.add(DateDuration::new(1, 2, 3, 4));
// New time of 14:30
datetime_iso.time = Time::try_new(14, 30, 0, 0).unwrap();

assert_eq!(datetime_iso.date.year().number, 1993);
assert_eq!(datetime_iso.date.month().number, 11);
assert_eq!(datetime_iso.date.day_of_month().0, 27);
assert_eq!(datetime_iso.time.hour, IsoHour::new_unchecked(14));
assert_eq!(datetime_iso.time.minute, IsoMinute::new_unchecked(30));
assert_eq!(datetime_iso.time.second, IsoSecond::new_unchecked(0));
assert_eq!(datetime_iso.time.nanosecond, NanoSecond::new_unchecked(0));
```
[`ICU4X`]: ../icu/index.html

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
