# DateTime Field Bag Design

## Background & Motivation

In ECMA-402 (`Intl.DateTimeFormat`), a formatting request is constructed from a set of user-specified options. These options represent a mix of different concerns, which are split in a compliant implementation:

1.  **Field-related options** (e.g., `year`, `month`, `day`, `hour`): These capture the user's choices for field selection and widths. They are mapped to `DateTimeFieldBag` (and subsequently converted to a `FieldSet` via the bridge).
2.  **Policy-related options** (e.g., `numberingSystem`, `hourCycle`, `calendar`): These represent broader formatter policy and are passed to the formatter via `DateTimeFormatterPreferences`.
3.  **Locale matching options** (e.g., `localeMatcher`): These are handled during locale negotiation beforehand and do not reach the formatter.

### Goals

*   Provide a human-readable, machine-parseable representation of datetime field requests.
*   Model the common ECMA-402 / ICU4C datetime field subset.
*   Keep the type focused on fields and field widths, not on other formatter options.
*   Make round-tripping through a UTS 35 skeleton string lossless for the supported subset.
*   Make conversion to `FieldSetBuilder` possible even when it is not exact.

### Non-Goals

*   Stock pattern presets (namely, the `dateStyle` and `timeStyle` presets: `Full`, `Long`, `Medium`, and `Short`), as opposed to individual field widths (like `month: "long"` or `weekday: "short"`).
*   Locale negotiation (such as the ECMA-402 `localeMatcher` option), which is not handled by ICU4X.
*   Hour-cycle preferences.
*   Numbering system preferences.
*   A full replacement for the existing `fieldsets` API.
*   A perfect round-trip between the bag and `FieldSetBuilder`.

## Proposed Solution

We propose a new module, `fieldbag`, centered around the `DateTimeFieldBag` struct.

The architecture is built around a 4-step mental model that defines how a formatting request flows into the ICU4X formatting pipeline:

1.  **`DateTimeFieldBag`** (Raw Request): Captures the fine-grained field presence and width choices from the user.
    *   *Example:* A request for "wide month and two-digit year", represented as `{ year: TwoDigit, month: Long }`.
2.  **UTS 35 Skeleton String** (Interchange Format): The precise, lossless wire format for the raw request (e.g., `yyMMMM`).
3.  **`FieldSetBuilder`** (The Bridge): A helper that takes a detailed `DateTimeFieldBag` (or its skeleton) and maps it to the closest matching ICU4X formatting category, collapsing widths if necessary.
    *   *Example:* Maps the `yyMMMM` request to a `YM` fieldset.
4.  **`CompositeFieldSet`** (Resolved Category): The concrete, optimized runtime enum that wraps the resolved category (e.g., wrapping a `DateFieldSet::YM`). This is a downstream choice, not the core representation of the request.

`DateTimeFieldBag` is a flat struct of optional fields, where each field represents a requested datetime component and its desired width. It acts as a clean, intermediate representation of a user's formatting request.

### Suggested API Surface

We recommend explicit, named methods for conversion and standard traits for serialization, making the lossy nature of the conversions clear:

*   `impl writeable::Writeable for DateTimeFieldBag` (enables canonical skeleton serialization)
*   `impl std::str::FromStr for DateTimeFieldBag` (strict parser using UTS 35 skeleton syntax)
    *   `type Err = DateTimeFieldBagParseError`
*   `impl DateTimeFieldBag`
    *   `pub fn to_string(&self) -> String` (convenience wrapper shadowing `ToString::to_string` for high-performance serialization)
    *   `pub fn try_from_skeleton(s: &str) -> Result<Self, DateTimeFieldBagParseError>` (explicit, self-documenting named constructor that delegates to `FromStr`)
    *   `pub fn to_field_set_builder(&self) -> FieldSetBuilder` (lossy conversion)
    *   `pub fn from_field_set_builder(builder: &FieldSetBuilder) -> Self` (best-effort reconstruction)

### Module Shape

The module name should be `fieldbag` in flat case, matching ICU4X's existing module naming style.
This avoids the overloaded word `components`, which already means something else in ICU4X.

The type name should be `DateTimeFieldBag`.

The module should be public as `icu_datetime::fieldbag`.
It may reuse `provider::fields` internally, but it should not be a provider-only module.

### Data Model

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
*   **Parsing:** Parsing uses the `FromStr` trait (returning `DateTimeFieldBagParseError`) or the `try_from_skeleton` helper. The parser must be strict and reject unsupported syntax rather than guessing.

### Constraints

*   The string syntax must be canonicalized on output.
*   Parsing must reject unsupported syntax, returning `DateTimeFieldBagParseError`.
*   **Parsing must reject explicit hour cycle symbols (`h`, `H`, `K`, `k`).** Skeletons containing these symbols must fail to parse.
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
| day period without hour | `a` / `b` / `B` family (depending on style/width) |
| hour numeric / two-digit | `j` / `jj` unless an explicit day-period field requires the `C` family |
| minute numeric / two-digit | `m` / `mm` |
| second numeric / two-digit | `s` / `ss` |
| fractional second digits | repeated `S` |
| time zone name | supported `z`, `O`, and `v` forms |

### Hour and Day Period Representation

The hour and day period fields are mapped to UTS 35 skeleton symbols to avoid introducing hour-cycle preferences into the bag, while still supporting standalone day periods.

*   **Hour-only requests:** Serialized using `j` (numeric) or `jj` (two-digit).
*   **Requests with hour and day period:** Serialized using the `C` family, which encodes both the hour padding and the day-period width (e.g., `C`/`CC` for abbreviated, `CCC`/`CCCC` for wide, `CCCCC`/`CCCCCC` for narrow).
*   **Requests with day period only (no hour):** Serialized using the `a`/`b`/`B` family depending on the requested day-period style (abbreviated/wide/narrow).

