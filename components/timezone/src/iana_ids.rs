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
/// ```
/// use icu::timezone::IanaToBcp47Mapper;
///
/// let mapper = IanaToBcp47Mapper::new();
/// let mapper_borrowed = mapper.as_borrowed();
///
/// // The IANA zone "Australia/Melbourne" is the BCP-47 zone "aumel"
/// assert_eq!(
///     mapper_borrowed.get("Australia/Melbourne"),
///     Some("aumel".parse().unwrap())
/// );
///
/// // Lookup is ASCII-case insensitive
/// assert_eq!(
///     mapper_borrowed.get("australia/melbourne"),
///     Some("aumel".parse().unwrap())
/// );
///
/// // The IANA zone "Australia/Victoria" is an alias
/// assert_eq!(
///     mapper_borrowed.get("Australia/Victoria"),
///     Some("aumel".parse().unwrap())
/// );
/// ```
#[derive(Debug)]
pub struct IanaToBcp47Mapper {
    data: DataPayload<IanaToBcp47MapV1Marker>,
}

impl IanaToBcp47Mapper {
    /// Creates a new [`IanaToBcp47Mapper`] using compiled data.
    ///
    /// See [`IanaToBcp47Mapper`] for an example.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    #[inline]
    pub const fn new() -> Self {
        IanaToBcp47Mapper {
            data: DataPayload::from_static_ref(
                crate::provider::Baked::SINGLETON_TIME_ZONE_IANA_TO_BCP47_V1,
            ),
        }
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
    /// Looks up a BCP-47 time zone identifier based on an ASCII-case-insensitive match for
    /// the given IANA time zone identifier.
    ///
    /// This is the type of match specified in [ECMAScript Temporal].
    ///
    /// See examples in [`IanaToBcp47Mapper`].
    ///
    /// [ECMAScript Temporal]: https://tc39.es/proposal-temporal/#sec-isavailabletimezonename
    pub fn get(&self, iana_id: &str) -> Option<TimeZoneBcp47Id> {
        // The longest IANA name in CLDR appears to be "America/Argentina/ComodRivadavia"
        // which is 32 characters long, so 48 should be plenty. Add a debug assertion
        // just in case.
        let name_for_lookup = match tinystr::TinyAsciiStr::<48>::from_str(iana_id) {
            Ok(tinystr) => tinystr.to_ascii_lowercase(),
            Err(tinystr::TinyStrError::TooLarge { .. }) => {
                debug_assert!(false, "IANA string too long for lookup");
                return None;
            }
            Err(_) => {
                // NonAscii
                return None;
            }
        };
        let idx = self.data.map.get(name_for_lookup.as_bytes())?;
        self.data.bcp47_ids.get(idx)
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
/// let mapper = IanaBcp47RoundTripMapper::new();
/// let mapper_borrowed = mapper.as_borrowed();
///
/// // Look up the time zone ID for "Asia/Calcutta"
/// let bcp47_id = mapper_borrowed.iana_to_bcp47("Asia/Calcutta");
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
    /// Creates a new [`IanaBcp47RoundTripMapper`] using compiled data.
    ///
    /// See [`IanaBcp47RoundTripMapper`] for an example.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    #[inline]
    pub const fn new() -> Self {
        const _: () = assert!(
            crate::provider::Baked::SINGLETON_TIME_ZONE_IANA_TO_BCP47_V1.checksum ==
            crate::provider::Baked::SINGLETON_TIME_ZONE_BCP47_TO_IANA_V1.checksum,
        );
        IanaBcp47RoundTripMapper {
            data1: DataPayload::from_static_ref(crate::provider::Baked::SINGLETON_TIME_ZONE_IANA_TO_BCP47_V1),
            data2: DataPayload::from_static_ref(crate::provider::Baked::SINGLETON_TIME_ZONE_BCP47_TO_IANA_V1),
        }
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
    /// Looks up a BCP-47 time zone identifier based on an ASCII-case-insensitive match for
    /// the given IANA time zone identifier.
    ///
    /// This is the type of match specified in [ECMAScript Temporal].
    ///
    /// See examples in [`IanaToBcp47Mapper`] or [`IanaBcp47RoundTripMapper`].
    ///
    /// [ECMAScript Temporal]: https://tc39.es/proposal-temporal/#sec-isavailabletimezonename
    pub fn iana_to_bcp47(&self, iana_id: &str) -> Option<TimeZoneBcp47Id> {
        IanaToBcp47MapperBorrowed { data: self.data1 }.get(iana_id)
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
