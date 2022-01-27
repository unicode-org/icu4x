// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Resource paths and related types.

use alloc::borrow::Cow;

use crate::error::{DataError, DataErrorKind};
use crate::helpers;
use core::default::Default;
use core::fmt;
use core::fmt::Write;
use icu_locid::LanguageIdentifier;
use writeable::{LengthHint, Writeable};

/// A compact hash of a [`ResourceKey`]. Useful for keys in maps.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
#[repr(transparent)]
pub struct ResourceKeyHash([u8; 4]);

impl ResourceKeyHash {
    const fn compute_from_str(path: &str) -> Self {
        Self(helpers::fxhash_32(path.as_bytes()).to_le_bytes())
    }
}

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
/// The path string has to contain at least one `/`, and end with `@` followed by one or more ASCII
/// digits. Paths do not contain characters other than ASCII letters and digits, `_`, `/`, `=`, and
/// `@`. Invalid paths are compile-time errors (as [`resource_key!`] uses `const`).
///
/// ```compile_fail,E0080
/// # use icu_provider::prelude::ResourceKey;
/// const K: ResourceKey = icu_provider::resource_key!("foobar@1");
/// ```
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct ResourceKey {
    // This string literal is wrapped in leading_tag!() and trailing_tag!() to make it detectable
    // in a compiled binary.
    path: &'static str,
    hash: ResourceKeyHash,
}

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

impl ResourceKey {
    /// Gets a human-readable representation of a [`ResourceKey`].
    ///
    /// The human-readable path string always contains at least one `/`, and it ends with `@`
    /// followed by one or more digits. Paths do not contain characters other than ASCII letters
    /// and digits, `_`, `/`, `=`, and `@`.
    ///
    /// Useful for reading and writing data to a file system.
    #[inline]
    pub fn get_path(&self) -> &str {
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

        // Regex: [a-zA-Z0-9=_]+(/[a-zA-Z0-9=_]+)*@[0-9]+
        enum State {
            Start,
            Body0,
            Slash,
            Body1,
            AtSign,
            Body2,
        }
        use State::*;
        i = start;
        let mut state = Start;
        loop {
            let content = if i < end {
                Some(path.as_bytes()[i])
            } else {
                None
            };
            state = match (state, content) {
                (Start | Body0, Some(b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'=')) => {
                    Body0
                }
                (Body0, Some(b'/')) => Slash,
                (Slash | Body1, Some(b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'=')) => {
                    Body1
                }
                (Body1, Some(b'/')) => Slash,
                (Body1, Some(b'@')) => AtSign,
                (AtSign | Body2, Some(b'0'..=b'9')) => Body2,

                // Success:
                (Body2, None) => {
                    return Ok(Self {
                        path,
                        hash: ResourceKeyHash::compute_from_str(path),
                    })
                }

                // Errors:
                (Start | Slash, _) => return Err(("[a-zA-Z0-9=_]", i)),
                (Body0, _) => return Err(("[a-zA-z0-9=_/]", i)),
                (Body1, _) => return Err(("[a-zA-z0-9=_/@]", i)),
                (AtSign | Body2, _) => return Err(("[0-9]", i)),
            };
            i += 1;
        }
    }

    /// Gets the last path component of a [`ResourceKey`] without the version suffix.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::prelude::*;
    ///
    /// let resc_key = icu_provider::hello_world::key::HELLO_WORLD_V1;
    /// assert_eq!("helloworld", resc_key.get_last_component_no_version());
    /// ```
    pub fn get_last_component_no_version(&self) -> &str {
        // This cannot fail because of the preconditions on path (at least one '/' and '@')
        // TODO(#1515): Consider deleting this method.
        self.get_path()
            .split('/')
            .last()
            .unwrap()
            .split('@')
            .next()
            .unwrap()
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

    // No slash:
    assert_eq!(
        ResourceKey::construct_internal(tagged!("hello_world@1")),
        Err(("[a-zA-z0-9=_/]", 25))
    );

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

    fn writeable_to_string(&self) -> Cow<str> {
        Cow::Borrowed(self.get_path())
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

impl Writeable for ResourceOptions {
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
        let mut initial = true;
        if let Some(variant) = &self.variant {
            variant.write_to(sink)?;
            initial = false;
        }
        if let Some(langid) = &self.langid {
            if !initial {
                sink.write_char('/')?;
            }
            langid.write_to(sink)?;
        }
        Ok(())
    }

    fn write_len(&self) -> LengthHint {
        let mut length_hint = LengthHint::exact(0);
        let mut initial = true;
        if let Some(variant) = &self.variant {
            length_hint += variant.write_len();
            initial = false;
        }
        if let Some(langid) = &self.langid {
            if !initial {
                length_hint += 1;
            }
            length_hint += langid.write_len();
        }
        length_hint
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
                resc_key: resource_key!("core/cardinal@1"),
                expected: "core/cardinal@1",
            },
            KeyTestCase {
                resc_key: resource_key!("core/maxlengthsubcatg@1"),
                expected: "core/maxlengthsubcatg@1",
            },
            KeyTestCase {
                resc_key: resource_key!("core/cardinal@65535"),
                expected: "core/cardinal@65535",
            },
        ]
    }

    #[test]
    fn test_options_to_string() {
        for cas in get_key_test_cases().iter() {
            assert_eq!(cas.expected, cas.resc_key.to_string());
            writeable::assert_writeable_eq!(&cas.resc_key, cas.expected);
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
