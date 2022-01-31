# Changelog


## Unreleased


## icu4x 0.5.0 (Jan 31, 2022)

  - General data model
    - `DataPayload` no longer needs a lifetime (#1297, #1279)
    - Re-write ResourceKey (#1511)
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
      - `TimeZoneFormatConfig` (#1256)
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
      - Implement data transformer for `Script_Extensions` map data (#1353)
      - Add `General_Category` predicate functions (#1310)
      - Implement `Grapheme_Cluster_Break`, `Word_Break`, and `Sentence_Break` Unicode properties (#1233)
      - Add fn to return UnicodeSet for Script_Extensions (#1529)
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
