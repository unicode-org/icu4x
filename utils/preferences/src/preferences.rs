// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locale_core::extensions::unicode::Key;

pub trait PreferenceKey {
    fn unicode_extension_key() -> Option<Key> {
        None
    }

    fn matches_ue_key(key: &Key) -> bool {
        Self::unicode_extension_key() == Some(*key)
    }

    fn is_custom(&self) -> bool {
        false
    }

    fn unicode_extension_value(&self) -> Option<icu_locale_core::extensions::unicode::Value> {
        None
    }
}

#[macro_export]
macro_rules! preferences {
    ($name:ident,
     $resolved_name:ident,
     {$($key:ident => $pref:ty),*}
     ) => (
        #[derive(Default, Debug)]
        #[non_exhaustive]
        pub struct $name {
            pub lid: Option<icu_locale_core::LanguageIdentifier>,
            $(
                pub $key: Option<$pref>,
            )*
        }

        #[non_exhaustive]
        #[derive(Debug, Clone)]
        pub struct $resolved_name {
            pub lid: icu_locale_core::LanguageIdentifier,

            $(
                pub $key: $pref,
            )*
        }

        impl From<(icu_locale_core::LanguageIdentifier, $resolved_name)> for $resolved_name {
            fn from(input: (icu_locale_core::LanguageIdentifier, $resolved_name)) -> Self {
                Self {
                    lid: input.0,
                    $(
                        $key: input.1.$key,
                    )*
                }
            }
        }

        impl From<icu_locale_core::Locale> for $name {
            fn from(loc: icu_locale_core::Locale) -> Self {

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

        #[allow(non_local_definitions)] // Locale is in a different crate
        impl From<$name> for icu_locale_core::Locale {
            fn from(input: $name) -> icu_locale_core::Locale {
                let id = input.lid.unwrap_or_default();
                let mut extensions = icu_locale_core::extensions::Extensions::new();
                $(
                    if let Some(value) = input.$key {
                        if let Some(ue) = <$pref>::unicode_extension_key() {
                            let val = value.unicode_extension_value().unwrap();
                            extensions.unicode.keywords.set(ue, val);
                        }
                    }
                )*
                icu_locale_core::Locale {
                    id,
                    extensions
                }
            }
        }

        #[allow(non_local_definitions)] // Locale is in a different crate
        impl From<&$resolved_name> for icu_locale_core::Locale {
            fn from(_input: &$resolved_name) -> icu_locale_core::Locale {
                todo!()
            }
        }

        impl $name {
            pub fn extend(&mut self, other: $name) {
                $(
                    if let Some(value) = other.$key {
                        self.$key = Some(value);
                    }
                )*
            }

            pub fn remove_custom(&mut self) {
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
        enum_keyword!(DummyKeyword {
            "default" => Default,
        }, "ab");

        preferences!(
            DummyPreferences,
            DummyResolvedPreferences,
            {
                dummy_keyword => DummyKeyword
            }
        );

        let loc: Locale = "und-u-ab-default-cd-foo".parse().unwrap();

        let prefs: DummyPreferences = loc.into();
        assert_eq!(prefs.dummy_keyword, Some(DummyKeyword::Default));
    }
}
