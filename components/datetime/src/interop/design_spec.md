# Design Doc: Datetime Interop Layer (ICU4X / ICU4C / ECMA)

## Status: Draft
**Author:** AI Agent (Gemini) working with @sffc

---

## 1. Background and Motivation

ICU4X provides a modern, modular, and lightweight internationalization library in Rust. However, during transition phases or in environments where system-provided libraries are preferred, clients may want to choose between ICU4X and ICU4C at compile-time or runtime.

Furthermore, clients targeting web environments often need to align with **ECMA-402 (Intl.DateTimeFormat)** options.

This document specifies a **Datetime Interop Layer** based on a **catchall options bag** and a **decoupled architecture** that avoids linking ICU4C into the Rust codebase.

The design aims to:
- Define a unified options bag covering ICU4X, ICU4C, and ECMA-402 options.
- Keep the Rust codebase (`icu_datetime`) free of ICU4C dependencies.
- Perform argument resolving (mapping options to ICU4C skeletons/styles) in Rust.
- Export these helpers via `icu_capi` (using Diplomat).
- Implement the actual switching and ICU4C calls in a header-only C/C++ layer (`ffi/icu4c_interop`).

---

## 2. Architecture Overview

The interop layer is split across three boundaries to maintain strict dependency separation:

```mermaid
graph TD
    Client["C/C++ Client Code"] --> InteropCPP["ffi/icu4c_interop Header-only C++"]
    
    subgraph rust["Rust Boundary (No ICU4C dependency)"]
        InteropCPP --> |Calls FFI| CAPI["icu_capi C API"]
        CAPI --> |Exposes| InteropRust["icu_datetime::interop"]
        InteropRust --> |Resolves Options| Options[DateTimeFormatterOptions]
    end

    subgraph linkage["C/C++ Linkage Boundary"]
        InteropCPP --> |Links| CAPI_SO["libicu_capi.so (ICU4X)"]
        InteropCPP --> |Links| ICU4C_SO["libicui18n.so (ICU4C)"]
    end
```

1.  **`icu_datetime::interop` (Rust)**: Contains `DateTimeFormatterOptions` (the catchall options bag) and the resolution logic to map these options to ICU4C-compatible arguments (skeletons or styles), without calling ICU4C.
2.  **`icu_capi` (Rust/FFI)**: Exposes the options bag and the resolution function to C/C++ using Diplomat.
3.  **`ffi/icu4c_interop` (C/C++ Header-only)**: The integration point for clients. It includes `icu_capi` headers and system ICU4C headers (`unicode/udat.h`). It contains the logic to switch between backends and call `libicui18n.so` or `libicu_capi.so` accordingly.

---

## 3. The Catchall Options Bag

We define a single, comprehensive options struct in `icu_datetime::interop`. Note that raw LDML patterns are excluded from this options bag to maintain backend symmetry (as ICU4X does not support arbitrary patterns at runtime; see Section 9).

```rust
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DateTimeFormatterOptions {
    // ==========================================
    // 1. Classical / ICU4C Overrides
    // ==========================================
    /// A classical skeleton string (e.g., "yMdHms").
    pub skeleton: Option<String>,

    // ==========================================
    // 2. High-level Styles (ECMA & ICU4C)
    // ==========================================
    pub date_style: Option<DateTimeStyle>,
    pub time_style: Option<DateTimeStyle>,

    // ==========================================
    // 3. ICU4X Builder-style Fields
    // ==========================================
    pub date_fields: Option<DateFields>,
    pub time_precision: Option<TimePrecision>,
    pub zone_style: Option<ZoneStyle>,
    pub alignment: Option<Alignment>,
    pub year_style: Option<YearStyle>,

    // ==========================================
    // 4. ECMA-402 Fine-Grained Field Options
    // ==========================================
    pub weekday: Option<WeekdayStyle>,
    pub era: Option<EraStyle>,
    pub year: Option<YearStyleOption>,
    pub month: Option<MonthStyle>,
    pub day: Option<DayStyle>,
    pub day_period: Option<DayPeriodStyle>,
    pub hour: Option<HourStyle>,
    pub minute: Option<MinuteStyle>,
    pub second: Option<SecondStyle>,
    pub fractional_second_digits: Option<u8>, // 1..9
    pub time_zone_name: Option<TimeZoneNameStyle>,

    // ==========================================
    // 5. Global Preferences
    // ==========================================
    pub hour_cycle: Option<HourCycle>,
}
```

