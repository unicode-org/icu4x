// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Documentation on zero-copy deserialization of locale types.
//!
//! [`Locale`] and [`LanguageIdentifier`] are highly structured types that cannot be directly
//! stored in a zero-copy data structure, such as those provided by the [`zerovec`] crate.
//! This page explains how to indirectly store these types in a [`zerovec`].
//!
//! There are two main use cases, which have different solutions:
//!
//! 1. **Lookup:** You need to locate a locale in a zero-copy vector, such as when querying a map.
//! 2. **Obtain:** You have a locale stored in a zero-copy vector, and you need to obtain a proper
//!    [`Locale`] or [`LanguageIdentifier`] for use elsewhere in your program.
//!
//! # Lookup
//!
//! To perform lookup, store the stringified locale in a canonical BCP-47 form as a byte array,
//! and then use [`Locale::cmp_bytes()`] to perform an efficient, zero-allocation lookup.
//!
//! ```
//! use std::str::FromStr;
//! use icu_locid::Locale;
//! use zerovec::ZeroMap;
//!
//! // ZeroMap from locales to integers
//! let data: &[(&[u8], u32)] = &[
//!     (b"de-DE-u-hc-h12", 5),
//!     (b"en-US-u-ca-buddhist", 10),
//!     (b"my-MM", 15),
//!     (b"sr-Cyrl-ME", 20),
//!     (b"zh-TW", 25),
//! ];
//! let zm: ZeroMap<[u8], u32> = data.iter().copied().collect();
//!
//! // Get the value associated with a locale
//! let loc: Locale = "en-US-u-ca-buddhist".parse().unwrap();
//! let value = zm.get_copied_by(|bytes| loc.cmp_bytes(bytes).reverse());
//! assert_eq!(value, Some(10));
//! ```
//!
//! # Obtain
//!
//! Obtaining a [`Locale`] or [`LanguageIdentifier`] is not generally a zero-copy operation, since
//! both of these types may require memory allocation. If possible, architect your code such that
//! you do not need to obtain a structured type.
//!
//! If need the structured type, such as if you need to manipulate it in some way, there are two
//! options: storing subtags, and storing a string for parsing.
//!
//! ## Storing Subtags
//!
//! If the data being stored only contains a limited number of subtags, you can store them as a
//! tuple, and then construct the [`LanguageIdentifier`] externally.
//!
//! ```
//! use icu_locid::LanguageIdentifier;
//! use icu_locid::subtags::{Language, Script, Region};
//! use icu_locid::{language, script, region, langid};
//! use zerovec::ZeroMap;
//!
//! // ZeroMap from integer to LSR (language-script-region)
//! let data: &[(u32, (Language, Option<Script>, Option<Region>))] = &[
//!     (5, (language!("de"), None, Some(region!("DE")))),
//!     (10, (language!("en"), None, Some(region!("US")))),
//!     (15, (language!("my"), None, Some(region!("MM")))),
//!     (20, (language!("sr"), Some(script!("Cyrl")), Some(region!("ME")))),
//!     (25, (language!("zh"), None, Some(region!("TW")))),
//! ];
//! let zm: ZeroMap<u32, (Language, Option<Script>, Option<Region>)> =
//!     data.iter().copied().collect();
//!
//! // Construct a LanguageIdentifier from a tuple entry
//! let lid: LanguageIdentifier = zm.get_copied(&25)
//!     .expect("element is present")
//!     .into();
//!
//! assert_eq!(lid, langid!("zh-TW"));
//! ```
//!
//! ## Storing Strings
//!
//! If it is necessary to store and obtain an arbitrary locale, it is currently recommended to
//! store a BCP-47 string and parse it when needed.
//!
//! Since the string is stored in an unparsed state, it is not safe to `unwrap` the result from
//! `Locale::from_bytes()`. See [icu4x#831](https://github.com/unicode-org/icu4x/issues/831)
//! for a discussion on potential data models that could ensure that the locale is valid during
//! deserialization.
//!
//! ```
//! use icu_locid::Locale;
//! use icu_locid::langid;
//! use zerovec::ZeroMap;
//!
//! // ZeroMap from integer to locale string
//! let data: &[(u32, &[u8])] = &[
//!     (5, b"de-DE-u-hc-h12"),
//!     (10, b"en-US-u-ca-buddhist"),
//!     (15, b"my-MM"),
//!     (20, b"sr-Cyrl-ME"),
//!     (25, b"zh-TW"),
//!     (30, b"INVALID"),
//! ];
//! let zm: ZeroMap<u32, [u8]> = data.iter().copied().collect();
//!
//! // Construct a Locale by parsing the string.
//! let value = zm.get(&25).expect("element is present");
//! let loc = Locale::from_bytes(value);
//! assert_eq!(loc, Ok(langid!("zh-TW").into()));
//!
//! // Invalid entries are fallible
//! let err_value = zm.get(&30).expect("element is present");
//! let err_loc = Locale::from_bytes(err_value);
//! assert!(matches!(err_loc, Err(_)));
//! ```
//!
//! [`Locale`]: crate::Locale
//! [`Locale::cmp_bytes()`]: crate::Locale::cmp_bytes()
//! [`LanguageIdentifier`]: crate::LanguageIdentifier

