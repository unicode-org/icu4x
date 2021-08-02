# Changelog

## Unreleased

  - …

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
