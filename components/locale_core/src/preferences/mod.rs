// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This API provides necessary functionality for building user preferences structs.
//!
//! This includes the ability
//! to `merge` information between the struct and a [`Locale`] and facilitate resolution of the
//! attributes against default values.
//!
//! The crate is intended primarily to be used by components constructors to normalize the format
//! of ingesting preferences across all of [`ICU4X`].
//!
//! [`ICU4X`]: ../icu/index.html
//! [`Locale`]: crate::Locale

pub mod extensions;
mod options;

#[doc(inline)]
pub use options::define_options;

/// TODO
pub trait PreferenceKey {
    /// TODO
    fn unicode_extension_key() -> Option<crate::extensions::unicode::Key> {
        None
    }

    /// TODO
    fn matches_ue_key(key: &crate::extensions::unicode::Key) -> bool {
        Self::unicode_extension_key() == Some(*key)
    }

    /// TODO
    fn is_custom(&self) -> bool {
        false
    }

    /// TODO
    fn unicode_extension_value(&self) -> Option<crate::extensions::unicode::Value> {
        None
    }
}

#[macro_export]
#[doc(hidden)]
/// TODO
macro_rules! __define_preferences {
    (
        $(#[$doc:meta])*
        $name:ident,
        {
            $(
                $(#[$key_doc:meta])*
                $key:ident: $pref:ty
            ),*
        }
     ) => (
        $(#[$doc])*
        #[derive(Default, Debug, Clone)]
        #[non_exhaustive]
        pub struct $name {
            pub(crate) language: $crate::subtags::Language,
            pub(crate) script: Option<$crate::subtags::Script>,
            pub(crate) region: Option<$crate::subtags::Region>,
            pub(crate) variant: Option<$crate::subtags::Variant>,
            pub(crate) subdivision: Option<$crate::subtags::Subtag>,
            pub(crate) ue_region: Option<$crate::subtags::Region>,

            $(
                $(#[$key_doc])*
                pub $key: Option<$pref>,
            )*
        }

        impl From<$crate::Locale> for $name {
            fn from(loc: $crate::Locale) -> Self {
                $name::from(&loc)
            }
        }

        impl From<&$crate::Locale> for $name {
            fn from(loc: &$crate::Locale) -> Self {
                use $crate::preferences::PreferenceKey;

                $(
                    let mut $key = None;
                )*

                for (k, v) in loc.extensions.unicode.keywords.iter() {
                    $(
                      if <$pref>::matches_ue_key(k) {
                          if let Ok(r) = TryInto::<$pref>::try_into(v.clone()) {
                              if !r.is_custom() {
                                  $key = Some(r);
                              }
                          }
                      }
                    )*
                }

                let sd = loc
                    .extensions
                    .unicode
                    .keywords
                    .get(&$crate::extensions::unicode::key!("sd"))
                    .and_then(|v| v.as_single_subtag().copied());
                let ue_region = loc
                        .extensions
                        .unicode
                        .keywords
                        .get(&$crate::extensions::unicode::key!("rg"))
                        .and_then(|v| v.as_single_subtag()
                            .and_then(|s| $crate::subtags::Region::try_from_str(s.as_str()).ok()));
                Self {
                    language: loc.id.language,
                    script: loc.id.script,
                    region: loc.id.region,
                    variant: loc.id.variants.iter().copied().next(),
                    subdivision: sd,
                    ue_region,

                    $(
                        $key,
                    )*
                }
            }
        }

        impl From<$crate::LanguageIdentifier> for $name {
            fn from(lid: $crate::LanguageIdentifier) -> Self {
                $name::from(&lid)
            }
        }

        impl From<&$crate::LanguageIdentifier> for $name {
            fn from(lid: &$crate::LanguageIdentifier) -> Self {
                Self {
                    language: lid.language,
                    script: lid.script,
                    region: lid.region,
                    variant: lid.variants.iter().copied().next(),
                    subdivision: None,
                    ue_region: None,

                    $(
                        $key: None,
                    )*
                }
            }
        }

        impl From<&icu_provider::DataLocale> for $name {
            fn from(loc: &icu_provider::DataLocale) -> Self {
                Self {
                    language: loc.language,
                    script: loc.script,
                    region: loc.region,
                    variant: loc.variant,
                    subdivision: loc.subdivision,
                    ue_region: None,

                    $(
                        $key: None,
                    )*
                }
            }
        }

        impl $name {
            /// Constructs a `Locale` corresponding to these preferences.
            pub fn into_locale(self) -> $crate::Locale {
                use $crate::preferences::PreferenceKey;
                $crate::Locale {
                    id: $crate::LanguageIdentifier {
                        language: self.language,
                        script: self.script,
                        region: self.region,
                        variants: self
                            .variant
                            .map($crate::subtags::Variants::from_variant)
                            .unwrap_or_default(),
                    },
                    extensions: {
                        let mut extensions = $crate::extensions::Extensions::default();
                        if self.subdivision.is_some() || self.ue_region.is_some() {
                            if let Some(sd) = self.subdivision {
                                extensions.unicode.keywords.set(
                                    $crate::extensions::unicode::key!("sd"),
                                    $crate::extensions::unicode::Value::from_subtag(Some(sd))
                                );
                            }
                            if let Some(rg) = self.ue_region {
                                extensions.unicode.keywords.set(
                                    $crate::extensions::unicode::key!("rg"),
                                    $crate::extensions::unicode::Value::try_from_str(rg.as_str()).unwrap()
                                );
                            }
                        }
                        $(
                            if let Some(value) = &self.$key {
                                if let Some(ue) = <$pref>::unicode_extension_key() {
                                    let val = value.unicode_extension_value().unwrap();
                                    extensions.unicode.keywords.set(ue, val);
                                }
                            }
                        )*
                        extensions
                    },
                }
            }

            /// TODO
            pub fn extend(&mut self, other: $name) {
                $(
                    if let Some(value) = other.$key {
                        self.$key = Some(value);
                    }
                )*
            }

            /// TODO
            pub fn remove_custom(&mut self) {
                use $crate::preferences::PreferenceKey;
                $(
                    if let Some(k) = &self.$key {
                        if k.is_custom() {
                            self.$key = None;
                        }
                    }
                )*
            }
        }
    )
}
#[doc(inline)]
pub use __define_preferences as define_preferences;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::preferences::extensions::unicode::enum_keyword;
    use crate::Locale;

    #[test]
    fn test_preferences() {
        #![allow(dead_code)]

        enum_keyword!(DummyKeyword {
            "default" => Default,
        }, "ab");

        define_preferences!(
            /// Preferences for the dummy formatter
            DummyPreferences,
            {
                /// Controls how dummyly the formatter behaves
                dummy_keyword: DummyKeyword
            }
        );

        let loc: Locale = "und-u-ab-default-cd-foo".parse().unwrap();

        let prefs = DummyPreferences::from(&loc);
        assert_eq!(prefs.dummy_keyword, Some(DummyKeyword::Default));
    }
}
