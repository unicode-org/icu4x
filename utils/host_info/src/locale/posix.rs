// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Parsing functionality for POSIX locale identifiers.
//! For more information, see [`PosixLocale`].
//!
//! # Usage example
//! ```
//! use icu_locale_core::{Locale, locale};
//! use icu_host_info::locale::{posix::PosixParseError, PosixLocale};
//!
//! # fn main() -> Result<(), PosixParseError> {
//! let posix_locale = PosixLocale::try_from_str("en_US.utf8@euro").unwrap();
//!
//! assert_eq!(Locale::try_from(posix_locale), Ok(locale!("en-US-u-cu-eur")));
//! # Ok(())
//! # }
//! ```

use displaydoc::Display;
use icu_locale_core::extensions::unicode::{key, value};
use icu_locale_core::extensions::Extensions;
use icu_locale_core::subtags::{language, script, variant, Language, Region, Variants};
use icu_locale_core::{locale, LanguageIdentifier, Locale, ParseError};

#[derive(Display, Debug, PartialEq)]
/// An error while parsing a POSIX locale identifier
pub enum PosixParseError {
    #[displaydoc("Empty locale")]
    EmptyLocale,
    #[displaydoc("Empty section beginning at offset {offset}")]
    EmptySection { offset: usize },
    #[displaydoc("Invalid character at offset {offset}")]
    InvalidCharacter { offset: usize },
    #[displaydoc("Invalid locale")]
    InvalidLocale,
    #[displaydoc("Delimiter repeated at offsets {first_offset} and {second_offset}")]
    RepeatedDelimiter {
        first_offset: usize,
        second_offset: usize,
    },
    #[displaydoc("Delimiters found out-of-order at offsets {first_offset} and {second_offset}")]
    UnorderedDelimiter {
        first_offset: usize,
        second_offset: usize,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Delimiter {
    Territory,
    Codeset,
    Modifier,
}

impl Delimiter {
    /// Find any optional sections, returning an error if the delimiters are invalid
    pub fn try_find_sections(src: &str) -> Result<Vec<(usize, Self)>, PosixParseError> {
        // Find the offset and delimiter of each optional section
        let optional_sections = src
            .chars()
            .enumerate()
            .flat_map(|(index, character)| match character {
                '_' => Some((index, Self::Territory)),
                '.' => Some((index, Self::Codeset)),
                '@' => Some((index, Self::Modifier)),
                _ => None,
            })
            .collect::<Vec<_>>();

        // Find any errors in the arrangement of delimiters
        for (index, (first_offset, first_delimiter)) in optional_sections.iter().enumerate() {
            // Find any repeated delimiters
            if let Some((second_offset, _second_delimiter)) = optional_sections
                .iter()
                // Check all delimiters past this index
                .skip(index + 1)
                .find(|(_second_offset, second_delimiter)| first_delimiter == second_delimiter)
            {
                return Err(PosixParseError::RepeatedDelimiter {
                    first_offset: *first_offset,
                    second_offset: *second_offset,
                });
            }

            // Find any delimiters that have been invalidated by a delimiter that should appear after it
            // For example "en.utf8_US" is invalid because codeset appears before territory
            if let Some((second_offset, second_delimiter)) = optional_sections.get(index + 1) {
                if first_delimiter > second_delimiter {
                    return Err(PosixParseError::UnorderedDelimiter {
                        first_offset: *first_offset,
                        second_offset: *second_offset,
                    });
                }
            }
        }

        Ok(optional_sections)
    }
}

#[derive(Debug)]
/// A parsed and validated POSIX locale identifier.
///
/// Locales are expected to be in the format `language[_territory][.codeset][@modifier]`;
/// only the language section is mandatory, all other sections are optional.
/// For example:
/// - All sections: `en_US.utf8@euro`
/// - Only required sections: `en`
///
/// See section 8.2 of the POSIX spec for more details:
/// <https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/V1_chap08.html#tag_08_02>
pub struct PosixLocale<'src> {
    language: &'src str,
    territory: Option<&'src str>,
    codeset: Option<&'src str>,
    // TODO: is it possible to have multiple modifiers?
    modifier: Option<&'src str>,
}

impl<'src> PosixLocale<'src> {
    /// Attempt to parse a POSIX locale.
    pub fn try_from_str(src: &'src str) -> Result<Self, PosixParseError> {
        // These cases are implementation-defined and can be ignored:
        // - Empty locales
        if src.is_empty() {
            return Err(PosixParseError::EmptyLocale);
        }
        // - Any locale containing '/'
        if let Some(offset) = src.find('/') {
            return Err(PosixParseError::InvalidCharacter { offset });
        }
        // - Locales consisting of "." or ".."
        if src == "." || src == ".." {
            return Err(PosixParseError::InvalidLocale);
        }

        // Find any optional sections, and return any delimiter-related errors
        let optional_sections = Delimiter::try_find_sections(src)?;

        // The language field continues until the start of the first optional section, if one exists
        let language = match optional_sections.first() {
            Some((offset, _delimiter)) => &src[..*offset],
            None => src,
        };

        // Make sure the language itself is non-empty
        if language.is_empty() {
            return Err(PosixParseError::EmptySection { offset: 0 });
        }

        let mut locale = Self {
            language,
            territory: None,
            codeset: None,
            modifier: None,
        };

        for (index, (start_offset, delimiter)) in optional_sections.iter().enumerate() {
            // Find the offset of the next section, or end of the string if none exist
            let end_offset = optional_sections
                .get(index + 1)
                .map(|(next_offset, _next_delimiter)| *next_offset)
                .unwrap_or(src.len());

            // Make sure this section is non-empty (more characters than just the delimiter)
            if start_offset + 1 >= end_offset {
                return Err(PosixParseError::EmptySection {
                    offset: *start_offset,
                });
            }

            // Write the section to the appropriate field
            let section_value = Some(&src[start_offset + 1..end_offset]);
            match delimiter {
                Delimiter::Territory => locale.territory = section_value,
                Delimiter::Codeset => locale.codeset = section_value,
                Delimiter::Modifier => locale.modifier = section_value,
            }
        }

        Ok(locale)
    }
}

impl<'s> TryFrom<PosixLocale<'s>> for Locale {
    type Error = ParseError;

