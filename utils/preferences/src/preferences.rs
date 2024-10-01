// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[doc(hidden)]
pub use icu_locale_core;

/// TODO
pub trait PreferenceKey {
    /// TODO
    fn unicode_extension_key() -> Option<icu_locale_core::extensions::unicode::Key> {
        None
    }

    /// TODO
    fn matches_ue_key(key: &icu_locale_core::extensions::unicode::Key) -> bool {
        Self::unicode_extension_key() == Some(*key)
    }

    /// TODO
    fn is_custom(&self) -> bool {
        false
    }

    /// TODO
    fn unicode_extension_value(&self) -> Option<icu_locale_core::extensions::unicode::Value> {
        None
    }
}

#[macro_export]
/// TODO
macro_rules! preferences {
    (
        $(#[$doc:meta])*
        $name:ident,
        $resolved_name:ident,
        {
            $(
                $(#[$key_doc:meta])*
                $key:ident: $pref:ty
            ),*
        }
     ) => (
        #[derive(Default, Debug, Clone)]
        $(#[$doc])*
        #[non_exhaustive]
        pub struct $name {
            #[doc = concat!("The locale that these `", stringify!($name), "` use.")]
            pub lid: Option<$crate::preferences::icu_locale_core::LanguageIdentifier>,
            $(
                $(#[$key_doc])*
                pub $key: Option<$pref>,
            )*
        }

        #[non_exhaustive]
        #[derive(Debug, Clone)]
        #[doc = concat!("The resolved version of `", stringify!($name), "`.")]
        pub struct $resolved_name {
            #[doc = concat!("The locale that these `", stringify!($name), "` use.")]
            pub lid: $crate::preferences::icu_locale_core::LanguageIdentifier,

            $(
                $(#[$key_doc])*
                pub $key: $pref,
            )*
        }

        impl From<$crate::preferences::icu_locale_core::Locale> for $name {
            fn from(loc: $crate::preferences::icu_locale_core::Locale) -> Self {
                use $crate::preferences::PreferenceKey;

                let lid = Some(loc.id);

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

                Self {
                    lid,
                    $(
                        $key,
                    )*
                }
            }
        }

        impl $name {
            /// Constructs a `Locale` corresponding to these preferences.
            pub fn into_locale(self) -> $crate::preferences::icu_locale_core::Locale {
                use $crate::preferences::PreferenceKey;
                let id = self.lid.unwrap_or_default();
                let mut extensions = $crate::preferences::icu_locale_core::extensions::Extensions::new();
                $(
                    if let Some(value) = &self.$key {
                        if let Some(ue) = <$pref>::unicode_extension_key() {
                            let val = value.unicode_extension_value().unwrap();
                            extensions.unicode.keywords.set(ue, val);
                        }
                    }
                )*
                $crate::preferences::icu_locale_core::Locale {
                    id,
                    extensions
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

        impl $resolved_name {
            /// TODO
            pub fn extend(&mut self, prefs: $name) {
                $(
                    if let Some(v) = prefs.$key {
                        self.$key = v;
                    }
                )*
            }
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enum_keyword;
    use icu_locale_core::Locale;

    #[test]
    fn test_preferences() {
        #![allow(dead_code)]

        enum_keyword!(DummyKeyword {
            "default" => Default,
        }, "ab");

        preferences!(
            /// Preferences for the dummy formatter
            DummyPreferences,
            DummyResolvedPreferences,
            {
                /// Controls how dummyly the formatter behaves
                dummy_keyword: DummyKeyword
            }
        );

        let loc: Locale = "und-u-ab-default-cd-foo".parse().unwrap();

        let prefs = DummyPreferences::from(loc);
        assert_eq!(prefs.dummy_keyword, Some(DummyKeyword::Default));
    }
}
