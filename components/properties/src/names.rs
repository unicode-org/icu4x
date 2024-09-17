// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::props::*;
use crate::provider::names::*;
use core::marker::PhantomData;
use icu_collections::codepointtrie::TrieValue;
use icu_provider::prelude::*;
use zerovec::ule::VarULE;

/// Private marker type for PropertyValueNameToEnumMapper
/// to work for all properties at once
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) struct ErasedNameToEnumMapV1Marker;
impl DynamicDataMarker for ErasedNameToEnumMapV1Marker {
    type DataStruct = PropertyValueNameToEnumMapV1<'static>;
}

/// A struct capable of looking up a property value from a string name.
/// Access its data by calling [`Self::as_borrowed()`] and using the methods on
/// [`PropertyValueNameToEnumMapperBorrowed`].
///
/// The name can be a short name (`Lu`), a long name(`Uppercase_Letter`),
/// or an alias.
///
/// Property names can be looked up using "strict" matching (looking for a name
/// that matches exactly), or "loose matching", where the name is allowed to deviate
/// in terms of ASCII casing, whitespace, underscores, and hyphens.
///
/// # Example
///
/// ```
/// use icu::properties::GeneralCategory;
///
/// let lookup = GeneralCategory::name_to_enum_mapper();
/// // short name for value
/// assert_eq!(
///     lookup.get_strict("Lu"),
///     Some(GeneralCategory::UppercaseLetter)
/// );
/// assert_eq!(
///     lookup.get_strict("Pd"),
///     Some(GeneralCategory::DashPunctuation)
/// );
/// // long name for value
/// assert_eq!(
///     lookup.get_strict("Uppercase_Letter"),
///     Some(GeneralCategory::UppercaseLetter)
/// );
/// assert_eq!(
///     lookup.get_strict("Dash_Punctuation"),
///     Some(GeneralCategory::DashPunctuation)
/// );
/// // name has incorrect casing
/// assert_eq!(lookup.get_strict("dashpunctuation"), None);
/// // loose matching of name
/// assert_eq!(
///     lookup.get_loose("dash-punctuation"),
///     Some(GeneralCategory::DashPunctuation)
/// );
/// // fake property
/// assert_eq!(lookup.get_strict("Animated_Gif"), None);
/// ```
#[derive(Debug)]
pub struct PropertyValueNameToEnumMapper<T> {
    map: DataPayload<ErasedNameToEnumMapV1Marker>,
    markers: PhantomData<fn() -> T>,
}

/// A borrowed wrapper around property value name-to-enum data, returned by
/// [`PropertyValueNameToEnumMapper::as_borrowed()`]. More efficient to query.
#[derive(Debug, Copy, Clone)]
pub struct PropertyValueNameToEnumMapperBorrowed<'a, T> {
    map: &'a PropertyValueNameToEnumMapV1<'a>,
    markers: PhantomData<fn() -> T>,
}

impl<T: TrieValue> PropertyValueNameToEnumMapper<T> {
    /// Construct a borrowed version of this type that can be queried.
    ///
    /// This avoids a potential small underlying cost per API call (like `get_strict()`) by consolidating it
    /// up front.
    #[inline]
    pub fn as_borrowed(&self) -> PropertyValueNameToEnumMapperBorrowed<'_, T> {
        PropertyValueNameToEnumMapperBorrowed {
            map: self.map.get(),
            markers: PhantomData,
        }
    }

    pub(crate) fn from_data<M>(data: DataPayload<M>) -> Self
    where
        M: DynamicDataMarker<DataStruct = PropertyValueNameToEnumMapV1<'static>>,
    {
        Self {
            map: data.cast(),
            markers: PhantomData,
        }
    }

    #[doc(hidden)] // used by FFI code
    pub fn erase(self) -> PropertyValueNameToEnumMapper<u16> {
        PropertyValueNameToEnumMapper {
            map: self.map.cast(),
            markers: PhantomData,
        }
    }
}

impl<T: TrieValue> PropertyValueNameToEnumMapperBorrowed<'_, T> {
    /// Get the property value as a u16, doing a strict search looking for
    /// names that match exactly
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::GeneralCategory;
    ///
    /// let lookup = GeneralCategory::name_to_enum_mapper();
    /// assert_eq!(
    ///     lookup.get_strict_u16("Lu"),
    ///     Some(GeneralCategory::UppercaseLetter as u16)
    /// );
    /// assert_eq!(
    ///     lookup.get_strict_u16("Uppercase_Letter"),
    ///     Some(GeneralCategory::UppercaseLetter as u16)
    /// );
    /// // does not do loose matching
    /// assert_eq!(lookup.get_strict_u16("UppercaseLetter"), None);
    /// ```
    #[inline]
    pub fn get_strict_u16(&self, name: &str) -> Option<u16> {
        get_strict_u16(self.map, name)
    }

    /// Get the property value as a `T`, doing a strict search looking for
    /// names that match exactly
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::GeneralCategory;
    ///
    /// let lookup = GeneralCategory::name_to_enum_mapper();
    /// assert_eq!(
    ///     lookup.get_strict("Lu"),
    ///     Some(GeneralCategory::UppercaseLetter)
    /// );
    /// assert_eq!(
    ///     lookup.get_strict("Uppercase_Letter"),
    ///     Some(GeneralCategory::UppercaseLetter)
    /// );
    /// // does not do loose matching
    /// assert_eq!(lookup.get_strict("UppercaseLetter"), None);
    /// ```
    #[inline]
    pub fn get_strict(&self, name: &str) -> Option<T> {
        T::try_from_u32(self.get_strict_u16(name)? as u32).ok()
    }

    /// Get the property value as a u16, doing a loose search looking for
    /// names that match case-insensitively, ignoring ASCII hyphens, underscores, and
    /// whitespaces.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::GeneralCategory;
    ///
    /// let lookup = GeneralCategory::name_to_enum_mapper();
    /// assert_eq!(
    ///     lookup.get_loose_u16("Lu"),
    ///     Some(GeneralCategory::UppercaseLetter as u16)
    /// );
    /// assert_eq!(
    ///     lookup.get_loose_u16("Uppercase_Letter"),
    ///     Some(GeneralCategory::UppercaseLetter as u16)
    /// );
    /// // does do loose matching
    /// assert_eq!(
    ///     lookup.get_loose_u16("UppercaseLetter"),
    ///     Some(GeneralCategory::UppercaseLetter as u16)
    /// );
    /// ```
    #[inline]
    pub fn get_loose_u16(&self, name: &str) -> Option<u16> {
        get_loose_u16(self.map, name)
    }

    /// Get the property value as a `T`, doing a loose search looking for
    /// names that match case-insensitively, ignoring ASCII hyphens, underscores, and
    /// whitespaces.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::GeneralCategory;
    ///
    /// let lookup = GeneralCategory::name_to_enum_mapper();
    /// assert_eq!(
    ///     lookup.get_loose("Lu"),
    ///     Some(GeneralCategory::UppercaseLetter)
    /// );
    /// assert_eq!(
    ///     lookup.get_loose("Uppercase_Letter"),
    ///     Some(GeneralCategory::UppercaseLetter)
    /// );
    /// // does do loose matching
    /// assert_eq!(
    ///     lookup.get_loose("UppercaseLetter"),
    ///     Some(GeneralCategory::UppercaseLetter)
    /// );
    /// ```
    #[inline]
    pub fn get_loose(&self, name: &str) -> Option<T> {
        T::try_from_u32(self.get_loose_u16(name)? as u32).ok()
    }
}

impl<T: TrieValue> PropertyValueNameToEnumMapperBorrowed<'static, T> {
    /// Cheaply converts a [`PropertyValueNameToEnumMapperBorrowed<'static>`] into a [`PropertyValueNameToEnumMapper`].
    ///
    /// Note: Due to branching and indirection, using [`PropertyValueNameToEnumMapper`] might inhibit some
    /// compile-time optimizations that are possible with [`PropertyValueNameToEnumMapperBorrowed`].
    pub const fn static_to_owned(self) -> PropertyValueNameToEnumMapper<T> {
        PropertyValueNameToEnumMapper {
            map: DataPayload::from_static_ref(self.map),
            markers: PhantomData,
        }
    }
}

