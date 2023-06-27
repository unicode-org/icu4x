// @generated
/// Implement [`DataProvider<ExemplarCharactersNumbersV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_exemplarchars_numbers_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::ExemplarCharactersNumbersV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::ExemplarCharactersNumbersV1Marker>, icu_provider::DataError> {
                static EN_ZA: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0.\0\0\x000\0\0\0:\0\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 16usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AR: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0\t\x06\0\0\n\x06\0\0\x1C\x06\0\0\x1D\x06\0\0`\x06\0\0m\x06\0\0\x0E \0\0\x0F \0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 33usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static CCP: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 17usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FR: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0\xB2\0\0\0\xB4\0\0\0\xB3\x02\0\0\xB4\x02\0\0\xE2\x02\0\0\xE3\x02\0\0H\x1D\0\0J\x1D\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0\x12\"\0\0\x13\"\0\0") }, 24usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static BN: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0\xE6\t\0\0\xF0\t\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 27usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static VALUES: [&<icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable; 19usize] = [&AR, &AR, &BN, &CCP, &CCP, &CCP, &EN_ZA, &CCP, &CCP, &CCP, &FR, &CCP, &EN_ZA, &CCP, &CCP, &CCP, &CCP, &CCP, &CCP];
                static KEYS: [&str; 19usize] = ["ar", "ar-EG", "bn", "ccp", "en", "en-001", "en-ZA", "es", "es-AR", "fil", "fr", "ja", "ru", "sr", "sr-Cyrl", "sr-Latn", "th", "tr", "und"];
                if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(payload)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::MissingLocale.with_req(<icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
