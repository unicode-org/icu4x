// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::TimeZoneError;
use crate::provider::names::*;
use crate::TimeZoneBcp47Id;
use icu_provider::prelude::*;

/// A mapper from IANA time zone identifiers to BCP-47 time zone identifiers.
///
/// # Examples
///
/// Demonstration of usage with strict and loose lookup:
///
/// ```
/// use icu::timezone::IanaToBcp47Mapper;
///
/// let mapper = IanaToBcp47Mapper::try_new_unstable(&icu_testdata::unstable()).unwrap();
///
/// // Strict: the IANA identifier is already in case-canonical form, and we find a match
/// let bcp47_id = mapper.as_borrowed().get_strict("America/Chicago");
/// assert_eq!(bcp47_id, Some("uschi".parse().unwrap()));
///
/// // Strict: the IANA identifier is not in the correct case, so we find no match
/// let bcp47_id = mapper.as_borrowed().get_strict("america/chicago");
/// assert_eq!(bcp47_id, None);
///
/// // Loose: we find the IANA identifier even though it is in the wrong case
/// let bcp47_id = mapper.as_borrowed().get_loose("america/chicago");
/// assert_eq!(bcp47_id, Some("uschi".parse().unwrap()));
/// ```
#[derive(Debug)]
pub struct IanaToBcp47Mapper {
    data: DataPayload<IanaToBcp47MapV1Marker>,
}

impl IanaToBcp47Mapper {
    /// Creates a new [`IanaToBcp47Mapper`].
    ///
    /// See [`IanaToBcp47Mapper`] for an example.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    #[inline]
    pub const fn new() -> Self {
        unimplemented!()
    }

    icu_provider::gen_any_buffer_data_constructors!(locale: skip, options: skip, error: TimeZoneError,
        #[cfg(skip)]
        functions: [
            new,
            try_new_with_any_provider,
            try_new_with_buffer_provider,
            try_new_unstable,
            Self,
        ]
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::new)]
    pub fn try_new_unstable<P>(provider: &P) -> Result<Self, TimeZoneError>
    where
        P: DataProvider<IanaToBcp47MapV1Marker> + ?Sized,
    {
        let data = provider.load(Default::default())?.take_payload()?;
        Ok(Self { data })
    }

    /// Returns a borrowed version of the mapper that can be queried.
    ///
    /// This avoids a small potential cost of reading the data pointer.
    pub fn as_borrowed(&self) -> IanaToBcp47MapperBorrowed {
        IanaToBcp47MapperBorrowed {
            data: self.data.get(),
        }
    }
}

/// A borrowed wrapper around IANA-to-BCP47 time zone data, returned by
/// [`IanaToBcp47Mapper::as_borrowed()`]. More efficient to query.
#[derive(Debug)]
pub struct IanaToBcp47MapperBorrowed<'a> {
    data: &'a IanaToBcp47MapV1<'a>,
}

impl<'a> IanaToBcp47MapperBorrowed<'a> {
    /// Looks up a BCP-47 time zone identifier based on an exact match for the given IANA
    /// time zone identifier.
    ///
    /// See examples in [`IanaToBcp47Mapper`].
    pub fn get_strict(&self, iana_id: &str) -> Option<TimeZoneBcp47Id> {
        let idx = self.data.map.get(iana_id)?;
        self.data.bcp47_ids.get(idx)
    }

    /// Looks up a BCP-47 time zone identifier based on an ASCII-case-insensitive match for
    /// the given IANA time zone identifier.
    ///
    /// This is the type of match specified in [ECMAScript Temporal].
    ///
    /// See examples in [`IanaToBcp47Mapper`].
    ///
    /// [ECMAScript Temporal]: https://tc39.es/proposal-temporal/#sec-isavailabletimezonename
    pub fn get_loose(&self, _iana_id: &str) -> Option<TimeZoneBcp47Id> {
        unimplemented!()
    }
}

