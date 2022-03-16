// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::parser::errors::ParserError;
use core::str::FromStr;
use tinystr::TinyAsciiStr;

/// A script subtag (examples: `"Latn"`, `"Arab"`, etc.)
///
/// [`Script`] represents a Unicode base language code conformat to the
/// [`unicode_script_id`] field of the Language and Locale Identifier.
///
/// # Examples
///
/// ```
/// use icu::locid::subtags::Script;
///
/// let script: Script = "Latn".parse()
///     .expect("Failed to parse a script subtag.");
/// ```
///
/// [`unicode_script_id`]: https://unicode.org/reports/tr35/#unicode_script_id
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, Copy)]
#[repr(transparent)]
pub struct Script(TinyAsciiStr<SCRIPT_LENGTH>);

pub const SCRIPT_LENGTH: usize = 4;

impl Script {
    /// A constructor which takes a utf8 slice, parses it and
    /// produces a well-formed [`Script`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Script;
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

        let s = TinyAsciiStr::from_bytes(v).map_err(|_| ParserError::InvalidSubtag)?;
        if !s.is_ascii_alphabetic() {
            return Err(ParserError::InvalidSubtag);
        }
        Ok(Self(s.to_ascii_titlecase()))
    }

    /// Safely creates a [`Script`] from a reference to its raw format
    /// as returned by [`Script::into_raw()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Script;
    ///
    /// assert!(matches!(Script::try_from_raw(*b"Latn"), Ok(_)));
    /// assert!(matches!(Script::try_from_raw(*b"Mymr"), Ok(_)));
    /// assert!(matches!(Script::try_from_raw(*b"1234"), Err(_)));
    ///
    /// // Unlike the other constructors, this one is case-sensitive:
    /// assert!(matches!(Script::try_from_raw(*b"LATN"), Err(_)));
    /// ```
    pub fn try_from_raw(v: [u8; 4]) -> Result<Self, ParserError> {
        let s = TinyAsciiStr::<{ core::mem::size_of::<Self>() }>::try_from_raw(v)
            .map_err(|_| ParserError::InvalidSubtag)?;
        let is_valid = match s.len() {
            SCRIPT_LENGTH => s.is_ascii_alphabetic() && s.is_ascii_titlecase(),
            _ => false,
        };
        if is_valid {
            Ok(Self(s))
        } else {
            Err(ParserError::InvalidSubtag)
        }
    }

    /// Deconstructs the [`Script`] into raw format to be consumed
    /// by [`from_raw_unchecked()`](Script::from_raw_unchecked()).
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Script;
    ///
    /// let script = Script::from_bytes(b"Latn")
    ///     .expect("Parsing failed.");
    ///
    /// let raw = script.into_raw();
    /// let script = unsafe { Script::from_raw_unchecked(raw) };
    /// assert_eq!(script, "Latn");
    /// ```
    pub fn into_raw(self) -> [u8; 4] {
        *self.0.all_bytes()
    }

    /// Constructor which takes a raw value returned by
    /// [`into_raw`](Script::into_raw()).
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Script;
    ///
    /// let script = Script::from_bytes(b"Latn")
    ///     .expect("Parsing failed.");
    ///
    /// let raw = script.into_raw();
    /// let script = unsafe { Script::from_raw_unchecked(raw) };
    /// assert_eq!(script, "Latn");
    /// ```
    ///
    /// # Safety
    ///
    /// This function accepts a [u8; 4] that is expected to be a valid [`TinyAsciiStr<4>`]
    /// representing a [`Script`] subtag in canonical syntax.
    pub const unsafe fn from_raw_unchecked(v: [u8; 4]) -> Self {
        Self(TinyAsciiStr::from_bytes_unchecked(v))
    }

    /// A helper function for displaying
    /// a [`Script`] subtag as a `&`[`str`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Script;
    ///
    /// let script = Script::from_bytes(b"Latn")
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(script.as_str(), "Latn");
    /// ```
    ///
    /// `Notice`: For many use cases, such as comparison,
    /// [`Script`] implements [`PartialEq`]`<&`[`str`]`>` which allows for direct comparisons.
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

impl_writeable_for_single_subtag!(Script, "Mymr");

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

impl From<Script> for TinyAsciiStr<4> {
    fn from(input: Script) -> Self {
        input.0
    }
}

// Safety checklist for ULE:
//
// 1. Must not include any uninitialized or padding bytes (true since transparent over a ULE).
// 2. Must have an alignment of 1 byte (true since transparent over a ULE).
// 3. ULE::validate_byte_slice() checks that the given byte slice represents a valid slice.
// 4. ULE::validate_byte_slice() checks that the given byte slice has a valid length.
// 5. All other methods must be left with their default impl.
// 6. Byte equality is semantic equality.
#[cfg(feature = "zerovec")]
unsafe impl zerovec::ule::ULE for Script {
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), zerovec::ZeroVecError> {
        let it = bytes.chunks_exact(core::mem::size_of::<Self>());
        if !it.remainder().is_empty() {
            return Err(zerovec::ZeroVecError::length::<Self>(bytes.len()));
        }
        for v in it {
            // The following can be removed once `array_chunks` is stabilized.
            let mut a = [0; core::mem::size_of::<Self>()];
            a.copy_from_slice(v);
            if Self::try_from_raw(a).is_err() {
                return Err(zerovec::ZeroVecError::parse::<Self>());
            }
        }
        Ok(())
    }
}

/// Impl enabling `Script` to be used in a [`ZeroVec`]. Enabled with the `"zerovec"` feature.
///
/// # Example
///
/// ```
/// use icu::locid::subtags::Script;
/// use icu::locid::macros::script;
/// use zerovec::ZeroVec;
///
/// let zv = ZeroVec::<Script>::parse_byte_slice(b"LatnAdlmMymrLatnLatn")
///     .expect("Valid script subtags");
/// assert_eq!(zv.get(1), Some(script!("Adlm")));
///
/// ZeroVec::<Script>::parse_byte_slice(b"invalid")
///     .expect_err("Invalid byte slice");
/// ```
///
/// [`ZeroVec`]: zerovec::ZeroVec
#[cfg(feature = "zerovec")]
impl zerovec::ule::AsULE for Script {
    type ULE = Self;
    fn to_unaligned(self) -> Self::ULE {
        self
    }
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        unaligned
    }
}

#[cfg(feature = "zerovec")]
impl<'a> zerovec::maps::ZeroMapKV<'a> for Script {
    type Container = zerovec::ZeroVec<'a, Script>;
    type GetType = Script;
    type OwnedType = Script;
}
