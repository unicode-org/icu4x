# Changelog

## icu4x 1.5.x


- `icu_calendar`
  - (1.5.1) Fix Japanese calendar Gregorian era year 0 (https://github.com/unicode-org/icu4x/issues/4968)
  - (1.5.2) Enforce C,packed, not just packed, on ULE types, fixing for incoming changes to `repr(Rust)` (https://github.com/unicode-org/icu4x/pull/5049)
- `icu_datetime`
  - (1.5.1) Fix incorrect assertion in week-of-year formatting (https://github.com/unicode-org/icu4x/issues/4977)
- `icu_casemap`
  - (1.5.1) Enforce C,packed, not just packed, on ULE types, fixing for incoming changes to `repr(Rust)` (https://github.com/unicode-org/icu4x/pull/5049)
- `icu_capi`
  - (1.5.1) Fix situations in which `libc_alloc` is specified as a dependency (https://github.com/unicode-org/icu4x/pull/5119)
- `icu_properties`
  - (1.5.1) Enforce C,packed, not just packed, on ULE types, fixing for incoming changes to `repr(Rust)` (https://github.com/unicode-org/icu4x/pull/5049)
- `zerovec`
  - (0.10.3) Fix size regression by making `twox-hash` dep `no_std` (https://github.com/unicode-org/icu4x/pull/5007)
  - (0.10.3) Enforce C,packed, not just packed, on ULE types, fixing for incoming changes to `repr(Rust)` (https://github.com/unicode-org/icu4x/pull/5049)
  - (0.10.4) Enforce C,packed on OptionVarULE (https://github.com/unicode-org/icu4x/pull/5143)
- `zerovec_derive`
  - (0.10.3) Enforce C,packed, not just packed, on ULE types, fixing for incoming changes to `repr(Rust)` (https://github.com/unicode-org/icu4x/pull/5049)

## icu4x 1.5 (May 28, 2024)

- Components
  - General
    - Compiled data updated to CLDR 45 and ICU 75 (unicode-org#4782)
  - `icu_calendar`
    - Fix duration offsetting and negative-year bugs in several calendars including Chinese, Islamic, Coptic, Ethiopian, and Hebrew (#4904)
    - Improved approximation for Persian calendrical calculations (https://github.com/unicode-org/icu4x/issues/4713)
    - Fix weekday calculations in negative ISO years (https://github.com/unicode-org/icu4x/pull/4894)
    - New `DateTime::local_unix_epoch()` convenience constructor (https://github.com/unicode-org/icu4x/pull/4479)
    - Add caching for all islamic calendars (https://github.com/unicode-org/icu4x/pull/4785)
    - Add caching for chinese based calendars (https://github.com/unicode-org/icu4x/pull/4411, https://github.com/unicode-org/icu4x/pull/4468)
    - Switch Hebrew to faster keviyah/Four Gates calculations (https://github.com/unicode-org/icu4x/pull/4504)
    - Replace 2820-year with 33-year cycle in Persian calendar, with override table (https://github.com/unicode-org/icu4x/pull/4770, https://github.com/unicode-org/icu4x/pull/4775, https://github.com/unicode-org/icu4x/pull/4796)
    - Fix bugs in several calendars with new continuity test (https://github.com/unicode-org/icu4x/pull/4904)
    - Fix year 2319 in the Chinese calendar (https://github.com/unicode-org/icu4x/pull/4929)
    - Fix ISO weekday calculations in negative years (https://github.com/unicode-org/icu4x/pull/4894)
  - `icu_collections`
    - Switch from `wasmer` to `wasmi` in `icu_codepointtrie_builder` (https://github.com/unicode-org/icu4x/pull/4621)
  - `icu_normalizer`
    - Make UTS 46 normalization non-experimental (#4712)
  - `icu_datetime`
    - Experimental "neo" datetime formatter with support for semantic skeleta and fine-grained data slicing (https://github.com/unicode-org/icu4x/issues/1317, https://github.com/unicode-org/icu4x/issues/3347)
    - `Writeable` and `Display` implementations now don't return `fmt::Error`s that don't originate from the `fmt::Write` anymore (#4732, #4851, #4863)
    - Make `CldrCalendar` trait sealed except with experimental feature (https://github.com/unicode-org/icu4x/pull/4392)
    - `FormattedDateTime` and `FormattedZonedDateTime` now implement `Clone` and `Copy` (https://github.com/unicode-org/icu4x/pull/4476)
  - `icu_experimental`
    - New home for all experimental components. This supersedes the published `icu_compactdecimal`, `icu_displaynames`, `icu_relativetime`, `icu_transliterate`, and `icu_unicodeset_parse` crates (#4564)
    - New experimental component `personnames` (#4050)
    - New experimental component `dimension`
      - Added `CurrencyFormatter`, which can format any currency based on the locale and the width (short and narrow are supported for now).
    - New experimental component `units` (#4605)
      - Added `UnitsConverter`, which can convert any single or compound units (such as `meter` and `square-meter-per-second`) to any compatible single or compound units.
  - `icu_locid`
    - Added `total_cmp` functions to `Locale` and other types to make them easier to use in `BTreeSet` (https://github.com/unicode-org/icu4x/pull/4608)
  - `icu_locid_transform`
    - Add `LocaleExpander::minimize_favor_script` (https://github.com/unicode-org/icu4x/pull/4752)
  - `icu_plurals`
    - Added support for `CompactDecimal` (https://github.com/unicode-org/icu4x/pull/4828)
  - `icu_properties`
    - Add `Aran` script code (https://github.com/unicode-org/icu4x/pull/4426)
    - Mark additional constructors as `const` (https://github.com/unicode-org/icu4x/pull/4584, https://github.com/unicode-org/icu4x/pull/4574)
    - Implement Joining_Type property (https://github.com/unicode-org/icu4x/pull/4599)
    - Implement Hangul_Syllable_Type property (https://github.com/unicode-org/icu4x/pull/4885)
  - `icu_segmenter`
    - Fixed line segmenter, using Unicode 15.0 (https://github.com/unicode-org/icu4x/pull/4389)
    - Fixed grapheme cluster segmenter and updated it to Unicode 15.1 (https://github.com/unicode-org/icu4x/pull/4536)
    - Updated sentence segmenter rules to Unicode 15.1 (https://github.com/unicode-org/icu4x/pull/4625)
    - Updated word segmenter rules to Unicode 15.1 (https://github.com/unicode-org/icu4x/pull/4625) 
    - Fixed `word_type()` and `is_word_like()` on `WordBreakIterator` for the last segment in complex script (https://github.com/unicode-org/icu4x/pull/4903)
  - `icu_timezone`
    - Added `TimeZoneIdMapper` to replace `IanaToBcp47Mapper` (https://github.com/unicode-org/icu4x/pull/4774)
- Data model and providers
  - `icu_datagen`
    - Add new API structure to `DatagenDriver` to better express the different aspects of `FallbackMode` (https://github.com/unicode-org/icu4x/issues/4629)
    - Datagen shows elapsed time for keys that are slow to generate (https://github.com/unicode-org/icu4x/pull/4469)
    - Datagen performance improvement by caching supported locales (https://github.com/unicode-org/icu4x/pull/4470)
    - Never use fallback for baked segmentation data (https://github.com/unicode-org/icu4x/pull/4510)
    - Propagate extension keywords and auxiliary keys to explicit locales (https://github.com/unicode-org/icu4x/pull/4533)
    - Add baked data macros to generate `IterableDataProvider` (#4800, #4868)
    - Add an option to generate data from an existing `DataProvider`, instead of from sources (#4814)
    - Implement `DeduplicationStrategy::RetainBaseLanguages` (https://github.com/unicode-org/icu4x/pull/4836)
  - `icu_provider`
    - (Small breakage) `DataPayload::new_owned()` is no longer `const`, this was a mistake (https://github.com/unicode-org/icu4x/pull/4456)
    - Add `NeverMarker` to allow for DataProvider bounds that never return data (https://github.com/unicode-org/icu4x/issues/4186)
    - Add `BoundProvider` to allow temporal separation between key and request (https://github.com/unicode-org/icu4x/pull/4877)
    - Add `DataPayloadOr` for more efficient memory layout to reduce stack size (https://github.com/unicode-org/icu4x/pull/4463)
  - `icu_provider_blob`
    - Blob v2 no longer allocates (https://github.com/unicode-org/icu4x/pull/4383)
- FFI:
    - All languages
      - Correctly handle invalid UTF8 (https://github.com/unicode-org/icu4x/pull/4353)
      - Correctly handle chars (https://github.com/unicode-org/icu4x/pull/4349)
      - Add `day_of_year` getter (https://github.com/unicode-org/icu4x/issues/4891)
      - Adding panic-handler and allocator features to icu_capi (#4516)
    - JS
      - Fixed a bug where slice length is computed incorrectly (https://github.com/rust-diplomat/diplomat/pull/372)
      - Changed file extensions for better compatibility with nodejs modules (https://github.com/rust-diplomat/diplomat/pull/387)
    - C++
      - Fixed a bug where a result header defines a struct that shadows the class' name  (https://github.com/rust-diplomat/diplomat/pull/394)
      - Add `explicit` keyword to internal constructors (https://github.com/rust-diplomat/diplomat/pull/386)
      - Small breakage: some functions that used to return `diplomat::Result<T, std::monostate>` now return `std::optional<T>` (#4635)
    - `icu_harfbuzz`
      - Switch to harfbuzz funcs (#4794)
- Utilities
    - `calendrical_calculations`:
        - Add Keviyah/Four Gates based optimized calculations module for the Hebrew calendar. (https://github.com/unicode-org/icu4x/pull/4504)
        - Expose `Hebrew` as a unit struct, add `Date::try_new_hebrew_date()`, `DateTime::try_new_hebrew_datetime()`. (https://github.com/unicode-org/icu4x/pulls/4532)
        - Deprecate `Hebrew::new_always_precomputing()`, `Date::try_new_hebrew_date_with_calendar()`, `DateTime::try_new_hebrew_datetime_with_calendar()`. The new implementation of the Hebrew calendar is faster and we do not need APIs for precomputation. (https://github.com/unicode-org/icu4x/pulls/4532)
    - `databake`
        - Add `impl Bake for PhantomData<T>` (https://github.com/unicode-org/icu4x/pull/4663)
    - `fixed_decimal`
        - Changed type of compact exponent from `i16` to `u8` (https://github.com/unicode-org/icu4x/pull/4828)
        - Stabilize rounding to increment methods for `FixedDecimal` (https://github.com/unicode-org/icu4x/pull/4578)
    - `icu_pattern`
        - Complete re-write of crate with support for multiple backends and better integration with ZeroVec and Writeable (https://github.com/unicode-org/icu4x/pull/4610)
    - `litemap`
        - Add `impl IntoIterator for LiteMap` by splitting `StoreIterableMut` trait (https://github.com/unicode-org/icu4x/pull/4359)
    - `yoke`
        - Remove `StableDeref` bound from `Yoke<Y, Option<C>>` methods (https://github.com/unicode-org/icu4x/pull/4457)
        - Added `CartableOptionPointer` and function to convert from `Yoke<Y, Option<C>>` (https://github.com/unicode-org/icu4x/pull/4449)\
    - `zerofrom`
        - Support `?Sized` type parameters which must be `Sized` to implement `ZeroFrom` (https://github.com/unicode-org/icu4x/pull/4657)
    - `zerotrie`
        - Add `as_borrowed_slice` and `AsRef` impl (https://github.com/unicode-org/icu4x/pull/4381)
        - Add `ZeroTrieSimpleAsciiCursor` for manual iteration (https://github.com/unicode-org/icu4x/pull/4383, https://github.com/unicode-org/icu4x/pull/4725)
        - Increase bound of `p` to solve more perfect hash functions; _might_ break serialized ZeroTriePerfectHash from previous versions (https://github.com/unicode-org/icu4x/pull/4888)
    - `zerovec`
        - Change `ZeroHashMap` to use `twox-hash` (https://github.com/unicode-org/icu4x/pull/4592)
        - Add a niche to `ZeroVec` to reduce memory usage (https://github.com/unicode-org/icu4x/pull/4491)
        - Add `ZeroVec::take_last` and `::take_first` (https://github.com/unicode-org/icu4x/pull/4651)
    - `writeable`
        - Add `TryWriteable` for fallibility (https://github.com/unicode-org/icu4x/pull/4787)
        - Add `writeable_cmp_bytes` for more efficient comparison (https://github.com/unicode-org/icu4x/pull/4402)

## icu4x 1.4.x

- [Remove icu_datagen's dep on `fractional`](https://github.com/unicode-org/icu4x/pull/4472)
   - `icu_datagen@1.4.1`
 - Fix normalization of character whose decomposition contains more than one starter and ends with a non-starter followed by a non-starter
   with a lower Canonical Combining Class than the last character of the decomposition. (https://github.com/unicode-org/icu4x/pull/4530)
   - `icu_normalizer@1.4.1`
 - Stricter version dependency on data crates
   - `icu_properties@1.4.2`, `icu_normalizer@1.4.3`, `icu_properties_data@1.4.1`
 - Enforce C,packed, not just packed, on ULE types, fixing for incoming changes to `repr(Rust)` (https://github.com/unicode-org/icu4x/pull/5049)
   - `icu_casemap@1.4.1`, `icu_properties@1.4.3`
   - A full fix also needs `zerovec@0.10.3`,`zerovec_derive@0.10.3`

## icu4x 1.4 (Nov 16, 2023)

- General
  - MSRV is now 1.67

- Components
    - Compiled data updated to CLDR 44 and ICU 74 (https://github.com/unicode-org/icu4x/pull/4245)
    - `icu_calendar`
      - Expose if a date is in a leap year (https://github.com/unicode-org/icu4x/pull/4273)
    - `icu_locid`
      - Canonicalize transform extensions to lowercase (https://github.com/unicode-org/icu4x/pull/4134)
    - `icu_plurals`
        - Experimental `PluralRulesWithRanges` for selecting the plural category for ranges (https://github.com/unicode-org/icu4x/issues/3012)
    - `icu_properties`
        - Support Indic_Syllabic_Category in icu_properties. (https://github.com/unicode-org/icu4x/pull/4176)
        - Add enum constants for new Unicode 15.1 Line_Break properties. (https://github.com/unicode-org/icu4x/issues/4132)
    - `icu_segmenter`
        - Fix Unicode 15.0 sentence segmentation (https://github.com/unicode-org/icu4x/pull/4213)
    - `icu_unicodeset_parse`
        - Add support to parse UAX#29 properties (https://github.com/unicode-org/icu4x/pull/4175)
- Data model and providers
    - `icu_provider`
        - Implement Any/BufferProvider for some smart pointers (https://github.com/unicode-org/icu4x/pull/4255) 
    - `icu_provider_blob`
      - New V2 serialization schema (https://github.com/unicode-org/icu4x/pull/4207)
    - `icu_datagen`
        - New CLI option `--format=blob2` (https://github.com/unicode-org/icu4x/pull/4207)
        - CLDR 44 compatibility fixes (https://github.com/unicode-org/icu4x/pull/4134, https://github.com/unicode-org/icu4x/pull/4156, https://github.com/unicode-org/icu4x/pull/4158)
        - Fix `supported_locales` for collator keys (https://github.com/unicode-org/icu4x/pull/4169)
        - CLI: Fix behavior of `--segmenter-lstm-root` such that it does not override `icuexportdata-root` (https://github.com/unicode-org/icu4x/pull/4277)
        - Make datagen work without `CARGO_PKG_RUST_VERSION` environment variable (https://github.com/unicode-org/icu4x/pull/4292)
- Utilities
    - `databake`
        - Add implementations for `HashSet`, `HashMap`, `BTreeSet`, `BTreeMap` (https://github.com/unicode-org/icu4x/pull/4268, https://github.com/unicode-org/icu4x/pull/4274, https://github.com/unicode-org/icu4x/pull/4295)  
        - Improvements to `databake::test_bake!()` (https://github.com/unicode-org/icu4x/pull/4182)
    - `fixed_decimal`
        - Experimental rounding increment support (https://github.com/unicode-org/icu4x/pull/4219, https://github.com/unicode-org/icu4x/pull/4246)
    - `litemap`
        - Implement `databake::Bake` on `LiteMap` (https://github.com/unicode-org/icu4x/pull/4275)
    - `tinystr`
        - Better Debug impl for UnvalidatedTinyAsciiStr (https://github.com/unicode-org/icu4x/pull/4189)
    - `zerovec`
        - Fix indexing bug in `ZeroMap2d::insert` (https://github.com/unicode-org/icu4x/pull/4160)

## icu4x 1.3.x

Some crates received additional 1.3.x patch releases:

- [Fix #4165, locale "vi" not working in Collator](https://github.com/unicode-org/icu4x/issues/4165):
    - `icu_collator@1.3.3`
    - `icu_collator_data@1.3.3`
- [Fix #4232, some locale data being improperly deduplicated](https://github.com/unicode-org/icu4x/issues/4232):
    - `icu_compactdecimal_data@1.3.4`
    - `icu_datagen@1.3.3`
    - `icu_datetime_data@1.3.4`
    - `icu_decimal_data@1.3.4`
    - `icu_displaynames_data@1.3.4`
    - `icu_properties_data@1.3.4`

## icu4x 1.3.2 (Oct 4, 2023)

1.3.2 was released to clean up the range dependency on `zerovec` deliberately introduced in 1.3.0. It includes all previous 1.3.x changes

## icu4x 1.3.1

A subset of crates received a 1.3.1 patch release, to incorporate documentation fixes (https://github.com/unicode-org/icu4x/pull/4096, https://github.com/unicode-org/icu4x/pull/4097, https://github.com/unicode-org/icu4x/pull/4099, https://github.com/unicode-org/icu4x/pull/4101). These crates were: `icu_calendar`, `icu_casemap`, `icu_datetime`, `icu_locid_transform`, `icu_provider`.

## icu4x 1.3 (Sep 25, 2023)

- General
  - All updated crates:
    - License updated to the [new Unicode license](https://www.unicode.org/license.txt)
    - MSRV is now 1.66
  - Components now have the option to use `compiled_data`, an efficient default set of data baked in to the component. This is now recommended over using a data provider unless you have specific needs driving it.
- Data model and providers
  - `icu_provider`:
    - Make `DataPayload` constructible from `&'static M::Yokeable` (https://github.com/unicode-org/icu4x/pull/3467)
    - Add `ExportableDataPayload::eq_dyn` (https://github.com/unicode-org/icu4x/pull/3639)
    - Better docs (https://github.com/unicode-org/icu4x/pull/3740, https://github.com/unicode-org/icu4x/pull/3742)
    - Moving fallback options into `icu_provider` (https://github.com/unicode-org/icu4x/pull/3651)
    - Add `DataPayload::dynamic_cast_mut` (https://github.com/unicode-org/icu4x/pull/3952)
    - (experimental) Add `AuxiliaryKey` to `DataLocale` (https://github.com/unicode-org/icu4x/pull/3872)
  - `icu_datagen`:
    - Completely revamped API (https://github.com/unicode-org/icu4x/pull/3705, https://github.com/unicode-org/icu4x/pull/3861, https://github.com/unicode-org/icu4x/pull/3951, https://github.com/unicode-org/icu4x/pull/4008, https://github.com/unicode-org/icu4x/pull/4041, https://github.com/unicode-org/icu4x/pull/3669)
        - The old API is still available behind the `legacy_api` default Cargo feature
    - Updated baked data output to be usable by `compiled_data` (https://github.com/unicode-org/icu4x/pull/3449, https://github.com/unicode-org/icu4x/pull/3493, https://github.com/unicode-org/icu4x/pull/3549, https://github.com/unicode-org/icu4x/pull/3500, https://github.com/unicode-org/icu4x/pull/3847)
    - Add recommended locale set and expand regions in datagen (https://github.com/unicode-org/icu4x/pull/3586)
    - Make datagen faster by caching more things (https://github.com/unicode-org/icu4x/pull/3625)
    - Consume CLDR-JSON resources keyed with default script (https://github.com/unicode-org/icu4x/pull/3772, https://github.com/unicode-org/icu4x/pull/3786)
    - (cli) Warn for `--locales all` (https://github.com/unicode-org/icu4x/pull/3691)
  - `icu_provider_adapters`: 
    - Deprecated `LocaleFallbacker`, use through `icu_locid_transform`
  - `icu_provider_blob`:
    - Returning `ExtraneousLocale` in `BlobDataProvider` (https://github.com/unicode-org/icu4x/pull/3562)
    - Fix empty keys in `BlobDataProvider` (https://github.com/unicode-org/icu4x/pull/3551) 
  - `icu_provider_fs`:
    - Correct error types for `icu_provider_fs` (https://github.com/unicode-org/icu4x/pull/3682) 
  - `icu_testdata`:
    - This crate has been superseded by `compiled_data` and is now deprecated.
    - Data for new components will not be added, and it will not be updated for ICU4X 2.0.
- Components:
  - Cross component:
    - All component crates now have a default `compiled_data` feature that enables constructors that do not require data providers, instead using data compiled into the library
  - `icu_calendar`
    - Support for new non-Gregorian calendars: Persian (Solar Hijri), ROC, Hebrew, Chinese, Korean (Dangi), and Islamic (civil, observational, tabular, and Umm al-Qura). Lunar calendars are not yet fully optimized.
    - Add `Ord` and `PartialOrd` to `icu_calendar` types (https://github.com/unicode-org/icu4x/pull/3468)
    - Add cyclic year to `FormattableYear` (https://github.com/unicode-org/icu4x/pull/3581)
  - `icu_casemap`
    - Newly stabilized crate
  - `icu_collator`
    - No additional changes
  - `icu_collections`
    - Fixed JSON serialization of unpaired surrogates (https://github.com/unicode-org/icu4x/pull/3892)
  - `icu_datetime`
    - Formatting for the new calendars added in `icu_calendar`
      - Includes preview formatting for Chinese and Korean with cyclic years and leap months
    - Hardening of edge cases surrounding the `Calendar` trait
    - `TimeZoneFormatter::load_*_format` functions:
      - Those with a provider argument gain an equivalent `TimeZoneFormatter::load_*_format` gated by `compiled_data`
      - Those without a provider argument renamed to `TimeZoneFormatter::include_*_format` with deprecation warning
  - `icu_decimal`
    - No additional changes
  - `icu_list`
    - No additional changes
  - `icu_locid`
    - Declarative macros are now re-exported from their own modules; old exports are deprecated; for example, `use icu::locid::extensions::unicode::value` now works, instead of `use icu::locid::extensions_unicode_value`
  - `icu_locid_transform`
    - New home of `LocaleFallbacker` and `fallback` module previously found in `icu_provider_adapters`
    - New `LocaleDirectionality` to access the right-to-left status of locales
  - `icu_normalizer`
    - No additional changes
  - `icu_plurals`
    - No additional changes
  - `icu_properties`
    - Compiled data functions added to `sets` and `maps` without `load_` prefix: `icu::properties::sets::basic_emoji()`
  - `icu_segmenter`
    - Algorithmic bug fixes such as https://github.com/unicode-org/icu4x/pull/3392
  - `icu_timezone`
    - New `IanaToBcp47Mapper` for converting between IANA time zone names and BCP-47 time zone names (https://github.com/unicode-org/icu4x/pull/2909)
 - Utils:
    - `calendrical_calculations`: New crate: 0.1.0
    - `crlify`: 1.0.2 -> 1.0.3
    - `databake` and `databake_derive`: 1.0.5 -> 1.0.6
      - `::` paths changed (https://github.com/unicode-org/icu4x/pull/3450)
    - `deduplicating_array`:  0.1.4 -> 0.1.5
    - `fixed_decimal`: 0.5.3 -> 0.5.4
      - Fix rounding bug (https://github.com/unicode-org/icu4x/pull/3644)
      - Reexport some types to be more consistent with ICU4X naming (https://github.com/unicode-org/icu4x/pull/3945)
    - `litemap`: 0.7.0 -> 0.7.1
      - Add const litemap getter functions for common types (https://github.com/unicode-org/icu4x/pull/3416)
    - `icu_pattern`: 0.1.4 -> 0.1.5
    - `tinystr`: 0.7.1 -> 0.7.2
      - Add `UnvalidatedTinyAsciiStr` (https://github.com/unicode-org/icu4x/pull/3406)
      - Make compatible with `T: Bake` on `ZeroVec` (https://github.com/unicode-org/icu4x/pull/3451)
    - `tzif`: 0.2.1 -> 0.2.2
    - `writeable`: 0.5.2 -> 0.5.3
    - `yoke` and `yoke_derive`: 0.7.1 -> 0.7.2
      - Make `impl<...> Debug for Yoke<...>` implementation sound (https://github.com/unicode-org/icu4x/pull/3686)
      - Add `KindaSortaDangling` internal type to make Yoke safe under strong protection (https://github.com/unicode-org/icu4x/pull/3735)
    - `zerofrom` and `zerofrom_derive`: 0.1.2 -> 0.1.3
      - Expand `ZeroFrom` to cover cases where attrs can borrow (https://github.com/unicode-org/icu4x/pull/3770)
      - Expand `ZeroFrom` impl on unsized &T (https://github.com/unicode-org/icu4x/pull/3467)
    - `zerovec` and `zerovec_derive`: 0.9.4 -> (0.9.6)
      - Add `ZeroTrie`, an efficient string-to-int collection (https://github.com/unicode-org/icu4x/pull/2722)
      - Handle incorrect `VarZeroVec` bytes representation (https://github.com/unicode-org/icu4x/pull/3883)
      - Better soundness under stacked borrows (https://github.com/unicode-org/icu4x/pull/3513, https://github.com/unicode-org/icu4x/pull/3524, https://github.com/unicode-org/icu4x/pull/3515, https://github.com/unicode-org/icu4x/pull/3509)
      - Deserialization for `VarZeroVec<VarZeroSlice>`, take 2 (https://github.com/unicode-org/icu4x/pull/3649)
      - Fix `MultiFieldsULE` (https://github.com/unicode-org/icu4x/pull/3642)
      - Add initial `zeroslice!` and `zerovec!` macros https://github.com/unicode-org/icu4x/pull/3453, Update `zerovec!`/`zeroslice!` macro syntax to nested arrays (https://github.com/unicode-org/icu4x/pull/3611)
      - Add support for custom varule fields in #[make_varule] (https://github.com/unicode-org/icu4x/pull/3580)
      - Add `UnvalidatedChar` (https://github.com/unicode-org/icu4x/pull/3444)
      - `get_copied_by` API (https://github.com/unicode-org/icu4x/pull/3351)
    - `zerovec` and `zerovec_derive`: 0.9.6 -> 0.10.0:
      - Adding `T: Bake` bound on `ZeroVec`'s `Bake` (https://github.com/unicode-org/icu4x/pull/3451)
 - FFI:
    - Feature support
      - Case mapping exports:
        - `ICU4XCaseMapper`
        - `ICU4XCaseMapCloser`
        - `ICU4XTitlecaseMapper`
        - `ICU4XTrailingCase`
      - Other new exports:
        - `ICU4XCodePointSetBuilder`
        - `ICU4XLocaleDirection`
        - `ICU4XLocaleDirectionality`
        - `ICU4XLocaleFallbackSupplement`
        - `ICU4XIanaToBcp47Mapper`
      - Enum variants added for calendar types and errors
    - C++
      - Don't use `std::move` for inline arguments (https://github.com/rust-diplomat/diplomat/pull/336)
 - Experimental:
   - `bies`
     - No additional changes
   - `icu_compactdecimal`
     - Convenience functions for formatting `f64` and `FixedDecimal` (https://github.com/unicode-org/icu4x/pull/3699)
   - `displaynames`
     - New `VariantDisplayNames` for accessing display names of locale variant subtags (https://github.com/unicode-org/icu4x/pull/3461)
     - `LocaleDisplayNamesFormatter` now supports full UTS 35 language identifier formatting (https://github.com/unicode-org/icu4x/pull/3587)
   - `harfbuzz`
     - No additional changes
   - `ixdtf`
     - No additional changes
   - `relativetime`
     - No additional changes
   - `transliterate`: New crate: 0.1.0
   - `unicodeset_parse`: New crate: 0.1.0
   - `zerotrie`: New crate: 0.1.0


## icu4x 1.2.x (Apr 17, 2023)

Note: A subset of crates received patch releases in the 1.2 stream.

- `databake`: 0.1.5
  - Fixed [#3356](https://github.com/unicode-org/icu4x/pull/3356), adding `allow` for clippy false-positives
- `icu_capi` 1.2.1
  - Fixed [#3344](https://github.com/unicode-org/icu4x/pull/3344), `buffer_provider` feature accidentally pulling in extra crates
- `icu_capi` 1.2.2
  - Use `intptr_t` instead of `ssize_t` for portability ([diplomat #326](https://github.com/rust-diplomat/diplomat/issues/326))

- `icu_datagen` 1.2.1
  - Fixed [#3339](https://github.com/unicode-org/icu4x/pull/3339), incorrect Cargo features
- `icu_datagen` 1.2.3
  - Fixed [#3355](https://github.com/unicode-org/icu4x/pull/3355), adding MSRV annotations to generated code
  - Fixed [#3369](https://github.com/unicode-org/icu4x/pull/3369), making datagen call `rustfmt` directly instead of using the `rust-format` dependency
- `icu_datagen` 1.2.4
  - Remove dependency on `clap`'s `"cargo"` feature to better support non-Cargo users (#3388)
- `icu_datagen` 1.2.5
  - Remove runtime dependency on segmenter data pulled from the cargo cache (#3391)
- `icu_locid_transform` 1.2.1
  - Fixed [#3332](https://github.com/unicode-org/icu4x/issues/3332), missing `+?Sized` bound
- `icu_segmenter` 1.2.1
  - Fixed [#3341](https://github.com/unicode-org/icu4x/pull/3341), incorrect results on some strings with mixed scripts
- `icu_provider` 1.2.1
  - Do not autoenable `postcard/use-std` ([#3376](https://github.com/unicode-org/icu4x/pull/3376))
- `icu_datetime` 1.2.1
  - Remove superfluous `JapaneseEraV1` provider bounds on `TypedZonedDateTimeFormatter` [#3379](https://github.com/unicode-org/icu4x/pull/3379)


## icu4x 1.2 (Apr 13, 2023)

- General
  - All updated crates:
    - Add missing `Debug` impls (#3206)
    - Update Rust edition to 2021 (#3075)
    - Internal clippy fixes
    - Unless otherwise specified, all crate updates are to version 1.2.
    - Out-of-cycle releases do not get their own changelog entries, so some entries may span multiple patch or pre-1.0 minor versions.
- Data model and providers
  - `icu_provider`:
    - Add support for silencing expected `DataError`s (#3262)
    - Removing `dhat` dependency (#3138)
    - Make trait `DataMarker: 'static` (#3137)
  - `icu_datagen`: (includes patch updates 1.1.1 and 1.1.2)
    - (lib) Add `Out::Baked` and `BakedOptions`; deprecate `Out::Module` (#3130)
    - (cli) Bump clap to 4.0, move to using derive (#3149)
    - Pare down datagen deps (#3160)
    - Support changes from CLDR 43 (#3182, #3201, #3204, #3205)
    - Add support for complemented range iterators (#3198)
    - Using byte string literals in databake (#3040)\
    - Datagen support for all new component features
    - (performance) Less `ZeroMap` mutation in datagen (#3098)
  - `icu_provider_adapters`: No other changes
  - `icu_provider_blob`: No other changes
  - `icu_provider_fs`:
    -  Remove sha2 dep (#3160)
  - `icu_provider_macros`: No other changes
- Components:
  - Cross component: No additional cross-component changes
  - `icu_calendar`
    - Document the bounds of `IsoSecond`, `Minute` and `Hour` (#3156)
  - `icu_collator`: No other changes
  - `icu_collections`:
    - Add `to_u32` for TrieValue (#3222)
    - Add `CPT::try_alloc_map_value` (#3207)
    - Add support for coalescing range iterators (#3198)
    - Allow inversion lists to be built from ranges that include `char::MAX` (#3203)
  - `icu_datetime`: No other changes
  - `icu_decimal`
    - Add `From<GroupingStrategy>` for `FixedDecimalFormatterOptions` (#3045)
  - `icu_list`
    - `ListJoinerPattern::from_parts_unchecked()` is now `from_parts()` and panics when necessary (#3052)
  - `icu_locid`
    - Reduce size of internal `ShortVec` abstraction (#3200)
    - Use `Box` in place of `Vec` in `ShortVec` (#3220)
  - `icu_locid_transform`
    - The default set of likely subtags is now only the subset of languages that have a basic or greater CLDR coverage level; the full set is much larger in CLDR 43 and can be accessed via new constructors (#3148, #3158, #3197)
  - `icu_normalizer`: No other changes
  - `icu_plurals`: No other changes
  - `icu_properties`
    - Add ability to obtain enumerated property value names from enum values (#3222, #3276)
    - Add ability to obtain enumerated property values from their names (#3103, #3152)
    - Add support for runtime-selected property lookup (#3169)
    - Add support for complemented range iterators (#3198)
    - Support data for Bidi properties with combined data structure (#3026, #3258)
  - `icu_segmenter`
    - Graduated from experimental
    - Add "auto" feature, enable "lstm" feature, establish new data key structure, improve error handling, and revise word and line segmenter constructors (#3010, #3267, #3270)
    - Rename `icu_segmenter` enums (#3285)
    - Allow access to rule status via word_type in WordSegmenter and over FFI (#3139, #3275)
    - Upgrade segmenter to Unicode 15.0.0 (#3273)
    - Return a line break at index 0 (#3289)
    - Improve the LSTM code and data model to be zero copy, improve error handling, be future-proof, and vectorize more operations (#3210, #3217, #3233, #3250, #3254, #3264, #3291)
    - Remove ndarray dependency (#3192)
    - Silencing expected `DataError`s (#3262)
    - Fix SB10 rule (#3126)
    - Polished docs and examples
  - `icu_timezone`: No other changes
 - Utils:
  - `crlify`: No change (still at 1.0.1)
  - `databake`: 1.1.3 -> 1.1.4
    - Using byte string literals in databake (#3040)
  - `deduplicating_array`: 0.1.3 -> 0.1.4
  - `fixed_decimal`: 0.5.2 -> 0.5.3
  - `litemap`: 0.6.1 -> 0.7.0
    - Move FromIterator impl to new `StoreFromIterable` trait, allowing constructing `LiteMap`s with different backing stores (#3220)
  - `pattern`: 0.1.3 -> 0.1.4
  - `tinystr`: No change (still at 0.7.1)
  - `tzif`: No change (still at 0.2.1)
  - `writeable`: 0.5.1 -> 0.5.2
  - `yoke`: 0.7.0 -> 0.7.1
  - `yoke-derive`: 0.7.0 -> 0.7.1
  - `zerofrom`: 0.1.1 -> 0.1.2
  - `zerofrom-derive`: 0.1.1 -> 0.1.2
  - `zerovec`: 0.9.2 -> 0.9.4
    - Add initial ZeroHashMap (#2579)
    - Add `ZeroSlice::get_as_array()` (#3192)
    - Add range dep of yoke to zerovec (#3089)
  - `zerovec-derive`: 0.9.2 -> 0.9.4
    - Add `#[zerovec::derive(Hash)]` (#2579)
    - Avoid using derive on autogenerated packed types (#3069)
 - FFI:
    - Feature support
      - Property value-to-name mappings (#3196)
      -  `UnicodeSets` (including exemplar chars) (#3177)
      -  Runtime-selected property lookup (#3169)
      -  Property lookup ranges and GeneralCategoryGroup (#3230)
      -  LocaleExpander: Add extended and non-extended constructors (#3197)
      -  Fill in `BreakIterator` API (#3275)
      -  Bidi reorder_visual (#3183)
      -  (experimental) Strongly typed display names API (#3190, #3188)
    - Add feature slicing to `icu_capi` (#3216)
    - Better FFI provider ownership (#3140)
 - Experimental:
   - `bies`: 0.2.0 -> 0.2.1
   - `icu_casemap`: 0.7.1 -> 0.7.2
   - `icu_compactdecimal`: 0.1.0 -> 0.2.0
    - Support configurable grouping separators in CompactDecimalFormatter (#3045)
   - `icu_displaynames`: 0.8.0 -> 0.10.0
     - Add ScriptDisplayNames (#3317)
     - Add LanguageDisplayNames with support for variants (#3058, #3113)
     - Add stronger typing (#3190)
   - `icu_harfbuzz`: New experimental port: Harfbuzz integration for ICU4X (v0.1.0)
   - `icu_relativetime`: 0.1.0 -> 0.1.1


## icu4x 1.1 (Jan 26, 2023)

* `icu_calendar`
  * Fix bug in `simple_week_of()` around unit size (#2951)
  * Fix math in calendar (#2714)
  * Add `div_rem_euclid` and use it in icu_calendar (#2704)
  * Fix Time::from_minute_with_remainder_days to handle negatives (#2643) (#2702)
  * doc improvements

* `icu_casemap`
  * doc improvements

* `icu_collator`
  * doc improvements

* `icu_collections`
  * Add APIs for returning exemplar characters data (#2812)
  * Readable JSON inversion lists (#2855)
  * Add `UnicodeSet` that supports strings (#2796)
  * Add documentation on `CodePointTrie` details and perf considerations (#2717)

* `icu_codepointtrie_builder`
  * internal improvements

* `icu_datetime`
  * internal and doc improvements

* `icu_decimal`
  * internal and doc improvements

* `icu_displaynames`
  * DisplayNames fixes (#2918)
  * Rename `Territory` -> `Region` for display names component (#2895)
  * Transformer code for `Language` display names (#2871)
  * Adding a function to get display name for a region. (#2816)
  * Transformer code for display names component. (#2635)
  * doc improvements

* `icu_list`
  * Untangling list provider from logic and fixing big endian safety bug (#2994)
  * Not allocating `Writeable`s for regex evaluation (#2991)
  * doc improvements

* `icu_locid`
  * Reject duplicated extensions (#2893)
  * More borrowing in locid's `write_to_string` (#2693)
  * doc improvements

* `icu_locid_transform`
  * Clean up dependency specifications so `serde` isn't pulled in by default (#2696)
  * doc improvements

* `icu_normalizer`
  * internal and doc improvements

* `icu_plurals`
  * doc improvements

* `icu_properties`
  * Add APIs for returning exemplar characters data (#2812)
  * Add API and testdata for `Basic_Emoji` property (#2802)
  * Add `UnicodeSet` that supports strings (#2796)
  * Update `Script` property value enums (#2787)
  * doc improvements

* `icu_segmenter`
  * Make metacrate features more specific (#2932)
  * Remove `serde` dependency from segmenter with `lstm` feature. (#2904)
  * Simplify construction of grapheme cluster break iterators (#2870)
  * Store grapheme cluster payload instead of grapheme cluster segmenter. (#2864)
  * `#[no_std]` for LSTM segmenter (#2845)
  * icu_segmenter: enforce `clippy::indexing_slicing`. (#2325)
  * Use `GraphemeClusterSegmenter` in `DictionarySegmenter` and `LstmSegmenter` (#2716)
  * Rename `*BreakSegmenter` to `*Segmenter` (#2707)
  * Remove unnecessary language check for East Asian language (SA property) (#2705)
  * internal and doc improvements

* `icu_timezone`
  * Adds a bytes parsing API for `GMTOffset` for `CustomTimeZone` and FFI (#2943, #2955)
  * doc improvements

* `icu_provider_adapters`
  * Add more `inner_mut` functions in `icu_provider_adapters` (#2987)
  * Fix error propagation in `MultiForkByErrorProvider` (#2986)
  * Add mutation methods to `MultiForkByErrorProvider` (#2972)

* `icu_provider_blob`
  * internal and doc improvements

* `icu_provider`
  * Setting correct `DataError` for `.as_deserializing()`, `.as_downcasting()` (#2993)
  * doc improvements

* `icu_datagen`
  * Removing experimental feature from datagen (#3005)
  * Fixing Spanish list regex (#2989)
  * Datagen CLI improvements (#2950)
  * Some reexports for datagen (#2958)
  * Databake improvements (#2906)
  * Exclude certain collations by default and add option to include them (#2789)
  * Allowing no keys in datagen CLI (#2731)
  * Fixing baked datagen for no keys and keys with no data (#2698)
  * Internal and doc improvements

* `icu_provider_fs`
  * internal and doc improvements

* `icu_testdata`
  * stabilized metadata
  * doc improvements
  * CLDR 42 and ICU 72 data

* `bies`
  * Bumped `writeable` version

* `databake`
  * Doc improvements

* `fixed_decimal`
  * Treat strings starting with a decimal point as valid in `FixedDecimal::from_str()` (#2937)
  * The actual formatting part of compact decimal formatting (#2898)
  * Allow noncompact `CompactDecimal` (#2889)
  * `CompactDecimal` and `ScientificDecimal` (#2847)
  * doc improvements

* `icu_pattern`
  * internal improvements

* `litemap`
  * internal and doc improvements

* `tinystr`
  * Add `std` feature and `Error` impl for `TinyStrError` (#3009)
  * internal and doc improvements

* `tzif`
  * internal and doc improvements

* `writeable`
  * Using `core` integer log when available (#3015)
  * `usize` and `isize` implementation
  * internal and doc improvements

* `yoke`
  * Add `prove_covariance_manually` guard for `CoerceUnsized` (#2936)
  * Allow `clippy::forget_copy` in `derive(Yokeable)` impl (#2775)
  * Fix soundness issue in `Yoke::attach_to_cart()` around implied bounds #2949

* `zerovec`
  * `ZeroMap2d` cursors

## Bug fixes in 1.0.x

### icu_collator 1.0.1

- Made case level setting actually take effect (#2892)

## icu4x 1.0 (Sept 27, 2022)

- General
  - Update license to Unicode-DFS-2016 (#2303)
  - Major improvements to documentation, bechmarks, and examples
  - Various performance and codesize improvements
  - FFI for all non-experimental components
- Data model and providers
  - Polished baked data provider (#2098, #2126, #2147)
  - Data key extraction from binary (#1950)
  - Add `LocaleFallbacker` with locale fallback algorithm (#2036, #2115, #2186, #2567)
  - Making `DataProvider: Sync + Send` (#1853)
  - Update to Postcard 1.0 (#2037, #2091, #2438)
  - De-duplication in `BlobDataProvider` (#2062)
  - Move `map_project` to closures (#2185)
  - Renaming of many traits and functions in `icu_provider` (#2207, #2222, #2223)
  - Passing `DataLocale` by reference (#2224)
  - Fix feature specification in provider/fs (#2527)
  - Rename `DataKey` methods and return `DataKeyPath` (#2565)
  - Add some useful data provider impls; refactor `AnyPayloadProvider` (#2564)
  - Removing `StaticDataProvider` (#2582)
  - Removing `InvariantDataProvider` (#2159)
  - Renaming load_payload, load_resource (#2222)
  - Renaming `DataOptions` to `DataLocale` (#2223)
  - Use an abstract predicate function in `ForkByKeyProvider` (#2249)
  - Add `UnvalidatedStr` and use it in `LocaleFallbackParentsV1` (#2502)
  - Add some useful data provider impls; refactor `AnyPayloadProvider` (#2564)
- Components:
  - Cross component:
    - `Format` to `Formatter` rename (#2184)
    - Uniform constructor style across all components, see #2573 (#2293, #2305, #2309, #2316, #2318, #2326, #2327, #2329, #2330, #2332, #2333, #2334)
    - Remove `format_to_write`s (#2528)
    - Make error enums more consistent (#2649)
    - More Copy arguments (#2654)
  - `calendar`
    - Emit month codes from calendars  (#2053)
    - Add `Date::new_from_codes()`; fix up per-calendar constructor functions (#2255)
    - Fix iso-to-fixed conversion (#1898)
    - Ethiopic calendars (#1831, #1902)
    - Replace hour/minute/second constructors `new_unchecked()` with getter `number()` (#1922)
    - Improve and rename `types::Year`/`types::Month` (#2157)
    - Add `japanext` calendar (#2181)
    - Replace unbounded arithmetic for calendar numeric types with bounded arithmetic. (#2273)
    - Make `Japanext` its own calendar type (#2311)
    - Pick default calendar based off of locale in `AnyCalendar`
    - Make `offset_date` handle wraparounds for months (#2373)
    - Hide duration stuff, rename `IncludedInAnyCalendar` (#2426)
    - `week_of` refactoring (#2462)
    - Fix arithmetic in Indian calendar (#2479)
    - Infallible `from_minutes_since_local_unix_epoch()` (#2646)
  - `collator`
    - New component (#1706)
    - Validate the length of last_primaries (#1916)
    - Use a higher numeric value for `Strength::Identical` (#1942)
    - Move and unescape collator and normalizer tests (#1943)
    - Tweak CollationMetadataV1 documentation and dead code (#1914)
    - GIGO fix-ups for the normalizer and the collator (#1931)
    - split_first_u16/split_first_u24 -> split_first (#2459)
    - Create options bag for CollatorOptions (#2475)
    - Clean up FFFD magic numbers in Collator with REPLACEMENT_CHAR (#2496)
    - Add traditional spanish and plumbing to make it work (#2497)
  - `collections`
    - New component (#2294, #2323, #2328, #2336)
    - Rename `CodePointSet` to `CodePointInversionList` (#2230)
    - Allow `CodePointTrie` to determine `error_value` at runtime from data (#2301)
    - Use GIGO with debug assertion in Char16Trie (#2537)
  - `datetime`
    - Formatting for `AnyCalendar`s (#1987, #2146)
    - Renaming `DateTimeFormatter` (etc) to `TypedDateTimeFormatter` and `AnyDateTimeFormatter` to `DateTimeFormatter` (#2298)
    - DateFormatter cleanups (#2304)
    - Remove Calendar type parameter from `TimeFormat` (#2282)
    - Class Hierarchy for `DateTimeFormat` (split into `DateFormat`, `TimeFormat`, etc) (#2133)
    - Making `time_granularity` public (#1867)
    - Add fractional seconds support to components bag (#1873)
    - Use `FixedDecimalFormat` in `DateTimeFormat` (#1952)
    - Include module name to disambiguate Pattern (#1889)
    - Use month codes in formatting (#2071)
    - Split date and time data keys. (#2093)
    - Move `Formatted[Zoned]DateTime` over to preextracting the date time input info (#2138, #2205)
    - Remove `MockZonedDateTime` (#2231)
    - Add an offset_fallback field in `TimeZoneFormatV1` (#2253)
    - Remove `HourCycle` from the public Lengths API (#2331)
    - Move mock datetime parsing code to test modules (#2436)
    - Stop returning error on mismatched locale and type calendar (#2477)
    - Change default length to medium (#2596)
    - Make expect_pattern GIGO (#2650)
  - `decimal`
    - Don't panic on invalid grouping sizes (#2042)
    - Remove signum and sign display options (#2070)
    - Add numbering system support (#2246)
  - `list`
    - ListStyle -> ListLength and add _with_length (#2628)
  - `locid`
    - Add `remove()` for vertical fallback (#1992)
    -  Update `Locale` and `LanguageIdentifier` comparison functions to `strict_cmp()` and `normalizing_eq()` (#2020)
    -  `normalizing_eq()`, `strict_cmp()` for LSRV subtags (#2048)
    -  Add `strict_cmp_iter()` (#2111, #2114)
    -  Removing auto-derived Ord impl for Locale/LangId (#2142)
    -  Enable `locale` macro to support single unicode key value pair extension (#2382)
    -  Reducing `locid_id` API surface (#2484)
    -  `private::Key` and `other::Key` to `::Subtag` (#2632)
  - `locid_transform`
    - Rename from `icu::locale_canonicalizer` (#2381)
    - `LocaleCanonicalizer`/`LocaleExpander` refactor (#2338)
  - `normalizer`
    - Promoted from experimental (#2058)
    - Add ComposingNormalizer for NFC, NFKC, and UTS 46 (#2039)
    - GIGO fix-ups for the normalizer and the collator (#1931)
    - Add support for NFKD and the decomposed counterpart of UTS 46 without ignored and disallowed (#1967)
    - Simplify Hangul composition (#2200)
    - Make sink-writing normalization methods non-experimental (#2201)
    - Uses tries instead of inversion lists for normalization data (#2235)
    - Consolidate the two auxiliary tries to the main NFD trie (#2371)
    - Use `char` instead of `U24` in normalizer data (#2481)
    - Make NFKD and UTS 46 data store only the difference form NFD (#1984)
  - `plurals`
    - Rename `select()` to `category_for()` for `PluralRules` (#2287)
    - Use From instead of TryFrom for signed integers (#2593)
    - `from_tr35_string` -> `get_for_cldr_string` (#2633)
    - Make PluralOperands fields private, add static constructor (#2598)
  - `properties`
    - Better properties return values (#2112, #1990, #2277, #2555)
    - Move properties data over to an (extensible) enum (#2140)
    - Renaming unicode property data struct names (#2198)
  - `timezone`
    - New component, split from `datetime` (#2265)
    - Add time period metazone to `TimeZonesProvider` (#1961)
    - Convert metazone period from string to i32 (#2085)
    - Improvements to `MetaZoneCalculator` (#2274)
    - Add `TimeVariant` wrapper (#2289)
    - TimeVariant -> ZoneVariant with a few more docs (#2427)
    - Assorted TimeZone fixes (#2478)
 - Utils:
  - `crlify`: No updates
  - `databake`:
    - Moved over from `crabbake` (#2068)
    - Some databake improvements (#2150)
    - Using static `LiteMap`s in databake (#2264)
  - `deduplicating_array`: No updates
  - `fixed_decimal`:
    - Switch FixedDecimal to a trivaluate sign (#2025)
    - Remove negate (#2060)
    - Improve integer operations (#1924)
    - Add `FixedDecimal::concatenate_right()` (#1953)
    - Implement `ceil()`, `floor()` and `truncate()` functions (#1923)
    - Define "magnitude" and introduce "position" concept (#1981)
    - Support for rounding modes (#2000, #2100, #2104, #2261)
    - Make `multiply_pow10)_` be infallible (#2285)
  - `litemap`:
    - Remove `serde_json` dep from zeromap/litemap and align features (#1939)
    - `LiteMap` of `&'a [(K, V)]` (#2242)
    - Enable `ShortVec` as a backend for `LiteMap` (#2356)
  - `pattern`: No updates
  - `tinystr`:
    - Make `Option<TinyAsciiStr>` be the same size as `TinyAsciiStr` (#2430)
  - `tzif`:
    - New crate (#2019)
    - Parse POSIX time-zone strings using Combine (#1973)
    - Parse TZif binary files using Combine (#1999)
  - `writeable`:
    - Rename `write_len` (#2529)
  - `yoke`:
    - Deprecate yoke's `badly` methods (#1930)
    - Rename `Yoke::project()` functions to `::map_project()` (#1955)
    - Remove stable_deref_trait/alloc from yoke's default feature set (#2094)
    - Move `map_project()` to closures (#2185)
  - `zerofrom`: No updates
  - `zerovec`:
    - Make `VarZeroVec` format configurable (#2306)
    - Add `FlexZeroVec` (#1790)
    - Add `NicheBytes` trait and `NichedOptionULE` (#2501)
    - Turn ZeroVec into a struct for optimization (#2599, #2622)
    - Improve performance of VarZeroVec::deserialize and add provider benches (#2603)
    - Add array impl for `ZeroMapKV` (#1875)
    - Remove lifetime from `ZeroVecLike` (#1901)
    - `ZeroVecLike` cleanup (#2024)
    - Remove `serde_json` dep from zeromap/litemap and align features (#1939)
    - Make various ZeroVec methods `const` (#1976)
    - Refactor ZeroMap2d and add get_by functions (#1876)
    - Add more zerovec impls for `usize` and `FlexZeroVec` (#2023)
    - Change charULE from 4 bytes to 3 bytes (#2015)
    - More impls in zerovec crate (#2054)
    - Add binary search and other utilities to `FlexZeroVec` (#2284)
    - Remove `KeyError` and rename `get()` to `get_2d()` (#2279)
    -  `EncodeAsVarULE` for `Cow` (#2376)
    -  Add `ExactSizeIterator` for `FlexZeroVec::iter_*()` (#2580)
    -  Add permutation to ZVL containers (#2605)
 - FFI:
   - All non-experimental components now covered by FFI
   - Add FFI error strategy (#2045)
   - Configurable DataProvider FFI (#2526)
 - Experimental:
   - `bies`:
   - `casemapping`:
   - `segmenter`:
     - Expose `RuleBreakIterator` as a public interface (#2408)
     - Merge `segmenter_lstm` with segmenter (#2087)
     - Use `CodePointTrie` in Segmenter (#1839)
     - Move language detection to language.rs (#1689)
     - Simplify function in rule_segmenter (#1880)
     - Use dictionary segmenter for word. (#1936)
     - Remove std dependency from segmenter_lstm. (#2064)
     - Add Lao and Khmer LSTM models (#2120)
     - Use multiple dictionaries for line/word segmenter. (#2209)
     - Add a feature option not to use unicode-segmentation (#2212)
     - Remove two char types in line segmenter and polish utf8 iterator naming (#2269)

## icu4x 0.6.0 (May 9, 2022)

  - General data model
    - Non-exhaustive errors for locid, calendar, decimal, plurals (#1792, #1793)
    - Rename "serialize" feature to "serde" (#1797)
    - Turn all errors into Copy types (#1657)
  - Components
    - `calendar`:
      - Coptic, Indian and Ethiopian calendars (#1660, #1715, #1779)
      - Calendar arithmetic (#1614)
    - `datetime`:
      - Formatting for fractional seconds (#1813, #1801)
      - Support for day of week in month ('F') (#1770)
      - Custom fallbacking for TimeZoneFormatter (#1591)
      - Support for week-of-month (#1468)
      - Bug fix to get_best_available_format_pattern skeleton matching logic (#1549)
    - `decimal`: No updates
    - `locale_canonicalizer`:
      - ZeroCopy support (#1760, #1777)
    - `locid`:
      - Simplified language representation (#1695)
      - Region, Script and Variant subtags ULE (#1696)
    - `plurals`:
      - Update data model to use `ZeroVec` (#1240)
    - `properties`:
      - Bidi support (#1716, #1784)
  - Utilities
    - `codepointtrie`:
      - Use 0 for error value for Rust out-of-bounds for primitive trie value types (#1804)
    - `crlify`: New util for line ending conversions
    - `deduplicating_array`: No updates
    - `fixed_decimal`:
      - Improvements to FixedDecimal f64 APIs (#1718)
    - `litemap`:
      - Pluggable LiteMap backends (#1769)
    - `pattern`: No updates
    - `uniset`: No updates
    - `writeable`: No updates
    - `yoke`: No updates
    - `zerofrom`: No updates
    - `zerovec`:
      - ZeroVec derive improvements (#1780)
      - Support non-Ord values in ZeroMap (#1743)
      - Add OptionULE and OptionVarULE (#1736)
      - Rename ZeroVec::from_slice and add new method for const-constructed ZeroSlice (#1728)
      - Treat ZeroMap sort order as an optional invariant (#1727)
      - Add ZeroMap::get_copied_by (#1722)
      - Generalize PairULE to support longer tuples (#1721)
      - Add more AsULE impls for primitives (#1672)
      - Add cast methods to ZeroVec and ZeroSlice (#1651)
      - Add RawBytesULE::slice_from_byte_slice (#1648)
      - Create façades for ZeroVec types, hide internal code organization modules (#1629)
      - Add zerovec::skip_kv and zerovec::skip_ord attributes, as well as generalized attribute handling framework (#1613)
      - Rename as_unaligned to to_unaligned (#1619)
  - FFI:
    - Updating to Diplomat 0.3
    - Making testdata an optional FFI dep (#1820)
    - Split out capi targets: make separate freertos, staticlib, and cdylib crates as targets (#1747)
  - Experimental:
    - `crabbake`: Initial version of baked data provider (#1825)
    - `segmenter`:
      - Support production-ready data provider for segmenters (#1652)
      - Implement dictionary based segmenter for line segmenter. (#1644)
      - Wire DataProvider into UAX29 segmenters (#1627)
      - Move UAX#14 defines to line.toml (#1568)
      - Add segmenter factories to generate UAX29 iterators (#1602)


## icu4x 0.5.0 (Jan 31, 2022)

  - General data model
    - `DataPayload` no longer needs a lifetime (#1297, #1279)
    - Re-write DataKey (#1511)
    - Rewrite ErasedDataProvider as AnyProvider (#1495)
    - Add EitherProvider and rename IterableDataProviderCore to IterableProvider (#1455)
    - Change DataRequest to be borrowed in BufferProvider (#1416)
    - Replace SerdeDeDataProvider with BufferProvider (#1369, #1384)
  - Components
    - `calendar`:
      - Julian, Japanese, and Buddhist calendars (#1351, #1394, #1305)
      - `DateTimeFormat` integration (#1339)
      - Bugfix around arithmetic (#1352)
    - `datetime`:
      - Week-of-year support (#1206)
      - `DateTimeFormat::resolve_components()` (#1362)
      - Era formatting (#1346)
      - `TimeZoneFormatterConfig` (#1256)
      - New data model for organizing calendar data (#1300)
      - Bugfix around missing localized strings in time zone data (#1405)
    - `decimal`: No updates
    - `locale_canonicalizer`:
      - Bugfix in maximization (#1171)
      - Update data model to use `LiteMap` (#1275)
    - `locid`: No updates
    - `plurals`:
      - Update data model to use `ZeroVec` (#1240)
    - `properties`:
      - Rename resource key category for properties (#1406)
      - Rename enums for `General_Category` (#1355)
      - Implement the `Canonical_Combining_Class` property (#1347)
      - Implement `Script_Extensions` property (#1353)
      - Add `General_Category` predicate functions (#1310)
      - Implement `Grapheme_Cluster_Break`, `Word_Break`, and `Sentence_Break` Unicode properties (#1233)
  - Utilities
    - `codepointtrie`: No changes
    - `deduplicating_array`: New utility for efficient serialized representation of data with duplicates
    - `fixed_decimal`:
      - Padding and truncation APIs (#1482, #1507, #1195)
      - Add double-to-decimal via ryū (#1217)
      - Handle exponents in `FixedDecimal::from_str()` (#1265)
    - `litemap`:
      - Add `LiteMap::get_indexed()` and `LiteMap::find_index()`
      - Handle serialization of tuples (etc) in litemaps (#1306)
    - `pattern`: No updates
    - `uniset`: No updates
    - `writeable`:
      - Adding parts functionality to `Writeable` (#1438)
      - Change `Writeable::writeable_to_string` to return a Cow (#1452)
      - Implementing `Writeable` for all integers (#1408)
      - Making `writeable::LengthHint` a range (#1400)
      - Simplifying `assert_writeable_parts_eq` and `assert_writeable_eq` (#1522, #1399)
    - `yoke`:
      - Remove `Yokeable::Output` from `ZeroCopyFrom` trait (#1499)
      - Add `EitherCart` (#1484)
      - Remove `attach_to_option_cart()` (#1348)
      - Homogenize yoke generic impls to always work with `Yokeable`, add `OwnedYokeable` (#1302)
    - `zerovec`:
      - Move over to a model where the vector types deref to `ZeroSlice` and `VarZeroSlice` (#1418, #1371)
      - Simplify `ZeroVec`/`VarZeroVec` error handling, consolidate `ULEError` type (#1389)
      - Simplify `ZeroMapKV` (#1334)
      - Add `ZeroMap2d`, a two-dimensional zero-copy map (#1432)
      - Add borrowed-only version of `ZeroMap` (#1238)
      - Add various helper functions to zerovec (#1430)
      - Rename `PlainOldULE` to `RawBytesULE` (#1413)
      - Improve `EncodeAsVarULE` (#1385)
      - Add `EncodeAsVarULE` for `ZeroVec` (#1274, #1407)
      - Various trait impls (#1332, #1330, #1328, #1287)
      - Document ULE alignment guarantee; update all impls with checklists (#1294)
      - Fix `PairULE` validation function (#1266)
  - FFI:
    - Updating to the latest Diplomat
    - Add FFI for constructing Data Structs, including decimal data structs (#1497)
    - Add padding/truncation to FFI (#1501)
    - Add FFI for constructing fixed decimals from float (#1483)
    - Properties FFI (#1269)
  - Experimental:
    - New ListFormatter experiment
    - More progress on segmentation experiment

## icu4x 0.4.0 (November 1, 2021)

  - Updated to CLDR 40 (#1216)
  - Functional Unicode property APIs and data; moved to new crate `icu_properties` (#148)
  - `DateTimeFormat` optimization: memory usage cut by two-thirds and perf improved by 40% (#519)
  - `BlobDataProvider` allowing for dynamic, asynchronous data loading (#1084)
  - Major upgrades to the `zerovec` utility crate and new benchmarks for `ZeroMap` (#1082, #1087)
  - Initial support for time zones in `DateTimeFormat` components bag (#845)
  - Bug fixes in `LocaleCanicalizer` (#1189, etc.)
  - Week of month/year arithmetic (#918)
  - Major improvements on code size across FFI (#962)
  - Iterations on ICU4X Segmenter (#1045, #1165, etc.)
  - Experimental `ListFormatter` (#1053)
  - ICU4X now requires Rust 1.56 (#1201)

## icu4x 0.3.0 (July 29, 2021)

  - Static data provider without filesystem dependency (#78)
  - Infrastructure around zero-copy deserialization, including `ZeroVec` and `Yoke` (#652, #667)
  - Most ICU4X components support `no_std` (#812)
  - Hour cycle preferences are used by DateTimeFormat (#671)
  - Skeleton matching supports additional field widths (#584)
  - Add canonicalize method to LocaleCanonicalizer (#747)
  - Added range iterator on UnicodeSet (#836)
  - Add PluralRules::categories() function (#789)
  - Consolidated data generation tool into new binary `icu4x-datagen`

## icu4x 0.2.0 (April 29, 2021)
  - Updated CLDR to 39 (#640)
  - Initial release of:
    - `icu_decimal` (#590)
    - `icu_locale_canonicalizer` (#417)
  - `DateTimeFormat` gets support for
    - day periods (#435)
    - time zones (#418)
    - components bag (#481)
  - `UnicodeSet` gets preliminary support for L3a subset (#478)
  - `PluralRules` support `E` operand (#407)
  - New utilities:
    - `Writeable`
    - `ZeroVec`
    - `LiteMap`
    - `FixedDecimal`
    - `Pattern`
  - Early prototype of C API - `icu_capi` 

## icu4x 0.1.0 (October 15, 2020)
  - Initial release of ICU4X.
  - Initial release of:
    - `icu`
    - `icu_locid`
    - `icu_plurals`
    - `icu_datetime`
    - `icu_uniset`
    - `icu_provider`
    - `icu_provider_fs`
    - `icu_provider_cldr`
    - `icu_testdata`
    - `icu4x_ecma402`
    - `fixed_decimal`
