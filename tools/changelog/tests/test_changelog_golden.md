

# Crates
=====================

- Components
  - General
    - Updated data to TZDB 2026c (unicode-org#8200)
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
- FFI
  - General
    - Fix an issue in JS bindings where enums in objects were not parsed correctly (unicode-org#7885)
- Utils
  - `writeable`
    - impl TryWriteable on references (unicode-org#8109)
    - impl TryWriteable on Either (unicode-org#8109)


# PRs with additional notes
=====================



# no changelog found
=====================

## docs(writeable): clarify to_string doc wording (https://github.com/unicode-org/icu4x/pull/8167)
Updates the doc comment generated for Writeable to_string fns per the wording suggested by @robertbastian in #8140.

Closes #8164


# Potentially misformatted (double check please!)
=====================

- Bump diplomat (https://github.com/unicode-org/icu4x/pull/7885)
- Initial person names formatter (https://github.com/unicode-org/icu4x/pull/9006)
- Person names titles support (https://github.com/unicode-org/icu4x/pull/9007)
- Infer region in calendar resolution (https://github.com/unicode-org/icu4x/pull/8102)
- Infer region in calendar resolution (https://github.com/unicode-org/icu4x/pull/8102)
- Infer region in calendar resolution (https://github.com/unicode-org/icu4x/pull/8102)
- writeable: impl TryWriteable on references, Either (https://github.com/unicode-org/icu4x/pull/8109)
- 2026c (https://github.com/unicode-org/icu4x/pull/8200)
- Add negative currency subpatterns (https://github.com/unicode-org/icu4x/pull/9001)
- Fix Julian leap year (https://github.com/unicode-org/icu4x/pull/9003)


# N/A
=====================

- fix Send/Sync impls on CartableOptionPointer, yoke 0.8.3 (https://github.com/unicode-org/icu4x/pull/8029)