**Standalone Day Period Handling:**
*   The skeleton parser **accepts** standalone day period symbols (`a`, `b`, `B`) and parses them into a `DateTimeFieldBag` with `day_period` set and `hour` unset. This ensures lossless round-tripping.
*   During conversion to `FieldSetBuilder` (`to_field_set_builder`), a standalone day period is not supported by the underlying dynamic fieldsets and will be rejected or normalized (best-effort).


## Conversion from DateTimeFieldBag to FieldSetBuilder

Conversion from `DateTimeFieldBag` to `FieldSetBuilder` is best-effort and lossy. It should not fail; when there is no exact mapping, it should choose a documented representative builder state.

While this lossy conversion is compliant with the ECMA-402 specification, it may introduce web compatibility issues in cases where different fields request different lengths (e.g., a wide month but an abbreviated weekday). We plan to investigate the scope of these web-compat risks under [CLDR-19550](https://unicode-org.atlassian.net/browse/CLDR-19550). If the risks are too high, our fallback plan is to enhance `FieldSetBuilder` (and the underlying formatting layer) to support more granular, per-field length hints, allowing it to honor mixed-width requests without collapsing them into a single coarse `Length`.

### What gets preserved

The conversion preserves the closest meaningful mapping for:
*   Overall date vs. time vs. date+time shape.
*   Common numeric widths.
*   Common text widths.
*   Supported time-zone styles.

### What gets approximated or omitted

*   **Width Simplification:** Narrow month and weekday widths collapse into a broader `Length` choice.
*   **Alignment/Padding:** Field-specific 2-digit (padded) width choices map to a builder-wide `Alignment::Column` preference. *(Rationale: In the `fieldsets` model, `Alignment::Column` is the primary mechanism to support 2-digit numeric fields, as column/tabular alignment is the main driver for developers opting into 2-digit widths. This mapping correctly captures this developer intent.)*
*   **Time Precision:** Minute-only or second-only requests are promoted to a normal time fieldset with the closest `TimePrecision`.
*   **Day Period:** Day period requests without an hour are rejected as invalid.
*   **Settings with no bag equivalent:** `TimePrecision::MinuteOptional` and other non-field builder options remain unset.

### `YearStyle` Forward Resolution

Since `FieldSetBuilder` controls year width via a combination of `Length` and `YearStyle`, while the bag controls it via direct field widths, we resolve the mapping as follows:
*   `year: Some(Year::Numeric)` $\rightarrow$ Maps to `YearStyle::Auto`. The builder's `length` will be guided towards `Length::Medium` or `Length::Long` to prefer full year display.
*   `year: Some(Year::TwoDigit)` $\rightarrow$ Maps to `YearStyle::Auto`. The builder's `length` will be guided towards `Length::Short` to prefer 2-digit display.
*   `era: Some(...)` $\rightarrow$ Maps to `YearStyle::WithEra` (overriding the `Auto` choice above to ensure era is displayed).
*   `era: None` $\rightarrow$ Maps to `YearStyle::Auto` (relying on locale defaults for era display).

The conversion should be documented as a reconstruction aid, not as a stable interchange format.

## Conversion from FieldSetBuilder to DateTimeFieldBag

Conversion from `FieldSetBuilder` back to `DateTimeFieldBag` is also best-effort and inherently lossy, as the builder stores category-level decisions and auxiliary options, while the bag stores field-level choices. It should not fail; the result should be a documented representative bag for the builder state.

### Compromises in this direction

*   A single builder `Length` value must be expanded into concrete field widths.
*   `Alignment::Column` in the builder implies that numeric fields in the reconstructed bag should prefer 2-digit (padded) widths (e.g., `yy`, `MM`, `dd`) to preserve the alignment intent.
*   `TimePrecision::MinuteOptional` must become a static field bag (likely hour+minute), losing the input-dependent optionality.
*   Builder defaults may need to be materialized into explicit bag values.
*   Zone styles must be mapped only where the bag supports a corresponding representation.

### `YearStyle` Mapping

`YearStyle` in the builder is mapped back to a combination of `year` and `era` fields in the reconstructed `DateTimeFieldBag`. Since `YearStyle` controls both century display and era visibility, the mapping is defined as:

*   `YearStyle::WithEra` $\rightarrow$ `year: Some(Year::Numeric)`, `era: Some(Text::Short)`
*   `YearStyle::NoEra` $\rightarrow$ `year: Some(Year::Numeric)`, `era: None`
*   `YearStyle::Full` $\rightarrow$ `year: Some(Year::Numeric)`, `era: None` (leaves era unset, see note below)
*   `YearStyle::Auto` $\rightarrow$ `year: Some(Year::TwoDigit)` (or `Numeric` depending on length), `era: None` (leaves era unset)

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

We rejected making `CompositeFieldSet` the direct source/target of `DateTimeFieldBag` conversion.
- `CompositeFieldSet` is a concrete runtime formatter input, not a general field request.
- Converting to it directly would force the broadest dynamic shape even when a caller needs a narrower one.
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

- Add the stock pattern preset layer for `Full` / `Long` / `Medium` / `Short`.
- Decide whether the bag should support more UTS 35 symbols over time, such as week fields, day-of-year, or modified Julian day.
- Decide whether to support `J`, the UTS 35 input skeleton symbol for locale-preferred hour without a day-period marker. This is not part of the initial ECMA-oriented model.
- Consider whether a public helper should expose conversion to narrower dynamic fieldset types, not only `FieldSetBuilder`.
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
