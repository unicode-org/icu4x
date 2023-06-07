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
    /// [📚 Help choosing a constructor](crate::constructors)
    /// <div class="stab unstable">
    /// ⚠️ The bounds on this function may change over time, including in SemVer minor releases.
    /// </div>
    pub fn try_new_unstable<P>(provider: &P) -> Result<Self, TimeZoneError>
    where
        P: DataProvider<IanaToBcp47MapV1Marker> + ?Sized,
    {
        let data = provider.load(Default::default())?.take_payload()?;
        Ok(Self { data })
    }

    icu_provider::gen_any_buffer_constructors!(locale: skip, options: skip, error: TimeZoneError);

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
        self.data
            .map
            .get_copied(NormalizedTimeZoneIdStr::from_str(iana_id))
    }

    /// Looks up a BCP-47 time zone identifier based on an ASCII-case-insensitive match for
    /// the given IANA time zone identifier.
    ///
    /// This is the type of match specified in [ECMAScript Temporal].
    ///
    /// See examples in [`IanaToBcp47Mapper`].
    ///
    /// [ECMAScript Temporal]: https://tc39.es/proposal-temporal/#sec-isavailabletimezonename
    pub fn get_loose(&self, iana_id: &str) -> Option<TimeZoneBcp47Id> {
        self.data
            .map
            .get_copied_by(|probe| probe.cmp_loose(NormalizedTimeZoneIdStr::from_str(iana_id)))
    }
}
