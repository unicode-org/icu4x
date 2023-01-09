// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use crate::errors::ffi::ICU4XError;
    use alloc::boxed::Box;
    use core::str;
    use diplomat_runtime::{DiplomatResult, DiplomatWriteable};
    use icu_locid::extensions::unicode::Key;
    use icu_locid::subtags::{Language, Region, Script};
    use icu_locid::Locale;
    use writeable::Writeable;

    use crate::common::ffi::ICU4XOrdering;

    #[diplomat::opaque]
    /// An ICU4X Locale, capable of representing strings like `"en-US"`.
    #[diplomat::rust_link(icu::locid::Locale, Struct)]
    pub struct ICU4XLocale(pub Locale);

    impl ICU4XLocale {
        /// Construct an [`ICU4XLocale`] from an locale identifier.
        #[diplomat::rust_link(icu::locid::Locale::try_from_bytes, FnInStruct)]
        #[diplomat::rust_link(icu::locid::Locale::from_str, FnInStruct, hidden)]
        pub fn create_from_string(name: &str) -> DiplomatResult<Box<ICU4XLocale>, ICU4XError> {
            let name = name.as_bytes(); // #2520
            Locale::try_from_bytes(name)
                .map_err(ICU4XError::from)
                .map(|l| Box::new(ICU4XLocale(l)))
                .into()
        }

        /// Construct a default undefined [`ICU4XLocale`] "und".
        #[diplomat::rust_link(icu::locid::Locale::UND, AssociatedConstantInStruct)]
        pub fn create_und() -> Box<ICU4XLocale> {
            Box::new(ICU4XLocale(Locale::UND))
        }

        /// Clones the [`ICU4XLocale`].
        #[diplomat::rust_link(icu::locid::Locale, Struct)]
        #[allow(clippy::should_implement_trait)]
        pub fn clone(&self) -> Box<ICU4XLocale> {
            Box::new(ICU4XLocale(self.0.clone()))
        }

        /// Write a string representation of the `LanguageIdentifier` part of
        /// [`ICU4XLocale`] to `write`.
        #[diplomat::rust_link(icu::locid::Locale::id, StructField)]
        pub fn basename(
            &self,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            #[allow(unused_variables)]
            let result = self.0.id.write_to(write).map_err(Into::into).into();
            write.flush();
            result
        }

        /// Write a string representation of the unicode extension to `write`
        #[diplomat::rust_link(icu::locid::Locale::extensions, StructField)]
        pub fn get_unicode_extension(
            &self,
            bytes: &str,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            let bytes = bytes.as_bytes(); // #2520
            Key::try_from_bytes(bytes)
                .map_err(ICU4XError::from)
                .and_then(|key| {
                    self.0
                        .extensions
                        .unicode
                        .keywords
                        .get(&key)
                        .ok_or(ICU4XError::LocaleUndefinedSubtagError)
                        .and_then(|value| {
                            let result = value.write_to(write).map_err(ICU4XError::from);
                            write.flush();
                            result
                        })
                })
                .into()
        }

        /// Write a string representation of [`ICU4XLocale`] language to `write`
        #[diplomat::rust_link(icu::locid::Locale::id, StructField)]
        pub fn language(
            &self,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            #[allow(unused_variables)]
            let result = self
                .0
                .id
                .language
                .write_to(write)
                .map_err(Into::into)
                .into();
            write.flush();
            result
        }

        /// Set the language part of the [`ICU4XLocale`].
        #[diplomat::rust_link(icu::locid::Locale::try_from_bytes, FnInStruct)]
        pub fn set_language(&mut self, bytes: &str) -> DiplomatResult<(), ICU4XError> {
            let bytes = bytes.as_bytes(); // #2520
            if bytes.is_empty() {
                self.0.id.language = Language::UND;
                return Ok(()).into();
            }
            match Language::try_from_bytes(bytes) {
                Ok(language) => {
                    self.0.id.language = language;
                    Ok(())
                }
                Err(e) => Err(e.into()),
            }
            .into()
        }

        /// Write a string representation of [`ICU4XLocale`] region to `write`
        #[diplomat::rust_link(icu::locid::Locale::id, StructField)]
        pub fn region(
            &self,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            if let Some(region) = self.0.id.region {
                #[allow(unused_variables)]
                let result = region.write_to(write).map_err(Into::into).into();
                write.flush();
                result
            } else {
                Result::Err(ICU4XError::LocaleUndefinedSubtagError).into()
            }
        }

        /// Set the region part of the [`ICU4XLocale`].
        #[diplomat::rust_link(icu::locid::Locale::try_from_bytes, FnInStruct)]
        pub fn set_region(&mut self, bytes: &str) -> DiplomatResult<(), ICU4XError> {
            let bytes = bytes.as_bytes(); // #2520
            if bytes.is_empty() {
                self.0.id.region = None;
                return Ok(()).into();
            }
            match Region::try_from_bytes(bytes) {
                Ok(region) => {
                    self.0.id.region = Some(region);
                    Ok(())
                }
                Err(e) => Err(e.into()),
            }
            .into()
        }

        /// Write a string representation of [`ICU4XLocale`] script to `write`
        #[diplomat::rust_link(icu::locid::Locale::id, StructField)]
        pub fn script(
            &self,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            if let Some(script) = self.0.id.script {
                #[allow(unused_variables)]
                let result = script.write_to(write).map_err(Into::into).into();
                write.flush();
                result
            } else {
                Result::Err(ICU4XError::LocaleUndefinedSubtagError).into()
            }
        }

        /// Set the script part of the [`ICU4XLocale`]. Pass an empty string to remove the script.
        #[diplomat::rust_link(icu::locid::Locale::try_from_bytes, FnInStruct)]
        pub fn set_script(&mut self, bytes: &str) -> DiplomatResult<(), ICU4XError> {
            let bytes = bytes.as_bytes(); // #2520
            if bytes.is_empty() {
                self.0.id.script = None;
                return Ok(()).into();
            }
            match Script::try_from_bytes(bytes) {
                Ok(script) => {
                    self.0.id.script = Some(script);
                    Ok(())
                }
                Err(e) => Err(e.into()),
            }
            .into()
        }

        /// Best effort locale canonicalizer that doesn't need any data
        ///
        /// Use ICU4XLocaleCanonicalizer for better control and functionality
        #[diplomat::rust_link(icu::locid::Locale::canonicalize, FnInStruct)]
        pub fn canonicalize(
            bytes: &str,
            write: &mut DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            let bytes = bytes.as_bytes(); // #2520
            let s = try_icu4x!(Locale::canonicalize(bytes));
            let result = s.write_to(write).map_err(Into::into).into();
            write.flush();
            result
        }
        /// Write a string representation of [`ICU4XLocale`] to `write`
        #[diplomat::rust_link(icu::locid::Locale::write_to, FnInStruct)]
        pub fn to_string(
            &self,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            #[allow(unused_variables)]
            let result = self.0.write_to(write).map_err(Into::into).into();
            write.flush();
            result
        }

        #[diplomat::rust_link(icu::locid::Locale::normalizing_eq, FnInStruct)]
        pub fn normalizing_eq(&self, other: &str) -> bool {
            let other = other.as_bytes(); // #2520
            if let Ok(other) = str::from_utf8(other) {
                self.0.normalizing_eq(other)
            } else {
                // invalid UTF8 won't be allowed in locales anyway
                false
            }
        }

        #[diplomat::rust_link(icu::locid::Locale::strict_cmp, FnInStruct)]
        pub fn strict_cmp(&self, other: &str) -> ICU4XOrdering {
            let other = other.as_bytes(); // #2520
            self.0.strict_cmp(other).into()
        }

        //============ CONSTANTS ============

        /// Construct an [`ICU4XLocale`] for the `ar` locale.
        pub fn create_ar() -> Box<ICU4XLocale> {
            Box::new(ICU4XLocale(icu_locid::locale!("ar")))
        }

        /// Construct an [`ICU4XLocale`] for the `bn` locale.
        pub fn create_bn() -> Box<ICU4XLocale> {
            Box::new(ICU4XLocale(icu_locid::locale!("bn")))
        }

        /// Construct an [`ICU4XLocale`] for the `de` locale.
        pub fn create_de() -> Box<ICU4XLocale> {
            Box::new(ICU4XLocale(icu_locid::locale!("de")))
        }

        /// Construct an [`ICU4XLocale`] for the `en` locale.
        pub fn create_en() -> Box<ICU4XLocale> {
            Box::new(ICU4XLocale(icu_locid::locale!("en")))
        }

        /// Construct an [`ICU4XLocale`] for the `en-GB` locale.
        pub fn create_en_gb() -> Box<ICU4XLocale> {
            Box::new(ICU4XLocale(icu_locid::locale!("en-GB")))
        }

        /// Construct an [`ICU4XLocale`] for the `es` locale.
        pub fn create_es() -> Box<ICU4XLocale> {
            Box::new(ICU4XLocale(icu_locid::locale!("es")))
        }

        /// Construct an [`ICU4XLocale`] for the `es-419` locale.
        pub fn create_es_419() -> Box<ICU4XLocale> {
            Box::new(ICU4XLocale(icu_locid::locale!("es-419")))
        }

        /// Construct an [`ICU4XLocale`] for the `fr` locale.
        pub fn create_fr() -> Box<ICU4XLocale> {
            Box::new(ICU4XLocale(icu_locid::locale!("fr")))
        }

        /// Construct an [`ICU4XLocale`] for the `hi` locale.
        pub fn create_hi() -> Box<ICU4XLocale> {
            Box::new(ICU4XLocale(icu_locid::locale!("hi")))
        }

        /// Construct an [`ICU4XLocale`] for the `id` locale.
        pub fn create_id() -> Box<ICU4XLocale> {
            Box::new(ICU4XLocale(icu_locid::locale!("id")))
        }

        /// Construct an [`ICU4XLocale`] for the `ja` locale.
        pub fn create_ja() -> Box<ICU4XLocale> {
            Box::new(ICU4XLocale(icu_locid::locale!("ja")))
        }

        /// Construct an [`ICU4XLocale`] for the `ko` locale.
        pub fn create_ko() -> Box<ICU4XLocale> {
            Box::new(ICU4XLocale(icu_locid::locale!("ko")))
        }

        /// Construct an [`ICU4XLocale`] for the `pt` locale.
        pub fn create_pt() -> Box<ICU4XLocale> {
            Box::new(ICU4XLocale(icu_locid::locale!("pt")))
        }

        /// Construct an [`ICU4XLocale`] for the `ru` locale.
        pub fn create_ru() -> Box<ICU4XLocale> {
            Box::new(ICU4XLocale(icu_locid::locale!("ru")))
        }

        /// Construct an [`ICU4XLocale`] for the `th` locale.
        pub fn create_th() -> Box<ICU4XLocale> {
            Box::new(ICU4XLocale(icu_locid::locale!("th")))
        }

        /// Construct an [`ICU4XLocale`] for the `tr` locale.
        pub fn create_tr() -> Box<ICU4XLocale> {
            Box::new(ICU4XLocale(icu_locid::locale!("tr")))
        }

        /// Construct an [`ICU4XLocale`] for the `ur` locale.
        pub fn create_ur() -> Box<ICU4XLocale> {
            Box::new(ICU4XLocale(icu_locid::locale!("ur")))
        }

        /// Construct an [`ICU4XLocale`] for the `vi` locale.
        pub fn create_vi() -> Box<ICU4XLocale> {
            Box::new(ICU4XLocale(icu_locid::locale!("vi")))
        }

        /// Construct an [`ICU4XLocale`] for the `zh` locale.
        pub fn create_zh() -> Box<ICU4XLocale> {
            Box::new(ICU4XLocale(icu_locid::locale!("zh")))
        }

        /// Construct an [`ICU4XLocale`] for the `zh-Hant` locale.
        pub fn create_zh_hant() -> Box<ICU4XLocale> {
            Box::new(ICU4XLocale(icu_locid::locale!("zh-Hant")))
        }
    }
}

impl ffi::ICU4XLocale {
    pub fn to_datalocale(&self) -> icu_provider::prelude::DataLocale {
        (&self.0).into()
    }
}
