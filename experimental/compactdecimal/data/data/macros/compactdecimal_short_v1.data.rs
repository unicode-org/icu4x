// @generated
/// Implement `DataProvider<ShortCompactDecimalFormatDataV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_compactdecimal_short_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.65"]
        impl icu_provider::DataProvider<icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker>, icu_provider::DataError> {
                static TO: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x05\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x03\0a\x04\0m\x05\0k\x06\0M\t\0P\x0C\0T") })
                    },
                };
                static MY: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x05\x06\x07\n\x0B\x0C\r\x0E") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0\x07\0\0\0\x08\0\0\0\t\0\0\0\n\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\n\0\0\0\0\0\x13\0)\0<\0L\0\\\0q\0\x86\0\xA1\0\xB9\0\x03\0\xC2\xA0\xE1\x80\x91\xE1\x80\xB1\xE1\x80\xAC\xE1\x80\x84\xE1\x80\xBA\x04\0\xC2\xA0\xE1\x80\x9E\xE1\x80\xB1\xE1\x80\xAC\xE1\x80\x84\xE1\x80\xBA\xE1\x80\xB8\x05\0\xC2\xA0\xE1\x80\x9E\xE1\x80\xAD\xE1\x80\x94\xE1\x80\xBA\xE1\x80\xB8\x06\0\xC2\xA0\xE1\x80\x9E\xE1\x80\x94\xE1\x80\xBA\xE1\x80\xB8\x07\0\xC2\xA0\xE1\x80\x80\xE1\x80\xAF\xE1\x80\x8B\xE1\x80\xB1\n\x0E\xE1\x80\x80\xE1\x80\xAF\xE1\x80\x8B\xE1\x80\xB1\xC2\xA0\xC2\xA0\xE1\x80\x91\x0B\x0E\xE1\x80\x80\xE1\x80\xAF\xE1\x80\x8B\xE1\x80\xB1\xC2\xA0\xC2\xA0\xE1\x80\x9E\x0C\x08\xE1\x80\x8B\xE1\x80\xB1\xC2\xA0\xC2\xA0\xE1\x80\x9E\xE1\x80\xAD\xE1\x80\x94\xE1\x80\xBA\xE1\x80\xB8\r\x08\xE1\x80\x8B\xE1\x80\xB1\xC2\xA0\xC2\xA0\xE1\x80\x9E\xE1\x80\x94\xE1\x80\xBA\xE1\x80\xB8\x0E\0\xC2\xA0\xE1\x80\x80\xE1\x80\xB1\xE1\x80\xAC\xE1\x80\x8B\xE1\x80\xAD") })
                    },
                };
                static ES_419: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x06\n\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x05\0\n\0\x0F\0\x19\0\x03\0\xC2\xA0K\x03\0\xC2\xA0k\x06\0\xC2\xA0M\t\0\xC2\xA0mil\xC2\xA0M\x0C\0\xC2\xA0B") })
                    },
                };
                static KM: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x0E\0\x1E\0+\0A\0\x03\0\xE1\x9E\x96\xE1\x9E\xB6\xE1\x9E\x93\xE1\x9F\x8B\x03\0\xC2\xA0\xE1\x9E\x96\xE1\x9E\xB6\xE1\x9E\x93\xE1\x9F\x8B\x06\0\xC2\xA0\xE1\x9E\x9B\xE1\x9E\xB6\xE1\x9E\x93\t\0\xC2\xA0\xE1\x9E\x94\xE1\x9F\x8A\xE1\x9E\xB8\xE1\x9E\x9B\xE1\x9E\xB6\xE1\x9E\x93\x0C\0\xC2\xA0\xE1\x9E\x91\xE1\x9F\x92\xE1\x9E\x9A\xE1\x9E\xB8\xE1\x9E\x9B\xE1\x9E\xB6\xE1\x9E\x93") })
                    },
                };
                static AR: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\x0C\0\x16\0 \0.\0<\0\x03\0\xC2\xA0\xD8\xA2\xD9\x84\xD8\xA7\xD9\x81\x03\0\xC2\xA0\xD8\xA3\xD9\x84\xD9\x81\x03\0\xC2\xA0\xD8\xA3\xD9\x84\xD9\x81\x06\0\xC2\xA0\xD9\x85\xD9\x84\xD9\x8A\xD9\x88\xD9\x86\t\0\xC2\xA0\xD9\x85\xD9\x84\xD9\x8A\xD8\xA7\xD8\xB1\x0C\0\xC2\xA0\xD8\xAA\xD8\xB1\xD9\x84\xD9\x8A\xD9\x88\xD9\x86") })
                    },
                };
                static YUE: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x08\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\n\0\x0F\0\x03\0\xE5\x8D\x83\x04\0\xE8\x90\xAC\x08\0\xE5\x84\x84\x0C\0\xE5\x85\x86") })
                    },
                };
                static KO: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x08\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\n\0\x0F\0\x03\0\xEC\xB2\x9C\x04\0\xEB\xA7\x8C\x08\0\xEC\x96\xB5\x0C\0\xEC\xA1\xB0") })
                    },
                };
                static KK: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\n\0\x11\0\x1B\0'\0\x03\0\xC2\xA0\xD0\xBC\xD1\x8B\xD2\xA3\x03\0\xC2\xA0\xD0\xBC.\x06\0\xC2\xA0\xD0\xBC\xD0\xBB\xD0\xBD\t\0\xC2\xA0\xD0\xBC\xD0\xBB\xD1\x80\xD0\xB4\x0C\0\xC2\xA0\xD1\x82\xD1\x80\xD0\xBB\xD0\xBD") })
                    },
                };
                static LO: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x06\t\x0C\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\r\0\x1A\0*\x007\0S\0\x03\0\xC2\xA0\xE0\xBA\x9E\xE0\xBA\xB1\xE0\xBA\x99\x03\0\xC2\xA0\xE0\xBA\x81\xE0\xBA\xB5\xE0\xBA\x9A\x06\0\xC2\xA0\xE0\xBA\xA5\xE0\xBB\x89\xE0\xBA\xB2\xE0\xBA\x99\t\0\xC2\xA0\xE0\xBA\x95\xE0\xBA\xB7\xE0\xBB\x89\x0C\0\xC2\xA0\xE0\xBA\xA5\xE0\xBB\x89\xE0\xBA\xB2\xE0\xBA\x99\xE0\xBA\xA5\xE0\xBB\x89\xE0\xBA\xB2\xE0\xBA\x99\x0C\0\xE0\xBA\xA5\xE0\xBA\xA5") })
                    },
                };
                static AS: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x06\x08\t\x0B\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0\x07\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x13\0 \x003\0@\0U\0g\0\x03\0\xC2\xA0\xE0\xA6\xB9\xE0\xA6\xBE\xE0\xA6\x9C\xE0\xA6\xBE\xE0\xA7\xB0\x05\0\xC2\xA0\xE0\xA6\xB2\xE0\xA6\xBE\xE0\xA6\x96\x06\0\xC2\xA0\xE0\xA6\xA8\xE0\xA6\xBF\xE0\xA6\xAF\xE0\xA7\x81\xE0\xA6\xA4\x06\0\xC2\xA0\xE0\xA6\xA8\xE0\xA6\xBF\xE0\xA6\x83\t\0\xC2\xA0\xE0\xA6\xB6\xE0\xA6\x83\xC2\xA0\xE0\xA6\x95\xE0\xA7\x8B\xE0\xA6\x83\t\0\xC2\xA0\xE0\xA6\xB6\xE0\xA6\x83\xC2\xA0\xE0\xA6\x95\xE0\xA6\x83\x0C\0\xC2\xA0\xE0\xA6\xB6\xE0\xA6\x83\xC2\xA0\xE0\xA6\xAA\xE0\xA6\x83") })
                    },
                };
                static BN: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x07\n\x0B\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x05\0\0\0\x06\0\0\0\x07\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x01\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\n\0\x14\0\x1E\x000\0@\0H\0\x03\0\xC2\xA0\xE0\xA6\xB9\xE0\xA6\xBE\x05\0\xC2\xA0\xE0\xA6\xB2\xE0\xA6\xBE\x07\0\xC2\xA0\xE0\xA6\x95\xE0\xA7\x8B\t\0\xC2\xA0\xE0\xA6\xB6\xE0\xA6\xA4\xC2\xA0\xE0\xA6\x95\xE0\xA7\x8B\t\0\xE0\xA6\xB6\xE0\xA6\xA4\xC2\xA0\xE0\xA6\x95\xE0\xA7\x8B\t\0\xE0\xA6\x95\xE0\xA7\x8B\x0C\0\xC2\xA0\xE0\xA6\xB2\xE0\xA6\xBE.\xE0\xA6\x95\xE0\xA7\x8B.") })
                    },
                };
                static EN_IN: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x07\n\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x03\0\x06\0\n\0\x0F\0\x03\0T\x05\0L\x07\0Cr\n\0TCr\x0C\0LCr") })
                    },
                };
                static MR: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x07\t\x0B\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\x07\0\x14\0$\x004\0D\0\x03\0\xC2\xA0\xE0\xA4\xB9\x05\0\xC2\xA0\xE0\xA4\xB2\xE0\xA4\xBE\xE0\xA4\x96\x07\0\xC2\xA0\xE0\xA4\x95\xE0\xA5\x8B\xE0\xA4\x9F\xE0\xA5\x80\t\0\xC2\xA0\xE0\xA4\x85\xE0\xA4\xAC\xE0\xA5\x8D\xE0\xA4\x9C\x0B\0\xC2\xA0\xE0\xA4\x96\xE0\xA4\xB0\xE0\xA5\x8D\xE0\xA4\xB5\r\0\xC2\xA0\xE0\xA4\xAA\xE0\xA4\xA6\xE0\xA5\x8D\xE0\xA4\xAE") })
                    },
                };
                static UR: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x07\t\x0B\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\x0C\0\x18\0$\0.\0:\0\x03\0\xC2\xA0\xDB\x81\xD8\xB2\xD8\xA7\xD8\xB1\x05\0\xC2\xA0\xD9\x84\xD8\xA7\xDA\xA9\xDA\xBE\x07\0\xC2\xA0\xDA\xA9\xD8\xB1\xD9\x88\xDA\x91\t\0\xC2\xA0\xD8\xA7\xD8\xB1\xD8\xA8\x0B\0\xC2\xA0\xDA\xA9\xDA\xBE\xD8\xB1\xD8\xA8\x0C\0\xC2\xA0\xD9\xB9\xD8\xB1\xDB\x8C\xD9\x84\xDB\x8C\xD9\x86") })
                    },
                };
                static NE: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x07\t\x0B\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\x10\0\x1D\0-\0:\0G\0\x03\0\xC2\xA0\xE0\xA4\xB9\xE0\xA4\x9C\xE0\xA4\xBE\xE0\xA4\xB0\x05\0\xC2\xA0\xE0\xA4\xB2\xE0\xA4\xBE\xE0\xA4\x96\x07\0\xC2\xA0\xE0\xA4\x95\xE0\xA4\xB0\xE0\xA5\x8B\xE0\xA4\xA1\t\0\xC2\xA0\xE0\xA4\x85\xE0\xA4\xB0\xE0\xA4\xAC\x0B\0\xC2\xA0\xE0\xA4\x96\xE0\xA4\xB0\xE0\xA4\xAC\r\0\xC2\xA0\xE0\xA4\xB6\xE0\xA4\x82\xE0\xA4\x96") })
                    },
                };
                static HI: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x07\t\x0B\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\x13\0 \0*\x004\0>\0\x03\0\xC2\xA0\xE0\xA4\xB9\xE0\xA4\x9C\xE0\xA4\xBC\xE0\xA4\xBE\xE0\xA4\xB0\x05\0\xC2\xA0\xE0\xA4\xB2\xE0\xA4\xBE\xE0\xA4\x96\x07\0\xC2\xA0\xE0\xA4\x95\xE0\xA5\xB0\t\0\xC2\xA0\xE0\xA4\x85\xE0\xA5\xB0\x0B\0\xC2\xA0\xE0\xA4\x96\xE0\xA5\xB0\r\0\xC2\xA0\xE0\xA4\xA8\xE0\xA5\x80\xE0\xA4\xB2") })
                    },
                };
                static PA: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x07\t\x0B\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\x13\0 \x000\0=\0J\0\x03\0\xC2\xA0\xE0\xA8\xB9\xE0\xA8\x9C\xE0\xA8\xBC\xE0\xA8\xBE\xE0\xA8\xB0\x05\0\xC2\xA0\xE0\xA8\xB2\xE0\xA9\xB1\xE0\xA8\x96\x07\0\xC2\xA0\xE0\xA8\x95\xE0\xA8\xB0\xE0\xA9\x8B\xE0\xA9\x9C\t\0\xC2\xA0\xE0\xA8\x85\xE0\xA8\xB0\xE0\xA8\xAC\x0B\0\xC2\xA0\xE0\xA8\x96\xE0\xA8\xB0\xE0\xA8\xAC\r\0\xC2\xA0\xE0\xA8\xA8\xE0\xA9\x80\xE0\xA8\xB2") })
                    },
                };
                static GU: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x07\t\x0B\x0C\r\x0E") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0\x07\0\0\0\x08\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x08\0\0\0\0\0\x10\0\x1D\0-\0:\0P\0i\0y\0\x03\0\xC2\xA0\xE0\xAA\xB9\xE0\xAA\x9C\xE0\xAA\xBE\xE0\xAA\xB0\x05\0\xC2\xA0\xE0\xAA\xB2\xE0\xAA\xBE\xE0\xAA\x96\x07\0\xC2\xA0\xE0\xAA\x95\xE0\xAA\xB0\xE0\xAB\x8B\xE0\xAA\xA1\t\0\xC2\xA0\xE0\xAA\x85\xE0\xAA\xAC\xE0\xAA\x9C\x0B\0\xC2\xA0\xE0\xAA\xA8\xE0\xAA\xBF\xE0\xAA\x96\xE0\xAA\xB0\xE0\xAB\x8D\xE0\xAA\xB5\x0C\0\xC2\xA0\xE0\xAA\xAE\xE0\xAA\xB9\xE0\xAA\xBE\xE0\xAA\xAA\xE0\xAA\xA6\xE0\xAB\x8D\xE0\xAA\xAE\r\0\xC2\xA0\xE0\xAA\xB6\xE0\xAA\x82\xE0\xAA\x95\xE0\xAB\x81\x0E\0\xC2\xA0\xE0\xAA\x9C\xE0\xAA\xB2\xE0\xAA\xA7\xE0\xAA\xBF") })
                    },
                };
                static CA: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\n\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\n\0\x10\0\x03\0\xC2\xA0k\x06\0\xC2\xA0M\t\0\xC2\xA0kM\x0C\0\xC2\xA0B") })
                    },
                };
                static ES_US: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\n\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\n\0\x14\0\x03\0\xC2\xA0K\x06\0\xC2\xA0M\t\0\xC2\xA0mil\xC2\xA0M\x0C\0\xC2\xA0B") })
                    },
                };
                static ES_MX: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\n\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\n\0\x14\0\x03\0\xC2\xA0k\x06\0\xC2\xA0M\t\0\xC2\xA0mil\xC2\xA0M\x0C\0\xC2\xA0B") })
                    },
                };
                static ES: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\n\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x07\0\x0C\0\x16\0\x03\0\xC2\xA0mil\x06\0\xC2\xA0M\t\0\xC2\xA0mil\xC2\xA0M\x0C\0\xC2\xA0B") })
                    },
                };
                static LO_U_NU_LAOO: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0B\0\x19\0$\0\x03\0\xE0\xBA\x9E\xE0\xBA\xB1\xE0\xBA\x99\x06\0\xE0\xBA\xA5\xE0\xBB\x89\xE0\xBA\xB2\xE0\xBA\x99\t\0\xE0\xBA\x95\xE0\xBA\xB7\xE0\xBB\x89\x0C\0\xE0\xBA\x9E\xE0\xBA\xB1\xE0\xBA\x99\xE0\xBA\x95\xE0\xBA\xB7\xE0\xBB\x89") })
                    },
                };
                static KA: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0B\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x0B\0\x19\0*\08\0\x03\0\xC2\xA0\xE1\x83\x90\xE1\x83\x97.\x06\0\xC2\xA0\xE1\x83\x9B\xE1\x83\x9A\xE1\x83\x9C.\t\0\xC2\xA0\xE1\x83\x9B\xE1\x83\x9A\xE1\x83\xA0\xE1\x83\x93.\t\0\xC2\xA0\xE1\x83\x9B\xE1\x83\x9A\xE1\x83\xA0.\x0C\0\xC2\xA0\xE1\x83\xA2\xE1\x83\xA0\xE1\x83\x9A.") })
                    },
                };
                static MN: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0B\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x0E\0\x18\0(\0,\0\x03\0\xC2\xA0\xD0\xBC\xD1\x8F\xD0\xBD\xD0\xB3\xD0\xB0\x06\0\xC2\xA0\xD1\x81\xD0\xB0\xD1\x8F\t\0\xC2\xA0\xD1\x82\xD1\x8D\xD1\x80\xD0\xB1\xD1\x83\xD0\xBC\t\0\xD0\xA2\x0C\0\xD0\x98\xD0\x9D") })
                    },
                };
                static PS: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0B\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x05\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x01\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x03\0K\x06\0M\t\0B\t\0G\t\0B\x0C\0T") })
                    },
                };
                static UZ_CYRL: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\n\0\x12\0\x1C\0\x03\0\xD0\xBC\xD0\xB8\xD0\xBD\xD0\xB3\x06\0\xD0\xBC\xD0\xBB\xD0\xBD\t\0\xD0\xBC\xD0\xBB\xD1\x80\xD0\xB4\x0C\0\xD1\x82\xD1\x80\xD0\xBB\xD0\xBD") })
                    },
                };
                static HY: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\n\0\x14\0 \0\x03\0\xC2\xA0\xD5\xB0\xD5\xA6\xD6\x80\x06\0\xC2\xA0\xD5\xB4\xD5\xAC\xD5\xB6\t\0\xC2\xA0\xD5\xB4\xD5\xAC\xD6\x80\xD5\xA4\x0C\0\xC2\xA0\xD5\xBF\xD6\x80\xD5\xAC\xD5\xB6") })
                    },
                };
                static KY: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\n\0\x14\0\x1E\0\x03\0\xC2\xA0\xD0\xBC\xD0\xB8\xD2\xA3\x06\0\xC2\xA0\xD0\xBC\xD0\xBB\xD0\xBD\t\0\xC2\xA0\xD0\xBC\xD0\xBB\xD0\xB4\x0C\0\xC2\xA0\xD1\x82\xD1\x80\xD0\xBB\xD0\xBD") })
                    },
                };
                static SQ: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\t\0\x10\0\x17\0\x03\0\xC2\xA0mij\xC3\xAB\x06\0\xC2\xA0mln\t\0\xC2\xA0mld\x0C\0\xC2\xA0bln") })
                    },
                };
                static TK: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\t\0\x10\0\x18\0\x03\0\xC2\xA0m\xC3\xBC\xC5\x88\x06\0\xC2\xA0mln\t\0\xC2\xA0mlrd\x0C\0\xC2\xA0trln") })
                    },
                };
                static BS: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\t\0\x11\0\x19\0\x03\0\xC2\xA0hilj.\x06\0\xC2\xA0mil.\t\0\xC2\xA0mlr.\x0C\0\xC2\xA0bil.") })
                    },
                };
                static SR_LATN: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\t\0\x11\0\x1A\0\x03\0\xC2\xA0hilj.\x06\0\xC2\xA0mil.\t\0\xC2\xA0mlrd.\x0C\0\xC2\xA0bil.") })
                    },
                };
                static FO: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\t\0\x12\0\x1A\0\x03\0\xC2\xA0t\xC3\xBAs.\x06\0\xC2\xA0mi\xC3\xB3.\t\0\xC2\xA0mia.\x0C\0\xC2\xA0bi\xC3\xB3.") })
                    },
                };
                static AZ: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x03\0\n\0\x12\0\x03\0K\x06\0\xC2\xA0mln\t\0\xC2\xA0mlrd\x0C\0\xC2\xA0trln") })
                    },
                };
                static HA: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x03\0\x06\0\t\0\x03\0D\x06\0M\t\0B\x0C\0T") })
                    },
                };
                static MS: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x03\0\x06\0\t\0\x03\0K\x06\0J\t\0B\x0C\0T") })
                    },
                };
                static CHR: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x03\0\x06\0\t\0\x03\0K\x06\0M\t\0B\x0C\0T") })
                    },
                };
                static UND: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x03\0\x06\0\t\0\x03\0K\x06\0M\t\0G\x0C\0T") })
                    },
                };
                static GA: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x03\0\x06\0\t\0\x03\0k\x06\0M\t\0B\x0C\0T") })
                    },
                };
                static BR: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x03\0\x06\0\t\0\x03\0k\x06\0M\t\0G\x0C\0T") })
                    },
                };
                static NL: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x03\0\x0B\0\x13\0\x03\0K\x06\0\xC2\xA0mln.\t\0\xC2\xA0mld.\x0C\0\xC2\xA0bln.") })
                    },
                };
                static NO: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x03\0\x0C\0\x14\0\x03\0k\x06\0\xC2\xA0mill.\t\0\xC2\xA0mrd.\x0C\0\xC2\xA0bill.") })
                    },
                };
                static JV: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x04\0\x07\0\n\0\x03\0\xC3\x88\x06\0Y\t\0M\x0C\0T") })
                    },
                };
                static FR_CA: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\n\0\x0F\0\x03\0\xC2\xA0k\x06\0\xC2\xA0M\t\0\xC2\xA0G\x0C\0\xC2\xA0T") })
                    },
                };
                static FR: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\n\0\x10\0\x03\0\xC2\xA0k\x06\0\xC2\xA0M\t\0\xC2\xA0Md\x0C\0\xC2\xA0Bn") })
                    },
                };
                static HU: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\n\0\x11\0\x03\0\xC2\xA0E\x06\0\xC2\xA0M\t\0\xC2\xA0Mrd\x0C\0\xC2\xA0B") })
                    },
                };
                static AF: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\n\0\x11\0\x03\0\xC2\xA0k\x06\0\xC2\xA0m\t\0\xC2\xA0mjd\x0C\0\xC2\xA0bn") })
                    },
                };
                static RO: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\r\0\x15\0\x03\0\xC2\xA0K\x06\0\xC2\xA0mil.\t\0\xC2\xA0mld.\x0C\0\xC2\xA0tril.") })
                    },
                };
                static DA: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\r\0\x15\0\x03\0\xC2\xA0t\x06\0\xC2\xA0mio.\t\0\xC2\xA0mia.\x0C\0\xC2\xA0bio.") })
                    },
                };
                static OR: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\r\0\x15\0\x03\0\xE0\xAC\xB9\x06\0\xE0\xAC\xA8\xE0\xAC\xBF\t\0\xE0\xAC\xAC\xE0\xAC\xBF\x0C\0\xE0\xAC\x9F\xE0\xAD\x8D\xE0\xAC\xB0\xE0\xAC\xBF") })
                    },
                };
                static TA: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\r\0\x15\0\x03\0\xE0\xAE\x86\x06\0\xE0\xAE\xAE\xE0\xAE\xBF\t\0\xE0\xAE\xAA\xE0\xAE\xBF\x0C\0\xE0\xAE\x9F\xE0\xAE\xBF") })
                    },
                };
                static SI: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\r\0\x15\0\x03\x03\xE0\xB6\xAF\x06\x06\xE0\xB6\xB8\xE0\xB7\x92\t\x06\xE0\xB6\xB6\xE0\xB7\x92\x0C\x0F\xE0\xB6\xA7\xE0\xB7\x8A\xE2\x80\x8D\xE0\xB6\xBB\xE0\xB7\x92") })
                    },
                };
                static VI: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\x0B\0\x10\0\x03\0\xC2\xA0N\x06\0\xC2\xA0Tr\t\0\xC2\xA0T\x0C\0\xC2\xA0NT") })
                    },
                };
                static TR: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\x0B\0\x11\0\x03\0\xC2\xA0B\x06\0\xC2\xA0Mn\t\0\xC2\xA0Mr\x0C\0\xC2\xA0Tn") })
                    },
                };
                static ID: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x06\0\x0C\0\x11\0\x03\0\xC2\xA0rb\x06\0\xC2\xA0jt\t\0\xC2\xA0M\x0C\0\xC2\xA0T") })
                    },
                };
                static HE: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x06\0\x0C\0\x12\0\x03\0K\xE2\x80\x8F\x06\0M\xE2\x80\x8F\t\0B\xE2\x80\x8F\x0C\0T\xE2\x80\x8F") })
                    },
                };
                static SV: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x06\0\x0C\0\x12\0\x03\0\xC2\xA0tn\x06\0\xC2\xA0mn\t\0\xC2\xA0md\x0C\0\xC2\xA0bn") })
                    },
                };
                static FF_ADLM: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x06\0\x0C\0\x16\0\x03\0\xF0\x9E\xA4\x93\x06\0\xF0\x9E\xA4\x81\t\0\xF0\x9E\xA4\x81\xF0\x9E\xA4\xB6\x0C\0\xF0\x9E\xA4\x9A") })
                    },
                };
                static FI: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x06\0\x0F\0\x17\0\x03\0\xC2\xA0t.\x06\0\xC2\xA0milj.\t\0\xC2\xA0mrd.\x0C\0\xC2\xA0bilj.") })
                    },
                };
                static KGP: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x07\0\r\0\x13\0\x03\0\xC2\xA0mil\x06\0\xC2\xA0mi\t\0\xC2\xA0bi\x0C\0\xC2\xA0tri") })
                    },
                };
                static YRL: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x07\0\r\0\x13\0\x03\0\xC2\xA0miu\x06\0\xC2\xA0mi\t\0\xC2\xA0bi\x0C\0\xC2\xA0tiri") })
                    },
                };
                static IS: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x07\0\r\0\x14\0\x03\0\xC2\xA0\xC3\xBE.\x06\0\xC2\xA0m.\t\0\xC2\xA0ma.\x0C\0\xC2\xA0bn") })
                    },
                };
                static KEA: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x07\0\x0C\0\x12\0\x03\0\xC2\xA0mil\x06\0\xC2\xA0M\t\0\xC2\xA0MM\x0C\0\xC2\xA0Bi") })
                    },
                };
                static PT_PT: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x07\0\x0C\0\x12\0\x03\0\xC2\xA0mil\x06\0\xC2\xA0M\t\0\xC2\xA0mM\x0C\0\xC2\xA0Bi") })
                    },
                };
                static AM: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x07\0\x0E\0\x15\0\x03\0\xC2\xA0\xE1\x88\xBA\x06\0\xC2\xA0\xE1\x88\x9A\t\0\xC2\xA0\xE1\x89\xA2\x0C\0\xC2\xA0\xE1\x89\xB5") })
                    },
                };
                static TI: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x07\0\x0E\0\x15\0\x03\0\xC2\xA0\xE1\x88\xBD\x06\0\xC2\xA0\xE1\x88\x9A\t\0\xC2\xA0\xE1\x89\xA2\x0C\0\xC2\xA0\xE1\x89\xB5") })
                    },
                };
                static IA: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x07\0\x0E\0\x15\0\x03\0\xC2\xA0mil\x06\0\xC2\xA0mln\t\0\xC2\xA0mld\x0C\0\xC2\xA0bln") })
                    },
                };
                static ET: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x07\0\x0E\0\x15\0\x03\0\xC2\xA0tuh\x06\0\xC2\xA0mln\t\0\xC2\xA0mld\x0C\0\xC2\xA0trln") })
                    },
                };
                static SW_KE: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x08\0\x0B\0\x0E\0\x03\x06elfu\xC2\xA0\x06\x01M\t\x01B\x0C\x01T") })
                    },
                };
                static SC: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x08\0\x0F\0\x16\0\x03\0\xC2\xA0m\xC3\xACg\x06\0\xC2\xA0Mln\t\0\xC2\xA0Mrd\x0C\0\xC2\xA0Bln") })
                    },
                };
                static PL: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x08\0\x0F\0\x16\0\x03\0\xC2\xA0tys.\x06\0\xC2\xA0mln\t\0\xC2\xA0mld\x0C\0\xC2\xA0bln") })
                    },
                };
                static UZ: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x08\0\x0F\0\x17\0\x03\0\xC2\xA0ming\x06\0\xC2\xA0mln\t\0\xC2\xA0mlrd\x0C\0\xC2\xA0trln") })
                    },
                };
                static CS: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x08\0\x10\0\x18\0\x03\0\xC2\xA0tis.\x06\0\xC2\xA0mil.\t\0\xC2\xA0mld.\x0C\0\xC2\xA0bil.") })
                    },
                };
                static HR: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x08\0\x10\0\x18\0\x03\0\xC2\xA0tis.\x06\0\xC2\xA0mil.\t\0\xC2\xA0mlr.\x0C\0\xC2\xA0bil.") })
                    },
                };
                static SL: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x08\0\x10\0\x18\0\x03\0\xC2\xA0tis.\x06\0\xC2\xA0mio.\t\0\xC2\xA0mrd.\x0C\0\xC2\xA0bil.") })
                    },
                };
                static DSB: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x08\0\x10\0\x18\0\x03\0\xC2\xA0tys.\x06\0\xC2\xA0mio.\t\0\xC2\xA0mrd.\x0C\0\xC2\xA0bil.") })
                    },
                };
                static BRX: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x08\0\x10\0\x18\0\x03\0\xE0\xA4\x95\xE0\xA5\x87\x06\0\xE0\xA4\x8F\xE0\xA4\xAE\t\0\xE0\xA4\xAC\xE0\xA4\xBF\x0C\0\xE0\xA4\xA4\xE0\xA4\xBF") })
                    },
                };
                static TE: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x08\0\x10\0\x18\0\x03\0\xE0\xB0\xB5\xE0\xB1\x87\x06\0\xE0\xB0\xAE\xE0\xB0\xBF\t\0\xE0\xB0\xAC\xE0\xB0\xBF\x0C\0\xE0\xB0\x9F\xE0\xB1\x8D\xE0\xB0\xB0\xE0\xB0\xBF") })
                    },
                };
                static KN: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x08\0\x10\0\x18\0\x03\0\xE0\xB2\xB8\xE0\xB2\xBE\x06\0\xE0\xB2\xAE\xE0\xB2\xBF\t\0\xE0\xB2\xAC\xE0\xB2\xBF\x0C\0\xE0\xB2\x9F\xE0\xB3\x8D\xE0\xB2\xB0\xE0\xB2\xBF") })
                    },
                };
                static LT: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0B\0\x13\0\x1C\0\x03\0\xC2\xA0t\xC5\xABkst.\x06\0\xC2\xA0mln.\t\0\xC2\xA0mlrd.\x0C\0\xC2\xA0trln.") })
                    },
                };
                static LV: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0B\0\x14\0\x1E\0\x03\0\xC2\xA0t\xC5\xABkst.\x06\0\xC2\xA0milj.\t\0\xC2\xA0mljrd.\x0C\0\xC2\xA0trilj.") })
                    },
                };
                static EL: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0B\0\x14\0\x1F\0\x03\0\xC2\xA0\xCF\x87\xCE\xB9\xCE\xBB.\x06\0\xC2\xA0\xCE\xB5\xCE\xBA.\t\0\xC2\xA0\xCE\xB4\xCE\xB9\xCF\x83.\x0C\0\xC2\xA0\xCF\x84\xCF\x81\xCE\xB9\xCF\x83.") })
                    },
                };
                static UK: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0B\0\x15\0!\0\x03\0\xC2\xA0\xD1\x82\xD0\xB8\xD1\x81.\x06\0\xC2\xA0\xD0\xBC\xD0\xBB\xD0\xBD\t\0\xC2\xA0\xD0\xBC\xD0\xBB\xD1\x80\xD0\xB4\x0C\0\xC2\xA0\xD1\x82\xD1\x80\xD0\xBB\xD0\xBD") })
                    },
                };
                static BE: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0B\0\x15\0!\0\x03\0\xC2\xA0\xD1\x82\xD1\x8B\xD1\x81.\x06\0\xC2\xA0\xD0\xBC\xD0\xBB\xD0\xBD\t\0\xC2\xA0\xD0\xBC\xD0\xBB\xD1\x80\xD0\xB4\x0C\0\xC2\xA0\xD1\x82\xD1\x80\xD0\xBB\xD0\xBD") })
                    },
                };
                static BG: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0B\0\x16\0#\0\x03\0\xC2\xA0\xD1\x85\xD0\xB8\xD0\xBB.\x06\0\xC2\xA0\xD0\xBC\xD0\xBB\xD0\xBD.\t\0\xC2\xA0\xD0\xBC\xD0\xBB\xD1\x80\xD0\xB4.\x0C\0\xC2\xA0\xD1\x82\xD1\x80\xD0\xBB\xD0\xBD.") })
                    },
                };
                static SR: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0B\0\x16\0#\0\x03\0\xC2\xA0\xD1\x85\xD0\xB8\xD1\x99.\x06\0\xC2\xA0\xD0\xBC\xD0\xB8\xD0\xBB.\t\0\xC2\xA0\xD0\xBC\xD0\xBB\xD1\x80\xD0\xB4.\x0C\0\xC2\xA0\xD0\xB1\xD0\xB8\xD0\xBB.") })
                    },
                };
                static TG: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0B\0\x16\0#\0\x03\0\xC2\xA0\xD2\xB3\xD0\xB7\xD1\x80.\x06\0\xC2\xA0\xD0\xBC\xD0\xBB\xD0\xBD.\t\0\xC2\xA0\xD0\xBC\xD0\xBB\xD1\x80\xD0\xB4.\x0C\0\xC2\xA0\xD1\x82\xD1\x80\xD0\xBB\xD0\xBD.") })
                    },
                };
                static SD: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0C\0\x18\0$\0\x03\0\xC2\xA0\xD9\x87\xD8\xB2\xD8\xA7\xD8\xB1\x06\0\xC2\xA0\xD9\x85\xD9\x84\xD9\x8A\xD9\x86\t\0\xC2\xA0\xD8\xA8\xD9\x84\xD9\x8A\xD9\x86\x0C\0\xC2\xA0\xD9\xBD\xD8\xB1\xD9\x84\xD9\x8A\xD9\x86") })
                    },
                };
                static FA: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0C\0\x1C\0.\0\x03\0\xC2\xA0\xD9\x87\xD8\xB2\xD8\xA7\xD8\xB1\x06\0\xC2\xA0\xD9\x85\xDB\x8C\xD9\x84\xDB\x8C\xD9\x88\xD9\x86\t\0\xC2\xA0\xD9\x85\xDB\x8C\xD9\x84\xDB\x8C\xD8\xA7\xD8\xB1\xD8\xAF\x0C\0\xC2\xA0\xD8\xAA\xD8\xB1\xDB\x8C\xD9\x84\xDB\x8C\xD9\x88\xD9\x86") })
                    },
                };
                static SW: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x05\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x01\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x08\0\x0B\0\x0E\0\x11\0\x03\x06elfu\xC2\xA0\x06\0M\t\0B\x0C\0B\x0C\0T") })
                    },
                };
                static MK: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\x08\t\x0B\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x06\0\0\0\x07\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x01\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0B\0\x16\0\x1C\0)\x002\0;\0\x03\0\xC2\xA0\xD0\xB8\xD0\xBB\xD1\x98.\x06\0\xC2\xA0\xD0\xBC\xD0\xB8\xD0\xBB.\x06\0\xC2\xA0\xD0\x9C\t\0\xC2\xA0\xD0\xBC\xD0\xB8\xD0\xBB\xD1\x98.\t\0\xC2\xA0\xD0\xBC\xD1\x98.\t\0\xC2\xA0\xD0\xBC\xD0\xB8.\x0C\0\xC2\xA0\xD0\xB1\xD0\xB8\xD0\xBB.") })
                    },
                };
                static BS_CYRL: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x04\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\n\0\x14\0 \0\x03\0\xC2\xA0\xD1\x85\xD0\xB8\xD1\x99\x06\0\xC2\xA0\xD0\xBC\xD0\xB8\xD0\xBB\t\0\xC2\xA0\xD0\xBC\xD0\xBB\xD1\x80\xD0\xB4\x0C\0\xC2\xA0\xD0\xB1\xD0\xB8\xD0\xBB") })
                    },
                };
                static ZH: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x04\x08\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0\x04\0\xE4\xB8\x87\x08\0\xE4\xBA\xBF\x0C\0\xE4\xB8\x87\xE4\xBA\xBF") })
                    },
                };
                static YUE_HANS: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x04\x08\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0\x04\0\xE4\xB8\x87\x08\0\xE4\xBA\xBF\x0C\0\xE5\x85\x86") })
                    },
                };
                static ZH_HANT: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x04\x08\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0\x04\0\xE8\x90\xAC\x08\0\xE5\x84\x84\x0C\0\xE5\x85\x86") })
                    },
                };
                static JA: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x04\x08\x0C\x10") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\n\0\x0F\0\x04\0\xE4\xB8\x87\x08\0\xE5\x84\x84\x0C\0\xE5\x85\x86\x10\0\xE4\xBA\xAC") })
                    },
                };
                static IT: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x07\0\x0E\0\x06\0\xC2\xA0Mln\t\0\xC2\xA0Mrd\x0C\0\xC2\xA0Bln") })
                    },
                };
                static DE: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x08\0\x10\0\x06\0\xC2\xA0Mio.\t\0\xC2\xA0Mrd.\x0C\0\xC2\xA0Bio.") })
                    },
                };
                static EU: <icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x06\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x05\0\x06\0\xC2\xA0M\x0C\0\xC2\xA0B") })
                    },
                };
                static VALUES: [&<icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable; 117usize] = [&AF, &AM, &AR, &AS, &AZ, &BE, &BG, &BN, &BR, &BRX, &BS, &BS_CYRL, &CA, &CHR, &CS, &CHR, &DA, &DE, &DSB, &EL, &CHR, &EN_IN, &ES, &ES_419, &ES_MX, &ES_US, &ET, &EU, &FA, &FF_ADLM, &FI, &CHR, &FO, &FR, &FR_CA, &GA, &CHR, &EU, &GU, &HA, &HE, &HI, &HR, &DSB, &HU, &HY, &IA, &ID, &IS, &IT, &JA, &JV, &KA, &KEA, &KGP, &KK, &KM, &KN, &KO, &CHR, &KY, &LO, &LO_U_NU_LAOO, &LT, &LV, &MK, &CHR, &MN, &MR, &MS, &MY, &NE, &NL, &NO, &OR, &PA, &CHR, &PL, &PS, &KGP, &PT_PT, &RO, &BE, &SC, &SD, &SI, &CS, &SL, &CHR, &SQ, &SR, &SR_LATN, &SV, &SW, &SW_KE, &TA, &TE, &TG, &CHR, &TI, &TK, &TO, &TR, &UK, &UND, &UR, &UZ, &UZ_CYRL, &VI, &YRL, &YUE, &YUE_HANS, &ZH, &CHR, &ZH_HANT, &CHR, &CHR];
                static KEYS: [&str; 117usize] = ["af", "am", "ar", "as", "az", "be", "bg", "bn", "br", "brx", "bs", "bs-Cyrl", "ca", "chr", "cs", "cy", "da", "de", "dsb", "el", "en", "en-IN", "es", "es-419", "es-MX", "es-US", "et", "eu", "fa", "ff-Adlm", "fi", "fil", "fo", "fr", "fr-CA", "ga", "gd", "gl", "gu", "ha", "he", "hi", "hr", "hsb", "hu", "hy", "ia", "id", "is", "it", "ja", "jv", "ka", "kea", "kgp", "kk", "km", "kn", "ko", "kok", "ky", "lo", "lo-u-nu-laoo", "lt", "lv", "mk", "ml", "mn", "mr", "ms", "my", "ne", "nl", "no", "or", "pa", "pcm", "pl", "ps", "pt", "pt-PT", "ro", "ru", "sc", "sd", "si", "sk", "sl", "so", "sq", "sr", "sr-Latn", "sv", "sw", "sw-KE", "ta", "te", "tg", "th", "ti", "tk", "to", "tr", "uk", "und", "ur", "uz", "uz-Cyrl", "vi", "yrl", "yue", "yue-Hans", "zh", "zh-HK", "zh-Hant", "zh-MO", "zu"];
                let mut metadata = icu_provider::DataResponseMetadata::default();
                let payload = if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    payload
                } else {
                    const FALLBACKER: icu::locid_transform::fallback::LocaleFallbackerWithConfig<'static> = icu::locid_transform::fallback::LocaleFallbacker::new().for_config(<icu::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.fallback_config());
                    let mut fallback_iterator = FALLBACKER.fallback_for(req.locale.clone());
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
