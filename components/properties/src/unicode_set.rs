// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::*;
use icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList;
use icu_provider::marker::ErasedMarker;
use icu_provider::prelude::*;

/// A wrapper around `UnicodeSet` data (characters and strings)
#[derive(Debug)]
pub struct UnicodeSetData {
    data: DataPayload<ErasedMarker<PropertyUnicodeSetV1<'static>>>,
}

impl UnicodeSetData {
    /// Creates a new [`UnicodeSetData`] for a [`UnicodeSetProperty`].
    ///
    /// See the documentation on [`UnicodeSetProperty`] implementations for details.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    #[allow(clippy::new_ret_no_self)]
    pub const fn new<P: UnicodeSetProperty>() -> UnicodeSetDataBorrowed<'static> {
        UnicodeSetDataBorrowed { set: P::SINGLETON }
    }

    /// A version of `new()` that uses custom data provided by a [`DataProvider`].
    ///
    /// Note that this will return an owned version of the data. Functionality is available on
    /// the borrowed version, accessible through [`UnicodeSetData::as_borrowed`].
    pub fn try_new_unstable<P: UnicodeSetProperty>(
        provider: &(impl DataProvider<P::DataMarker> + ?Sized),
    ) -> Result<UnicodeSetData, DataError> {
        Ok(UnicodeSetData::from_data(
            provider.load(Default::default())?.payload,
        ))
    }

    /// Construct a borrowed version of this type that can be queried.
    ///
    /// This avoids a potential small underlying cost per API call (ex: `contains()`) by consolidating it
    /// up front.
    #[inline]
    pub fn as_borrowed(&self) -> UnicodeSetDataBorrowed<'_> {
        UnicodeSetDataBorrowed {
            set: self.data.get(),
        }
    }

    /// Construct a new one from loaded data
    ///
    /// Typically it is preferable to use getters instead
    pub(crate) fn from_data<M>(data: DataPayload<M>) -> Self
    where
        M: DynamicDataMarker<DataStruct = PropertyUnicodeSetV1<'static>>,
    {
        Self { data: data.cast() }
    }

    /// Construct a new owned [`CodePointInversionListAndStringList`]
    pub fn from_code_point_inversion_list_string_list(
        set: CodePointInversionListAndStringList<'static>,
    ) -> Self {
        let set = PropertyUnicodeSetV1::from_code_point_inversion_list_string_list(set);
        UnicodeSetData::from_data(
            DataPayload::<ErasedMarker<PropertyUnicodeSetV1<'static>>>::from_owned(set),
        )
    }

    /// Convert this type to a [`CodePointInversionListAndStringList`] as a borrowed value.
    ///
    /// The data backing this is extensible and supports multiple implementations.
    /// Currently it is always [`CodePointInversionListAndStringList`]; however in the future more backends may be
    /// added, and users may select which at data generation time.
    ///
    /// This method returns an `Option` in order to return `None` when the backing data provider
    /// cannot return a [`CodePointInversionListAndStringList`], or cannot do so within the expected constant time
    /// constraint.
    pub fn as_code_point_inversion_list_string_list(
        &self,
    ) -> Option<&CodePointInversionListAndStringList<'_>> {
        self.data.get().as_code_point_inversion_list_string_list()
    }

    /// Convert this type to a [`CodePointInversionListAndStringList`], borrowing if possible,
    /// otherwise allocating a new [`CodePointInversionListAndStringList`].
    ///
    /// The data backing this is extensible and supports multiple implementations.
    /// Currently it is always [`CodePointInversionListAndStringList`]; however in the future more backends may be
    /// added, and users may select which at data generation time.
    ///
    /// The performance of the conversion to this specific return type will vary
    /// depending on the data structure that is backing `self`.
    pub fn to_code_point_inversion_list_string_list(
        &self,
    ) -> CodePointInversionListAndStringList<'_> {
        self.data.get().to_code_point_inversion_list_string_list()
    }
}

/// A borrowed wrapper around code point set data, returned by
/// [`UnicodeSetData::as_borrowed()`]. More efficient to query.
#[derive(Clone, Copy, Debug)]
pub struct UnicodeSetDataBorrowed<'a> {
    set: &'a PropertyUnicodeSetV1<'a>,
}

impl<'a> UnicodeSetDataBorrowed<'a> {
    /// Check if the set contains the string. Strings consisting of one character
    /// are treated as a character/code point.
    ///
    /// This matches ICU behavior for ICU's `UnicodeSet`.
    #[inline]
    pub fn contains_str(self, s: &str) -> bool {
        self.set.contains_str(s)
    }

    /// Check if the set contains the code point.
    #[inline]
    pub fn contains(&self, ch: char) -> bool {
        self.set.contains(ch)
    }

    /// See [`Self::contains`].
    #[inline]
    pub fn contains32(&self, cp: u32) -> bool {
        self.set.contains32(cp)
    }
}

impl UnicodeSetDataBorrowed<'static> {
    /// Cheaply converts a [`UnicodeSetDataBorrowed<'static>`] into a [`UnicodeSetData`].
    ///
    /// Note: Due to branching and indirection, using [`UnicodeSetData`] might inhibit some
    /// compile-time optimizations that are possible with [`UnicodeSetDataBorrowed`].
    pub const fn static_to_owned(self) -> UnicodeSetData {
        UnicodeSetData {
            data: DataPayload::from_static_ref(self.set),
        }
    }
}

/// TODO
pub trait UnicodeSetProperty: crate::private::Sealed {
    #[doc(hidden)]
    type DataMarker: DataMarker<DataStruct = PropertyUnicodeSetV1<'static>>;
    #[doc(hidden)]
    #[cfg(feature = "compiled_data")]
    const SINGLETON: &'static PropertyUnicodeSetV1<'static>;
}
