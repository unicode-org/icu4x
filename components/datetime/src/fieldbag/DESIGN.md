# DateTime Field Bag Design

## Background & Motivation

In ECMA-402 (`Intl.DateTimeFormat`), formatting is driven from an options bag. The options fall into roughly three buckets:

1.  **Field-related options** (e.g., `year`, `month`, `day`, `hour`, `hour12`): These capture the developer's choices for field selection and widths.
2.  **Preference-related options** (e.g., `numberingSystem`, `hourCycle`, `calendar`): These represent user-specific formatting preferences.
3.  **Locale matching options** (e.g., `localeMatcher`): These are handled during locale negotiation beforehand and do not reach the formatter.

Bucket (2) is handled with `DateTimeFormatterPreferences`, and bucket (3) is handled in the data load layer. However, ICU4X did not previously offer a way to handle bucket (1).

### Goals

*   Provide a human-readable, machine-parseable representation of datetime field-related options.
*   Keep the type focused on fields and field widths, not on other formatter options.
*   Make round-tripping through a UTS 35 skeleton string lossless for the supported subset.
*   Make conversion to `FieldSetBuilder` possible even when it is not exact.

### Non-Goals

*   Stock pattern presets (namely, the `dateStyle` and `timeStyle` presets: `Full`, `Long`, `Medium`, and `Short`), as opposed to individual field widths (like `month: "long"` or `weekday: "short"`).
*   Locale negotiation (such as the ECMA-402 `localeMatcher` option), which is not handled by ICU4X.
*   A replacement for the existing `fieldsets` API.
*   A perfect round-trip between the bag and `FieldSetBuilder`.

## Proposed Solution

We propose a new module, `fieldbag`, centered around the `DateTimeFieldBag` struct.

The architecture is built around 4 data representations that represent different steps in the flow of options into the ICU4X formatting pipeline:

1.  **`DateTimeFieldBag`** (Raw Request): Captures the fine-grained field presence and width choices from the developer (i.e., "field-related options").
    *   *Example:* A request for "wide month and two-digit year", represented as `{ year: TwoDigit, month: Long }`.
