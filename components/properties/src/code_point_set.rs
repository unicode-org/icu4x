// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! The functions in this module return a [`CodePointSetData`] containing
//! the set of characters with a particular Unicode property.
//!
//! The descriptions of most properties are taken from [`TR44`], the documentation for the
//! Unicode Character Database.  Some properties are instead defined in [`TR18`], the
//! documentation for Unicode regular expressions. In particular, Annex C of this document
//! defines properties for POSIX compatibility.
//!
//! [`CodePointSetData`]: crate::sets::CodePointSetData
//! [`TR44`]: https://www.unicode.org/reports/tr44
//! [`TR18`]: https://www.unicode.org/reports/tr18

use crate::provider::*;
use crate::runtime::UnicodeProperty;
use core::ops::RangeInclusive;
use icu_collections::codepointinvlist::CodePointInversionList;
use icu_provider::prelude::*;

/// A set of Unicode code points. Access its data via the borrowed version,
/// [`CodePointSetDataBorrowed`].
///
/// # Example
/// ```rust
/// use icu::properties::CodePointSetData;
/// use icu::properties::props::Alphabetic;
///
/// let alphabetic = CodePointSetData::new::<Alphabetic>();
///
/// assert!(!alphabetic.contains('3'));
/// assert!(!alphabetic.contains('੩'));  // U+0A69 GURMUKHI DIGIT THREE
/// assert!(alphabetic.contains('A'));
/// assert!(alphabetic.contains('Ä'));  // U+00C4 LATIN CAPITAL LETTER A WITH DIAERESIS
/// ```
#[derive(Debug)]
pub struct CodePointSetData {
    data: DataPayload<ErasedPropertyCodePointSetV1Marker>,
}

impl CodePointSetData {
    /// Creates a new [`CodePointSetData`] for a [`BinaryProperty`].
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[allow(clippy::new_ret_no_self)]
    #[cfg(feature = "compiled_data")]
    pub const fn new<P: BinaryProperty>() -> CodePointSetDataBorrowed<'static> {
        CodePointSetDataBorrowed { set: P::SINGLETON }
    }

    /// A version of [`Self::new()`] that uses custom data provided by a [`DataProvider`].
    ///
    /// Note that this will return an owned version of the data. Functionality is available on
    /// the borrowed version, accessible through [`CodePointSetData::as_borrowed`].
    pub fn try_new_unstable<P: BinaryProperty>(
        provider: &(impl DataProvider<P::DataMarker> + ?Sized),
    ) -> Result<CodePointSetData, DataError> {
        Ok(CodePointSetData::from_data(
            provider.load(Default::default())?.payload,
        ))
    }

    /// Construct a borrowed version of this type that can be queried.
    ///
    /// This owned version if returned by functions that use a runtime data provider.
    #[inline]
    pub fn as_borrowed(&self) -> CodePointSetDataBorrowed<'_> {
        CodePointSetDataBorrowed {
            set: self.data.get(),
        }
    }

    /// Construct a new one from loaded data
    ///
    /// Typically it is preferable to use getters like [`load_ascii_hex_digit()`] instead
    pub(crate) fn from_data<M>(data: DataPayload<M>) -> Self
    where
        M: DynamicDataMarker<DataStruct = PropertyCodePointSetV1<'static>>,
    {
        Self { data: data.cast() }
    }

    /// Construct a new owned [`CodePointInversionList`]
    pub fn from_code_point_inversion_list(set: CodePointInversionList<'static>) -> Self {
        let set = PropertyCodePointSetV1::from_code_point_inversion_list(set);
        CodePointSetData::from_data(
            DataPayload::<ErasedPropertyCodePointSetV1Marker>::from_owned(set),
        )
    }

    /// Convert this type to a [`CodePointInversionList`] as a borrowed value.
    ///
    /// The data backing this is extensible and supports multiple implementations.
    /// Currently it is always [`CodePointInversionList`]; however in the future more backends may be
    /// added, and users may select which at data generation time.
    ///
    /// This method returns an `Option` in order to return `None` when the backing data provider
    /// cannot return a [`CodePointInversionList`], or cannot do so within the expected constant time
    /// constraint.
    pub fn as_code_point_inversion_list(&self) -> Option<&CodePointInversionList<'_>> {
        self.data.get().as_code_point_inversion_list()
    }

    /// Convert this type to a [`CodePointInversionList`], borrowing if possible,
    /// otherwise allocating a new [`CodePointInversionList`].
    ///
    /// The data backing this is extensible and supports multiple implementations.
    /// Currently it is always [`CodePointInversionList`]; however in the future more backends may be
    /// added, and users may select which at data generation time.
    ///
    /// The performance of the conversion to this specific return type will vary
    /// depending on the data structure that is backing `self`.
    pub fn to_code_point_inversion_list(&self) -> CodePointInversionList<'_> {
        self.data.get().to_code_point_inversion_list()
    }
}

