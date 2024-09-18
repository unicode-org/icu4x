// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::props::*;
use crate::provider::names::*;
use core::marker::PhantomData;
use icu_collections::codepointtrie::TrieValue;
use icu_provider::prelude::*;
use yoke::Yokeable;
use zerovec::ule::VarULE;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ErasedNameToEnumMapV1Marker;
impl DynamicDataMarker for ErasedNameToEnumMapV1Marker {
    type DataStruct = PropertyValueNameToEnumMapV1<'static>;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ErasedEnumToValueNameLinearMapV1Marker;
impl DynamicDataMarker for ErasedEnumToValueNameLinearMapV1Marker {
    type DataStruct = PropertyEnumToValueNameLinearMapV1<'static>;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ErasedEnumToValueNameSparseMapV1Marker;
impl DynamicDataMarker for ErasedEnumToValueNameSparseMapV1Marker {
    type DataStruct = PropertyEnumToValueNameSparseMapV1<'static>;
}

/// A struct capable of looking up a property value from a string name.
/// Access its data by calling [`Self::as_borrowed()`] and using the methods on
/// [`PropertyParserBorrowed`].
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
/// use icu::properties::PropertyParser;
/// use icu::properties::props::GeneralCategory;
///
/// let lookup = PropertyParser::<GeneralCategory>::new();
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
pub struct PropertyParser<T> {
    map: DataPayload<ErasedNameToEnumMapV1Marker>,
    markers: PhantomData<fn() -> T>,
}

/// A borrowed wrapper around property value name-to-enum data, returned by
/// [`PropertyParser::as_borrowed()`]. More efficient to query.
#[derive(Debug, Copy, Clone)]
pub struct PropertyParserBorrowed<'a, T> {
    map: &'a PropertyValueNameToEnumMapV1<'a>,
    markers: PhantomData<fn() -> T>,
}

impl<T> PropertyParser<T> {
    /// Creates a new instance of `PropertyParser<T>` using compiled data.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> PropertyParserBorrowed<'static, T>
    where
        T: ParseableEnumeratedProperty,
    {
        PropertyParserBorrowed {
            map: T::SINGLETON,
            markers: PhantomData,
        }
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::new)]
    pub fn try_new_unstable(
        provider: &(impl DataProvider<T::DataMarker> + ?Sized),
    ) -> Result<Self, DataError>
    where
        T: ParseableEnumeratedProperty,
    {
        Ok(Self {
            map: provider.load(Default::default())?.payload.cast(),
            markers: PhantomData,
        })
    }

    /// Construct a borrowed version of this type that can be queried.
    ///
    /// This avoids a potential small underlying cost per API call (like `get_strict()`) by consolidating it
    /// up front.
    #[inline]
    pub fn as_borrowed(&self) -> PropertyParserBorrowed<'_, T> {
        PropertyParserBorrowed {
            map: self.map.get(),
            markers: PhantomData,
        }
    }

    #[doc(hidden)] // used by FFI code
    pub fn erase(self) -> PropertyParser<u16> {
        PropertyParser {
            map: self.map.cast(),
            markers: PhantomData,
        }
    }
}

impl<T: TrieValue> PropertyParserBorrowed<'_, T> {
    /// Get the property value as a u16, doing a strict search looking for
    /// names that match exactly
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::PropertyParser;
    /// use icu::properties::props::GeneralCategory;
    ///
    /// let lookup = PropertyParser::<GeneralCategory>::new();
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
    /// use icu::properties::PropertyParser;
    /// use icu::properties::props::GeneralCategory;
    ///
    /// let lookup = PropertyParser::<GeneralCategory>::new();
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
    /// use icu::properties::PropertyParser;
    /// use icu::properties::props::GeneralCategory;
    ///
    /// let lookup = PropertyParser::<GeneralCategory>::new();
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
    /// use icu::properties::PropertyParser;
    /// use icu::properties::props::GeneralCategory;
    ///
    /// let lookup = PropertyParser::<GeneralCategory>::new();
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

