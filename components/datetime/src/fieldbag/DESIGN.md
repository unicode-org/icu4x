# DateTime Field Bag Design

## Background & Motivation

In ECMA-402 (`Intl.DateTimeFormat`), formatting is driven from an options bag. The options fall into roughly three buckets:

1.  **Field-related options** (e.g., `year`, `month`, `day`, `hour`, `hour12`): These capture the developer's choices for field selection and widths.
2.  **Preference-related options** (e.g., `numberingSystem`, `hourCycle`, `calendar`): These represent user-specific formatting preferences.
3.  **Locale matching options** (e.g., `localeMatcher`): These are handled during locale negotiation beforehand and do not reach the formatter.

Bucket (2) is handled with `DateTimeFormatterPreferences`, and bucket (3) is handled in the data load layer. `icu_datetime::fieldbag` handles bucket (1) and provides interop with UTS #35 skeleton strings and ICU4X semantic skeletons (`FieldSetBuilder` and `CompositeFieldSet`).

### Goals

*   Provide a human-readable, machine-parseable representation of datetime field-related options.
*   Keep the type focused on fields and field widths (plus explicit 12-hour vs. 24-hour clock choice `hour_kind`), not on locale or numbering-system preferences.
*   Make round-tripping through a UTS 35 skeleton string lossless for the supported subset.
*   Make conversion to `FieldSetBuilder` and `CompositeFieldSet` possible even when it is not exact.

### Non-Goals

*   Stock pattern presets (namely, the `dateStyle` and `timeStyle` presets: `Full`, `Long`, `Medium`, and `Short`), as opposed to individual field widths (like `month: "long"` or `weekday: "short"`).
*   Locale negotiation (such as the ECMA-402 `localeMatcher` option), which is not handled by ICU4X.
*   A replacement for the existing `fieldsets` API.
*   A perfect round-trip between the bag and `FieldSetBuilder`.

## Architecture & API Surface

The `icu_datetime::fieldbag` module centers around the `DateTimeFieldBag` struct across 4 data representations:

1.  **`DateTimeFieldBag`** (Raw Request): Captures fine-grained field presence and width choices from the developer.
    *   *Example:* A request for "wide month and two-digit year", represented as `{ year: Some(Year::TwoDigit), month: Some(Month::Long), .. }`.
