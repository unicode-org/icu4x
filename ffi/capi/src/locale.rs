// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;
    use core::str::FromStr;
    use diplomat_runtime::DiplomatResult;
    use icu_locid::extensions::unicode::Key;
    use icu_locid::Locale;

    use writeable::Writeable;

    #[diplomat::opaque]
    /// An ICU4X Locale, capable of representing strings like `"en-US"`.
    /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html) for more information.
    pub struct ICU4XLocale(pub Locale);

    pub enum ICU4XLocaleError {
        Undefined,
        Error,
    }

    impl ICU4XLocale {
        /// Construct an [`ICU4XLocale`] from an locale identifier.
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.from_bytes) for more information.
        pub fn create(name: &str) -> Option<Box<ICU4XLocale>> {
            Locale::from_str(name)
                .ok()
                .map(|l| Box::new(ICU4XLocale(l)))
        }

        /// Clones the [`ICU4XLocale`].
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html) for more information.
        #[allow(clippy::should_implement_trait)]
        pub fn clone(&self) -> Box<ICU4XLocale> {
            Box::new(ICU4XLocale(self.0.clone()))
        }

        /// Write a string representation of the `LanguageIdentifier` part of
        /// [`ICU4XLocale`] to `write`.
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id) for more information.
        pub fn basename(
            &self,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XLocaleError> {
            #[allow(unused_variables)]
            let result = self
                .0
                .id
                .write_to(write)
                .map_err(|_| ICU4XLocaleError::Error)
                .into();
            write.flush();
            result
        }

        /// Write a string representation of the unicode extension to `write`
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.extensions) for more information.
        pub fn get_unicode_extension(
            &self,
            bytes: &str,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XLocaleError> {
            if let Ok(key) = Key::from_bytes(bytes.as_bytes()) {
                if let Some(value) = self.0.get_unicode_extension(&key) {
                    #[allow(unused_variables)]
                    let result = value
                        .write_to(write)
                        .map_err(|_| ICU4XLocaleError::Error)
                        .into();
                    write.flush();
                    result
                } else {
                    Result::Err(ICU4XLocaleError::Undefined).into()
                }
            } else {
                Result::Err(ICU4XLocaleError::Error).into()
            }
        }

        /// Write a string representation of [`ICU4XLocale`] language to `write`
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id) for more information.
        pub fn language(
            &self,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XLocaleError> {
            #[allow(unused_variables)]
            let result = self
                .0
                .id
                .language
                .write_to(write)
                .map_err(|_| ICU4XLocaleError::Error)
                .into();
            write.flush();
            result
        }

        /// Write a string representation of [`ICU4XLocale`] region to `write`
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id) for more information.
        pub fn region(
            &self,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XLocaleError> {
            if let Some(region) = self.0.id.region {
                #[allow(unused_variables)]
                let result = region
                    .write_to(write)
                    .map_err(|_| ICU4XLocaleError::Error)
                    .into();
                write.flush();
                result
            } else {
                Result::Err(ICU4XLocaleError::Undefined).into()
            }
        }

        /// Write a string representation of [`ICU4XLocale`] script to `write`
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id) for more information.
        pub fn script(
            &self,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XLocaleError> {
            if let Some(script) = self.0.id.script {
                #[allow(unused_variables)]
                let result = script
                    .write_to(write)
                    .map_err(|_| ICU4XLocaleError::Error)
                    .into();
                write.flush();
                result
            } else {
                Result::Err(ICU4XLocaleError::Undefined).into()
            }
        }

        /// Write a string representation of [`ICU4XLocale`] to `write`
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html) for more information.
        pub fn tostring(
            &self,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XLocaleError> {
            #[allow(unused_variables)]
            let result = self
                .0
                .write_to(write)
                .map_err(|_| ICU4XLocaleError::Error)
                .into();
            write.flush();
            result
        }
    }
}