impl<T: TrieValue> PropertyParserBorrowed<'static, T> {
    /// Cheaply converts a [`PropertyParserBorrowed<'static>`] into a [`PropertyParser`].
    ///
    /// Note: Due to branching and indirection, using [`PropertyParser`] might inhibit some
    /// compile-time optimizations that are possible with [`PropertyParserBorrowed`].
    pub const fn static_to_owned(self) -> PropertyParser<T> {
        PropertyParser {
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

/// A struct capable of looking up a property name from a value
/// Access its data by calling [`Self::as_borrowed()`] and using the methods on
/// [`PropertyNamesBorrowed`].
///
/// The name returned may be a short (`"KV"`) or long (`"Kana_Voicing"`) name, depending
/// on the constructor used.
///
/// # Example
///
/// ```
/// use icu::properties::PropertyNames;
/// use icu::properties::props::CanonicalCombiningClass;
///
/// let names = PropertyNames::<CanonicalCombiningClass>::new_long();
/// assert_eq!(
///     names.get(CanonicalCombiningClass::KanaVoicing),
///     Some("Kana_Voicing")
/// );
/// assert_eq!(
///     names.get(CanonicalCombiningClass::AboveLeft),
///     Some("Above_Left")
/// );
/// ```
pub struct PropertyNames<T: NamedEnumeratedProperty> {
    map: DataPayload<T::ErasedMarker>,
    markers: PhantomData<fn(T) -> ()>,
}

impl<T: NamedEnumeratedProperty> core::fmt::Debug for PropertyNames<T>
where
    DataPayload<T::ErasedMarker>: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PropertyNames")
            .field("map", &self.map)
            .field("markers", &self.markers)
            .finish()
    }
}

/// A borrowed wrapper around property value name-to-enum data, returned by
/// [`PropertyNames::as_borrowed()`]. More efficient to query.
#[derive(Debug, Copy, Clone)]
pub struct PropertyNamesBorrowed<'a, T: NamedEnumeratedProperty> {
    map: &'a T::DataType,
    markers: PhantomData<fn(T) -> ()>,
}

impl<T: NamedEnumeratedProperty> PropertyNames<T> {
    /// Creates a new instance of `PropertyNames<T>` that returns short names.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn new_short() -> PropertyNamesBorrowed<'static, T> {
        PropertyNamesBorrowed {
            map: T::SINGLETON_SHORT,
            markers: PhantomData,
        }
    }

    /// Creates a new instance of `PropertyNames<T>` that returns long names.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn new_long() -> PropertyNamesBorrowed<'static, T> {
        PropertyNamesBorrowed {
            map: T::SINGLETON_LONG,
            markers: PhantomData,
        }
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::new_short)]
    pub fn try_new_short_unstable(
        provider: &(impl DataProvider<T::DataMarkerShort> + ?Sized),
    ) -> Result<Self, DataError> {
        Ok(Self {
            map: provider.load(Default::default())?.payload.cast(),
            markers: PhantomData,
        })
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::new_long)]
    pub fn try_new_long_unstable(
        provider: &(impl DataProvider<T::DataMarkerLong> + ?Sized),
    ) -> Result<Self, DataError> {
        Ok(Self {
            map: provider.load(Default::default())?.payload.cast(),
            markers: PhantomData,
        })
    }

    /// Construct a borrowed version of this type that can be queried.
    ///
    /// This avoids a potential small underlying cost per API call (like `get_static()`) by consolidating it
    /// up front.
    #[inline]
    pub fn as_borrowed(&self) -> PropertyNamesBorrowed<'_, T> {
        PropertyNamesBorrowed {
            map: unsafe {
                &*(self.map.get() as *const <T::DataType as Yokeable>::Output as *const T::DataType)
            },
            markers: PhantomData,
        }
    }
}

impl<T: NamedEnumeratedProperty> PropertyNamesBorrowed<'_, T> {
    /// Get the property name given a value
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu::properties::PropertyNames;
    /// use icu::properties::props::CanonicalCombiningClass;
    ///
    /// let lookup = PropertyNames::<CanonicalCombiningClass>::new_long();
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
        self.map.get(property.to_u32())
    }
}

impl<T: NamedEnumeratedProperty> PropertyNamesBorrowed<'static, T> {
    /// Cheaply converts a [`PropertyNamesBorrowed<'static>`] into a [`PropertyNames`].
    ///
    /// Note: Due to branching and indirection, using [`PropertyNames`] might inhibit some
    /// compile-time optimizations that are possible with [`PropertyNamesBorrowed`].
    pub const fn static_to_owned(self) -> PropertyNames<T> {
        PropertyNames {
            map: DataPayload::from_static_ref(self.map),
            markers: PhantomData,
        }
    }
}

