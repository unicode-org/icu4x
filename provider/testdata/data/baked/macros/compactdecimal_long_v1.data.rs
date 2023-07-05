// @generated
/// Implement [`DataProvider<LongCompactDecimalFormatDataV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_compactdecimal_long_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker>, icu_provider::DataError> {
                static TH: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x05\x06\t\n\x0B\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0\x07\0\0\0\x08\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x08\0\0\0\0\0\x0C\0\x1E\0*\09\0Q\0o\0\x87\0\x03\0 \xE0\xB8\x9E\xE0\xB8\xB1\xE0\xB8\x99\x04\0 \xE0\xB8\xAB\xE0\xB8\xA1\xE0\xB8\xB7\xE0\xB9\x88\xE0\xB8\x99\x05\0 \xE0\xB9\x81\xE0\xB8\xAA\xE0\xB8\x99\x06\0 \xE0\xB8\xA5\xE0\xB9\x89\xE0\xB8\xB2\xE0\xB8\x99\t\0 \xE0\xB8\x9E\xE0\xB8\xB1\xE0\xB8\x99\xE0\xB8\xA5\xE0\xB9\x89\xE0\xB8\xB2\xE0\xB8\x99\n\0 \xE0\xB8\xAB\xE0\xB8\xA1\xE0\xB8\xB7\xE0\xB9\x88\xE0\xB8\x99\xE0\xB8\xA5\xE0\xB9\x89\xE0\xB8\xB2\xE0\xB8\x99\x0B\0 \xE0\xB9\x81\xE0\xB8\xAA\xE0\xB8\x99\xE0\xB8\xA5\xE0\xB9\x89\xE0\xB8\xB2\xE0\xB8\x99\x0C\0 \xE0\xB8\xA5\xE0\xB9\x89\xE0\xB8\xB2\xE0\xB8\x99\xE0\xB8\xA5\xE0\xB9\x89\xE0\xB8\xB2\xE0\xB8\x99") })
                    },
                };
                static FR: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\n\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x05\x06\x05\x01\x05\x01\x05\x01\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\n\0\0\0\0\0\n\0\x12\0\x19\0!\0+\x006\0A\0M\0W\0\x03\0 millier\x03\0 mille\x03\xFFmille\x03\0 mille\x06\0 million\x06\0 millions\t\0 milliard\t\0 milliards\x0C\0 billion\x0C\0 billions") })
                    },
                };
                static AR: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x06\x08\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0\x05\0\0\0\x06\0\0\0\x07\0\0\0\x08\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x05\x03\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x08\0\0\0\0\0\x0B\0\x14\0\x1D\0,\09\0F\0S\0\x03\0 \xD8\xA2\xD9\x84\xD8\xA7\xD9\x81\x03\0 \xD8\xA3\xD9\x84\xD9\x81\x03\0 \xD8\xA3\xD9\x84\xD9\x81\x06\0 \xD9\x85\xD9\x84\xD8\xA7\xD9\x8A\xD9\x8A\xD9\x86\x06\0 \xD9\x85\xD9\x84\xD9\x8A\xD9\x88\xD9\x86\x06\0 \xD9\x85\xD9\x84\xD9\x8A\xD9\x88\xD9\x86\t\0 \xD9\x85\xD9\x84\xD9\x8A\xD8\xA7\xD8\xB1\x0C\0 \xD8\xAA\xD8\xB1\xD9\x84\xD9\x8A\xD9\x88\xD9\x86") })
                    },
                };
                static BN: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x07\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x12\0\x1E\0-\0\x03\0 \xE0\xA6\xB9\xE0\xA6\xBE\xE0\xA6\x9C\xE0\xA6\xBE\xE0\xA6\xB0\x05\0 \xE0\xA6\xB2\xE0\xA6\xBE\xE0\xA6\x96\x07\0 \xE0\xA6\x95\xE0\xA7\x8B\xE0\xA6\x9F\xE0\xA6\xBF\x0C\0 \xE0\xA6\xB2\xE0\xA6\xBE\xE0\xA6\x96 \xE0\xA6\x95\xE0\xA7\x8B\xE0\xA6\x9F\xE0\xA6\xBF") })
                    },
                };
                static CCP: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x03\0\x06\0\t\0\x03\0K\x06\0M\t\0G\x0C\0T") })
                    },
                };
                static TR: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x06\0\x0F\0\x18\0\x03\0 bin\x06\0 milyon\t\0 milyar\x0C\0 trilyon") })
                    },
                };
                static EN: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0B\0\x15\0\x1F\0\x03\0 thousand\x06\0 million\t\0 billion\x0C\0 trillion") })
                    },
                };
                static FIL: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x05\x01\x05\x01\x05\x01\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x08\0\0\0\0\0\x07\0\x11\0\x1A\0&\0/\0;\0E\0\x03\0 libo\x03\0 na libo\x06\0 milyon\x06\0 na milyon\t\0 bilyon\t\0 na bilyon\x0C\0 trilyon\x0C\0 na trilyon") })
                    },
                };
                static SR_LATN: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x07\0\0\0\t\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x01\x05\x01\x03\x05\x01\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\t\0\0\0\0\0\n\0\x14\0\x1D\0'\x003\0?\0K\0T\0\x03\0 hiljade\x03\0 hiljada\x06\0 milion\x06\0 miliona\t\0 milijarda\t\0 milijarde\t\0 milijardi\x0C\0 bilion\x0C\0 biliona") })
                    },
                };
                static SR: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x07\0\0\0\t\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x01\x05\x01\x03\x05\x01\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\t\0\0\0\0\0\x0F\0\x1E\0-\0>\0S\0h\0}\0\x8C\0\x03\0 \xD1\x85\xD0\xB8\xD1\x99\xD0\xB0\xD0\xB4\xD0\xB5\x03\0 \xD1\x85\xD0\xB8\xD1\x99\xD0\xB0\xD0\xB4\xD0\xB0\x06\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\x06\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\xD0\xB0\t\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xB8\xD1\x98\xD0\xB0\xD1\x80\xD0\xB4\xD0\xB0\t\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xB8\xD1\x98\xD0\xB0\xD1\x80\xD0\xB4\xD0\xB5\t\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xB8\xD1\x98\xD0\xB0\xD1\x80\xD0\xB4\xD0\xB8\x0C\0 \xD0\xB1\xD0\xB8\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\x0C\0 \xD0\xB1\xD0\xB8\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\xD0\xB0") })
                    },
                };
                static RU: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\0\0\0\x06\0\0\0\t\0\0\0\x0C\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x04\x05\x01\x04\x05\x01\x04\x05\x01\x04\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x0F\0\x1C\0+\0<\0Q\0d\0w\0\x8E\0\xA3\0\xB6\0\xCD\0\x03\0 \xD1\x82\xD1\x8B\xD1\x81\xD1\x8F\xD1\x87\xD0\xB0\x03\0 \xD1\x82\xD1\x8B\xD1\x81\xD1\x8F\xD1\x87\x03\0 \xD1\x82\xD1\x8B\xD1\x81\xD1\x8F\xD1\x87\xD0\xB8\x06\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\x06\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\xD0\xBE\xD0\xB2\x06\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\xD0\xB0\t\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xB0\xD1\x80\xD0\xB4\t\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xB0\xD1\x80\xD0\xB4\xD0\xBE\xD0\xB2\t\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xB0\xD1\x80\xD0\xB4\xD0\xB0\x0C\0 \xD1\x82\xD1\x80\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\x0C\0 \xD1\x82\xD1\x80\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\xD0\xBE\xD0\xB2\x0C\0 \xD1\x82\xD1\x80\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\xD0\xB0") })
                    },
                };
                static ES_AR: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\x07\t\x0C\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0\x07\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x01\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x10\0\x1B\0&\x005\0?\0\x03\0 mil\x06\0 mill\xC3\xB3n\x06\0 millones\x06\0 millones\t\0 mil millones\x0C\0 bill\xC3\xB3n\x0C\0 billones") })
                    },
                };
                static ES: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\x07\t\x0C\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x07\0\0\0\x08\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x01\x05\x05\x05\x01\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x08\0\0\0\0\0\x06\0\x10\0\x1B\0&\x005\0?\0J\0\x03\0 mil\x06\0 mill\xC3\xB3n\x06\0 millones\x06\0 millones\t\0 mil millones\x0C\0 bill\xC3\xB3n\x0C\0 billones\x0C\0 billones") })
                    },
                };
                static JA: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x04\x08\x0C\x10") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\n\0\x0F\0\x04\0\xE4\xB8\x87\x08\0\xE5\x84\x84\x0C\0\xE5\x85\x86\x10\0\xE4\xBA\xAC") })
                    },
                };
                static VALUES: [&<icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable; 24usize] = [&AR, &AR, &AR, &AR, &BN, &BN, &CCP, &CCP, &EN, &EN, &EN, &ES, &ES_AR, &FIL, &FR, &JA, &RU, &SR, &SR, &SR_LATN, &TH, &TH, &TR, &CCP];
                static KEYS: [&str; 24usize] = ["ar", "ar-EG", "ar-EG-u-nu-latn", "ar-u-nu-latn", "bn", "bn-u-nu-latn", "ccp", "ccp-u-nu-latn", "en", "en-001", "en-ZA", "es", "es-AR", "fil", "fr", "ja", "ru", "sr", "sr-Cyrl", "sr-Latn", "th", "th-u-nu-thai", "tr", "und"];
                if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(payload)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::MissingLocale.with_req(<icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
