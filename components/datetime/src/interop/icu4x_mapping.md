# ICU4X Backend Mapping Algorithm

This document describes the algorithm to map the unified `DateTimeFormatterOptions` to ICU4X's static/data-efficient `CompositeFieldSet`.

## 1. Styles Mapping

High-level `date_style` and `time_style` are mapped to a combination of `Length` and default fieldsets (`DateFields::YMD` for date, `TimePrecision::Second` for time):

-   Example: `date_style: Long` maps to `DateFieldSet::YMD(YMD::long())`.

## 2. Individual Fields (ECMA) to `CompositeFieldSet`

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
    Since ICU4X applies a single `Length` to the entire fieldset, mixed styles (e.g., short year with a long month) must be resolved to a single "best fit" `Length`:
    *   If any field uses `Long` (wide): `Length::Long`.
    *   Else if any field uses `Short` (abbreviated) or `Medium`: `Length::Medium`.
    *   Else if all fields are `Numeric` or `TwoDigit`: `Length::Short`.

4.  **Determine `YearStyle`**:
    *   If `era` is requested: `YearStyle::WithEra`.
    *   Else: `YearStyle::Auto`.