/// A struct capable of converting `icu::properties::props::Script` to `icu::locale::subtags::Script`.
///
/// Access its data by calling [`Self::as_borrowed()`] and using the methods on
/// [`ScriptMapperBorrowed`].
///
/// # Example
///
/// ```
/// use icu::properties::script::ScriptMapper;
/// use icu::properties::props::Script;
/// use icu::locale::subtags::script;
///
/// let lookup = ScriptMapper::new();
/// assert_eq!(lookup.get(Script::Brahmi), Some(script!("Brah")));
/// assert_eq!(lookup.get(Script::Hangul), Some(script!("Hang")));
/// ```
#[derive(Debug)]
pub struct ScriptMapper {
    map: DataPayload<ScriptValueToShortNameV1Marker>,
}

/// A borrowed wrapper around property value name-to-enum data, returned by
/// [`ScriptMapper::as_borrowed()`]. More efficient to query.
#[derive(Debug, Copy, Clone)]
pub struct ScriptMapperBorrowed<'a> {
    map: &'a PropertyScriptToIcuScriptMapV1<'a>,
}

impl ScriptMapper {
    /// Creates a new instance of `ScriptMapper` using compiled data.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> ScriptMapperBorrowed<'static> {
        ScriptMapperBorrowed {
            map: crate::provider::Baked::SINGLETON_SCRIPT_VALUE_TO_SHORT_NAME_V1_MARKER,
        }
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::new)]
    pub fn try_new_unstable(
        provider: &(impl DataProvider<ScriptValueToShortNameV1Marker> + ?Sized),
    ) -> Result<Self, DataError> {
        Ok(Self {
            map: provider.load(Default::default())?.payload,
        })
    }

    /// Construct a borrowed version of this type that can be queried.
    ///
    /// This avoids a potential small underlying cost per API call (like `get_static()`) by consolidating it
    /// up front.
    #[inline]
    pub fn as_borrowed(&self) -> ScriptMapperBorrowed<'_> {
        ScriptMapperBorrowed {
            map: self.map.get(),
        }
    }
}

impl ScriptMapperBorrowed<'_> {
    /// Get the property name given a value
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu::properties::script::ScriptMapper;
    /// use icu::properties::props::Script;
    /// use icu::locale::subtags::script;
    ///
    /// let lookup = ScriptMapper::new();
    /// assert_eq!(lookup.get(Script::Brahmi), Some(script!("Brah")));
    /// assert_eq!(lookup.get(Script::Hangul), Some(script!("Hang")));
    /// ```
    #[inline]
    pub fn get(&self, property: Script) -> Option<icu_locale_core::subtags::Script> {
        let prop = usize::try_from(property.to_u32()).ok()?;
        self.map.map.get(prop).and_then(|o| o.0)
    }
}

impl ScriptMapperBorrowed<'static> {
    /// Cheaply converts a [`ScriptMapperBorrowed<'static>`] into a [`ScriptMapper`].
    ///
    /// Note: Due to branching and indirection, using [`ScriptMapper`] might inhibit some
    /// compile-time optimizations that are possible with [`ScriptMapperBorrowed`].
    pub const fn static_to_owned(self) -> ScriptMapper {
        ScriptMapper {
            map: DataPayload::from_static_ref(self.map),
        }
    }
}

/// TODO
pub trait ParseableEnumeratedProperty: crate::private::Sealed + TrieValue {
    #[doc(hidden)]
    type DataMarker: DataMarker<DataStruct = PropertyValueNameToEnumMapV1<'static>>;
    #[doc(hidden)]
    #[cfg(feature = "compiled_data")]
    const SINGLETON: &'static PropertyValueNameToEnumMapV1<'static>;
}

// Abstract over Linear/Sparse representation
pub trait PropertyEnumToValueNameLookup {
    fn get(&self, prop: u32) -> Option<&str>;
}

impl PropertyEnumToValueNameLookup for PropertyEnumToValueNameLinearMapV1<'_> {
    fn get(&self, prop: u32) -> Option<&str> {
        self.map.get(usize::try_from(prop).ok()?)
    }
}

impl PropertyEnumToValueNameLookup for PropertyEnumToValueNameSparseMapV1<'_> {
    fn get(&self, prop: u32) -> Option<&str> {
        self.map.get(&u16::try_from(prop).ok()?)
    }
}

