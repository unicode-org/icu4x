use crate::parser::errors::ParserError;
use std::ops::RangeInclusive;
use std::str::FromStr;
use tinystr::TinyStr8;

/// Variant subtag (examples: `"macos"`, `"posix"`, `"1996"` etc.)
///
/// `Variant` represents a Unicode base language code conformat to the
/// [`unicode_variant_id`] field of the Language and Locale Identifier.
///
/// # Example
/// ```
/// use icu_locale::subtags::Variant;
///
/// let variant: Variant = "macos".parse()
///     .expect("Failed to parse a variant subtag.");
/// ```
///
/// [`unicode_variant_id`]: https://unicode.org/reports/tr35/#unicode_variant_id
#[derive(Default, Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct Variant(Box<[TinyStr8]>);

const VARIANT_LENGTH: RangeInclusive<usize> = 4..=8;
const VARIANT_NUM_ALPHA_LENGTH: usize = 4;

impl Variant {
    /// A constructor which takes a utf8 slice, parses it and
    /// produces a well-formed `Variant`.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_locale::subtags::Variant;
    ///
    /// let variant = Variant::from_bytes(b"posix")
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(variant, "posix");
    /// ```
    pub fn from_bytes(input: &[u8]) -> Result<Self, ParserError> {
        let mut iter = input.split(|c| *c == b'-' || *c == b'_');
        Self::try_from_iter(&mut iter)
    }

    pub fn from_vec_unchecked(input: Vec<TinyStr8>) -> Self {
        Self(input.into_boxed_slice())
    }

    pub(crate) fn try_from_iter<'a>(
        iter: &mut impl Iterator<Item = &'a [u8]>,
    ) -> Result<Self, ParserError> {
        let parts = iter
            .map(|subtag| Self::parse_subtag(subtag))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self(parts.into_boxed_slice()))
    }

    pub fn parse_subtag(v: &[u8]) -> Result<TinyStr8, ParserError> {
        let slen = v.len();

        if !VARIANT_LENGTH.contains(&slen) {
            return Err(ParserError::InvalidSubtag);
        }

        let s = TinyStr8::from_bytes(v).map_err(|_| ParserError::InvalidSubtag)?;

        if !s.is_ascii_alphanumeric() {
            return Err(ParserError::InvalidSubtag);
        }

        if slen == VARIANT_NUM_ALPHA_LENGTH && !v[0].is_ascii_digit() {
            return Err(ParserError::InvalidSubtag);
        }

        Ok(s.to_ascii_lowercase())
    }

    pub fn clear(&mut self) {
        self.0 = Box::new([]);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl FromStr for Variant {
    type Err = ParserError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(source.as_bytes())
    }
}

impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut initial = true;
        for subtag in self.0.iter() {
            if initial {
                write!(f, "{}", subtag)?;
                initial = false;
            } else {
                write!(f, "-{}", subtag)?;
            }
        }
        Ok(())
    }
}
