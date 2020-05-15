use crate::parser::errors::ParserError;
use std::str::FromStr;
use tinystr::TinyStr4;

/// Script subtag (examples: `"Latn"`, `"Arab"`, etc.)
///
/// `Script` represents a Unicode base language code conformat to the
/// [`unicode_script_id`] field of the Language and Locale Identifier.
///
/// # Example
/// ```
/// use icu_locale::subtags::Script;
///
/// let script: Script = "Latn".parse()
///     .expect("Failed to parse a script subtag.");
/// ```
///
/// [`unicode_script_id`]: https://unicode.org/reports/tr35/#unicode_script_id
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, Copy)]
pub struct Script(TinyStr4);

pub(crate) const SCRIPT_LENGTH: usize = 4;

impl Script {
    /// A constructor which takes a utf8 slice, parses it and
    /// produces a well-formed `Script`.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_locale::subtags::Script;
    ///
    /// let script = Script::from_bytes(b"Latn")
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(script, "Latn");
    /// ```
    pub fn from_bytes(v: &[u8]) -> Result<Self, ParserError> {
        if v.len() != SCRIPT_LENGTH {
            return Err(ParserError::InvalidSubtag);
        }

        let s = TinyStr4::from_bytes(v).map_err(|_| ParserError::InvalidSubtag)?;
        if !s.is_ascii_alphabetic() {
            return Err(ParserError::InvalidSubtag);
        }
        Ok(Self(s.to_ascii_titlecase()))
    }

    /// A helper function for displaying
    /// a `Script` subtag as a `&str`.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_locale::subtags::Script;
    ///
    /// let script = Script::from_bytes(b"Latn")
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(script.as_str(), "Latn");
    /// ```
    ///
    /// `Notice`: For many use cases, such as comparison,
    /// `Script` implements `PartialEq<&str>` which allows for direct comparisons.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl FromStr for Script {
    type Err = ParserError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(source.as_bytes())
    }
}

impl std::fmt::Display for Script {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl PartialEq<&str> for Script {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl<'l> From<&'l Script> for &'l str {
    fn from(input: &'l Script) -> Self {
        input.as_str()
    }
}
