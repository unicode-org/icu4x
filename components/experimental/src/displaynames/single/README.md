# Single Display Names

This module provides formatters for loading and rendering a single localized display name at a time. 

Unlike the `multi` module, which loads the entire database of names for a given type (e.g., all regions) into a `ZeroMap`, the `single` module loads only the data necessary for the specific subtag or identifier requested. This is highly optimized for binary size and memory usage in resource-constrained environments where only a few names are needed at runtime.

For usage examples, see the integration tests in `tests/displaynames/tests.rs`.

## Type Architecture

The following diagram shows the relationships between the owned and borrowed types in the `single` module, and their implementation of the `Writeable` trait:

```mermaid
classDiagram
    class Writeable {
        <<interface>>
        +write_to(&self, sink)
    }

    class LanguageDisplayNameOwned~M~ {
        -lid: LanguageIdentifier
        -options: DisplayNamesOptions
        +try_new(prefs, options, lid) Self
        +try_new_menu(prefs, options, lid) Self
        +as_borrowed(&self) LanguageDisplayName
    }
    class LanguageDisplayName {
        +write_to(&self, sink)
    }
    LanguageDisplayNameOwned ..> LanguageDisplayName : borrows to
    LanguageDisplayName ..|> Writeable : implements

    class RegionDisplayNameOwned {
        -subtag: Region
        +try_new(prefs, subtag) Self
        +as_borrowed(&self) RegionDisplayName
    }
    class RegionDisplayName {
        +write_to(&self, sink)
    }
    RegionDisplayNameOwned ..> RegionDisplayName : borrows to
    RegionDisplayName ..|> Writeable : implements

    class ScriptDisplayNameOwned {
        -subtag: Script
        +try_new(prefs, subtag) Self
        +as_borrowed(&self) ScriptDisplayName
    }
    class ScriptDisplayName {
        +write_to(&self, sink)
    }
    ScriptDisplayNameOwned ..> ScriptDisplayName : borrows to
    ScriptDisplayName ..|> Writeable : implements

    class VariantDisplayNameOwned {
        -subtag: Variant
        +try_new(prefs, subtag) Self
        +as_borrowed(&self) VariantDisplayName
    }
    class VariantDisplayName {
        +write_to(&self, sink)
    }
    VariantDisplayNameOwned ..> VariantDisplayName : borrows to
    VariantDisplayName ..|> Writeable : implements
```

## Formatters & Constructors

The module provides the following formatters, each with a borrowed version and an `Owned` version that holds the data lifetime. 

### Constructors and Value-Passing
All owned constructors take their target subtag or `LanguageIdentifier` **by value** because the owned struct needs to store the identifier for fallback purposes, and copying/moving these identifiers is highly efficient in ICU4X (using `TinyStr` under the hood).

*   **`LanguageDisplayName` / `LanguageDisplayNameOwned<M>`**: Formats a full `LanguageIdentifier` (language, script, region, and variants) into a localized string. It is generic over the display model (Standard/Dialect vs. Menu).
    *   `LanguageDisplayNameOwned::try_new(prefs, options, lid)`: Constructor for `Standard` and `Dialect` display styles. Takes `LanguageIdentifier` by value.
    *   `LanguageMenuDisplayNameOwned::try_new_menu(prefs, options, lid)`: Constructor for `Menu` display style. Takes `LanguageIdentifier` by value.
*   **`RegionDisplayName` / `RegionDisplayNameOwned`**: Formats a single `Region` subtag (e.g., `US` -> "United States").
    *   `RegionDisplayNameOwned::try_new(prefs, subtag)`: Constructor. Takes `Region` by value.
*   **`ScriptDisplayName` / `ScriptDisplayNameOwned`**: Formats a single `Script` subtag (e.g., `Latn` -> "Latin").
    *   `ScriptDisplayNameOwned::try_new(prefs, subtag)`: Constructor. Takes `Script` by value.
*   **`VariantDisplayName` / `VariantDisplayNameOwned`**: Formats a single `Variant` subtag (e.g., `valencia` -> "Valencian").
    *   `VariantDisplayNameOwned::try_new(prefs, subtag)`: Constructor. Takes `Variant` by value.

---

## Data Markers & Indexing

The `single` module uses the following data markers. Because it loads names for specific subtags at runtime, it utilizes a two-level indexing strategy combining the target **locale** (for translation) and **marker attributes** (for the subtag).

### 1. Subtag Display Names (Indexed by Locale + Marker Attribute)
These markers contain the localized name for a specific subtag. They are indexed by the **locale** of the translation (e.g., `en`) and a **marker attribute** containing the BCP-47 subtag string (e.g., `US`, `Latn`, `zh`).

