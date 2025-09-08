// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locale_core::Locale;

use crate::ParseError;

pub struct AppleLocale<'src> {
    src: &'src str,
}

impl<'src> AppleLocale<'src> {
    pub fn try_from_str(src: &'src str) -> Result<Self, ParseError> {
        Ok(Self { src })
    }

    pub fn try_convert_lossy(&self) -> Result<Locale, ParseError> {
        let locale = Locale::try_from_str(self.src)?;

        Ok(locale)
    }
}
