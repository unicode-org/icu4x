# icu_timezone [![crates.io](https://img.shields.io/crates/v/icu_timezone)](https://crates.io/crates/icu_timezone)

<!-- cargo-rdme start -->

Types for resolving and manipulating time zones.

## Fields

In ICU4X, a [formattable time zone](CustomTimeZone) consists of four different fields:

1. The offset from GMT
2. The time zone ID
3. The metazone ID
4. The zone variant (standard or daylight time)

### GMT Offset

The GMT offset precisely states the time difference between the time zone in question and
Greenwich Mean Time (GMT) or Coordinated Universal Time (UTC).

In localized strings, it is often rendered as "GMT-6", meaning 6 hours less than GMT.

### Time Zone

The time zone ID corresponds to a time zone from the time zone database. The time zone ID
usually corresponds to the largest city in the time zone.

There are two mostly-interchangeable standards for time zone IDs:

1. IANA time zone IDs, like `"America/Chicago"`
2. BCP-47 time zone IDs, like `"uschi"`

ICU4X uses BCP-47 time zone IDs for all of its APIs.

### Metazone

A metazone is a collection of multiple time zones that share the same localized formatting
at a particular date and time.

For example, "America/Chicago" and "America/Indiana/Knox" both map to US Central Time, or
`"America_Central"`.

The mapping from time zone to metazone depends on the date. For example, from 1991 to 2006,
"America/Indiana/Knox" mapped to US Eastern Time instead of US Central Time.

As with time zone IDs, there are two interchangeable forms:

1. Long form, like `"America_Central"`
2. Short form compatible with BCP-47, like `"amce"`

ICU4X uses the short form.

Note: in ICU4X, "metazone" is one word and "time zone" is two words, except for this crate
and module name, where "timezone" is used with no separators.

### Zone Variant

Many metazones use different names and offsets in the summer than in the winter. In ICU4X,
this is called the _zone variant_. There are two zone variants:

1. `"dt"` = daylight or summer time
2. `"st"` = standard or winter time

Note: It is optional (not required) to set the zone variant when constructing a
[`CustomTimeZone`]. Therefore, the list of possible variants does not include a generic variant
to represent the lack of a preference.

## Calculations

In date/time processing, normally only a subset of information is available, and the other
fields must be computed from it.

The following calculations are currently supported or will be supported:

1. Time Zone + Local DateTime → Meta Zone ([`MetazoneCalculator`])
2. Time Zone + Absolute Time → Offset + Zone Variant (not yet supported)

## Examples

Create a time zone for which the offset and time zone ID are already known, and calculate
the metazone based on a certain local datetime:

```rust
use icu_calendar::DateTime;
use icu_timezone::CustomTimeZone;
use icu_timezone::GmtOffset;
use icu_timezone::MetazoneCalculator;
use tinystr::TinyAsciiStr;

// Create a time zone for America/Chicago at GMT-6:
let mut time_zone = CustomTimeZone::new_empty();
time_zone.gmt_offset = "-0600".parse::<GmtOffset>().ok();
time_zone.time_zone_id =
    "uschi".parse::<TinyAsciiStr<8>>().ok().map(Into::into);

// Compute the metazone at January 1, 2022:
let mzc = MetazoneCalculator::new();
let datetime = DateTime::try_new_iso_datetime(2022, 1, 1, 0, 0, 0).unwrap();
time_zone.maybe_calculate_metazone(&mzc, &datetime);

assert_eq!("amce", time_zone.metazone_id.unwrap().0.as_str());
```

<!-- cargo-rdme end -->

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
