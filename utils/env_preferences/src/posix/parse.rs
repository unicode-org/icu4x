// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use displaydoc::Display;
use icu_locale::extensions::unicode::{key, value};
use icu_locale::extensions::Extensions;
use icu_locale::subtags::{language, script, variant, Language, Region, Variants};
use icu_locale::{LanguageIdentifier, Locale};

use super::aliases::get_bcp47_subtags_from_posix_alias;

#[derive(Display, Debug, PartialEq)]
pub enum ParseError {
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

#[derive(Display, Debug, PartialEq)]
pub enum ConversionError {
    #[displaydoc("Invalid language")]
    InvalidLanguage {
        start_offset: usize,
        end_offset: usize,
    },
    #[displaydoc("Invalid region")]
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
                "saaho" => language = language!("ssy"),
                // This keeps `variants` sorted; "-posix" comes before "-valencia"
                "valencia" => variants.push(variant!("valencia")),
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
                variants: Variants::from_vec_unchecked(variants),
            },
            extensions,
        })
    }
}
