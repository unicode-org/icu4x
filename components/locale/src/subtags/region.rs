use crate::parser::errors::ParserError;
use std::str::FromStr;
use tinystr::TinyStr4;

/// Region subtag (examples: `"US"`, `"CN"`, `"AR"` etc.)
///
/// `Region` represents a Unicode base language code conformat to the
/// [`unicode_region_id`] field of the Language and Locale Identifier.
///
/// # Examples
///
/// ```
/// use icu_locale::subtags::Region;
///
/// let region: Region = "DE".parse()
///     .expect("Failed to parse a region subtag.");
/// ```
///
/// [`unicode_region_id`]: https://unicode.org/reports/tr35/#unicode_region_id
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, Copy)]
pub struct Region(TinyStr4);

const REGION_ALPHA_LENGTH: usize = 2;
const REGION_NUM_LENGTH: usize = 3;

impl Region {
    /// A constructor which takes a utf8 slice, parses it and
    /// produces a well-formed `Region`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::subtags::Region;
    ///
    /// let region = Region::from_bytes(b"fr")
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(region, "FR");
    /// ```
    pub fn from_bytes(v: &[u8]) -> Result<Self, ParserError> {
        match v.len() {
            REGION_ALPHA_LENGTH => {
                let s = TinyStr4::from_bytes(v).map_err(|_| ParserError::InvalidSubtag)?;
                if !s.is_ascii_alphabetic() {
                    return Err(ParserError::InvalidSubtag);
                }
                Ok(Self(s.to_ascii_uppercase()))
            }
            REGION_NUM_LENGTH => {
                let s = TinyStr4::from_bytes(v).map_err(|_| ParserError::InvalidSubtag)?;
                if !s.is_ascii_numeric() {
                    return Err(ParserError::InvalidSubtag);
                }
                Ok(Self(s))
            }
            _ => Err(ParserError::InvalidSubtag),
        }
    }

    /// A helper function for displaying
    /// a `Region` subtag as a `&str`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::subtags::Region;
    ///
    /// let region = Region::from_bytes(b"it")
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(region.as_str(), "IT");
    /// ```
    ///
    /// `Notice`: For many use cases, such as comparison,
    /// `Region` implements `PartialEq<&str>` which allows for direct comparisons.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl FromStr for Region {
    type Err = ParserError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(source.as_bytes())
    }
}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl PartialEq<&str> for Region {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl<'l> From<&'l Region> for &'l str {
    fn from(input: &'l Region) -> Self {
        input.as_str()
    }
}