2.  **UTS 35 Skeleton String** (Interchange Format): The lossless wire format for the raw request (e.g., `"yyMMMM"`). Because `Writeable` and `FromStr` already serialize and parse compact skeleton strings, `DateTimeFieldBag` intentionally omits `serde` implementations (#8722).
3.  **`FieldSetBuilder`** (The Bridge): Maps a `DateTimeFieldBag` to the closest matching ICU4X semantic skeleton category and options, collapsing widths or filling intermediate fields if necessary.
4.  **`CompositeFieldSet`** (Resolved Category): The concrete runtime enum wrapping the resolved category (e.g., `CompositeFieldSet::CalendarPeriod(CalendarPeriodFieldSet::YM(..))`).

### Public API

*   `DateTimeFieldBag` (`#[non_exhaustive]`, derives `Debug, Copy, Clone, PartialEq, Eq, Hash, Default`; 12 single-byte optional fields, `<= 16` bytes so conversion methods take `self` by value, #8719, #8723):
    *   `pub const fn new() -> Self` (creates an all-`None` bag in `const` contexts, #8723)
    *   `pub fn try_from_skeleton(skeleton: &str) -> Result<Self, DateTimeFieldBagParseError>` (strict UTS #35 skeleton parser)
    *   `pub fn from_skeleton(skeleton: &str) -> Self` (lenient parser that recovers from non-canonical symbols, invalid widths, unknown fields, and pattern literals, #8720)
    *   `pub fn to_field_set_builder(self) -> FieldSetBuilder` (best-effort conversion to a builder)
    *   `pub fn to_composite_field_set(self) -> CompositeFieldSet` (infallible conversion to a composite field set)
    *   `pub fn from_field_set_builder(builder: &FieldSetBuilder) -> Self` (best-effort reconstruction)
*   `impl writeable::Writeable for DateTimeFieldBag` (and `Display` via `impl_display_with_writeable!`)
*   `impl core::str::FromStr for DateTimeFieldBag` (delegates to `try_from_skeleton`)

### Data Model

`DateTimeFieldBag` is a flat struct of 12 optional field enums (`fieldbag::field::*`):

- `era: Option<Era>` (`Short`, `Long`, `Narrow`)
- `year: Option<Year>` (`Numeric`, `TwoDigit`)
- `month: Option<Month>` (`Numeric`, `TwoDigit`, `Short`, `Long`, `Narrow`)
- `day: Option<Day>` (`Numeric`, `TwoDigit`)
- `weekday: Option<Weekday>` (`Short`, `Long`, `Narrow`)
- `day_period: Option<DayPeriod>` (`FlexibleShort`, `FlexibleLong`, `FlexibleNarrow`)
- `hour_kind: Option<HourKind>` (`Clock12`, `Clock24`, corresponding to ECMA-402 `hour12`)
- `hour: Option<Hour>` (`Numeric`, `TwoDigit`)
- `minute: Option<Minute>` (`Numeric`, `TwoDigit`)
- `second: Option<Second>` (`Numeric`, `TwoDigit`)
- `fractional_second_digits: Option<FractionalSecondDigits>` (`F1`, `F2`, `F3`)
- `time_zone_name: Option<TimeZoneName>` (`ShortSpecific`, `LongSpecific`, `ShortOffset`, `LongOffset`, `ShortGeneric`, `LongGeneric`, plus ICU4X/UTS #35 `Location` and `ExemplarCity`, #8715)

The bag does not carry numbering-system or calendar preferences (which belong on `DateTimeFormatterPreferences`). How `hour_kind` is plumbed to and from formatters is tracked in #8714.

## String Serialization (UTS 35 Skeletons)

### Canonical Mappings

| Bag concept | Canonical skeleton form |
|---|---|
| era short / long / narrow | `G` / `GGGG` / `GGGGG` |
| year numeric / two-digit | `y` / `yy` |
| month numeric / two-digit | `M` / `MM` |
| month short / long / narrow | `MMM` / `MMMM` / `MMMMM` |
| day numeric / two-digit | `d` / `dd` |
| weekday short / long / narrow | `E` / `EEEE` / `EEEEE` |
| flexible day period short / long / narrow | `B` / `BBBB` / `BBBBB` |
| hour (locale clock) numeric / two-digit | `j` / `jj` |
| hour (12-hour `Clock12`) numeric / two-digit | `h` / `hh` |
| hour (24-hour `Clock24`) numeric / two-digit | `H` / `HH` |
| minute numeric / two-digit | `m` / `mm` |
| second numeric / two-digit | `s` / `ss` |
| fractional second digits (1–3) | `S` / `SS` / `SSS` |
| time zone specific short / long | `z` / `zzzz` |
| time zone localized offset short / long | `O` / `OOOO` |
| time zone generic short / long | `v` / `vvvv` |
| time zone exemplar city / location | `VVV` / `VVVV` |

### Strict vs. Lenient Parsing

*   `try_from_skeleton` rejects duplicate fields (`DuplicateField`), literals (`UnexpectedLiteral`), unclosed quotes (`SyntaxError`), non-canonical field lengths (`InvalidFieldLength`), non-canonical stand-alone/variant symbols such as `L`→`M`, `c`/`e`→`E`, `K`→`h`, `k`→`H`, `C`/`J`→`j`, `Z`/`x`/`X`→`O` (`NonCanonicalField`), and unsupported fields (`UnknownField`).
*   `from_skeleton` applies the same normalization and width clamping while ignoring errors, allowing callers to ingest pattern strings or external skeletons best-effort.
*   When `hour` is `None`, standalone `hour_kind` is omitted during UTS #35 skeleton serialization because UTS #35 encodes clock kind on the hour symbol (`j` vs. `h` vs. `H`). Further day-period skeleton handling (`a`, `b`, `C`) is tracked in #8716.

## Conversion Between `DateTimeFieldBag` and `FieldSetBuilder`

### Forward Conversion (`to_field_set_builder` / `to_composite_field_set`)

Conversion from `DateTimeFieldBag` to `FieldSetBuilder` is best-effort and lossy:

*   **Empty Bags (#8717):** An all-`None` bag is an incomplete intermediate state; both `to_field_set_builder` and `to_composite_field_set` default empty bags (and bags containing only standalone `hour_kind` without `hour`) to `DateFields::YMD`, matching ECMA-402 `Intl.DateTimeFormat`'s default when no component options are given.
*   **Implied Intermediate Fields:** Gaps in requested date or time fields are filled to form a supported semantic skeleton (e.g., `y` + `d` → `DateFields::YMD`, standalone `s` → `TimePrecision::Second`). Calendar periods (`Y`, `M`, `YM`) combined with time or time zone promote to `DateFields::YMD` (#8531).
*   **Field-Specific Widths (#8718):** Per-field `Long` / `Short` / `Narrow` widths collapse into a single `options::Length` (`Narrow` currently maps to `Length::Medium`; full narrow and mixed-width support is tracked under #8718 / CLDR-19550).
*   **Alignment/Padding:** Two-digit numeric widths on `month`, `day`, or `hour` map to `Alignment::Column`.
*   **Day Period:** Standalone `day_period` without `hour` promotes to `TimePrecision::Hour` (#487).
*   **`YearStyle` Resolution:**
    *   `era: Some(_)` → `YearStyle::WithEra`
    *   `era: None`, `year: Some(Year::Numeric)` → `YearStyle::Full`
    *   `era: None`, `year: Some(Year::TwoDigit)` → `YearStyle::Auto`

### Reverse Conversion (`from_field_set_builder`)

Conversion from `FieldSetBuilder` back to `DateTimeFieldBag` expands category-level builder options into a canonical representative bag:

*   `Length` expands into concrete `month` and `weekday` widths.
*   `Alignment::Column` selects two-digit numeric widths (`MM`, `dd`, `jj`, `mm`, `ss`).
*   `YearStyle` maps back to `year` and `era` (`WithEra` → `Year::Numeric` + `Era::Short`; `Full` / `NoEra` → `Year::Numeric`; `Auto` → `Year::TwoDigit`).
*   `ZoneStyle` maps 1-to-1 with all 8 `TimeZoneName` variants (#8715); note that `Location` and `ExemplarCity` have no ECMA-402 `timeZoneName` equivalent and must be omitted or approximated when converting a `DateTimeFieldBag` to an ECMA-402 options bag.
*   `day_period` (#487) and `hour_kind` (#8714) are currently left as `None`.

## Open Follow-Ups for 2.5 Graduation

*   **#8714:** Plumbing `hour_kind` (`HourCycle`) between `DateTimeFieldBag`, `FieldSetBuilder` / `fieldsets::T`, and `DateTimeFormatterPreferences`.
*   **#8716:** Handling `a`, `b`, and `C` day-period symbols and fallback resolution across CLDR `timeData`, `-u-hc`, and field bags.
*   **#8718:** Evaluating `Narrow` collapsing and mixed per-field widths (CLDR-19550).
*   **#8721:** Exposing `from_skeleton` constructors on Diplomat FFI datetime formatters.
