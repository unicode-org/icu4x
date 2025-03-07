// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locale::{Locale, ParseError};

use super::aliases::find_windows_alias;
use crate::RetrievalError;

pub struct WindowsLocale<'src> {
    src: &'src str,
}

impl<'src> WindowsLocale<'src> {
    pub fn try_from_str(src: &'src str) -> Result<Self, RetrievalError> {
        Ok(Self { src })
    }

    pub fn try_convert_lossy(&self) -> Result<Locale, ParseError> {
        match find_windows_alias(self.src) {
            Some(locale) => Ok(locale),
            None => Locale::try_from_str(self.src),
        }
    }
}
