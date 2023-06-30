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
                static TO: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x05\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x03\0a\x04\0m\x05\0k\x06\0M\t\0P\x0C\0T") })
                    },
                };
                static MY: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x05\x06\x07\n\x0B\x0C\r\x0E") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0\x07\0\0\0\x08\0\0\0\t\0\0\0\n\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\n\0\0\0\0\0\x13\0)\0<\0L\0\\\0q\0\x86\0\xA1\0\xB9\0\x03\0\xC2\xA0\xE1\x80\x91\xE1\x80\xB1\xE1\x80\xAC\xE1\x80\x84\xE1\x80\xBA\x04\0\xC2\xA0\xE1\x80\x9E\xE1\x80\xB1\xE1\x80\xAC\xE1\x80\x84\xE1\x80\xBA\xE1\x80\xB8\x05\0\xC2\xA0\xE1\x80\x9E\xE1\x80\xAD\xE1\x80\x94\xE1\x80\xBA\xE1\x80\xB8\x06\0\xC2\xA0\xE1\x80\x9E\xE1\x80\x94\xE1\x80\xBA\xE1\x80\xB8\x07\0\xC2\xA0\xE1\x80\x80\xE1\x80\xAF\xE1\x80\x8B\xE1\x80\xB1\n\x0E\xE1\x80\x80\xE1\x80\xAF\xE1\x80\x8B\xE1\x80\xB1\xC2\xA0\xC2\xA0\xE1\x80\x91\x0B\x0E\xE1\x80\x80\xE1\x80\xAF\xE1\x80\x8B\xE1\x80\xB1\xC2\xA0\xC2\xA0\xE1\x80\x9E\x0C\x08\xE1\x80\x8B\xE1\x80\xB1\xC2\xA0\xC2\xA0\xE1\x80\x9E\xE1\x80\xAD\xE1\x80\x94\xE1\x80\xBA\xE1\x80\xB8\r\x08\xE1\x80\x8B\xE1\x80\xB1\xC2\xA0\xC2\xA0\xE1\x80\x9E\xE1\x80\x94\xE1\x80\xBA\xE1\x80\xB8\x0E\0\xC2\xA0\xE1\x80\x80\xE1\x80\xB1\xE1\x80\xAC\xE1\x80\x8B\xE1\x80\xAD") })
                    },
                };
                static ES_419: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x06\n\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x05\0\n\0\x0F\0\x19\0\x03\0\xC2\xA0K\x03\0\xC2\xA0k\x06\0\xC2\xA0M\t\0\xC2\xA0mil\xC2\xA0M\x0C\0\xC2\xA0B") })
                    },
                };
                static KM: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x0E\0\x1E\0+\0A\0\x03\0\xE1\x9E\x96\xE1\x9E\xB6\xE1\x9E\x93\xE1\x9F\x8B\x03\0\xC2\xA0\xE1\x9E\x96\xE1\x9E\xB6\xE1\x9E\x93\xE1\x9F\x8B\x06\0\xC2\xA0\xE1\x9E\x9B\xE1\x9E\xB6\xE1\x9E\x93\t\0\xC2\xA0\xE1\x9E\x94\xE1\x9F\x8A\xE1\x9E\xB8\xE1\x9E\x9B\xE1\x9E\xB6\xE1\x9E\x93\x0C\0\xC2\xA0\xE1\x9E\x91\xE1\x9F\x92\xE1\x9E\x9A\xE1\x9E\xB8\xE1\x9E\x9B\xE1\x9E\xB6\xE1\x9E\x93") })
                    },
                };
                static AR: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\x0C\0\x16\0 \0.\0<\0\x03\0\xC2\xA0\xD8\xA2\xD9\x84\xD8\xA7\xD9\x81\x03\0\xC2\xA0\xD8\xA3\xD9\x84\xD9\x81\x03\0\xC2\xA0\xD8\xA3\xD9\x84\xD9\x81\x06\0\xC2\xA0\xD9\x85\xD9\x84\xD9\x8A\xD9\x88\xD9\x86\t\0\xC2\xA0\xD9\x85\xD9\x84\xD9\x8A\xD8\xA7\xD8\xB1\x0C\0\xC2\xA0\xD8\xAA\xD8\xB1\xD9\x84\xD9\x8A\xD9\x88\xD9\x86") })
                    },
                };
                static YUE: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x08\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\n\0\x0F\0\x03\0\xE5\x8D\x83\x04\0\xE8\x90\xAC\x08\0\xE5\x84\x84\x0C\0\xE5\x85\x86") })
                    },
                };
                static KO: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x08\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\n\0\x0F\0\x03\0\xEC\xB2\x9C\x04\0\xEB\xA7\x8C\x08\0\xEC\x96\xB5\x0C\0\xEC\xA1\xB0") })
                    },
                };
                static KK: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\n\0\x11\0\x1B\0'\0\x03\0\xC2\xA0\xD0\xBC\xD1\x8B\xD2\xA3\x03\0\xC2\xA0\xD0\xBC.\x06\0\xC2\xA0\xD0\xBC\xD0\xBB\xD0\xBD\t\0\xC2\xA0\xD0\xBC\xD0\xBB\xD1\x80\xD0\xB4\x0C\0\xC2\xA0\xD1\x82\xD1\x80\xD0\xBB\xD0\xBD") })
                    },
                };
                static LO: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x06\t\x0C\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\r\0\x1A\0*\x007\0S\0\x03\0\xC2\xA0\xE0\xBA\x9E\xE0\xBA\xB1\xE0\xBA\x99\x03\0\xC2\xA0\xE0\xBA\x81\xE0\xBA\xB5\xE0\xBA\x9A\x06\0\xC2\xA0\xE0\xBA\xA5\xE0\xBB\x89\xE0\xBA\xB2\xE0\xBA\x99\t\0\xC2\xA0\xE0\xBA\x95\xE0\xBA\xB7\xE0\xBB\x89\x0C\0\xC2\xA0\xE0\xBA\xA5\xE0\xBB\x89\xE0\xBA\xB2\xE0\xBA\x99\xE0\xBA\xA5\xE0\xBB\x89\xE0\xBA\xB2\xE0\xBA\x99\x0C\0\xE0\xBA\xA5\xE0\xBA\xA5") })
                    },
                };
                static AS: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x06\x08\t\x0B\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0\x07\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x13\0 \x003\0@\0U\0g\0\x03\0\xC2\xA0\xE0\xA6\xB9\xE0\xA6\xBE\xE0\xA6\x9C\xE0\xA6\xBE\xE0\xA7\xB0\x05\0\xC2\xA0\xE0\xA6\xB2\xE0\xA6\xBE\xE0\xA6\x96\x06\0\xC2\xA0\xE0\xA6\xA8\xE0\xA6\xBF\xE0\xA6\xAF\xE0\xA7\x81\xE0\xA6\xA4\x06\0\xC2\xA0\xE0\xA6\xA8\xE0\xA6\xBF\xE0\xA6\x83\t\0\xC2\xA0\xE0\xA6\xB6\xE0\xA6\x83\xC2\xA0\xE0\xA6\x95\xE0\xA7\x8B\xE0\xA6\x83\t\0\xC2\xA0\xE0\xA6\xB6\xE0\xA6\x83\xC2\xA0\xE0\xA6\x95\xE0\xA6\x83\x0C\0\xC2\xA0\xE0\xA6\xB6\xE0\xA6\x83\xC2\xA0\xE0\xA6\xAA\xE0\xA6\x83") })
                    },
                };
                static BN: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x07\n\x0B\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x05\0\0\0\x06\0\0\0\x07\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x01\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\n\0\x14\0\x1E\x000\0@\0H\0\x03\0\xC2\xA0\xE0\xA6\xB9\xE0\xA6\xBE\x05\0\xC2\xA0\xE0\xA6\xB2\xE0\xA6\xBE\x07\0\xC2\xA0\xE0\xA6\x95\xE0\xA7\x8B\t\0\xC2\xA0\xE0\xA6\xB6\xE0\xA6\xA4\xC2\xA0\xE0\xA6\x95\xE0\xA7\x8B\t\0\xE0\xA6\xB6\xE0\xA6\xA4\xC2\xA0\xE0\xA6\x95\xE0\xA7\x8B\t\0\xE0\xA6\x95\xE0\xA7\x8B\x0C\0\xC2\xA0\xE0\xA6\xB2\xE0\xA6\xBE.\xE0\xA6\x95\xE0\xA7\x8B.") })
                    },
                };
                static EN_IN: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x07\n\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x03\0\x06\0\n\0\x0F\0\x03\0T\x05\0L\x07\0Cr\n\0TCr\x0C\0LCr") })
                    },
                };
                static MR: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x07\t\x0B\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\x07\0\x14\0$\x004\0D\0\x03\0\xC2\xA0\xE0\xA4\xB9\x05\0\xC2\xA0\xE0\xA4\xB2\xE0\xA4\xBE\xE0\xA4\x96\x07\0\xC2\xA0\xE0\xA4\x95\xE0\xA5\x8B\xE0\xA4\x9F\xE0\xA5\x80\t\0\xC2\xA0\xE0\xA4\x85\xE0\xA4\xAC\xE0\xA5\x8D\xE0\xA4\x9C\x0B\0\xC2\xA0\xE0\xA4\x96\xE0\xA4\xB0\xE0\xA5\x8D\xE0\xA4\xB5\r\0\xC2\xA0\xE0\xA4\xAA\xE0\xA4\xA6\xE0\xA5\x8D\xE0\xA4\xAE") })
                    },
                };
                static UR: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x07\t\x0B\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\x0C\0\x18\0$\0.\0:\0\x03\0\xC2\xA0\xDB\x81\xD8\xB2\xD8\xA7\xD8\xB1\x05\0\xC2\xA0\xD9\x84\xD8\xA7\xDA\xA9\xDA\xBE\x07\0\xC2\xA0\xDA\xA9\xD8\xB1\xD9\x88\xDA\x91\t\0\xC2\xA0\xD8\xA7\xD8\xB1\xD8\xA8\x0B\0\xC2\xA0\xDA\xA9\xDA\xBE\xD8\xB1\xD8\xA8\x0C\0\xC2\xA0\xD9\xB9\xD8\xB1\xDB\x8C\xD9\x84\xDB\x8C\xD9\x86") })
                    },
                };
                static NE: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x07\t\x0B\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\x10\0\x1D\0-\0:\0G\0\x03\0\xC2\xA0\xE0\xA4\xB9\xE0\xA4\x9C\xE0\xA4\xBE\xE0\xA4\xB0\x05\0\xC2\xA0\xE0\xA4\xB2\xE0\xA4\xBE\xE0\xA4\x96\x07\0\xC2\xA0\xE0\xA4\x95\xE0\xA4\xB0\xE0\xA5\x8B\xE0\xA4\xA1\t\0\xC2\xA0\xE0\xA4\x85\xE0\xA4\xB0\xE0\xA4\xAC\x0B\0\xC2\xA0\xE0\xA4\x96\xE0\xA4\xB0\xE0\xA4\xAC\r\0\xC2\xA0\xE0\xA4\xB6\xE0\xA4\x82\xE0\xA4\x96") })
                    },
                };
                static HI: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x07\t\x0B\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\x13\0 \0*\x004\0>\0\x03\0\xC2\xA0\xE0\xA4\xB9\xE0\xA4\x9C\xE0\xA4\xBC\xE0\xA4\xBE\xE0\xA4\xB0\x05\0\xC2\xA0\xE0\xA4\xB2\xE0\xA4\xBE\xE0\xA4\x96\x07\0\xC2\xA0\xE0\xA4\x95\xE0\xA5\xB0\t\0\xC2\xA0\xE0\xA4\x85\xE0\xA5\xB0\x0B\0\xC2\xA0\xE0\xA4\x96\xE0\xA5\xB0\r\0\xC2\xA0\xE0\xA4\xA8\xE0\xA5\x80\xE0\xA4\xB2") })
                    },
                };
                static PA: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x07\t\x0B\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\x13\0 \x000\0=\0J\0\x03\0\xC2\xA0\xE0\xA8\xB9\xE0\xA8\x9C\xE0\xA8\xBC\xE0\xA8\xBE\xE0\xA8\xB0\x05\0\xC2\xA0\xE0\xA8\xB2\xE0\xA9\xB1\xE0\xA8\x96\x07\0\xC2\xA0\xE0\xA8\x95\xE0\xA8\xB0\xE0\xA9\x8B\xE0\xA9\x9C\t\0\xC2\xA0\xE0\xA8\x85\xE0\xA8\xB0\xE0\xA8\xAC\x0B\0\xC2\xA0\xE0\xA8\x96\xE0\xA8\xB0\xE0\xA8\xAC\r\0\xC2\xA0\xE0\xA8\xA8\xE0\xA9\x80\xE0\xA8\xB2") })
                    },
                };
                static GU: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x07\t\x0B\x0C\r\x0E") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0\x07\0\0\0\x08\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x08\0\0\0\0\0\x10\0\x1D\0-\0:\0P\0i\0y\0\x03\0\xC2\xA0\xE0\xAA\xB9\xE0\xAA\x9C\xE0\xAA\xBE\xE0\xAA\xB0\x05\0\xC2\xA0\xE0\xAA\xB2\xE0\xAA\xBE\xE0\xAA\x96\x07\0\xC2\xA0\xE0\xAA\x95\xE0\xAA\xB0\xE0\xAB\x8B\xE0\xAA\xA1\t\0\xC2\xA0\xE0\xAA\x85\xE0\xAA\xAC\xE0\xAA\x9C\x0B\0\xC2\xA0\xE0\xAA\xA8\xE0\xAA\xBF\xE0\xAA\x96\xE0\xAA\xB0\xE0\xAB\x8D\xE0\xAA\xB5\x0C\0\xC2\xA0\xE0\xAA\xAE\xE0\xAA\xB9\xE0\xAA\xBE\xE0\xAA\xAA\xE0\xAA\xA6\xE0\xAB\x8D\xE0\xAA\xAE\r\0\xC2\xA0\xE0\xAA\xB6\xE0\xAA\x82\xE0\xAA\x95\xE0\xAB\x81\x0E\0\xC2\xA0\xE0\xAA\x9C\xE0\xAA\xB2\xE0\xAA\xA7\xE0\xAA\xBF") })
                    },
                };
                static CA: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\n\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\n\0\x10\0\x03\0\xC2\xA0k\x06\0\xC2\xA0M\t\0\xC2\xA0kM\x0C\0\xC2\xA0B") })
                    },
                };
                static ES_US: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\n\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\n\0\x14\0\x03\0\xC2\xA0K\x06\0\xC2\xA0M\t\0\xC2\xA0mil\xC2\xA0M\x0C\0\xC2\xA0B") })
                    },
                };
                static ES_MX: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\n\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\n\0\x14\0\x03\0\xC2\xA0k\x06\0\xC2\xA0M\t\0\xC2\xA0mil\xC2\xA0M\x0C\0\xC2\xA0B") })
                    },
                };
                static ES: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\n\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x07\0\x0C\0\x16\0\x03\0\xC2\xA0mil\x06\0\xC2\xA0M\t\0\xC2\xA0mil\xC2\xA0M\x0C\0\xC2\xA0B") })
                    },
                };
                static LO_U_NU_LAOO: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0B\0\x19\0$\0\x03\0\xE0\xBA\x9E\xE0\xBA\xB1\xE0\xBA\x99\x06\0\xE0\xBA\xA5\xE0\xBB\x89\xE0\xBA\xB2\xE0\xBA\x99\t\0\xE0\xBA\x95\xE0\xBA\xB7\xE0\xBB\x89\x0C\0\xE0\xBA\x9E\xE0\xBA\xB1\xE0\xBA\x99\xE0\xBA\x95\xE0\xBA\xB7\xE0\xBB\x89") })
                    },
                };
                static KA: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0B\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x0B\0\x19\0*\08\0\x03\0\xC2\xA0\xE1\x83\x90\xE1\x83\x97.\x06\0\xC2\xA0\xE1\x83\x9B\xE1\x83\x9A\xE1\x83\x9C.\t\0\xC2\xA0\xE1\x83\x9B\xE1\x83\x9A\xE1\x83\xA0\xE1\x83\x93.\t\0\xC2\xA0\xE1\x83\x9B\xE1\x83\x9A\xE1\x83\xA0.\x0C\0\xC2\xA0\xE1\x83\xA2\xE1\x83\xA0\xE1\x83\x9A.") })
                    },
                };
                static MN: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0B\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x0E\0\x18\0(\0,\0\x03\0\xC2\xA0\xD0\xBC\xD1\x8F\xD0\xBD\xD0\xB3\xD0\xB0\x06\0\xC2\xA0\xD1\x81\xD0\xB0\xD1\x8F\t\0\xC2\xA0\xD1\x82\xD1\x8D\xD1\x80\xD0\xB1\xD1\x83\xD0\xBC\t\0\xD0\xA2\x0C\0\xD0\x98\xD0\x9D") })
                    },
                };
                static PS: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0B\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x05\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x01\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x03\0K\x06\0M\t\0B\t\0G\t\0B\x0C\0T") })
                    },
                };
                static UZ_CYRL: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\n\0\x12\0\x1C\0\x03\0\xD0\xBC\xD0\xB8\xD0\xBD\xD0\xB3\x06\0\xD0\xBC\xD0\xBB\xD0\xBD\t\0\xD0\xBC\xD0\xBB\xD1\x80\xD0\xB4\x0C\0\xD1\x82\xD1\x80\xD0\xBB\xD0\xBD") })
                    },
                };
                static HY: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\n\0\x14\0 \0\x03\0\xC2\xA0\xD5\xB0\xD5\xA6\xD6\x80\x06\0\xC2\xA0\xD5\xB4\xD5\xAC\xD5\xB6\t\0\xC2\xA0\xD5\xB4\xD5\xAC\xD6\x80\xD5\xA4\x0C\0\xC2\xA0\xD5\xBF\xD6\x80\xD5\xAC\xD5\xB6") })
                    },
                };
                static KY: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\n\0\x14\0\x1E\0\x03\0\xC2\xA0\xD0\xBC\xD0\xB8\xD2\xA3\x06\0\xC2\xA0\xD0\xBC\xD0\xBB\xD0\xBD\t\0\xC2\xA0\xD0\xBC\xD0\xBB\xD0\xB4\x0C\0\xC2\xA0\xD1\x82\xD1\x80\xD0\xBB\xD0\xBD") })
                    },
                };
                static SQ: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\t\0\x10\0\x17\0\x03\0\xC2\xA0mij\xC3\xAB\x06\0\xC2\xA0mln\t\0\xC2\xA0mld\x0C\0\xC2\xA0bln") })
                    },
                };
                static TK: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\t\0\x10\0\x18\0\x03\0\xC2\xA0m\xC3\xBC\xC5\x88\x06\0\xC2\xA0mln\t\0\xC2\xA0mlrd\x0C\0\xC2\xA0trln") })
                    },
                };
                static BS: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\t\0\x11\0\x19\0\x03\0\xC2\xA0hilj.\x06\0\xC2\xA0mil.\t\0\xC2\xA0mlr.\x0C\0\xC2\xA0bil.") })
                    },
                };
                static SR_LATN: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\t\0\x11\0\x1A\0\x03\0\xC2\xA0hilj.\x06\0\xC2\xA0mil.\t\0\xC2\xA0mlrd.\x0C\0\xC2\xA0bil.") })
                    },
                };
                static FO: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\t\0\x12\0\x1A\0\x03\0\xC2\xA0t\xC3\xBAs.\x06\0\xC2\xA0mi\xC3\xB3.\t\0\xC2\xA0mia.\x0C\0\xC2\xA0bi\xC3\xB3.") })
                    },
                };
                static AZ: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x03\0\n\0\x12\0\x03\0K\x06\0\xC2\xA0mln\t\0\xC2\xA0mlrd\x0C\0\xC2\xA0trln") })
                    },
                };
                static HA: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x03\0\x06\0\t\0\x03\0D\x06\0M\t\0B\x0C\0T") })
                    },
                };
                static MS: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x03\0\x06\0\t\0\x03\0K\x06\0J\t\0B\x0C\0T") })
                    },
                };
                static CHR: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x03\0\x06\0\t\0\x03\0K\x06\0M\t\0B\x0C\0T") })
                    },
                };
                static AST: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x03\0\x06\0\t\0\x03\0K\x06\0M\t\0G\x0C\0T") })
                    },
                };
                static GA: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x03\0\x06\0\t\0\x03\0k\x06\0M\t\0B\x0C\0T") })
                    },
                };
                static BR: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x03\0\x06\0\t\0\x03\0k\x06\0M\t\0G\x0C\0T") })
                    },
                };
                static NL: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x03\0\x0B\0\x13\0\x03\0K\x06\0\xC2\xA0mln.\t\0\xC2\xA0mld.\x0C\0\xC2\xA0bln.") })
                    },
                };
                static NB: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x03\0\x0C\0\x14\0\x03\0k\x06\0\xC2\xA0mill.\t\0\xC2\xA0mrd.\x0C\0\xC2\xA0bill.") })
                    },
                };
                static JV: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x04\0\x07\0\n\0\x03\0\xC3\x88\x06\0Y\t\0M\x0C\0T") })
                    },
                };
                static FR_CA: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\n\0\x0F\0\x03\0\xC2\xA0k\x06\0\xC2\xA0M\t\0\xC2\xA0G\x0C\0\xC2\xA0T") })
                    },
                };
                static FR: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\n\0\x10\0\x03\0\xC2\xA0k\x06\0\xC2\xA0M\t\0\xC2\xA0Md\x0C\0\xC2\xA0Bn") })
                    },
                };
                static HU: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\n\0\x11\0\x03\0\xC2\xA0E\x06\0\xC2\xA0M\t\0\xC2\xA0Mrd\x0C\0\xC2\xA0B") })
                    },
                };
                static AF: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\n\0\x11\0\x03\0\xC2\xA0k\x06\0\xC2\xA0m\t\0\xC2\xA0mjd\x0C\0\xC2\xA0bn") })
                    },
                };
                static RO: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\r\0\x15\0\x03\0\xC2\xA0K\x06\0\xC2\xA0mil.\t\0\xC2\xA0mld.\x0C\0\xC2\xA0tril.") })
                    },
                };
                static DA: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\r\0\x15\0\x03\0\xC2\xA0t\x06\0\xC2\xA0mio.\t\0\xC2\xA0mia.\x0C\0\xC2\xA0bio.") })
                    },
                };
                static OR: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\r\0\x15\0\x03\0\xE0\xAC\xB9\x06\0\xE0\xAC\xA8\xE0\xAC\xBF\t\0\xE0\xAC\xAC\xE0\xAC\xBF\x0C\0\xE0\xAC\x9F\xE0\xAD\x8D\xE0\xAC\xB0\xE0\xAC\xBF") })
                    },
                };
                static TA: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\r\0\x15\0\x03\0\xE0\xAE\x86\x06\0\xE0\xAE\xAE\xE0\xAE\xBF\t\0\xE0\xAE\xAA\xE0\xAE\xBF\x0C\0\xE0\xAE\x9F\xE0\xAE\xBF") })
                    },
                };
                static SI: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\r\0\x15\0\x03\x03\xE0\xB6\xAF\x06\x06\xE0\xB6\xB8\xE0\xB7\x92\t\x06\xE0\xB6\xB6\xE0\xB7\x92\x0C\x0F\xE0\xB6\xA7\xE0\xB7\x8A\xE2\x80\x8D\xE0\xB6\xBB\xE0\xB7\x92") })
                    },
                };
                static VI: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\x0B\0\x10\0\x03\0\xC2\xA0N\x06\0\xC2\xA0Tr\t\0\xC2\xA0T\x0C\0\xC2\xA0NT") })
                    },
                };
                static TR: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\x0B\0\x11\0\x03\0\xC2\xA0B\x06\0\xC2\xA0Mn\t\0\xC2\xA0Mr\x0C\0\xC2\xA0Tn") })
                    },
                };
                static ID: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x06\0\x0C\0\x11\0\x03\0\xC2\xA0rb\x06\0\xC2\xA0jt\t\0\xC2\xA0M\x0C\0\xC2\xA0T") })
                    },
                };
                static HE: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x06\0\x0C\0\x12\0\x03\0K\xE2\x80\x8F\x06\0M\xE2\x80\x8F\t\0B\xE2\x80\x8F\x0C\0T\xE2\x80\x8F") })
                    },
                };
                static SV: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x06\0\x0C\0\x12\0\x03\0\xC2\xA0tn\x06\0\xC2\xA0mn\t\0\xC2\xA0md\x0C\0\xC2\xA0bn") })
                    },
                };
                static FF_ADLM: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x06\0\x0C\0\x16\0\x03\0\xF0\x9E\xA4\x93\x06\0\xF0\x9E\xA4\x81\t\0\xF0\x9E\xA4\x81\xF0\x9E\xA4\xB6\x0C\0\xF0\x9E\xA4\x9A") })
                    },
                };
                static FI: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x06\0\x0F\0\x17\0\x03\0\xC2\xA0t.\x06\0\xC2\xA0milj.\t\0\xC2\xA0mrd.\x0C\0\xC2\xA0bilj.") })
                    },
                };
                static KGP: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x07\0\r\0\x13\0\x03\0\xC2\xA0mil\x06\0\xC2\xA0mi\t\0\xC2\xA0bi\x0C\0\xC2\xA0tri") })
                    },
                };
                static YRL: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x07\0\r\0\x13\0\x03\0\xC2\xA0miu\x06\0\xC2\xA0mi\t\0\xC2\xA0bi\x0C\0\xC2\xA0tiri") })
                    },
                };
                static IS: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x07\0\r\0\x14\0\x03\0\xC2\xA0\xC3\xBE.\x06\0\xC2\xA0m.\t\0\xC2\xA0ma.\x0C\0\xC2\xA0bn") })
                    },
                };
                static KEA: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x07\0\x0C\0\x12\0\x03\0\xC2\xA0mil\x06\0\xC2\xA0M\t\0\xC2\xA0MM\x0C\0\xC2\xA0Bi") })
                    },
                };
                static PT_AO: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x07\0\x0C\0\x12\0\x03\0\xC2\xA0mil\x06\0\xC2\xA0M\t\0\xC2\xA0mM\x0C\0\xC2\xA0Bi") })
                    },
                };
                static AM: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x07\0\x0E\0\x15\0\x03\0\xC2\xA0\xE1\x88\xBA\x06\0\xC2\xA0\xE1\x88\x9A\t\0\xC2\xA0\xE1\x89\xA2\x0C\0\xC2\xA0\xE1\x89\xB5") })
                    },
                };
                static TI: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x07\0\x0E\0\x15\0\x03\0\xC2\xA0\xE1\x88\xBD\x06\0\xC2\xA0\xE1\x88\x9A\t\0\xC2\xA0\xE1\x89\xA2\x0C\0\xC2\xA0\xE1\x89\xB5") })
                    },
                };
                static IA: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x07\0\x0E\0\x15\0\x03\0\xC2\xA0mil\x06\0\xC2\xA0mln\t\0\xC2\xA0mld\x0C\0\xC2\xA0bln") })
                    },
                };
                static ET: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x07\0\x0E\0\x15\0\x03\0\xC2\xA0tuh\x06\0\xC2\xA0mln\t\0\xC2\xA0mld\x0C\0\xC2\xA0trln") })
                    },
                };
                static SW_KE: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x08\0\x0B\0\x0E\0\x03\x06elfu\xC2\xA0\x06\x01M\t\x01B\x0C\x01T") })
                    },
                };
                static SC: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x08\0\x0F\0\x16\0\x03\0\xC2\xA0m\xC3\xACg\x06\0\xC2\xA0Mln\t\0\xC2\xA0Mrd\x0C\0\xC2\xA0Bln") })
                    },
                };
                static PL: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x08\0\x0F\0\x16\0\x03\0\xC2\xA0tys.\x06\0\xC2\xA0mln\t\0\xC2\xA0mld\x0C\0\xC2\xA0bln") })
                    },
                };
                static UZ: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x08\0\x0F\0\x17\0\x03\0\xC2\xA0ming\x06\0\xC2\xA0mln\t\0\xC2\xA0mlrd\x0C\0\xC2\xA0trln") })
                    },
                };
                static CS: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x08\0\x10\0\x18\0\x03\0\xC2\xA0tis.\x06\0\xC2\xA0mil.\t\0\xC2\xA0mld.\x0C\0\xC2\xA0bil.") })
                    },
                };
                static HR: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x08\0\x10\0\x18\0\x03\0\xC2\xA0tis.\x06\0\xC2\xA0mil.\t\0\xC2\xA0mlr.\x0C\0\xC2\xA0bil.") })
                    },
                };
                static SL: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x08\0\x10\0\x18\0\x03\0\xC2\xA0tis.\x06\0\xC2\xA0mio.\t\0\xC2\xA0mrd.\x0C\0\xC2\xA0bil.") })
                    },
                };
                static DSB: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x08\0\x10\0\x18\0\x03\0\xC2\xA0tys.\x06\0\xC2\xA0mio.\t\0\xC2\xA0mrd.\x0C\0\xC2\xA0bil.") })
                    },
                };
                static BRX: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x08\0\x10\0\x18\0\x03\0\xE0\xA4\x95\xE0\xA5\x87\x06\0\xE0\xA4\x8F\xE0\xA4\xAE\t\0\xE0\xA4\xAC\xE0\xA4\xBF\x0C\0\xE0\xA4\xA4\xE0\xA4\xBF") })
                    },
                };
                static TE: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x08\0\x10\0\x18\0\x03\0\xE0\xB0\xB5\xE0\xB1\x87\x06\0\xE0\xB0\xAE\xE0\xB0\xBF\t\0\xE0\xB0\xAC\xE0\xB0\xBF\x0C\0\xE0\xB0\x9F\xE0\xB1\x8D\xE0\xB0\xB0\xE0\xB0\xBF") })
                    },
                };
                static KN: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x08\0\x10\0\x18\0\x03\0\xE0\xB2\xB8\xE0\xB2\xBE\x06\0\xE0\xB2\xAE\xE0\xB2\xBF\t\0\xE0\xB2\xAC\xE0\xB2\xBF\x0C\0\xE0\xB2\x9F\xE0\xB3\x8D\xE0\xB2\xB0\xE0\xB2\xBF") })
                    },
                };
                static LT: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0B\0\x13\0\x1C\0\x03\0\xC2\xA0t\xC5\xABkst.\x06\0\xC2\xA0mln.\t\0\xC2\xA0mlrd.\x0C\0\xC2\xA0trln.") })
                    },
                };
                static LV: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0B\0\x14\0\x1E\0\x03\0\xC2\xA0t\xC5\xABkst.\x06\0\xC2\xA0milj.\t\0\xC2\xA0mljrd.\x0C\0\xC2\xA0trilj.") })
                    },
                };
                static EL: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0B\0\x14\0\x1F\0\x03\0\xC2\xA0\xCF\x87\xCE\xB9\xCE\xBB.\x06\0\xC2\xA0\xCE\xB5\xCE\xBA.\t\0\xC2\xA0\xCE\xB4\xCE\xB9\xCF\x83.\x0C\0\xC2\xA0\xCF\x84\xCF\x81\xCE\xB9\xCF\x83.") })
                    },
                };
                static UK: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0B\0\x15\0!\0\x03\0\xC2\xA0\xD1\x82\xD0\xB8\xD1\x81.\x06\0\xC2\xA0\xD0\xBC\xD0\xBB\xD0\xBD\t\0\xC2\xA0\xD0\xBC\xD0\xBB\xD1\x80\xD0\xB4\x0C\0\xC2\xA0\xD1\x82\xD1\x80\xD0\xBB\xD0\xBD") })
                    },
                };
                static BE: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0B\0\x15\0!\0\x03\0\xC2\xA0\xD1\x82\xD1\x8B\xD1\x81.\x06\0\xC2\xA0\xD0\xBC\xD0\xBB\xD0\xBD\t\0\xC2\xA0\xD0\xBC\xD0\xBB\xD1\x80\xD0\xB4\x0C\0\xC2\xA0\xD1\x82\xD1\x80\xD0\xBB\xD0\xBD") })
                    },
                };
                static BG: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0B\0\x16\0#\0\x03\0\xC2\xA0\xD1\x85\xD0\xB8\xD0\xBB.\x06\0\xC2\xA0\xD0\xBC\xD0\xBB\xD0\xBD.\t\0\xC2\xA0\xD0\xBC\xD0\xBB\xD1\x80\xD0\xB4.\x0C\0\xC2\xA0\xD1\x82\xD1\x80\xD0\xBB\xD0\xBD.") })
                    },
                };
                static SR: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0B\0\x16\0#\0\x03\0\xC2\xA0\xD1\x85\xD0\xB8\xD1\x99.\x06\0\xC2\xA0\xD0\xBC\xD0\xB8\xD0\xBB.\t\0\xC2\xA0\xD0\xBC\xD0\xBB\xD1\x80\xD0\xB4.\x0C\0\xC2\xA0\xD0\xB1\xD0\xB8\xD0\xBB.") })
                    },
                };
                static TG: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0B\0\x16\0#\0\x03\0\xC2\xA0\xD2\xB3\xD0\xB7\xD1\x80.\x06\0\xC2\xA0\xD0\xBC\xD0\xBB\xD0\xBD.\t\0\xC2\xA0\xD0\xBC\xD0\xBB\xD1\x80\xD0\xB4.\x0C\0\xC2\xA0\xD1\x82\xD1\x80\xD0\xBB\xD0\xBD.") })
                    },
                };
                static SD: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0C\0\x18\0$\0\x03\0\xC2\xA0\xD9\x87\xD8\xB2\xD8\xA7\xD8\xB1\x06\0\xC2\xA0\xD9\x85\xD9\x84\xD9\x8A\xD9\x86\t\0\xC2\xA0\xD8\xA8\xD9\x84\xD9\x8A\xD9\x86\x0C\0\xC2\xA0\xD9\xBD\xD8\xB1\xD9\x84\xD9\x8A\xD9\x86") })
                    },
                };
                static FA: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0C\0\x1C\0.\0\x03\0\xC2\xA0\xD9\x87\xD8\xB2\xD8\xA7\xD8\xB1\x06\0\xC2\xA0\xD9\x85\xDB\x8C\xD9\x84\xDB\x8C\xD9\x88\xD9\x86\t\0\xC2\xA0\xD9\x85\xDB\x8C\xD9\x84\xDB\x8C\xD8\xA7\xD8\xB1\xD8\xAF\x0C\0\xC2\xA0\xD8\xAA\xD8\xB1\xDB\x8C\xD9\x84\xDB\x8C\xD9\x88\xD9\x86") })
                    },
                };
                static SW: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x05\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x01\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x08\0\x0B\0\x0E\0\x11\0\x03\x06elfu\xC2\xA0\x06\0M\t\0B\x0C\0B\x0C\0T") })
                    },
                };
                static MK: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\x08\t\x0B\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x06\0\0\0\x07\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x01\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0B\0\x16\0\x1C\0)\x002\0;\0\x03\0\xC2\xA0\xD0\xB8\xD0\xBB\xD1\x98.\x06\0\xC2\xA0\xD0\xBC\xD0\xB8\xD0\xBB.\x06\0\xC2\xA0\xD0\x9C\t\0\xC2\xA0\xD0\xBC\xD0\xB8\xD0\xBB\xD1\x98.\t\0\xC2\xA0\xD0\xBC\xD1\x98.\t\0\xC2\xA0\xD0\xBC\xD0\xB8.\x0C\0\xC2\xA0\xD0\xB1\xD0\xB8\xD0\xBB.") })
                    },
                };
                static BS_CYRL: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x04\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\n\0\x14\0 \0\x03\0\xC2\xA0\xD1\x85\xD0\xB8\xD1\x99\x06\0\xC2\xA0\xD0\xBC\xD0\xB8\xD0\xBB\t\0\xC2\xA0\xD0\xBC\xD0\xBB\xD1\x80\xD0\xB4\x0C\0\xC2\xA0\xD0\xB1\xD0\xB8\xD0\xBB") })
                    },
                };
                static ZH: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x04\x08\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0\x04\0\xE4\xB8\x87\x08\0\xE4\xBA\xBF\x0C\0\xE4\xB8\x87\xE4\xBA\xBF") })
                    },
                };
                static YUE_HANS: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x04\x08\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0\x04\0\xE4\xB8\x87\x08\0\xE4\xBA\xBF\x0C\0\xE5\x85\x86") })
                    },
                };
                static ZH_HANT: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x04\x08\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0\x04\0\xE8\x90\xAC\x08\0\xE5\x84\x84\x0C\0\xE5\x85\x86") })
                    },
                };
                static JA: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x04\x08\x0C\x10") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\n\0\x0F\0\x04\0\xE4\xB8\x87\x08\0\xE5\x84\x84\x0C\0\xE5\x85\x86\x10\0\xE4\xBA\xAC") })
                    },
                };
                static IT: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x07\0\x0E\0\x06\0\xC2\xA0Mln\t\0\xC2\xA0Mrd\x0C\0\xC2\xA0Bln") })
                    },
                };
                static DE: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x08\0\x10\0\x06\0\xC2\xA0Mio.\t\0\xC2\xA0Mrd.\x0C\0\xC2\xA0Bio.") })
                    },
                };
                static EU: <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x06\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x05\0\x06\0\xC2\xA0M\x0C\0\xC2\xA0B") })
                    },
                };
                static VALUES: [&<icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable; 538usize] = [&AF, &AF, &AM, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AS, &AS, &AST, &AZ, &AZ, &BE, &BE, &BG, &AST, &AST, &AST, &AST, &BN, &BN, &BN, &BN, &BR, &BRX, &BRX, &BS, &BS_CYRL, &BS, &CA, &CA, &CA, &CA, &CA, &AST, &CHR, &CS, &AST, &CHR, &DA, &DA, &DE, &DE, &DE, &DE, &DE, &DE, &DE, &AST, &AST, &DSB, &EL, &EL, &EL, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &EN_IN, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &CHR, &ES, &ES_419, &ES_419, &ES_419, &ES_419, &ES_419, &ES_419, &ES_419, &ES_419, &ES_419, &ES_419, &ES, &ES_419, &ES, &ES_419, &ES_419, &ES, &ES_MX, &ES_419, &ES_419, &ES_419, &ES, &ES_419, &ES_419, &ES_419, &ES_US, &ES_419, &ES_419, &ET, &EU, &FA, &FA, &FA, &FA, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FI, &CHR, &FO, &FO, &FR, &FR, &FR, &FR, &FR, &FR, &FR_CA, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &GA, &GA, &CHR, &EU, &GU, &GU, &HA, &HA, &HA, &HE, &HI, &EN_IN, &HI, &HR, &HR, &DSB, &HU, &HY, &IA, &ID, &AST, &IS, &IT, &IT, &IT, &IT, &JA, &JV, &JV, &KA, &KEA, &KGP, &KK, &KM, &KM, &KN, &KN, &KO, &KO, &CHR, &CHR, &AST, &AST, &AST, &AST, &AST, &KY, &LO, &LO_U_NU_LAOO, &LT, &LV, &AST, &AST, &AST, &MK, &CHR, &CHR, &MN, &AST, &AST, &AST, &AST, &MR, &MR, &MS, &MS, &MS, &MS, &MY, &MY, &NB, &NB, &NE, &NE, &NE, &NE, &NL, &NL, &NL, &NL, &NL, &NL, &NL, &NB, &NB, &OR, &OR, &PA, &PA, &PA, &PA, &CHR, &PL, &PS, &PS, &PS, &PS, &KGP, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &AST, &AST, &AST, &AST, &AST, &AST, &RO, &RO, &BE, &BE, &BE, &BE, &BE, &BE, &AST, &AST, &AST, &AST, &AST, &AST, &SC, &SD, &SD, &SD, &AST, &SD, &SI, &CS, &SL, &CHR, &CHR, &CHR, &CHR, &SQ, &SQ, &SQ, &SR, &SR, &SR, &SR, &SR_LATN, &SR_LATN, &SR_LATN, &SR_LATN, &AST, &AST, &SV, &SV, &SV, &SW, &SW, &SW_KE, &SW, &TA, &TA, &TA, &TA, &TA, &TA, &TA, &TA, &TE, &TE, &TG, &CHR, &CHR, &TI, &TI, &TK, &TO, &TR, &TR, &AST, &UK, &AST, &UR, &UR, &UR, &UR, &UZ, &UZ_CYRL, &UZ, &VI, &AST, &AST, &AST, &AST, &YRL, &YRL, &YRL, &YUE, &YUE_HANS, &YUE_HANS, &YUE, &YUE, &YUE, &ZH, &ZH, &ZH, &ZH, &ZH, &ZH_HANT, &CHR, &CHR, &CHR, &CHR, &ZH_HANT, &ZH, &CHR];
                static KEYS: [&str; 538usize] = ["af", "af-NA", "am", "ar", "ar-AE", "ar-AE-u-nu-arab", "ar-BH", "ar-BH-u-nu-latn", "ar-DJ", "ar-DJ-u-nu-latn", "ar-DZ", "ar-DZ-u-nu-arab", "ar-EG", "ar-EG-u-nu-latn", "ar-EH", "ar-EH-u-nu-arab", "ar-ER", "ar-ER-u-nu-latn", "ar-IL", "ar-IL-u-nu-latn", "ar-IQ", "ar-IQ-u-nu-latn", "ar-JO", "ar-JO-u-nu-latn", "ar-KM", "ar-KM-u-nu-latn", "ar-KW", "ar-KW-u-nu-latn", "ar-LB", "ar-LB-u-nu-latn", "ar-LY", "ar-LY-u-nu-arab", "ar-MA", "ar-MA-u-nu-arab", "ar-MR", "ar-MR-u-nu-latn", "ar-OM", "ar-OM-u-nu-latn", "ar-PS", "ar-PS-u-nu-latn", "ar-QA", "ar-QA-u-nu-latn", "ar-SA", "ar-SA-u-nu-latn", "ar-SD", "ar-SD-u-nu-latn", "ar-SO", "ar-SO-u-nu-latn", "ar-SS", "ar-SS-u-nu-latn", "ar-SY", "ar-SY-u-nu-latn", "ar-TD", "ar-TD-u-nu-latn", "ar-TN", "ar-TN-u-nu-arab", "ar-YE", "ar-YE-u-nu-latn", "ar-u-nu-latn", "as", "as-u-nu-latn", "ast", "az", "az-Latn", "be", "be-tarask", "bg", "bgc", "bgc-u-nu-latn", "bho", "bho-u-nu-latn", "bn", "bn-IN", "bn-IN-u-nu-latn", "bn-u-nu-latn", "br", "brx", "brx-u-nu-deva", "bs", "bs-Cyrl", "bs-Latn", "ca", "ca-AD", "ca-ES-valencia", "ca-FR", "ca-IT", "ceb", "chr", "cs", "cv", "cy", "da", "da-GL", "de", "de-AT", "de-BE", "de-CH", "de-IT", "de-LI", "de-LU", "doi", "doi-u-nu-deva", "dsb", "el", "el-CY", "el-polyton", "en", "en-001", "en-150", "en-AE", "en-AG", "en-AI", "en-AS", "en-AT", "en-AU", "en-BB", "en-BE", "en-BI", "en-BM", "en-BS", "en-BW", "en-BZ", "en-CA", "en-CC", "en-CH", "en-CK", "en-CM", "en-CX", "en-CY", "en-DE", "en-DG", "en-DK", "en-DM", "en-ER", "en-FI", "en-FJ", "en-FK", "en-FM", "en-GB", "en-GD", "en-GG", "en-GH", "en-GI", "en-GM", "en-GU", "en-GY", "en-HK", "en-IE", "en-IL", "en-IM", "en-IN", "en-IO", "en-JE", "en-JM", "en-KE", "en-KI", "en-KN", "en-KY", "en-LC", "en-LR", "en-LS", "en-MG", "en-MH", "en-MO", "en-MP", "en-MS", "en-MT", "en-MU", "en-MV", "en-MW", "en-MY", "en-NA", "en-NF", "en-NG", "en-NL", "en-NR", "en-NU", "en-NZ", "en-PG", "en-PH", "en-PK", "en-PN", "en-PR", "en-PW", "en-RW", "en-SB", "en-SC", "en-SD", "en-SE", "en-SG", "en-SH", "en-SI", "en-SL", "en-SS", "en-SX", "en-SZ", "en-TC", "en-TK", "en-TO", "en-TT", "en-TV", "en-TZ", "en-UG", "en-UM", "en-VC", "en-VG", "en-VI", "en-VU", "en-WS", "en-ZA", "en-ZM", "en-ZW", "es", "es-419", "es-AR", "es-BO", "es-BR", "es-BZ", "es-CL", "es-CO", "es-CR", "es-CU", "es-DO", "es-EA", "es-EC", "es-GQ", "es-GT", "es-HN", "es-IC", "es-MX", "es-NI", "es-PA", "es-PE", "es-PH", "es-PR", "es-PY", "es-SV", "es-US", "es-UY", "es-VE", "et", "eu", "fa", "fa-AF", "fa-AF-u-nu-latn", "fa-u-nu-latn", "ff-Adlm", "ff-Adlm-BF", "ff-Adlm-BF-u-nu-latn", "ff-Adlm-CM", "ff-Adlm-CM-u-nu-latn", "ff-Adlm-GH", "ff-Adlm-GH-u-nu-latn", "ff-Adlm-GM", "ff-Adlm-GM-u-nu-latn", "ff-Adlm-GW", "ff-Adlm-GW-u-nu-latn", "ff-Adlm-LR", "ff-Adlm-LR-u-nu-latn", "ff-Adlm-MR", "ff-Adlm-MR-u-nu-latn", "ff-Adlm-NE", "ff-Adlm-NE-u-nu-latn", "ff-Adlm-NG", "ff-Adlm-NG-u-nu-latn", "ff-Adlm-SL", "ff-Adlm-SL-u-nu-latn", "ff-Adlm-SN", "ff-Adlm-SN-u-nu-latn", "ff-Adlm-u-nu-latn", "fi", "fil", "fo", "fo-DK", "fr", "fr-BE", "fr-BF", "fr-BI", "fr-BJ", "fr-BL", "fr-CA", "fr-CD", "fr-CF", "fr-CG", "fr-CH", "fr-CI", "fr-CM", "fr-DJ", "fr-DZ", "fr-GA", "fr-GF", "fr-GN", "fr-GP", "fr-GQ", "fr-HT", "fr-KM", "fr-LU", "fr-MA", "fr-MC", "fr-MF", "fr-MG", "fr-ML", "fr-MQ", "fr-MR", "fr-MU", "fr-NC", "fr-NE", "fr-PF", "fr-PM", "fr-RE", "fr-RW", "fr-SC", "fr-SN", "fr-SY", "fr-TD", "fr-TG", "fr-TN", "fr-VU", "fr-WF", "fr-YT", "ga", "ga-GB", "gd", "gl", "gu", "gu-u-nu-gujr", "ha", "ha-GH", "ha-NE", "he", "hi", "hi-Latn", "hi-u-nu-deva", "hr", "hr-BA", "hsb", "hu", "hy", "ia", "id", "ig", "is", "it", "it-CH", "it-SM", "it-VA", "ja", "jv", "jv-u-nu-java", "ka", "kea", "kgp", "kk", "km", "km-u-nu-khmr", "kn", "kn-u-nu-knda", "ko", "ko-KP", "kok", "kok-u-nu-deva", "ks", "ks-Arab", "ks-Arab-u-nu-latn", "ks-Deva", "ks-u-nu-latn", "ky", "lo", "lo-u-nu-laoo", "lt", "lv", "mai", "mai-u-nu-deva", "mi", "mk", "ml", "ml-u-nu-mlym", "mn", "mni", "mni-Beng", "mni-Beng-u-nu-latn", "mni-u-nu-latn", "mr", "mr-u-nu-latn", "ms", "ms-BN", "ms-ID", "ms-SG", "my", "my-u-nu-latn", "nb", "nb-SJ", "ne", "ne-IN", "ne-IN-u-nu-latn", "ne-u-nu-latn", "nl", "nl-AW", "nl-BE", "nl-BQ", "nl-CW", "nl-SR", "nl-SX", "nn", "no", "or", "or-u-nu-orya", "pa", "pa-Guru", "pa-Guru-u-nu-guru", "pa-u-nu-guru", "pcm", "pl", "ps", "ps-PK", "ps-PK-u-nu-latn", "ps-u-nu-latn", "pt", "pt-AO", "pt-CH", "pt-CV", "pt-GQ", "pt-GW", "pt-LU", "pt-MO", "pt-MZ", "pt-PT", "pt-ST", "pt-TL", "qu", "qu-BO", "qu-EC", "raj", "raj-u-nu-latn", "rm", "ro", "ro-MD", "ru", "ru-BY", "ru-KG", "ru-KZ", "ru-MD", "ru-UA", "sa", "sa-u-nu-latn", "sat", "sat-Olck", "sat-Olck-u-nu-latn", "sat-u-nu-latn", "sc", "sd", "sd-Arab", "sd-Arab-u-nu-latn", "sd-Deva", "sd-u-nu-latn", "si", "sk", "sl", "so", "so-DJ", "so-ET", "so-KE", "sq", "sq-MK", "sq-XK", "sr", "sr-Cyrl", "sr-Cyrl-BA", "sr-Cyrl-XK", "sr-Latn", "sr-Latn-BA", "sr-Latn-ME", "sr-Latn-XK", "su", "su-Latn", "sv", "sv-AX", "sv-FI", "sw", "sw-CD", "sw-KE", "sw-UG", "ta", "ta-LK", "ta-LK-u-nu-tamldec", "ta-MY", "ta-MY-u-nu-tamldec", "ta-SG", "ta-SG-u-nu-tamldec", "ta-u-nu-tamldec", "te", "te-u-nu-telu", "tg", "th", "th-u-nu-thai", "ti", "ti-ER", "tk", "to", "tr", "tr-CY", "tt", "uk", "und", "ur", "ur-IN", "ur-IN-u-nu-latn", "ur-u-nu-arabext", "uz", "uz-Cyrl", "uz-Latn", "vi", "wo", "xh", "yo", "yo-BJ", "yrl", "yrl-CO", "yrl-VE", "yue", "yue-Hans", "yue-Hans-u-nu-hanidec", "yue-Hant", "yue-Hant-u-nu-hanidec", "yue-u-nu-hanidec", "zh", "zh-Hans", "zh-Hans-SG", "zh-Hans-SG-u-nu-hanidec", "zh-Hans-u-nu-hanidec", "zh-Hant", "zh-Hant-HK", "zh-Hant-HK-u-nu-hanidec", "zh-Hant-MO", "zh-Hant-MO-u-nu-hanidec", "zh-Hant-u-nu-hanidec", "zh-u-nu-hanidec", "zu"];
                let mut metadata = icu_provider::DataResponseMetadata::default();
                let payload = if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    payload
                } else {
                    let mut fallback_iterator = icu_locid_transform::fallback::LocaleFallbacker::new().fallback_for(<icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.into(), req.locale.clone());
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