/// Avoid monomorphizing multiple copies of this function
fn get_strict_u16(payload: &PropertyValueNameToEnumMapV1<'_>, name: &str) -> Option<u16> {
    // NormalizedPropertyName has no invariants so this should be free, but
    // avoid introducing a panic regardless
    let name = NormalizedPropertyNameStr::parse_byte_slice(name.as_bytes()).ok()?;
    payload.map.get_copied(name)
}

/// Avoid monomorphizing multiple copies of this function
fn get_loose_u16(payload: &PropertyValueNameToEnumMapV1<'_>, name: &str) -> Option<u16> {
    // NormalizedPropertyName has no invariants so this should be free, but
    // avoid introducing a panic regardless
    let name = NormalizedPropertyNameStr::parse_byte_slice(name.as_bytes()).ok()?;
    payload.map.get_copied_by(|p| p.cmp_loose(name))
}

/// Private marker type for PropertyEnumToValueNameSparseMapper
/// to work for all properties at once
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) struct ErasedEnumToValueNameSparseMapV1Marker;
impl DynamicDataMarker for ErasedEnumToValueNameSparseMapV1Marker {
    type DataStruct = PropertyEnumToValueNameSparseMapV1<'static>;
}

/// A struct capable of looking up a property name from a value
/// Access its data by calling [`Self::as_borrowed()`] and using the methods on
/// [`PropertyEnumToValueNameSparseMapperBorrowed`].
///
/// This mapper is used for properties with sparse values, like [`CanonicalCombiningClass`].
/// It may be obtained using methods like [`CanonicalCombiningClass::get_enum_to_long_name_mapper()`].
///
/// The name returned may be a short (`"KV"`) or long (`"Kana_Voicing"`) name, depending
/// on the constructor used.
///
/// # Example
///
/// ```
/// use icu::properties::CanonicalCombiningClass;
///
/// let lookup = CanonicalCombiningClass::enum_to_long_name_mapper();
/// assert_eq!(
///     lookup.get(CanonicalCombiningClass::KanaVoicing),
///     Some("Kana_Voicing")
/// );
/// assert_eq!(
///     lookup.get(CanonicalCombiningClass::AboveLeft),
///     Some("Above_Left")
/// );
/// ```
#[derive(Debug)]
pub struct PropertyEnumToValueNameSparseMapper<T> {
    map: DataPayload<ErasedEnumToValueNameSparseMapV1Marker>,
    markers: PhantomData<fn(T) -> ()>,
}

/// A borrowed wrapper around property value name-to-enum data, returned by
/// [`PropertyEnumToValueNameSparseMapper::as_borrowed()`]. More efficient to query.
#[derive(Debug, Copy, Clone)]
pub struct PropertyEnumToValueNameSparseMapperBorrowed<'a, T> {
    map: &'a PropertyEnumToValueNameSparseMapV1<'a>,
    markers: PhantomData<fn(T) -> ()>,
}

impl<T: TrieValue> PropertyEnumToValueNameSparseMapper<T> {
    /// Construct a borrowed version of this type that can be queried.
    ///
    /// This avoids a potential small underlying cost per API call (like `get_static()`) by consolidating it
    /// up front.
    #[inline]
    pub fn as_borrowed(&self) -> PropertyEnumToValueNameSparseMapperBorrowed<'_, T> {
        PropertyEnumToValueNameSparseMapperBorrowed {
            map: self.map.get(),
            markers: PhantomData,
        }
    }

    /// Construct a new one from loaded data
    ///
    /// Typically it is preferable to use methods on individual property value types
    /// (like [`Script::TBD()`]) instead.
    pub(crate) fn from_data<M>(data: DataPayload<M>) -> Self
    where
        M: DynamicDataMarker<DataStruct = PropertyEnumToValueNameSparseMapV1<'static>>,
    {
        Self {
            map: data.cast(),
            markers: PhantomData,
        }
    }
}

impl<T: TrieValue> PropertyEnumToValueNameSparseMapperBorrowed<'_, T> {
    /// Get the property name given a value
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu::properties::CanonicalCombiningClass;
    ///
    /// let lookup = CanonicalCombiningClass::enum_to_long_name_mapper();
    /// assert_eq!(
    ///     lookup.get(CanonicalCombiningClass::KanaVoicing),
    ///     Some("Kana_Voicing")
    /// );
    /// assert_eq!(
    ///     lookup.get(CanonicalCombiningClass::AboveLeft),
    ///     Some("Above_Left")
    /// );
    /// ```
    #[inline]
    pub fn get(&self, property: T) -> Option<&str> {
        let prop = u16::try_from(property.to_u32()).ok()?;
        self.map.map.get(&prop)
    }
}

impl<T: TrieValue> PropertyEnumToValueNameSparseMapperBorrowed<'static, T> {
    /// Cheaply converts a [`PropertyEnumToValueNameSparseMapperBorrowed<'static>`] into a [`PropertyEnumToValueNameSparseMapper`].
    ///
    /// Note: Due to branching and indirection, using [`PropertyEnumToValueNameSparseMapper`] might inhibit some
    /// compile-time optimizations that are possible with [`PropertyEnumToValueNameSparseMapperBorrowed`].
    pub const fn static_to_owned(self) -> PropertyEnumToValueNameSparseMapper<T> {
        PropertyEnumToValueNameSparseMapper {
            map: DataPayload::from_static_ref(self.map),
            markers: PhantomData,
        }
    }
}

/// Private marker type for PropertyEnumToValueNameLinearMapper
/// to work for all properties at once
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) struct ErasedEnumToValueNameLinearMapV1Marker;
impl DynamicDataMarker for ErasedEnumToValueNameLinearMapV1Marker {
    type DataStruct = PropertyEnumToValueNameLinearMapV1<'static>;
}

/// A struct capable of looking up a property name from a value
/// Access its data by calling [`Self::as_borrowed()`] and using the methods on
/// [`PropertyEnumToValueNameLinearMapperBorrowed`].
///
/// This mapper is used for properties with sequential values, like [`GeneralCategory`].
/// It may be obtained using methods like [`GeneralCategory::get_enum_to_long_name_mapper()`].
///
/// The name returned may be a short (`"Lu"`) or long (`"Uppercase_Letter"`) name, depending
/// on the constructor used.
///
/// # Example
///
/// ```
/// use icu::properties::GeneralCategory;
///
/// let lookup = GeneralCategory::enum_to_long_name_mapper();
/// assert_eq!(
///     lookup.get(GeneralCategory::UppercaseLetter),
///     Some("Uppercase_Letter")
/// );
/// assert_eq!(
///     lookup.get(GeneralCategory::DashPunctuation),
///     Some("Dash_Punctuation")
/// );
/// ```
#[derive(Debug)]
pub struct PropertyEnumToValueNameLinearMapper<T> {
    map: DataPayload<ErasedEnumToValueNameLinearMapV1Marker>,
    markers: PhantomData<fn(T) -> ()>,
}

/// A borrowed wrapper around property value name-to-enum data, returned by
/// [`PropertyEnumToValueNameLinearMapper::as_borrowed()`]. More efficient to query.
#[derive(Debug, Copy, Clone)]
pub struct PropertyEnumToValueNameLinearMapperBorrowed<'a, T> {
    map: &'a PropertyEnumToValueNameLinearMapV1<'a>,
    markers: PhantomData<fn(T) -> ()>,
}

impl<T: TrieValue> PropertyEnumToValueNameLinearMapper<T> {
    /// Construct a borrowed version of this type that can be queried.
    ///
    /// This avoids a potential small underlying cost per API call (like `get_static()`) by consolidating it
    /// up front.
    #[inline]
    pub fn as_borrowed(&self) -> PropertyEnumToValueNameLinearMapperBorrowed<'_, T> {
        PropertyEnumToValueNameLinearMapperBorrowed {
            map: self.map.get(),
            markers: PhantomData,
        }
    }

    /// Construct a new one from loaded data
    ///
    /// Typically it is preferable to use methods on individual property value types
    /// (like [`Script::TBD()`]) instead.
    pub(crate) fn from_data<M>(data: DataPayload<M>) -> Self
    where
        M: DynamicDataMarker<DataStruct = PropertyEnumToValueNameLinearMapV1<'static>>,
    {
        Self {
            map: data.cast(),
            markers: PhantomData,
        }
    }
}