use crate::subtags::{Language, Region, Script, Variant};
use zerovec::ule::{AsULE, ULE};
use zerovec::ZeroVec;
use zerovec::ZeroVecError;

// Safety checklist for ULE:
//
// 1. Must not include any uninitialized or padding bytes (true since transparent over a ULE).
// 2. Must have an alignment of 1 byte (true since transparent over a ULE).
// 3. ULE::validate_byte_slice() checks that the given byte slice represents a valid slice.
// 4. ULE::validate_byte_slice() checks that the given byte slice has a valid length.
// 5. All other methods must be left with their default impl.
// 6. Byte equality is semantic equality.
unsafe impl ULE for Language {
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> {
        let it = bytes.chunks_exact(core::mem::size_of::<Self>());
        if !it.remainder().is_empty() {
            return Err(ZeroVecError::length::<Self>(bytes.len()));
        }
        for v in it {
            // The following can be removed once `array_chunks` is stabilized.
            let mut a = [0; core::mem::size_of::<Self>()];
            a.copy_from_slice(v);
            if Self::try_from_raw(a).is_err() {
                return Err(ZeroVecError::parse::<Self>());
            }
        }
        Ok(())
    }
}

/// Impl enabling `Language` to be used in a [`ZeroVec`]. Enabled with the `"zerovec"` feature.
///
/// # Example
///
/// ```
/// use icu::locid::subtags::Language;
/// use icu::locid::language;
/// use zerovec::ZeroVec;
///
/// let zv = ZeroVec::<Language>::parse_byte_slice(b"de\0fr\0arsar\0")
///     .expect("Valid language subtags");
/// assert_eq!(zv.get(1), Some(language!("fr")));
///
/// ZeroVec::<Language>::parse_byte_slice(b"invalid")
///     .expect_err("Invalid byte slice");
/// ```
///
/// [`ZeroVec`]: zerovec::ZeroVec
impl AsULE for Language {
    type ULE = Self;
    fn to_unaligned(self) -> Self::ULE {
        self
    }
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        unaligned
    }
}

impl<'a> zerovec::maps::ZeroMapKV<'a> for Language {
    type Container = zerovec::ZeroVec<'a, Language>;
    type GetType = Language;
    type OwnedType = Language;
}

