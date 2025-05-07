// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Parsing functionality for Windows LCIDs.
//! For more information, see [`WindowsLocale`].
//!
//! # Usage example
//! ```
//! # use icu_locale_core::locale;
//! # use env_preferences::parse::windows::WindowsLocale;
//! # use env_preferences::LocaleError;
//! # fn main() -> Result<(), LocaleError> {
//! let windows_locale = WindowsLocale::try_from_str("en-US")?;
//! assert_eq!(windows_locale.try_convert_lossy()?, locale!("en-US"));
//! # Ok(())
//! # }
//! ```

use icu_locale_core::extensions::unicode::{key, Keywords, Unicode};
use icu_locale_core::extensions::Extensions;
use icu_locale_core::{LanguageIdentifier, Locale};

use super::aliases::{find_windows_language_alias_lossy, strip_windows_collation_suffix_lossy};
use crate::ParseError;

pub struct WindowsLocale<'src> {
    src: &'src str,
}

impl<'src> WindowsLocale<'src> {
    pub fn try_from_str(src: &'src str) -> Result<Self, ParseError> {
        Ok(Self { src })
    }

    /// ## Edge cases
    /// ```
    /// # use icu_locale_core::locale;
    /// # use env_preferences::parse::windows::WindowsLocale;
    /// # use env_preferences::LocaleError;
    /// # fn main() -> Result<(), LocaleError> {
    /// // Known invalid values are converted to a matching BCP-47 identifier
    /// assert_eq!(
    ///     WindowsLocale::try_from_str("zh-yue-HK")?.try_convert_lossy()?,
    ///     locale!("yue-HK")
    /// );
    ///
    /// // Known collation suffixes and converted to `-u-co-VALUE` extension syntax
    /// assert_eq!(
    ///     WindowsLocale::try_from_str("de-DE_phoneb")?.try_convert_lossy()?,
    ///     locale!("de-DE-u-co-phonebk")
    /// );
    /// assert_eq!(
    ///     WindowsLocale::try_from_str("zh-TW_pronun")?.try_convert_lossy()?,
    ///     locale!("zh-TW-u-co-zhuyin")
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_convert_lossy(&self) -> Result<Locale, ParseError> {
        let (lcid, collation_value) = strip_windows_collation_suffix_lossy(self.src);
        let keywords = match collation_value {
            // Add the -u-co-VALUE extension to the locale
            Some(collation_value) => Keywords::new_single(key!("co"), collation_value),
            // No collation value found, use default keywords
            None => Keywords::new(),
        };

        // Use a matching alias if found
        let language = match find_windows_language_alias_lossy(lcid) {
            Some(locale) => locale,
            None => LanguageIdentifier::try_from_str(lcid)?,
        };

        Ok(Locale {
            id: language,
            extensions: Extensions::from_unicode(Unicode {
                keywords,
                ..Unicode::new()
            }),
        })
    }
}
