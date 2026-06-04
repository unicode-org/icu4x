# Design Doc: Datetime Interop Layer (ICU4X / ICU4C / ECMA)

## Status: Draft (Names and locations are subject to bikeshedding)
**Author:** AI Agent (Gemini) working with @sffc

> [!NOTE]
> All names, FFI paths, C++ class structures, and Rust module locations proposed in this document are tentative and subject to revision (bikeshedding) during implementation.

---

## 1. Background and Motivation

ICU4X provides a modern, modular, and lightweight internationalization library in Rust. ICU4C is the established C/C++ internationalization library. During transition phases, or in environments where system-provided libraries are preferred, clients may want to choose between ICU4X (Rust) and ICU4C (C/C++) at compile-time or runtime.

Furthermore, clients targeting web environments often need to align with **ECMA-402 (Intl.DateTimeFormat)** options.

This document specifies a **Datetime Interop Layer** based on a **catchall options bag** and a **decoupled architecture** that avoids linking ICU4C into the Rust codebase.

The design aims to:
- Define a unified options bag covering ICU4X, ICU4C, and ECMA-402 options.
- Keep the Rust codebase (`icu_datetime`) free of ICU4C dependencies.
- Perform argument resolving (mapping options to ICU4C skeletons/styles) in Rust.
- Export these helpers via `icu_capi` using **Diplomat** (ICU4X's FFI binding generation tool).
- Implement the actual switching and ICU4C calls in a header-only C/C++ layer (`ffi/icu4c_interop`).

### 1.1. Key Benefits

*   **Dependency Isolation**: The Rust `icu_datetime` crate remains 100% pure Rust and does not need to link with `libicui18n.so`. This keeps Rust builds fast and simple.
*   **Single Source of Truth for Options**: The complex logic of mapping ECMA-402 and ICU4X options to skeletons is written once in Rust, ensuring consistent behavior.
*   **Flexible Linkage**: C++ clients can choose to link only ICU4X, only ICU4C, or both, as the switching logic is header-only and resolved at the C++ compile/link stage.
*   **Zero-Cost Switching**: In production, if a client decides to compile only with ICU4X, the C++ compiler can optimize away the ICU4C branches.

---

## 2. Architecture Overview

The interop layer is split across three boundaries to maintain strict dependency separation:

```mermaid
graph TD
    Client["C/C++ Client"] --> InteropCPP["ffi/icu4c_interop (C++)"]

    subgraph FFI ["FFI Boundary (icu_capi)"]
        CAPI_FT["DateTimeFormatter FFI"]
        CAPI_Interop["Icu4cResolvedArgs FFI"]
    end

    subgraph Rust ["Rust Library (icu_datetime)"]
        ICU4X_Rust["DateTimeFormatter (Rust)"]
        Rust_Interop["icu_datetime::interop"]
    end

    subgraph ICU4C_Lib ["ICU4C Library"]
        ICU4C_C["ICU4C (udat.h)"]
    end

    InteropCPP -->|ICU4X: Create & Format| CAPI_FT
    CAPI_FT --> ICU4X_Rust

    InteropCPP -->|ICU4C: Resolve Options| CAPI_Interop
    CAPI_Interop --> Rust_Interop

    InteropCPP -->|ICU4C: Format| ICU4C_C
```

1.  **`icu_datetime::interop` (Rust)**: Contains `DateTimeFormatterOptions` (the catchall options bag) and the resolution logic to map these options to ICU4C-compatible arguments (skeletons or styles), without calling ICU4C.
2.  **`icu_capi` (Rust/FFI)**: Exposes the options bag and the resolution function to C/C++ using Diplomat.
3.  **`ffi/icu4c_interop` (C/C++ Header-only)**: The integration point for clients. It includes `icu_capi` headers and system ICU4C headers (`unicode/udat.h`). It contains the logic to switch between backends and call `libicui18n.so` or `libicu_capi.so` accordingly.

---

## 3. Main Rust Crate Components (`icu_datetime::interop`)

This module in the `icu_datetime` crate contains the core Rust structures and logic for the interop layer, serving as the bridge between the catchall configuration and the backend-specific formatters.

### 3.1. Module Layout and Types

The module exposes the following key components:

-   **`DateTimeFormatterOptions` (Struct)**:
    *   The unified catchall options bag that aggregates options from ECMA-402, ICU4X, and ICU4C. All fields are optional.
    *   Exposes a method (e.g., `to_fieldset`) that resolves the options to an ICU4X `CompositeFieldSet` (ICU4X's internal representation of selected date/time fields and their display widths).
    *   *Details of fields and the mapping algorithm are documented in [Options and Resolution Config](options_config.md#1-the-catchall-options-bag).*
-   **`Icu4cResolvedArgs` (Struct)**:
    *   An intermediate structure that holds the resolved arguments required to initialize an ICU4C formatter (skeleton, date style, and time style) after precedence rules have been applied.
    *   Exposes a constructor (e.g., `resolve`) that accepts `DateTimeFormatterOptions` and returns `Icu4cResolvedArgs`.
    *   *Details and the resolution algorithm are documented in [Options and Resolution Config](options_config.md#21-resolved-output-rust-struct).*

### 3.2. Option Resolution Precedence (Summary)

The resolution logic maps the unified options to backend-specific targets. It follows a strict order of precedence to resolve conflicts:
1.  **Explicit Skeleton**: If `skeleton` is set, it overrides everything else (used directly for ICU4C).
2.  **High-Level Styles**: If `date_style` or `time_style` are set, they override individual field options.
3.  **Individual Fields**: If neither skeletons nor styles are set, individual field options (ECMA-402 or ICU4X-specific) are used. ECMA-style options take precedence over ICU4X-specific options if both are present.

---

## 4. FFI Export Crate (`icu_capi`)

Using Diplomat, the interop layer exposes the following C-compatible interface:

-   **`ffi::DateTimeFormatterOptions` Struct**: A C-compatible version of the catchall options bag, mapping to Rust's `icu_datetime::interop::DateTimeFormatterOptions`.
-   **`ffi::Icu4cResolvedArgs` Opaque Type**: A thin wrapper around Rust's `icu_datetime::interop::Icu4cResolvedArgs`.
    *   Exposes a constructor `resolve` that accepts `ffi::DateTimeFormatterOptions`, calls the Rust resolution logic, and returns the wrapped `ffi::Icu4cResolvedArgs`.
    *   Exposes a method to write the resolved `skeleton` into a `DiplomatWriteable` (a C-compatible string buffer).
    *   Exposes methods to get the resolved `date_style` and `time_style` as integers.
-   **`ffi::DateTimeFormatter` Constructor**: A new constructor `create_from_interop_options` is added to the existing `ffi::DateTimeFormatter` in `icu_capi`. It accepts `ffi::DateTimeFormatterOptions`, resolves it to a `CompositeFieldSet` internally, and returns a `Box<ffi::DateTimeFormatter>`.

---

## 5. Header-only C++ Integration Layer (`ffi/icu4c_interop`)

A C++ wrapper (e.g., `icu_interop::DateTimeFormatter`) is provided as a header-only library. It manages the switching logic and calls the appropriate underlying library.

### 5.1. Initialization Flow

1.  **Switch Backend**: The mechanism for selecting the backend (ICU4X vs. ICU4C) is an open question. Possible approaches include:
    *   **Runtime Selection**: The constructor accepts a `Backend` enum. This allows dynamic switching but may retain code size overhead for both backends.
    *   **Compile-time Macro**: Selected via preprocessor definitions (e.g., `-DICU_INTEROP_BACKEND_ICU4X`), guaranteeing zero overhead for the unused backend.
    *   **Template Parameter**: The class is templated on the backend (e.g., `DateTimeFormatter<Backend::ICU4X>`).
2.  **ICU4X Path**:
    *   Calls `ffi::DateTimeFormatter::create_from_interop_options` using the provided options bag to obtain a `ffi::DateTimeFormatter`.
3.  **ICU4C Path**:
    *   Calls `ffi::Icu4cResolvedArgs::resolve` using the provided options bag to get the resolved skeleton or styles, which are then used to construct the ICU4C `UDateFormat` formatter.

### 5.2. Formatting Flow

1.  **ICU4X Path**:
    *   Calls the formatting function on `ffi::DateTimeFormatter` with the input.
2.  **ICU4C Path**:
    *   Converts the FFI input object (e.g., `ffi::DateTime`) to the appropriate ICU4C type (e.g., `UDate` or `UCalendar`) and formats it using the ICU4C formatter.

## 6. Input Data Type Mapping

A key difference between ICU4C and ICU4X is how they handle time zones and input representation:
*   **ICU4C** historically accepts `UDate` (double, milliseconds since epoch) and performs time zone conversions internally using its own copy of the Time Zone Database (TZDB).
*   **ICU4X** defers time zone database conversions to third-party libraries. Its formatters expect pre-resolved, structured "Temporal-like" types (such as `Date`, `DateTime`, and `ZonedDateTime`) where calendar arithmetic and time zone offsets have already been applied.

To maintain backend symmetry and leverage ICU4X's modern design, the interop layer will **only accept ICU4X input types** (or thin C++ wrappers around them).

### 6.1. Conversion Path
*   **ICU4X Path**: Direct pass-through of ICU4X structured types to the ICU4X formatter.
*   **ICU4C Path**: The C++ interop layer must convert the structured ICU4X input types into ICU4C-compatible representations (e.g., extracting the fields to populate a `UCalendar`, or calculating a local `UDate` if necessary) before calling `udat_format`.

---

## 7. Error Handling

Since both ICU4X and ICU4C initialization can fail (e.g., due to missing locale data or invalid options), the C++ interop layer must propagate these errors:
*   **Initialization Failures**: The C++ constructor or initialization method should return a status indicator (e.g., a boolean or an error code) reflecting whether the underlying backend formatter was successfully created.
*   **FFI Error Propagation**: Errors returned from the Rust FFI (via Diplomat's result wrappers) should be mapped to the C++ interface, avoiding throwing C++ exceptions to ensure compatibility with environments where exceptions are disabled.

---

## 8. Future Work: Raw Pattern Support

Currently, the ICU4X `DateTimeFormatter` is designed around semantic skeletons and pre-compiled data, and does not support formatting arbitrary raw pattern strings at runtime. To maintain symmetry across backends in the interop layer, raw pattern support (e.g., `pattern: Option<String>`) has been excluded from the unified `DateTimeFormatterOptions` bag.

Future work will investigate how to support raw patterns. This is challenging because it requires ICU4X to support arbitrary patterns. The following options will be evaluated:

1.  **On-the-fly Pattern Compilation**: Allow ICU4X to compile raw patterns at runtime. This would involve parsing the pattern string into a `Pattern` struct and then converting it to a `PackedPattern`, which requires memory allocation.
2.  **Polymorphic Formatter API**: Modify the interop API to return an enum or interface that can represent either a `DateTimeFormatter` (skeleton-based) or a `DateTimePatternFormatter` (pattern-based, if a separate pattern formatter is introduced in ICU4X).
3.  **Segregated Pattern Interop**: Keep the skeleton-based interop and pattern-based interop separate. Pattern-based interop could be moved to a separate header/module entirely, allowing clients who don't need raw patterns to avoid the overhead of pattern parsing code.
