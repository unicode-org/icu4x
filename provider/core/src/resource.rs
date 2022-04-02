// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Resource paths and related types.

use alloc::borrow::Cow;

use crate::error::{DataError, DataErrorKind};
use crate::helpers;
use alloc::string::{String, ToString};
use alloc::vec;
use core::default::Default;
use core::fmt;
use core::fmt::Write;
use icu_locid::extensions::unicode as unicode_ext;
use icu_locid::subtags::{Language, Script, Region};
use icu_locid::{LanguageIdentifier, Locale};
use writeable::{LengthHint, Writeable};
use zerovec::ule::*;

#[doc(hidden)]
#[macro_export]
macro_rules! leading_tag {
    () => {
        "\nicu4x_key_tag"
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! trailing_tag {
    () => {
        "\n"
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! tagged {
    ($without_tags:expr) => {
        concat!(
            $crate::leading_tag!(),
            $without_tags,
            $crate::trailing_tag!()
        )
    };
}

/// A compact hash of a [`ResourceKey`]. Useful for keys in maps.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash, ULE)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct ResourceKeyHash([u8; 4]);

impl ResourceKeyHash {
    const fn compute_from_str(path: &str) -> Self {
        Self(
            helpers::fxhash_32(path.as_bytes(), leading_tag!().len(), trailing_tag!().len())
                .to_le_bytes(),
        )
    }
}

impl<'a> zerovec::maps::ZeroMapKV<'a> for ResourceKeyHash {
    type Container = zerovec::ZeroVec<'a, ResourceKeyHash>;
    type GetType = <ResourceKeyHash as AsULE>::ULE;
    type OwnedType = ResourceKeyHash;
}

impl AsULE for ResourceKeyHash {
    type ULE = Self;
    #[inline]
    fn to_unaligned(self) -> Self::ULE {
        self
    }
    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        unaligned
    }
}

// Safe since the ULE type is `self`.
unsafe impl EqULE for ResourceKeyHash {}

/// Used for loading data from an ICU4X data provider.
///
/// A resource key is tightly coupled with the code that uses it to load data at runtime.
/// Executables can be searched for `ResourceKey` instances to produce optimized data files.
/// Therefore, users should not generally create ResourceKey instances; they should instead use
/// the ones exported by a component.
///
/// `ResourceKey`s are created with the [`resource_key!`] macro:
///
/// ```
/// # use icu_provider::prelude::ResourceKey;
/// const K: ResourceKey = icu_provider::resource_key!("foo/bar@1");
/// ```
///
/// The human-readable path string ends with `@` followed by one or more digits (the version
/// number). Paths do not contain characters other than ASCII letters and digits, `_`, `/`, `=`.
///
/// Invalid paths are compile-time errors (as [`resource_key!`] uses `const`).
///
/// ```compile_fail,E0080
/// # use icu_provider::prelude::ResourceKey;
/// const K: ResourceKey = icu_provider::resource_key!("foo/../bar@1");
/// ```
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct ResourceKey {
    // This string literal is wrapped in leading_tag!() and trailing_tag!() to make it detectable
    // in a compiled binary.
    path: &'static str,
    hash: ResourceKeyHash,
}

impl ResourceKey {
    /// Gets a human-readable representation of a [`ResourceKey`].
    ///
    /// The human-readable path string ends with `@` followed by one or more digits (the version
    /// number). Paths do not contain characters other than ASCII letters and digits, `_`, `/`, `=`.
    ///
    /// Useful for reading and writing data to a file system.
    #[inline]
    pub fn get_path(&self) -> &'static str {
        // This becomes const with `const_ptr_offset` and `const_slice_from_raw_parts`.
        unsafe {
            // Safe due to invariant that self.path is tagged correctly
            core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                self.path.as_ptr().add(leading_tag!().len()),
                self.path.len() - trailing_tag!().len() - leading_tag!().len(),
            ))
        }
    }

    /// Gets a machine-readable representation of a [`ResourceKey`].
    ///
    /// The machine-readable hash is 4 bytes and can be used as the key in a map.
    ///
    /// The hash is a 32-bit FxHash of the path, computed as if on a little-endian platform.
    #[inline]
    pub const fn get_hash(&self) -> ResourceKeyHash {
        self.hash
    }

    #[doc(hidden)]
    // Error is a str of the expected character class and the index where it wasn't encountered
    // The indexing operations in this function have been reviewed in detail and won't panic.
    #[allow(clippy::indexing_slicing)]
    pub const fn construct_internal(path: &'static str) -> Result<Self, (&'static str, usize)> {
        if path.len() < leading_tag!().len() + trailing_tag!().len() {
            return Err(("tag", 0));
        }
        // Start and end of the untagged part
        let start = leading_tag!().len();
        let end = path.len() - trailing_tag!().len();

        // Check tags
        let mut i = 0;
        while i < leading_tag!().len() {
            if path.as_bytes()[i] != leading_tag!().as_bytes()[i] {
                return Err(("tag", 0));
            }
            i += 1;
        }
        i = 0;
        while i < trailing_tag!().len() {
            if path.as_bytes()[end + i] != trailing_tag!().as_bytes()[i] {
                return Err(("tag", end + 1));
            }
            i += 1;
        }

        // Regex: [a-zA-Z0-9=_][a-zA-Z0-9=_/]*@[0-9]+
        enum State {
            Empty,
            Body,
            At,
            Version,
        }
        use State::*;
        i = start;
        let mut state = Empty;
        loop {
            let byte = if i < end {
                Some(path.as_bytes()[i])
            } else {
                None
            };
            state = match (state, byte) {
                (Empty | Body, Some(b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'=')) => Body,
                (Body, Some(b'/')) => Body,
                (Body, Some(b'@')) => At,
                (At | Version, Some(b'0'..=b'9')) => Version,
                // One of these cases will be hit at the latest when i == end, so the loop converges.
                (Version, None) => {
                    return Ok(Self {
                        path,
                        hash: ResourceKeyHash::compute_from_str(path),
                    })
                }

                (Empty, _) => return Err(("[a-zA-Z0-9=_]", i)),
                (Body, _) => return Err(("[a-zA-z0-9=_/@]", i)),
                (At | Version, _) => return Err(("[0-9]", i)),
            };
            i += 1;
        }
    }

    /// Returns [`Ok`] if this data key matches the argument, or the appropriate error.
    ///
    /// Convenience method for data providers that support a single [`ResourceKey`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::prelude::*;
    ///
    /// const FOO_BAR: ResourceKey = icu_provider::resource_key!("foo/bar@1");
    /// const FOO_BAZ: ResourceKey = icu_provider::resource_key!("foo/baz@1");
    /// const BAR_BAZ: ResourceKey = icu_provider::resource_key!("bar/baz@1");
    ///
    /// assert!(matches!(
    ///     FOO_BAR.match_key(FOO_BAR),
    ///     Ok(())
    /// ));
    /// assert!(matches!(
    ///     FOO_BAR.match_key(FOO_BAZ),
    ///     Err(DataError { kind: DataErrorKind::MissingResourceKey, .. })
    /// ));
    /// assert!(matches!(
    ///     FOO_BAR.match_key(BAR_BAZ),
    ///     Err(DataError { kind: DataErrorKind::MissingResourceKey, .. })
    /// ));
    ///
    /// // The error context contains the argument:
    /// assert_eq!(
    ///     FOO_BAR.match_key(BAR_BAZ).unwrap_err().key,
    ///     Some(BAR_BAZ)
    /// );
    /// ```
    pub fn match_key(&self, key: Self) -> Result<(), DataError> {
        if *self == key {
            Ok(())
        } else {
            Err(DataErrorKind::MissingResourceKey.with_key(key))
        }
    }
}

