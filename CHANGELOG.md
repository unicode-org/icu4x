# Changelog

## Unreleased

Fully filled in up to 30c187f4b7


- Components
    - General
        - Use HTTPS links in docs (unicode-org#7212)
    - `icu_calendar`
        - Introduce a new `Month` type, preferred over using month codes (unicode-org#7147)
        - Restrict the range of valid dates constructed via certain constructors (unicode-org#7219, unicode-org#7227)
        - Fix out-of-range bug during date arithmetic (unicode-org#7257)
        - Implement ISO 8601 date duration parsing (#7355)
        - Correctly produce `ethioaa` calendars from `CalendarAlgorithm` (unicode-org#7321)
        - Respect `-u-rg` in calendar resolution (unicode-org#7376)
        - Optimize the stack size of `Date` types (unicode-org#7220)
        - `AnyCalendar` cleanups and docs fixes (unicode-org#7223, unicode-org#7225)
        - Improve Hijri docs (unicode-org#7330, unicode-org#7332, unicode-org#7333)
        - Remove `Hijri<AstronomicalSimulation>` simulation code, retaining hardcoded data, falling back to Tabular for non-modern dates (unicode-org#7301)
        - Replace `Date::day_of_week` by `Date::weekday` (unicode-org#7288)
        - Deprecate `Date::new_from_iso`/`Date::to_iso` (unicode-org#7287)
        - Optimize Hebrew and Julian calendars (unicode-org#7213)
        - Optimize day/week diffing to use RDs (unicode-org#7308)
    - `icu_casemap`
        - General changes only
    - `icu_collections`
        - Add `CodePointInversionListAndStringList::contains_utf8` (unicode-org#7363)
    - `icu_codepointtrie_builder`: `0.5.1 -> ???`
        - Remove serde dep from `icu_codepointtrie_builder` (unicode-org#7298)
    - `icu_collator`
        - General changes only
    - `icu_datetime`
        - Fix error handling for FieldSetBuilder (unicode-org#7245)
        - Add some support for `U` (cyclic year) datetime symbol (unicode-org#7328)
        - Remove old datetime data structs (unicode-org#7205)
        - Assorted improvements to icu_datetime docs (unicode-org#7244)
    - `icu_decimal`
        - General changes only
    - `icu_experimental`: `0.4.0 -> ???`
        - `compactdecimal`
            - Don't hallucinate patterns (unicode-org#7387)
            - Pack compact decimal data (unicode-org#7388)
            - Round fractional compact decimals to two significant digits (unicode-org#7389)
        - `currency`
            - Add currency fractions provider (unicode-org#7278)
            - Update comments for currency data structures to enhance clarity (unicode-org#7405)
        - `dimension`
        - `measure`
        - `relativetime`
        - `units`
            - Correct region extraction for categorized display names (#7421)
    - `icu`
        - Add example for measuring Date::try_from_fields code size (unicode-org#7297)
    - `icu_list`
        - General changes only
    - `icu_locale`
        - Add docs discouraging direct conversion from Locale to DataLocale for locale fallback (unicode-org#7348)
    - `icu_locale_core`
        - Fix regional override `-u-rg` (unicode-org#7337) and regional subdivision `-u-sd` (unicode-org#7341) to fix region-priority data loading in other components
        - (Breaking internal API) The `struct_keyword!` macro was changed to operate on references for conversions (unicode-orgunicode-org#7361)
        - Add `From<&Value>` for struct preferences (unicode-org#7361)
        - Correctly parse `-u-ca-ethiopic-amete-alem` alias (#7413)
        - Add `LocalePreferences::from_locale_strict` (unicode-org#7377)
        - Fix `LanguageIdentifier::normalize_utf8` example (unicode-org#7372)
        - Use better types in `LocalePreferences` (unicode-org#7360)
    - `icu_normalizer`
        - Move `harfbuzz-traits` implementations into component crates (unicode-org#7200)
    - `icu_pattern`
        - Create SinglePlaceholderPattern::PASS_THROUGH (unicode-org#7393)
    - `icu_plurals`
        - FourBitMetadata should be checked against 0x10 or 0x0F, not 0x80 (unicode-org#7395)
        - Fix overflow in `PluralOperands` constructor (#7425)
    - `icu_properties`
        - Add enumerated property `Numeric_Type` (unicode-org#7157)
        - Add enumerated property `Joining_Group` (unicode-org#7293)
        - Add missing convenience API for `Basic_Emoji` and `EmojiSet` (unicode-org#7358)
        - Stabilise `IndicConjuctBreak` (unicode-org#7280)
        - Constify `PropertyNamesLong`/`PropertNamesShort`/`PropertyParser` constructors (unicode-orgunicode-org#7294)
        - Fix script values (unicode-org#7269)
        - Move `harfbuzz-traits` implementations into component crates (unicode-org#7200)
        - Add conversions for `unicode_bidi::BidiClass` (unicode-org#7272)
        - Add conversions for properties/locale scripts (unicode-org#7270)
        - Validate properties names, constants (unicode-org#7284, unicode-org#7281)
    - `icu_segmenter`
        - Add non-complex segmenter constructors (unicode-org#7268)
        - (Experimental) Initial code for RAdaBoost word segmenter for Chinese and CNN word segmenter for Thai (unicode-org#7122, unicode-org#7217, unicode-org#7246, unicode-org#7344)
    - `icu_time`
        - Add docs for `DateTime`/`ZonedDateTime` semantics (unicode-org#7275)
        - Relax some bounds (unicode-org#7286)
- Data model and providers
    - `icu_provider_adapters`
        - General changes only
    - `icu_provider_baked`
        - Use `Index32` in baked data by default (unicode-org#7310)
    - `icu_provider_blob`
        - General changes only
    - `icu_provider`
        - Breaking: Add associated type to `MaybeEncodeAsVarULE` trait and use Index32 (unicode-org#7310)
        - Add serde cargo feature comments to icu_provider buf/serde.rs (unicode-org#7408)
    - `icu_provider_export`
        - Print warning when multiple filters are applied to same marker (unicode-org#7240)
    - `icu4x-datagen`
        - Add `--attribute-filter` cli flag to icu4x-datagen (unicode-org#7236)
    - `icu_provider_registry`
        - Various new data markers as needed by component crates
    - `icu_provider_source`
        - Support for generating any new data markers as needed by component crates
        - Update datagen to TZDB 2025c (unicode-org#7306)
        - Always generate fast-mode data for NFD and NFKD tries (unicode-org#7222)
- FFI
    - `icu_capi`
        - FFI analogues for *most* new ICU4X component APIs
        - Add FFI property `try_from_str` (unicode-org#7367)
    - (Experimental) Basic Kotlin bindings (unicode-org#7237, unicode-org#7256, unicode-org#7265)
    - Use stable Dart 3.10 (unicode-org#7243)
    - `icu_harfbuzz`
        - Retire the `icu_harfbuzz` crate. The `icu_properties` and `icu_normalizer` types now directly implement the `harfbuzz-traits`
- Utils
    - `bies`: No change (`0.2.5`)
    - `calendrical_calculations`: `0.2.3 -> ???`
        - Optimize Hebrew and Julian calendars (unicode-org#7213)
        - Add docs about the Skaukat criterion (unicode-org#7331)
    - `crlify`: No change (`1.0.4`)
    - `databake`: No change (`0.2.0`)
    - `fixed_decimal`: No change (`0.7.1`)
    - `ixdtf`: No change (`0.6.4`)
    - `litemap`: No change (`0.8.1`)
    - `potential_utf`: No change (`0.1.4`)
    - `resb`: No change (`0.1.1`)
    - `tinystr`: No change (`0.8.2`)
    - `tzif`: No change (`0.4.1`)
    - `writeable`: `0.6.2 -> ???`
        - Add writeable::adapters::Concat and writeable::concat_writeable! (unicode-org#6929)
    - `yoke`, `yoke_derive`: `0.8.1 -> ???`
        - impl common traits (Display, PartialEq/Eq, PartialOrd/Ord) (#7400)
        - derive: Allow trait bounds in `where` clauses (unicode-org#7230)
    - `zerofrom`: No change (`0.1.6`)
    - `zerotrie`: `0.2.3 -> ???`
        - Add `ZeroAsciiDenseSparse2dTrie` for more efficient storage of data keys with many attributes (unicode-org#7264, unicode-org#7304, unicode-org#7305)
    - `zerovec`: `0.11.5 -> ???`
        - `schemars` support (unicode-org#7209)
    - `zerovec`: No change (`0.11.2`)
    - `zoneinfo64`: `0.2.1 -> ???`
        - Internal cleanups

## icu 2.1.x

Several crates have had patch releases in the 2.1 stream:

- Components
    - (2.1.1) General
        - Fix `icu_locale_core` dependency (unicode-org#7191)
    - (2.1.2)`icu_properties`
        - Fix some property constants (unicode-org#7269, unicode-org#7281, unicode-org#7284)
        - Add conversions for `unicode_bidi::BidiClass` (unicode-org#7272)
        - Add `IndicConjunctBreak` (unicode-org#7280)
        - Add conversion between `icu::properties::props::Script` and `icu::locale::subtags:Script` (unicode-org#7270)
    - (2.1.2) `icu_segmenter`
        - Add non-complex line and word break constructors (unicode-org#7268)
- Data model and providers
    - (2.1.2) `icu_provider_source`, `icu_locale_data`, `icu_datetime_data`, `icu_experimental_data`
        - Update to CLDR 48.1 (unicode-org#7396)
    - (2.1.2) `icu_provider_registry`, `icu_provider_source`
        - Add `IndicConjunctBreak` (unicode-org#7280)

## icu4x 2.1

- Components
    - General
        - Update MSRV to 1.83 (unicode-org#7066)
        - Remove unused dependencies (unicode-org#6978)
    - `icu_calendar`
        - Collapse `Dangi` and `Chinese` into `EastAsianTraditional`, expose (unstable) customization trait (unicode-org#6938, unicode-org#7159)
        - Collapse Hijri calendar types, expose (unstable) `Rules` trait for customization (unicode-org#6935)
        - Add Easter holiday to `Gregorian` and `Julian` (unicode-org#6899)
        - Implement `PartialOrd` for `Date` unconditionally (unicode-org#7090)
        - Switch Chinese and Korean calendars to no longer being data-driven, hardcoding their data (unicode-org#6934, unicode-org#7008)
        - (unstable) Add `Date::try_from_fields` for flexibly building Temporal dates (unicode-org#6910, unicode-org#7116, unicode-org#7163)
        - (unstable) Implement date arithmetic according to Temporal specification (unicode-org#6992, unicode-org#7012)
        - Make `extended_year` have consistent behavior across calendars, matching specced behavior in CLDR and Temporal (unicode-org#6800, unicode-org#6937)
        - Remove era aliases for Persian (unicode-org#7124)
        - Fix `und-SA-u-ca-islamic` (unicode-org#6736)
        - Fix calendar preference resolution (unicode-org#7158)
        - Deprecate convenience constructors of lunisolar calendars (unicode-org#7143)
        - Avoid panics for large past/future dates in astronomical calendars (unicode-org#6876)
        - Improve some Gregorian calendar code (unicode-org#6870)
        - Optimise `day_of_provided_year`, `date_from_provided_year_day` for ISO/Gregorian (unicode-org#6883)
        - Add a lot more documentation on individual calendars (unicode-org#7016, unicode-org#7033, unicode-org#7036, unicode-org#7037, unicode-org#7047, unicode-org#7082, unicode-org#7151)
        - Compare Chinese calendar against Hong Kong observatory (unicode-org#6944)
        - Test Korean calendar against KASI (unicode-org#7041)
    - `icu_casemap`
        - General changes only
    - `icu_collator`
        - Add `CollatorBorrowed::write_sort_key_to`; to support generating sort keys (unicode-org#6537, unicode-org#6656, unicode-org#6708, unicode-org#7186)
        - Make merge separator compare less than U+0000 on the identical strength (unicode-org#6814, unicode-org#6823)
        - Add Latin1 comparisons to the collator (unicode-org#6674)
        - Avoid double-validating `char`s (unicode-org#6924)
    - `icu_collections`
        - Fix building without the `alloc` crate (unicode-org#6997)
        - Optimize `CodePointTrie` by hoisting fast path bound checks to constructor (unicode-org#6863)
        - Make trivial constructors for Char16Trie / Char16TrieIterator inline-eligible (unicode-org#6864)
    - `icu_datetime`
        - Add `Preferences::from_locale_strict` (unicode-org#6737)
        - Change short localized offset format to use +0 (unicode-org#6729)
        - Fix `und-SA-u-ca-islamic` (unicode-org#6736)
        - Use atTime patterns where available (unicode-org#7106)
        - Disambiguate inconsistent metazones (unicode-org#6755, unicode-org#7160)
        - Implement `u` and `g` fields (unicode-org#6930)
        - Use accurate `ZoneNameTimestamps` (unicode-org#6942)
        - Improve formatting for Etc/GMT+X zones (unicode-org#7055)
    - `icu_decimal`
        - General changes only
    - `icu_experimental`: `0.3.0 -> 0.4.0`
        - `units`
            - Refactor measure unit parser by using fixed unit ids (unicode-org#6683)
        - `measure`
            - End-to-End Categorized Units Formatter with sliced data (Core / Extended / Outlier)  (unicode-org#6830)
            - Add short representation generator for MeasureUnit (unicode-org#6685)
            - Add `CategorizedFormatter` for unit formatting (unicode-org#6710)
            - Introduce `duration` category for measure units (unicode-org#6676)
            - Add category module for measure units with associated traits and implementations (unicode-org#6535)
            - Add UnitsIdsV1 to support the fundamental units' indices (unicode-org#6598)
        - `relativetime`
            - Fix relativetime preferences to observe numbering system (unicode-org#6928)
        - `dimension`
            - Migrate PercentEssentials to VarZeroCow; add test for #4662 (unicode-org#6716)
            - Refactor currency provider structure (unicode-org#6732)
    - `icu`
        - Move examples into metacrate (unicode-org#6591)
    - `icu_list`
        - General changes only
    - `icu_locale`
        - Add `extend` to Unicode Extensions (unicode-org#7112)
    - `icu_locale_core`
        - Fix building without the `alloc` crate (unicode-org#6997)
        - Implement Serialize and Deserialize for Locale (unicode-org#6829)
        - Add `Preferences::from_locale_strict` (unicode-org#6737)
    - `icu_normalizer`
        - Permit enabling optimizations when the data is known at compile time to always be a fast trie (unicode-org#6906)
        - Performance work counteracting a compiler regression (unicode-org#6825)
        - Jump forward throwing away work instead of panic in release builds (unicode-org#6816)
    - `icu_pattern`: `0.4.0 -> 0.4.1`
        - General changes only
    - `icu_plurals`
        - Change debug output of PluralElementsPackedULE (unicode-org#6841)
    - `icu_properties`
        - Fix building without the `alloc` crate (unicode-org#6997)
        - Optimize property map lookups (unicode-org#6886)
        - Add support for missing binary properties:
            - ID_Compat_Math_Continue
            - ID_Compat_Math_Start
            - IDS_Unary_Operator
            - Modifier_Combining_Mark
    - `icu_segmenter`
        - General changes only
    - `icu_time`
        - Deprecate `VariantOffsetCalculator` (unicode-org#6905)
        - Deprecate time zone variant APIs (unicode-org#6754)
        - Move `TimeZoneVariant` to provider module (unicode-org#6822)
        - Treat `Etc/GMT+x` as a raw offset, improve formatting (unicode-org#7055)
        - Store `ZoneNameTimestamp` as UTC (unicode-org#6746)
        - Use accurate `ZoneNameTimestamps` (unicode-org#6942)
        - Optimize DST offsets using a lookup table (unicode-org#6765)
- Data model and providers
    - `icu_provider_baked`
        - Expose const structs in baked data (unicode-org#6652)
    - `icu_provider`
        - Fix building without the `alloc` crate (unicode-org#6997)
    - `icu4x-datagen`
        - Don't require sources to be set in `icu4x-datagen` (unicode-org#6892)
    - `icu_provider_source`
        - Update to CLDR 48 (unicode-org#6793, unicode-org#6989, unicode-org#7046, unicode-org#7167)
        - Don't complain about not covering `Factory` timezone in datagen (unicode-org#6768)
        - Detect rearguard TZDB differences (unicode-org#6943, unicode-org#6749)
        - Include metazone periods before the horizon if the metazone is included anyway (unicode-org#6747)
        - Handle locales without `territories.json`, locales with inconsistent patterns (unicode-org#6709)
- FFI
    - `icu_capi`
        - All C++ enums now default to a valid value; which is the `Default` impl where there is one, and some semi-logical value otherwise. This has changed defaults in some cases and may cause a behavioral change for people relying on C++ default constructors. (unicode-org#6692)
        - Wasm code now uses the stable standard `wasm-c-abi=spec` (unicode-org#6679)
        - Enable experimental code in NPM (unicode-org#6743)
        - Allow consumers to determine bindings directories (unicode-org#6887)
        - Document deprecated APIs (unicode-org#6890)
        - FFI for most new APIs
        - `ListFormatter::format` now takes a `diplomat::span<const diplomat::string_view_for_slice>` instead of a `diplomat::span<std::string_view>` to handle soundness issues on some platforms (unicode-org#6974)
    - Dart version `2.1.0-dev.0`, `2.1.0-dev.1`
    - NPM version `2.1.0`
- Utils
    - General
        - Util MSRV is decoupled from ICU4X MSRV; uniformly set to 1.82 for now (unicode-org#7125)
    - `bies`: `0.2.4 -> 0.2.5`
        - Minor internal changes
    - `calendrical_calculations`: `0.2.2 -> 0.2.3`
        - Add RataDie::in_well_behaved_astronomical_range(), use to avoid panics (unicode-org#6876)
    - `crlify`: No change
    - `databake`: No change
    - `databake_derive`: `0.2.0 -> 0.2.1`
        - Require public fields for `Bake` derive (unicode-org#6586)
    - `fixed_decimal`: `0.7.0 -> 0.7.1`
        - Minor internal changes
    - `ixdtf`: `0.6.3 -> 0.6.4`
        - Add to_static_string for IXDTF errors (unicode-org#6917)
    - `litemap`: `0.8.0 -> 0.8.1`
        - Use `serde_core` (unicode-org#6991)
    - `potential_utf`: `0.1.3 -> 0.1.4`
        - Use `serde_core` (unicode-org#6991)
    - `resb`: `0.1.0 -> 0.1.1`
        - Use `serde_core` (unicode-org#6991)
    - `tinystr`: `0.8.1 -> 0.8.2`
        - Add TinyAsciiStr::new_unsigned_decimal (unicode-org#6912)
        - Use `serde_core` (unicode-org#6991)
    - `tzif`: `0.4.0 -> 0.4.1`
        - Minor internal changes
    - `writeable`: `0.6.1 -> 0.6.2`
        - Fix building without the `alloc` crate (unicode-org#6985)
    - `yoke`, `yoke_derive`: `0.8.0 -> 0.8.1`
        - Add four `map_with_cart` methods to `yoke::Yoke`, similar to `Yoke::map_project` but
        additionally providing a reference to the cart. (unicode-org#6781)
        - Add `Yoke::with_mut_return`, similar to `Yoke::with_mut` but with a callback that may
            return any `'static` type. (unicode-org#6827)
        - Allow clippy::mem_forget in yoke_derive (unicode-org#6891)
    - `zerofrom`, `zerofrom_derive`: No change
    - `zerotrie`: `0.2.2 -> 0.2.3`
        - Use `serde_core` (unicode-org#6991)
        - Internal docs and lints (unicode-org#6740)
        - Fix test failures in zerotrie on 32-bit platforms (unicode-org#6697)
    - `zerovec`: `0.11.4 -> 0.11.5`
        - Write a proper safety comment for ZeroVec::truncate (unicode-org#6809)
        - Use `serde_core` (unicode-org#6991)
    - `zerovec_derive`: `0.11.1 -> 0.11.2`
        - Minor internal changes
    - `zoneinfo64`: `0.2.0 -> 0.2.1`
        - Minor internal changes

## icu4x 2.0.x

Several crates have had patch releases in the 2.0 stream:

- `icu_calendar`
  - (2.0.1) Fix chinese day-of-year (unicode-org#6567)
  - (2.0.2) Respect `-u-fw` keyword in `WeekInformation` (unicode-org#6615)
  - (2.0.3) Fix extended year for Roc/Ethiopic (unicode-org#6721)
  - (2.0.3) Fix treatment of None era code for Gregorian (unicode-org#6794)
  - (2.0.4) Fix a sign error in `RataDie::until`, add `RataDie::since` (unicode-org#6861)
  - (2.0.5) Fix calendrical-calculations dependency (unicode-org#6919)
- `icu_properties`, `icu_properties_data`
  - (2.0.1) Fix a visibility bug in compiled data (unicode-org#6580)
- `icu_provider_baked`
  - (2.0.1) Fix an issue where a single-locale data generation would skip fallback (unicode-org#6582)
- `icu_capi`
  - (2.0.1) Rename string-methods on `DecomposingNormalizer` to match those on `ComposingNormalizer` (unicode-org#6594)
  - (2.0.1) Add `DataProvider` constructors in JS and Dart (unicode-org#6596)
  - (2.0.1) Fix `TimeZoneVariant` constructor (unicode-org#6610)
  - (2.0.2) Add `Locale::set_unicode_extension` (unicode-org#6636)
- `icu_datetime_data`, `icu_time_data`, `icu_provider_source`
  - (2.0.1) Update to tzdb 2025b
- `calendrical_calculations`
    - (0.2.1) Fix a sign error in `RataDie::until`, add `RataDie::since` (unicode-org#6861)
    - (0.2.2) Make `iso_year_from_fixed`, `day_before_year` public (unicode-org#6871)
    - (0.2.2) Optimise some calculations for `iso` (unicode-org#6883)
    - (0.2.2) Add Easter holiday to `iso` and `julian` (unicode-org#6899)
- `ixdtf`
    - (0.6.0) Add UTF16 handling (unicode-org#6577)
    - (0.6.0) Add TimeZoneParser::parse_identifier for TimeZoneRecord (unicode-org#6584)
    - (0.6.0) Reject empty durations when parsing ISO8601 durations (unicode-org#6718)
    - (0.6.0) Handle ambiguous time parsing with MonthDay and YearMonth in `ixdtf` (unicode-org#6717)
    - (0.6.1) Fix is_valid_month_day argument ordering bug (unicode-org#6756)
    - (0.6.2) Offset must have a sign (unicode-org#6763)
    - (0.6.2) Correctly handle ambiguous annotations (unicode-org#6776)
- `potential_utf`
    - (0.1.3) Add `.chars()` to `PotentialUtf16` (unicode-org#6726)
- `zerovec`:
    - (0.11.3) Make `VZV::Default` work with non-default index (unicode-org#6661)
    - (0.11.3) Make ZeroVec.iter().collect() faster (unicode-org#6764)
    - (0.11.3) Implement `ZeroMapKV` for `VarTupleULE` (unicode-org#6750)
    - (0.11.3) Add `ZeroVec::truncated()` (unicode-org#6604)
    - (0.11.4) Fix safety issue in `ZeroVec::truncated()` (unicode-org#6805)
- `zoneinfo64`
    - (0.1.0) New crate
    - (0.2.0) Remove `icu_time` dependency (unicode-org#6914)
    - (0.2.0) Add gap offset data to `PossibleOffset::None` to help resolve forward transitions (unicode-org#6913)

## icu4x 2.0

ICU4X 2.0 is ICU4X's new major release. Many things have changed, we recommend going through the full changelog for 2.0, 2.0-beta1, and 2.0-beta2 to understand the changes fully.

This changelog entry *only* covers changes from ICU4X 2.0.0-beta2 to ICU4X 2.0.

Some major changes worth highlighting:
- Most locale-dependent APIs now take type-safe "preferences" objects instead of locales.
    - Preference bags are often built from a locale, but they can also be built manually or merged from other sources.
    - Preference bags are taken by value, so most call sites will need to change from `&locale.into()` to `locale.into()` or `(&locale).into()`
- The datetime formatting APIs have been completely redesigned for better data usage and performance. We recommend looking at the new API and using it from the ground up, rather than replacing API calls one at a time.

- Components
    - General
        - 1.82 MSRV (unicode-org#6413)
        - Fix reexport documentation to be clearer when things are reexports (unicode-org##6372)
        - Put `MaybeEncodeAsVarULE` impls behind the `"export"` feature (unicode-org#6221)
    - `icu_calendar`
        - Rename `Islamic` calendars to `Hijri` (unicode-org#6214)
        - Collapse `IslamicCivil` into `IslamicTabular` (unicode-org#6384)
        - Rename `IslamicObservational` to `IslamicSimulated` (unicode-org#6387)
        - Rename `wrap_calendar_in_*` APIs to `as_borrowed`, `into_ref_counted`, `into_atomic_ref_counted` (unicode-org#6392)
        - Make `am` era have index 0 and remove `bd` era from Coptic calendar (unicode-org#6458)
        - Update era codes to match CLDR/Temporal (unicode-org#6405, unicode-org#6525, unicode-org#6246)
        - Change cyclic calendar constructors to use ISO years (unicode-org##6431)
        - Fix Meiji start date (unicode-org##6432)
        - Mark `Calendar` `UnstableSealed` (unicode-org##6483)
        - Simplify `EraYear` (unicode-org##6443)
        - Allow different calendars to return different year-info types (unicode-org##6439)
        - Remove Umm-al-Qura data struct (unicode-org##6404)
        - Remove `prev_year` info (unicode-org#6382)
        - Use regions, not languages, for calendar algorithm defaults (unicode-org#6325)
        - Hide week-of-month, non-iso week-of-year (unicode-org#6319)
        - Expose `Date` to/from `RataDie` (unicode-org#6369)
    - `icu_casemap`
        - Add borrowed types (`CaseMapperBorrowed`, etc) to icu_casemap (unicode-org#6353)
    - `icu_collator`
        - Identical prefix optimization for the collator (unicode-org#6496)
        - Remove backward second level from the API (unicode-org#6291)
    - `icu_collections`
        - Remove some panics from `CodePointTrie`, which should no longer pull in panic machinery even with arithmetic panics enabled for lookup (unicode-org#6204)
    - `icu_datetime`
        - A lot of things have changed, it is preferable to reacquaint oneself with this crate from the ground up.
        - Make `.hm()` a constructor instead of a method (unicode-org#6423, unicode-org#6448)
        - Add `.time_hm[s]` on date fieldsets (unicode-org#6448)
        - Rename `.with_length` to `.for_length` (unicode-org#6448)
        - Length no longer has explicit `repr(u8)` discriminants (unicode-org#6423)
        - Fix incorrect datetime skeletons being used in datetime formatting (unicode-org#6428)
        - Split `DateTimeWriteError` and improve docs (unicode-org#6528)
        - `DateTime` field set API improvements (unicode-org#6448)
        - Bound `CldrCalendar` on `UnstableSealed` not `private::Sealed` (unicode-org#6475)
        - Remove `SubsecondError` and replace with Option API (unicode-org#6500)
        - Rename `ZonedDateTime::try_from_str to try_full_from_str` (unicode-org#6506)
        - Remove dead type DateTimeSkeletons and its data marker (unicode-org#6499)
        - Change ConflictingField error to contain the previously loaded field (unicode-org#6480)
        - Various refactors for comprehensibility (unicode-org#6423)
    - `icu_decimal`
        - No changes beyond cross-cutting changes
    - `icu_experimental`
      - `units`
        - Update unit conversion to accept unit constant denominator (unicode-org#6199)
        - Refactor `MeasureUnitParser` and update related components (unicode-org#6328)
      - `transliterator`
          - Add `Any-Lower` and `Any-Upper` transliterators (unicode-org#6260)
    - `icu_list`
        - No changes beyond cross-cutting changes
    - `icu_locale_core`
        - No changes beyond cross-cutting changes
    - `icu_locale`
        - Add `UNKNOWN` constants to locale types (unicode-org#6526)
        - Rename `is_default()` to `is_unknown()` (unicode-org#6507)
        - Remove `HourCycle::H24` (unicode-org#6426)
        - Remove `Language`/`LanguageIdentifier`/`Locale` `Default` impls (unicode-org#6529)
        - Remove BCP-47 APIs from `AnyCalendarKind`, use `CalendarAlgorithm` instead (unicode-org#6228)
    - `icu_pattern`
        - No update, still at `0.4.0`
    - `icu_plurals`
        - Make `PluralElements::map` take `FnMut`; add `try_map()` (unicode-org#6478)
        - Add `PluralElements` `[try_]for_each()`, `[try_]for_each_mut()` (unicode-org#6479)
    - `icu_properties`
        - Add `Vertical_Orientation`  (unicode-org#6290)
    - `icu_segmenter`
        - Reduce trait complexity (unicode-org#6417)
        - Reorganize module structure (unicode-org#6409, unicode-org#6441)
        - Add borrowed versions of segmenter types (unicode-org#6395)
        - Update UAX#29 text segmenter data rules to 16.0. (unicode-org#6367)
    - `icu_time`
        - A lot of things have changed, it is preferable to reacquaint oneself with this crate from the ground up.
        - Various refactors for comprehensibility (unicode-org#6460)
        - Rename `ZonedDateTime::try_from_str` to `try_full_from_str` (unicode-org#6506)
        - Add type `ZoneNameTimestamp` for time zone display names (unicode-org#6513)
        - `from_epoch_milliseconds_and_utc_offset` and follow-ups (unicode-org#6515)
        - Add `Time::noon()` (unicode-org#6536)
        - Make `DateTime`, `ZonedDateTime` `Clone` and `Copy` (unicode-org#6527)
- Data model and providers
    - General
        - Remove explicit icu_locale/icu_provider_baked deps from baked data (unicode-org#6340)
        - Add `#[non_exhaustive]` to most marker types (unicode-org#6401)
        - A lot of data markers have been renamed (unicode-org#6229, unicode-org#6508, unicode-org#6449)
    - `icu_provider`
        - Put `MaybeEncodeAsVarULE` behind the `"export"` feature (unicode-org#6221)
        - Move baked_provider scaffolding to `icu_provider`. Baked crates no longer need to depend directly on `icu_provider_baked`. (unicode-org#6351)
        - Clean up `icu_provider` docs (unicode-org#6226)
    - `icu_provider_adapters`
        - No changes aside from general changes
    - `icu_provider_baked`
        - Move baked_provider scaffolding to `icu_provider`. Baked crates no longer need to depend directly on `icu_provider_baked`. (unicode-org#6351)
    - `icu_provider_fs`
        - `AbstractSerializer` is now a sealed trait (unicode-org#6263)
        - Use marker path variant for `FsProvider` (unicode-org#6467)
    - `icu4x-datagen`
        - Improve ergonomics of icu4x-datagen (unicode-org#6476)
        - `CollationHanDatabase` -> `CollationRootHan` (unicode-org#6375)
    - `icu_provider_registry`
        - A lot of data markers have been renamed (unicode-org#6229, unicode-org#6508, unicode-org#6449)
    - `icu_provider_source`
        - `CollationHanDatabase` -> `CollationRootHan` (unicode-org#6375)
        - Fix kanji date formatting (by using dateSkeletons instead of dateFormats) (unicode-org#6428)
        - Validate calendar code against CLDR Era data (unicode-org#6252)
        - Generate `Any-Lower` and `Any-Upper` transliterators (unicode-org#6260)
        - Simplify calendars in datagen (unicode-org#6331)
        - Update icuexportdata (unicode-org#6522)
- FFI
    - `icu_capi`
        - This crate's API has been updated to continue reflecting ICU4X's API. ICU4X 2.0 breaking changes will likely be a breaking change here as well.
        - All `ICU4XFoo` types have now been unprefixed into `Foo`; backends like C++ use namespacing now.
    - Dart version `2.0.0-dev.0`
    - NPM version `2.0.0`
- Utils
    - General
        - Update most dependencies to latest major version (unicode-org#6266)
        - 1.82 MSRV (unicode-org#6413)
    - `bies`: `0.2.3 -> 0.2.4`
        - Update some dependencies (unicode-org#6266)
    - `calendrical_calculations`: `0.1.3 -> 0.2.0`
      - Add a `RataDie` public API for dealing with R.D. dates (unicode-org#6440)
      - Parametrise the `HijriSimulated` calendar by location (unicode-org#6332)
      - Use ISO dates for Chinese/Dangi epoch (unicode-org#6465)
      - Fix Saturday week wraparound logic at the 7-18-0 ḥalakim boundary (unicode-org#6267, unicode-org#6270)
    - `crlify`:
        - No update, still at `1.0.4`
    - `databake`, `databake-derive`
        - No update, still at `0.2.0`
    - `deduplicating_array`
        - No update, still at `0.1.8`
    - `fixed_decimal`: `0.7.0 -> 0.7.1`
        - Fix reexport documentation to be clearer when things are reexports (unicode-org#6372)
    - `ixdtf`: `0.4.0 -> 0.5.0`
        - Add TimeZoneParser and adjustments to `UtcOffsetRecord` (unicode-org#6283)
        - Fix for offset parsing leap seconds (unicode-org#6213)
        - Remove support for U+2212 (unicode-org#6207)
    - `litemap`: `0.7.5 -> 0.8.0`
        - Impl `Extend` for `LiteMap` and avoid quadratic behavior in from_iter and deserialize. This is breaking: it splits `StoreMut` into `StoreMut` and `StoreBulkMut`. (unicode-org#6132)
        - Fix test panics/segfaults in litemap on big-endian hosts (unicode-org#6293)
    - `potential_utf`:
        - No update, still at `0.1.2`
    - `tinystr`:
        - No update, still at `0.8.1`
    - `writeable`:
        - No update, still at `0.6.1`
    - `yoke`, `yoke-derive`:
        - No update, still at `0.8.0`
    - `zerofrom`, `zerofrom-derive`:
        - No update, still at `0.1.6`
    - `zerovec`: `0.11.1 -> 0.11.2`
        - Use `const` blocks (unicode-orgc#6300)
        - No update to `zerovec-derive`, still at `0.1.6`
    - `zerotrie`: `0.2.1 -> 0.2.2`
        - Fix `icu_locale_core` dep (unicode-org#6266)


## icu4x 2.0-beta2

- Components
    - General
        - Update data to CLDR 47 Beta 1, ICU 77 RC, TZDB 2025a
        - Some crates have been given the ability to be built without `alloc` (unicode-org#6077, unicode-org#6078, unicode-org#6166)
        - Consistently wrap all options in None in Rust and FFI (unicode-org#6084)
        - Options now consistently live in `options` modules (unicode-org#6138)
        - Remove `compiled_data` `Default` constructors from types which are prone to change sufficiently often (unicode-org#5958)
        - Move types over to uniform `::new()` constructor convention (unicode-org#6141)
        - Implement display on all error types (unicode-org#5969)
        - Use `core::error`, remove unused `std` features (unicode-org#5973)
        - Remove bench features (unicode-org#5986)
    - `icu_calendar`
        - Rename `IsoWeekday` to `Weekday` (unicode-org#6140)
        - Add `Date::wrap_calendar_in_ref` (unicode-org#6016)
        - `Time` and `DateTime` moved to `icu_time` component (unicode-org#5961)
        - Audit exhaustiveness of `icu_calendar::types` (unicode-org#5978)
        - Add calendar argument to IXDTF parsing, making it available without `compiled_data` feature (unicode-org#5982)
        - Use correct day of year for `Japanese` (unicode-org#5992)
    - `icu_casemap`
        - Add borrowed variants to casemapper types (unicode-org#6088)
        - Add missing keys to `CollatorPreferences` (unicode-org#5950)
    - `icu_collections`
        - Remove some panics (unicode-org#6052)
    - `icu_datetime`
        - Improvements to all-new semantic skeleta.
        - Add new datetime field set builder (unicode-org#5944)
        - Add `with_fset` fns to datetime types (unicode-org#5942)
        - Rename zone field sets (unicode-org#6096)
        - Rename `YearStyle::Always` to `YearStyle::WithEra` (unicode-org#6012)
        - Rename `format_any_calendar()` to `format()` (unicode-org#6020)
        - Add conversions between `Formatter` and `Names` (unicode-org#6110)
        - Renames: `FixedCalendarDateTimeNames`, `cast_into_fset` (unicode-org#6117)
        - Rename `FractionalSecondDigits` to `SubsecondDigits` (unicode-org#6139)
        - Rename `TimeFormatter` to `NoCalendarFormatter` (unicode-org#6175)
        - Remove ISO parsing specialisations (unicode-org#5998)
        - Make `field` module be unstable and add stable enums for `DateTimeNames` (unicode-org#5959)
        - Add exemplar cities format (`VVV`) (unicode-org#6018)
    - `icu_decimal`
        - Rename to `DecimalFormatter` (unicode-org#6075)
        - Add `numbering_system` getter to `DecimalFormatter` (unicode-org#5904)
        - Allow overriding the numbering system during decimal format (unicode-org#5914)
    - `icu_experimental`: `0.2.0 -> 0.3.0`
        - Implement Long/Short/Narrow Compact Currency Formatter (unicode-org#5456, unicode-org#5450)
        - More transliterator fixes; drop CLDR 46.0 support (unicode-org#6005)
        - Refactor MeasureUnit to use `single_units` instead of `contained_units` (unicode-org#6159)
    - `icu_list`
        - No changes other than general cross-crate changes
    - `icu_locale`
        - `LocaleExpander`, `LocaleDirectionality`, and `LocaleCanonicalizer` distinguish between `new_common()` and `new_extended()` constructors (unicode-org#5958)
    - `icu_locale_core` 
        - Stop accepting underscores as subtag separators (unicode-org#5943)
        - Make `LocalePreferences` internally-private, move `DataLocale` to `icu_locale_core`, other refactors (unicode-org#5988)
        - Fix `cmn-hans-cn-t-ca-u-ca-x_t-u` (unicode-org#6001)
    - `icu_normalizer`
        - Use longer lifetime when returning normalize iter (unicode-org#6060)
        - Introduce `split_normalized` and make `is_normalized_up_to` private in Rust (unicode-org#5932)
        - Make `utf16_iter`/`utf8_iter` deps optional (unicode-org#5927)
        - Return `Cow` instead of `String`/`Vec` (unicode-org#5906)
        - Change icuexportdata trie format to improve normalizer performance (unicode-org#5813)
    - `icu_pattern`: `0.3.0 -> 0.4.0`
        - Change `PlaceholderValueProvider` and parts behavior (unicode-org#5908)
    - `icu_plurals`
        - Move experimental plural ranges code into `provider` (unicode-org#6103)
    - `icu_properties`
        - Add to/from ICU4C APIs to enumerated properties (unicode-org#6091)
        - `GeneralCategoryGroup::contains` now accepts `self` by value (unicode-org#5952)
        - Fix minor unsafety internal to names API (unicode-org#6059)
        - Add convenience APIs that allow direct querying of properties (unicode-org#6066)
    - `icu_segmenter`
        - Segmenters that can take a content locale now specify `_root()` on their default localeless constructors (unicode-org#5958)
    - `icu_time`
        - Renamed from `icu_timezone`, with renames and restructures (unicode-org#6123, unicode-org#6152)
        - `Time` and `DateTime` moved to `icu_time` component (unicode-org#5961)
        - Replace `IxdtfParser` by methods on `ZonedDateTime` (unicode-org#5985)
        - Rename `CustomZonedDateTime` to `ZonedDateTime` (unicode-org#5983)
        - Human readable time zone transition dates (unicode-org#6124)
        - Allow iterating over all IANA IDs (unicode-org#6155)
        - Remove ISO parsing specialisations (unicode-org#5998)
        - Restrict `Time` to ISO-8601 range, remove arithmetic (unicode-org#6002)
- Data model and providers
    - General
        - Many improvements to internal data model for specific data keys (unicode-org#5994, unicode-org#6043, unicode-org#5813)
    - `icu_provider`
        - There is now a `data_marker!` macro (unicode-org#6072)
        - Remove `AnyProvider` (unicode-org#6086)
        - Remove YokeTraitHack from icu (unicode-org#6126)
        - Add a checksum field to `DataResponseMetadata` (unicode-org#6046)
        - Remove data marker paths from release binaries (unicode-org#5981)
        - Add `MaybeAsVarULE` and `MaybeEncodeAsVarULE` with plumbing into `DataProvider::<ExportMarker>::tokenize_encoded_seq` (unicode-org#6133)
    - `icu_provider_macros`
        - Removed
    - `icu_provider_baked`
        - Change `DataStore` to return `DataPayload` (unicode-org#6135)
    - `icu_provider_export`
        - Don't use the file system to pass export results (unicode-org#6087)
    - `icu_provider_source`
        - Download retries for `SourceDataProvider` (unicode-org#6092)
        - Improvements to timezone datagen (unicode-org#6023, unicode-org#6125, unicode-org#6105, unicode-org#6112, unicode-org#5966, unicode-org#6115)
- Utils
    - Cross cutting changes:
        - Use `core::error` (unicode-org#5973)
        - Remove bench feature. This is technically breaking, but not considered breaking since this is an internal feature. (unicode-org#5986)
    - `bies`:
        - No update, still at `0.2.3`
    - `calendrical_calculations`: `0.1.2 -> 0.1.3`
        - No changes other than general cross-crate changes
    - `crlify`
        - No update, still at `1.0.4`
    - `databake`, `databake_derive`
        - No update, still at `0.2.0`
    - `deduplicating_array`: `0.1.7 -> 0.1.8`
        - No changes other than general cross-crate changes
    - `fixed_decimal`: `0.6.0 -> 0.7.0`
        - Rename `FixedDecimal` to `Decimal`, introduce `UnsignedDecimal` (unicode-org#5667, unicode-org#6143, unicode-org#6146)
        - Add `trim_end_if_integer` (unicode-org#5903)
    - `ixdtf`: `0.3.0 -> 0.4.0`
        - Handle unbound fraction length (unicode-org#6036)
        - Bug fix for subsecond part (unicode-org#6004)
        - Expand size of duration records to support ECMA402 (unicode-org#5996)
    - `litemap`: `0.7.4 -> 0.7.5`
        - Implement `Entry` apis for `LiteMap` (unicode-org#6131)
        - Add `BTreeMap`-like apis to `LiteMap` (unicode-org#5894)
        - Make `litemap` no-alloc (unicode-org#6077)
    - `potential_utf`: `0.1.1 -> 0.1.2`
        - Add `PartialEq` (etc) impls for `PotentialUtf` types (unicode-org#5994)
    - `tinystr`: `0.8.0 -> 0.8.1`
        - Add explicit features list to Cargo.toml (unicode-org#6040)
        - Implement Borrow for `TinyAsciiStr` (unicode-org#6179)
    - `tzif`: `0.3.0 -> 0.4.0`
        - Preserve stucture of timezone designation list (unicode-org#5581)
    - `writeable`: `0.6.0 -> 0.6.1`
        - No changes other than general cross-crate changes
    - `yoke`, `yoke_derive`: `0.7.5 -> 0.8.0`
        - Relax trait bound for `EitherCart` in `Yoke` crate (unicode-org#5876)
        - Remove `YokeTraitHack` (unicode-org#6126)
    - `zerofrom`: `0.1.5 -> 0.1.6`
        - No changes other than general cross-crate changes
    - `zerotrie`: `0.2.0 -> 0.2.1`
        - Improved unsafe code (unicode-org#6054)
    - `zerovec`, `zerovec_derive`: `0.11.0 -> 0.11.1`
        - derive: Reduce number of panicky calls introduced by derive (unicode-org#6052)
        - Add `VarZeroSliceIter`, `ZeroSliceIter` (unicode-org#5924)
        - Add manual eyepatch to `VarZeroCow` (unicode-org#6189)

## icu4x 2.0-beta1

- Components
    - General
        - Constructors updated to take type-safe "preferences" objects instead of locales.
            - Preference bags are often built from a locale, but they can also be built manually or merged from other sources.
            - Preference bags are taken by value, so most call sites will need to change from `&locale.into()` to `locale.into()` or `(&locale).into()`
        - Compiled data updated to CLDR 45 and ICU 75 (unicode-org#4782)
        - Compiled data updated to CLDR 46 (unicode-org#5479, unicode-org#5598)
        - Fine-tuning error enums; removal of obsolete variants (unicode-org#4959, unicode-org#4638, unicode-org#5019, unicode-org#5041, unicode-org#5146)
        - Refactored FromStr and try_from_str functions (unicode-org#4934, unicode-org#5085)
        - Deleted various deprecated functions
        - Shadow `default` instead of making new function `const_default` (unicode-org#5354)
        - Rename marker types (unicode-org#4999)
        - Fix take/into function names and conventions (unicode-org#5723)
    - `icu`
        - Adding `datagen` feature to meta crate (unicode-org#5047)
    - `icu_calendar`
        - Restructure icu_calendar public API to put all calendars under `cal` (unicode-org#5701)
        - Consistently name calendar-specific `Date`/`DateTime` functions that have a calendar argument (unicode-org#5692)
        - Move all calendar types to `cal` module (unicode-org#5701)
        - Shorten integer types returned by `day_of_month()`, `week_of_month()`, and `week_of_year()` to `u8` (unicode-org#5702)
        - IntoAnyCalendar: new kind() method (unicode-org#4975)
        - Fixed various broken assertions (unicode-org#4986)
        - Fix Japanese calendar year 0 (unicode-org#5006)
        - Add Date, Time, and DateTime from str impls via IXDTF (unicode-org#5260)
        - Fix bug by consistently 1-indexing months and days (unicode-org#5726)
        - Refactor YearInfo to separate cyclic/Temporal/Formatting eras (unicode-org#5509)
        - FormattableMonth -> MonthInfo, and split standard from formatting month codes (unicode-org#5536)
        - Remove unix epoch APIs (unicode-org#5748)
        - Shorten integer type of day_of_month, week_of_month, week_of_year (unicode-org#5702)
    - `icu_collator`
        - Introduce a borrowed version of the collator (unicode-org#5513)
        - Adapt the collator to Unicode 16 normalization changes (unicode-org#4878)
        - Use script fallback in collator (unicode-org#5743)
    - `icu_collections`
    - `icu_normalizer`
        - Introduce borrowed variants of normalizer structs (unicode-org#5413)
        - Make the normalizer work with new Unicode 16 normalization behaviors (unicode-org#4860)
    - `icu_datetime`
        - All-new semantic skeletons. (unicode-org#1317, unicode-org#4945)
            - This is a re-write of `DateTimeFormatter` and most of the `icu_datetime` crate. It adds the ability to format specific subsets of fields, conforming with LDML version 46. All call sites of ICU4X 1.x `DateTimeFormatter` will need to be updated. It is recommended to start fresh with the new crate documentation.
        - Optimize YearNames storage (unicode-org#5721)
        - Drop support for week-of-year, to likely be re-added in a future version or upon request (unicode-org#5569)
        - Remove VVV time zone format, to possibly be added back later (unicode-org#5659)
        - Fix generic location format for single-tz countries (unicode-org#5657)
        - Support localized offsets with seconds (unicode-org#5674)
        - Audit datetime format function (unicode-org#5789)
    - `icu_decimal`
        - Reduce stack size of DecimalSymbolsV1 (unicode-org#5804)
        - Split numbering systems out of decimal data (unicode-org#5822, unicode-org#5830)
    - `icu_experimental`: `0.1.0 -> 0.2.0`
        - New experimental DurationFormatter component (unicode-org#801)
        - Implement `UnitsFormatter` (unicode-org#5000)
        - Implement Short Compact Currency Formatter Provider and Populate Associated Data (unicode-org#5361)
        - Implement `LongCurrencyFormatter` for Long Currency Formatting (unicode-org#5351)
        - Improvements to transliteration (unicode-org#5469, unicode-org#5489, unicode-org#5712)
        - Expose `CompactDecimalFormatterOptions` in the compactdecimal module (unicode-org#5770)
        - Add constant denominator support to MeasureUnit (unicode-org#6193)
    - `icu_locale`
        - New crate (with parts of `icu_locid` and `icu_locid_transform`)
        - Add preferences module (unicode-org#4996, unicode-org#5729)
        - Remove `AliasesV1`, `LikelySubtagsV1` (unicode-org#5337)
        - Remove `_manual_slice` constructors (unicode-org#5312)
        - Use `is_default` instead of `is_empty` or `is_und` for locale-ish types (unicode-org#5359)
        - Add script fallback priority; improve language/region fallback priority (unicode-org#5344)
        - Allow `LocaleDirectionality` to wrap a `LocaleExpander` with user-controlled storage (unicode-org#5704)
        - Allow `LocaleCanonicalizer` to wrap a `LocaleExpander` with user-controlled storage (unicode-org#5718)
        - Split `canonicalize()` on `Locale` and `LanguageIdentifier` into `canonicalize()` and `canonicalize_utf8()`, and have it return a `Cow` (unicode-org#5727)
        - Remove Ord impl from extensions::unicode::Unicode (unicode-org#5617)
        - Locale `canonicalize` -> `normalize` (unicode-org#5766)
        - Use `AsRef` in `LocaleDirectionality`'s type parameter (unicode-org#5704)
        - Add Expander argument to LocaleCanonicalizer (unicode-org#5718)
    - `icu_locale_core`
        - New crate, renamed from `icu_locid`
        - Removed `Ord` and `PartialOrd` impl from `extensions::unicode::Unicode` (unicode-org#5617)
        - Move generic Subtag to subtags; use it in Value (unicode-org#4932, unicode-org#4941)
        - Remove AsRef and instead introduce Cow-returning canonicalize methods on locale/langid (unicode-org#5727)
    - `icu_pattern`: `0.2.0 -> 0.3.0`
        - Changes to bytes constructors (unicode-org#5034, unicode-org#5072)
        - Bake, zerovec integration (unicode-org#5030)
    - `icu_plurals`
        - Add PluralElements for algorithmic plural selection (unicode-org#5622)
    - `icu_properties`
        - New properties API (unicode-org#5548)
        - Move exemplar chars with new API (unicode-org#5434)
        - Split `PropertyNames` (unicode-org#5575)
        - Model `BidiAuxiliaryProperties` as an `EnumeratedProperty` (unicode-org#5597)
        - Change `Script` property API (unicode-org#5628)
        - Rename `UnicodeSetData` to `EmojiSetData` (unicode-org#5627)
    - `icu_segmenter`
        - Support Unicode 15.1 for line segmenter (unicode-org#5218)
        - Use data marker attributes for model selection (unicode-org#4985)
        - Fixes to segmentation algorithm (unicode-org#5001)
        - Add LocaleData parameter for word/sentence segmenter (unicode-org#5318)
        - Add content_locale member to LineBreakOptions (unicode-org#5565)
    - `icu_timezone`
        - Implement `ixdtf` for `CustomTimeZone` and `CustomZonedDateTime` (unicode-org#5349)
        - GMT -> UTC (unicode-org#5512)
        - Clean up Windows mapper API (unicode-org#5735)
        - Support inference of time zone variants (unicode-org#5540)
        - Add `WindowsTimeZoneMapper` and `WindowsZonesToBcp47Map` provider (unicode-org#5524)
        - Handle `Z` correctly in IXDTF (unicode-org#5742)
        - Make an `IxdtfParser` type (unicode-org#5736)
        - Fewer overlapping constructors for `TimeZoneInfo` (unicode-org#5747)
        - Private fields for `TimeZoneInfo` (unicode-org#5747)
        - Return type for `ZoneOffsetCalculator`
        - Remove generic metazone values that match location values (unicode-org#5751)
        - Special-case `Z`, `Z[Etc/UTC]` and `Z[Etc/GMT]` in IXDTF parser (unicode-org#5757)
        - Deduplicate tz locations against root (unicode-org#5759)
        - Make `ZoneVariant` a closed, non-exhaustive enum (unicode-org#5760)
        - Replace `CustomTimeZone` with `TimeZoneInfo` (unicode-org#5691)
- Data model and providers
    - `icu_provider_adapters`
        - Reworking filter provider (unicode-org#5148)
        - Rename AnyPayloadProvider to FixedProvider, make it generic (unicode-org#5502)
        - impl ExportableProvider for ForkByErrorProvider and add tutorial (unicode-org#5503)
    - `icu_provider_baked`
        - New crate, split from `icu_datagen` (unicode-org#5009)
        - Reexport baked data dependencies in data crates (unicode-org#5033)
        - Changing baked data to use zerotrie (unicode-org#5064)
    - `icu_provider_blob`
        - Remove Blob schema v1, v2, add v3 (unicode-org#5608)
    - `icu_provider`
        - Replace `AnyPayloadProvider` with `FixedProvider<M>`
        - Added DataMarkerAttributes, new framework for data request specificity (unicode-org#4981)
        - Remove BufferProvider trait in favor of DynamicDataProvider (unicode-org#4992)
        - Rename data key to data marker [info] (unicode-org#5003)
        - Make `DataRequest.payload` non-optional (unicode-org#5037)
        - Remove `IterableDataProvider::supports_request` and rename `supported_requests` to `iter_requests` (unicode-org#5089)
        - Better modularisation in `icu_provider` (unicode-org#5088)
        - Refactor request architecture with DataIdentifier (unicode-org#5103, unicode-org#5293)
        - Create `DryDataProvider` for dry-run loading of data (unicode-org#5141)
        - Add DataPayload::dynamic_cast with example (unicode-org#5467)
        - Add `icu_provider::marker::ErasedMarker` (unicode-org#5590)
    - `icu_provider_export`
        - New crate, split from `icu_datagen` (unicode-org#5172)
        - Removed legacy datagen API (unicode-org#4962, unicode-org#5059)
        - Improved datagen APIs (unicode-org#5114, unicode-org#5629)
        - Move internal fallback option to baked exporter options (unicode-org#5036)
        - Allow using marker names in datagen (unicode-org#5060)
    - `icu_provider_fs`
    - `icu4x-datagen`
        - New crate, split from `icu_datagen` (unicode-org#5011)
    - `icu_provider_registry`
        - New crate, split from `icu_datagen` (unicode-org#5010, unicode-org#5177)
    - `icu_provider_source`
        - New crate, split from `icu_datagen` (unicode-org#5016, unicode-org#5173)
    - `icu_testdata`
        - Deleted obsolete crate (unicode-org#4960)
- FFI:
    - All languages
        - Complete revamp. "ICU4X" prefixes replaced with namespaces. Add version to ABI.
        - Remove `icu_` prefix from `icu_capi` features (unicode-org#5365)
        - Replace WASM-demo with Diplomat demo_gen output (unicode-org#5418)
    - JS
    - C++
    - `icu_harfbuzz`
- Utilities
    - `bies`: `0.2.2 -> 0.2.3`
        - Minor cleanups
    - `calendrical_calculations`: `0.1.2 -> 0.1.3`
        - Minor cleanups
    - `crlify`: No change (`1.0.4`)
    - `databake`, `databake_derive`: `0.1.8 -> 0.2.0`
        - Breaking: Introduce `BakeSize`, allowing for the calculation of the size of baked. (unicode-org#5169)
            - This is automatically required by `#[derive(Bake)]`, making it a breaking change
        - Fix `test_bake!` (unicode-org#5092, unicode-org#5559)
    - `deduplicating_array`: `0.1.6 -> 0.1.7`
        - Minor cleanups
    - `env_preferences`: New crate `0.1.0`
        - Add `env_preferences` crate (unicode-org#5081)
        - This crate does not itself get used by ICU4X, but can potentially be used with ICU4X.
    - `fixed_decimal`: `0.5.6 -> 0.6.0`
        - `FloatPrecision::Floating` renamed to `FloatPrecision::RoundTrip` (unicode-org#5616)
        - `FixedDecimal::concatenated_end()` now returns both `self` and `other` in the error case. (unicode-org#5623)
        - Simplify FixedDecimal's rounding APIs (unicode-org#5028)
        - fix: `pad_end` function does not accept the 0 position (unicode-org#5319)
        - Return `Err((self, other))` in FixedDecimal::concatenated_end (unicode-org#5623)
        - Rename FloatPrecision::Floating to FloatPrecision::RoundTrip (unicode-org#5616)
    - `ixdtf`: `0.2.0 -> 0.3.0`
        - Changed to `&[u8]` parsing (unicode-org#4918)
    - `litemap`: `0.7.3 -> 0.7.4`
        - Fix integer overflow for `LiteMap` by using correct `Store` trait method (unicode-org#5113)
    - `potential_utf`: New crate `0.1.0`
        - New crate with unvalidated types split from `zerovec` (unicode-org#5364)
    - `tzif`: New crate `0.2.3 -> 0.3.0`
        - Various breaking changes to APIs from "General" section above
    - `tinystr`: `0.7.6 -> 0.8.0`
        - Add UTF-16 constructors
        - Rename `TinyStrError` to `ParseError` (unicode-org#5405)
        - Add TinyAsciiStr::concat (unicode-org#5772)
        - Various breaking changes to APIs from "General" section above
    - `yoke`, `yoke_derive`: `0.7.4 -> 0.7.5`
        - Unsafe review feedback (unicode-org#5046, unicode-org#5104)
    - `zerofrom`, `zerofrom_derive: `0.1.4 -> 0.1.5`
        - Minor cleanups
    - `zerotrie`: `0.1.3 -> 0.2.0`
        - Minor improvement to zerotrie hash function (unicode-org#5106)
        - New bytes representation, downstream of `zerovec`'s new optimizations'
        - Various breaking changes to APIs from "General" section above
    - `zerovec`, `zerovec_derive`: `0.10.4, 0.10.3 -> 0.11.0`
        - This release has multiple changes that affect the bit representation of various types. Do not update to this release if you wish to retain stable data formats.
            - Change the `VarZeroVecFormat` values shipped by default to use the same index and length width. This breaks data layout for all `VarZeroVec`s. (unicode-org#5594)
            - Change the `VarZeroVec` format to not store a superfluous 0 index at the beginning of the index array. This breaks data layout for all `VarZeroVec`s (unicode-org#5601)
            - Optimize `MultiFieldsULE` to not store a length anymore. This breaks data layout for any `#[make_varule]`-using struct with multiple variable-sized fields. (unicode-org#5522, unicode-org#5593)
        - Enforce C,packed, not just packed (unicode-org#5049, unicode-org#5143)
        - Remove `FlexZeroVec` (unicode-org#5604)
        - Add VarTuple and VarTupleULE (unicode-org#5511)
        - Add TupleNVarULE (unicode-org#5777)
        - Add VarZeroCow (unicode-org#5809)
        - Add ZeroVec::as_slice (unicode-org#5621)
        - Serde impls for VarULE types (Var tuple types, and make_var) (unicode-org#5802)
        - Add VarZeroVecFormat support to VarTuple and make_var (unicode-org#5808)
        - Consistently use `bytes` not `byte_slice` (unicode-org#5816)
        - Implement `Bake` for VZV types of different formats (unicode-org#5719)
    - `writeable`: `0.5.5` -> `0.6.0`
        - Make `Writeable::writeable_cmp_bytes` a free function `writeable::cmp_bytes` (unicode-org#5737)
        - Add `writeable::to_string_or_borrow` for writing with reference bytes (unicode-org#5738)
        - Add Writeable WithPart helper (unicode-org#5328)
        - Make writeable_cmp_bytes a free function (unicode-org#5737)
        - Add writeable::write_or_ref (unicode-org#5738)
        - Generate to_string concrete fn in impl_display_with_writeable (unicode-org#5827)

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
  `icu_*_data`
  - (1.5.1) Add build.rs to workspace includes (unicode-org#6356)

## icu4x 1.5 (May 28, 2024)

- Components
  - General
    - Compiled data updated to CLDR 45 and ICU 75 (unicode-org#4782)
  - `icu_calendar`
    - Fix duration offsetting and negative-year bugs in several calendars including Chinese, Islamic, Coptic, Ethiopian, and Hebrew (unicode-org#4904)
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
    - Make UTS 46 normalization non-experimental (unicode-org#4712)
  - `icu_datetime`
    - Experimental "neo" datetime formatter with support for semantic skeleta and fine-grained data slicing (https://github.com/unicode-org/icu4x/issues/1317, https://github.com/unicode-org/icu4x/issues/3347)
    - `Writeable` and `Display` implementations now don't return `fmt::Error`s that don't originate from the `fmt::Write` anymore (unicode-org#4732, unicode-org#4851, unicode-org#4863)
    - Make `CldrCalendar` trait sealed except with experimental feature (https://github.com/unicode-org/icu4x/pull/4392)
    - `FormattedDateTime` and `FormattedZonedDateTime` now implement `Clone` and `Copy` (https://github.com/unicode-org/icu4x/pull/4476)
  - `icu_experimental`
    - New home for all experimental components. This supersedes the published `icu_compactdecimal`, `icu_displaynames`, `icu_relativetime`, `icu_transliterate`, and `icu_unicodeset_parse` crates (unicode-org#4564)
    - New experimental component `personnames` (unicode-org#4050)
    - New experimental component `dimension`
      - Added `CurrencyFormatter`, which can format any currency based on the locale and the width (short and narrow are supported for now).
    - New experimental component `units` (unicode-org#4605)
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
    - Add baked data macros to generate `IterableDataProvider` (unicode-org#4800, unicode-org#4868)
    - Add an option to generate data from an existing `DataProvider`, instead of from sources (unicode-org#4814)
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
      - Adding panic-handler and allocator features to icu_capi (unicode-org#4516)
    - JS
      - Fixed a bug where slice length is computed incorrectly (https://github.com/rust-diplomat/diplomat/pull/372)
      - Changed file extensions for better compatibility with nodejs modules (https://github.com/rust-diplomat/diplomat/pull/387)
    - C++
      - Fixed a bug where a result header defines a struct that shadows the class' name  (https://github.com/rust-diplomat/diplomat/pull/394)
      - Add `explicit` keyword to internal constructors (https://github.com/rust-diplomat/diplomat/pull/386)
      - Small breakage: some functions that used to return `diplomat::Result<T, std::monostate>` now return `std::optional<T>` (unicode-org#4635)
    - `icu_harfbuzz`
      - Switch to harfbuzz funcs (unicode-org#4794)
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
  - Remove dependency on `clap`'s `"cargo"` feature to better support non-Cargo users (unicode-org#3388)
- `icu_datagen` 1.2.5
  - Remove runtime dependency on segmenter data pulled from the cargo cache (unicode-org#3391)
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
    - Add missing `Debug` impls (unicode-org#3206)
    - Update Rust edition to 2021 (unicode-org#3075)
    - Internal clippy fixes
    - Unless otherwise specified, all crate updates are to version 1.2.
    - Out-of-cycle releases do not get their own changelog entries, so some entries may span multiple patch or pre-1.0 minor versions.
- Data model and providers
  - `icu_provider`:
    - Add support for silencing expected `DataError`s (unicode-org#3262)
    - Removing `dhat` dependency (unicode-org#3138)
    - Make trait `DataMarker: 'static` (unicode-org#3137)
  - `icu_datagen`: (includes patch updates 1.1.1 and 1.1.2)
    - (lib) Add `Out::Baked` and `BakedOptions`; deprecate `Out::Module` (unicode-org#3130)
    - (cli) Bump clap to 4.0, move to using derive (unicode-org#3149)
    - Pare down datagen deps (unicode-org#3160)
    - Support changes from CLDR 43 (unicode-org#3182, unicode-org#3201, unicode-org#3204, unicode-org#3205)
    - Add support for complemented range iterators (unicode-org#3198)
    - Using byte string literals in databake (unicode-org#3040)\
    - Datagen support for all new component features
    - (performance) Less `ZeroMap` mutation in datagen (unicode-org#3098)
  - `icu_provider_adapters`: No other changes
  - `icu_provider_blob`: No other changes
  - `icu_provider_fs`:
    -  Remove sha2 dep (unicode-org#3160)
  - `icu_provider_macros`: No other changes
- Components:
  - Cross component: No additional cross-component changes
  - `icu_calendar`
    - Document the bounds of `IsoSecond`, `Minute` and `Hour` (unicode-org#3156)
  - `icu_collator`: No other changes
  - `icu_collections`:
    - Add `to_u32` for TrieValue (unicode-org#3222)
    - Add `CPT::try_alloc_map_value` (unicode-org#3207)
    - Add support for coalescing range iterators (unicode-org#3198)
    - Allow inversion lists to be built from ranges that include `char::MAX` (unicode-org#3203)
  - `icu_datetime`: No other changes
  - `icu_decimal`
    - Add `From<GroupingStrategy>` for `FixedDecimalFormatterOptions` (unicode-org#3045)
  - `icu_list`
    - `ListJoinerPattern::from_parts_unchecked()` is now `from_parts()` and panics when necessary (unicode-org#3052)
  - `icu_locid`
    - Reduce size of internal `ShortVec` abstraction (unicode-org#3200)
    - Use `Box` in place of `Vec` in `ShortVec` (unicode-org#3220)
  - `icu_locid_transform`
    - The default set of likely subtags is now only the subset of languages that have a basic or greater CLDR coverage level; the full set is much larger in CLDR 43 and can be accessed via new constructors (unicode-org#3148, unicode-org#3158, unicode-org#3197)
  - `icu_normalizer`: No other changes
  - `icu_plurals`: No other changes
  - `icu_properties`
    - Add ability to obtain enumerated property value names from enum values (unicode-org#3222, unicode-org#3276)
    - Add ability to obtain enumerated property values from their names (unicode-org#3103, unicode-org#3152)
    - Add support for runtime-selected property lookup (unicode-org#3169)
    - Add support for complemented range iterators (unicode-org#3198)
    - Support data for Bidi properties with combined data structure (unicode-org#3026, unicode-org#3258)
  - `icu_segmenter`
    - Graduated from experimental
    - Add "auto" feature, enable "lstm" feature, establish new data key structure, improve error handling, and revise word and line segmenter constructors (unicode-org#3010, unicode-org#3267, unicode-org#3270)
    - Rename `icu_segmenter` enums (unicode-org#3285)
    - Allow access to rule status via word_type in WordSegmenter and over FFI (unicode-org#3139, unicode-org#3275)
    - Upgrade segmenter to Unicode 15.0.0 (unicode-org#3273)
    - Return a line break at index 0 (unicode-org#3289)
    - Improve the LSTM code and data model to be zero copy, improve error handling, be future-proof, and vectorize more operations (unicode-org#3210, unicode-org#3217, unicode-org#3233, unicode-org#3250, unicode-org#3254, unicode-org#3264, unicode-org#3291)
    - Remove ndarray dependency (unicode-org#3192)
    - Silencing expected `DataError`s (unicode-org#3262)
    - Fix SB10 rule (unicode-org#3126)
    - Polished docs and examples
  - `icu_timezone`: No other changes
 - Utils:
  - `crlify`: No change (still at 1.0.1)
  - `databake`: 1.1.3 -> 1.1.4
    - Using byte string literals in databake (unicode-org#3040)
  - `deduplicating_array`: 0.1.3 -> 0.1.4
  - `fixed_decimal`: 0.5.2 -> 0.5.3
  - `litemap`: 0.6.1 -> 0.7.0
    - Move FromIterator impl to new `StoreFromIterable` trait, allowing constructing `LiteMap`s with different backing stores (unicode-org#3220)
  - `pattern`: 0.1.3 -> 0.1.4
  - `tinystr`: No change (still at 0.7.1)
  - `tzif`: No change (still at 0.2.1)
  - `writeable`: 0.5.1 -> 0.5.2
  - `yoke`: 0.7.0 -> 0.7.1
  - `yoke-derive`: 0.7.0 -> 0.7.1
  - `zerofrom`: 0.1.1 -> 0.1.2
  - `zerofrom-derive`: 0.1.1 -> 0.1.2
  - `zerovec`: 0.9.2 -> 0.9.4
    - Add initial ZeroHashMap (unicode-org#2579)
    - Add `ZeroSlice::get_as_array()` (unicode-org#3192)
    - Add range dep of yoke to zerovec (unicode-org#3089)
  - `zerovec-derive`: 0.9.2 -> 0.9.4
    - Add `#[zerovec::derive(Hash)]` (unicode-org#2579)
    - Avoid using derive on autogenerated packed types (unicode-org#3069)
 - FFI:
    - Feature support
      - Property value-to-name mappings (unicode-org#3196)
      -  `UnicodeSets` (including exemplar chars) (unicode-org#3177)
      -  Runtime-selected property lookup (unicode-org#3169)
      -  Property lookup ranges and GeneralCategoryGroup (unicode-org#3230)
      -  LocaleExpander: Add extended and non-extended constructors (unicode-org#3197)
      -  Fill in `BreakIterator` API (unicode-org#3275)
      -  Bidi reorder_visual (unicode-org#3183)
      -  (experimental) Strongly typed display names API (unicode-org#3190, unicode-org#3188)
    - Add feature slicing to `icu_capi` (unicode-org#3216)
    - Better FFI provider ownership (unicode-org#3140)
 - Experimental:
   - `bies`: 0.2.0 -> 0.2.1
   - `icu_casemap`: 0.7.1 -> 0.7.2
   - `icu_compactdecimal`: 0.1.0 -> 0.2.0
    - Support configurable grouping separators in CompactDecimalFormatter (unicode-org#3045)
   - `icu_displaynames`: 0.8.0 -> 0.10.0
     - Add ScriptDisplayNames (unicode-org#3317)
     - Add LanguageDisplayNames with support for variants (unicode-org#3058, unicode-org#3113)
     - Add stronger typing (unicode-org#3190)
   - `icu_harfbuzz`: New experimental port: Harfbuzz integration for ICU4X (v0.1.0)
   - `icu_relativetime`: 0.1.0 -> 0.1.1


## icu4x 1.1 (Jan 26, 2023)

* `icu_calendar`
  * Fix bug in `simple_week_of()` around unit size (unicode-org#2951)
  * Fix math in calendar (unicode-org#2714)
  * Add `div_rem_euclid` and use it in icu_calendar (unicode-org#2704)
  * Fix Time::from_minute_with_remainder_days to handle negatives (unicode-org#2643, unicode-org#2702)
  * doc improvements

* `icu_casemap`
  * doc improvements

* `icu_collator`
  * doc improvements

* `icu_collections`
  * Add APIs for returning exemplar characters data (unicode-org#2812)
  * Readable JSON inversion lists (unicode-org#2855)
  * Add `UnicodeSet` that supports strings (unicode-org#2796)
  * Add documentation on `CodePointTrie` details and perf considerations (unicode-org#2717)

* `icu_codepointtrie_builder`
  * internal improvements

* `icu_datetime`
  * internal and doc improvements

* `icu_decimal`
  * internal and doc improvements

* `icu_displaynames`
  * DisplayNames fixes (unicode-org#2918)
  * Rename `Territory` -> `Region` for display names component (unicode-org#2895)
  * Transformer code for `Language` display names (unicode-org#2871)
  * Adding a function to get display name for a region. (unicode-org#2816)
  * Transformer code for display names component. (unicode-org#2635)
  * doc improvements

* `icu_list`
  * Untangling list provider from logic and fixing big endian safety bug (unicode-org#2994)
  * Not allocating `Writeable`s for regex evaluation (unicode-org#2991)
  * doc improvements

* `icu_locid`
  * Reject duplicated extensions (unicode-org#2893)
  * More borrowing in locid's `write_to_string` (unicode-org#2693)
  * doc improvements

* `icu_locid_transform`
  * Clean up dependency specifications so `serde` isn't pulled in by default (unicode-org#2696)
  * doc improvements

* `icu_normalizer`
  * internal and doc improvements

* `icu_plurals`
  * doc improvements

* `icu_properties`
  * Add APIs for returning exemplar characters data (unicode-org#2812)
  * Add API and testdata for `Basic_Emoji` property (unicode-org#2802)
  * Add `UnicodeSet` that supports strings (unicode-org#2796)
  * Update `Script` property value enums (unicode-org#2787)
  * doc improvements

* `icu_segmenter`
  * Make metacrate features more specific (unicode-org#2932)
  * Remove `serde` dependency from segmenter with `lstm` feature. (unicode-org#2904)
  * Simplify construction of grapheme cluster break iterators (unicode-org#2870)
  * Store grapheme cluster payload instead of grapheme cluster segmenter. (unicode-org#2864)
  * `#[no_std]` for LSTM segmenter (unicode-org#2845)
  * icu_segmenter: enforce `clippy::indexing_slicing`. (unicode-org#2325)
  * Use `GraphemeClusterSegmenter` in `DictionarySegmenter` and `LstmSegmenter` (unicode-org#2716)
  * Rename `*BreakSegmenter` to `*Segmenter` (unicode-org#2707)
  * Remove unnecessary language check for East Asian language (SA property) (unicode-org#2705)
  * internal and doc improvements

* `icu_timezone`
  * Adds a bytes parsing API for `GMTOffset` for `CustomTimeZone` and FFI (unicode-org#2943, unicode-org#2955)
  * doc improvements

* `icu_provider_adapters`
  * Add more `inner_mut` functions in `icu_provider_adapters` (unicode-org#2987)
  * Fix error propagation in `MultiForkByErrorProvider` (unicode-org#2986)
  * Add mutation methods to `MultiForkByErrorProvider` (unicode-org#2972)

* `icu_provider_blob`
  * internal and doc improvements

* `icu_provider`
  * Setting correct `DataError` for `.as_deserializing()`, `.as_downcasting()` (unicode-org#2993)
  * doc improvements

* `icu_datagen`
  * Removing experimental feature from datagen (unicode-org#3005)
  * Fixing Spanish list regex (unicode-org#2989)
  * Datagen CLI improvements (unicode-org#2950)
  * Some reexports for datagen (unicode-org#2958)
  * Databake improvements (unicode-org#2906)
  * Exclude certain collations by default and add option to include them (unicode-org#2789)
  * Allowing no keys in datagen CLI (unicode-org#2731)
  * Fixing baked datagen for no keys and keys with no data (unicode-org#2698)
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
  * Treat strings starting with a decimal point as valid in `FixedDecimal::from_str()` (unicode-org#2937)
  * The actual formatting part of compact decimal formatting (unicode-org#2898)
  * Allow noncompact `CompactDecimal` (unicode-org#2889)
  * `CompactDecimal` and `ScientificDecimal` (unicode-org#2847)
  * doc improvements

* `icu_pattern`
  * internal improvements

* `litemap`
  * internal and doc improvements

* `tinystr`
  * Add `std` feature and `Error` impl for `TinyStrError` (unicode-org#3009)
  * internal and doc improvements

* `tzif`
  * internal and doc improvements

* `writeable`
  * Using `core` integer log when available (unicode-org#3015)
  * `usize` and `isize` implementation
  * internal and doc improvements

* `yoke`
  * Add `prove_covariance_manually` guard for `CoerceUnsized` (unicode-org#2936)
  * Allow `clippy::forget_copy` in `derive(Yokeable)` impl (unicode-org#2775)
  * Fix soundness issue in `Yoke::attach_to_cart()` around implied bounds #2949

* `zerovec`
  * `ZeroMap2d` cursors

## Bug fixes in 1.0.x

### icu_collator 1.0.1

- Made case level setting actually take effect (unicode-org#2892)

## icu4x 1.0 (Sept 27, 2022)

- General
  - Update license to Unicode-DFS-2016 (unicode-org#2303)
  - Major improvements to documentation, bechmarks, and examples
  - Various performance and codesize improvements
  - FFI for all non-experimental components
- Data model and providers
  - Polished baked data provider (unicode-org#2098, unicode-org#2126, unicode-org#2147)
  - Data key extraction from binary (unicode-org#1950)
  - Add `LocaleFallbacker` with locale fallback algorithm (unicode-org#2036, unicode-org#2115, unicode-org#2186, unicode-org#2567)
  - Making `DataProvider: Sync + Send` (unicode-org#1853)
  - Update to Postcard 1.0 (unicode-org#2037, unicode-org#2091, unicode-org#2438)
  - De-duplication in `BlobDataProvider` (unicode-org#2062)
  - Move `map_project` to closures (unicode-org#2185)
  - Renaming of many traits and functions in `icu_provider` (unicode-org#2207, unicode-org#2222, unicode-org#2223)
  - Passing `DataLocale` by reference (unicode-org#2224)
  - Fix feature specification in provider/fs (unicode-org#2527)
  - Rename `DataKey` methods and return `DataKeyPath` (unicode-org#2565)
  - Add some useful data provider impls; refactor `AnyPayloadProvider` (unicode-org#2564)
  - Removing `StaticDataProvider` (unicode-org#2582)
  - Removing `InvariantDataProvider` (unicode-org#2159)
  - Renaming load_payload, load_resource (unicode-org#2222)
  - Renaming `DataOptions` to `DataLocale` (unicode-org#2223)
  - Use an abstract predicate function in `ForkByKeyProvider` (unicode-org#2249)
  - Add `UnvalidatedStr` and use it in `LocaleFallbackParentsV1` (unicode-org#2502)
  - Add some useful data provider impls; refactor `AnyPayloadProvider` (unicode-org#2564)
- Components:
  - Cross component:
    - `Format` to `Formatter` rename (unicode-org#2184)
    - Uniform constructor style across all components, see #2573 (unicode-org#2293, unicode-org#2305, unicode-org#2309, unicode-org#2316, unicode-org#2318, unicode-org#2326, unicode-org#2327, unicode-org#2329, unicode-org#2330, unicode-org#2332, unicode-org#2333, unicode-org#2334)
    - Remove `format_to_write`s (unicode-org#2528)
    - Make error enums more consistent (unicode-org#2649)
    - More Copy arguments (unicode-org#2654)
  - `calendar`
    - Emit month codes from calendars  (unicode-org#2053)
    - Add `Date::new_from_codes()`; fix up per-calendar constructor functions (unicode-org#2255)
    - Fix iso-to-fixed conversion (unicode-org#1898)
    - Ethiopic calendars (unicode-org#1831, unicode-org#1902)
    - Replace hour/minute/second constructors `new_unchecked()` with getter `number()` (unicode-org#1922)
    - Improve and rename `types::Year`/`types::Month` (unicode-org#2157)
    - Add `japanext` calendar (unicode-org#2181)
    - Replace unbounded arithmetic for calendar numeric types with bounded arithmetic. (unicode-org#2273)
    - Make `Japanext` its own calendar type (unicode-org#2311)
    - Pick default calendar based off of locale in `AnyCalendar`
    - Make `offset_date` handle wraparounds for months (unicode-org#2373)
    - Hide duration stuff, rename `IncludedInAnyCalendar` (unicode-org#2426)
    - `week_of` refactoring (unicode-org#2462)
    - Fix arithmetic in Indian calendar (unicode-org#2479)
    - Infallible `from_minutes_since_local_unix_epoch()` (unicode-org#2646)
  - `collator`
    - New component (unicode-org#1706)
    - Validate the length of last_primaries (unicode-org#1916)
    - Use a higher numeric value for `Strength::Identical` (unicode-org#1942)
    - Move and unescape collator and normalizer tests (unicode-org#1943)
    - Tweak CollationMetadataV1 documentation and dead code (unicode-org#1914)
    - GIGO fix-ups for the normalizer and the collator (unicode-org#1931)
    - split_first_u16/split_first_u24 -> split_first (unicode-org#2459)
    - Create options bag for CollatorOptions (unicode-org#2475)
    - Clean up FFFD magic numbers in Collator with REPLACEMENT_CHAR (unicode-org#2496)
    - Add traditional spanish and plumbing to make it work (unicode-org#2497)
  - `collections`
    - New component (unicode-org#2294, unicode-org#2323, unicode-org#2328, unicode-org#2336)
    - Rename `CodePointSet` to `CodePointInversionList` (unicode-org#2230)
    - Allow `CodePointTrie` to determine `error_value` at runtime from data (unicode-org#2301)
    - Use GIGO with debug assertion in Char16Trie (unicode-org#2537)
  - `datetime`
    - Formatting for `AnyCalendar`s (unicode-org#1987, unicode-org#2146)
    - Renaming `DateTimeFormatter` (etc) to `TypedDateTimeFormatter` and `AnyDateTimeFormatter` to `DateTimeFormatter` (unicode-org#2298)
    - DateFormatter cleanups (unicode-org#2304)
    - Remove Calendar type parameter from `TimeFormat` (unicode-org#2282)
    - Class Hierarchy for `DateTimeFormat` (split into `DateFormat`, `TimeFormat`, etc) (unicode-org#2133)
    - Making `time_granularity` public (unicode-org#1867)
    - Add fractional seconds support to components bag (unicode-org#1873)
    - Use `FixedDecimalFormat` in `DateTimeFormat` (unicode-org#1952)
    - Include module name to disambiguate Pattern (unicode-org#1889)
    - Use month codes in formatting (unicode-org#2071)
    - Split date and time data keys. (unicode-org#2093)
    - Move `Formatted[Zoned]DateTime` over to preextracting the date time input info (unicode-org#2138, unicode-org#2205)
    - Remove `MockZonedDateTime` (unicode-org#2231)
    - Add an offset_fallback field in `TimeZoneFormatV1` (unicode-org#2253)
    - Remove `HourCycle` from the public Lengths API (unicode-org#2331)
    - Move mock datetime parsing code to test modules (unicode-org#2436)
    - Stop returning error on mismatched locale and type calendar (unicode-org#2477)
    - Change default length to medium (unicode-org#2596)
    - Make expect_pattern GIGO (unicode-org#2650)
  - `decimal`
    - Don't panic on invalid grouping sizes (unicode-org#2042)
    - Remove signum and sign display options (unicode-org#2070)
    - Add numbering system support (unicode-org#2246)
  - `list`
    - ListStyle -> ListLength and add _with_length (unicode-org#2628)
  - `locid`
    - Add `remove()` for vertical fallback (unicode-org#1992)
    -  Update `Locale` and `LanguageIdentifier` comparison functions to `strict_cmp()` and `normalizing_eq()` (unicode-org#2020)
    -  `normalizing_eq()`, `strict_cmp()` for LSRV subtags (unicode-org#2048)
    -  Add `strict_cmp_iter()` (unicode-org#2111, unicode-org#2114)
    -  Removing auto-derived Ord impl for Locale/LangId (unicode-org#2142)
    -  Enable `locale` macro to support single unicode key value pair extension (unicode-org#2382)
    -  Reducing `locid_id` API surface (unicode-org#2484)
    -  `private::Key` and `other::Key` to `::Subtag` (unicode-org#2632)
  - `locid_transform`
    - Rename from `icu::locale_canonicalizer` (unicode-org#2381)
    - `LocaleCanonicalizer`/`LocaleExpander` refactor (unicode-org#2338)
  - `normalizer`
    - Promoted from experimental (unicode-org#2058)
    - Add ComposingNormalizer for NFC, NFKC, and UTS 46 (unicode-org#2039)
    - GIGO fix-ups for the normalizer and the collator (unicode-org#1931)
    - Add support for NFKD and the decomposed counterpart of UTS 46 without ignored and disallowed (unicode-org#1967)
    - Simplify Hangul composition (unicode-org#2200)
    - Make sink-writing normalization methods non-experimental (unicode-org#2201)
    - Uses tries instead of inversion lists for normalization data (unicode-org#2235)
    - Consolidate the two auxiliary tries to the main NFD trie (unicode-org#2371)
    - Use `char` instead of `U24` in normalizer data (unicode-org#2481)
    - Make NFKD and UTS 46 data store only the difference form NFD (unicode-org#1984)
  - `plurals`
    - Rename `select()` to `category_for()` for `PluralRules` (unicode-org#2287)
    - Use From instead of TryFrom for signed integers (unicode-org#2593)
    - `from_tr35_string` -> `get_for_cldr_string` (unicode-org#2633)
    - Make PluralOperands fields private, add static constructor (unicode-org#2598)
  - `properties`
    - Better properties return values (unicode-org#2112, unicode-org#1990, unicode-org#2277, unicode-org#2555)
    - Move properties data over to an (extensible) enum (unicode-org#2140)
    - Renaming unicode property data struct names (unicode-org#2198)
  - `timezone`
    - New component, split from `datetime` (unicode-org#2265)
    - Add time period metazone to `TimeZonesProvider` (unicode-org#1961)
    - Convert metazone period from string to i32 (unicode-org#2085)
    - Improvements to `MetaZoneCalculator` (unicode-org#2274)
    - Add `TimeVariant` wrapper (unicode-org#2289)
    - TimeVariant -> ZoneVariant with a few more docs (unicode-org#2427)
    - Assorted TimeZone fixes (unicode-org#2478)
 - Utils:
  - `crlify`: No updates
  - `databake`:
    - Moved over from `crabbake` (unicode-org#2068)
    - Some databake improvements (unicode-org#2150)
    - Using static `LiteMap`s in databake (unicode-org#2264)
  - `deduplicating_array`: No updates
  - `fixed_decimal`:
    - Switch FixedDecimal to a trivaluate sign (unicode-org#2025)
    - Remove negate (unicode-org#2060)
    - Improve integer operations (unicode-org#1924)
    - Add `FixedDecimal::concatenate_right()` (unicode-org#1953)
    - Implement `ceil()`, `floor()` and `truncate()` functions (unicode-org#1923)
    - Define "magnitude" and introduce "position" concept (unicode-org#1981)
    - Support for rounding modes (unicode-org#2000, unicode-org#2100, unicode-org#2104, unicode-org#2261)
    - Make `multiply_pow10)_` be infallible (unicode-org#2285)
  - `litemap`:
    - Remove `serde_json` dep from zeromap/litemap and align features (unicode-org#1939)
    - `LiteMap` of `&'a [(K, V)]` (unicode-org#2242)
    - Enable `ShortVec` as a backend for `LiteMap` (unicode-org#2356)
  - `pattern`: No updates
  - `tinystr`:
    - Make `Option<TinyAsciiStr>` be the same size as `TinyAsciiStr` (unicode-org#2430)
  - `tzif`:
    - New crate (unicode-org#2019)
    - Parse POSIX time-zone strings using Combine (unicode-org#1973)
    - Parse TZif binary files using Combine (unicode-org#1999)
  - `writeable`:
    - Rename `write_len` (unicode-org#2529)
  - `yoke`:
    - Deprecate yoke's `badly` methods (unicode-org#1930)
    - Rename `Yoke::project()` functions to `::map_project()` (unicode-org#1955)
    - Remove stable_deref_trait/alloc from yoke's default feature set (unicode-org#2094)
    - Move `map_project()` to closures (unicode-org#2185)
  - `zerofrom`: No updates
  - `zerovec`:
    - Make `VarZeroVec` format configurable (unicode-org#2306)
    - Add `FlexZeroVec` (unicode-org#1790)
    - Add `NicheBytes` trait and `NichedOptionULE` (unicode-org#2501)
    - Turn ZeroVec into a struct for optimization (unicode-org#2599, unicode-org#2622)
    - Improve performance of VarZeroVec::deserialize and add provider benches (unicode-org#2603)
    - Add array impl for `ZeroMapKV` (unicode-org#1875)
    - Remove lifetime from `ZeroVecLike` (unicode-org#1901)
    - `ZeroVecLike` cleanup (unicode-org#2024)
    - Remove `serde_json` dep from zeromap/litemap and align features (unicode-org#1939)
    - Make various ZeroVec methods `const` (unicode-org#1976)
    - Refactor ZeroMap2d and add get_by functions (unicode-org#1876)
    - Add more zerovec impls for `usize` and `FlexZeroVec` (unicode-org#2023)
    - Change charULE from 4 bytes to 3 bytes (unicode-org#2015)
    - More impls in zerovec crate (unicode-org#2054)
    - Add binary search and other utilities to `FlexZeroVec` (unicode-org#2284)
    - Remove `KeyError` and rename `get()` to `get_2d()` (unicode-org#2279)
    -  `EncodeAsVarULE` for `Cow` (unicode-org#2376)
    -  Add `ExactSizeIterator` for `FlexZeroVec::iter_*()` (unicode-org#2580)
    -  Add permutation to ZVL containers (unicode-org#2605)
 - FFI:
   - All non-experimental components now covered by FFI
   - Add FFI error strategy (unicode-org#2045)
   - Configurable DataProvider FFI (unicode-org#2526)
 - Experimental:
   - `bies`:
   - `casemapping`:
   - `segmenter`:
     - Expose `RuleBreakIterator` as a public interface (unicode-org#2408)
     - Merge `segmenter_lstm` with segmenter (unicode-org#2087)
     - Use `CodePointTrie` in Segmenter (unicode-org#1839)
     - Move language detection to language.rs (unicode-org#1689)
     - Simplify function in rule_segmenter (unicode-org#1880)
     - Use dictionary segmenter for word. (unicode-org#1936)
     - Remove std dependency from segmenter_lstm. (unicode-org#2064)
     - Add Lao and Khmer LSTM models (unicode-org#2120)
     - Use multiple dictionaries for line/word segmenter. (unicode-org#2209)
     - Add a feature option not to use unicode-segmentation (unicode-org#2212)
     - Remove two char types in line segmenter and polish utf8 iterator naming (unicode-org#2269)

## icu4x 0.6.0 (May 9, 2022)

  - General data model
    - Non-exhaustive errors for locid, calendar, decimal, plurals (unicode-org#1792, unicode-org#1793)
    - Rename "serialize" feature to "serde" (unicode-org#1797)
    - Turn all errors into Copy types (unicode-org#1657)
  - Components
    - `calendar`:
      - Coptic, Indian and Ethiopian calendars (unicode-org#1660, unicode-org#1715, unicode-org#1779)
      - Calendar arithmetic (unicode-org#1614)
    - `datetime`:
      - Formatting for fractional seconds (unicode-org#1813, unicode-org#1801)
      - Support for day of week in month ('F') (unicode-org#1770)
      - Custom fallbacking for TimeZoneFormatter (unicode-org#1591)
      - Support for week-of-month (unicode-org#1468)
      - Bug fix to get_best_available_format_pattern skeleton matching logic (unicode-org#1549)
    - `decimal`: No updates
    - `locale_canonicalizer`:
      - ZeroCopy support (unicode-org#1760, unicode-org#1777)
    - `locid`:
      - Simplified language representation (unicode-org#1695)
      - Region, Script and Variant subtags ULE (unicode-org#1696)
    - `plurals`:
      - Update data model to use `ZeroVec` (unicode-org#1240)
    - `properties`:
      - Bidi support (unicode-org#1716, unicode-org#1784)
  - Utilities
    - `codepointtrie`:
      - Use 0 for error value for Rust out-of-bounds for primitive trie value types (unicode-org#1804)
    - `crlify`: New util for line ending conversions
    - `deduplicating_array`: No updates
    - `fixed_decimal`:
      - Improvements to FixedDecimal f64 APIs (unicode-org#1718)
    - `litemap`:
      - Pluggable LiteMap backends (unicode-org#1769)
    - `pattern`: No updates
    - `uniset`: No updates
    - `writeable`: No updates
    - `yoke`: No updates
    - `zerofrom`: No updates
    - `zerovec`:
      - ZeroVec derive improvements (unicode-org#1780)
      - Support non-Ord values in ZeroMap (unicode-org#1743)
      - Add OptionULE and OptionVarULE (unicode-org#1736)
      - Rename ZeroVec::from_slice and add new method for const-constructed ZeroSlice (unicode-org#1728)
      - Treat ZeroMap sort order as an optional invariant (unicode-org#1727)
      - Add ZeroMap::get_copied_by (unicode-org#1722)
      - Generalize PairULE to support longer tuples (unicode-org#1721)
      - Add more AsULE impls for primitives (unicode-org#1672)
      - Add cast methods to ZeroVec and ZeroSlice (unicode-org#1651)
      - Add RawBytesULE::slice_from_byte_slice (unicode-org#1648)
      - Create façades for ZeroVec types, hide internal code organization modules (unicode-org#1629)
      - Add zerovec::skip_kv and zerovec::skip_ord attributes, as well as generalized attribute handling framework (unicode-org#1613)
      - Rename as_unaligned to to_unaligned (unicode-org#1619)
  - FFI:
    - Updating to Diplomat 0.3
    - Making testdata an optional FFI dep (unicode-org#1820)
    - Split out capi targets: make separate freertos, staticlib, and cdylib crates as targets (unicode-org#1747)
  - Experimental:
    - `crabbake`: Initial version of baked data provider (unicode-org#1825)
    - `segmenter`:
      - Support production-ready data provider for segmenters (unicode-org#1652)
      - Implement dictionary based segmenter for line segmenter. (unicode-org#1644)
      - Wire DataProvider into UAX29 segmenters (unicode-org#1627)
      - Move UAX#14 defines to line.toml (unicode-org#1568)
      - Add segmenter factories to generate UAX29 iterators (unicode-org#1602)


## icu4x 0.5.0 (Jan 31, 2022)

  - General data model
    - `DataPayload` no longer needs a lifetime (unicode-org#1297, unicode-org#1279)
    - Re-write DataKey (unicode-org#1511)
    - Rewrite ErasedDataProvider as AnyProvider (unicode-org#1495)
    - Add EitherProvider and rename IterableDataProviderCore to IterableProvider (unicode-org#1455)
    - Change DataRequest to be borrowed in BufferProvider (unicode-org#1416)
    - Replace SerdeDeDataProvider with BufferProvider (unicode-org#1369, unicode-org#1384)
  - Components
    - `calendar`:
      - Julian, Japanese, and Buddhist calendars (unicode-org#1351, unicode-org#1394, unicode-org#1305)
      - `DateTimeFormat` integration (unicode-org#1339)
      - Bugfix around arithmetic (unicode-org#1352)
    - `datetime`:
      - Week-of-year support (unicode-org#1206)
      - `DateTimeFormat::resolve_components()` (unicode-org#1362)
      - Era formatting (unicode-org#1346)
      - `TimeZoneFormatterConfig` (unicode-org#1256)
      - New data model for organizing calendar data (unicode-org#1300)
      - Bugfix around missing localized strings in time zone data (unicode-org#1405)
    - `decimal`: No updates
    - `locale_canonicalizer`:
      - Bugfix in maximization (unicode-org#1171)
      - Update data model to use `LiteMap` (unicode-org#1275)
    - `locid`: No updates
    - `plurals`:
      - Update data model to use `ZeroVec` (unicode-org#1240)
    - `properties`:
      - Rename resource key category for properties (unicode-org#1406)
      - Rename enums for `General_Category` (unicode-org#1355)
      - Implement the `Canonical_Combining_Class` property (unicode-org#1347)
      - Implement `Script_Extensions` property (unicode-org#1353)
      - Add `General_Category` predicate functions (unicode-org#1310)
      - Implement `Grapheme_Cluster_Break`, `Word_Break`, and `Sentence_Break` Unicode properties (unicode-org#1233)
  - Utilities
    - `codepointtrie`: No changes
    - `deduplicating_array`: New utility for efficient serialized representation of data with duplicates
    - `fixed_decimal`:
      - Padding and truncation APIs (unicode-org#1482, unicode-org#1507, unicode-org#1195)
      - Add double-to-decimal via ryū (unicode-org#1217)
      - Handle exponents in `FixedDecimal::from_str()` (unicode-org#1265)
    - `litemap`:
      - Add `LiteMap::get_indexed()` and `LiteMap::find_index()`
      - Handle serialization of tuples (etc) in litemaps (unicode-org#1306)
    - `pattern`: No updates
    - `uniset`: No updates
    - `writeable`:
      - Adding parts functionality to `Writeable` (unicode-org#1438)
      - Change `Writeable::writeable_to_string` to return a Cow (unicode-org#1452)
      - Implementing `Writeable` for all integers (unicode-org#1408)
      - Making `writeable::LengthHint` a range (unicode-org#1400)
      - Simplifying `assert_writeable_parts_eq` and `assert_writeable_eq` (unicode-org#1522, unicode-org#1399)
    - `yoke`:
      - Remove `Yokeable::Output` from `ZeroCopyFrom` trait (unicode-org#1499)
      - Add `EitherCart` (unicode-org#1484)
      - Remove `attach_to_option_cart()` (unicode-org#1348)
      - Homogenize yoke generic impls to always work with `Yokeable`, add `OwnedYokeable` (unicode-org#1302)
    - `zerovec`:
      - Move over to a model where the vector types deref to `ZeroSlice` and `VarZeroSlice` (unicode-org#1418, unicode-org#1371)
      - Simplify `ZeroVec`/`VarZeroVec` error handling, consolidate `ULEError` type (unicode-org#1389)
      - Simplify `ZeroMapKV` (unicode-org#1334)
      - Add `ZeroMap2d`, a two-dimensional zero-copy map (unicode-org#1432)
      - Add borrowed-only version of `ZeroMap` (unicode-org#1238)
      - Add various helper functions to zerovec (unicode-org#1430)
      - Rename `PlainOldULE` to `RawBytesULE` (unicode-org#1413)
      - Improve `EncodeAsVarULE` (unicode-org#1385)
      - Add `EncodeAsVarULE` for `ZeroVec` (unicode-org#1274, unicode-org#1407)
      - Various trait impls (unicode-org#1332, unicode-org#1330, unicode-org#1328, unicode-org#1287)
      - Document ULE alignment guarantee; update all impls with checklists (unicode-org#1294)
      - Fix `PairULE` validation function (unicode-org#1266)
  - FFI:
    - Updating to the latest Diplomat
    - Add FFI for constructing Data Structs, including decimal data structs (unicode-org#1497)
    - Add padding/truncation to FFI (unicode-org#1501)
    - Add FFI for constructing fixed decimals from float (unicode-org#1483)
    - Properties FFI (unicode-org#1269)
  - Experimental:
    - New ListFormatter experiment
    - More progress on segmentation experiment

## icu4x 0.4.0 (November 1, 2021)

  - Updated to CLDR 40 (unicode-org#1216)
  - Functional Unicode property APIs and data; moved to new crate `icu_properties` (unicode-org#148)
  - `DateTimeFormat` optimization: memory usage cut by two-thirds and perf improved by 40% (unicode-org#519)
  - `BlobDataProvider` allowing for dynamic, asynchronous data loading (unicode-org#1084)
  - Major upgrades to the `zerovec` utility crate and new benchmarks for `ZeroMap` (unicode-org#1082, unicode-org#1087)
  - Initial support for time zones in `DateTimeFormat` components bag (unicode-org#845)
  - Bug fixes in `LocaleCanicalizer` (unicode-org#1189, etc.)
  - Week of month/year arithmetic (unicode-org#918)
  - Major improvements on code size across FFI (unicode-org#962)
  - Iterations on ICU4X Segmenter (unicode-org#1045, unicode-org#1165, etc.)
  - Experimental `ListFormatter` (unicode-org#1053)
  - ICU4X now requires Rust 1.56 (unicode-org#1201)

## icu4x 0.3.0 (July 29, 2021)

  - Static data provider without filesystem dependency (unicode-org#78)
  - Infrastructure around zero-copy deserialization, including `ZeroVec` and `Yoke` (unicode-org#652, unicode-org#667)
  - Most ICU4X components support `no_std` (unicode-org#812)
  - Hour cycle preferences are used by DateTimeFormat (unicode-org#671)
  - Skeleton matching supports additional field widths (unicode-org#584)
  - Add canonicalize method to LocaleCanonicalizer (unicode-org#747)
  - Added range iterator on UnicodeSet (unicode-org#836)
  - Add PluralRules::categories() function (unicode-org#789)
  - Consolidated data generation tool into new binary `icu4x-datagen`

## icu4x 0.2.0 (April 29, 2021)
  - Updated CLDR to 39 (unicode-org#640)
  - Initial release of:
    - `icu_decimal` (unicode-org#590)
    - `icu_locale_canonicalizer` (unicode-org#417)
  - `DateTimeFormat` gets support for
    - day periods (unicode-org#435)
    - time zones (unicode-org#418)
    - components bag (unicode-org#481)
  - `UnicodeSet` gets preliminary support for L3a subset (unicode-org#478)
  - `PluralRules` support `E` operand (unicode-org#407)
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
