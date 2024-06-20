# tinystr [![crates.io](https://img.shields.io/crates/v/tinystr)](https://crates.io/crates/tinystr)

<!-- cargo-rdme start -->

## `icu_preferences`

`icu_preferences` is a utility crate of the [`ICU4X`] project.

This API provides necessary functionality for building user preferences structs with ability
to `merge` information between the struct and a [`Locale`] and facilitate resolution of the
attributes against default values.

The crate is intended primarily to be used by components constructors to normalize the format
of ingesting preferences across all of [`ICU4X`].

## Examples:

```rust
use icu_preferences::{
  preferences,
  preferences::PreferenceKey,
  extensions::unicode::keywords::HourCycle,
};
use icu_locale_core::LanguageIdentifier;

pub fn get_defaults(lid: &Option<LanguageIdentifier>) -> ExampleComponentResolvedPreferences {
    unimplemented!()
}

preferences!(
    ExampleComponentPreferences,
    ExampleComponentResolvedPreferences,
    {
        hour_cycle => HourCycle
    }
);

pub struct ExampleComponent {
    resolved_prefs: ExampleComponentResolvedPreferences,
}

impl ExampleComponent {
    pub fn new(prefs: ExampleComponentPreferences) -> Self {
        // Retrieve the default values for the given [`LanguageIdentifier`].
        let mut resolved_prefs = get_defaults(&prefs.lid);

        // Resolve them against provided preferences.
        resolved_prefs.extend(prefs);

        Self { resolved_prefs }
    }
}
```

[`ICU4X`]: ../icu/index.html
[`Locale`]: icu_locale_core::Locale

<!-- cargo-rdme end -->

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
