# DateTime Field Bag Design

## Background & Motivation

In ECMA-402 (`Intl.DateTimeFormat`), a formatting request is constructed from a set of user-specified options. These options represent a mix of different concerns:

1.  **Field selection and widths:** e.g., `year: "numeric"`, `month: "long"`, `day: "numeric"`, `weekday: "short"`.
2.  **Formatter policy:** e.g., `hourCycle: "h23"`, `numberingSystem: "latn"`, `calendar: "gregory"`.
3.  **Locale negotiation:** e.g., `localeMatcher: "best fit"`.

Currently, ICU4X's `fieldsets` API is designed around optimized, pre-compiled formatting categories (like `YMD` or `YMDT`) to minimize data size and maximize performance. However, this optimized model is too rigid to directly represent the fine-grained, dynamic field-level requests coming from ECMA-402, where users can request arbitrary combinations of fields and widths.

To bridge this gap, we need a lower-level, highly flexible representation of a datetime formatting request that matches the ECMA-402 model of independent fields and widths, without carrying the broader formatter policy.

In a compliant ECMA-402 implementation, the options are split:
- **Field-related options** (e.g., `year`, `month`, `day`, `hour`) are mapped to `DateTimeFieldBag` (and subsequently converted to a `FieldSet` via the bridge).
- **Policy-related options** (e.g., `numberingSystem`, `hourCycle`, `calendar`) are passed to the formatter via `DateTimeFormatterPreferences`.
- **Locale matching options** (e.g., `localeMatcher`) are handled during locale negotiation beforehand and do not reach the formatter.

## Proposed Solution

We propose a new module, `fieldbag`, centered around the `DateTimeFieldBag` struct.

`DateTimeFieldBag` is a flat struct of optional fields, where each field represents a requested datetime component and its desired width. It acts as a clean, intermediate representation of a user's formatting request.

### Relationship to Existing APIs

Unlike the existing `fieldsets` APIs, which represent resolved, optimized formatting categories, `DateTimeFieldBag` represents the raw, unresolved request.

*   **`DateTimeFieldBag`** (Unresolved Request): Captures fine-grained field presence and width choices.
    *   *Example:* A request for "wide month and two-digit year", represented as `{ year: TwoDigit, month: Long }` (or skeleton `yyMMMM`).
*   **`FieldSetBuilder`** (The Bridge): A helper that takes a detailed `DateTimeFieldBag` and maps it to the closest matching ICU4X formatting category, collapsing widths if necessary.
    *   *Example:* Maps the `yyMMMM` request to a `Date` category with a `Long` length.
*   **`CompositeFieldSet`** (Resolved Category): The concrete runtime enum that wraps the resolved category (e.g., wrapping a `DateFieldSet::YM`).

The bag should support two public conversions:

1. A lossless string form based on UTS 35 classical skeleton syntax, for the subset the bag can represent.
2. A best-effort bridge to and from `FieldSetBuilder`, for interoperability with ICU4X dynamic field sets.

## Goals

- Provide a human-readable, machine-parseable representation of datetime field requests.
- Model the common ECMA-402 / ICU4C datetime field subset.
- Keep the type focused on fields and field widths, not on other formatter options.
- Make round-tripping through a UTS 35 skeleton string lossless for the supported subset.
- Make conversion to `FieldSetBuilder` possible even when it is not exact.

## Non-Goals

- Stock pattern presets (namely, the `dateStyle` and `timeStyle` presets: `Full`, `Long`, `Medium`, and `Short`), as opposed to individual field widths (like `month: "long"` or `weekday: "short"`).
- Locale negotiation (such as the ECMA-402 `localeMatcher` option), which is not handled by ICU4X.
- Hour-cycle preferences.
- Numbering system preferences.
- A full replacement for the existing `fieldsets` API.
- A perfect round-trip between the bag and `FieldSetBuilder`.

## Module Shape

The module name should be `fieldbag` in flat case, matching ICU4X's existing module naming style.
This avoids the overloaded word `components`, which already means something else in ICU4X.

The type name should be `DateTimeFieldBag`.

The module should be public as `icu_datetime::fieldbag`.
It may reuse `provider::fields` internally, but it should not be a provider-only module.

## Data Model

`DateTimeFieldBag` is a struct of optional fields, not an ordered list.
Each field captures the field family and its width choice.

The initial bag should cover ECMA-402 `Intl.DateTimeFormat` field options that map naturally to
UTS 35 skeleton syntax:

- era
- year
- month
- day
- weekday
- day period
- hour
- minute
- second
- subsecond
- time zone name

The bag does not carry:

- hour cycle preference, including the current `components::Bag::hour_cycle` field
- numbering system preference
- year style preference
- other formatter-level knobs that belong to `FieldSetBuilder`

## String Serialization (UTS 35 Skeletons)

The primary exact interchange format for `DateTimeFieldBag` is a string using UTS 35 classical skeleton syntax for the representable subset.

*   **Serialization:** String output must use ICU4X `Writeable`, not `Display`. The serialization always produces a canonicalized string, meaning the same bag state always serializes to the same skeleton string.
*   **Parsing:** Parsing may use a named constructor or a parsing trait. The parser must be strict and reject unsupported syntax rather than guessing.

