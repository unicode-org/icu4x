#[macro_export]
macro_rules! preferences {
    ($name:ident,
     $default_name:ident,
     $resolved_name:ident,
     {$($key:ident => $pref:ty, $resolved:ty, $ue:expr),*},
     $options_name:ident,
     {$($k:ident => $v:ty),*}
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

        impl Preferences for $name {
            fn language(&self) -> &icu_locid::subtags::Language {
                self.lid
                    .as_ref()
                    .map(|lid| &lid.language)
                    .unwrap_or(&icu_locid::subtags::Language::UND)
            }

            fn script(&self) -> Option<&icu_locid::subtags::Script> {
                self.lid.as_ref().and_then(|lid| lid.script.as_ref())
            }

            fn region(&self) -> Option<&icu_locid::subtags::Region> {
                self.lid.as_ref().and_then(|lid| lid.region.as_ref())
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
            pub fn merge_locale(&mut self, locale: &Locale) -> Result<(), ()> {
                if let Some(lid) = &mut self.lid {
                    if lid.language.is_empty() && !locale.id.language.is_empty() {
                        lid.language = locale.id.language;
                    };
                    if lid.script.is_none() && locale.id.script.is_some() {
                        lid.script = locale.id.script;
                    }
                    if lid.region.is_none() && locale.id.region.is_some() {
                        lid.region = locale.id.region;
                    }
                } else {
                    self.lid = Some(locale.id.clone());
                }

                $(
                    //XXX: Use same loop as in From
                    let ue = $ue.unwrap();
                    if self.$key.is_none() {
                        if let Some(value) = locale
                            .extensions
                            .unicode
                            .keywords
                            .get(&ue)
                        {
                            self.$key = Some(TryInto::try_into(value)?);
                        }
                    }
                )*
                Ok(())
            }
        }

        impl $resolved_name {
            pub fn resolve(&mut self, prefs: &$name) {
                let mut language = prefs.language();
                if language.is_empty() {
                    language = &self.lid.language;
                }
                $(
                    if let Some(v) = prefs.$key {
                        self.$key = v;
                    }
                )*
            }
        }

        #[derive(Default, Debug)]
        #[non_exhaustive]
        pub struct $options_name {
            $(
                pub $k: $v,
            )*
        }
    )
}
