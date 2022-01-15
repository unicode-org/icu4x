// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Resource paths and related types.

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::string::ToString;

use crate::error::{DataError, DataErrorKind};
use core::default::Default;
use core::fmt;
use core::fmt::Write;
use icu_locid::LanguageIdentifier;
use tinystr::{TinyStr4};
use writeable::{LengthHint, Writeable};
use crate::helpers;

/// A top-level collection of related resource keys.
#[non_exhaustive]
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum ResourceCategory {
    Core,
    Calendar,
    DateTime,
    Decimal,
    LocaleCanonicalizer,
    Plurals,
    TimeZone,
    Properties,
    ListFormatter,
    Segmenter,
    PrivateUse(TinyStr4),
}

impl ResourceCategory {
    /// Gets or builds a string form of this [`ResourceCategory`].
    pub fn as_str(&self) -> Cow<'static, str> {
        match self {
            Self::Core => Cow::Borrowed("core"),
            Self::Calendar => Cow::Borrowed("calendar"),
            Self::DateTime => Cow::Borrowed("datetime"),
            Self::Decimal => Cow::Borrowed("decimal"),
            Self::LocaleCanonicalizer => Cow::Borrowed("locale_canonicalizer"),
            Self::Plurals => Cow::Borrowed("plurals"),
            Self::TimeZone => Cow::Borrowed("time_zone"),
            Self::Properties => Cow::Borrowed("props"),
            Self::ListFormatter => Cow::Borrowed("list_formatter"),
            Self::Segmenter => Cow::Borrowed("segmenter"),
            Self::PrivateUse(id) => {
                let mut result = String::from("x-");
                result.push_str(id.as_str());
                Cow::Owned(result)
            }
        }
    }
}

impl fmt::Display for ResourceCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.as_str())
    }
}

impl writeable::Writeable for ResourceCategory {
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
        sink.write_str(&self.as_str())
    }

    fn write_len(&self) -> writeable::LengthHint {
        writeable::LengthHint::exact(self.as_str().len())
    }
}

/// A compact hash of a [`ResourceKey`]. Useful for keys in maps.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
#[repr(transparent)]
pub struct ResourceKeyHash([u8; 4]);

impl ResourceKeyHash {
    const fn compute_from_str(path: &str) -> Self {
        Self(helpers::fxhash_32(path.as_bytes()).to_le_bytes())
    }
}

/// The resource key used for loading data from an ICU4X data provider.
///
/// A resource key is tightly coupled with the code that uses it to load data at runtime.
/// Executables can be searched for ResourceKey instances to produce optimized data files.
/// Therefore, users should not generally create ResourceKey instances; they should instead use
/// the ones exported by a component.
#[derive(PartialEq, Eq, Copy, Clone)]
#[repr(C)]
pub struct ResourceKey {
    path: &'static str,
    _tag0: [u8; 8],
    hash: ResourceKeyHash,
    _tag1: [u8; 2],
}