/// TODO
pub trait NamedEnumeratedProperty: ParseableEnumeratedProperty {
    #[doc(hidden)]
    type DataType: 'static + for<'a> Yokeable<'a> + PropertyEnumToValueNameLookup;
    #[doc(hidden)]
    type ErasedMarker: DynamicDataMarker<DataStruct = Self::DataType>;
    #[doc(hidden)]
    type DataMarkerLong: DataMarker<DataStruct = Self::DataType>;
    #[doc(hidden)]
    type DataMarkerShort: DataMarker<DataStruct = Self::DataType>;
    #[doc(hidden)]
    #[cfg(feature = "compiled_data")]
    const SINGLETON_LONG: &'static Self::DataType;
    #[doc(hidden)]
    #[cfg(feature = "compiled_data")]
    const SINGLETON_SHORT: &'static Self::DataType;
}

macro_rules! impl_value_getter {
    (
        impl $ty:ident {
            $(#[$attr_n2e:meta])*
            $marker_n2e:ident / $singleton_n2e:ident;
            $(
                Sparse:
                $(#[$attr_e2sn:meta])*
                $marker_e2sn:ident / $singleton_e2sn:ident;
                $(#[$attr_e2ln:meta])*
                $marker_e2ln:ident / $singleton_e2ln:ident;
            )?
        }
    ) => {
        // $(#[$attr_n2e])*
        impl ParseableEnumeratedProperty for $ty {
            type DataMarker = $marker_n2e;
            #[cfg(feature = "compiled_data")]
            const SINGLETON: &'static PropertyValueNameToEnumMapV1<'static> = crate::provider::Baked::$singleton_n2e;
        }

        $(
            // $(#[$attr_e2sn])*
            // $(#[$attr_e2ln])*
            impl NamedEnumeratedProperty for $ty {
                type DataType = PropertyEnumToValueNameSparseMapV1<'static>;
                type ErasedMarker = ErasedEnumToValueNameSparseMapV1Marker;
                type DataMarkerLong = crate::provider::$marker_e2ln;
                type DataMarkerShort = crate::provider::$marker_e2sn;
                #[cfg(feature = "compiled_data")]
                const SINGLETON_LONG: &'static Self::DataType = crate::provider::Baked::$singleton_e2ln;
                #[cfg(feature = "compiled_data")]
                const SINGLETON_SHORT: &'static Self::DataType = crate::provider::Baked::$singleton_e2sn;
            }
        )?
    };
    (
        impl $ty:ident {
            $(#[$attr_n2e:meta])*
            $marker_n2e:ident / $singleton_n2e:ident;
            $(
                Linear:
                $(#[$attr_e2sn:meta])*
                $marker_e2sn:ident / $singleton_e2sn:ident;
                $(#[$attr_e2ln:meta])*
                $marker_e2ln:ident / $singleton_e2ln:ident;
            )?
        }
    ) => {
        // $(#[$attr_n2e])*
        impl ParseableEnumeratedProperty for $ty {
            type DataMarker = crate::provider::$marker_n2e;
            #[cfg(feature = "compiled_data")]
            const SINGLETON: &'static PropertyValueNameToEnumMapV1<'static> = crate::provider::Baked::$singleton_n2e;
        }

        $(
            // $(#[$attr_e2sn])*
            // $(#[$attr_e2ln])*
            impl NamedEnumeratedProperty for $ty {
                type DataType = PropertyEnumToValueNameLinearMapV1<'static>;
                type ErasedMarker = ErasedEnumToValueNameLinearMapV1Marker;
                type DataMarkerLong = crate::provider::$marker_e2ln;
                type DataMarkerShort = crate::provider::$marker_e2sn;
                #[cfg(feature = "compiled_data")]
                const SINGLETON_LONG: &'static Self::DataType = crate::provider::Baked::$singleton_e2ln;
                #[cfg(feature = "compiled_data")]
                const SINGLETON_SHORT: &'static Self::DataType = crate::provider::Baked::$singleton_e2sn;
            }
        )?
    };
}

impl_value_getter! {
    impl BidiClass {
        /// Return a [`PropertyParser`], capable of looking up values
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
        BidiClassNameToValueV1Marker / SINGLETON_BIDI_CLASS_NAME_TO_VALUE_V1_MARKER;
        Linear:
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
        BidiClassValueToShortNameV1Marker / SINGLETON_BIDI_CLASS_VALUE_TO_SHORT_NAME_V1_MARKER;
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
        BidiClassValueToLongNameV1Marker / SINGLETON_BIDI_CLASS_VALUE_TO_LONG_NAME_V1_MARKER;
    }
}

impl_value_getter! {
    impl GeneralCategory {
        /// Return a [`PropertyParser`], capable of looking up values
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
        GeneralCategoryNameToValueV1Marker / SINGLETON_GENERAL_CATEGORY_NAME_TO_VALUE_V1_MARKER;
        Linear:
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
        GeneralCategoryValueToShortNameV1Marker / SINGLETON_GENERAL_CATEGORY_VALUE_TO_SHORT_NAME_V1_MARKER;
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
        GeneralCategoryValueToLongNameV1Marker / SINGLETON_GENERAL_CATEGORY_VALUE_TO_LONG_NAME_V1_MARKER;
    }
}

impl_value_getter! {
    impl GeneralCategoryGroup {
        /// Return a [`PropertyParser`], capable of looking up values
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
        GeneralCategoryMaskNameToValueV1Marker / SINGLETON_GENERAL_CATEGORY_MASK_NAME_TO_VALUE_V1_MARKER;
    }
}

impl_value_getter! {
    impl Script {
        /// Return a [`PropertyParser`], capable of looking up values
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
        ScriptNameToValueV1Marker / SINGLETON_SCRIPT_NAME_TO_VALUE_V1_MARKER;
        // /// Return a [`PropertyEnumToValueNameLinearMapper`], capable of looking up short names
        // /// for values of the `Script` enumerated property.
        // ///
        // /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        // ///
        // /// [📚 Help choosing a constructor](icu_provider::constructors)
        // ///
        // /// # Example
        // ///
        // /// ```
        // /// use icu::properties::Script;
        // /// use icu::locale::subtags::script;
        // ///
        // /// let lookup = Script::enum_to_icu_script_mapper();
        // /// assert_eq!(lookup.get(Script::Brahmi), Some(script!("Brah")));
        // /// assert_eq!(lookup.get(Script::Hangul), Some(script!("Hang")));
        // /// ```
        // ScriptMapper -> ScriptValueToShortNameV1Marker / SINGLETON_SCRIPT_VALUE_TO_SHORT_NAME_V1_MARKER;
        // /// Return a [`ScriptMapper`], capable of looking up long names
        // /// for values of the `Script` enumerated property.
        // ///
        // /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        // ///
        // /// [📚 Help choosing a constructor](icu_provider::constructors)
        // ///
        // /// # Example
        // ///
        // /// ```
        // /// use icu::properties::Script;
        // ///
        // /// let lookup = Script::enum_to_long_name_mapper();
        // /// assert_eq!(lookup.get(Script::Brahmi), Some("Brahmi"));
        // /// assert_eq!(lookup.get(Script::Hangul), Some("Hangul"));
        // /// ```
        // PropertyEnumToValueNameLinearMapper -> ScriptValueToLongNameV1Marker / SINGLETON_SCRIPT_VALUE_TO_LONG_NAME_V1_MARKER;
    }
}

impl_value_getter! {
   impl HangulSyllableType {
        /// Return a [`PropertyParser`], capable of looking up values
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
        HangulSyllableTypeNameToValueV1Marker / SINGLETON_HANGUL_SYLLABLE_TYPE_NAME_TO_VALUE_V1_MARKER;
        Linear:
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
        HangulSyllableTypeValueToShortNameV1Marker / SINGLETON_HANGUL_SYLLABLE_TYPE_VALUE_TO_SHORT_NAME_V1_MARKER;
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
        HangulSyllableTypeValueToLongNameV1Marker / SINGLETON_HANGUL_SYLLABLE_TYPE_VALUE_TO_LONG_NAME_V1_MARKER;
    }
}

impl_value_getter! {
    impl EastAsianWidth {
        /// Return a [`PropertyParser`], capable of looking up values
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
        EastAsianWidthNameToValueV1Marker / SINGLETON_EAST_ASIAN_WIDTH_NAME_TO_VALUE_V1_MARKER;
        Linear:
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
        EastAsianWidthValueToShortNameV1Marker / SINGLETON_EAST_ASIAN_WIDTH_VALUE_TO_SHORT_NAME_V1_MARKER;
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
        EastAsianWidthValueToLongNameV1Marker / SINGLETON_EAST_ASIAN_WIDTH_VALUE_TO_LONG_NAME_V1_MARKER;
    }
}

impl_value_getter! {
    impl LineBreak {
        /// Return a [`PropertyParser`], capable of looking up values
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
        LineBreakNameToValueV1Marker / SINGLETON_LINE_BREAK_NAME_TO_VALUE_V1_MARKER;
        Linear:
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
        LineBreakValueToShortNameV1Marker / SINGLETON_LINE_BREAK_VALUE_TO_SHORT_NAME_V1_MARKER;
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
        LineBreakValueToLongNameV1Marker / SINGLETON_LINE_BREAK_VALUE_TO_LONG_NAME_V1_MARKER;
    }
}

impl_value_getter! {
    impl GraphemeClusterBreak {
        /// Return a [`PropertyParser`], capable of looking up values
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
        GraphemeClusterBreakNameToValueV1Marker / SINGLETON_GRAPHEME_CLUSTER_BREAK_NAME_TO_VALUE_V1_MARKER;
        Linear:
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
        GraphemeClusterBreakValueToShortNameV1Marker / SINGLETON_GRAPHEME_CLUSTER_BREAK_VALUE_TO_SHORT_NAME_V1_MARKER;
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
        GraphemeClusterBreakValueToLongNameV1Marker / SINGLETON_GRAPHEME_CLUSTER_BREAK_VALUE_TO_LONG_NAME_V1_MARKER;
    }
}

impl_value_getter! {
    impl WordBreak {
        /// Return a [`PropertyParser`], capable of looking up values
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
        WordBreakNameToValueV1Marker / SINGLETON_WORD_BREAK_NAME_TO_VALUE_V1_MARKER;
        Linear:
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
        WordBreakValueToShortNameV1Marker / SINGLETON_WORD_BREAK_VALUE_TO_SHORT_NAME_V1_MARKER;
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
        WordBreakValueToLongNameV1Marker / SINGLETON_WORD_BREAK_VALUE_TO_LONG_NAME_V1_MARKER;
    }
}

impl_value_getter! {
    impl SentenceBreak {
        /// Return a [`PropertyParser`], capable of looking up values
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
        SentenceBreakNameToValueV1Marker / SINGLETON_SENTENCE_BREAK_NAME_TO_VALUE_V1_MARKER;
        Linear:
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
        SentenceBreakValueToShortNameV1Marker / SINGLETON_SENTENCE_BREAK_VALUE_TO_SHORT_NAME_V1_MARKER;
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
        SentenceBreakValueToLongNameV1Marker / SINGLETON_SENTENCE_BREAK_VALUE_TO_LONG_NAME_V1_MARKER;
    }
}

impl_value_getter! {
    impl CanonicalCombiningClass {
        /// Return a [`PropertyParser`], capable of looking up values
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
        CanonicalCombiningClassNameToValueV1Marker / SINGLETON_CANONICAL_COMBINING_CLASS_NAME_TO_VALUE_V1_MARKER;
        Sparse:
        /// Return a [`PropertyNames`], capable of looking up short names
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
        CanonicalCombiningClassValueToShortNameV1Marker / SINGLETON_CANONICAL_COMBINING_CLASS_VALUE_TO_SHORT_NAME_V1_MARKER;
        /// Return a [`PropertyNames`], capable of looking up long names
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
        CanonicalCombiningClassValueToLongNameV1Marker / SINGLETON_CANONICAL_COMBINING_CLASS_VALUE_TO_LONG_NAME_V1_MARKER;
    }
}

impl_value_getter! {
    impl IndicSyllabicCategory {
        /// Return a [`PropertyParser`], capable of looking up values
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
        IndicSyllabicCategoryNameToValueV1Marker / SINGLETON_INDIC_SYLLABIC_CATEGORY_NAME_TO_VALUE_V1_MARKER;
        Linear:
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
        IndicSyllabicCategoryValueToShortNameV1Marker / SINGLETON_INDIC_SYLLABIC_CATEGORY_VALUE_TO_SHORT_NAME_V1_MARKER;
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
        IndicSyllabicCategoryValueToLongNameV1Marker / SINGLETON_INDIC_SYLLABIC_CATEGORY_VALUE_TO_LONG_NAME_V1_MARKER;
    }
}

impl_value_getter! {
    impl JoiningType {
        /// Return a [`PropertyParser`], capable of looking up values
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
        JoiningTypeNameToValueV1Marker / SINGLETON_JOINING_TYPE_NAME_TO_VALUE_V1_MARKER;
        Linear:
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
        JoiningTypeValueToShortNameV1Marker / SINGLETON_JOINING_TYPE_VALUE_TO_SHORT_NAME_V1_MARKER;
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
        JoiningTypeValueToLongNameV1Marker / SINGLETON_JOINING_TYPE_VALUE_TO_LONG_NAME_V1_MARKER;
    }
}
