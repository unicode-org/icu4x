// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::TimeZoneError;
use crate::provider::names::*;
use crate::TimeZoneBcp47Id;
use alloc::borrow::Cow;
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
    /// TODO(2.0): Remove this field and always assume v2
    is_v1: bool,
    data: DataPayload<IanaToBcp47MapV2Marker>,
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
            is_v1: false,
            data: DataPayload::from_static_ref(
                crate::provider::Baked::SINGLETON_TIME_ZONE_IANA_TO_BCP47_V2,
            ),
        }
    }

    icu_provider::gen_any_buffer_data_constructors!(locale: skip, options: skip, error: TimeZoneError,
        #[cfg(skip)]
        functions: [
            new,
            try_new_with_any_provider,
            try_new_with_buffer_provider,
            try_new_compat,
            Self,
        ]
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::new)]
    pub fn try_new_unstable<P>(provider: &P) -> Result<Self, TimeZoneError>
    where
        P: DataProvider<IanaToBcp47MapV2Marker> + ?Sized,
    {
        let data = provider.load(Default::default())?.take_payload()?;
        Ok(Self { is_v1: false, data })
    }

    fn try_new_compat<P>(provider: &P) -> Result<Self, TimeZoneError>
    where
        P: ?Sized
        + DataProvider<IanaToBcp47MapV1Marker>
        + DataProvider<IanaToBcp47MapV2Marker>,
    {
        if let Ok(data) = DataProvider::<IanaToBcp47MapV2Marker>::load(provider, Default::default()) {
            let data = data.take_payload()?;
            Ok(Self { is_v1: false, data })
        } else {
            let data = DataProvider::<IanaToBcp47MapV1Marker>::load(provider, Default::default())?.take_payload()?;
            let data = data.map_project::<IanaToBcp47MapV2Marker, _>(|v1, _| v1.into());
            Ok(Self { is_v1: true, data })
        }
    }

    /// Returns a borrowed version of the mapper that can be queried.
    ///
    /// This avoids a small potential cost of reading the data pointer.
    pub fn as_borrowed(&self) -> IanaToBcp47MapperBorrowed {
        IanaToBcp47MapperBorrowed {
            is_v1: self.is_v1,
            data: self.data.get(),
        }
    }
}

/// A borrowed wrapper around IANA-to-BCP47 time zone data, returned by
/// [`IanaToBcp47Mapper::as_borrowed()`]. More efficient to query.
#[derive(Debug)]
pub struct IanaToBcp47MapperBorrowed<'a> {
    /// TODO(2.0): Remove this field and always assume v2
    is_v1: bool,
    data: &'a IanaToBcp47MapV2<'a>,
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
        if self.is_v1 {
            self.get_v1(iana_id)
        } else {
            self.get_v2(iana_id)
        }
    }

    /// TODO(2.0): Remove this compatibility function
    fn get_v1(&self, iana_id: &str) -> Option<TimeZoneBcp47Id> {
        // The longest IANA name in CLDR appears to be "America/Argentina/ComodRivadavia"
        // which is 32 characters long, so 48 should be plenty. Add a debug assertion
        // just in case.
        // TODO: Use a ZeroTrieSimpleAsciiCursor
        let name_for_lookup = match tinystr::TinyAsciiStr::<48>::from_bytes(iana_id.as_bytes()) {
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
        // Fix the trie to be a PerfectHash trie
        let trie = zerotrie::ZeroTriePerfectHash::from_bytes(self.data.map.as_bytes());
        let idx = trie.get(name_for_lookup.as_bytes())?;
        self.data.bcp47_ids.get(idx)
    }

    fn get_v2(&self, iana_id: &str) -> Option<TimeZoneBcp47Id> {
        let idx = self.data.map.get(iana_id.as_bytes())?;
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
    /// TODO(2.0): Remove this field and always assume v2
    is_v1: bool,
    iana_to_bcp47: DataPayload<IanaToBcp47MapV2Marker>,
    bcp47_to_iana: DataPayload<Bcp47ToIanaMapV1Marker>,
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
            crate::provider::Baked::SINGLETON_TIME_ZONE_IANA_TO_BCP47_V2.bcp47_ids_checksum
                == crate::provider::Baked::SINGLETON_TIME_ZONE_BCP47_TO_IANA_V1.bcp47_ids_checksum,
        );
        IanaBcp47RoundTripMapper {
            is_v1: false,
            iana_to_bcp47: DataPayload::from_static_ref(
                crate::provider::Baked::SINGLETON_TIME_ZONE_IANA_TO_BCP47_V2,
            ),
            bcp47_to_iana: DataPayload::from_static_ref(
                crate::provider::Baked::SINGLETON_TIME_ZONE_BCP47_TO_IANA_V1,
            ),
        }
    }

    icu_provider::gen_any_buffer_data_constructors!(locale: skip, options: skip, error: TimeZoneError,
        #[cfg(skip)]
        functions: [
            new,
            try_new_with_any_provider,
            try_new_with_buffer_provider,
            try_new_compat,
            Self,
        ]
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::new)]
    pub fn try_new_unstable<P>(provider: &P) -> Result<Self, TimeZoneError>
    where
        P: DataProvider<IanaToBcp47MapV2Marker> + DataProvider<Bcp47ToIanaMapV1Marker> + ?Sized,
    {
        let iana_to_bcp47 = provider.load(Default::default())?.take_payload()?;
        let bcp47_to_iana = provider.load(Default::default())?.take_payload()?;
        let obj = Self { is_v1: false, iana_to_bcp47, bcp47_to_iana };
        if obj.iana_to_bcp47.get().bcp47_ids_checksum != obj.bcp47_to_iana.get().bcp47_ids_checksum {
            return Err(TimeZoneError::MismatchedChecksums);
        }
        Ok(obj)
    }

    fn try_new_compat<P>(provider: &P) -> Result<Self, TimeZoneError>
    where
        P: ?Sized
        + DataProvider<IanaToBcp47MapV1Marker>
        + DataProvider<IanaToBcp47MapV2Marker>
        + DataProvider<Bcp47ToIanaMapV1Marker>,
    {
        let iana_to_bcp47 = IanaToBcp47Mapper::try_new_compat(provider)?;
        let bcp47_to_iana = provider.load(Default::default())?.take_payload()?;
        let obj = Self { is_v1: iana_to_bcp47.is_v1, iana_to_bcp47: iana_to_bcp47.data, bcp47_to_iana };
        if obj.iana_to_bcp47.get().bcp47_ids_checksum != obj.bcp47_to_iana.get().bcp47_ids_checksum {
            return Err(TimeZoneError::MismatchedChecksums);
        }
        Ok(obj)
    }

    /// Returns a borrowed version of the mapper that can be queried.
    ///
    /// This avoids a small potential cost of reading the data pointer.
    pub fn as_borrowed(&self) -> IanaBcp47RoundTripMapperBorrowed {
        IanaBcp47RoundTripMapperBorrowed {
            is_v1: self.is_v1,
            iana_to_bcp47: self.iana_to_bcp47.get(),
            bcp47_to_iana: self.bcp47_to_iana.get(),
        }
    }
}