#[test]
fn test_path_syntax() {
    // Valid keys:
    assert!(ResourceKey::construct_internal(tagged!("hello/world@1")).is_ok());
    assert!(ResourceKey::construct_internal(tagged!("hello/world/foo@1")).is_ok());
    assert!(ResourceKey::construct_internal(tagged!("hello/world@999")).is_ok());
    assert!(ResourceKey::construct_internal(tagged!("hello_world/foo@1")).is_ok());
    assert!(ResourceKey::construct_internal(tagged!("hello_458/world@1")).is_ok());
    assert!(ResourceKey::construct_internal(tagged!("hello_world@1")).is_ok());

    // No version:
    assert_eq!(
        ResourceKey::construct_internal(tagged!("hello/world")),
        Err(("[a-zA-z0-9=_/@]", 25))
    );

    assert_eq!(
        ResourceKey::construct_internal(tagged!("hello/world@")),
        Err(("[0-9]", 26))
    );
    assert_eq!(
        ResourceKey::construct_internal(tagged!("hello/world@foo")),
        Err(("[0-9]", 26))
    );
    assert_eq!(
        ResourceKey::construct_internal(tagged!("hello/world@1foo")),
        Err(("[0-9]", 27))
    );

    // Invalid characters:
    assert_eq!(
        ResourceKey::construct_internal(tagged!("你好/世界@1")),
        Err(("[a-zA-Z0-9=_]", 14))
    );

    // Invalid tag:
    assert_eq!(
        ResourceKey::construct_internal(concat!("hello/world@1", trailing_tag!())),
        Err(("tag", 0))
    );
    assert_eq!(
        ResourceKey::construct_internal(concat!(leading_tag!(), "hello/world@1")),
        Err(("tag", 27))
    );
    assert_eq!(
        ResourceKey::construct_internal("hello/world@1"),
        Err(("tag", 0))
    );
}