*   **`LocaleNamesLanguageLongV1` / `LocaleNamesLanguageShortV1`**:
    *   *Attribute*: Language subtag (e.g., `en`, `zh`).
    *   *Description*: Contains a single string representing the localized long or short name for the language.
*   **`LocaleNamesScriptLongV1` / `LocaleNamesScriptShortV1`**:
    *   *Attribute*: Script subtag (e.g., `Latn`, `Hant`).
    *   *Description*: Contains a single string representing the localized long or short name for the script.
*   **`LocaleNamesRegionLongV1` / `LocaleNamesRegionShortV1`**:
    *   *Attribute*: Region subtag (e.g., `US`, `FR`, `001`).
    *   *Description*: Contains a single string representing the localized long or short name for the region.
*   **`LocaleNamesVariantLongV1`**:
    *   *Attribute*: Variant subtag (e.g., `valencia`).
    *   *Description*: Contains a single string representing the localized name for the variant.
*   **`LocaleNamesLanguageMenuLongV1`**:
    *   *Attribute*: Language subtag (e.g., `zh`).
    *   *Description*: Contains the split "core" and "extension" parts of the localized menu name (used for hierarchical dropdowns).

### 2. Formatting Patterns (Indexed by Locale Only)
These markers contain the patterns used to combine subtags. They are indexed by the **locale** only, as the patterns apply to all formatting operations for that language.

*   **`LocaleDisplayPatternV1`**:
    *   *Attribute*: None (indexed by locale only).
    *   *Contains*:
        *   `localePattern`: The pattern used to combine the base language name with qualifiers (e.g., `"{0} ({1})"`).
        *   `localeSeparator`: The separator used to join multiple qualifiers (e.g., `"{0}, {1}"`).

---

## Architecture & Design

The `single` module adheres to general ICU4X design principles to support `no_std` environments and minimize resource usage:

*   **Lazy Formatting (`Writeable`)**: Formatters do not allocate `String`s upon construction. Instead, they store the raw identifiers and loaded `DataPayload`s, deferring formatting until `Writeable::write_to` is called to write directly to the output sink.
*   **Stack Optimization**: We leverage standard ICU4X patterns like `DataPayloadOr` to store optional payloads (e.g., optional script/region) without the stack size overhead of `Option<DataPayload>`.
*   **Payload Erasing**: We use `ErasedMarker` to allow fields to hold either Long or Short variants of the payloads polymorphically at runtime, sharing the same underlying `VarZeroCow<'static, str>` data struct.
    *   *Type Alias*: `pub(crate) type ErasedDisplayNameMarker = icu_provider::marker::ErasedMarker<VarZeroCow<'static, str>>;`
*   **Zero-Allocation Interpolation**: We use the `icu_pattern` crate to interpolate the base language and qualifiers (joined by `localeSeparator`) directly into the output sink.

### Generic Owned Type with Shared Borrowed Type

To support both standard formatting and the specialized `LanguageDisplay::Menu` (which requires `LocaleNamesLanguageMenuLongV1` wrapping `MenuNameParts` instead of `str`), we use a single generic owned type `LanguageDisplayNameOwned<M>` that delegates its language payload type to a model trait, inspired by the `Model` generic in `TimeZoneInfo`.

#### 1. The Model Trait and Implementations
We define a `LanguageDisplayNameModel` trait with an associated type `LanguagePayload`. 

We implement this trait for two marker models:
*   **`models::Standard`**: Used for Standard and Dialect display styles. The `LanguagePayload` is `DataPayloadOr<ErasedDisplayNameMarker, ()>`.
*   **`models::Menu`**: Used for Menu display style. The `LanguagePayload` is `DataPayload<LocaleNamesLanguageMenuLongV1>`.

We define type aliases for the primary user-facing APIs:
*   `pub type LanguageDisplayNameOwned = LanguageDisplayNameOwned<models::Standard>;`
*   `pub type LanguageMenuDisplayNameOwned = LanguageDisplayNameOwned<models::Menu>;`

#### 2. The Generic Owned Struct
`LanguageDisplayNameOwned<M>` holds the `LanguageIdentifier`, `DisplayNamesOptions`, the generic `language_payload` (determined by `M`), optional `script_payload` and `region_payload` (both using `DataPayloadOr` with `ErasedDisplayNameMarker`), a vector of `variant_payloads` (using `ErasedDisplayNameMarker`), and the `pattern_payload` (using `LocaleDisplayPatternV1`).