    fn try_from(input: PosixLocale<'s>) -> Result<Self, Self::Error> {
        // The default "C"/"POSIX" locale should map to "en-US-posix",
        // which is the default behaviour in ICU4C:
        // https://github.com/unicode-org/icu/blob/795d7ac82c4b29cf721d0ad62c0b178347d453bf/icu4c/source/common/putil.cpp#L1738
        if input.language == "C" || input.language == "POSIX" {
            return Ok(locale!("en-US-posix"));
        }

        let mut extensions = Extensions::new();
        let mut script = None;
        let mut variant = None;

        // Parse the language/region
        let mut language = Language::try_from_str(input.language)?;
        let region = input.territory.map(Region::try_from_str).transpose()?;

        if let Some(modifier) = input.modifier {
            match modifier.to_ascii_lowercase().as_str() {
                "euro" => {
                    extensions.unicode.keywords.set(key!("cu"), value!("eur"));
                }
                // Known script modifiers
                "cyrillic" => script = Some(script!("Cyrl")),
                "devanagari" => script = Some(script!("Deva")),
                "latin" => script = Some(script!("Latn")),
                // Saaho seems to be the only "legacy variant" that appears as a modifier:
                // https://www.unicode.org/reports/tr35/#table-legacy-variant-mappings
                "saaho" => language = language!("ssy"),
                "valencia" => variant = Some(variant!("valencia")),
                // Some modifiers are known but can't be expressed as a BCP-47 identifier
                // e.g. "@abegede", "@iqtelif"
                _ => (),
            }
        }

        Ok(Locale {
            id: LanguageIdentifier {
                language,
                region,
                script,
                variants: variant.map_or_else(Variants::new, Variants::from_variant),
            },
            extensions,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expect_success(src: &str, expected: &str) {
        let posix_locale = PosixLocale::try_from_str(src).expect(src);
        let converted_locale: Locale = posix_locale.try_into().expect(src);

        let expected_locale = Locale::try_from_str(expected).expect(src);
        assert_eq!(converted_locale, expected_locale, "POSIX locale: `{src}`");
    }

    #[test]
    fn default_locale() {
        expect_success("C", "en-US-posix");
        expect_success("POSIX", "en-US-posix");
    }

    #[test]
    fn region() {
        expect_success("en_US", "en-US");
        expect_success("ne_NP", "ne-NP");
        expect_success("zh_TW", "zh-TW");
    }

    #[test]
    fn codeset_ignored() {
        expect_success("lv_LV.iso885913", "lv-LV");
        expect_success("hy_AM.armscii8", "hy-AM");
    }

    #[test]
    fn modifier() {
        // Currency
        expect_success("it_IT@euro", "it-IT-u-cu-eur");

        // Script
        expect_success("uz_UZ@cyrillic", "uz-Cyrl-UZ");
        expect_success("sd_IN@devanagari", "sd-Deva-IN");
        expect_success("sr_RS@latin", "sr-Latn-RS");

        // Language
        expect_success("aa_ER@saaho", "ssy-ER");

        // Variant
        expect_success("ca_ES@valencia", "ca-ES-valencia");
    }

    mod error {
        mod parse {
            use crate::locale::{posix::PosixParseError, PosixLocale};

            fn expect_error(src: &str, posix_error: PosixParseError) {
                let result = PosixLocale::try_from_str(src);

                match result {
                    Ok(invalid_locale) => {
                        panic!("Expected the error `{posix_error:?}`, got the locale `{invalid_locale:?}` from input of `{src}`")
                    }
                    Err(error) => {
                        assert_eq!(error, posix_error, "Comparing expected output of `{src}`")
                    }
                }
            }

            #[test]
            fn empty_locale() {
                expect_error("", PosixParseError::EmptyLocale);
            }

            #[test]
            fn empty_section() {
                // Single, empty optional section
                expect_error("en_", PosixParseError::EmptySection { offset: 2 });
                expect_error("en.", PosixParseError::EmptySection { offset: 2 });
                expect_error("en@", PosixParseError::EmptySection { offset: 2 });

                // Multiple optional sections, one empty
                expect_error("en_.utf8@euro", PosixParseError::EmptySection { offset: 2 });
                expect_error("en_US.@euro", PosixParseError::EmptySection { offset: 5 });
                expect_error("en_US.utf8@", PosixParseError::EmptySection { offset: 10 });

                // Single delimiter (excluding "." as that should return `PosixParseError::InvalidLocale` instead)
                expect_error("_", PosixParseError::EmptySection { offset: 0 });
                expect_error("@", PosixParseError::EmptySection { offset: 0 });

                // All delimiters
                expect_error("_.@", PosixParseError::EmptySection { offset: 0 });
            }

            #[test]
            fn invalid_character() {
                const SAMPLE_LOCALES: [&str; 2] = [
                    "en",              // No optional fields
                    "en_US.utf8@euro", // All optional fields
                ];

                for locale in SAMPLE_LOCALES {
                    // Insert an invalid character ('/') at every position along the sample locale
                    for offset in 0..=locale.len() {
                        let (left, right) = locale.split_at(offset);
                        let invalid_locale = format!("{left}/{right}");
                        expect_error(
                            &invalid_locale,
                            PosixParseError::InvalidCharacter { offset },
                        );
                    }
                }

                // Test a single '/' character
                expect_error("/", PosixParseError::InvalidCharacter { offset: 0 });
            }

            #[test]
            fn invalid_locale() {
                expect_error(".", PosixParseError::InvalidLocale);
                expect_error("..", PosixParseError::InvalidLocale);
            }

            #[test]
            fn repeated_delimiter() {
                // Repeated delimiter at the end of locale
                expect_error(
                    "en_US.utf8@euro_US",
                    PosixParseError::RepeatedDelimiter {
                        first_offset: 2,
                        second_offset: 15,
                    },
                );
                expect_error(
                    "en_US.utf8@euro.utf8",
                    PosixParseError::RepeatedDelimiter {
                        first_offset: 5,
                        second_offset: 15,
                    },
                );
                expect_error(
                    "en_US.utf8@euro@euro",
                    PosixParseError::RepeatedDelimiter {
                        first_offset: 10,
                        second_offset: 15,
                    },
                );

                // Multiple repeated delimiters
                expect_error(
                    "en.utf8.utf8.utf8",
                    PosixParseError::RepeatedDelimiter {
                        first_offset: 2,
                        second_offset: 7,
                    },
                );

                // Consecutive repeated delimiters
                expect_error(
                    "en__US.utf8@euro",
                    PosixParseError::RepeatedDelimiter {
                        first_offset: 2,
                        second_offset: 3,
                    },
                );
                expect_error(
                    "en_US..utf8@euro",
                    PosixParseError::RepeatedDelimiter {
                        first_offset: 5,
                        second_offset: 6,
                    },
                );
                expect_error(
                    "en_US.utf8@@euro",
                    PosixParseError::RepeatedDelimiter {
                        first_offset: 10,
                        second_offset: 11,
                    },
                );
            }

            #[test]
            fn unordered_delimiter() {
                expect_error(
                    "en_US@euro.utf8",
                    PosixParseError::UnorderedDelimiter {
                        first_offset: 5,
                        second_offset: 10,
                    },
                );
                expect_error(
                    "en.utf8_US@euro",
                    PosixParseError::UnorderedDelimiter {
                        first_offset: 2,
                        second_offset: 7,
                    },
                );
                expect_error(
                    "en.utf8@euro_US",
                    PosixParseError::UnorderedDelimiter {
                        first_offset: 7,
                        second_offset: 12,
                    },
                );
                expect_error(
                    "en@euro_US.utf8",
                    PosixParseError::UnorderedDelimiter {
                        first_offset: 2,
                        second_offset: 7,
                    },
                );
                expect_error(
                    "en@euro.utf8_US",
                    PosixParseError::UnorderedDelimiter {
                        first_offset: 2,
                        second_offset: 7,
                    },
                );
            }

            #[test]
            fn offset() {
                // Empty section
                let src = "en_.utf8@euro";
                match PosixLocale::try_from_str(src) {
                    Err(PosixParseError::EmptySection { offset }) => {
                        assert_eq!(&src[offset..offset + 1], "_");
                    }
                    _ => unreachable!(),
                }

                // Invalid character
                let src = "en_U/S";
                match PosixLocale::try_from_str(src) {
                    Err(PosixParseError::InvalidCharacter { offset }) => {
                        assert_eq!(&src[offset..offset + 1], "/");
                    }
                    _ => unreachable!(),
                }

                // Repeated delimiter
                let src = "en_US.utf8@euro_US";
                match PosixLocale::try_from_str(src) {
                    Err(PosixParseError::RepeatedDelimiter {
                        first_offset,
                        second_offset,
                    }) => {
                        assert_eq!(&src[first_offset..first_offset + 1], "_");
                        assert_eq!(&src[second_offset..second_offset + 1], "_");
                    }
                    _ => unreachable!(),
                }

                // Unordered delimiter
                let src = "en_US@euro.utf8";
                match PosixLocale::try_from_str(src) {
                    Err(PosixParseError::UnorderedDelimiter {
                        first_offset,
                        second_offset,
                    }) => {
                        assert_eq!(&src[first_offset..first_offset + 1], "@");
                        assert_eq!(&src[second_offset..second_offset + 1], ".");
                    }
                    _ => unreachable!(),
                }
            }
        }

        mod conversion {
            use crate::locale::PosixLocale;
            use icu_locale_core::Locale;

            fn expect_error(src: &str, icu_error: icu_locale_core::ParseError) {
                let result: Result<Locale, _> =
                    PosixLocale::try_from_str(src).expect(src).try_into();
                match result {
                    Ok(invalid_locale) => {
                        panic!("Expected the error `{icu_error:?}`, got the locale `{invalid_locale:?}` from input of `{src}`")
                    }
                    Err(error) => {
                        assert_eq!(error, icu_error, "Comparing expected output of `{src}`")
                    }
                }
            }

            #[test]
            fn invalid_language() {
                expect_error("invalid", icu_locale_core::ParseError::InvalidLanguage);
            }

            #[test]
            fn invalid_region() {
                expect_error("en_invalid", icu_locale_core::ParseError::InvalidSubtag);
            }
        }
    }
}
