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
                static FA: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0-\0\0\0.\0\0\0/\0\0\x000\0\0\0:\0\0\0\t\x06\0\0\n\x06\0\0j\x06\0\0m\x06\0\0\xF0\x06\0\0\xFA\x06\0\0\x0E \0\0\x0F \0\x000 \0\x001 \0\0\x12\"\0\0\x13\"\0\0") }, 31usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static EU: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0-\0\0\0.\0\0\0/\0\0\x000\0\0\0:\0\0\x000 \0\x001 \0\0\x12\"\0\0\x13\"\0\0") }, 16usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static ET: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0-\0\0\x000\0\0\0:\0\0\x000 \0\x001 \0\0\x12\"\0\0\x13\"\0\0") }, 15usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AF: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0.\0\0\x000\0\0\0:\0\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 16usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static MY: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0@\x10\0\0J\x10\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 27usize)
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
                static PS: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0\t\x06\0\0\n\x06\0\0j\x06\0\0m\x06\0\0\xF0\x06\0\0\xFA\x06\0\0\x0E \0\0\x0F \0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0\x12\"\0\0\x13\"\0\0") }, 33usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static HE: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0\x0E \0\0\x0F \0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 18usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AM: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 17usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static YUE: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0\x070\0\0\x080\0\0\0N\0\0\x01N\0\0\x03N\0\0\x04N\0\0\tN\0\0\nN\0\0]N\0\0^N\0\0\x8CN\0\0\x8DN\0\0\x94N\0\0\x95N\0\0kQ\0\0lQ\0\0mQ\0\0nQ\0\0\xDBV\0\0\xDCV\0\0") }, 27usize)
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
                static GU: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0\xE6\n\0\0\xF0\n\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 27usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AS: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0\xE6\t\0\0\xF0\t\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 27usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TA: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0\xE6\x0B\0\0\xF0\x0B\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 27usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KN: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0\xE6\x0C\0\0\xF0\x0C\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 27usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static PA: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0f\n\0\0p\n\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 27usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static ML: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0f\r\0\0p\r\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 27usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static HI: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0f\t\0\0p\t\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 27usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static OR: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0f\x0B\0\0p\x0B\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 27usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TE: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0f\x0C\0\0p\x0C\0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 27usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static UR: <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\0\0\0&\0\0\0+\0\0\0/\0\0\x000\0\0\0:\0\0\0k\x06\0\0m\x06\0\0\xF0\x06\0\0\xFA\x06\0\0\x0E \0\0\x0F \0\0\x11 \0\0\x12 \0\x000 \0\x001 \0\0") }, 30usize)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static VALUES: [&<icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::DataMarker>::Yokeable; 94usize] = [&AF, &AM, &AR, &AS, &AM, &AF, &AF, &AS, &AM, &AM, &AF, &AM, &AM, &AM, &AM, &AM, &AM, &ET, &EU, &FA, &ET, &AM, &FR, &AM, &AM, &AM, &GU, &AM, &HE, &HI, &AM, &AM, &AF, &AF, &AM, &AM, &AM, &AM, &AM, &AM, &AF, &AF, &AM, &KN, &AM, &HI, &AF, &AM, &ET, &AF, &AM, &ML, &AM, &HI, &AM, &MY, &HI, &AM, &ET, &ET, &OR, &PA, &AM, &AF, &PS, &AM, &AM, &AF, &AM, &AM, &AF, &EU, &AM, &AF, &AM, &AM, &ET, &AM, &TA, &TE, &AM, &AF, &AM, &AF, &AM, &UR, &AF, &AM, &AM, &YUE, &YUE, &YUE, &YUE, &AM];
                static KEYS: [&str; 94usize] = ["af", "am", "ar", "as", "az", "be", "bg", "bn", "bs", "ca", "cs", "cy", "da", "de", "el", "en", "es", "et", "eu", "fa", "fi", "fil", "fr", "ga", "gd", "gl", "gu", "ha", "he", "hi", "hi-Latn", "hr", "hu", "hy", "id", "ig", "is", "it", "ja", "jv", "ka", "kk", "km", "kn", "ko", "kok", "ky", "lo", "lt", "lv", "mk", "ml", "mn", "mr", "ms", "my", "ne", "nl", "nn", "no", "or", "pa", "pcm", "pl", "ps", "pt", "ro", "ru", "sd", "si", "sk", "sl", "so", "sq", "sr", "sr-Latn", "sv", "sw", "ta", "te", "th", "tk", "tr", "uk", "und", "ur", "uz", "vi", "yo", "yue", "yue-Hans", "zh", "zh-Hant", "zu"];
                let mut metadata = icu_provider::DataResponseMetadata::default();
                let payload = if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    payload
                } else {
                    let mut fallback_iterator = icu_locid_transform::fallback::LocaleFallbacker::new().fallback_for(<icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::KeyedDataMarker>::KEY.into(), req.locale.clone());
                    loop {
                        if let Ok(payload) = KEYS.binary_search_by(|k| fallback_iterator.get().strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                            metadata.locale = Some(fallback_iterator.take());
                            break payload;
                        }
                        fallback_iterator.step();
                    }
                };
                Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(payload)), metadata })
            }
        }
    };
}