// Safety checklist for ULE:
//
// 1. Must not include any uninitialized or padding bytes (true since transparent over a ULE).
// 2. Must have an alignment of 1 byte (true since transparent over a ULE).
// 3. ULE::validate_byte_slice() checks that the given byte slice represents a valid slice.
// 4. ULE::validate_byte_slice() checks that the given byte slice has a valid length.
// 5. All other methods must be left with their default impl.
// 6. Byte equality is semantic equality.
unsafe impl ULE for Script {
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> {
        let it = bytes.chunks_exact(core::mem::size_of::<Self>());
        if !it.remainder().is_empty() {
            return Err(ZeroVecError::length::<Self>(bytes.len()));
        }
        for v in it {
            // The following can be removed once `array_chunks` is stabilized.
            let mut a = [0; core::mem::size_of::<Self>()];
            a.copy_from_slice(v);
            if Self::try_from_raw(a).is_err() {
                return Err(ZeroVecError::parse::<Self>());
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
/// use icu::locid::script;
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
impl AsULE for Script {
    type ULE = Self;
    fn to_unaligned(self) -> Self::ULE {
        self
    }
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        unaligned
    }
}

impl<'a> zerovec::maps::ZeroMapKV<'a> for Script {
    type Container = ZeroVec<'a, Script>;
    type GetType = Script;
    type OwnedType = Script;
}

// Safety checklist for ULE:
//
// 1. Must not include any uninitialized or padding bytes (true since transparent over a ULE).
// 2. Must have an alignment of 1 byte (true since transparent over a ULE).
// 3. ULE::validate_byte_slice() checks that the given byte slice represents a valid slice.
// 4. ULE::validate_byte_slice() checks that the given byte slice has a valid length.
// 5. All other methods must be left with their default impl.
// 6. Byte equality is semantic equality.
unsafe impl ULE for Region {
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> {
        let it = bytes.chunks_exact(core::mem::size_of::<Self>());
        if !it.remainder().is_empty() {
            return Err(ZeroVecError::length::<Self>(bytes.len()));
        }
        for v in it {
            // The following can be removed once `array_chunks` is stabilized.
            let mut a = [0; core::mem::size_of::<Self>()];
            a.copy_from_slice(v);
            if Self::try_from_raw(a).is_err() {
                return Err(ZeroVecError::parse::<Self>());
            }
        }
        Ok(())
    }
}

/// Impl enabling `Region` to be used in a [`ZeroVec`]. Enabled with the `"zerovec"` feature.
///
/// # Example
///
/// ```
/// use icu::locid::subtags::Region;
/// use icu::locid::region;
/// use zerovec::ZeroVec;
///
/// let zv = ZeroVec::<Region>::parse_byte_slice(b"GB\0419001DE\0")
///     .expect("Valid region subtags");
/// assert_eq!(zv.get(1), Some(region!("419")));
///
/// ZeroVec::<Region>::parse_byte_slice(b"invalid")
///     .expect_err("Invalid byte slice");
/// ```
///
/// [`ZeroVec`]: zerovec::ZeroVec
impl AsULE for Region {
    type ULE = Self;
    fn to_unaligned(self) -> Self::ULE {
        self
    }
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        unaligned
    }
}

impl<'a> zerovec::maps::ZeroMapKV<'a> for Region {
    type Container = ZeroVec<'a, Region>;
    type GetType = Region;
    type OwnedType = Region;
}

// Safety checklist for ULE:
//
// 1. Must not include any uninitialized or padding bytes (true since transparent over a ULE).
// 2. Must have an alignment of 1 byte (true since transparent over a ULE).
// 3. ULE::validate_byte_slice() checks that the given byte slice represents a valid slice.
// 4. ULE::validate_byte_slice() checks that the given byte slice has a valid length.
// 5. All other methods must be left with their default impl.
// 6. Byte equality is semantic equality.
unsafe impl ULE for Variant {
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> {
        let it = bytes.chunks_exact(core::mem::size_of::<Self>());
        if !it.remainder().is_empty() {
            return Err(ZeroVecError::length::<Self>(bytes.len()));
        }
        for v in it {
            // The following can be removed once `array_chunks` is stabilized.
            let mut a = [0; core::mem::size_of::<Self>()];
            a.copy_from_slice(v);
            if Self::try_from_raw(a).is_err() {
                return Err(ZeroVecError::parse::<Self>());
            }
        }
        Ok(())
    }
}

/// Impl enabling `Variant` to be used in a [`ZeroVec`]. Enabled with the `"zerovec"` feature.
///
/// # Example
///
/// ```
/// use icu::locid::subtags::Variant;
/// use icu::locid::variant;
/// use zerovec::ZeroVec;
///
/// let zv = ZeroVec::<Variant>::parse_byte_slice(b"fonipa\0\01992\0\0\0\0posix\0\0\0")
///     .expect("Valid variant subtags");
/// assert_eq!(zv.get(1), Some(variant!("1992")));
///
/// ZeroVec::<Variant>::parse_byte_slice(b"invalid")
///     .expect_err("Invalid byte slice");
/// ```
///
/// [`ZeroVec`]: zerovec::ZeroVec
impl AsULE for Variant {
    type ULE = Self;
    fn to_unaligned(self) -> Self::ULE {
        self
    }
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        unaligned
    }
}

impl<'a> zerovec::maps::ZeroMapKV<'a> for Variant {
    type Container = ZeroVec<'a, Variant>;
    type GetType = Variant;
    type OwnedType = Variant;
}