/// See [`ResourceKey`].
#[macro_export]
macro_rules! resource_key {
    ($path:expr) => {{
        // Force the ResourceKey into a const context
        const RESOURCE_KEY_MACRO_CONST: $crate::ResourceKey = {
            match $crate::ResourceKey::construct_internal($crate::tagged!($path)) {
                Ok(v) => v,
                #[allow(clippy::panic)]
                // TODO(#1668) Clippy exceptions need docs or fixing.
                Err(_) => panic!(concat!("Invalid resource key: ", $path)),
                // TODO Once formatting is const:
                // Err((expected, index)) => panic!(
                //     "Invalid resource key {:?}: expected {:?}, found {:?} ",
                //     $path,
                //     expected,
                //     $crate::tagged!($path).get(index..))
                // );
            }
        };
        RESOURCE_KEY_MACRO_CONST
    }};
}

impl fmt::Debug for ResourceKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ResourceKey{")?;
        fmt::Display::fmt(self, f)?;
        f.write_char('}')?;
        Ok(())
    }
}

impl fmt::Display for ResourceKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Writeable::write_to(self, f)
    }
}

impl Writeable for ResourceKey {
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
        self.get_path().write_to(sink)
    }

    fn write_len(&self) -> LengthHint {
        self.get_path().write_len()
    }

    fn write_to_string(&self) -> Cow<str> {
        Cow::Borrowed(self.get_path())
    }
}

/// A variant and language identifier, used for requesting data from a data provider.
///
/// The fields in a [`ResourceOptions`] are not generally known until runtime.
#[derive(PartialEq, Clone, Default, PartialOrd, Eq, Ord)]
pub struct ResourceOptions {
    langid: LanguageIdentifier,
    keywords: unicode_ext::Keywords,
}

impl fmt::Debug for ResourceOptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ResourceOptions{{{}}}", self)
    }
}

impl fmt::Display for ResourceOptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeable::Writeable::write_to(self, f)
    }
}

impl Writeable for ResourceOptions {
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
        self.langid.write_to(sink)?;
        if !self.keywords.is_empty() {
            sink.write_str("-u-")?;
            self.keywords.write_to(sink)?;
        }
        Ok(())
    }

    fn write_len(&self) -> LengthHint {
        self.langid.write_len()
            + if !self.keywords.is_empty() { 3 } else { 0 }
            + self.keywords.write_len()
    }
}

impl From<LanguageIdentifier> for ResourceOptions {
    fn from(langid: LanguageIdentifier) -> Self {
        Self {
            langid,
            keywords: unicode_ext::Keywords::new(),
        }
    }
}

