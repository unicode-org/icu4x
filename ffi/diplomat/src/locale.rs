// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use crate::errors::ffi::ICU4XError;
    use alloc::boxed::Box;
    use core::str::FromStr;
    use diplomat_runtime::DiplomatResult;
    use icu_locid::extensions::unicode::Key;
    use icu_locid::subtags::{Language, Region, Script};
    use icu_locid::Locale;

    use writeable::Writeable;

    #[diplomat::opaque]
    /// An ICU4X Locale, capable of representing strings like `"en-US"`.
    #[diplomat::rust_link(icu::locid::Locale, Struct)]
    pub struct ICU4XLocale(pub Locale);

    impl ICU4XLocale {
        /// Construct an [`ICU4XLocale`] from an locale identifier.
        #[diplomat::rust_link(icu::locid::Locale::from_bytes, FnInStruct)]
        #[diplomat::rust_link(icu::locid::Locale::from_str, FnInStruct, hidden)]
        pub fn create(name: &str) -> DiplomatResult<Box<ICU4XLocale>, ICU4XError> {
            Locale::from_str(name)
                .map_err(ICU4XError::from)
                .map(|l| Box::new(ICU4XLocale(l)))
                .into()
        }

        /// Construct an [`ICU4XLocale`] for the English language.
        pub fn create_en() -> Box<ICU4XLocale> {
            Box::new(ICU4XLocale(icu_locid::locale!("en")))
        }

        /// Construct an [`ICU4XLocale`] for the Bangla language.
        pub fn create_bn() -> Box<ICU4XLocale> {
            Box::new(ICU4XLocale(icu_locid::locale!("bn")))
        }

        /// Construct a default undefined [`ICU4XLocale`] "und".
        pub fn und() -> Box<ICU4XLocale> {
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
            Key::from_bytes(bytes.as_bytes())
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
        #[diplomat::rust_link(icu::locid::Locale::from_bytes, FnInStruct)]
        pub fn set_language(&mut self, bytes: &str) -> DiplomatResult<(), ICU4XError> {
            if bytes.is_empty() {
                self.0.id.language = Language::UND;
                return Ok(()).into();
            }
            match Language::from_bytes(bytes.as_bytes()) {
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
        #[diplomat::rust_link(icu::locid::Locale::from_bytes, FnInStruct)]
        pub fn set_region(&mut self, bytes: &str) -> DiplomatResult<(), ICU4XError> {
            if bytes.is_empty() {
                self.0.id.region = None;
                return Ok(()).into();
            }
            match Region::from_bytes(bytes.as_bytes()) {
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
        #[diplomat::rust_link(icu::locid::Locale::from_bytes, FnInStruct)]
        pub fn set_script(&mut self, bytes: &str) -> DiplomatResult<(), ICU4XError> {
            if bytes.is_empty() {
                self.0.id.script = None;
                return Ok(()).into();
            }
            match Script::from_bytes(bytes.as_bytes()) {
                Ok(script) => {
                    self.0.id.script = Some(script);
                    Ok(())
                }
                Err(e) => Err(e.into()),
            }
            .into()
        }

        /// Write a string representation of [`ICU4XLocale`] to `write`
        #[diplomat::rust_link(icu::locid::Locale::write_to, FnInStruct)]
        #[diplomat::rust_link(icu::locid::Locale::write_to_parts, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::locid::Locale::write_to_string, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::locid::Locale::write_len, FnInStruct, hidden)]
        pub fn to_string(
            &self,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            #[allow(unused_variables)]
            let result = self.0.write_to(write).map_err(Into::into).into();
            write.flush();
            result
        }
    }
}

impl ffi::ICU4XLocale {
    pub fn to_datalocale(&self) -> icu_provider::prelude::DataLocale {
        (&self.0).into()
    }
}
