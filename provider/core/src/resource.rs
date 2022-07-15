// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Resource paths and related types.

use alloc::borrow::Cow;
use icu_locid::ordering::SubtagOrderingResult;

use crate::error::{DataError, DataErrorKind};
use crate::helpers;
use core::cmp::Ordering;
use core::default::Default;
use core::fmt;
use core::fmt::Write;
use icu_locid::extensions::unicode as unicode_ext;
use icu_locid::subtags::{Language, Region, Script, Variants};
use icu_locid::{LanguageIdentifier, Locale};
use writeable::{LengthHint, Writeable};
use zerovec::ule::*;

#[cfg(doc)]
use icu_locid::subtags::Variant;

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

/// Hint for what to prioritize during fallback when data is unavailable.
///
/// For example, if `"en-US"` is requested, but we have no data for that specific locale,
/// fallback may take us to `"en"` or `"und-US"` to check for data.
#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
#[non_exhaustive]
pub enum FallbackPriority {
    /// Prioritize the language. This is the default behavior.
    ///
    /// For example, `"en-US"` should go to `"en"` and then `"und"`.
    Language,
    /// Prioritize the region.
    ///
    /// For example, `"en-US"` should go to `"und-US"` and then `"und"`.
    Region,
}

impl FallbackPriority {
    /// Const-friendly version of [`Default::default`].
    pub const fn const_default() -> Self {
        Self::Language
    }
}

impl Default for FallbackPriority {
    fn default() -> Self {
        Self::const_default()
    }
}

/// Metadata statically associated with a particular [`ResourceKey`].
#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
#[non_exhaustive]
pub struct ResourceKeyMetadata {
    /// What to prioritize when fallbacking on this [`ResourceKey`].
    pub fallback_priority: FallbackPriority,
    /// A Unicode extension keyword to consider when loading data for this [`ResourceKey`].
    pub extension_key: Option<icu_locid::extensions::unicode::Key>,
}

impl ResourceKeyMetadata {
    /// Const-friendly version of [`Default::default`].
    pub const fn const_default() -> Self {
        Self {
            fallback_priority: FallbackPriority::const_default(),
            extension_key: None,
        }
    }

    /// Create a new [`ResourceKeyMetadata`] with the specified options.
    pub const fn from_fallback_priority_and_extension_key(
        fallback_priority: FallbackPriority,
        extension_key: Option<icu_locid::extensions::unicode::Key>,
    ) -> Self {
        // Note: We need this function because the struct is non-exhaustive.
        Self {
            fallback_priority,
            extension_key,
        }
    }
}

