#[macro_export]
macro_rules! preferences {
    ($name:ident,
     $default_name:ident,
     $resolved_name:ident,
     {$($key:ident => $pref:ty, $resolved:ty, $ue:expr),*}
     ) => (
        #[derive(Default)]
        #[non_exhaustive]
        pub struct $name {
            pub lid: Option<icu_locid::LanguageIdentifier>,
            $(
                pub $key: $pref,
            )*
        }

        #[non_exhaustive]
        pub struct $default_name {
            $(
                pub $key: $resolved,
            )*
        }

        pub struct $resolved_name {
            pub lid: icu_locid::LanguageIdentifier,

            $(
                pub $key: $resolved,
            )*
        }

        impl From<(LanguageIdentifier, &$default_name)> for $resolved_name {
            fn from(input: (LanguageIdentifier, &$default_name)) -> Self {
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
            #[allow(clippy::result_unit_err)]
            pub fn merge_locale(&mut self, locale: &Locale) {
                if let Some(lid) = &mut self.lid {
                    lid.merge(&locale.id, false);
                } else {
                    self.lid = Some(locale.id.clone());
                }

                for (k, v) in locale.extensions.unicode.keywords.iter() {
                    $(
                      if self.$key.is_none() {
                          if let Some(ue) = &$ue {
                              if k == ue {
                                  if let Ok(r) = TryInto::try_into(v) {
                                      self.$key = Some(r);
                                  }
                              }
                          }
                      }
                    )*
                }
            }
        }

        impl $resolved_name {
            pub fn resolve(&mut self, prefs: &$name) {
                $(
                    if let Some(v) = prefs.$key {
                        self.$key = v;
                    }
                )*
            }
        }
    )
}
