

# Crates
=====================


## FFI

- Fix an issue in JS bindings where enums in objects were not parsed correctly (unicode-org#7885)

## General

- Updated data to TZDB 2026c (unicode-org#8200)

## icu_calendar

- Add `AnyCalendarKind::try_new` and deprecate  `AnyCalendarKind::new`. The new version uses locale data to infer calendars from locales. (unicode-org#8102)
- Deprecate `CalendarPreferences::resolve_calendar`. This method did not perform likely-subtags expansion. (unicode-org#8102)

## icu_casemapping

- Fix `TrailingCase::Unchanged` handling for Dutch (unicode-org#7863)

## icu_collator_data`/`icu_provider_source

- Fixed an issue where the emoji collation was not loading correctly (unicode-org#7989)

## icu_datetime

- Use the correct calendar even if the region is only implied by the language (i.e. `fa`) (unicode-org#8102)

## icu_locale_core

- `preferences` types now implement `databake` (feature-gated) (unicode-org#8102)

## writeable

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
- Fix emoji collation (https://github.com/unicode-org/icu4x/pull/7989)
- Infer region in calendar resolution (https://github.com/unicode-org/icu4x/pull/8102)
- Infer region in calendar resolution (https://github.com/unicode-org/icu4x/pull/8102)
- Infer region in calendar resolution (https://github.com/unicode-org/icu4x/pull/8102)
- writeable: impl TryWriteable on references, Either (https://github.com/unicode-org/icu4x/pull/8109)
- 2026c (https://github.com/unicode-org/icu4x/pull/8200)


# N/A
=====================

- fix Send/Sync impls on CartableOptionPointer, yoke 0.8.3 (https://github.com/unicode-org/icu4x/pull/8029)