> [!NOTE]
> **Allocation Papercut**: Storing `variant_payloads` in a `Vec` requires heap allocation, which prevents this struct from being strictly allocation-free in `no_std` environments without an allocator. 
> 
> *Follow-up (TODO #7825)*: We should explore using a stack-allocated collection like `SmallVec` (e.g., `SmallVec<[DataPayload<ErasedDisplayNameMarker>; 1]>`) to eliminate heap allocation for the vast majority of locales that have zero or one variant.

#### 3. The Shared Borrowed Struct
Both models resolve their payload differences and borrow to a single, non-generic **`LanguageDisplayName<'a>`** struct. 

This struct holds only cheap, borrowed references:
*   `base_name`: The resolved base language name (or "core" part for menu style) as a `&'a str`.
*   `menu_extension`: The optional menu extension part as an `Option<&'a str>`.
*   `script_name` and `region_name`: Optional resolved names as `Option<&'a str>`.
*   `variants`: A borrowed slice of variant payloads (`&'a [DataPayload<ErasedDisplayNameMarker>]`). We borrow the payloads directly rather than extracting a dereferenced list of `&'a str`s to avoid heap allocation during `as_borrowed()`. While a dereferenced list would be cleaner, this zero-allocation design is preferred and acceptable since variants are rarely present and not on a hot path.
*   `locale_pattern` and `locale_separator`: Borrowed patterns from the pattern payload.

#### 4. Resolution in `as_borrowed()`
The differences are resolved in the model-specific implementations of `as_borrowed()`:
*   For the **`Standard`** model: `base_name` is resolved from `language_payload` (falling back to the raw BCP-47 language subtag if missing), and `menu_extension` is `None`.
*   For the **`Menu`** model: `base_name` is resolved from `language_payload.get().core`, and `menu_extension` is `Some(language_payload.get().extension)`.

#### 5. Zero-Allocation Formatting Pipeline
In `Writeable::write_to` for `LanguageDisplayName<'a>`, we treat `menu_extension` (if present), `script_name`, `region_name`, and the resolved variant names as qualifiers. A stack-allocated `QualifiersWriteable` helper joins them using `locale_separator` directly into the sink, and `icu_pattern::Pattern::interpolate` combines `base_name` and the qualifiers into `locale_pattern` directly into the output sink without any heap allocation.

---

## UTS #35 Compliance

The formatting algorithm strictly follows **UTS #35 Part 3: Section 3 (Locale Display Names)**.

### 1. Locale Display Patterns (UTS #35 §3.1)
We load `LocaleDisplayPatternV1` which contains pre-parsed `DoublePlaceholderPattern`s from CLDR:
*   **`localePattern`**: Combines the language with qualifiers (e.g., `"{0} ({1})"`).
*   **`localeSeparator`**: Joins multiple qualifiers (e.g., `"{0}, {1}"`).

### 2. Dialect vs. Standard Handling (UTS #35 §3.1)
We support three language display modes via `LanguageDisplay` (matching ECMA-402):
*   **`LanguageDisplay::Dialect`**: The formatter first attempts to load a specific dialect name matching the identifier (e.g., `en-GB` -> "British English"). If not found, it falls back to `Standard`.
*   **`LanguageDisplay::Standard`**: The formatter bypasses dialect lookup and constructs the name from the base language and qualifiers (e.g., `en-GB` -> "English (United Kingdom)").
*   **`LanguageDisplay::Menu`**: The formatter loads menu-optimized names.

### 3. Variant Names (UTS #35 §3.15)
Localized variant names are loaded individually and appended to the qualifiers list, joined by the `localeSeparator`.

### 4. Menu Style Support (UTS #35 §3.1 & CLDR-19336)
We support `LanguageDisplay::Menu` using `LocaleNamesLanguageMenuLongV1` (which contains `MenuNameParts` with `core` and `extension` fields) for dropdown menus. The `core` part is displayed as the main language name, and the `extension` is joined with other qualifiers.

As per [CLDR-19336](https://unicode-org.atlassian.net/browse/CLDR-19336), some languages in CLDR have an `alt="menu"` name. During datagen/loading, if an `alt="menu"` name is present, we construct the `MenuNameParts` payload using that string as the `core` part, with an empty `extension` part.

---

## Limitations & Future Work

The following features defined in UTS #35 are currently not supported and are planned for future releases:

1.  **Locale Extension Keywords (UTS #35 §3.2)**: Formatting Unicode extension keys and types (e.g., `-u-ca-gregory` -> "Calendar: Gregorian") using `localeKeyTypePattern`.
