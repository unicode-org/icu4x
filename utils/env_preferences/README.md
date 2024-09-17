<!-- cargo-rdme start -->

# env_preferences

`env_preferences` is a crate to retrieve system locale and preferences for
Apple, Linux & Windows systems

It currently fetches locales for the operating system
currently in `String` format.

In the current setup, it is not ensured that the locale retrieved will be
converted to [`ICU4X Locale`](https://crates.io/crates/icu_locale)

It also retrieves preferences for [`Calendar`](https://crates.io/crates/icu_calendar)
& [`TimeZone`](https://crates.io/crates/icu_timezone)

<!-- cargo-rdme end -->