/// A borrowed wrapper around code point set data, returned by
/// [`CodePointSetData::as_borrowed()`]. More efficient to query.
#[derive(Clone, Copy, Debug)]
pub struct CodePointSetDataBorrowed<'a> {
    set: &'a PropertyCodePointSetV1<'a>,
}

impl CodePointSetDataBorrowed<'static> {
    /// Cheaply converts a [`CodePointSetDataBorrowed<'static>`] into a [`CodePointSetData`].
    ///
    /// Note: Due to branching and indirection, using [`CodePointSetData`] might inhibit some
    /// compile-time optimizations that are possible with [`CodePointSetDataBorrowed`].
    pub const fn static_to_owned(self) -> CodePointSetData {
        CodePointSetData {
            data: DataPayload::from_static_ref(self.set),
        }
    }
}

impl<'a> CodePointSetDataBorrowed<'a> {
    /// Check if the set contains a character
    ///
    /// ```rust
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::Alphabetic;
    ///
    /// let alphabetic = CodePointSetData::new::<Alphabetic>();
    ///
    /// assert!(!alphabetic.contains('3'));
    /// assert!(!alphabetic.contains('੩'));  // U+0A69 GURMUKHI DIGIT THREE
    /// assert!(alphabetic.contains('A'));
    /// assert!(alphabetic.contains('Ä'));  // U+00C4 LATIN CAPITAL LETTER A WITH DIAERESIS
    /// ```
    #[inline]
    pub fn contains(self, ch: char) -> bool {
        self.set.contains(ch)
    }

    /// Check if the set contains a character as a UTF32 code unit
    ///
    /// ```rust
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::Alphabetic;
    ///
    /// let alphabetic = CodePointSetData::new::<Alphabetic>();
    ///
    /// assert!(!alphabetic.contains32(0x0A69));  // U+0A69 GURMUKHI DIGIT THREE
    /// assert!(alphabetic.contains32(0x00C4));  // U+00C4 LATIN CAPITAL LETTER A WITH DIAERESIS
    /// ```
    #[inline]
    pub fn contains32(self, ch: u32) -> bool {
        self.set.contains32(ch)
    }

    // Yields an [`Iterator`] returning the ranges of the code points that are
    /// included in the [`CodePointSetData`]
    ///
    /// Ranges are returned as [`RangeInclusive`], which is inclusive of its
    /// `end` bound value. An end-inclusive behavior matches the ICU4C/J
    /// behavior of ranges, ex: `UnicodeSet::contains(UChar32 start, UChar32 end)`.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::Alphabetic;
    ///
    /// let alphabetic = CodePointSetData::new::<Alphabetic>();
    /// let mut ranges = alphabetic.iter_ranges();
    ///
    /// assert_eq!(Some(0x0041..=0x005A), ranges.next()); // 'A'..'Z'
    /// assert_eq!(Some(0x0061..=0x007A), ranges.next()); // 'a'..'z'
    /// ```
    #[inline]
    pub fn iter_ranges(self) -> impl Iterator<Item = RangeInclusive<u32>> + 'a {
        self.set.iter_ranges()
    }

    // Yields an [`Iterator`] returning the ranges of the code points that are
    /// *not* included in the [`CodePointSetData`]
    ///
    /// Ranges are returned as [`RangeInclusive`], which is inclusive of its
    /// `end` bound value. An end-inclusive behavior matches the ICU4C/J
    /// behavior of ranges, ex: `UnicodeSet::contains(UChar32 start, UChar32 end)`.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::Alphabetic;
    ///
    /// let alphabetic = CodePointSetData::new::<Alphabetic>();
    /// let mut ranges = alphabetic.iter_ranges();
    ///
    /// assert_eq!(Some(0x0041..=0x005A), ranges.next()); // 'A'..'Z'
    /// assert_eq!(Some(0x0061..=0x007A), ranges.next()); // 'a'..'z'
    /// ```
    #[inline]
    pub fn iter_ranges_complemented(self) -> impl Iterator<Item = RangeInclusive<u32>> + 'a {
        self.set.iter_ranges_complemented()
    }
}

/// A binary Unicode character property.
///
/// See [`CodePointSetData`] for usage information.
pub trait BinaryProperty: crate::private::Sealed {
    #[doc(hidden)]
    type DataMarker: DataMarker<DataStruct = PropertyCodePointSetV1<'static>>;
    #[doc(hidden)]
    #[cfg(feature = "compiled_data")]
    const SINGLETON: &'static PropertyCodePointSetV1<'static>;
    #[doc(hidden)]
    const VALUE: UnicodeProperty;
}
