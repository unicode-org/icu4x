

# Crates
=====================

- Components
  - General
    - Updated data to TZDB 2026c (unicode-org#8200)
    - Updated to Unicode 18 (unicode-org#9010)
  - `icu_calendar`
    - Add `AnyCalendarKind::try_new` and deprecate  `AnyCalendarKind::new`. The new version uses locale data to infer calendars from locales. (unicode-org#8102)
    - Deprecate `CalendarPreferences::resolve_calendar`. This method did not perform likely-subtags expansion. (unicode-org#8102)
    - Fix bug in leap year calculation for Julian calendar (unicode-org#9003)
  - `icu_casemapping`
    - Fix `TrailingCase::Unchanged` handling for Dutch (unicode-org#7863)
  - `icu_datetime`
    - Use the correct calendar even if the region is only implied by the language (i.e. `fa`) (unicode-org#8102)
    - Improved performance of datetime formatting (unicode-org#9002)
      - Optimized cache lookup
      - Reduced allocations in helper functions
        - Avoided cloning locale in hot path
  - `icu_experimental`
    - Renamed data marker `UnitIdsV1` to `UnitsIdV1`. (unicode-org#9009)
  - `icu_experimental/currency`
    - Add formatting support for negative currency subpatterns (unicode-org#9001)
  - `icu_experimental/personnames`
    - Add initial implementation of person names formatter (unicode-org#9006)
    - Add support for titles and honorifics in person names (unicode-org#9007)
  - `icu_list`
    - Add support for custom list styles (unicode-org#9004)
      - Allowed users to pass custom templates
      - Added validation for templates
        - Checked for matching placeholders
        - Verified order of placeholders
  - `icu_locale_core`
    - `preferences` types now implement `databake` (feature-gated) (unicode-org#8102)
  - `icu_properties`
    - Update properties data for Unicode 16.0 (unicode-org#9005)
      - Loaded new property files
      - Updated script definitions
        - Added new scripts
        - Updated aliases for existing scripts
    - Add `Block` enumerated property (unicode-org#9008)
  - `icu_segmenter`
    - Fix a panic on empty input (unicode-org#9008)
      - New method: `LineSegmenter::is_empty`
    - Fix line breaks at the end of Southeast Asian runs (unicode-org#9011)
      - Breaks at the end of Thai runs now follow the UAX #14 rules, so lines no longer break before closing punctuation.
      - Thai followed by Latin letters no longer breaks.
- FFI
  - General
    - Fix an issue in JS bindings where enums in objects were not parsed correctly (unicode-org#7885)
- Utils
  - `retired_crate`
    - Retired (unicode-org#9010)
  - `writeable`
    - impl TryWriteable on references (unicode-org#8109)
    - impl TryWriteable on Either (unicode-org#8109)
  - `zerovec`
    - Add `ZeroVec::frobnicate` (unicode-org#9010)
  - `zerovec_derive`
    - Minor soundness improvement (unicode-org#9012)


# PRs with additional notes
=====================



# no changelog found
=====================

## docs(writeable): clarify to_string doc wording (https://github.com/unicode-org/icu4x/pull/8167)
Updates the doc comment generated for Writeable to_string fns per the wording suggested by @robertbastian in #8140.

Closes #8164


# Potentially misformatted (double check please!)
=====================



# N/A
=====================

- fix Send/Sync impls on CartableOptionPointer, yoke 0.8.3 (https://github.com/unicode-org/icu4x/pull/8029)
- Template comment with N/A (https://github.com/unicode-org/icu4x/pull/9013)