/// A borrowed wrapper around IANA-BCP47 time zone data, returned by
/// [`IanaBcp47RoundTripMapper::as_borrowed()`]. More efficient to query.
#[derive(Debug)]
pub struct IanaBcp47RoundTripMapperBorrowed<'a> {
    /// TODO(2.0): Remove this field and always assume v2
    is_v1: bool,
    iana_to_bcp47: &'a IanaToBcp47MapV2<'a>,
    bcp47_to_iana: &'a Bcp47ToIanaMapV1<'a>,
}

impl IanaBcp47RoundTripMapperBorrowed<'_> {
    /// Looks up a BCP-47 time zone identifier based on an ASCII-case-insensitive match for
    /// the given IANA time zone identifier.
    ///
    /// This is the type of match specified in [ECMAScript Temporal].
    ///
    /// See examples in [`IanaToBcp47Mapper`] or [`IanaBcp47RoundTripMapper`].
    ///
    /// [ECMAScript Temporal]: https://tc39.es/proposal-temporal/#sec-isavailabletimezonename
    pub fn iana_to_bcp47(&self, iana_id: &str) -> Option<TimeZoneBcp47Id> {
        IanaToBcp47MapperBorrowed { is_v1: self.is_v1, data: self.iana_to_bcp47 }.get(iana_id)
    }

    /// Looks up the canonical IANA time zone identifier of a BCP-47
    /// time zone identifier.
    ///
    /// See examples in [`IanaBcp47RoundTripMapper`].
    pub fn bcp47_to_iana(&self, bcp47_id: TimeZoneBcp47Id) -> Option<&str> {
        let index = self.iana_to_bcp47.bcp47_ids.binary_search(&bcp47_id).ok()?;
        self.bcp47_to_iana.canonical_iana_ids.get(index)
    }

    /// Normalizes the case of an IANA time zone identifier. Does not canonicalize the identifier,
    /// and the identifier does not need to exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::timezone::IanaBcp47RoundTripMapper;
    ///
    /// let mapper = IanaBcp47RoundTripMapper::new();
    /// let mapper_borrowed = mapper.as_borrowed();
    ///
    /// assert_eq!(
    ///     "Asia/Ho_Chi_Minh",
    ///     mapper_borrowed.normalize_iana("Asia/Ho_chi_minh")
    /// );
    ///
    /// // Does not canonicalize identifiers:
    /// assert_eq!(
    ///     "Asia/Saigon",
    ///     mapper_borrowed.normalize_iana("Asia/Saigon")
    /// );
    ///
    /// // Identifier need not exist:
    /// assert_eq!(
    ///     "America/Fake_City",
    ///     mapper_borrowed.normalize_iana("America/faKe_CitY")
    /// );
    /// ```
    pub fn normalize_iana<'a>(&'a self, iana_id: &'a str) -> Cow<'a, str> {
        if let Some(bcp47_id) = self.iana_to_bcp47(iana_id) {
            if let Some(iana_roundtrip) = self.bcp47_to_iana(bcp47_id) {
                if iana_id.eq_ignore_ascii_case(iana_roundtrip) {
                    return Cow::Borrowed(iana_roundtrip);
                }
            }
        }
        to_ypotryll_case(iana_id)
    }
}

