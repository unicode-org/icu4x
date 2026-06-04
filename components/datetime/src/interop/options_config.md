# Datetime Interop: Options Bag and Resolution Config

This document describes the unified `DateTimeFormatterOptions` (the catchall options bag) and how it is resolved to backend-specific arguments for ICU4C and ICU4X.

---

## 1. The Catchall Options Bag

Instead of separate option structures, a single unified `DateTimeFormatterOptions` struct is exposed. Note that raw LDML patterns are excluded from this options bag to maintain backend symmetry (as ICU4X does not support arbitrary patterns at runtime; see [design_spec.md](design_spec.md#8-future-work-raw-pattern-support)).

### 1.1. Options Table

The options bag supports the following fields:

| Option Name | Type / Values | Description |
|---|---|---|
| `skeleton` | String | **ICU4C Override**: Classical skeleton (e.g., `yMdHms`). If set, individual fields are ignored. |
| `date_style` | `Full`, `Long`, `Medium`, `Short` | **High-level Style**: Pre-defined style for the date part (ECMA/ICU4C). |
| `time_style` | `Full`, `Long`, `Medium`, `Short` | **High-level Style**: Pre-defined style for the time part (ECMA/ICU4C). |
| `date_fields` | [`DateFields`](https://docs.rs/icu_datetime/latest/icu_datetime/fieldsets/builder/enum.DateFields.html) | **ICU4X Field**: Pre-defined date field combinations (YMD, MD, etc.). |
| `time_precision`| [`TimePrecision`](https://docs.rs/icu_datetime/latest/icu_datetime/options/enum.TimePrecision.html) | **ICU4X Field**: Time precision (Hour, Minute, Second, Subsecond, etc.). |
| `zone_style` | [`ZoneStyle`](https://docs.rs/icu_datetime/latest/icu_datetime/fieldsets/builder/enum.ZoneStyle.html) | **ICU4X Field**: Timezone style (SpecificLong, LocalizedOffset, etc.). |
| `alignment` | [`Alignment`](https://docs.rs/icu_datetime/latest/icu_datetime/options/enum.Alignment.html) | **ICU4X Option**: Column/Auto alignment. |
| `year_style` | [`YearStyle`](https://docs.rs/icu_datetime/latest/icu_datetime/options/enum.YearStyle.html) | **ICU4X Option**: Year style (WithEra, NoEra, etc.). |
| `weekday` | `Narrow`, `Short`, `Long` | **ECMA Field Style**: Weekday display style. |
| `era` | `Narrow`, `Short`, `Long` | **ECMA Field Style**: Era display style. |
| `year` | `Numeric`, `TwoDigit` | **ECMA Field Style**: Year display style. |
| `month` | `Numeric`, `TwoDigit`, `Narrow`, `Short`, `Long` | **ECMA Field Style**: Month display style. |
| `day` | `Numeric`, `TwoDigit` | **ECMA Field Style**: Day display style. |
| `day_period` | `Narrow`, `Short`, `Long` | **ECMA Field Style**: AM/PM display style. |
| `hour` | `Numeric`, `TwoDigit` | **ECMA Field Style**: Hour display style. |
| `minute` | `Numeric`, `TwoDigit` | **ECMA Field Style**: Minute display style. |
| `second` | `Numeric`, `TwoDigit` | **ECMA Field Style**: Second display style. |
| `fractional_second_digits` | 1..9 | **ECMA Field Style**: Number of fractional second digits. |
| `time_zone_name` | `Short`, `Long`, `ShortOffset`, `LongOffset`, `ShortGeneric`, `LongGeneric` | **ECMA Field Style**: Timezone display style. |
| `hour_cycle` | [`HourCycle`](https://docs.rs/icu_datetime/latest/icu_datetime/preferences/enum.HourCycle.html) | **Preference**: Hour cycle override (H11, H12, H23, H24). |

---

## 2. ICU4C Backend Mapping (Skeletons & Styles)

To support ICU4C, the unified `DateTimeFormatterOptions` must be resolved to ICU4C-compatible arguments (skeletons or styles). This resolution logic runs in Rust (in the `icu_datetime::interop` module) and does not invoke ICU4C directly.

### 2.1. Resolved Output (Rust Struct)

The resolution logic in `icu_datetime::interop` returns a Rust struct `Icu4cResolvedArgs` containing:
-   `skeleton`: `Option<String>` (resolved classical skeleton).
-   `date_style`: `Option<Style>` (resolved date style).
-   `time_style`: `Option<Style>` (resolved time style).

### 2.2. Precedence Rules

The resolution logic follows this order of precedence:

1.  **Skeleton**: If `skeleton` is set in the options, it is returned as the resolved skeleton. Styles are set to `None`.
2.  **Styles**: If `date_style` or `time_style` is set, they are returned. The skeleton is set to `None`. Individual field options are ignored.
3.  **Individual Fields**: If neither skeletons nor styles are set:
    *   **ECMA-style Fields**: If ECMA options (`year`, `month`, `day`, etc.) are present, they are mapped to standard UTS 35 skeleton symbols. This is a direct **one-to-one mapping** (e.g., `year: Numeric` -> `y`, `month: Long` -> `MMMM`, `day: TwoDigit` -> `dd`). These mapped symbols are concatenated in UTS 35 canonical order to form the resolved classical skeleton.
    *   **ICU4X-style Fields**: If no ECMA options are present, but ICU4X builder-style options (`date_fields`, `time_precision`, `zone_style`, etc.) are present, they are treated as an ICU4X semantic skeleton. This semantic skeleton is mapped to standard skeleton symbols according to the UTS 35 rules cited below.
    *   **Conflict Resolution**: If both ECMA-style and ICU4X-style options are present, the ECMA-style options take precedence.
    *   Styles are set to `None`.

### 2.3. UTS 35 Mapping Details

When mapping semantic options to classical skeletons, the interop layer strictly follows the [UTS 35: Unicode Technical Standard #35 (Part 4: Dates)](https://unicode.org/reports/tr35/tr35-dates.html) specification:

-   **Basic Field Mapping**: Follows [UTS 35: Mapping to Standard Skeletons](https://unicode.org/reports/tr35/tr35-dates.html#Mapping_to_Standard_Skeletons) (handles standalone vs. formatting contexts like `LLLL` vs. `MMMM` for month).
-   **Time Precision Variations**: Follows [UTS 35: Time Precision Skeleton Variations](https://unicode.org/reports/tr35/tr35-dates.html#Semantic_Time_Precision_Skeleton_Variations).
-   **Year Style Variations**: Follows [UTS 35: Year Style Skeleton Variations](https://unicode.org/reports/tr35/tr35-dates.html#Semantic_Year_Style_Skeleton_Variations).

The resolved standard symbols are concatenated in UTS 35 canonical order to form the final classical skeleton.

---

## 3. ICU4X Backend Mapping (FieldSets)

To construct an ICU4X formatter, the unified `DateTimeFormatterOptions` must be resolved to a `CompositeFieldSet`.

The mapping resolves the options bag to a `CompositeFieldSet`, which is a dynamic enum covering all supported combinations of date, time, and timezone fieldsets in ICU4X.

### 3.1. Styles Mapping

High-level `date_style` and `time_style` are mapped to a combination of `Length` and default fieldsets (`DateFields::YMD` for date, `TimePrecision::Second` for time):

-   Example: `date_style: Long` maps to `DateFieldSet::YMD(YMD::long())`.

### 3.2. Individual Fields (ECMA) to `CompositeFieldSet`

When individual field options (like `year`, `month`, `hour`) are provided instead of high-level styles, they are resolved to a `CompositeFieldSet` using the following steps:

1.  **Determine `DateFields`**:
    *   If `year`, `month`, and `day` are present: `DateFields::YMD`
    *   If `month` and `day` are present: `DateFields::MD`
    *   If only `year` is present: `DateFields::Y`
    *   If only `month` is present: `DateFields::M`
    *   If `weekday` is present with a date: `DateFields::YMDE`, `DateFields::MDE`, or `DateFields::DE` (depending on which other date fields are present).

2.  **Determine `TimePrecision`**:
    *   If `hour`, `minute`, and `second` are present: `TimePrecision::Second` (or `Subsecond` if `fractional_second_digits` is set).
    *   If `hour` and `minute` are present: `TimePrecision::Minute`.
    *   If only `hour` is present: `TimePrecision::Hour`.

3.  **Determine `Length` / Field Styles**:
    Since ICU4X applies a single `Length` to the entire fieldset, mixed styles (e.g., short year with a long month) must be resolved to a single "best fit" `Length` (note that there is ongoing work to add features like field-specific length hints to semantic skeletons to better support mixed styles in the future):
    *   If any field uses `Long` (wide): `Length::Long`.
    *   Else if any field uses `Short` (abbreviated) or `Medium`: `Length::Medium`.
    *   Else if all fields are `Numeric` or `TwoDigit`: `Length::Short`.

4.  **Determine `YearStyle`**:
    *   If `era` is requested: `YearStyle::WithEra`.
    *   Else: `YearStyle::Auto`.