impl Default for ResourceKeyMetadata {
    fn default() -> Self {
        Self::const_default()
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
    metadata: ResourceKeyMetadata,
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

    /// Gets the metadata associated with this [`ResourceKey`].
    #[inline]
    pub const fn get_metadata(&self) -> ResourceKeyMetadata {
        self.metadata
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
            MetaOpen,
            MetaU,
            MetaUDash,
            MetaUDashB,
            MetaClose,
            MetaAfter,
        }
        use State::*;
        i = start;
        let mut state = Empty;
        let mut fallback_priority = FallbackPriority::const_default();
        let mut extension_key_first_byte = b'\0';
        let mut extension_key = None;
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
                (Version | MetaAfter, None) => {
                    return Ok(Self {
                        path,
                        hash: ResourceKeyHash::compute_from_str(path),
                        metadata: ResourceKeyMetadata {
                            fallback_priority,
                            extension_key,
                        },
                    })
                }

                (Version | MetaAfter, Some(b'[')) => MetaOpen,
                (MetaOpen, Some(b'R')) => {
                    fallback_priority = FallbackPriority::Region;
                    MetaClose
                }
                (MetaOpen, Some(b'u')) => MetaU,
                (MetaU, Some(b'-')) => MetaUDash,
                (MetaUDash, Some(b @ b'a'..=b'z')) => {
                    extension_key_first_byte = b;
                    MetaUDashB
                }
                (MetaUDashB, Some(b @ b'a'..=b'z')) => {
                    extension_key =
                        match unicode_ext::Key::from_bytes(&[extension_key_first_byte, b]) {
                            Ok(v) => Some(v),
                            Err(_) => unreachable!(),
                        };
                    MetaClose
                }
                (MetaClose, Some(b']')) => MetaAfter,

                (Empty, _) => return Err(("[a-zA-Z0-9_]", i)),
                (Body, _) => return Err(("[a-zA-z0-9_/@]", i)),
                (At, _) => return Err(("[0-9]", i)),
                (Version, _) => return Err(("[0-9\\[]", i)),
                (MetaOpen, _) => return Err(("[uR]", i)),
                (MetaU, _) => return Err(("[-]", i)),
                (MetaUDash, _) => return Err(("[a-z]", i)),
                (MetaUDashB, _) => return Err(("[a-z]", i)),
                (MetaClose, _) => return Err(("[\\]]", i)),
                (MetaAfter, _) => return Err(("[\\[]", i)),
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

#[test]
fn test_path_syntax() {
    // Valid keys:
    ResourceKey::construct_internal(tagged!("hello/world@1")).unwrap();
    ResourceKey::construct_internal(tagged!("hello/world/foo@1")).unwrap();
    ResourceKey::construct_internal(tagged!("hello/world@999")).unwrap();
    ResourceKey::construct_internal(tagged!("hello_world/foo@1")).unwrap();
    ResourceKey::construct_internal(tagged!("hello_458/world@1")).unwrap();
    ResourceKey::construct_internal(tagged!("hello_world@1")).unwrap();
    ResourceKey::construct_internal(tagged!("foo@1[R]")).unwrap();
    ResourceKey::construct_internal(tagged!("foo@1[u-ca]")).unwrap();
    ResourceKey::construct_internal(tagged!("foo@1[R][u-ca]")).unwrap();

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
        Err(("[0-9\\[]", concat!(leading_tag!(), "hello/world@1").len()))
    );

    // Invalid meta:
    assert_eq!(
        ResourceKey::construct_internal(tagged!("foo@1[U]")),
        Err(("[uR]", concat!(leading_tag!(), "foo@1[").len()))
    );
    assert_eq!(
        ResourceKey::construct_internal(tagged!("foo@1[uca]")),
        Err(("[-]", concat!(leading_tag!(), "foo@1[u").len()))
    );
    assert_eq!(
        ResourceKey::construct_internal(tagged!("foo@1[u-")),
        Err(("[a-z]", concat!(leading_tag!(), "foo@1[u-").len()))
    );
    assert_eq!(
        ResourceKey::construct_internal(tagged!("foo@1[u-caa]")),
        Err(("[\\]]", concat!(leading_tag!(), "foo@1[u-ca").len()))
    );
    assert_eq!(
        ResourceKey::construct_internal(tagged!("foo@1[R")),
        Err(("[\\]]", concat!(leading_tag!(), "foo@1[u").len()))
    );

    // Invalid characters:
    assert_eq!(
        ResourceKey::construct_internal(tagged!("你好/世界@1")),
        Err(("[a-zA-Z0-9_]", leading_tag!().len()))
    );

    // Invalid tag:
    assert_eq!(
        ResourceKey::construct_internal(concat!("hello/world@1", trailing_tag!()),),
        Err(("tag", 0))
    );
    assert_eq!(
        ResourceKey::construct_internal(concat!(leading_tag!(), "hello/world@1"),),
        Err(("tag", concat!(leading_tag!(), "hello/world@1").len()))
    );
    assert_eq!(
        ResourceKey::construct_internal("hello/world@1"),
        Err(("tag", 0))
    );
}

#[test]
fn test_metadata_parsing() {
    use icu_locid::extensions_unicode_key as key;
    assert_eq!(
        ResourceKey::construct_internal(tagged!("hello/world@1")).map(|k| k.get_metadata()),
        Ok(ResourceKeyMetadata {
            fallback_priority: FallbackPriority::Language,
            extension_key: None
        })
    );
    assert_eq!(
        ResourceKey::construct_internal(tagged!("hello/world@1[R]")).map(|k| k.get_metadata()),
        Ok(ResourceKeyMetadata {
            fallback_priority: FallbackPriority::Region,
            extension_key: None
        })
    );
    assert_eq!(
        ResourceKey::construct_internal(tagged!("hello/world@1[u-ca]")).map(|k| k.get_metadata()),
        Ok(ResourceKeyMetadata {
            fallback_priority: FallbackPriority::Language,
            extension_key: Some(key!("ca"))
        })
    );
    assert_eq!(
        ResourceKey::construct_internal(tagged!("hello/world@1[R][u-ca]"))
            .map(|k| k.get_metadata()),
        Ok(ResourceKeyMetadata {
            fallback_priority: FallbackPriority::Region,
            extension_key: Some(key!("ca"))
        })
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
                #[allow(clippy::panic)] // Const context
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
#[derive(PartialEq, Clone, Default, Eq, Hash)]
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

impl AsMut<ResourceOptions> for ResourceOptions {
    fn as_mut(&mut self) -> &mut ResourceOptions {
        self
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
            + if !self.keywords.is_empty() {
                self.keywords.write_len() + 3
            } else {
                LengthHint::exact(0)
            }
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
        Self {
            langid: locale.id,
            keywords: locale.extensions.unicode.keywords,
        }
    }
}

impl From<&Locale> for ResourceOptions {
    fn from(locale: &Locale) -> Self {
        // TODO(#1109): Implement proper vertical fallback
        Self {
            langid: locale.id.clone(),
            keywords: locale.extensions.unicode.keywords.clone(),
        }
    }
}

impl ResourceOptions {
    /// Compare this [`ResourceOptions`] with BCP-47 bytes.
    ///
    /// The return value is equivalent to what would happen if you first converted this
    /// [`ResourceOptions`] to a BCP-47 string and then performed a byte comparison.
    ///
    /// This function is case-sensitive and results in a *total order*, so it is appropriate for
    /// binary search. The only argument producing [`Ordering::Equal`] is `self.to_string()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::ResourceOptions;
    /// use icu_locid::Locale;
    /// use std::cmp::Ordering;
    ///
    /// let bcp47_strings: &[&str] = &[
    ///     "ca-ES",
    ///     "ca-ES-u-ca-buddhist",
    ///     "ca-ES-valencia",
    ///     "pl-Latn-PL",
    ///     "und",
    ///     "und-fonipa",
    ///     "und-u-ca-hebrew",
    ///     "und-u-ca-japanese",
    ///     "zh",
    /// ];
    ///
    /// for ab in bcp47_strings.windows(2) {
    ///     let a = ab[0];
    ///     let b = ab[1];
    ///     assert!(a.cmp(b) == Ordering::Less);
    ///     let a_loc: ResourceOptions = a.parse::<Locale>().unwrap().into();
    ///     assert_eq!(a, a_loc.to_string());
    ///     assert!(a_loc.strict_cmp(a.as_bytes()) == Ordering::Equal, "{} == {}", a, a);
    ///     assert!(a_loc.strict_cmp(b.as_bytes()) == Ordering::Less, "{} < {}", a, b);
    ///     let b_loc: ResourceOptions = b.parse::<Locale>().unwrap().into();
    ///     assert_eq!(b, b_loc.to_string());
    ///     assert!(b_loc.strict_cmp(b.as_bytes()) == Ordering::Equal, "{} == {}", b, b);
    ///     assert!(b_loc.strict_cmp(a.as_bytes()) == Ordering::Greater, "{} > {}", b, a);
    /// }
    /// ```
    pub fn strict_cmp(&self, other: &[u8]) -> Ordering {
        let subtags = other.split(|b| *b == b'-');
        let mut subtag_result = self.langid.strict_cmp_iter(subtags);
        if self.has_unicode_ext() {
            let mut subtags = match subtag_result {
                SubtagOrderingResult::Subtags(s) => s,
                SubtagOrderingResult::Ordering(o) => return o,
            };
            match subtags.next() {
                Some(b"u") => (),
                Some(s) => return s.cmp(b"u").reverse(),
                None => return Ordering::Greater,
            }
            subtag_result = self.keywords.strict_cmp_iter(subtags);
        }
        subtag_result.end()
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

    /// Returns whether this [`ResourceOptions`] has all empty fields (no components).
    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }

    /// Returns whether the [`LanguageIdentifier`] associated with this request is `und`.
    ///
    /// Note that this only checks the language identifier; extension keywords may also be set.
    /// To check the entire `ResourceOptions`, use [`ResourceOptions::is_empty()`].
    pub fn is_langid_und(&self) -> bool {
        self.langid == LanguageIdentifier::UND
    }

    /// Gets the [`LanguageIdentifier`] for this [`ResourceOptions`].
    ///
    /// This may allocate memory if there are variant subtags. If you need only the language,
    /// script, and/or region subtag, use the specific getters for those subtags:
    ///
    /// - [`ResourceOptions::language()`]
    /// - [`ResourceOptions::script()`]
    /// - [`ResourceOptions::region()`]
    ///
    /// If you have ownership over the `ResourceOptions`, use [`ResourceOptions::into_locale()`]
    /// and then access the `id` field.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::langid;
    /// use icu_provider::prelude::*;
    ///
    /// const FOO_BAR: ResourceKey = icu_provider::resource_key!("foo/bar@1");
    ///
    /// let req_no_langid = DataRequest {
    ///     options: ResourceOptions::default(),
    ///     metadata: Default::default(),
    /// };
    ///
    /// let req_with_langid = DataRequest {
    ///     options: langid!("ar-EG").into(),
    ///     metadata: Default::default(),
    /// };
    ///
    /// assert_eq!(req_no_langid.options.get_langid(), langid!("und"));
    /// assert_eq!(req_with_langid.options.get_langid(), langid!("ar-EG"));
    /// ```
    pub fn get_langid(&self) -> LanguageIdentifier {
        self.langid.clone()
    }

    /// Overrides the entire [`LanguageIdentifier`] portion of this [`ResourceOptions`].
    #[inline]
    pub fn set_langid(&mut self, lid: LanguageIdentifier) {
        self.langid = lid;
    }

    /// Converts this [`ResourceOptions`] into a [`Locale`].
    ///
    /// See also [`ResourceOptions::get_langid()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::{langid, subtags_language as language, subtags_region as region, Locale};
    /// use icu_provider::prelude::*;
    ///
    /// let locale: Locale = "it-IT-u-ca-coptic".parse().expect("Valid BCP-47");
    /// let options: ResourceOptions = locale.into();
    ///
    /// assert_eq!(options.to_string(), "it-IT-u-ca-coptic");
    /// assert_eq!(options.get_langid(), langid!("it-IT"));
    /// assert_eq!(options.language(), language!("it"));
    /// assert_eq!(options.script(), None);
    /// assert_eq!(options.region(), Some(region!("IT")));
    ///
    /// let locale = options.into_locale();
    /// assert_eq!(locale.to_string(), "it-IT-u-ca-coptic");
    /// ```
    pub fn into_locale(self) -> Locale {
        let mut loc = Locale {
            id: self.langid,
            ..Default::default()
        };
        loc.extensions.unicode.keywords = self.keywords;
        loc
    }

    /// Returns the [`Language`] for this [`ResourceOptions`].
    #[inline]
    pub fn language(&self) -> Language {
        self.langid.language
    }

    /// Returns the [`Language`] for this [`ResourceOptions`].
    #[inline]
    pub fn set_language(&mut self, language: Language) {
        self.langid.language = language;
    }

    /// Returns the [`Script`] for this [`ResourceOptions`].
    #[inline]
    pub fn script(&self) -> Option<Script> {
        self.langid.script
    }

    /// Sets the [`Script`] for this [`ResourceOptions`].
    #[inline]
    pub fn set_script(&mut self, script: Option<Script>) {
        self.langid.script = script;
    }

    /// Returns the [`Region`] for this [`ResourceOptions`].
    #[inline]
    pub fn region(&self) -> Option<Region> {
        self.langid.region
    }

    /// Sets the [`Region`] for this [`ResourceOptions`].
    #[inline]
    pub fn set_region(&mut self, region: Option<Region>) {
        self.langid.region = region;
    }

    /// Returns whether there are any [`Variant`] subtags in this [`ResourceOptions`].
    #[inline]
    pub fn has_variants(&self) -> bool {
        !self.langid.variants.is_empty()
    }

    #[inline]
    pub fn set_variants(&mut self, variants: Variants) {
        self.langid.variants = variants;
    }

    /// Removes all [`Variant`] subtags in this [`ResourceOptions`].
    #[inline]
    pub fn clear_variants(&mut self) -> Variants {
        self.langid.variants.clear()
    }

    /// Gets the value of the specified Unicode extension keyword for this [`ResourceOptions`].
    #[inline]
    pub fn get_unicode_ext(&self, key: &unicode_ext::Key) -> Option<unicode_ext::Value> {
        self.keywords.get(key).cloned()
    }

    /// Returns whether there are any Unicode extension keywords in this [`ResourceOptions`].
    #[inline]
    pub fn has_unicode_ext(&self) -> bool {
        !self.keywords.is_empty()
    }

    /// Returns whether a specific Unicode extension keyword is present in this [`ResourceOptions`].
    #[inline]
    pub fn contains_unicode_ext(&self, key: &unicode_ext::Key) -> bool {
        self.keywords.contains_key(key)
    }

    /// Returns whether this [`ResourceOptions`] contains a Unicode extension keyword
    /// with the specified key and value.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::{extensions_unicode_key as key, extensions_unicode_value as value, Locale};
    /// use icu_provider::prelude::*;
    ///
    /// let locale: Locale = "it-IT-u-ca-coptic".parse().expect("Valid BCP-47");
    /// let options: ResourceOptions = locale.into();
    ///
    /// assert_eq!(options.get_unicode_ext(&key!("hc")), None);
    /// assert_eq!(
    ///     options.get_unicode_ext(&key!("ca")),
    ///     Some(value!("coptic"))
    /// );
    /// assert!(options.matches_unicode_ext(&key!("ca"), &value!("coptic"),));
    /// ```
    #[inline]
    pub fn matches_unicode_ext(&self, key: &unicode_ext::Key, value: &unicode_ext::Value) -> bool {
        self.keywords.get(key) == Some(value)
    }

    /// Sets the value for a specific Unicode extension keyword on this [`ResourceOptions`].
    #[inline]
    pub fn set_unicode_ext(
        &mut self,
        key: unicode_ext::Key,
        value: unicode_ext::Value,
    ) -> Option<unicode_ext::Value> {
        self.keywords.set(key, value)
    }

    /// Removes a specific Unicode extension keyword from this [`ResourceOptions`], returning
    /// the value if it was present.
    #[inline]
    pub fn remove_unicode_ext(&mut self, key: &unicode_ext::Key) -> Option<unicode_ext::Value> {
        self.keywords.remove(key)
    }

    /// Retains a subset of keywords as specified by the predicate function.
    #[inline]
    pub fn retain_unicode_ext<F>(&mut self, predicate: F)
    where
        F: FnMut(&unicode_ext::Key) -> bool,
    {
        self.keywords.retain_by_key(predicate)
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