/// Converts a string to [ypotryll case](https://stackoverflow.com/a/54524664/1407170).
fn to_ypotryll_case(input: &str) -> Cow<str> {
    let mut is_title = true;
    let mut output = Cow::Borrowed(input);
    for i in 0..output.as_bytes().len() {
        #[allow(clippy::indexing_slicing)] // i is in range
        let current_byte = output.as_bytes()[i];
        let expected_byte = if is_title {
            current_byte.to_ascii_uppercase()
        } else {
            current_byte.to_ascii_lowercase()
        };
        if current_byte != expected_byte {
            debug_assert!(current_byte.is_ascii());
            // Safety: the byte at index i is an ASCII byte because the only bytes that can change
            // via to_ascii_*case are ASCII bytes, and we're replacing it with another ASCII byte.
            #[allow(clippy::indexing_slicing)] // i is in range
            unsafe {
                output.to_mut().as_bytes_mut()[i] = expected_byte
            };
        }
        if current_byte.is_ascii_alphanumeric() {
            is_title = false;
        } else {
            is_title = true;
        }
    }
    output
}

#[test]
fn test_to_ypotryll_case() {
    struct TestCase<'a> {
        input: &'a str,
        expected: &'a str,
    }
    let cases = [
        TestCase {
            input: "abc",
            expected: "Abc",
        },
        TestCase {
            input: "abc_def",
            expected: "Abc_Def",
        },
        TestCase {
            input: "abc/def",
            expected: "Abc/Def",
        },
        TestCase {
            input: "abc/def_ghi",
            expected: "Abc/Def_Ghi",
        },
        TestCase {
            input: "a1b",
            expected: "A1b",
        },
        TestCase {
            input: "1ab",
            expected: "1ab",
        },
    ];
    for TestCase { input, expected } in &cases {
        let output = to_ypotryll_case(input);
        let roundtrip = to_ypotryll_case(&output);
        let upper_input = input
            .chars()
            .map(|c| c.to_ascii_uppercase())
            .collect::<String>();
        let upper_output = to_ypotryll_case(&upper_input);
        assert_eq!(expected, &output);
        assert_eq!(expected, &roundtrip);
        assert_eq!(expected, &upper_output);
    }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod tests {
    use super::*;

    struct RejectByKeyProvider {
        keys: Vec<DataKey>,
    }

    impl<M: KeyedDataMarker> DataProvider<M> for RejectByKeyProvider where crate::provider::Baked: DataProvider<M> {
        fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
            if self.keys.contains(&M::KEY) {
                return Err(DataErrorKind::MissingDataKey.with_str_context("rejected"));
            }
            crate::provider::Baked.load(req)
        }
    }

    icu_provider::impl_dynamic_data_provider!(RejectByKeyProvider, [
        IanaToBcp47MapV1Marker,
        IanaToBcp47MapV2Marker,
        Bcp47ToIanaMapV1Marker,
    ], AnyMarker);

    #[test]
    fn test_old_key_oneway() {
        let provider = RejectByKeyProvider {
            keys: vec![IanaToBcp47MapV2Marker::KEY],
        };
        let mapper = IanaToBcp47Mapper::try_new_with_any_provider(&provider.as_any_provider()).unwrap();
        let mapper_borrowed = mapper.as_borrowed();
        
        // The IANA zone "Australia/Melbourne" is the BCP-47 zone "aumel"
        assert_eq!(
            mapper_borrowed.get("AUSTRALIA/MELBOURNE"),
            Some("aumel".parse().unwrap())
        );
    }

    #[test]
    fn test_old_key_roundtrip() {
        let provider = RejectByKeyProvider {
            keys: vec![IanaToBcp47MapV2Marker::KEY],
        };
        let mapper = IanaBcp47RoundTripMapper::try_new_with_any_provider(&provider.as_any_provider()).unwrap();
        let mapper_borrowed = mapper.as_borrowed();
        
        // The IANA zone "Australia/Melbourne" is the BCP-47 zone "aumel"
        assert_eq!(
            mapper_borrowed.iana_to_bcp47("AUSTRALIA/MELBOURNE"),
            Some("aumel".parse().unwrap())
        );
    }
}
