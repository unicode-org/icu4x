// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locale::extensions::unicode::{key, Keywords, Unicode};
use icu_locale::extensions::Extensions;
use icu_locale::{LanguageIdentifier, Locale, ParseError};

use super::aliases::{find_windows_alias_lossy, strip_windows_collation_suffix_lossy};
use crate::RetrievalError;

pub struct WindowsLocale<'src> {
    src: &'src str,
}

impl<'src> WindowsLocale<'src> {
    pub fn try_from_str(src: &'src str) -> Result<Self, RetrievalError> {
        Ok(Self { src })
    }

    pub fn try_convert_lossy(&self) -> Result<Locale, ParseError> {
        let (lcid, collation_value) = strip_windows_collation_suffix_lossy(self.src);
        let keywords = match collation_value {
            // Add the -u-co-VALUE extension to the locale
            Some(collation_value) => Keywords::new_single(key!("co"), collation_value),
            // No collation value found, use default keywords
            None => Keywords::new(),
        };

        // Use a matching alias if found
        let language = match find_windows_alias_lossy(lcid) {
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