/// A mapper that supports mapping IANA identifiers to BCP-47 identifiers and also
/// the other way around.
///
/// This is mainly useful if the IANA identifier needs to be recovered or
/// canonicalized.
///
/// # Examples
///
/// Demonstration of canonicalization of the time zone identifier:
///
/// ```
/// use icu::timezone::IanaBcp47RoundTripMapper;
///
/// let mapper = IanaBcp47RoundTripMapper::try_new_unstable(&icu_testdata::unstable()).unwrap();
/// let mapper_borrowed = mapper.as_borrowed();
/// 
/// // Look up the time zone ID for "Asia/Calcutta"
/// let bcp47_id = mapper_borrowed.iana_to_bcp47_strict("asia/calcutta");
/// assert_eq!(bcp47_id, Some("inccu".parse().unwrap()));
///
/// // Get it back as the canonical form "Asia/Kolkata"
/// let iana_id = mapper_borrowed.bcp47_to_iana(bcp47_id.unwrap());
/// assert_eq!(iana_id, Some("Asia/Kolkata"))
/// ```
#[derive(Debug)]
pub struct IanaBcp47RoundTripMapper {
    data1: DataPayload<IanaToBcp47MapV1Marker>,
    data2: DataPayload<Bcp47ToIanaMapV1Marker>,
}

impl IanaBcp47RoundTripMapper {
    /// Creates a new [`IanaBcp47RoundTripMapper`].
    ///
    /// See [`IanaBcp47RoundTripMapper`] for an example.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    #[inline]
    pub const fn new() -> Self {
        unimplemented!()
    }

    icu_provider::gen_any_buffer_data_constructors!(locale: skip, options: skip, error: TimeZoneError,
        #[cfg(skip)]
        functions: [
            new,
            try_new_with_any_provider,
            try_new_with_buffer_provider,
            try_new_unstable,
            Self,
        ]
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::new)]
    pub fn try_new_unstable<P>(provider: &P) -> Result<Self, TimeZoneError>
    where
        P: DataProvider<IanaToBcp47MapV1Marker> + DataProvider<Bcp47ToIanaMapV1Marker> + ?Sized,
    {
        let data1 = provider.load(Default::default())?.take_payload()?;
        let data2 = provider.load(Default::default())?.take_payload()?;
        let obj = Self { data1, data2 };
        if obj.data1.get().bcp47_ids_checksum != obj.data2.get().bcp47_ids_checksum {
            return Err(TimeZoneError::MismatchedChecksums);
        }
        Ok(obj)
    }

    /// Returns a borrowed version of the mapper that can be queried.
    ///
    /// This avoids a small potential cost of reading the data pointer.
    pub fn as_borrowed(&self) -> IanaBcp47RoundTripMapperBorrowed {
        IanaBcp47RoundTripMapperBorrowed {
            data1: self.data1.get(),
            data2: self.data2.get(),
        }
    }
}

/// A borrowed wrapper around IANA-BCP47 time zone data, returned by
/// [`IanaBcp47RoundTripMapper::as_borrowed()`]. More efficient to query.
#[derive(Debug)]
pub struct IanaBcp47RoundTripMapperBorrowed<'a> {
    data1: &'a IanaToBcp47MapV1<'a>,
    data2: &'a Bcp47ToIanaMapV1<'a>,
}

impl<'a> IanaBcp47RoundTripMapperBorrowed<'a> {
    /// Looks up a BCP-47 time zone identifier based on an exact match for the given IANA
    /// time zone identifier.
    ///
    /// See examples in [`IanaToBcp47Mapper`].
    pub fn iana_to_bcp47_strict(&self, iana_id: &str) -> Option<TimeZoneBcp47Id> {
        let idx = self.data1.map.get(iana_id)?;
        self.data1.bcp47_ids.get(idx)
    }

    /// Looks up a BCP-47 time zone identifier based on an ASCII-case-insensitive match for
    /// the given IANA time zone identifier.
    ///
    /// This is the type of match specified in [ECMAScript Temporal].
    ///
    /// See examples in [`IanaToBcp47Mapper`].
    ///
    /// [ECMAScript Temporal]: https://tc39.es/proposal-temporal/#sec-isavailabletimezonename
    pub fn iana_to_bcp47_loose(&self, _iana_id: &str) -> Option<TimeZoneBcp47Id> {
        unimplemented!()
    }

    /// Looks up the canonical IANA time zone identifier of a BCP-47
    /// time zone identifier.
    ///
    /// See examples in [`IanaBcp47RoundTripMapper`].
    pub fn bcp47_to_iana(&self, bcp47_id: TimeZoneBcp47Id) -> Option<&str> {
        let index = self.data1.bcp47_ids.binary_search(&bcp47_id).ok()?;
        self.data2.canonical_iana_ids.get(index)
    }
}
