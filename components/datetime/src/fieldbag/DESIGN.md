# DateTime Field Bag Design

## Summary

`fieldbag` is the module for a compact, field-level description of a datetime formatting request.
It is intended to represent the subset of `Intl.DateTimeFormat` / ICU4C-style datetime syntax that
describes which fields should appear and how wide each field should be, without carrying broader
formatter policy.

In a compliant ECMA-402 implementation, the options are split:
- **Field-related options** (e.g., `year`, `month`, `day`, `hour`) are mapped to `DateTimeFieldBag` (and subsequently converted to a `FieldSet`).
- **Policy-related options** (e.g., `numberingSystem`, `hourCycle`, `calendar`) are passed to the formatter via `DateTimeFormatterPreferences`.
- **Locale matching options** (e.g., `localeMatcher`) are handled during locale negotiation beforehand and do not reach the formatter.

The central type is `DateTimeFieldBag`.

This module is deliberately narrower than ICU4X `fieldsets`:

- `DateTimeFieldBag` is about fine-grained field content and widths (e.g., requesting a "wide month and two-digit year", represented as `yyMMMM`).
- `FieldSetBuilder` is the bridge that resolves these detailed requests into coarser ICU4X categories (e.g., mapping `yyMMMM` to a Date category with a Long length).
- `CompositeFieldSet` is a top-level dynamic enum that wraps all possible runtime fieldset categories (e.g., wrapping a resolved `DateFieldSet::YMD`).

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

## Represented Syntax

The string form should follow UTS 35 classical skeleton conventions for the subset this bag can
represent. The exact symbol set is intentionally limited to the bag's model.

Representative mappings:

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

The implementation should define one canonical output per represented bag state.
That means the same bag should always serialize to the same skeleton string.

The hour field is the main place where the design must not accidentally reintroduce an hour-cycle
preference. ECMA-402 has `hour` as a field and `hourCycle` as a separate option. Since the bag
contains only fields, the external skeleton form should use UTS 35 input skeleton symbols rather
than choosing `h`, `H`, or `K` as policy.

For hour-only requests, the canonical skeleton output should use `j`:

- `j` for numeric hour
- `jj` for two-digit hour

For requests with an explicit day-period field, the canonical skeleton output should use `C`.
The `C` family carries both hour padding and day-period width:

- `C` / `CC` for abbreviated day period with numeric / two-digit hour
- `CCC` / `CCCC` for wide day period with numeric / two-digit hour
- `CCCCC` / `CCCCCC` for narrow day period with numeric / two-digit hour

The implementation may need fieldbag-specific parsing and writing for `j` and `C`, since these
symbols are UTS 35 input skeleton symbols and must not occur in CLDR pattern or skeleton data.
In this design, a day-period field without an hour should not be produced by builder conversion,
and parsing one should either reject it or define a documented normalization.

## String Form

The string form should use UTS 35 classical skeleton syntax for the representable subset.
The intent is that a valid bag can be written to a canonical string and parsed back into the same bag.

This is the primary exact interchange format for the type.

String output should use ICU4X `Writeable`, not `Display`.
Parsing may use a named constructor or a parsing trait, but the public docs should emphasize that
the accepted syntax is UTS 35 skeleton syntax for the supported subset.

### Why string round-trip is the canonical form

UTS 35 skeleton syntax already captures the core idea of the bag:

- which fields are present
- how wide they are
- an established cross-implementation vocabulary

That makes the string form a better interchange format than inventing a new ad hoc serialization.

### Constraints

- The string syntax should be canonicalized on output.
- Parsing should reject unsupported syntax rather than guessing.
- If a UTS 35 string contains information the bag cannot represent, the parse should fail or normalize only when the normalization is documented and unambiguous.

## Conversion To `FieldSetBuilder`

Conversion from `DateTimeFieldBag` to `FieldSetBuilder` should be best-effort, not exact.
It should not fail; when there is no exact mapping, it should choose a documented representative
builder state.

While this lossy conversion is compliant with the ECMA-402 specification, it may introduce web
compatibility issues in cases where different fields request different lengths (e.g., a wide
month but an abbreviated weekday), which must be collapsed into a single coarser builder-wide
style. We plan to investigate the scope of these web-compat risks and, as needed, iterate on
the semantic skeleton design to support more granular options.

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
- Field-specific 2-digit (padded) width choices in the bag (such as two-digit year, month, or day) map to a builder-wide `Alignment::Column` preference, signaling that numeric padding is desired for column alignment.
- Minute-only or second-only bags may need to be promoted to a normal time fieldset with the closest
  `TimePrecision`.
- Day period requests do not currently have a clean dynamic fieldset equivalent and need documented
  fallback behavior.

### What should not be invented

Some builder settings have no corresponding bag state and should generally remain unset unless a
clear field-level signal exists:

- `YearStyle`
- `TimePrecision::MinuteOptional`
- any future builder options that are not field-presence or field-width information

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