### 3.1. Supporting Enums

```rust
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DateTimeStyle {
    Full,
    Long,
    Medium,
    Short,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WeekdayStyle { Narrow, Short, Long }

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EraStyle { Narrow, Short, Long }

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum YearStyleOption { Numeric, TwoDigit }

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MonthStyle { Numeric, TwoDigit, Narrow, Short, Long }

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DayStyle { Numeric, TwoDigit }

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DayPeriodStyle { Narrow, Short, Long }

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HourStyle { Numeric, TwoDigit }

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MinuteStyle { Numeric, TwoDigit }

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SecondStyle { Numeric, TwoDigit }

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TimeZoneNameStyle {
    Short,
    Long,
    ShortOffset,
    LongOffset,
    ShortGeneric,
    LongGeneric,
}
```

---

## 4. Rust Resolution Layer (`icu_datetime::interop`)

This module is responsible for translating the options bag into concrete arguments that ICU4C can understand.

```rust
#[repr(i32)]
pub enum InteropDateFormatStyle {
    None = -1,
    Full = 0,
    Long = 1,
    Medium = 2,
    Short = 3,
}

pub struct Icu4cResolvedArgs {
    pub skeleton: Option<String>,
    pub date_style: InteropDateFormatStyle,
    pub time_style: InteropDateFormatStyle,
}

/// Resolves the catchall options bag into ICU4C arguments.
/// Does NOT call ICU4C.
pub fn resolve_icu4c_args(options: &DateTimeFormatterOptions) -> Icu4cResolvedArgs {
    // Resolution logic following precedence:
    // 1. If skeleton is set -> return skeleton, styles = None
    // 2. If styles are set -> return styles, skeleton = None
    // 3. If individual fields are set -> construct skeleton, return skeleton, styles = None
}
```

---

## 5. Backend Mapping Specifications

To handle overlap and ensure consistent output across backends, the interop layer defines strict mapping rules.

### 5.1. ICU4C Backend Mapping

