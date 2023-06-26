// @generated
/// Implement [`DataProvider<ExemplarCharactersIndexV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_exemplarchars_index_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::ExemplarCharactersIndexV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::ExemplarCharactersIndexV1Marker>, icu_provider::DataError> {
                static AR: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"'\x06\0\0)\x06\0\0*\x06\0\0;\x06\0\0A\x06\0\0I\x06\0\0J\x06\0\0K\x06\0\0") }, 28usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static EN: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0") }, 26usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SR_LATN: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0\x06\x01\0\0\x07\x01\0\0\x0C\x01\0\0\r\x01\0\0`\x01\0\0a\x01\0\0}\x01\0\0~\x01\0\0") }, 30usize)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x05\0D\xC5\xBDLJNJ") },
                ));
                static TR: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0\xC7\0\0\0\xC8\0\0\0\xD6\0\0\0\xD7\0\0\0\xDC\0\0\0\xDD\0\0\x000\x01\0\x001\x01\0\0^\x01\0\0_\x01\0\0") }, 31usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FIL: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0\xD1\0\0\0\xD2\0\0\0") }, 27usize)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0Ng") },
                ));
                static ES: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0\xD1\0\0\0\xD2\0\0\0") }, 27usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static JA: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"B0\0\0C0\0\0K0\0\0L0\0\0U0\0\0V0\0\0_0\0\0`0\0\0j0\0\0k0\0\0o0\0\0p0\0\0~0\0\0\x7F0\0\0\x840\0\0\x850\0\0\x890\0\0\x8A0\0\0\x8F0\0\0\x900\0\0") }, 10usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static RU: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x04\0\0\x02\x04\0\0\x10\x04\0\0*\x04\0\0+\x04\0\0,\x04\0\0-\x04\0\x000\x04\0\0") }, 31usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TH: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x0E\0\0/\x0E\0\0") }, 46usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SR: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\x04\0\0\x03\x04\0\0\x08\x04\0\0\x0C\x04\0\0\x0F\x04\0\0\x19\x04\0\0\x1A\x04\0\0)\x04\0\0") }, 30usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static CCP: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x11\x01\0'\x11\x01\0") }, 36usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static BN: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x85\t\0\0\x8C\t\0\0\x8F\t\0\0\x91\t\0\0\x93\t\0\0\xA9\t\0\0\xAA\t\0\0\xB1\t\0\0\xB2\t\0\0\xB3\t\0\0\xB6\t\0\0\xBA\t\0\0") }, 43usize)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xA6\x95\xE0\xA7\x8D\xE0\xA6\xB7") },
                ));
                static UND: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(zerovec::ZeroVec::new(), 0usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static VALUES: [&<icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable; 19usize] = [&AR, &AR, &BN, &CCP, &EN, &EN, &EN, &ES, &ES, &FIL, &EN, &JA, &RU, &SR, &SR, &SR_LATN, &TH, &TR, &UND];
                static KEYS: [&str; 19usize] = ["ar", "ar-EG", "bn", "ccp", "en", "en-001", "en-ZA", "es", "es-AR", "fil", "fr", "ja", "ru", "sr", "sr-Cyrl", "sr-Latn", "th", "tr", "und"];
                if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(payload)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::MissingLocale.with_req(<icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