### Constraints

*   The string syntax must be canonicalized on output.
*   Parsing must reject unsupported syntax.
*   **Parsing must reject explicit hour cycle symbols (`h`, `H`, `K`, `k`).** Skeletons containing these symbols must fail to parse, enforcing that `DateTimeFieldBag` only represents the request for an hour field (using `j` or `C`), with the hour cycle policy resolved separately.
*   If a UTS 35 string contains information the bag cannot represent, the parse must fail.

### Representative Mappings

| Bag concept | Skeleton form |
|---|---|
| year numeric / two-digit | `y` / `yy` |
| month numeric / two-digit | `M` / `MM` |
| month abbreviated / wide / narrow | `MMM` / `MMMM` / `MMMMM` |
| era short / wide / narrow | `G` / `GGGG` / `GGGGG` |
| weekday short / wide / narrow | `E` / `EEEE` / `EEEEE` |
| day numeric / two-digit | `d` / `dd` |
| day period with hour | `C` family, with width determined by day-period style and hour padding |
| hour numeric / two-digit | `j` / `jj` unless an explicit day-period field requires the `C` family |
| minute numeric / two-digit | `m` / `mm` |
| second numeric / two-digit | `s` / `ss` |
| fractional second digits | repeated `S` |
| time zone name | supported `z`, `O`, and `v` forms |

### Hour and Day Period Representation

The hour and day period fields are mapped to UTS 35 input skeleton symbols (`j` and `C`) to avoid introducing hour-cycle preferences into the bag.

*   **Hour-only requests:** Serialized using `j` (numeric) or `jj` (two-digit).
*   **Requests with explicit day period:** Serialized using the `C` family, which encodes both the hour padding and the day-period width (e.g., `C`/`CC` for abbreviated, `CCC`/`CCCC` for wide, `CCCCC`/`CCCCCC` for narrow).

Since `j` and `C` are input skeleton symbols, they map 1-to-1 to the bag's internal state (hour presence/width and day-period presence/width). During conversion to `FieldSetBuilder`, a day-period field without an hour is not supported and should be rejected or normalized.


## Conversion To `FieldSetBuilder`

Conversion from `DateTimeFieldBag` to `FieldSetBuilder` should be best-effort, not exact.
It should not fail; when there is no exact mapping, it should choose a documented representative
builder state.

