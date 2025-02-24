// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locale::extensions::unicode::{key, value};
use icu_locale::extensions::Extensions;
use icu_locale::subtags::{language, script, variant, Language, Region, Variants};
use icu_locale::{LanguageIdentifier, Locale};

use super::posix_aliases::get_bcp47_subtags_from_posix_alias;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    EmptyLocale,
    EmptySection {
        offset: usize,
    },
    InvalidCharacter {
        offset: usize,
    },
    InvalidLocale,
    RepeatedDelimiter {
        first_offset: usize,
        second_offset: usize,
    },
    UnorderedDelimiter {
        first_offset: usize,
        second_offset: usize,
    },
}

#[derive(Debug, PartialEq)]
pub enum ConversionError {
    InvalidLanguage {
        start_offset: usize,
        end_offset: usize,
    },
    InvalidRegion {
        start_offset: usize,
        end_offset: usize,
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
    pub fn try_find_sections(src: &str) -> Result<Vec<(usize, Self)>, ParseError> {
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
                return Err(ParseError::RepeatedDelimiter {
                    first_offset: *first_offset,
                    second_offset: *second_offset,
                });
            }

            // Find any delimiters that have been invalidated by a delimiter that should appear after it
            // For example "en.utf8_US" is invalid because codeset appears before territory
            if let Some((second_offset, second_delimiter)) = optional_sections.get(index + 1) {
                if first_delimiter > second_delimiter {
                    return Err(ParseError::UnorderedDelimiter {
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
pub struct PosixLocale<'src> {
    language: &'src str,
    territory: Option<&'src str>,
    codeset: Option<&'src str>,
    // TODO: is it possible to have multiple modifiers?
    modifier: Option<&'src str>,
}

impl<'src> PosixLocale<'src> {
    /// Attempt to parse a POSIX locale.
    ///
    /// Locales are expected to be in the format `language[_territory][.codeset][@modifier]`;
    /// only the language section is mandatory, all other sections are optional.
    /// Example with all sections: `en_US.utf8@euro`
    ///
    /// See section 8.2 of the POSIX spec for more details:
    /// <https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/V1_chap08.html#tag_08_02>
    pub fn try_from_str(src: &'src str) -> Result<Self, ParseError> {
        // These cases are implementation-defined and can be ignored:
        // - Empty locales
        if src.is_empty() {
            return Err(ParseError::EmptyLocale);
        }
        // - Any locale containing '/'
        if let Some(offset) = src.find('/') {
            return Err(ParseError::InvalidCharacter { offset });
        }
        // - Locales consisting of "." or ".."
        if src == "." || src == ".." {
            return Err(ParseError::InvalidLocale);
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
            return Err(ParseError::EmptySection { offset: 0 });
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
                return Err(ParseError::EmptySection {
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

    /// Attempt to convert a POSIX locale into a valid BCP-47 identifier.
    ///
    /// This is a best-effort conversion process, and there are valid
    /// POSIX locales that will return an error or silently ignore data.
    /// In particular, the codeset section is always ignored, and only some common modifiers are handled
    /// (unknown modifiers will be silently ignored).
    ///
    /// If the "logging" feature is enabled, this ignored data will be logged as a warning.
    pub fn try_convert_lossy(&self) -> Result<Locale, ConversionError> {
        // Check if the language matches a known alias
        let mut language = get_bcp47_subtags_from_posix_alias(self.language)
            .map(|(language, _region)| language)
            .or(Language::try_from_str(self.language).ok())
            .ok_or(ConversionError::InvalidLanguage {
                start_offset: 0,
                end_offset: self.language.len(),
            })?;

        // Use the associated region if the language matches a known alias
        let region = get_bcp47_subtags_from_posix_alias(self.language)
            .map(|(_language, region)| Ok(region))
            .unwrap_or(
                self.territory
                    .map(|territory| {
                        Region::try_from_str(territory).map_err(|_err| {
                            ConversionError::InvalidRegion {
                                // Add 1 to skip the delimiter
                                start_offset: self.language.len() + 1,
                                // Add 1 to include the final character
                                end_offset: self.language.len() + territory.len() + 1,
                            }
                        })
                    })
                    .transpose(),
            )?;

        if let Some(codeset) = self.codeset {
            icu_provider::log::warn!("Ignoring codeset: `{codeset}`");
        }

        let mut extensions = Extensions::new();
        let mut script = None;
        let mut variants = vec![variant!("posix")];

        if let Some(modifier) = self.modifier {
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
                "saaho" => {
                    // The only known locale with this modifier is `aa_ER`
                    if self.language != "aa" || self.territory != Some("ER") {
                        icu_provider::log::warn!("Overriding language: `{}`->`ssy`", self.language);
                    }
                    language = language!("ssy")
                }
                // This keeps `variants` sorted; "-posix" comes before "-valencia"
                "valencia" => variants.push(variant!("valencia")),
                // Some modifiers are known but can't be expressed as a BCP-47 identifier
                // e.g. "@abegede", "@iqtelif"
                _ => {
                    icu_provider::log::warn!("Ignoring modifier: `{modifier}`");
                }
            }
        }

        Ok(Locale {
            id: LanguageIdentifier {
                language,
                region,
                script,
                variants: Variants::from_vec_unchecked(variants),
            },
            extensions,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::posix::PosixLocale;
    use icu_locale::Locale;

    fn expect_success(src: &str, expected: &str) {
        let posix_locale = PosixLocale::try_from_str(src)
            .expect(&format!("Unable to parse POSIX locale: `{src}`"));
        let converted_locale = posix_locale
            .try_convert_lossy()
            .expect(&format!("Unable to convert to BCP-47 identifier: `{src}`"));

        let expected_locale = Locale::try_from_str(expected)
            .expect(&format!("Unable to parse BCP-47 identifier: `{expected}`"));
        assert_eq!(converted_locale, expected_locale, "POSIX locale: `{src}`");
    }

    #[test]
    fn default_locale() {
        expect_success("C", "und-posix");
        expect_success("POSIX", "und-posix");
    }

    #[test]
    fn region() {
        expect_success("en_US", "en-US-posix");
        expect_success("ne_NP", "ne-NP-posix");
        expect_success("zh_TW", "zh-TW-posix");
    }

    #[test]
    fn codeset_ignored() {
        expect_success("lv_LV.iso885913", "lv-LV-posix");
        expect_success("hy_AM.armscii8", "hy-AM-posix");
    }

    #[test]
    fn modifier() {
        // Currency
        expect_success("it_IT@euro", "it-IT-posix-u-cu-eur");

        // Script
        expect_success("uz_UZ@cyrillic", "uz-Cyrl-UZ-posix");
        expect_success("sd_IN@devanagari", "sd-Deva-IN-posix");
        expect_success("sr_RS@latin", "sr-Latn-RS-posix");

        // Language
        expect_success("aa_ER@saaho", "ssy-ER-posix");

        // Variant
        expect_success("ca_ES@valencia", "ca-ES-posix-valencia");
    }

    mod error {
        mod parse {
            use crate::parse::posix::{ParseError, PosixLocale};

            fn expect_error(src: &str, expected: ParseError) {
                let result = PosixLocale::try_from_str(src);
                match result {
                    Ok(invalid_locale) => {
                        panic!("Expected the error `{expected:?}`, got the locale `{invalid_locale:?}` from input of `{src}`")
                    }
                    Err(error) => {
                        assert_eq!(error, expected, "Comparing expected output of `{src}`")
                    }
                }
            }

            #[test]
            fn empty_locale() {
                expect_error("", ParseError::EmptyLocale);
            }

            #[test]
            fn empty_section() {
                // Single, empty optional section
                expect_error("en_", ParseError::EmptySection { offset: 2 });
                expect_error("en.", ParseError::EmptySection { offset: 2 });
                expect_error("en@", ParseError::EmptySection { offset: 2 });

                // Multiple optional sections, one empty
                expect_error("en_.utf8@euro", ParseError::EmptySection { offset: 2 });
                expect_error("en_US.@euro", ParseError::EmptySection { offset: 5 });
                expect_error("en_US.utf8@", ParseError::EmptySection { offset: 10 });

                // Single delimiter (excluding "." as that should return `ParseError::InvalidLocale` instead)
                expect_error("_", ParseError::EmptySection { offset: 0 });
                expect_error("@", ParseError::EmptySection { offset: 0 });

                // All delimiters
                expect_error("_.@", ParseError::EmptySection { offset: 0 });
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
                        expect_error(&invalid_locale, ParseError::InvalidCharacter { offset });
                    }
                }

                // Test a single '/' character
                expect_error("/", ParseError::InvalidCharacter { offset: 0 });
            }

            #[test]
            fn invalid_locale() {
                expect_error(".", ParseError::InvalidLocale);
                expect_error("..", ParseError::InvalidLocale);
            }

            #[test]
            fn repeated_delimiter() {
                // Repeated delimiter at the end of locale
                expect_error(
                    "en_US.utf8@euro_US",
                    ParseError::RepeatedDelimiter {
                        first_offset: 2,
                        second_offset: 15,
                    },
                );
                expect_error(
                    "en_US.utf8@euro.utf8",
                    ParseError::RepeatedDelimiter {
                        first_offset: 5,
                        second_offset: 15,
                    },
                );
                expect_error(
                    "en_US.utf8@euro@euro",
                    ParseError::RepeatedDelimiter {
                        first_offset: 10,
                        second_offset: 15,
                    },
                );

                // Multiple repeated delimiters
                expect_error(
                    "en.utf8.utf8.utf8",
                    ParseError::RepeatedDelimiter {
                        first_offset: 2,
                        second_offset: 7,
                    },
                );

                // Consecutive repeated delimiters
                expect_error(
                    "en__US.utf8@euro",
                    ParseError::RepeatedDelimiter {
                        first_offset: 2,
                        second_offset: 3,
                    },
                );
                expect_error(
                    "en_US..utf8@euro",
                    ParseError::RepeatedDelimiter {
                        first_offset: 5,
                        second_offset: 6,
                    },
                );
                expect_error(
                    "en_US.utf8@@euro",
                    ParseError::RepeatedDelimiter {
                        first_offset: 10,
                        second_offset: 11,
                    },
                );
            }

            #[test]
            fn unordered_delimiter() {
                expect_error(
                    "en_US@euro.utf8",
                    ParseError::UnorderedDelimiter {
                        first_offset: 5,
                        second_offset: 10,
                    },
                );
                expect_error(
                    "en.utf8_US@euro",
                    ParseError::UnorderedDelimiter {
                        first_offset: 2,
                        second_offset: 7,
                    },
                );
                expect_error(
                    "en.utf8@euro_US",
                    ParseError::UnorderedDelimiter {
                        first_offset: 7,
                        second_offset: 12,
                    },
                );
                expect_error(
                    "en@euro_US.utf8",
                    ParseError::UnorderedDelimiter {
                        first_offset: 2,
                        second_offset: 7,
                    },
                );
                expect_error(
                    "en@euro.utf8_US",
                    ParseError::UnorderedDelimiter {
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
                    Err(ParseError::EmptySection { offset }) => {
                        assert_eq!(&src[offset..offset + 1], "_");
                    }
                    _ => unreachable!(),
                }

                // Invalid character
                let src = "en_U/S";
                match PosixLocale::try_from_str(src) {
                    Err(ParseError::InvalidCharacter { offset }) => {
                        assert_eq!(&src[offset..offset + 1], "/");
                    }
                    _ => unreachable!(),
                }

                // Repeated delimiter
                let src = "en_US.utf8@euro_US";
                match PosixLocale::try_from_str(src) {
                    Err(ParseError::RepeatedDelimiter {
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
                    Err(ParseError::UnorderedDelimiter {
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
            use crate::parse::posix::{ConversionError, PosixLocale};

            fn expect_error(src: &str, expected: ConversionError) {
                let result = PosixLocale::try_from_str(src)
                    .expect(&format!("Unable to parse POSIX locale: `{src}`"))
                    .try_convert_lossy();
                match result {
                    Ok(invalid_locale) => {
                        panic!("Expected the error `{expected:?}`, got the locale `{invalid_locale:?}` from input of `{src}`")
                    }
                    Err(error) => {
                        assert_eq!(error, expected, "Comparing expected output of `{src}`")
                    }
                }
            }

            #[test]
            fn invalid_language() {
                expect_error(
                    "invalid",
                    ConversionError::InvalidLanguage {
                        start_offset: 0,
                        end_offset: 7,
                    },
                );
            }

            #[test]
            fn invalid_region() {
                expect_error(
                    "en_invalid",
                    ConversionError::InvalidRegion {
                        start_offset: 3,
                        end_offset: 10,
                    },
                );
            }
        }
    }
}