2.  **UTS 35 Skeleton String** (Interchange Format): The lossless wire format for the raw request (e.g., `"yyMMMM"`), except that standalone `hour_kind` without `hour` is omitted because UTS 35 encodes clock kind on the hour symbol. Because `Writeable` and `FromStr` already serialize and parse compact skeleton strings, `DateTimeFieldBag` intentionally omits `serde` implementations (#8722).
3.  **`FieldSetBuilder`** (The Bridge): A helper that takes a detailed `DateTimeFieldBag` (or its skeleton) and maps it to the closest matching ICU4X formatting category, collapsing widths if necessary.
    *   *Example:* Maps the `"yyMMMM"` request to a `CalendarPeriodFieldSet::YM` fieldset.
4.  **`CompositeFieldSet`** (Resolved Category): The concrete, optimized runtime enum that wraps the resolved category (e.g., wrapping a `CalendarPeriodFieldSet::YM`). This is a downstream choice, not the core representation of the request.

`DateTimeFieldBag` is a flat struct of optional fields, where each field represents a requested datetime component and its desired width. It acts as a clean, intermediate representation of a user's formatting request.

### Public API Surface

We provide explicit, named methods for conversion and standard traits for serialization, making the lossy nature of the conversions clear (`DateTimeFieldBag` is `Copy` and `<= 16` bytes, so conversion methods take `self` by value, #8719; derives `Hash`, #8723):

*   `impl DateTimeFieldBag`
    *   `pub const fn new() -> Self` (creates an all-`None` bag in `const` contexts, #8723)
    *   `pub fn try_from_skeleton(skeleton: &str) -> Result<Self, DateTimeFieldBagParseError>` (strict skeleton parser)
    *   `pub fn from_skeleton(skeleton: &str) -> Self` (lenient skeleton parser that recovers from non-canonical symbols/widths, literals, duplicates, and unknown fields, #8720)
    *   `pub fn to_field_set_builder(self) -> FieldSetBuilder` (lossy conversion)
    *   `pub fn to_composite_field_set(self) -> CompositeFieldSet` (infallible convenience wrapper over `to_field_set_builder`)
    *   `pub fn from_field_set_builder(builder: &FieldSetBuilder) -> Self` (best-effort reconstruction)
*   `impl writeable::Writeable for DateTimeFieldBag` (and `Display` via `impl_display_with_writeable!`)
*   `impl core::str::FromStr for DateTimeFieldBag` (convenience wrapper over `try_from_skeleton(&str)`)

### Module Shape

The module name should be `fieldbag` in flat case, matching ICU4X's existing module naming style.
This avoids the overloaded word `components`, which already means something else in ICU4X.

The type name should be `DateTimeFieldBag`.

The module should be public as `icu_datetime::fieldbag`.
It may reuse `provider::fields` internally, but it should not be a provider-only module.

### Data Model

`DateTimeFieldBag` is a struct of optional fields, not an ordered list.
Each field captures the field family and its width choice.

The bag covers ECMA-402 `Intl.DateTimeFormat` field options (plus minor ICU4X/UTS 35 extensions) that map naturally to UTS 35 skeleton syntax:

- era (`Era`)
- year (`Year`)
- month (`Month`)
- day (`Day`)
- weekday (`Weekday`)
- day period (`DayPeriod`)
- hour kind (`HourKind`: 12-hour vs. 24-hour clock, corresponding to ECMA-402 `hour12`)
- hour (`Hour`)
- minute (`Minute`)
- second (`Second`)
- subsecond (`FractionalSecondDigits`)
- time zone name style (`TimeZoneName`, including ICU4X/UTS 35 `Location` and `ExemplarCity`, #8715)

The bag does not carry:

- locale hour cycle subtag preferences (`-u-hc`; plumbing `hour_kind` to/from formatters is tracked in #8714)
- numbering system preference
- year style preference
- other formatter-level knobs that belong to `FieldSetBuilder`

## String Serialization (UTS 35 Skeletons)

The primary exact interchange format for `DateTimeFieldBag` is a string using UTS 35 classical skeleton syntax for the representable subset.

As discussed above, the string format is parsed and serialized by `DateTimeFieldBag`.

### Constraints

*   The string syntax must be canonicalized on output.
*   Strict parsing (`try_from_skeleton`) rejects unsupported syntax, duplicates, literals, non-canonical symbols/lengths, and unknown fields, returning `DateTimeFieldBagParseError`.
*   Lenient parsing (`from_skeleton`) normalizes non-canonical symbols (such as `L`→`M`, `c`/`e`→`E`, `K`→`h`, `k`→`H`, `C`/`J`→`j`, `Z`/`x`/`X`→`O`) and clamps invalid lengths while ignoring errors (#8720).
*   Explicit 12-hour and 24-hour clock choices are represented via `h`/`hh` (`Clock12`) and `H`/`HH` (`Clock24`), while locale-default clock kind uses `j`/`jj`. When `hour` is `None`, standalone `hour_kind` is omitted on output.

### Representative Mappings

| Bag concept | Skeleton form |
|---|---|
| year numeric / two-digit | `y` / `yy` |
| month numeric / two-digit | `M` / `MM` |
| month abbreviated / wide / narrow | `MMM` / `MMMM` / `MMMMM` |
| era short / wide / narrow | `G` / `GGGG` / `GGGGG` |
| weekday short / wide / narrow | `E` / `EEEE` / `EEEEE` |
| day numeric / two-digit | `d` / `dd` |
| flexible day period short / wide / narrow | `B` / `BBBB` / `BBBBB` |
| hour (locale clock) numeric / two-digit | `j` / `jj` |
| hour (12-hour / 24-hour) numeric / two-digit | `h` / `hh` / `H` / `HH` |
| minute numeric / two-digit | `m` / `mm` |
| second numeric / two-digit | `s` / `ss` |
| fractional second digits | `S` / `SS` / `SSS` |
| time zone name | `z` / `zzzz`, `O` / `OOOO`, `v` / `vvvv`, `VVV` / `VVVV` (#8715) |

### Hour and Day Period Representation

*   **Hour requests:** Serialized using `j`/`jj` when `hour_kind` is `None`, `h`/`hh` for `HourKind::Clock12`, and `H`/`HH` for `HourKind::Clock24`.
*   **Flexible day period (`DayPeriod`):** Currently serialized using `B` / `BBBB` / `BBBBB` independently of `hour`. Handling of `a`, `b`, and `C` skeleton symbols is tracked in #8716 (and underlying formatter support in #487).

**Standalone Day Period Handling:**
*   The skeleton parser accepts `B` day period symbols and parses them into a `DateTimeFieldBag` with `day_period` set even when `hour` is unset, preserving round-tripping.
*   During conversion to `FieldSetBuilder` (`to_field_set_builder`), a standalone day period is not supported by the underlying dynamic fieldsets and implies the presence of an hour field.


## Conversion from DateTimeFieldBag to FieldSetBuilder

Conversion from `DateTimeFieldBag` to `FieldSetBuilder` is best-effort and lossy. It should not fail; when there is no exact mapping, it should choose a documented representative builder state.

While this lossy conversion is compliant with the ECMA-402 specification, it may introduce web compatibility issues in cases where different fields request different lengths (e.g., a wide month but an abbreviated weekday). We plan to investigate the scope of these web-compat risks under [CLDR-19550](https://unicode-org.atlassian.net/browse/CLDR-19550) (#8718). If the risks are too high, our fallback plan is to enhance `FieldSetBuilder` (and the underlying formatting layer) to support more granular, per-field length hints, allowing it to honor mixed-width requests without collapsing them into a single coarse `Length`.

### What gets preserved

The conversion preserves the closest meaningful mapping for:
*   Overall date vs. time vs. date+time shape.
*   Common numeric widths.
*   Common text widths.
*   Supported time-zone styles.

### What gets approximated or omitted

*   **Empty Bags (#8717):** An all-`None` bag (or a bag with only standalone `hour_kind` and no `hour`) is an incomplete intermediate state and defaults to `DateFields::YMD`, matching ECMA-402 `Intl.DateTimeFormat`.
*   **Field-Specific Widths:** Narrow month and weekday widths collapse into a broader `Length` choice (#8718).
*   **Alignment/Padding:** Two-digit (padded) width choices on `month`, `day`, or `hour` map to a builder-wide `Alignment::Column` preference. *(Rationale: In the `fieldsets` model, `Alignment::Column` is the primary mechanism to support 2-digit numeric fields, as column/tabular alignment is the main driver for developers opting into 2-digit widths. This mapping correctly captures this developer intent.)*
*   **Time Precision:** Minute-only or second-only requests are promoted to a normal time fieldset with the closest `TimePrecision`, and calendar periods (`Y`, `M`, `YM`) combined with time or zone promote to `DateFields::YMD` (#8531).
*   **Day Period:** Day period requests without an hour imply the presence of an hour.

### `YearStyle` Forward Resolution

Since `FieldSetBuilder` controls year width via a combination of `Length` and `YearStyle`, while the bag controls it via direct field widths, we resolve the mapping as follows:
*   `era: None` $\rightarrow$ depends on `year`:
    *   `year: Some(Year::Numeric)` $\rightarrow$ `YearStyle::Full`.
    *   `year: Some(Year::TwoDigit)` $\rightarrow$ `YearStyle::Auto`.
*   `era: Some(...)` $\rightarrow$ `YearStyle::WithEra` regardless of the value of `year`

## Conversion from FieldSetBuilder to DateTimeFieldBag

Conversion from `FieldSetBuilder` back to `DateTimeFieldBag` is also best-effort and inherently lossy, as the builder stores category-level decisions and auxiliary options, while the bag stores field-level choices. It should not fail; the result should be a documented representative bag for the builder state.

### Compromises in this direction

*   A single builder `Length` value must be expanded into concrete field widths.
*   `Alignment::Column` in the builder implies that numeric fields in the reconstructed bag prefer 2-digit (padded) widths (`MM`, `dd`, `jj`, `mm`, `ss`) to preserve the alignment intent.
*   `TimePrecision::MinuteOptional` must become a static field bag (hour+minute), losing the input-dependent optionality.
*   Builder defaults may need to be materialized into explicit bag values.
*   All 8 `ZoneStyle` variants map 1-to-1 with `TimeZoneName` (#8715); note that `Location` and `ExemplarCity` have no ECMA-402 `timeZoneName` equivalent and must be omitted or approximated when converting to an ECMA-402 options bag.

### `YearStyle` Mapping

`YearStyle` in the builder is mapped back to a combination of `year` and `era` fields in the reconstructed `DateTimeFieldBag`. Since `YearStyle` controls both century display and era visibility, the mapping is defined as:

*   `YearStyle::WithEra` $\rightarrow$ `year: Some(Year::Numeric)`, `era: Some(Era::Short)`
*   `YearStyle::NoEra` $\rightarrow$ `year: Some(Year::Numeric)`, `era: None`
*   `YearStyle::Full` $\rightarrow$ `year: Some(Year::Numeric)`, `era: None`
*   `YearStyle::Auto` $\rightarrow$ `year: Some(Year::TwoDigit)`, `era: None`

*Note on Era Display:* For `Full` and `Auto`, the era is left unset (`None`) in the bag. We recommend linking this behavior to future improvements in the [TC39 Intl.eraDisplay proposal](https://github.com/tc39/proposal-intl-era-display) for better granularity in the future.

### Recommended rule

The reverse conversion should pick a canonical representative bag for a builder state. That keeps the behavior predictable even when multiple bag shapes could correspond to the same builder.

## Design Decisions & Alternatives Considered

### Why keep `fieldbag` separate from `fieldsets`

- It keeps the user-facing field request model small and focused.
- It avoids mixing formatter presets and field-level syntax.
- It keeps the lossless string form independent from dynamic fieldset construction.
- It prevents the module from becoming a catch-all for datetime formatting policy.

### Why convert via `FieldSetBuilder` instead of directly to/from `CompositeFieldSet`

We rejected making `CompositeFieldSet` the sole source/target of `DateTimeFieldBag` conversion (providing `to_composite_field_set` only as a convenience wrapper over `to_field_set_builder`):
- `CompositeFieldSet` is a concrete runtime formatter input, not a general field request.
- Converting to it exclusively would force the broadest dynamic shape even when a caller needs a narrower one.
- Using `FieldSetBuilder` as the intermediate bridge preserves useful state, allows for better optimization before committing to a final `CompositeFieldSet`, and avoids discarding information about the builder-style intermediate state.

### Why not model stock lengths here

Stock lengths such as `Full`, `Long`, `Medium`, and `Short` are a higher-level preset system.
They expand into fieldsets and sometimes inject non-bag policy, such as time-zone style.
They should live in a separate layer.

### Module Naming Alternatives Rejected

We considered and rejected the following names for the new module:
*   **`components`**: Already used to describe the ICU4X crate collection.
*   **`skeleton`**: Exposes an internal standard term that many users will not know (though the string form remains the wire format).
*   **`pattern`**: ICU4X already uses `pattern` for other datetime concepts.
*   **`fieldset`**: `fieldsets` already has a distinct meaning in ICU4X.
*   **`options`**: The bag is not a complete formatter options object.

## Future Work

- Plumb `hour_kind` (`HourCycle`) between `DateTimeFieldBag`, `FieldSetBuilder` / `fieldsets::T`, and `DateTimeFormatterPreferences` (#8714).
- Handle `a`, `b`, and `C` day-period skeleton symbols and fallback resolution (#8716).
- Evaluate `Narrow` collapsing and mixed per-field widths under CLDR-19550 (#8718).
- Expose `from_skeleton` constructors on Diplomat FFI datetime formatters (#8721).
- Add the stock pattern preset layer for `Full` / `Long` / `Medium` / `Short` (#8740).
- Decide whether the bag should support more UTS 35 symbols over time, such as week fields, day-of-year, or modified Julian day.
- Decide whether to support `J`, the UTS 35 input skeleton symbol for locale-preferred hour without a day-period marker. This is not part of the initial ECMA-oriented model.

## Implementation Notes

The implementation stays close to the existing ICU4X datetime architecture:

- implement and expose the bag from `icu_datetime::fieldbag`
- reuse provider field primitives only when they fit the public fieldbag contract
- use `Writeable` for string output (deriving `Display` via `impl_display_with_writeable!`)
- keep parsing logic explicit and conservative
- reuse existing `FieldSetBuilder` conversion helpers where possible
- avoid coupling the bag to locale fallback or formatter selection logic