While this lossy conversion is compliant with the ECMA-402 specification, it may introduce web
compatibility issues in cases where different fields request different lengths (e.g., a wide
month but an abbreviated weekday), which must be collapsed into a single coarser builder-wide
style. We plan to investigate the scope of these web-compat risks under
[CLDR-19550](https://unicode-org.atlassian.net/browse/CLDR-19550). If the risks are too high,
our fallback plan is to enhance `FieldSetBuilder` (and the underlying formatting layer) to
support more granular, per-field length hints, allowing it to honor mixed-width requests
without collapsing them into a single coarse `Length`.

The builder is richer in some ways and coarser in others:

- It models formatter categories such as date, time, zone, and combinations of those.
- It has formatter options that the bag does not store.
- It uses higher-level knobs like `Length`, `TimePrecision`, `Alignment`, and `YearStyle`.

### What gets preserved

The conversion should preserve the closest meaningful mapping for:

- overall date vs time vs date+time shape
- common numeric widths
- common text widths
- supported time-zone styles

### What gets approximated

Several bag details do not map one-for-one into builder options:

- Narrow month and weekday widths may collapse into a broader `Length` choice.
- Exact year width may need to be expressed through `YearStyle` plus the selected fieldset family.
- Field-specific 2-digit (padded) width choices in the bag (such as two-digit year, month, or day) map to a builder-wide `Alignment::Column` preference, signaling that numeric padding is desired for column alignment. *(Rationale: In the `fieldsets` model, `Alignment::Column` is the primary mechanism to support 2-digit numeric fields, as column/tabular alignment is the main driver for developers opting into 2-digit widths. This mapping correctly captures this developer intent.)*
- Minute-only or second-only bags may need to be promoted to a normal time fieldset with the closest
  `TimePrecision`.
- Day period requests do not currently have a clean dynamic fieldset equivalent and need documented
  fallback behavior.

### What should not be invented

Some builder settings have no corresponding bag state and should generally remain unset unless a
clear field-level signal exists:

- `TimePrecision::MinuteOptional`
- any future builder options that are not field-presence or field-width information

### YearStyle Mapping

`YearStyle` does not map to a single field in `DateTimeFieldBag`, but its intent is fully represented
by the **combination of the `year` and `era` fields**:

- `YearStyle::WithEra` maps to `year: Some(Year::Numeric)` + `era: Some(Text::Short)`.
- `YearStyle::NoEra` maps to `year: Some(Year::Numeric)` + `era: None`.
- `YearStyle::Full` maps to `year: Some(Year::Numeric)` (forcing 4-digit year) + `era: None` (or auto).
- `YearStyle::Auto` maps to `year: Some(Year::TwoDigit)` (or `Numeric` depending on length) + `era: None` (or auto).

During conversion, these combinations should be mapped logically to preserve the year/era display policy.

The conversion should be documented as a reconstruction aid, not as a stable interchange format.

## Conversion From `FieldSetBuilder`

Conversion from `FieldSetBuilder` back to `DateTimeFieldBag` should also be best-effort.
It should not fail; the result should be a documented representative bag for the builder state.

This direction is inherently lossy because the builder stores category-level decisions and auxiliary
options, while the bag stores field-level choices.

### Compromises in this direction

- A single builder `Length` value has to be expanded into concrete field widths.
- `YearStyle` must be turned into a representative year/era shape.
- `Alignment::Column` in the builder implies that numeric fields in the reconstructed bag should prefer 2-digit (padded) widths (e.g., `yy`, `MM`, `dd`) to preserve the alignment intent.
- `TimePrecision::MinuteOptional` must become a static field bag, probably hour+minute, losing the
  input-dependent optionality.
- Builder defaults may need to be materialized into explicit bag values.
- Zone styles must be mapped only where the bag supports a corresponding representation.

### Recommended rule

The reverse conversion should pick a canonical representative bag for a builder state.
That keeps the behavior predictable even when multiple bag shapes could correspond to the same builder.

## Suggested API Surface

Keep the API explicit rather than using blanket trait conversions.
The conversion to and from `FieldSetBuilder` is always possible in principle, but it is not an exact
semantic conversion, so generic `From` / `Into` implementations would set the wrong expectation.

Recommended method names:

- `to_field_set_builder(...)`
- `from_field_set_builder(...)`

If the team wants the lossy nature to be unmistakable, the reverse direction can be named more
explicitly, for example `from_field_set_builder_best_effort(...)`.

The methods should be documented in plain language about:

- what is preserved
- what is approximated
- what is dropped
- which direction is intended for interchange versus reconstruction

## Tradeoffs

### Why keep `fieldbag` separate from `fieldsets`

- It keeps the user-facing field request model small and focused.
- It avoids mixing formatter presets and field-level syntax.
- It keeps the lossless string form independent from dynamic fieldset construction.
- It prevents the module from becoming a catch-all for datetime formatting policy.

### Why not map directly to `CompositeFieldSet`

- `CompositeFieldSet` is a concrete runtime formatter input, not a general field request.
- It forces the broadest dynamic shape even when the caller needs a narrower one.
- It discards information about the builder-style intermediate state that can still be useful.

### Why not model stock lengths here

Stock lengths such as `Full`, `Long`, `Medium`, and `Short` are a higher-level preset system.
They expand into fieldsets and sometimes inject non-bag policy, such as time-zone style.
They should live in a separate layer.

## Rejected Alternatives

### `components`

Rejected because it is already used to describe the ICU4X crate collection.

### `skeleton`

Rejected as the module name because it exposes an internal standard term that many users will not know.
The skeleton string representation can still be the wire format.

### `pattern`

Rejected because ICU4X already uses `pattern` for other datetime concepts.

### `fieldset`

Rejected because `fieldsets` already means something else in ICU4X.

### `options`

Rejected because the bag is not a complete formatter options object.

### `FieldSetBuilder` as the only public bridge

Rejected because the bag needs its own exact interchange form. `FieldSetBuilder` is a public and
useful bridge into dynamic fieldsets, but it is not the canonical representation of the bag.

### `CompositeFieldSet` as the only public bridge

Rejected because it over-commits to the broadest runtime fieldset and hides useful intermediate shape.

## Future Work

- Add the stock pattern preset layer for `Full` / `Long` / `Medium` / `Short`.
- Decide whether the bag should support more UTS 35 symbols over time, such as week fields,
  day-of-year, or modified Julian day.
- Decide whether to support `J`, the UTS 35 input skeleton symbol for locale-preferred hour without
  a day-period marker. This is not part of the initial ECMA-oriented model.
- Document exact canonicalization rules for the string form.
- Define the exact best-effort rules for mapping `Alignment`, `YearStyle`, and `TimePrecision`.
- Consider whether a public helper should expose conversion to narrower dynamic fieldset types, not
  only `FieldSetBuilder`.
- Add tests that verify the intended exact and best-effort properties:
  - bag -> string -> bag
  - bag -> builder produces the expected closest `FieldSetBuilder`
  - builder -> bag produces the expected representative `DateTimeFieldBag`
  - formatter -> builder -> formatter

## Implementation Notes

The implementation should stay close to the existing ICU4X datetime architecture:

- implement and expose the bag from `icu_datetime::fieldbag`
- reuse provider field primitives only when they fit the public fieldbag contract
- use `Writeable` for string output, not `Display`
- keep parsing logic explicit and conservative
- reuse existing `FieldSetBuilder` conversion helpers where possible
- avoid coupling the bag to locale fallback or formatter selection logic

The overall mental model should be:

1. `DateTimeFieldBag` captures a field request.
2. A UTS 35 string is the precise interchange form for that request.
3. `FieldSetBuilder` is the best-effort bridge into ICU4X dynamic fieldsets.
4. `CompositeFieldSet` remains a downstream choice, not the core representation.
