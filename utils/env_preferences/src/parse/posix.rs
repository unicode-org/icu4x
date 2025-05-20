// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Parsing functionality for POSIX locale identifiers.
//! For more information, see [`PosixLocale`].
//!
//! # Usage example
//! ```
//! # use icu_locale_core::locale;
//! # use env_preferences::parse::posix::PosixLocale;
//! # use env_preferences::LocaleError;
//! # fn main() -> Result<(), LocaleError> {
//! let posix_locale = PosixLocale::try_from_str("en_US")?;
//! assert_eq!(posix_locale.try_convert_lossy()?, locale!("en-US"));
//! # Ok(())
//! # }
//! ```

use displaydoc::Display;
use icu_locale_core::extensions::unicode::{key, value};
use icu_locale_core::extensions::Extensions;
use icu_locale_core::subtags::{language, script, variant, Language, Region, Variants};
use icu_locale_core::{locale, LanguageIdentifier, Locale};

use crate::ParseError;

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
    /// For example:
    /// - All sections: `en_US.utf8@euro`
    /// - Only required sections: `en`
    ///
    /// See section 8.2 of the POSIX spec for more details:
    /// <https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/V1_chap08.html#tag_08_02>
    pub fn try_from_str(src: &'src str) -> Result<Self, ParseError> {
        // These cases are implementation-defined and can be ignored:
        // - Empty locales
        if src.is_empty() {
            return Err(ParseError::Posix(PosixParseError::EmptyLocale));
        }
        // - Any locale containing '/'
        if let Some(offset) = src.find('/') {
            return Err(ParseError::Posix(PosixParseError::InvalidCharacter {
                offset,
            }));
        }
        // - Locales consisting of "." or ".."
        if src == "." || src == ".." {
            return Err(ParseError::Posix(PosixParseError::InvalidLocale));
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
            return Err(ParseError::Posix(PosixParseError::EmptySection {
                offset: 0,
            }));
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
                return Err(ParseError::Posix(PosixParseError::EmptySection {
                    offset: *start_offset,
                }));
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
    /// # Examples
    ///
    /// ## Parsing behaviour
    /// ```
    /// # use icu_locale_core::locale;
    /// # use env_preferences::parse::posix::PosixLocale;
    /// # use env_preferences::LocaleError;
    /// # fn main() -> Result<(), LocaleError> {
    /// // Locales will always include the `posix` variant
    /// assert_eq!(
    ///     PosixLocale::try_from_str("en_US")?.try_convert_lossy()?,
    ///     locale!("en-US")
    /// );
    /// // The codeset field will be ignored
    /// assert_eq!(
    ///     PosixLocale::try_from_str("en_US.iso88591")?.try_convert_lossy()?,
    ///     locale!("en-US")
    /// );
    /// // Any unknown modifiers will be ignored
    /// assert_eq!(
    ///     PosixLocale::try_from_str("en_US@unknown")?.try_convert_lossy()?,
    ///     locale!("en-US")
    /// );
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ## Edge cases
    /// ```
    /// # use icu_locale_core::locale;
    /// # use env_preferences::parse::posix::PosixLocale;
    /// # use env_preferences::LocaleError;
    /// # fn main() -> Result<(), LocaleError> {
    /// // The default "C"/"POSIX" locale will be converted to "en-US-posix"
    /// assert_eq!(
    ///     PosixLocale::try_from_str("C")?.try_convert_lossy()?,
    ///     locale!("en-US-posix")
    /// );
    /// assert_eq!(
    ///     PosixLocale::try_from_str("POSIX")?.try_convert_lossy()?,
    ///     locale!("en-US-posix")
    /// );
    ///
    /// // Known script modifiers will be converted to the matching CLDR keys
    /// assert_eq!(
    ///     PosixLocale::try_from_str("uz_UZ@cyrillic")?.try_convert_lossy()?,
    ///     locale!("uz-Cyrl-UZ")
    /// );
    /// assert_eq!(
    ///     PosixLocale::try_from_str("ks_IN@devanagari")?.try_convert_lossy()?,
    ///     locale!("ks-Deva-IN")
    /// );
    /// assert_eq!(
    ///     PosixLocale::try_from_str("be_BY@latin")?.try_convert_lossy()?,
    ///     locale!("be-Latn-BY")
    /// );
    ///
    /// // Other known modifiers are handled accordingly
    /// assert_eq!(
    ///     PosixLocale::try_from_str("en_US@euro")?.try_convert_lossy()?,
    ///     locale!("en-US-u-cu-eur")
    /// );
    /// assert_eq!(
    ///     PosixLocale::try_from_str("aa_ER@saaho")?.try_convert_lossy()?,
    ///     locale!("ssy-ER")
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_convert_lossy(&self) -> Result<Locale, ParseError> {
        // The default "C"/"POSIX" locale should map to "en-US-posix",
        // which is the default behaviour in ICU4C:
        // https://github.com/unicode-org/icu/blob/795d7ac82c4b29cf721d0ad62c0b178347d453bf/icu4c/source/common/putil.cpp#L1738
        if self.language == "C" || self.language == "POSIX" {
            return Ok(locale!("en-US-posix"));
        }

        let mut extensions = Extensions::new();
        let mut script = None;
        let mut variant = None;

        // Parse the language/region
        let mut language = Language::try_from_str(self.language)?;
        let region = self.territory.map(Region::try_from_str).transpose()?;

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