impl ResourceKey {
    /// Gets a human-readable representation of a [`ResourceKey`].
    /// 
    /// The human-readable path string always contains at least one '/', and it ends with '@'
    /// followed by one or more digits. Paths do not contain characters other than lowercase
    /// ASCII, '_', '/', and '@'.
    /// 
    /// Useful for reading and writing data to a file system.
    #[inline]
    pub const fn get_path(&self) -> &str {
        self.path
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

    /// Creates a new ResourceKey from a path, panicking if the path is invalid.
    /// 
    /// # Panics
    /// 
    /// Panics if the syntax of the path is not valid.
    #[inline]
    pub const fn new(path: &'static str) -> Self {
        match Self::try_new(path) {
            Ok(v) => v,
            Err(_) => panic!("{}", path) // Invalid syntax!
        }
    }

    /// Creates a new ResourceKey from a path, returning an error if the path is invalid.
    #[inline]
    pub const fn try_new(path: &'static str) -> Result<Self, ()> {
        match Self::check_path_syntax(path) {
            Ok(_) => Ok(Self {
                path,
                _tag0: *b"ICU4XK[\x02",
                hash: ResourceKeyHash::compute_from_str(path),
                _tag1: *b"\x03]",
            }),
            Err(_) => Err(())
        }
    }

    const fn check_path_syntax(path: &str) -> Result<(), ()> {
        // Approximate regex: \w+(/\w+)*@\d+
        // State 0 = start of string
        // State 1 = after first character
        // State 2 = after a slash
        // State 3 = after a character after a slash
        // State 4 = after @
        // State 5 = after a digit after @
        let mut i = 0;
        let mut state = 0;
        let path_bytes = path.as_bytes();
        while i < path_bytes.len() {
            let c = path_bytes[i];
            state = match (state, c) {
                (0 | 1, b'a'..=b'z' | b'0'..=b'9' | b'_') => 1,
                (1, b'/') => 2,
                (2 | 3, b'a'..=b'z' | b'0'..=b'9' | b'_') => 3,
                (3, b'/') => 2,
                (3, b'@') => 4,
                (4 | 5, b'0'..=b'9') => 5,
                _ => return Err(())
            };
            i += 1;
        }
        if state != 5 {
            return Err(())
        }
        Ok(())
    }
}

#[test]
fn test_path_syntax() {
    // Valid keys:
    assert!(matches!(ResourceKey::try_new("hello/world@1"), Ok(_)));
    assert!(matches!(ResourceKey::try_new("hello/world/foo@1"), Ok(_)));
    assert!(matches!(ResourceKey::try_new("hello/world@999"), Ok(_)));
    assert!(matches!(ResourceKey::try_new("hello_world/foo@1"), Ok(_)));
    assert!(matches!(ResourceKey::try_new("hello_458/world@1"), Ok(_)));

    // No slash:
    assert!(matches!(ResourceKey::try_new("hello_world@1"), Err(_)));

    // No version:
    assert!(matches!(ResourceKey::try_new("hello/world"), Err(_)));
    assert!(matches!(ResourceKey::try_new("hello/world@"), Err(_)));
    assert!(matches!(ResourceKey::try_new("hello/world@foo"), Err(_)));

    // Invalid characters:
    assert!(matches!(ResourceKey::try_new("你好/世界@1"), Err(_)));
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
        self.path.write_to(sink)
    }

    fn write_len(&self) -> LengthHint {
        self.path.write_len()
    }
}

impl ResourceKey {
    /// Gets the standard path components of this [`ResourceKey`]. These components should be used when
    /// persisting the [`ResourceKey`] on the filesystem or in structured data.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::prelude::*;
    ///
    /// let resc_key = icu_provider::hello_world::key::HELLO_WORLD_V1;
    /// let components: Vec<String> = resc_key
    ///     .iter_components()
    ///     .map(|x| x.into_owned())
    ///     .collect();
    ///
    /// assert_eq!(
    ///     ["core", "helloworld@1"],
    ///     components[..]
    /// );
    /// ```
    pub fn iter_components(&self) -> impl Iterator<Item = Cow<str>> {
        self.get_path().split('/').map(|s| Cow::Borrowed(s))
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
    /// const FOO_BAR: ResourceKey = ResourceKey::new("foo/bar@1");
    /// const FOO_BAZ: ResourceKey = ResourceKey::new("foo/baz@1");
    /// const BAR_BAZ: ResourceKey = ResourceKey::new("bar/baz@1");
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

/// A variant and language identifier, used for requesting data from a
/// [`DataProvider`](crate::DataProvider).
///
/// The fields in a [`ResourceOptions`] are not generally known until runtime.
#[derive(PartialEq, Clone, Default)]
pub struct ResourceOptions {
    // TODO: Consider making multiple variant fields.
    pub variant: Option<Cow<'static, str>>,
    pub langid: Option<LanguageIdentifier>,
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

impl writeable::Writeable for ResourceOptions {
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
        let mut initial = true;
        for component in self.iter_components() {
            if initial {
                initial = false;
            } else {
                sink.write_char('/')?;
            }
            sink.write_str(&*component)?;
        }
        Ok(())
    }

    fn write_len(&self) -> writeable::LengthHint {
        let mut result = 0;
        let mut initial = true;
        for component in self.iter_components() {
            if initial {
                initial = false;
            } else {
                result += 1;
            }
            result += component.len();
        }
        writeable::LengthHint::exact(result)
    }
}

impl From<LanguageIdentifier> for ResourceOptions {
    /// Create a ResourceOptions with the given language identifier and an empty variant field.
    fn from(langid: LanguageIdentifier) -> Self {
        Self {
            langid: Some(langid),
            variant: None,
        }
    }
}

impl ResourceOptions {
    /// Gets the standard path components of this [`ResourceOptions`]. These components should be used when
    /// persisting the [`ResourceOptions`] on the filesystem or in structured data.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::borrow::Cow;
    /// use icu_provider::prelude::*;
    /// use icu_locid_macros::langid;
    ///
    /// let resc_options = ResourceOptions {
    ///     variant: Some(Cow::Borrowed("GBP")),
    ///     langid: Some(langid!("pt_BR")),
    /// };
    /// let components: Vec<String> = resc_options
    ///     .iter_components()
    ///     .map(|s| s.into_owned())
    ///     .collect();
    ///
    /// assert_eq!(
    ///     ["GBP", "pt-BR"],
    ///     components[..]
    /// );
    /// ```
    pub fn iter_components(&self) -> impl Iterator<Item = Cow<str>> {
        let components_array: [Option<Cow<str>>; 2] = [
            self.variant.clone(),
            self.langid.as_ref().map(|s| Cow::Owned(s.to_string())),
        ];
        IntoIterator::into_iter(components_array).filter_map(|x| x)
    }

    /// Returns whether this [`ResourceOptions`] has all empty fields (no components).
    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

#[derive(Clone, PartialEq)]
pub struct ResourcePath {
    pub key: ResourceKey,
    pub options: ResourceOptions,
}

impl fmt::Debug for ResourcePath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ResourcePath{{{}}}", self)
    }
}

impl fmt::Display for ResourcePath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeable::Writeable::write_to(self, f)
    }
}

