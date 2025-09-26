Input Fields to ICU4X DateTimeFormat
====================================

When working with internationalized dates and times, it is important to display fields in the correct order, width, and calendar system for the locale. This document discusses how date and time information and data should be provided to ICU4X DateTimeFormat.

## Background

ICU4X implements date and time formatting in compliance with [UTS 35](http://unicode.org/reports/tr35/). Of particular interest is Part 4, Dates. UTS 35 Part 4 covers a broad range of operations, including:

- Date/time formatting symbols
- Calendar systems
- Time zones
- Selecting patterns
- Formatting with patterns
- Parsing dates and times

All of these pieces will inevitably be needed for an i18n-ready application. However, we feel that each piece should be modularized as much as possible, in order to avoid a monolithic DateTimeFormat.

Therefore, our goal in this document is to express a set of fields that can be used to format a date in any calendar system in any time zone and in any locale. *Resolving locales into specific patterns and input data into specific calendar systems and time zones is very important, but it is out of scope for this particular document.*

### How Patterns Work

An example pattern would be:

    yyyy.MM.dd G 'at' HH:mm:ss zzz

Given the correct data in en-US, this results in:

    1996.07.10 AD at 15:08:56 PDT

This example illustrates many of the categories of fields defined in UTS 35. The [full list](https://unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table) of fields can be found in UTS 35.

Not all patterns contain all fields. Patterns are also allowed to contain the same field multiple times, sometimes done when expressing a date in multiple calendar systems at once.

## Input Fields

### Naive Data Model

The date or time being formatted needs to expose all fields required by the pattern. Since the exact pattern is usually not known until runtime when the locale is specified, the most fail-proof way to provide data is to provide a struct with strings for every field in every width. For example:

```rust
pub struct NaiveDateTime {
    /// G..GGG (AD)
    pub era_abbr: String,

    /// GGGG (Anno Domini)
    pub era_wide: String,

    /// GGGGG (A)
    pub era_narrow: String,

    /// y (2, 20, 201, 2017, 20173)
    pub year_numeric: String,

    /// yy (02, 20, 01, 17, 73)
    pub year_2_digit: String,

    /// yyy (002, 020, 201, 2017, 20173)
    pub year_3_digit: String,

    /// yyyy (0002, 0020, 0201, 2017, 20173)
    pub year_4_digit: String,

    // ...
}
```

The big advantage of this data structure is that it is calendar system agnostic: Gregorian, Hebrew, Japanese, and Hijri dates can all be expressed in this data structure.

However, it has obvious flaws:

1. It should be up to the library to load the localized strings such as "Anno Domini" from locale data.
2. The majority of fields in this struct will get thrown away; we don't want to pre-compute them.

### First Iteration

What follows is a basic data model that is able to represent fields for any calendar system, but solve the two problems listed above.

```rust
pub struct FirstIterationDateTime {
    /// Era identifier: "ad", "ce", "rewia", ...
    /// Used for field "G.."
    pub era: TinyStr8,

    /// Year number: 2020, 5780, ...
    /// Used for field "y.."
    pub year: i64,

    // ...

    /// Month identifier: "jan", "feb", "ayar", ...
    /// Used for fields "MMM.." and "LLL.."
    pub month: TinyStr8,

    /// Month number: 1, 2, 3
    /// Used for fields "M", "MM", "L", "LL"
    pub month_number: u8,

    // ...

    /// Day Period identifier: "am", "pm"
    /// Used for field "a.."
    pub period_a: TinyStr8

    /// Extended Day Period identifier: "am", "noon", "midnight", ...
    /// Used for field "b.."
    pub period_b: TinyStr8,

    /// Flexible Day Period identifier: "night", ...
    pub period_flex: TinyStr8,

    /// Hour in h11 hour cycle: 0-11
    pub hour_h11: u8,

    /// Hour in h12 hour cycle: 1-12
    pub hour_h12: u8,

    /// Hour in h23 hour cycle: 0-23
    pub hour_h23: u8,

    /// Hour in h24 hour cycle: 1-24
    pub hour_h24: u8,

    // ...
}
```

We can simplify this further by consolidating fields in the same category.

### Caveats for each field type

#### Era

It is unclear whether the toggle switch between AD and CE should live in the input data or in the library. CLDR does not appear to provide data that maps from a locale to the preference between AD and CE. Further investigation required.

#### Year

There are several types of years in UTS 35:

1. Normal calendar year (`y`)
2. Calendar year indexed by weeks instead of days (`Y`)
3. Extended numeric year (`u`)
4. Cyclic year (`U`)
5. Related Gregorian year (`r`)

Calendrical calculations are required to convert any of these to any other, so they should all be present in the input struct. In many cases, the same data can be used in multiple fields.

#### Month

In the Gregorian calendar, there is a 1-to-1 mapping from month numbers to month names. However, this is not the case in all calendar systems. For example, in the Hebrew calendar:

| ISO-8601 Day | Hebrew Long | Hebrew Short |
|---|---|---|
| 2019-04-06 | 1 Nisan 5779 | 8/1/5779 |
| 2020-03-26 | 1 Nisan 5780 | 7/1/5780 |

The Chinese calendar has similar behavior. Therefore, the month number and month string identifier must be stored separately.

For years with leap months, the leap can be part of the identifier: `"aya"`, `"ayai"`, `"ayaii"`.

**Note:** most Hebrew patterns do not show a numeric month, even when the numeric month is requested. Further investigation should be performed to establish whether a 1-to-1 mapping between month numbers and month names can be extended into the Hebrew calendar.

#### Day Period and Hour

In the Gregorian calendar, it is possible to compute all of the following fields from one, `hour_h23`:

- `period_a`
- `hour_h11`
- `hour_h12`
- `hour_h24`

It may also be possible to generalize this mapping to the other two types of day periods.

Further investigation is required to determine whether this mapping would be sufficient for non-Gregorian calendars.

### Second Iteration

With all of the fields condensed down to their essentials, the full struct becomes (where `TinyStr8` indicates an identifier string):

```rust
pub struct SecondIterationDateTime {
    // Date Fields
    pub era: TinyStr8,
    pub year: i64,
    pub year_week: i64,
    pub year_extended: i64,
    pub year_cyclic: TinyStr8,
    pub year_related: i64,
    pub quarter: TinyStr8,
    pub month: TinyStr8,
    pub month_number: u8,
    pub week: u64,
    pub day_of_month: u64,
    pub day_of_year: u64,
    pub day_of_week_in_month: u64,
    pub julian_day: i64,
    pub weekday: TinyStr8,

    // Time Fields
    pub hour_h23: u8,
    pub minute: u8,
    pub second: u8,
    pub nanosecond: u32,
    pub milliseconds_in_day: u64,

    // Time Zone Fields
    pub zone: TinyStr8,
    pub offset_seconds: u32,
}
```

The above struct is still larger than it needs to be in most cases, harming ergonomics and performance. But, now that we have distilled the information we need, we can work on the next step:

### Trait Object

A trait object returning the above information could be implemented for each individual calendar system. For example, the Gregorian implementation would return the same value for all five types of years, whereas the Chinese implementation would return different values.

```rust
pub struct GregorianDateTime {
    year: i64,
    month: u8,
    day: u8,
    hour_h23: u8,
    minute: u8,
    second: u8,
    nanosecond: u32,
}

impl DateTimeTraitObject for GregorianDateTime {
    pub era(&self) {
        if self.year >= 0 { "ad" } else { "bc" }
    }

    pub year(&self) {
        self.year
    }

    pub year_extended(&self) {
        self.year
    }

    // ...

    pub month(&self) {
        match self.month {
            1 => tinystr!(8, "jan"),
            2 => tinystr!(8, "feb"),
            3 => tinystr!(8, "mar"),
            4 => tinystr!(8, "apr"),
            // ...
            _ => tinystr!(8, "unknown"),
        }
    }

    pub month_number(&self) {
        self.month
    }

    // ...
}
```

## Data Provider Model

With all calendar systems merged into a common set of identifiers for month and weekday names, we can model the locale data to be calendar system agnostic.

```rust
#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
pub struct MonthNamesV1 {
    // note: we'd use tuple_vec_map in production
    pub names: Map<TinyStr8, String>,
}
```

Example data:

```yaml
month_names:
  jan: January
  feb: February
  # ...
  aya: Ayar
  ayai: Ayar I
  ayaii: Ayar II
  # ...
  cha: Chaitra
  vai: Vaisakha
  # ...
```

The advantage of this data model is that the calendar system is fully abstracted away from DateTimeFormat. The disadvantage is that more data would be carried than generally required. To reduce data size, a build-time flag could be used to filter out month names from certain calendar systems as desired.