impl<T: TrieValue> PropertyEnumToValueNameLinearMapperBorrowed<'_, T> {
    /// Get the property name given a value
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu::properties::GeneralCategory;
    ///
    /// let lookup = GeneralCategory::enum_to_short_name_mapper();
    /// assert_eq!(lookup.get(GeneralCategory::UppercaseLetter), Some("Lu"));
    /// assert_eq!(lookup.get(GeneralCategory::DashPunctuation), Some("Pd"));
    /// ```
    #[inline]
    pub fn get(&self, property: T) -> Option<&str> {
        let prop = usize::try_from(property.to_u32()).ok()?;
        self.map.map.get(prop).filter(|x| !x.is_empty())
    }
}

impl<T: TrieValue> PropertyEnumToValueNameLinearMapperBorrowed<'static, T> {
    /// Cheaply converts a [`PropertyEnumToValueNameLinearMapperBorrowed<'static>`] into a [`PropertyEnumToValueNameLinearMapper`].
    ///
    /// Note: Due to branching and indirection, using [`PropertyEnumToValueNameLinearMapper`] might inhibit some
    /// compile-time optimizations that are possible with [`PropertyEnumToValueNameLinearMapperBorrowed`].
    pub const fn static_to_owned(self) -> PropertyEnumToValueNameLinearMapper<T> {
        PropertyEnumToValueNameLinearMapper {
            map: DataPayload::from_static_ref(self.map),
            markers: PhantomData,
        }
    }
}

/// Private marker type for PropertyScriptToIcuScriptMapper
/// to work for all properties at once
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) struct ErasedEnumToValueNameLinearTiny4MapV1Marker;
impl DynamicDataMarker for ErasedEnumToValueNameLinearTiny4MapV1Marker {
    type DataStruct = PropertyScriptToIcuScriptMapV1<'static>;
}

/// A struct capable of looking up a property name from a value
/// Access its data by calling [`Self::as_borrowed()`] and using the methods on
/// [`PropertyScriptToIcuScriptMapperBorrowed`].
///
/// This mapper is used for the [`Script`] property.
/// It may be obtained using methods like [`Script::get_enum_to_icu_script_mapper()`].
///
/// # Example
///
/// ```
/// use icu::properties::Script;
/// use icu::locale::subtags::script;
///
/// let lookup = Script::enum_to_icu_script_mapper();
/// assert_eq!(lookup.get(Script::Brahmi), Some(script!("Brah")));
/// assert_eq!(lookup.get(Script::Hangul), Some(script!("Hang")));
/// ```
#[derive(Debug)]
pub struct PropertyScriptToIcuScriptMapper<T> {
    map: DataPayload<ErasedEnumToValueNameLinearTiny4MapV1Marker>,
    markers: PhantomData<fn(T) -> ()>,
}

/// A borrowed wrapper around property value name-to-enum data, returned by
/// [`PropertyScriptToIcuScriptMapper::as_borrowed()`]. More efficient to query.
#[derive(Debug, Copy, Clone)]
pub struct PropertyScriptToIcuScriptMapperBorrowed<'a, T> {
    map: &'a PropertyScriptToIcuScriptMapV1<'a>,
    markers: PhantomData<fn(T) -> ()>,
}

impl<T: TrieValue> PropertyScriptToIcuScriptMapper<T> {
    /// Construct a borrowed version of this type that can be queried.
    ///
    /// This avoids a potential small underlying cost per API call (like `get_static()`) by consolidating it
    /// up front.
    #[inline]
    pub fn as_borrowed(&self) -> PropertyScriptToIcuScriptMapperBorrowed<'_, T> {
        PropertyScriptToIcuScriptMapperBorrowed {
            map: self.map.get(),
            markers: PhantomData,
        }
    }

    /// Construct a new one from loaded data
    ///
    /// Typically it is preferable to use methods on individual property value types
    /// (like [`Script::TBD()`]) instead.
    pub(crate) fn from_data<M>(data: DataPayload<M>) -> Self
    where
        M: DynamicDataMarker<DataStruct = PropertyScriptToIcuScriptMapV1<'static>>,
    {
        Self {
            map: data.cast(),
            markers: PhantomData,
        }
    }
}

impl<T: TrieValue> PropertyScriptToIcuScriptMapperBorrowed<'_, T> {
    /// Get the property name given a value
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu::properties::Script;
    /// use icu::locale::subtags::script;
    ///
    /// let lookup = Script::enum_to_icu_script_mapper();
    /// assert_eq!(lookup.get(Script::Brahmi), Some(script!("Brah")));
    /// assert_eq!(lookup.get(Script::Hangul), Some(script!("Hang")));
    /// ```
    #[inline]
    pub fn get(&self, property: T) -> Option<icu_locale_core::subtags::Script> {
        let prop = usize::try_from(property.to_u32()).ok()?;
        self.map.map.get(prop).and_then(|o| o.0)
    }
}

impl<T: TrieValue> PropertyScriptToIcuScriptMapperBorrowed<'static, T> {
    /// Cheaply converts a [`PropertyScriptToIcuScriptMapperBorrowed<'static>`] into a [`PropertyScriptToIcuScriptMapper`].
    ///
    /// Note: Due to branching and indirection, using [`PropertyScriptToIcuScriptMapper`] might inhibit some
    /// compile-time optimizations that are possible with [`PropertyScriptToIcuScriptMapperBorrowed`].
    pub const fn static_to_owned(self) -> PropertyScriptToIcuScriptMapper<T> {
        PropertyScriptToIcuScriptMapper {
            map: DataPayload::from_static_ref(self.map),
            markers: PhantomData,
        }
    }
}