impl writeable::Writeable for ResourcePath {
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
        writeable::Writeable::write_to(&self.key, sink)?;
        if !self.options.is_empty() {
            sink.write_char('/')?;
            writeable::Writeable::write_to(&self.options, sink)?;
        }
        Ok(())
    }

    fn write_len(&self) -> writeable::LengthHint {
        let mut result = writeable::Writeable::write_len(&self.key);
        if !self.options.is_empty() {
            result += writeable::Writeable::write_len(&self.options) + 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct KeyTestCase {
        pub resc_key: ResourceKey,
        pub expected: &'static str,
    }

    fn get_key_test_cases() -> [KeyTestCase; 3] {
        [
            KeyTestCase {
                resc_key: ResourceKey::new("core/cardinal@1"),
                expected: "core/cardinal@1",
            },
            KeyTestCase {
                resc_key: ResourceKey::new("core/maxlengthsubcatg@1"),
                expected: "core/maxlengthsubcatg@1",
            },
            KeyTestCase {
                resc_key: ResourceKey::new("core/cardinal@65535"),
                expected: "core/cardinal@65535",
            },
        ]
    }

    #[test]
    fn test_options_to_string() {
        for cas in get_key_test_cases().iter() {
            assert_eq!(cas.expected, cas.resc_key.to_string());
            writeable::assert_writeable_eq!(&cas.resc_key, cas.expected);
            assert_eq!(
                cas.expected,
                cas.resc_key
                    .iter_components()
                    .collect::<Vec<Cow<str>>>()
                    .join("/")
            );
        }
    }

    struct OptionsTestCase {
        pub resc_options: ResourceOptions,
        pub expected: &'static str,
    }

    fn get_options_test_cases() -> [OptionsTestCase; 3] {
        use icu_locid_macros::langid;
        [
            OptionsTestCase {
                resc_options: ResourceOptions {
                    variant: None,
                    langid: Some(LanguageIdentifier::und()),
                },
                expected: "und",
            },
            OptionsTestCase {
                resc_options: ResourceOptions {
                    variant: Some(Cow::Borrowed("GBP")),
                    langid: Some(LanguageIdentifier::und()),
                },
                expected: "GBP/und",
            },
            OptionsTestCase {
                resc_options: ResourceOptions {
                    variant: Some(Cow::Borrowed("GBP")),
                    langid: Some(langid!("en-ZA")),
                },
                expected: "GBP/en-ZA",
            },
        ]
    }

    #[test]
    fn test_key_to_string() {
        for cas in get_options_test_cases().iter() {
            assert_eq!(cas.expected, cas.resc_options.to_string());
            writeable::assert_writeable_eq!(&cas.resc_options, cas.expected);
            assert_eq!(
                cas.expected,
                cas.resc_options
                    .iter_components()
                    .collect::<Vec<Cow<str>>>()
                    .join("/")
            );
        }
    }

    #[test]
    fn test_resource_path_to_string() {
        for key_cas in get_key_test_cases().iter() {
            for options_cas in get_options_test_cases().iter() {
                let expected = if options_cas.resc_options.is_empty() {
                    key_cas.expected.to_string()
                } else {
                    format!("{}/{}", key_cas.expected, options_cas.expected)
                };
                let resource_path = ResourcePath {
                    key: key_cas.resc_key,
                    // Note: once https://github.com/rust-lang/rust/pull/80470 is accepted,
                    // we won't have to clone here.
                    options: options_cas.resc_options.clone(),
                };
                assert_eq!(expected, resource_path.to_string());
                writeable::assert_writeable_eq!(&resource_path, expected);
            }
        }
    }
}
