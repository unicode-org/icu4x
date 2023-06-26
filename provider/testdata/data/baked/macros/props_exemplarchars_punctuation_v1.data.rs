// @generated
/// Implement [`DataProvider<ExemplarCharactersPunctuationV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_exemplarchars_punctuation_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::ExemplarCharactersPunctuationV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::ExemplarCharactersPunctuationV1Marker>, icu_provider::DataError> {
                static AR: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0#\0\0\0'\0\0\0*\0\0\0-\0\0\0/\0\0\0:\0\0\0;\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x0C\x06\0\0\r\x06\0\0\x1B\x06\0\0\x1C\x06\0\0\x1F\x06\0\0 \x06\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0& \0\0' \0\0") }, 20usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static JA: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0%\0\0\0'\0\0\0(\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0_\0\0\0`\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA7\0\0\0\xA8\0\0\0\xB6\0\0\0\xB7\0\0\0\x10 \0\0\x12 \0\0\x14 \0\0\x17 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0% \0\0' \0\x000 \0\x001 \0\x002 \0\x004 \0\0; \0\0< \0\0> \0\0? \0\0\x010\0\0\x040\0\0\x080\0\0\x120\0\0\x140\0\0\x160\0\0\x1C0\0\0\x1D0\0\0\xFB0\0\0\xFC0\0\0\x01\xFF\0\0\x04\xFF\0\0\x05\xFF\0\0\x0B\xFF\0\0\x0C\xFF\0\0\x10\xFF\0\0\x1A\xFF\0\0\x1C\xFF\0\0\x1F\xFF\0\0!\xFF\0\0;\xFF\0\0>\xFF\0\0?\xFF\0\0@\xFF\0\0[\xFF\0\0\\\xFF\0\0]\xFF\0\0^\xFF\0\0a\xFF\0\0f\xFF\0\0") }, 86usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FR: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0'\0\0\0(\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x19 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0' \0\0") }, 30usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FIL: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 29usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static BN: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 32usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static CCP: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\xA7\0\0\0\xA8\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0@\x11\x01\0D\x11\x01\0") }, 36usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static RU: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x19 \0\0\x1A \0\0\x1B \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0& \0\0' \0\0") }, 32usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static ES: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0&\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0<\0\0\0?\0\0\0A\0\0\0[\0\0\0^\0\0\0\xA1\0\0\0\xA2\0\0\0\xA7\0\0\0\xA8\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0\xBF\0\0\0\xC0\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0  \0\0\" \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 37usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TH: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0$\0\0\0'\0\0\0+\0\0\0,\0\0\x000\0\0\0:\0\0\0;\0\0\0@\0\0\0A\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x15 \0\0\x18 \0\0\x1A \0\0\x1C \0\0\x1E \0\0& \0\0' \0\x002 \0\x004 \0\0") }, 26usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SR: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0#\0\0\0$\0\0\0(\0\0\0+\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\x10 \0\0\x12 \0\0\x13 \0\0\x14 \0\0\x18 \0\0\x19 \0\0\x1A \0\0\x1B \0\0\x1C \0\0\x1D \0\0\x1E \0\0\x1F \0\0& \0\0' \0\0") }, 23usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static UND: <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0(\0\0\0*\0\0\0,\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\x11 \0\0\x12 \0\0") }, 14usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static VALUES: [&<icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::DataMarker>::Yokeable; 19usize] = [&AR, &AR, &BN, &CCP, &BN, &BN, &BN, &ES, &ES, &FIL, &FR, &JA, &RU, &SR, &SR, &SR, &TH, &BN, &UND];
                static KEYS: [&str; 19usize] = ["ar", "ar-EG", "bn", "ccp", "en", "en-001", "en-ZA", "es", "es-AR", "fil", "fr", "ja", "ru", "sr", "sr-Cyrl", "sr-Latn", "th", "tr", "und"];
                if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(payload)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::MissingLocale.with_req(<icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
