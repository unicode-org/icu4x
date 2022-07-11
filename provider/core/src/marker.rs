// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Marker types and traits for DataProvider.

use crate::error::{DataError, DataErrorKind};
use crate::helpers;
use crate::yoke::Yokeable;
use alloc::borrow::Cow;
use core::fmt;
use core::fmt::Write;
use writeable::{LengthHint, Writeable};
use zerovec::ule::*;

/// Trait marker for data structs. All types delivered by the data provider must be associated with
/// something implementing this trait.
///
/// By convention, the non-standard `Marker` suffix is used by types implementing DataMarker.
///
/// In addition to a marker type implementing DataMarker, the following impls must also be present
/// for the data struct:
///
/// - `impl<'a> Yokeable<'a>` (required)
/// - `impl ZeroFrom<Self>`
///
/// See also some common pre-made DataMarker impls in this module.
///
/// # Examples
///
/// Implementing DataMarker for a custom type:
///
/// ```
/// use icu_provider::prelude::*;
/// use icu_provider::yoke::*;
/// use icu_provider::zerofrom::*;
/// use std::borrow::Cow;
/// use std::rc::Rc;
///
/// #[derive(Yokeable, ZeroFrom)]
/// struct MyDataStruct<'data> {
///     message: Cow<'data, str>,
/// }
///
/// struct MyDataStructMarker;
///
/// impl DataMarker for MyDataStructMarker {
///     type Yokeable = MyDataStruct<'static>;
/// }
///
/// // We can now use MyDataStruct with DataProvider:
/// let s = MyDataStruct {
///     message: Cow::Owned("Hello World".into()),
/// };
/// let payload = DataPayload::<MyDataStructMarker>::from_owned(s);
/// assert_eq!(payload.get().message, "Hello World");
/// ```
pub trait DataMarker {
    /// A type that implements [`Yokeable`]. This should typically be the `'static` version of a
    /// data struct.
    type Yokeable: for<'a> Yokeable<'a>;
}

pub trait ResourceMarker: DataMarker {
    const KEY: ResourceKey;
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
    type Slice = zerovec::ZeroSlice<ResourceKeyHash>;
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
/// number). Paths do not contain characters other than ASCII letters and digits, `_`, `/`.
///
/// Invalid paths are compile-time errors (as [`resource_key!`] uses `const`).
///
/// ```compile_fail,E0080
/// # use icu_provider::prelude::ResourceKey;
/// const K: ResourceKey = icu_provider::resource_key!("foo/../bar@1");
/// ```
#[derive(Copy, Clone)]
pub struct ResourceKey {
    // This string literal is wrapped in leading_tag!() and trailing_tag!() to make it detectable
    // in a compiled binary.
    path: &'static str,
    hash: ResourceKeyHash,
}

impl PartialEq for ResourceKey {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash && self.get_path() == other.get_path()
    }
}

impl Eq for ResourceKey {}

impl core::hash::Hash for ResourceKey {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.hash.hash(state)
    }
}

impl ResourceKey {
    /// Gets a human-readable representation of a [`ResourceKey`].
    ///
    /// The human-readable path string ends with `@` followed by one or more digits (the version
    /// number). Paths do not contain characters other than ASCII letters and digits, `_`, `/`.
    ///
    /// Useful for reading and writing data to a file system.
    #[inline]
    pub fn get_path(&self) -> &'static str {
        // This becomes const in 1.64
        unsafe {
            // Safe due to invariant that self.path is tagged correctly
            core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                self.path.as_ptr().add(leading_tag!().len()),
                self.path.len() - trailing_tag!().len() - leading_tag!().len(),
            ))
        }
    }

    /// Gets a platform-independent hash of a [`ResourceKey`].
    ///
    /// The hash is 4 bytes and allows for fast key comparison.
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

        // Regex: [a-zA-Z0-9_][a-zA-Z0-9_/]*@[0-9]+
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
                (Empty | Body, Some(b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_')) => Body,
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

                (Empty, _) => return Err(("[a-zA-Z0-9_]", i)),
                (Body, _) => return Err(("[a-zA-z0-9_/@]", i)),
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
    /// assert!(matches!(FOO_BAR.match_key(FOO_BAR), Ok(())));
    /// assert!(matches!(
    ///     FOO_BAR.match_key(FOO_BAZ),
    ///     Err(DataError {
    ///         kind: DataErrorKind::MissingResourceKey,
    ///         ..
    ///     })
    /// ));
    /// assert!(matches!(
    ///     FOO_BAR.match_key(BAR_BAZ),
    ///     Err(DataError {
    ///         kind: DataErrorKind::MissingResourceKey,
    ///         ..
    ///     })
    /// ));
    ///
    /// // The error context contains the argument:
    /// assert_eq!(FOO_BAR.match_key(BAR_BAZ).unwrap_err().key, Some(BAR_BAZ));
    /// ```
    pub fn match_key(&self, key: Self) -> Result<(), DataError> {
        if *self == key {
            Ok(())
        } else {
            Err(DataErrorKind::MissingResourceKey.with_key(key))
        }
    }
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
        Err((
            "[a-zA-z0-9_/@]",
            concat!(leading_tag!(), "hello/world").len()
        ))
    );

    assert_eq!(
        ResourceKey::construct_internal(tagged!("hello/world@")),
        Err(("[0-9]", concat!(leading_tag!(), "hello/world@").len()))
    );
    assert_eq!(
        ResourceKey::construct_internal(tagged!("hello/world@foo")),
        Err(("[0-9]", concat!(leading_tag!(), "hello/world@").len()))
    );
    assert_eq!(
        ResourceKey::construct_internal(tagged!("hello/world@1foo")),
        Err(("[0-9]", concat!(leading_tag!(), "hello/world@1").len()))
    );

    // Invalid characters:
    assert_eq!(
        ResourceKey::construct_internal(tagged!("你好/世界@1")),
        Err(("[a-zA-Z0-9_]", leading_tag!().len()))
    );

    // Invalid tag:
    assert_eq!(
        ResourceKey::construct_internal(concat!("hello/world@1", trailing_tag!())),
        Err(("tag", 0))
    );
    assert_eq!(
        ResourceKey::construct_internal(concat!(leading_tag!(), "hello/world@1")),
        Err(("tag", concat!(leading_tag!(), "hello/world@1").len()))
    );
    assert_eq!(
        ResourceKey::construct_internal("hello/world@1"),
        Err(("tag", 0))
    );
}

#[test]
fn test_key_to_string() {
    struct KeyTestCase {
        pub key: ResourceKey,
        pub expected: &'static str,
    }

    for cas in [
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
    ] {
        assert_eq!(cas.expected, cas.key.to_string());
        writeable::assert_writeable_eq!(&cas.key, cas.expected);
    }
}
