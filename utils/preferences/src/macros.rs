// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[macro_export]
macro_rules! preferences {
    ($name:ident,
     $resolved_name:ident,
     {$($key:ident => $pref:ty, $resolved:ty, $ue:expr),*}
     ) => (
        #[derive(Default, Debug)]
        #[non_exhaustive]
        pub struct $name {
            pub lid: Option<icu_locid::LanguageIdentifier>,
            $(
                pub $key: $pref,
            )*
        }

        #[non_exhaustive]
        #[derive(Clone, Debug)]
        pub struct $resolved_name {
            pub lid: icu_locid::LanguageIdentifier,

            $(
                pub $key: $resolved,
            )*
        }

        impl From<(LanguageIdentifier, &$resolved_name)> for $resolved_name {
            fn from(input: (LanguageIdentifier, &$resolved_name)) -> Self {
                Self {
                    lid: input.0,
                    $(
                        $key: input.1.$key,
                    )*
                }
            }
        }

        impl From<Locale> for $name {
            fn from(loc: Locale) -> Self {
                let lid = Some(loc.id);

                $(
                    let mut $key = None;
                )*

                for (k, v) in loc.extensions.unicode.keywords.iter() {
                    $(
                      if let Some(ue) = &$ue {
                          if k == ue {
                              if let Ok(r) = TryInto::try_into(v) {
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
            pub fn extend<T: Into<$name>>(&mut self, other: T) {
                let other = other.into();
                $(
                    if let Some(value) = other.$key {
                        self.$key = Some(value);
                    }
                )*
            }
        }

        impl $resolved_name {
            pub fn extend(&mut self, prefs: &$name) {
                $(
                    if let Some(v) = prefs.$key {
                        self.$key = v;
                    }
                )*
            }
        }
    )
}