macro_rules! impl_value_getter {
    (
        // the marker type for names lookup (name_to_enum, enum_to_short_name, enum_to_long_name)
        markers: $marker_n2e:ident / $singleton_n2e:ident $(, $marker_e2sn:ident / $singleton_e2sn:ident, $marker_e2ln:ident / $singleton_e2ln:ident)?;
        impl $ty:ident {
            $(#[$attr_n2e:meta])*
            $vis_n2e:vis fn $name_n2e:ident() / $cname_n2e:ident();
            $(

                $(#[$attr_e2sn:meta])*
                $vis_e2sn:vis fn $name_e2sn:ident() / $cname_e2sn:ident() -> $mapper_e2sn:ident / $mapper_e2snb:ident;
                $(#[$attr_e2ln:meta])*
                $vis_e2ln:vis fn $name_e2ln:ident() / $cname_e2ln:ident() -> $mapper_e2ln:ident / $mapper_e2lnb:ident;
            )?
        }
    ) => {
        impl $ty {
            $(#[$attr_n2e])*
            #[cfg(feature = "compiled_data")]
            $vis_n2e const fn $cname_n2e() -> PropertyValueNameToEnumMapperBorrowed<'static, $ty> {
                PropertyValueNameToEnumMapperBorrowed {
                    map: crate::provider::Baked::$singleton_n2e,
                    markers: PhantomData,
                }
            }

            #[doc = concat!("A version of [`", stringify!($ty), "::", stringify!($cname_n2e), "()`] that uses custom data provided by a [`DataProvider`].")]
            ///
            /// [📚 Help choosing a constructor](icu_provider::constructors)
            $vis_n2e fn $name_n2e(
                provider: &(impl DataProvider<$marker_n2e> + ?Sized)
            ) -> Result<PropertyValueNameToEnumMapper<$ty>, DataError> {
                Ok(PropertyValueNameToEnumMapper::from_data(provider.load(Default::default())?.payload))
            }

            $(
                $(#[$attr_e2sn])*
                #[cfg(feature = "compiled_data")]
                $vis_e2sn const fn $cname_e2sn() -> $mapper_e2snb<'static, $ty> {
                    $mapper_e2snb {
                        map: crate::provider::Baked::$singleton_e2sn,
                        markers: PhantomData,
                    }
                }

                #[doc = concat!("A version of [`", stringify!($ty), "::", stringify!($cname_e2sn), "()`] that uses custom data provided by a [`DataProvider`].")]
                ///
                /// [📚 Help choosing a constructor](icu_provider::constructors)
                $vis_e2sn fn $name_e2sn(
                    provider: &(impl DataProvider<$marker_e2sn> + ?Sized)
                ) -> Result<$mapper_e2sn<$ty>, DataError> {
                    Ok($mapper_e2sn::from_data(provider.load(Default::default())?.payload))
                }

                $(#[$attr_e2ln])*
                #[cfg(feature = "compiled_data")]
                $vis_e2ln const fn $cname_e2ln() -> $mapper_e2lnb<'static, $ty> {
                    $mapper_e2lnb {
                        map: crate::provider::Baked::$singleton_e2ln,
                        markers: PhantomData,
                    }
                }

                #[doc = concat!("A version of [`", stringify!($ty), "::", stringify!($cname_e2ln), "()`] that uses custom data provided by a [`DataProvider`].")]
                ///
                /// [📚 Help choosing a constructor](icu_provider::constructors)
                $vis_e2ln fn $name_e2ln(
                    provider: &(impl DataProvider<$marker_e2ln> + ?Sized)
                ) -> Result<$mapper_e2ln<$ty>, DataError> {
                    Ok($mapper_e2ln::from_data(provider.load(Default::default())?.payload))
                }
            )?
        }
    }
}

impl_value_getter! {
    markers: BidiClassNameToValueV1Marker / SINGLETON_BIDI_CLASS_NAME_TO_VALUE_V1_MARKER, BidiClassValueToShortNameV1Marker / SINGLETON_BIDI_CLASS_VALUE_TO_SHORT_NAME_V1_MARKER, BidiClassValueToLongNameV1Marker / SINGLETON_BIDI_CLASS_VALUE_TO_LONG_NAME_V1_MARKER;
    impl BidiClass {
        /// Return a [`PropertyValueNameToEnumMapper`], capable of looking up values
        /// from strings for the `Bidi_Class` enumerated property
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::BidiClass;
        ///
        /// let lookup = BidiClass::name_to_enum_mapper();
        /// // short name for value
        /// assert_eq!(lookup.get_strict("AN"), Some(BidiClass::ArabicNumber));
        /// assert_eq!(lookup.get_strict("NSM"), Some(BidiClass::NonspacingMark));
        /// // long name for value
        /// assert_eq!(lookup.get_strict("Arabic_Number"), Some(BidiClass::ArabicNumber));
        /// assert_eq!(lookup.get_strict("Nonspacing_Mark"), Some(BidiClass::NonspacingMark));
        /// // name has incorrect casing
        /// assert_eq!(lookup.get_strict("arabicnumber"), None);
        /// // loose matching of name
        /// assert_eq!(lookup.get_loose("arabicnumber"), Some(BidiClass::ArabicNumber));
        /// // fake property
        /// assert_eq!(lookup.get_strict("Upside_Down_Vertical_Backwards_Mirrored"), None);
        /// ```
        pub fn get_name_to_enum_mapper() / name_to_enum_mapper();
        /// Return a [`PropertyEnumToValueNameLinearMapper`], capable of looking up short names
        /// for values of the `Bidi_Class` enumerated property
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::BidiClass;
        ///
        /// let lookup = BidiClass::enum_to_short_name_mapper();
        /// assert_eq!(lookup.get(BidiClass::ArabicNumber), Some("AN"));
        /// assert_eq!(lookup.get(BidiClass::NonspacingMark), Some("NSM"));
        /// ```
        pub fn get_enum_to_short_name_mapper() / enum_to_short_name_mapper() -> PropertyEnumToValueNameLinearMapper / PropertyEnumToValueNameLinearMapperBorrowed;
        /// Return a [`PropertyEnumToValueNameLinearMapper`], capable of looking up long names
        /// for values of the `Bidi_Class` enumerated property
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::BidiClass;
        ///
        /// let lookup = BidiClass::enum_to_long_name_mapper();
        /// assert_eq!(lookup.get(BidiClass::ArabicNumber), Some("Arabic_Number"));
        /// assert_eq!(lookup.get(BidiClass::NonspacingMark), Some("Nonspacing_Mark"));
        /// ```
        pub fn get_enum_to_long_name_mapper() / enum_to_long_name_mapper() -> PropertyEnumToValueNameLinearMapper / PropertyEnumToValueNameLinearMapperBorrowed;
    }
}

impl_value_getter! {
    markers: GeneralCategoryNameToValueV1Marker / SINGLETON_GENERAL_CATEGORY_NAME_TO_VALUE_V1_MARKER, GeneralCategoryValueToShortNameV1Marker / SINGLETON_GENERAL_CATEGORY_VALUE_TO_SHORT_NAME_V1_MARKER, GeneralCategoryValueToLongNameV1Marker / SINGLETON_GENERAL_CATEGORY_VALUE_TO_LONG_NAME_V1_MARKER;
    impl GeneralCategory {
        /// Return a [`PropertyValueNameToEnumMapper`], capable of looking up values
        /// from strings for the `General_Category` enumerated property.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::GeneralCategory;
        ///
        /// let lookup = GeneralCategory::name_to_enum_mapper();
        /// // short name for value
        /// assert_eq!(lookup.get_strict("Lu"), Some(GeneralCategory::UppercaseLetter));
        /// assert_eq!(lookup.get_strict("Pd"), Some(GeneralCategory::DashPunctuation));
        /// // long name for value
        /// assert_eq!(lookup.get_strict("Uppercase_Letter"), Some(GeneralCategory::UppercaseLetter));
        /// assert_eq!(lookup.get_strict("Dash_Punctuation"), Some(GeneralCategory::DashPunctuation));
        /// // name has incorrect casing
        /// assert_eq!(lookup.get_strict("dashpunctuation"), None);
        /// // loose matching of name
        /// assert_eq!(lookup.get_loose("dash-punctuation"), Some(GeneralCategory::DashPunctuation));
        /// // fake property
        /// assert_eq!(lookup.get_loose("Animated_Gif"), None);
        /// ```
        pub fn get_name_to_enum_mapper() / name_to_enum_mapper();
        /// Return a [`PropertyEnumToValueNameLinearMapper`], capable of looking up short names
        /// for values of the `General_Category` enumerated property.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::GeneralCategory;
        ///
        /// let lookup = GeneralCategory::enum_to_short_name_mapper();
        /// assert_eq!(lookup.get(GeneralCategory::UppercaseLetter), Some("Lu"));
        /// assert_eq!(lookup.get(GeneralCategory::DashPunctuation), Some("Pd"));
        /// assert_eq!(lookup.get(GeneralCategory::FinalPunctuation), Some("Pf"));
        /// ```
        pub fn get_enum_to_short_name_mapper() / enum_to_short_name_mapper() -> PropertyEnumToValueNameLinearMapper / PropertyEnumToValueNameLinearMapperBorrowed;
        /// Return a [`PropertyEnumToValueNameLinearMapper`], capable of looking up long names
        /// for values of the `General_Category` enumerated property.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::GeneralCategory;
        ///
        /// let lookup = GeneralCategory::enum_to_long_name_mapper();
        /// assert_eq!(lookup.get(GeneralCategory::UppercaseLetter), Some("Uppercase_Letter"));
        /// assert_eq!(lookup.get(GeneralCategory::DashPunctuation), Some("Dash_Punctuation"));
        /// assert_eq!(lookup.get(GeneralCategory::FinalPunctuation), Some("Final_Punctuation"));
        /// ```
        pub fn get_enum_to_long_name_mapper() / enum_to_long_name_mapper() -> PropertyEnumToValueNameLinearMapper / PropertyEnumToValueNameLinearMapperBorrowed;
    }
}

impl_value_getter! {
    markers: GeneralCategoryMaskNameToValueV1Marker / SINGLETON_GENERAL_CATEGORY_MASK_NAME_TO_VALUE_V1_MARKER;
    impl GeneralCategoryGroup {
        /// Return a [`PropertyValueNameToEnumMapper`], capable of looking up values
        /// from strings for the `General_Category_Mask` mask property.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::GeneralCategoryGroup;
        ///
        /// let lookup = GeneralCategoryGroup::name_to_enum_mapper();
        /// // short name for value
        /// assert_eq!(lookup.get_strict("L"), Some(GeneralCategoryGroup::Letter));
        /// assert_eq!(lookup.get_strict("LC"), Some(GeneralCategoryGroup::CasedLetter));
        /// assert_eq!(lookup.get_strict("Lu"), Some(GeneralCategoryGroup::UppercaseLetter));
        /// assert_eq!(lookup.get_strict("Zp"), Some(GeneralCategoryGroup::ParagraphSeparator));
        /// assert_eq!(lookup.get_strict("P"), Some(GeneralCategoryGroup::Punctuation));
        /// // long name for value
        /// assert_eq!(lookup.get_strict("Letter"), Some(GeneralCategoryGroup::Letter));
        /// assert_eq!(lookup.get_strict("Cased_Letter"), Some(GeneralCategoryGroup::CasedLetter));
        /// assert_eq!(lookup.get_strict("Uppercase_Letter"), Some(GeneralCategoryGroup::UppercaseLetter));
        /// // alias name
        /// assert_eq!(lookup.get_strict("punct"), Some(GeneralCategoryGroup::Punctuation));
        /// // name has incorrect casing
        /// assert_eq!(lookup.get_strict("letter"), None);
        /// // loose matching of name
        /// assert_eq!(lookup.get_loose("letter"), Some(GeneralCategoryGroup::Letter));
        /// // fake property
        /// assert_eq!(lookup.get_strict("EverythingLol"), None);
        /// ```
        pub fn get_name_to_enum_mapper() / name_to_enum_mapper();
    }
}

impl_value_getter! {
    markers: ScriptNameToValueV1Marker / SINGLETON_SCRIPT_NAME_TO_VALUE_V1_MARKER, ScriptValueToShortNameV1Marker / SINGLETON_SCRIPT_VALUE_TO_SHORT_NAME_V1_MARKER, ScriptValueToLongNameV1Marker / SINGLETON_SCRIPT_VALUE_TO_LONG_NAME_V1_MARKER;
    impl Script {
        /// Return a [`PropertyValueNameToEnumMapper`], capable of looking up values
        /// from strings for the `Script` enumerated property.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::Script;
        ///
        /// let lookup = Script::name_to_enum_mapper();
        /// // short name for value
        /// assert_eq!(lookup.get_strict("Brah"), Some(Script::Brahmi));
        /// assert_eq!(lookup.get_strict("Hang"), Some(Script::Hangul));
        /// // long name for value
        /// assert_eq!(lookup.get_strict("Brahmi"), Some(Script::Brahmi));
        /// assert_eq!(lookup.get_strict("Hangul"), Some(Script::Hangul));
        /// // name has incorrect casing
        /// assert_eq!(lookup.get_strict("brahmi"), None);
        /// // loose matching of name
        /// assert_eq!(lookup.get_loose("brahmi"), Some(Script::Brahmi));
        /// // fake property
        /// assert_eq!(lookup.get_strict("Linear_Z"), None);
        /// ```
        pub fn get_name_to_enum_mapper() / name_to_enum_mapper();
        /// Return a [`PropertyEnumToValueNameLinearMapper`], capable of looking up short names
        /// for values of the `Script` enumerated property.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::Script;
        /// use icu::locale::subtags::script;
        ///
        /// let lookup = Script::enum_to_icu_script_mapper();
        /// assert_eq!(lookup.get(Script::Brahmi), Some(script!("Brah")));
        /// assert_eq!(lookup.get(Script::Hangul), Some(script!("Hang")));
        /// ```
        pub fn get_enum_to_icu_script_mapper() / enum_to_icu_script_mapper() -> PropertyScriptToIcuScriptMapper / PropertyScriptToIcuScriptMapperBorrowed;
        /// Return a [`PropertyScriptToIcuScriptMapper`], capable of looking up long names
        /// for values of the `Script` enumerated property.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::Script;
        ///
        /// let lookup = Script::enum_to_long_name_mapper();
        /// assert_eq!(lookup.get(Script::Brahmi), Some("Brahmi"));
        /// assert_eq!(lookup.get(Script::Hangul), Some("Hangul"));
        /// ```
        pub fn get_enum_to_long_name_mapper() / enum_to_long_name_mapper() -> PropertyEnumToValueNameLinearMapper / PropertyEnumToValueNameLinearMapperBorrowed;
    }
}

impl_value_getter! {
    markers: HangulSyllableTypeNameToValueV1Marker / SINGLETON_HANGUL_SYLLABLE_TYPE_NAME_TO_VALUE_V1_MARKER, HangulSyllableTypeValueToShortNameV1Marker / SINGLETON_HANGUL_SYLLABLE_TYPE_VALUE_TO_SHORT_NAME_V1_MARKER, HangulSyllableTypeValueToLongNameV1Marker / SINGLETON_HANGUL_SYLLABLE_TYPE_VALUE_TO_LONG_NAME_V1_MARKER;
    impl HangulSyllableType {
        /// Return a [`PropertyValueNameToEnumMapper`], capable of looking up values
        /// from strings for the `Bidi_Class` enumerated property
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::HangulSyllableType;
        ///
        /// let lookup = HangulSyllableType::name_to_enum_mapper();
        /// // short name for value
        /// assert_eq!(lookup.get_strict("L"), Some(HangulSyllableType::LeadingJamo));
        /// assert_eq!(lookup.get_strict("LV"), Some(HangulSyllableType::LeadingVowelSyllable));
        /// // long name for value
        /// assert_eq!(lookup.get_strict("Leading_Jamo"), Some(HangulSyllableType::LeadingJamo));
        /// assert_eq!(lookup.get_strict("LV_Syllable"), Some(HangulSyllableType::LeadingVowelSyllable));
        /// // name has incorrect casing
        /// assert_eq!(lookup.get_strict("lv"), None);
        /// // loose matching of name
        /// assert_eq!(lookup.get_loose("lv"), Some(HangulSyllableType::LeadingVowelSyllable));
        /// // fake property
        /// assert_eq!(lookup.get_strict("LT_Syllable"), None);
        /// ```
        pub fn get_name_to_enum_mapper() / name_to_enum_mapper();
        /// Return a [`PropertyEnumToValueNameLinearMapper`], capable of looking up short names
        /// for values of the `Bidi_Class` enumerated property
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::HangulSyllableType;
        ///
        /// let lookup = HangulSyllableType::enum_to_short_name_mapper();
        /// assert_eq!(lookup.get(HangulSyllableType::LeadingJamo), Some("L"));
        /// assert_eq!(lookup.get(HangulSyllableType::LeadingVowelSyllable), Some("LV"));
        /// ```
        pub fn get_enum_to_short_name_mapper() / enum_to_short_name_mapper() -> PropertyEnumToValueNameLinearMapper / PropertyEnumToValueNameLinearMapperBorrowed;
        /// Return a [`PropertyEnumToValueNameLinearMapper`], capable of looking up long names
        /// for values of the `Bidi_Class` enumerated property
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::HangulSyllableType;
        ///
        /// let lookup = HangulSyllableType::enum_to_long_name_mapper();
        /// assert_eq!(lookup.get(HangulSyllableType::LeadingJamo), Some("Leading_Jamo"));
        /// assert_eq!(lookup.get(HangulSyllableType::LeadingVowelSyllable), Some("LV_Syllable"));
        /// ```
        pub fn get_enum_to_long_name_mapper() / enum_to_long_name_mapper() -> PropertyEnumToValueNameLinearMapper / PropertyEnumToValueNameLinearMapperBorrowed;
    }
}

impl_value_getter! {
    markers: EastAsianWidthNameToValueV1Marker / SINGLETON_EAST_ASIAN_WIDTH_NAME_TO_VALUE_V1_MARKER, EastAsianWidthValueToShortNameV1Marker / SINGLETON_EAST_ASIAN_WIDTH_VALUE_TO_SHORT_NAME_V1_MARKER, EastAsianWidthValueToLongNameV1Marker / SINGLETON_EAST_ASIAN_WIDTH_VALUE_TO_LONG_NAME_V1_MARKER;
    impl EastAsianWidth {
        /// Return a [`PropertyValueNameToEnumMapper`], capable of looking up values
        /// from strings for the `East_Asian_Width` enumerated property.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::EastAsianWidth;
        ///
        /// let lookup = EastAsianWidth::name_to_enum_mapper();
        /// // short name for value
        /// assert_eq!(lookup.get_strict("N"), Some(EastAsianWidth::Neutral));
        /// assert_eq!(lookup.get_strict("H"), Some(EastAsianWidth::Halfwidth));
        /// // long name for value
        /// assert_eq!(lookup.get_strict("Neutral"), Some(EastAsianWidth::Neutral));
        /// assert_eq!(lookup.get_strict("Halfwidth"), Some(EastAsianWidth::Halfwidth));
        /// // name has incorrect casing / extra hyphen
        /// assert_eq!(lookup.get_strict("half-width"), None);
        /// // loose matching of name
        /// assert_eq!(lookup.get_loose("half-width"), Some(EastAsianWidth::Halfwidth));
        /// // fake property
        /// assert_eq!(lookup.get_strict("TwoPointFiveWidth"), None);
        /// ```
        pub fn get_name_to_enum_mapper() / name_to_enum_mapper();
        /// Return a [`PropertyEnumToValueNameLinearMapper`], capable of looking up short names
        /// for values of the `East_Asian_Width` enumerated property.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::EastAsianWidth;
        ///
        /// let lookup = EastAsianWidth::enum_to_short_name_mapper();
        /// assert_eq!(lookup.get(EastAsianWidth::Neutral), Some("N"));
        /// assert_eq!(lookup.get(EastAsianWidth::Halfwidth), Some("H"));
        /// ```
        pub fn get_enum_to_short_name_mapper() / enum_to_short_name_mapper() -> PropertyEnumToValueNameLinearMapper / PropertyEnumToValueNameLinearMapperBorrowed;
        /// Return a [`PropertyEnumToValueNameLinearMapper`], capable of looking up long names
        /// for values of the `East_Asian_Width` enumerated property.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::EastAsianWidth;
        ///
        /// let lookup = EastAsianWidth::enum_to_long_name_mapper();
        /// assert_eq!(lookup.get(EastAsianWidth::Neutral), Some("Neutral"));
        /// assert_eq!(lookup.get(EastAsianWidth::Halfwidth), Some("Halfwidth"));
        /// ```
        pub fn get_enum_to_long_name_mapper() / enum_to_long_name_mapper() -> PropertyEnumToValueNameLinearMapper / PropertyEnumToValueNameLinearMapperBorrowed;
    }
}

impl_value_getter! {
    markers: LineBreakNameToValueV1Marker / SINGLETON_LINE_BREAK_NAME_TO_VALUE_V1_MARKER, LineBreakValueToShortNameV1Marker / SINGLETON_LINE_BREAK_VALUE_TO_SHORT_NAME_V1_MARKER, LineBreakValueToLongNameV1Marker / SINGLETON_LINE_BREAK_VALUE_TO_LONG_NAME_V1_MARKER;
    impl LineBreak {
        /// Return a [`PropertyValueNameToEnumMapper`], capable of looking up values
        /// from strings for the `Line_Break` enumerated property.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::LineBreak;
        ///
        /// let lookup = LineBreak::name_to_enum_mapper();
        /// // short name for value
        /// assert_eq!(lookup.get_strict("BK"), Some(LineBreak::MandatoryBreak));
        /// assert_eq!(lookup.get_strict("AL"), Some(LineBreak::Alphabetic));
        /// // long name for value
        /// assert_eq!(lookup.get_strict("Mandatory_Break"), Some(LineBreak::MandatoryBreak));
        /// assert_eq!(lookup.get_strict("Alphabetic"), Some(LineBreak::Alphabetic));
        /// // name has incorrect casing and dash instead of underscore
        /// assert_eq!(lookup.get_strict("mandatory-Break"), None);
        /// // loose matching of name
        /// assert_eq!(lookup.get_loose("mandatory-Break"), Some(LineBreak::MandatoryBreak));
        /// // fake property
        /// assert_eq!(lookup.get_strict("Stochastic_Break"), None);
        /// ```
        pub fn get_name_to_enum_mapper() / name_to_enum_mapper();
        /// Return a [`PropertyEnumToValueNameLinearMapper`], capable of looking up short names
        /// for values of the `Line_Break` enumerated property.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::LineBreak;
        ///
        /// let lookup = LineBreak::enum_to_short_name_mapper();
        /// assert_eq!(lookup.get(LineBreak::MandatoryBreak), Some("BK"));
        /// assert_eq!(lookup.get(LineBreak::Alphabetic), Some("AL"));
        /// ```
        pub fn get_enum_to_short_name_mapper() / enum_to_short_name_mapper() -> PropertyEnumToValueNameLinearMapper / PropertyEnumToValueNameLinearMapperBorrowed;
        /// Return a [`PropertyEnumToValueNameLinearMapper`], capable of looking up long names
        /// for values of the `Line_Break` enumerated property.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::LineBreak;
        ///
        /// let lookup = LineBreak::enum_to_long_name_mapper();
        /// assert_eq!(lookup.get(LineBreak::MandatoryBreak), Some("Mandatory_Break"));
        /// assert_eq!(lookup.get(LineBreak::Alphabetic), Some("Alphabetic"));
        /// ```
        pub fn get_enum_to_long_name_mapper() / enum_to_long_name_mapper() -> PropertyEnumToValueNameLinearMapper / PropertyEnumToValueNameLinearMapperBorrowed;
    }
}

impl_value_getter! {
    markers: GraphemeClusterBreakNameToValueV1Marker / SINGLETON_GRAPHEME_CLUSTER_BREAK_NAME_TO_VALUE_V1_MARKER, GraphemeClusterBreakValueToShortNameV1Marker / SINGLETON_GRAPHEME_CLUSTER_BREAK_VALUE_TO_SHORT_NAME_V1_MARKER, GraphemeClusterBreakValueToLongNameV1Marker / SINGLETON_GRAPHEME_CLUSTER_BREAK_VALUE_TO_LONG_NAME_V1_MARKER;
    impl GraphemeClusterBreak {
        /// Return a [`PropertyValueNameToEnumMapper`], capable of looking up values
        /// from strings for the `Grapheme_Cluster_Break` enumerated property.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::GraphemeClusterBreak;
        ///
        /// let lookup = GraphemeClusterBreak::name_to_enum_mapper();
        /// // short name for value
        /// assert_eq!(lookup.get_strict("EX"), Some(GraphemeClusterBreak::Extend));
        /// assert_eq!(lookup.get_strict("RI"), Some(GraphemeClusterBreak::RegionalIndicator));
        /// // long name for value
        /// assert_eq!(lookup.get_strict("Extend"), Some(GraphemeClusterBreak::Extend));
        /// assert_eq!(lookup.get_strict("Regional_Indicator"), Some(GraphemeClusterBreak::RegionalIndicator));
        /// // name has incorrect casing and lacks an underscore
        /// assert_eq!(lookup.get_strict("regionalindicator"), None);
        /// // loose matching of name
        /// assert_eq!(lookup.get_loose("regionalindicator"), Some(GraphemeClusterBreak::RegionalIndicator));
        /// // fake property
        /// assert_eq!(lookup.get_strict("Regional_Indicator_Two_Point_Oh"), None);
        /// ```
        pub fn get_name_to_enum_mapper() / name_to_enum_mapper();
        /// Return a [`PropertyEnumToValueNameLinearMapper`], capable of looking up short names
        /// for values of the `Grapheme_Cluster_Break` enumerated property.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::GraphemeClusterBreak;
        ///
        /// let lookup = GraphemeClusterBreak::enum_to_short_name_mapper();
        /// assert_eq!(lookup.get(GraphemeClusterBreak::Extend), Some("EX"));
        /// assert_eq!(lookup.get(GraphemeClusterBreak::RegionalIndicator), Some("RI"));
        /// ```
        pub fn get_enum_to_short_name_mapper() / enum_to_short_name_mapper() -> PropertyEnumToValueNameLinearMapper / PropertyEnumToValueNameLinearMapperBorrowed;
        /// Return a [`PropertyEnumToValueNameLinearMapper`], capable of looking up long names
        /// for values of the `Grapheme_Cluster_Break` enumerated property.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::GraphemeClusterBreak;
        ///
        /// let lookup = GraphemeClusterBreak::enum_to_long_name_mapper();
        /// assert_eq!(lookup.get(GraphemeClusterBreak::Extend), Some("Extend"));
        /// assert_eq!(lookup.get(GraphemeClusterBreak::RegionalIndicator), Some("Regional_Indicator"));
        /// ```
        pub fn get_enum_to_long_name_mapper() / enum_to_long_name_mapper() -> PropertyEnumToValueNameLinearMapper / PropertyEnumToValueNameLinearMapperBorrowed;
    }
}

impl_value_getter! {
    markers: WordBreakNameToValueV1Marker / SINGLETON_WORD_BREAK_NAME_TO_VALUE_V1_MARKER, WordBreakValueToShortNameV1Marker / SINGLETON_WORD_BREAK_VALUE_TO_SHORT_NAME_V1_MARKER, WordBreakValueToLongNameV1Marker / SINGLETON_WORD_BREAK_VALUE_TO_LONG_NAME_V1_MARKER;
    impl WordBreak {
        /// Return a [`PropertyValueNameToEnumMapper`], capable of looking up values
        /// from strings for the `Word_Break` enumerated property.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::WordBreak;
        ///
        /// let lookup = WordBreak::name_to_enum_mapper();
        /// // short name for value
        /// assert_eq!(lookup.get_strict("KA"), Some(WordBreak::Katakana));
        /// assert_eq!(lookup.get_strict("LE"), Some(WordBreak::ALetter));
        /// // long name for value
        /// assert_eq!(lookup.get_strict("Katakana"), Some(WordBreak::Katakana));
        /// assert_eq!(lookup.get_strict("ALetter"), Some(WordBreak::ALetter));
        /// // name has incorrect casing
        /// assert_eq!(lookup.get_strict("Aletter"), None);
        /// // loose matching of name
        /// assert_eq!(lookup.get_loose("Aletter"), Some(WordBreak::ALetter));
        /// assert_eq!(lookup.get_loose("w_seg_space"), Some(WordBreak::WSegSpace));
        /// // fake property
        /// assert_eq!(lookup.get_strict("Quadruple_Quote"), None);
        /// ```
        pub fn get_name_to_enum_mapper() / name_to_enum_mapper();
        /// Return a [`PropertyEnumToValueNameLinearMapper`], capable of looking up short names
        /// for values of the `Word_Break` enumerated property.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::WordBreak;
        ///
        /// let lookup = WordBreak::enum_to_short_name_mapper();
        /// assert_eq!(lookup.get(WordBreak::Katakana), Some("KA"));
        /// assert_eq!(lookup.get(WordBreak::ALetter), Some("LE"));
        /// assert_eq!(lookup.get(WordBreak::WSegSpace), Some("WSegSpace"));
        /// ```
        pub fn get_enum_to_short_name_mapper() / enum_to_short_name_mapper() -> PropertyEnumToValueNameLinearMapper / PropertyEnumToValueNameLinearMapperBorrowed;
        /// Return a [`PropertyEnumToValueNameLinearMapper`], capable of looking up long names
        /// for values of the `Word_Break` enumerated property.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::WordBreak;
        ///
        /// let lookup = WordBreak::enum_to_long_name_mapper();
        /// assert_eq!(lookup.get(WordBreak::Katakana), Some("Katakana"));
        /// assert_eq!(lookup.get(WordBreak::ALetter), Some("ALetter"));
        /// assert_eq!(lookup.get(WordBreak::WSegSpace), Some("WSegSpace"));
        /// ```
        pub fn get_enum_to_long_name_mapper() / enum_to_long_name_mapper() -> PropertyEnumToValueNameLinearMapper / PropertyEnumToValueNameLinearMapperBorrowed;
    }
}

impl_value_getter! {
    markers: SentenceBreakNameToValueV1Marker / SINGLETON_SENTENCE_BREAK_NAME_TO_VALUE_V1_MARKER, SentenceBreakValueToShortNameV1Marker / SINGLETON_SENTENCE_BREAK_VALUE_TO_SHORT_NAME_V1_MARKER, SentenceBreakValueToLongNameV1Marker / SINGLETON_SENTENCE_BREAK_VALUE_TO_LONG_NAME_V1_MARKER;
    impl SentenceBreak {
        /// Return a [`PropertyValueNameToEnumMapper`], capable of looking up values
        /// from strings for the `Sentence_Break` enumerated property.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::SentenceBreak;
        ///
        /// let lookup = SentenceBreak::name_to_enum_mapper();
        /// // short name for value
        /// assert_eq!(lookup.get_strict("FO"), Some(SentenceBreak::Format));
        /// assert_eq!(lookup.get_strict("NU"), Some(SentenceBreak::Numeric));
        /// // long name for value
        /// assert_eq!(lookup.get_strict("Format"), Some(SentenceBreak::Format));
        /// assert_eq!(lookup.get_strict("Numeric"), Some(SentenceBreak::Numeric));
        /// // name has incorrect casing
        /// assert_eq!(lookup.get_strict("fOrmat"), None);
        /// // loose matching of name
        /// assert_eq!(lookup.get_loose("fOrmat"), Some(SentenceBreak::Format));
        /// // fake property
        /// assert_eq!(lookup.get_strict("Fixer_Upper"), None);
        /// ```
        pub fn get_name_to_enum_mapper() / name_to_enum_mapper();
        /// Return a [`PropertyEnumToValueNameLinearMapper`], capable of looking up short names
        /// for values of the `Sentence_Break` enumerated property.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::SentenceBreak;
        ///
        /// let lookup = SentenceBreak::enum_to_short_name_mapper();
        /// assert_eq!(lookup.get(SentenceBreak::Format), Some("FO"));
        /// assert_eq!(lookup.get(SentenceBreak::Numeric), Some("NU"));
        /// ```
        pub fn get_enum_to_short_name_mapper() / enum_to_short_name_mapper() -> PropertyEnumToValueNameLinearMapper / PropertyEnumToValueNameLinearMapperBorrowed;
        /// Return a [`PropertyEnumToValueNameLinearMapper`], capable of looking up long names
        /// for values of the `Sentence_Break` enumerated property.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::SentenceBreak;
        ///
        /// let lookup = SentenceBreak::enum_to_long_name_mapper();
        /// assert_eq!(lookup.get(SentenceBreak::Format), Some("Format"));
        /// assert_eq!(lookup.get(SentenceBreak::Numeric), Some("Numeric"));
        /// assert_eq!(lookup.get(SentenceBreak::SContinue), Some("SContinue"));
        /// ```
        pub fn get_enum_to_long_name_mapper() / enum_to_long_name_mapper() -> PropertyEnumToValueNameLinearMapper / PropertyEnumToValueNameLinearMapperBorrowed;
    }
}

impl_value_getter! {
    markers: CanonicalCombiningClassNameToValueV1Marker / SINGLETON_CANONICAL_COMBINING_CLASS_NAME_TO_VALUE_V1_MARKER, CanonicalCombiningClassValueToShortNameV1Marker / SINGLETON_CANONICAL_COMBINING_CLASS_VALUE_TO_SHORT_NAME_V1_MARKER, CanonicalCombiningClassValueToLongNameV1Marker / SINGLETON_CANONICAL_COMBINING_CLASS_VALUE_TO_LONG_NAME_V1_MARKER;
    impl CanonicalCombiningClass {
        /// Return a [`PropertyValueNameToEnumMapper`], capable of looking up values
        /// from strings for the `Canonical_Combining_Class` enumerated property.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::CanonicalCombiningClass;
        ///
        /// let lookup = CanonicalCombiningClass::name_to_enum_mapper();
        /// // short name for value
        /// assert_eq!(lookup.get_strict("AL"), Some(CanonicalCombiningClass::AboveLeft));
        /// assert_eq!(lookup.get_strict("ATBL"), Some(CanonicalCombiningClass::AttachedBelowLeft));
        /// assert_eq!(lookup.get_strict("CCC10"), Some(CanonicalCombiningClass::CCC10));
        /// // long name for value
        /// assert_eq!(lookup.get_strict("Above_Left"), Some(CanonicalCombiningClass::AboveLeft));
        /// assert_eq!(lookup.get_strict("Attached_Below_Left"), Some(CanonicalCombiningClass::AttachedBelowLeft));
        /// // name has incorrect casing and hyphens
        /// assert_eq!(lookup.get_strict("attached-below-left"), None);
        /// // loose matching of name
        /// assert_eq!(lookup.get_loose("attached-below-left"), Some(CanonicalCombiningClass::AttachedBelowLeft));
        /// // fake property
        /// assert_eq!(lookup.get_strict("Linear_Z"), None);
        /// ```
        pub fn get_name_to_enum_mapper() / name_to_enum_mapper();
        /// Return a [`PropertyEnumToValueNameSparseMapper`], capable of looking up short names
        /// for values of the `Canonical_Combining_Class` enumerated property.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::CanonicalCombiningClass;
        ///
        /// let lookup = CanonicalCombiningClass::enum_to_short_name_mapper();
        /// assert_eq!(lookup.get(CanonicalCombiningClass::AboveLeft), Some("AL"));
        /// assert_eq!(lookup.get(CanonicalCombiningClass::AttachedBelowLeft), Some("ATBL"));
        /// assert_eq!(lookup.get(CanonicalCombiningClass::CCC10), Some("CCC10"));
        /// ```
        pub fn get_enum_to_short_name_mapper() / enum_to_short_name_mapper() -> PropertyEnumToValueNameSparseMapper / PropertyEnumToValueNameSparseMapperBorrowed;
        /// Return a [`PropertyEnumToValueNameSparseMapper`], capable of looking up long names
        /// for values of the `Canonical_Combining_Class` enumerated property.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::CanonicalCombiningClass;
        ///
        /// let lookup = CanonicalCombiningClass::enum_to_long_name_mapper();
        /// assert_eq!(lookup.get(CanonicalCombiningClass::AboveLeft), Some("Above_Left"));
        /// assert_eq!(lookup.get(CanonicalCombiningClass::AttachedBelowLeft), Some("Attached_Below_Left"));
        /// assert_eq!(lookup.get(CanonicalCombiningClass::CCC10), Some("CCC10"));
        /// ```
        pub fn get_enum_to_long_name_mapper() / enum_to_long_name_mapper() -> PropertyEnumToValueNameSparseMapper / PropertyEnumToValueNameSparseMapperBorrowed;
    }
}

impl_value_getter! {
    markers: IndicSyllabicCategoryNameToValueV1Marker / SINGLETON_INDIC_SYLLABIC_CATEGORY_NAME_TO_VALUE_V1_MARKER, IndicSyllabicCategoryValueToShortNameV1Marker / SINGLETON_INDIC_SYLLABIC_CATEGORY_VALUE_TO_SHORT_NAME_V1_MARKER, IndicSyllabicCategoryValueToLongNameV1Marker / SINGLETON_INDIC_SYLLABIC_CATEGORY_VALUE_TO_LONG_NAME_V1_MARKER;
    impl IndicSyllabicCategory {
        /// Return a [`PropertyValueNameToEnumMapper`], capable of looking up values
        /// from strings for the `Indic_Syllabic_Category` enumerated property.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::IndicSyllabicCategory;
        ///
        /// let lookup = IndicSyllabicCategory::name_to_enum_mapper();
        /// // long/short name for value
        /// assert_eq!(lookup.get_strict("Brahmi_Joining_Number"), Some(IndicSyllabicCategory::BrahmiJoiningNumber));
        /// assert_eq!(lookup.get_strict("Vowel_Independent"), Some(IndicSyllabicCategory::VowelIndependent));
        /// // name has incorrect casing and hyphens
        /// assert_eq!(lookup.get_strict("brahmi-joining-number"), None);
        /// // loose matching of name
        /// assert_eq!(lookup.get_loose("brahmi-joining-number"), Some(IndicSyllabicCategory::BrahmiJoiningNumber));
        /// // fake property
        /// assert_eq!(lookup.get_strict("Tone_Number"), None);
        /// ```
        pub fn get_name_to_enum_mapper() / name_to_enum_mapper();
        /// Return a [`PropertyEnumToValueNameLinearMapper`], capable of looking up short names
        /// for values of the `Indic_Syllabic_Category` enumerated property.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::IndicSyllabicCategory;
        ///
        /// let lookup = IndicSyllabicCategory::enum_to_short_name_mapper();
        /// assert_eq!(lookup.get(IndicSyllabicCategory::BrahmiJoiningNumber), Some("Brahmi_Joining_Number"));
        /// assert_eq!(lookup.get(IndicSyllabicCategory::VowelIndependent), Some("Vowel_Independent"));
        /// ```
        pub fn get_enum_to_short_name_mapper() / enum_to_short_name_mapper() -> PropertyEnumToValueNameLinearMapper / PropertyEnumToValueNameLinearMapperBorrowed;
        /// Return a [`PropertyEnumToValueNameLinearMapper`], capable of looking up long names
        /// for values of the `Indic_Syllabic_Category` enumerated property.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::IndicSyllabicCategory;
        ///
        /// let lookup = IndicSyllabicCategory::enum_to_long_name_mapper();
        /// assert_eq!(lookup.get(IndicSyllabicCategory::BrahmiJoiningNumber), Some("Brahmi_Joining_Number"));
        /// assert_eq!(lookup.get(IndicSyllabicCategory::VowelIndependent), Some("Vowel_Independent"));
        /// ```
        pub fn get_enum_to_long_name_mapper() / enum_to_long_name_mapper() -> PropertyEnumToValueNameLinearMapper / PropertyEnumToValueNameLinearMapperBorrowed;
    }
}

impl_value_getter! {
    markers: JoiningTypeNameToValueV1Marker / SINGLETON_JOINING_TYPE_NAME_TO_VALUE_V1_MARKER, JoiningTypeValueToShortNameV1Marker / SINGLETON_JOINING_TYPE_VALUE_TO_SHORT_NAME_V1_MARKER, JoiningTypeValueToLongNameV1Marker / SINGLETON_JOINING_TYPE_VALUE_TO_LONG_NAME_V1_MARKER;
    impl JoiningType {
        /// Return a [`PropertyValueNameToEnumMapper`], capable of looking up values
        /// from strings for the `Joining_Type` enumerated property.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::JoiningType;
        ///
        /// let lookup = JoiningType::name_to_enum_mapper();
        /// // short name for value
        /// assert_eq!(lookup.get_strict("T"), Some(JoiningType::Transparent));
        /// assert_eq!(lookup.get_strict("D"), Some(JoiningType::DualJoining));
        /// // long name for value
        /// assert_eq!(lookup.get_strict("Join_Causing"), Some(JoiningType::JoinCausing));
        /// assert_eq!(lookup.get_strict("Non_Joining"), Some(JoiningType::NonJoining));
        /// // name has incorrect casing
        /// assert_eq!(lookup.get_strict("LEFT_JOINING"), None);
        /// // loose matching of name
        /// assert_eq!(lookup.get_loose("LEFT_JOINING"), Some(JoiningType::LeftJoining));
        /// // fake property
        /// assert_eq!(lookup.get_strict("Inner_Joining"), None);
        /// ```
        pub fn get_name_to_enum_mapper() / name_to_enum_mapper();
        /// Return a [`PropertyEnumToValueNameLinearMapper`], capable of looking up short names
        /// for values of the `Joining_Type` enumerated property.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::JoiningType;
        ///
        /// let lookup = JoiningType::enum_to_short_name_mapper();
        /// assert_eq!(lookup.get(JoiningType::JoinCausing), Some("C"));
        /// assert_eq!(lookup.get(JoiningType::LeftJoining), Some("L"));
        /// ```
        pub fn get_enum_to_short_name_mapper() / enum_to_short_name_mapper() -> PropertyEnumToValueNameLinearMapper / PropertyEnumToValueNameLinearMapperBorrowed;
        /// Return a [`PropertyEnumToValueNameLinearMapper`], capable of looking up long names
        /// for values of the `Joining_Type` enumerated property.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Example
        ///
        /// ```
        /// use icu::properties::JoiningType;
        ///
        /// let lookup = JoiningType::enum_to_long_name_mapper();
        /// assert_eq!(lookup.get(JoiningType::Transparent), Some("Transparent"));
        /// assert_eq!(lookup.get(JoiningType::NonJoining), Some("Non_Joining"));
        /// assert_eq!(lookup.get(JoiningType::RightJoining), Some("Right_Joining"));
        /// ```
        pub fn get_enum_to_long_name_mapper() / enum_to_long_name_mapper() -> PropertyEnumToValueNameLinearMapper / PropertyEnumToValueNameLinearMapperBorrowed;
    }
}
