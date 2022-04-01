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

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
