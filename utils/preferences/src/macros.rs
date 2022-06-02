#[macro_export]
macro_rules! preferences {
    ($trait:ident, $name:ident, $resolved_name:ident, {$($key:ident => $pref:ty, $resolved:ty, $ue:expr),*}) => (
        pub trait $trait: Preferences {
            $(
                fn $key(&self) -> $pref {
                    None
                }
            )*
        }

        #[derive(Default)]
        #[non_exhaustive]
        pub struct $name {
            pub lid: Option<icu_locid::LanguageIdentifier>,
            $(
                pub $key: $pref,
            )*
        }

        pub struct $resolved_name {
            pub lid: icu_locid::LanguageIdentifier,

            $(
                $key: $resolved,
            )*
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

        impl $trait for $name {
            $(
                fn $key(&self) -> $pref {
                    self.$key
                }
            )*
        }

        impl TryFrom<Locale> for $name {
            type Error = ();

            fn try_from(loc: Locale) -> Result<Self, Self::Error> {
                let mut lid = Some(loc.id);

                Ok(Self {
                    lid,
                    $(
                        $key: {
                            if let Some(ue) = $ue {
                                if let Some(value) = loc
                                    .extensions
                                    .unicode
                                    .keywords
                                    .get(&ue) {
                                        Some(TryInto::try_into(value)?)
                                    } else {
                                        None
                                    }
                            } else {
                                None
                            }
                        },
                    )*
                })
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
            fn resolve(&mut self, prefs: &$name) {
                let mut language = prefs.language();
                if language.is_empty() {
                    language = &self.lid.language;
                }
                $(
                    if let Some(v) = prefs.$key() {
                        self.$key = v;
                    }
                )*
            }
        }
    )
}