ICU4C is dynamic and maps naturally to skeletons and styles. The mapping from semantic options to classical skeletons is based on [UTS 35: Unicode Technical Standard #35 (Part 4: Dates)](https://unicode.org/reports/tr35/tr35-dates.html).

#### 5.1.1. Basic Field Mapping

To convert from semantic fields to standard skeleton symbols, follow the mapping table below (adapted from UTS 35):

| Semantic Field | Standalone? | Option / Casing | Long | Medium | Short | Notes |
|---|---|---|---|---|---|---|
| **Year** | N/A | N/A | `y` | `y` | `y` | Default from locale's `datetimeSkeleton` |
| **Month** | No | N/A | `MMMM` | `MMM` | `M` / `MM` | `MM` if 2-digit requested |
| | Yes | N/A | `LLLL` | `LLL` | `L` / `LL` | Standalone context |
| **Day** | N/A | N/A | `d` | `d` | `d` / `dd` | `dd` if alignment is `Column` |
| **Weekday**| No | N/A | `EEEE` | `EEE` | `EEE` | |
| | Yes | N/A | `EEEE` | `EEE` | `EEEEE` | Standalone short maps to narrow |
| **Time** | N/A | unset | `C` | `C` | `C` | Locale default hour cycle |
| | N/A | 12h (H11/H12) | `h` | `h` | `h` / `hh` | `hh` if 2-digit requested |
| | N/A | 24h (H23/H24) | `H` | `H` | `H` / `HH` | `HH` if 2-digit requested |
| **Zone** | No | Generic | `v` | `v` | `v` | Non-standalone generic |
| | Yes | Generic | `vvvv` | `vvvv` | `v` | Standalone generic |
| | No | Specific | `z` | `z` | `z` | Non-standalone specific |
| | Yes | Specific | `zzzz` | `zzzz` | `z` | Standalone specific |
| | N/A | Location | `VVVV` | `VVVV` | `VVVV` | |
| | N/A | Offset | `OOOO` | `OOOO` | `O` | Long offset uses `OOOO` |

#### 5.1.2. Time Precision Skeleton Variations

Apply the following adjustments to the skeleton based on the requested time precision:
- **Hour**: No change (uses the hour symbol resolved above).
- **Minute**: Append `m` (or `mm` if 2-digit requested).
- **MinuteOptional**: Append `m` if the input time has a non-zero minute.
- **Second**: Append `m` and `s` (or `ss` if 2-digit requested).
- **FractionalSecond**: Append `m`, `s`, and `N` occurrences of `S` (where `N` is `fractional_second_digits`).

#### 5.1.3. Year Style Skeleton Variations

Apply the following adjustments to the year/era symbols in the skeleton:
- **Auto**: Use the year/era fields from the locale's default skeleton (e.g., `y`, `yy`, `yG`).
- **Full**: Force 4-digit year by replacing `yy` with `y` (or `yyyy` if explicit).
- **WithEra**: Force era display by replacing `yy` with `y` and appending `G` (or `GGGG` for long, `GGGGG` for narrow) if no era symbol is present.

Symbols are concatenated in UTS 35 canonical order.

### 5.2. ICU4X Backend Mapping

ICU4X is static/data-efficient. Mapping arbitrary options to ICU4X requires resolving them to a `CompositeFieldSet`.

#### Styles:
- `date_style` / `time_style` -> Map to `Length` and `DateFields::YMD` / `TimePrecision::Second` or similar default fieldsets.
  - e.g., `date_style: Long` -> `DateFieldSet::YMD(YMD::long())`.

#### Individual Fields (ECMA) to `CompositeFieldSet`:
1.  **Determine `DateFields`**:
    *   If `year`, `month`, `day` are present -> `DateFields::YMD`
    *   If `month`, `day` are present -> `DateFields::MD`
    *   If only `year` -> `DateFields::Y`
    *   If only `month` -> `DateFields::M`
    *   If `weekday` is present with date -> `DateFields::YMDE` or `DateFields::MDE` or `DateFields::DE` (depending on other fields).
2.  **Determine `TimePrecision`**:
    *   If `hour`, `minute`, `second` -> `TimePrecision::Second` (or `Subsecond` if fractional seconds are set).
    *   If `hour`, `minute` -> `TimePrecision::Minute`.
    *   If only `hour` -> `TimePrecision::Hour`.
3.  **Determine `Length` / Field Styles**:
    Since ICU4X applies a single `Length` to the entire fieldset, mixed styles (e.g., short year, long month) must be resolved to a single "best fit" `Length`:
    *   If any field uses `Long` (wide), use `Length::Long`.
    *   Else if any field uses `Short` (abbreviated) or `Medium`, use `Length::Medium`.
    *   Else if all fields are `Numeric` / `TwoDigit`, use `Length::Short`.
4.  **Determine `YearStyle`**:
    *   If `era` is requested -> `YearStyle::WithEra`.
    *   Else -> `YearStyle::Auto`.

---

## 6. FFI Export Layer (`icu_capi`)

Using Diplomat, we expose the options bag and the resolution function.

```rust
// ffi/capi/src/interop.rs (conceptual Diplomat bridge)
#[diplomat::bridge]
pub mod ffi {
    use crate::locale::ffi::ICU4XLocale;
    
    // Diplomat-compatible version of DateTimeFormatterOptions
    pub struct ICU4XDateTimeFormatterOptions { ... }

    #[diplomat::opaque]
    pub struct ICU4CIteropResolvedArgs {
        pub(crate) inner: icu_datetime::interop::Icu4cResolvedArgs,
    }

    impl ICU4CIteropResolvedArgs {
        pub fn skeleton(&self, writeable: &mut diplomat_runtime::DiplomatWriteable) -> DiplomatResult<(), ()> {
             // writes self.inner.skeleton to writeable
        }
        pub fn date_style(&self) -> i32 { self.inner.date_style as i32 }
        pub fn time_style(&self) -> i32 { self.inner.time_style as i32 }
    }

    pub fn resolve_icu4c_args(options: &ICU4XDateTimeFormatterOptions) -> Box<ICU4CIteropResolvedArgs> {
        Box::new(ICU4CIteropResolvedArgs {
            inner: icu_datetime::interop::resolve_icu4c_args(&options.into())
        })
    }
}
```

---

## 7. C/C++ Header-only Interop Layer (`ffi/icu4c_interop`)

This layer is distributed as C++ headers that clients include. It handles the dynamic switching and calls the respective libraries.

### Conceptual C++ API:

```cpp
#pragma once
#include "icu_capi.h" // ICU4X C API
#include <unicode/udat.h> // ICU4C C API
#include <string>

namespace icu_interop {

enum class Backend {
    ICU4X,
    ICU4C
};

class DateTimeFormatter {
public:
    DateTimeFormatter(const std::string& locale, const ICU4XDateTimeFormatterOptions& options, Backend backend) 
        : backend_(backend) {
        if (backend_ == Backend::ICU4X) {
            // Initialize ICU4X formatter using icu_capi
            icu4x_formatter_ = icu4x_datetime_formatter_create(locale.c_str(), &options);
        } else {
            // 1. Call ICU4X CAPI to resolve arguments
            auto resolved = icu4x_interop_resolve_icu4c_args(&options);
            
            // 2. Extract resolved args
            std::string skeleton = get_skeleton(resolved);
            UDateFormatStyle date_style = (UDateFormatStyle)icu4x_interop_resolved_args_date_style(resolved);
            UDateFormatStyle time_style = (UDateFormatStyle)icu4x_interop_resolved_args_time_style(resolved);
            
            // 3. Initialize ICU4C
            UErrorCode status = U_ZERO_ERROR;
            if (!skeleton.empty()) {
                // Use udatpg to get best pattern, then udat_open
                UDateTimePatternGenerator* pg = udatpg_open(locale.c_str(), &status);
                // ... udatpg_getBestPattern ...
                // ... udat_open ...
                udatpg_close(pg);
            } else {
                icu4c_formatter_ = udat_open(time_style, date_style, locale.c_str(), nullptr, 0, nullptr, 0, &status);
            }
            icu4x_interop_resolved_args_destroy(resolved);
        }
    }

    ~DateTimeFormatter() {
        if (icu4x_formatter_) icu4x_datetime_formatter_destroy(icu4x_formatter_);
        if (icu4c_formatter_) udat_close(icu4c_formatter_);
    }

    std::string format(const ICU4XDateTime& datetime) {
        if (backend_ == Backend::ICU4X) {
            // Format using ICU4X CAPI
            return icu4x_datetime_formatter_format(icu4x_formatter_, &datetime);
        } else {
            // 1. Convert ICU4X datetime input to UDate (milliseconds)
            UDate udate = convert_to_udate(datetime);
            // 2. Format using ICU4C
            UErrorCode status = U_ZERO_ERROR;
            UChar result[64];
            udat_format(icu4c_formatter_, udate, result, 64, nullptr, &status);
            return convert_to_std_string(result);
        }
    }

private:
    Backend backend_;
    ICU4XDateTimeFormatter* icu4x_formatter_ = nullptr;
    UDateFormat* icu4c_formatter_ = nullptr;
    
    UDate convert_to_udate(const ICU4XDateTime& datetime) {
        // Implementation uses icu_capi getters to extract year, month, day, etc.
        // and calculates epoch milliseconds.
    }
};

} // namespace icu_interop
```

---

## 8. Key Benefits of this Architecture

1.  **Dependency Isolation**: The Rust `icu_datetime` crate remains 100% pure Rust and does not need to link with `libicui18n.so`. This keeps Rust builds fast and simple.
2.  **Single Source of Truth for Options**: The complex logic of mapping ECMA-402 and ICU4X options to skeletons is written once in Rust, ensuring consistent behavior.
3.  **Flexible Linkage**: C++ clients can choose to link only ICU4X, only ICU4C, or both, as the switching logic is header-only and resolved at the C++ compile/link stage.
4.  **Zero-Cost Switching**: In production, if a client decides to compile only with ICU4X, the C++ compiler can optimize away the ICU4C branches.

---

## 9. Future Work: Raw Pattern Support

Currently, the ICU4X `DateTimeFormatter` is designed around semantic skeletons and pre-compiled data, and does not support formatting arbitrary raw pattern strings at runtime. To maintain symmetry across backends in the interop layer, raw pattern support (e.g., `pattern: Option<String>`) has been excluded from the unified `DateTimeFormatterOptions` bag.

Future work will investigate how to support raw patterns. This is challenging because it requires ICU4X to support arbitrary patterns. The following options will be evaluated:

1.  **On-the-fly Pattern Compilation**: Allow ICU4X to compile raw patterns at runtime. This would involve parsing the pattern string into a `Pattern` struct and then converting it to a `PackedPattern`, which requires memory allocation.
2.  **Polymorphic Formatter API**: Modify the interop API to return an enum or interface that can represent either a `DateTimeFormatter` (skeleton-based) or a `DateTimePatternFormatter` (pattern-based, if a separate pattern formatter is introduced in ICU4X).
3.  **Segregated Pattern Interop**: Keep the skeleton-based interop and pattern-based interop separate. Pattern-based interop could be moved to a separate header/module entirely, allowing clients who don't need raw patterns to avoid the overhead of pattern parsing code.
