// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Parsing functionality for Windows LCIDs.
//! For more information, see [`WindowsLocale`].
//!
//! # Usage example
//! ```
//! use icu_locale_core::{Locale, locale};
//! use icu_host_info::locale::{WindowsLocale, windows::WindowsLocaleParseError};
//!
//! # fn main() -> Result<(), WindowsLocaleParseError> {
//! let windows_locale = WindowsLocale::try_from_str("zh-CN_radstr")?;
//!
//! assert_eq!(Locale::try_from(windows_locale), Ok(locale!("zh-CN-u-co-unihan")));
//! # Ok(())
//! # }
//! ```

use displaydoc::Display;
use icu_locale_core::extensions::unicode::{key, value, Keywords, Unicode, Value};
use icu_locale_core::extensions::Extensions;
use icu_locale_core::{langid, LanguageIdentifier, Locale, ParseError};

#[derive(Display, Debug, PartialEq)]
/// An error while parsing a Windows locale identifier
pub enum WindowsLocaleParseError {}

/// A parsed and validated Windows locale identifier.
pub struct WindowsLocale<'src> {
    src: &'src str,
}

impl<'src> WindowsLocale<'src> {
    pub fn try_from_str(src: &'src str) -> Result<Self, WindowsLocaleParseError> {
        Ok(Self { src })
    }
}

impl<'src> TryFrom<WindowsLocale<'src>> for Locale {
    type Error = ParseError;

    fn try_from(input: WindowsLocale<'src>) -> Result<Self, Self::Error> {
        let (lcid, collation_value) = strip_windows_collation_suffix_lossy(input.src);
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

fn strip_windows_collation_suffix_lossy(lcid: &str) -> (&str, Option<Value>) {
    // All known LCIDs containing an underscore are used for a collation suffix
    if let Some((prefix, suffix)) = lcid.split_once('_') {
        let collation_value = match suffix {
            "phoneb" => value!("phonebk"),
            "pronun" => value!("zhuyin"),
            "radstr" => value!("unihan"),
            "stroke" => value!("stroke"),
            "tradnl" => value!("trad"),
            // Strip the suffix on LCIDs with an underscore but no (known) matching CLDR data
            _ => return (prefix, None),
        };

        // Return the LCID with the stripped prefix, and the matching CLDR collation key
        (prefix, Some(collation_value))
    } else {
        // No underscore found, return the LCID as-is
        (lcid, None)
    }
}

/// Find a BCP-47 identifier from a list of known Windows aliases.
fn find_windows_language_alias_lossy(lcid: &str) -> Option<LanguageIdentifier> {
    match lcid {
        "zh-yue-HK" => Some(langid!("yue-HK")),
        // LCID with no (known) matching CLDR data: "math alphanumeric sorting"
        // This would be `x-IV_mathan`, but the collation suffix may already be stripped by
        // `strip_windows_collation_suffix_lossy`. For some reason, `LocaleEnumProcEx` also uses
        // `x-IV-mathan`, so that is included here too.
        // https://learn.microsoft.com/en-us/windows/win32/api/winnls/nc-winnls-locale_enumprocex
        "x-IV" | "x-IV_mathan" | "x-IV-mathan" => Some(langid!("und")),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expect_success(src: &str, expected: &str) {
        let windows_locale = WindowsLocale::try_from_str(src).expect(src);
        let locale = Locale::try_from(windows_locale).expect(src);

        assert_eq!(
            locale,
            Locale::try_from_str(expected).unwrap(),
            "Case: {src}"
        );
    }

    #[test]
    fn collation() {
        /// All MS-LCID collation entries with a known matching CLDR collation value
        const CASES: [(&str, &str); 12] = [
            ("de-DE_phoneb", "de-DE-u-co-phonebk"),
            ("es-ES_tradnl", "es-ES-u-co-trad"),
            ("ja-JP_radstr", "ja-JP-u-co-unihan"),
            ("zh-CN_phoneb", "zh-CN-u-co-phonebk"),
            ("zh-CN_stroke", "zh-CN-u-co-stroke"),
            ("zh-HK_radstr", "zh-HK-u-co-unihan"),
            ("zh-MO_radstr", "zh-MO-u-co-unihan"),
            ("zh-MO_stroke", "zh-MO-u-co-stroke"),
            ("zh-SG_phoneb", "zh-SG-u-co-phonebk"),
            ("zh-SG_stroke", "zh-SG-u-co-stroke"),
            ("zh-TW_pronun", "zh-TW-u-co-zhuyin"),
            ("zh-TW_radstr", "zh-TW-u-co-unihan"),
        ];

        for (src, expected) in CASES {
            expect_success(src, expected);
        }
    }

    #[test]
    fn collation_strip_known_invalid() {
        // All MS-LCID collation entries with NO known matching CLDR collation value
        expect_success("hu-HU_tchncl", "hu-HU");
        expect_success("ka-GE_modern", "ka-GE");
    }

    #[test]
    fn collation_strip_unknown() {
        expect_success("en-US_unknown", "en-US");
        expect_success("en-US_unknown_multiple_underscores", "en-US");
        expect_success("en-US_unknown-with-hyphens", "en-US");
    }

    #[test]
    fn alias() {
        expect_success("zh-yue-HK", "yue-HK");
        expect_success("x-IV-mathan", "und");
    }
}