impl From<Locale> for ResourceOptions {
    fn from(locale: Locale) -> Self {
        // TODO(#1109): Implement proper vertical fallback
        debug_assert!(locale.extensions.is_empty());
        Self {
            langid: locale.id,
            keywords: locale.extensions.unicode.keywords,
        }
    }
}

impl ResourceOptions {
    /// TODO(#1109): Delete this function and use vertical fallback instead
    pub fn temp_for_region(region: Option<Region>) -> Self {
        Self {
            langid: LanguageIdentifier::from(region),
            keywords: unicode_ext::Keywords::new(),
        }
    }

    // TODO(#1109): Delete this function and use vertical fallback instead
    #[allow(clippy::unwrap_used)] // temporary function
    pub fn temp_with_unicode_ext(langid: LanguageIdentifier, key: &str, value: &str) -> Self {
        let key = unicode_ext::Key::from_bytes(key.as_bytes()).unwrap();
        let value = unicode_ext::Value::from_bytes(value.as_bytes()).unwrap();
        Self {
            langid,
            keywords: unicode_ext::Keywords::from_vec_unchecked(vec![(key, value)]),
        }
    }

    // TODO(#1109): Delete this function and use vertical fallback instead
    #[allow(clippy::unwrap_used)] // temporary function
    pub fn temp_get_extension(&self, key: &str) -> Option<String> {
        let key = unicode_ext::Key::from_bytes(key.as_bytes()).unwrap();
        self.keywords.get(key).map(|v| v.to_string())
    }

    /// Returns whether this [`ResourceOptions`] has all empty fields (no components).
    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }

    /// Returns whether the [`LanguageIdentifier`] associated with this request is `und`.
    pub fn is_und(&self) -> bool {
        self.langid == LanguageIdentifier::und()
    }

    /// Returns the [`LanguageIdentifier`] for this [`ResourceOptions`].
    pub fn langid(&self) -> LanguageIdentifier {
        self.langid.clone()
    }

    /// Returns the [`Language`] for this [`ResourceOptions`].
    pub fn language(&self) -> Language {
        self.langid.language
    }

    /// Returns the [`Script`] for this [`ResourceOptions`].
    pub fn script(&self) -> Option<Script> {
        self.langid.script
    }

    /// Returns the [`Region`] for this [`ResourceOptions`].
    pub fn region(&self) -> Option<Region> {
        self.langid.region
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct KeyTestCase {
        pub key: ResourceKey,
        pub expected: &'static str,
    }

    fn get_key_test_cases() -> [KeyTestCase; 3] {
        [
            KeyTestCase {
                key: resource_key!("core/cardinal@1"),
                expected: "core/cardinal@1",
            },
            KeyTestCase {
                key: resource_key!("core/maxlengthsubcatg@1"),
                expected: "core/maxlengthsubcatg@1",
            },
            KeyTestCase {
                key: resource_key!("core/cardinal@65535"),
                expected: "core/cardinal@65535",
            },
        ]
    }

    #[test]
    fn test_options_to_string() {
        for cas in get_key_test_cases().iter() {
            assert_eq!(cas.expected, cas.key.to_string());
            writeable::assert_writeable_eq!(&cas.key, cas.expected);
        }
    }

    struct OptionsTestCase {
        pub options: ResourceOptions,
        pub expected: &'static str,
    }

    fn get_options_test_cases() -> [OptionsTestCase; 3] {
        use std::str::FromStr;
        [
            OptionsTestCase {
                options: Locale::UND.into(),
                expected: "und",
            },
            OptionsTestCase {
                options: Locale::from_str("und-u-cu-gbp").unwrap().into(),
                expected: "und-u-cu-gbp",
            },
            OptionsTestCase {
                options: Locale::from_str("en-ZA-u-cu-gbp").unwrap().into(),
                expected: "en-ZA-u-cu-gbp",
            },
        ]
    }

    #[test]
    fn test_key_to_string() {
        for cas in get_options_test_cases().iter() {
            assert_eq!(cas.expected, cas.options.to_string());
            writeable::assert_writeable_eq!(&cas.options, cas.expected);
        }
    }
}
