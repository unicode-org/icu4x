// @generated
/// Implement [`DataProvider<ShortCompactDecimalFormatDataV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_compactdecimal_short_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker>, icu_provider::DataError> {
                static ES_AR: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x06\n\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x05\0\n\0\x0F\0\x19\0\x03\0\xC2\xA0K\x03\0\xC2\xA0k\x06\0\xC2\xA0M\t\0\xC2\xA0mil\xC2\xA0M\x0C\0\xC2\xA0B") })
                    },
                };
                static AR: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\x0C\0\x16\0 \0.\0<\0\x03\0\xC2\xA0\xD8\xA2\xD9\x84\xD8\xA7\xD9\x81\x03\0\xC2\xA0\xD8\xA3\xD9\x84\xD9\x81\x03\0\xC2\xA0\xD8\xA3\xD9\x84\xD9\x81\x06\0\xC2\xA0\xD9\x85\xD9\x84\xD9\x8A\xD9\x88\xD9\x86\t\0\xC2\xA0\xD9\x85\xD9\x84\xD9\x8A\xD8\xA7\xD8\xB1\x0C\0\xC2\xA0\xD8\xAA\xD8\xB1\xD9\x84\xD9\x8A\xD9\x88\xD9\x86") })
                    },
                };
                static BN: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x07\n\x0B\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x05\0\0\0\x06\0\0\0\x07\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x01\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\n\0\x14\0\x1E\x000\0@\0H\0\x03\0\xC2\xA0\xE0\xA6\xB9\xE0\xA6\xBE\x05\0\xC2\xA0\xE0\xA6\xB2\xE0\xA6\xBE\x07\0\xC2\xA0\xE0\xA6\x95\xE0\xA7\x8B\t\0\xC2\xA0\xE0\xA6\xB6\xE0\xA6\xA4\xC2\xA0\xE0\xA6\x95\xE0\xA7\x8B\t\0\xE0\xA6\xB6\xE0\xA6\xA4\xC2\xA0\xE0\xA6\x95\xE0\xA7\x8B\t\0\xE0\xA6\x95\xE0\xA7\x8B\x0C\0\xC2\xA0\xE0\xA6\xB2\xE0\xA6\xBE.\xE0\xA6\x95\xE0\xA7\x8B.") })
                    },
                };
                static ES: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\n\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x07\0\x0C\0\x16\0\x03\0\xC2\xA0mil\x06\0\xC2\xA0M\t\0\xC2\xA0mil\xC2\xA0M\x0C\0\xC2\xA0B") })
                    },
                };
                static SR_LATN: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\t\0\x11\0\x1A\0\x03\0\xC2\xA0hilj.\x06\0\xC2\xA0mil.\t\0\xC2\xA0mlrd.\x0C\0\xC2\xA0bil.") })
                    },
                };
                static EN: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x03\0\x06\0\t\0\x03\0K\x06\0M\t\0B\x0C\0T") })
                    },
                };
                static CCP: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x03\0\x06\0\t\0\x03\0K\x06\0M\t\0G\x0C\0T") })
                    },
                };
                static FR: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\n\0\x10\0\x03\0\xC2\xA0k\x06\0\xC2\xA0M\t\0\xC2\xA0Md\x0C\0\xC2\xA0Bn") })
                    },
                };
                static TR: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\x0B\0\x11\0\x03\0\xC2\xA0B\x06\0\xC2\xA0Mn\t\0\xC2\xA0Mr\x0C\0\xC2\xA0Tn") })
                    },
                };
                static RU: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0B\0\x15\0!\0\x03\0\xC2\xA0\xD1\x82\xD1\x8B\xD1\x81.\x06\0\xC2\xA0\xD0\xBC\xD0\xBB\xD0\xBD\t\0\xC2\xA0\xD0\xBC\xD0\xBB\xD1\x80\xD0\xB4\x0C\0\xC2\xA0\xD1\x82\xD1\x80\xD0\xBB\xD0\xBD") })
                    },
                };
                static SR: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0B\0\x16\0#\0\x03\0\xC2\xA0\xD1\x85\xD0\xB8\xD1\x99.\x06\0\xC2\xA0\xD0\xBC\xD0\xB8\xD0\xBB.\t\0\xC2\xA0\xD0\xBC\xD0\xBB\xD1\x80\xD0\xB4.\x0C\0\xC2\xA0\xD0\xB1\xD0\xB8\xD0\xBB.") })
                    },
                };
                static JA: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x04\x08\x0C\x10") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\n\0\x0F\0\x04\0\xE4\xB8\x87\x08\0\xE5\x84\x84\x0C\0\xE5\x85\x86\x10\0\xE4\xBA\xAC") })
                    },
                };
                static VALUES: [&<icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable; 24usize] = [&AR, &AR, &AR, &AR, &BN, &BN, &CCP, &CCP, &EN, &EN, &EN, &ES, &ES_AR, &EN, &FR, &JA, &RU, &SR, &SR, &SR_LATN, &EN, &EN, &TR, &CCP];
                static KEYS: [&str; 24usize] = ["ar", "ar-EG", "ar-EG-u-nu-latn", "ar-u-nu-latn", "bn", "bn-u-nu-latn", "ccp", "ccp-u-nu-latn", "en", "en-001", "en-ZA", "es", "es-AR", "fil", "fr", "ja", "ru", "sr", "sr-Cyrl", "sr-Latn", "th", "th-u-nu-thai", "tr", "und"];
                if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(payload)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::MissingLocale.with_req(<icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
