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
                static TO: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x05\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\x06\0\r\0\x14\0\x1E\0(\0\x03\0 afe\x04\0 mano\x05\0 kilu\x06\0 miliona\t\0 piliona\x0C\0 tiliona") })
                    },
                };
                static SD: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x05\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0\x07\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x01\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0B\0\x16\0!\0,\x007\0B\0\x03\0 \xDA\xBE\xD8\xB2\xD8\xA7\xD8\xB1\x03\0 \xDA\xBE\xD8\xB2\xD8\xA7\xD8\xB1\x03\0 \xD9\x87\xD8\xB2\xD8\xA7\xD8\xB1\x03\0 \xD9\x87\xD8\xB2\xD8\xA7\xD8\xB1\x06\0 \xD9\x85\xD9\x84\xD9\x8A\xD9\x86\t\0 \xD8\xA8\xD9\x84\xD9\x8A\xD9\x86\x0C\0 \xD9\xBD\xD8\xB1\xD9\x84\xD9\x8A\xD9\x86") })
                    },
                };
                static MY: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x05\x06\x07\x0B\x0C\r\x0E") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0\x07\0\0\0\x08\0\0\0\t\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\t\0\0\0\0\0\x12\0'\09\0H\0W\0y\0\x98\0\xB4\0\x03\0 \xE1\x80\x91\xE1\x80\xB1\xE1\x80\xAC\xE1\x80\x84\xE1\x80\xBA\x04\0 \xE1\x80\x9E\xE1\x80\xB1\xE1\x80\xAC\xE1\x80\x84\xE1\x80\xBA\xE1\x80\xB8\x05\0 \xE1\x80\x9E\xE1\x80\xAD\xE1\x80\x94\xE1\x80\xBA\xE1\x80\xB8\x06\0 \xE1\x80\x9E\xE1\x80\x94\xE1\x80\xBA\xE1\x80\xB8\x07\0 \xE1\x80\x80\xE1\x80\xAF\xE1\x80\x8B\xE1\x80\xB1\x0B\r\xE1\x80\x80\xE1\x80\xAF\xE1\x80\x8B\xE1\x80\xB1  \xE1\x80\x9E\xE1\x80\xB1\xE1\x80\xAC\xE1\x80\x84\xE1\x80\xBA\xE1\x80\xB8\x0C\r\xE1\x80\x80\xE1\x80\xAF\xE1\x80\x8B\xE1\x80\xB1  \xE1\x80\x9E\xE1\x80\xAD\xE1\x80\x94\xE1\x80\xBA\xE1\x80\xB8\r\r\xE1\x80\x80\xE1\x80\xAF\xE1\x80\x8B\xE1\x80\xB1  \xE1\x80\x9E\xE1\x80\x94\xE1\x80\xBA\xE1\x80\xB8\x0E\0 \xE1\x80\x80\xE1\x80\xB1\xE1\x80\xAC\xE1\x80\x8B\xE1\x80\xAD") })
                    },
                };
                static SO: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\x06\0\x0C\0\x12\0\x1B\0$\0\x03\0 kun\x03\0 Kun\x03\0 Kun\x06\0 Milyan\t\0 Bilyan\x0C\0 Tirilyan") })
                    },
                };
                static FR: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\n\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x05\x06\x05\x01\x05\x01\x05\x01\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\n\0\0\0\0\0\n\0\x12\0\x19\0!\0+\x006\0A\0M\0W\0\x03\0 millier\x03\0 mille\x03\xFFmille\x03\0 mille\x06\0 million\x06\0 millions\t\0 milliard\t\0 milliards\x0C\0 billion\x0C\0 billions") })
                    },
                };
                static CY: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x06\t\x0C\x0E") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x04\0\0\0\t\0\0\0\x0E\0\0\0\x13\0\0\0\x18\0\0\0\x1E\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\x03\x04\x05\0\x02\x03\x04\x05\0\x02\x03\x04\x05\0\x02\x03\x04\x05\0\x02\x03\x04\x05\0\x01\x02\x03\x04\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x1E\0\0\0\0\0\x03\0\x06\0\t\0\x0F\0\x12\0\x15\0\x18\0\x1B\0!\0$\0'\0*\0-\x006\09\0<\0?\0B\0K\0N\0Q\0T\0W\0a\0d\0g\0j\0m\0p\0\x03\0K\x03\0K\x03\0K\x03\0 mil\x03\0K\x03\0K\x03\0K\x03\0K\x03\0 mil\x06\0M\x06\0M\x06\0M\x06\0M\x06\0 miliwn\t\0B\t\0B\t\0B\t\0B\t\0 biliwn\x0C\0T\x0C\0T\x0C\0T\x0C\0T\x0C\0 triliwn\x0C\0T\x0C\0T\x0C\0T\x0C\0T\x0C\0T\x0C\0 triliwn") })
                    },
                };
                static GA: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x06\x07\t\n\x0B\x0C\r\x0E") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x04\0\0\0\x05\0\0\0\t\0\0\0\n\0\0\0\x0F\0\0\0\x11\0\0\0\x12\0\0\0\x16\0\0\0\x18\0\0\0\x19\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x02\x03\x05\x05\x01\x02\x03\x05\x05\x01\x02\x03\x04\x05\x04\x05\x05\x02\x03\x04\x05\x04\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x19\0\0\0\0\0\t\0\x12\0\x1B\0#\0+\x007\0C\0O\0Z\0e\0q\0}\0\x89\0\x95\0\xA0\0\xAC\0\xB7\0\xC2\0\xCF\0\xDC\0\xE9\0\xF5\0\x02\x01\x0E\x01\x03\0 mh\xC3\xADle\x03\0 mh\xC3\xADle\x03\0 mh\xC3\xADle\x03\0 m\xC3\xADle\x03\0 m\xC3\xADle\x06\0 mhilli\xC3\xBAn\x06\0 mhilli\xC3\xBAn\x06\0 mhilli\xC3\xBAn\x06\0 milli\xC3\xBAn\x06\0 milli\xC3\xBAn\t\0 bhilli\xC3\xBAn\t\0 bhilli\xC3\xBAn\t\0 bhilli\xC3\xBAn\t\0 mbilli\xC3\xBAn\t\0 billi\xC3\xBAn\t\0 mbilli\xC3\xBAn\t\0 billi\xC3\xBAn\t\0 billi\xC3\xBAn\x0C\0 thrilli\xC3\xBAn\x0C\0 thrilli\xC3\xBAn\x0C\0 dtrilli\xC3\xBAn\x0C\0 trilli\xC3\xBAn\x0C\0 dtrilli\xC3\xBAn\x0C\0 trilli\xC3\xBAn\x0C\0 trilli\xC3\xBAn") })
                    },
                };
                static BG: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x06\x07\t\n\x0C\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0\x05\0\0\0\x06\0\0\0\x08\0\0\0\t\0\0\0\x0B\0\0\0\x0C\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x05\x05\x01\x05\x05\x01\x05\x05\x01\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\n\0\x19\0(\x007\0H\0Y\0j\0}\0\x90\0\xA1\0\xB4\0\x03\0 \xD1\x85\xD0\xB8\xD0\xBB.\x03\0 \xD1\x85\xD0\xB8\xD0\xBB\xD1\x8F\xD0\xB4\xD0\xB8\x03\0 \xD1\x85\xD0\xB8\xD0\xBB\xD1\x8F\xD0\xB4\xD0\xB8\x06\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\x06\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\xD0\xB0\x06\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\xD0\xB0\t\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xB8\xD0\xB0\xD1\x80\xD0\xB4\t\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xB8\xD0\xB0\xD1\x80\xD0\xB4\xD0\xB0\t\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xB8\xD0\xB0\xD1\x80\xD0\xB4\xD0\xB0\x0C\0 \xD1\x82\xD1\x80\xD0\xB8\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\x0C\0 \xD1\x82\xD1\x80\xD0\xB8\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\xD0\xB0\x0C\0 \xD1\x82\xD1\x80\xD0\xB8\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\xD0\xB0") })
                    },
                };
                static IT: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x06\x07\t\n\x0C\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0\x05\0\0\0\x06\0\0\0\x08\0\0\0\t\0\0\0\x0B\0\0\0\x0C\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x05\x05\x01\x05\x05\x01\x05\x05\x01\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x07\0\x0E\0\x15\0\x1F\0)\x003\0>\0I\0T\0e\0u\0\x03\xFFmille\x03\0 mila\x03\0 mila\x06\0 milione\x06\0 milioni\x06\0 milioni\t\0 miliardo\t\0 miliardi\t\0 miliardi\x0C\0 mille miliardi\x0C\0 mila miliardi\x0C\0 mila miliardi") })
                    },
                };
                static CA: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x06\x07\t\n\x0C\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0\x05\0\0\0\x06\0\0\0\x08\0\0\0\t\0\0\0\x0B\0\0\0\x0C\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x05\x05\x01\x05\x05\x01\x05\x05\x01\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x08\0\x11\0\x1A\0#\0-\x007\0J\0^\0r\0{\0\x85\0\x03\0 miler\x03\0 milers\x03\0 milers\x06\0 mili\xC3\xB3\x06\0 milions\x06\0 milions\t\0 miler de milions\t\0 milers de milions\t\0 milers de milions\x0C\0 bili\xC3\xB3\x0C\0 bilions\x0C\0 bilions") })
                    },
                };
                static FI: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x06\x07\t\n\x0C\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0\x05\0\0\0\x06\0\0\0\x08\0\0\0\t\0\0\0\x0B\0\0\0\x0C\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x05\x05\x01\x05\x05\x01\x05\x05\x01\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x08\0\x12\0\x1C\0'\x003\0?\0J\0V\0b\0m\0y\0\x03\0 tuhat\x03\0 tuhatta\x03\0 tuhatta\x06\0 miljoona\x06\0 miljoonaa\x06\0 miljoonaa\t\0 miljardi\t\0 miljardia\t\0 miljardia\x0C\0 biljoona\x0C\0 biljoonaa\x0C\0 biljoonaa") })
                    },
                };
                static TE: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x06\x07\t\n\x0C\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0\x05\0\0\0\x06\0\0\0\x08\0\0\0\t\0\0\0\x0B\0\0\0\x0C\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x05\x05\x01\x05\x05\x01\x05\x05\x01\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x0F\0\x1E\0-\0E\0c\0\x81\0\x99\0\xB7\0\xD5\0\xF3\0\x17\x01\x03\0 \xE0\xB0\xB5\xE0\xB1\x87\xE0\xB0\xAF\xE0\xB0\xBF\x03\0 \xE0\xB0\xB5\xE0\xB1\x87\xE0\xB0\xB2\xE0\xB1\x81\x03\0 \xE0\xB0\xB5\xE0\xB1\x87\xE0\xB0\xB2\xE0\xB1\x81\x06\0 \xE0\xB0\xAE\xE0\xB0\xBF\xE0\xB0\xB2\xE0\xB0\xBF\xE0\xB0\xAF\xE0\xB0\xA8\xE0\xB1\x8D\x06\0 \xE0\xB0\xAE\xE0\xB0\xBF\xE0\xB0\xB2\xE0\xB0\xBF\xE0\xB0\xAF\xE0\xB0\xA8\xE0\xB1\x8D\xE0\xB0\xB2\xE0\xB1\x81\x06\0 \xE0\xB0\xAE\xE0\xB0\xBF\xE0\xB0\xB2\xE0\xB0\xBF\xE0\xB0\xAF\xE0\xB0\xA8\xE0\xB1\x8D\xE0\xB0\xB2\xE0\xB1\x81\t\0 \xE0\xB0\xAC\xE0\xB0\xBF\xE0\xB0\xB2\xE0\xB0\xBF\xE0\xB0\xAF\xE0\xB0\xA8\xE0\xB1\x8D\t\0 \xE0\xB0\xAC\xE0\xB0\xBF\xE0\xB0\xB2\xE0\xB0\xBF\xE0\xB0\xAF\xE0\xB0\xA8\xE0\xB1\x8D\xE0\xB0\xB2\xE0\xB1\x81\t\0 \xE0\xB0\xAC\xE0\xB0\xBF\xE0\xB0\xB2\xE0\xB0\xBF\xE0\xB0\xAF\xE0\xB0\xA8\xE0\xB1\x8D\xE0\xB0\xB2\xE0\xB1\x81\x0C\0 \xE0\xB0\x9F\xE0\xB1\x8D\xE0\xB0\xB0\xE0\xB0\xBF\xE0\xB0\xB2\xE0\xB0\xBF\xE0\xB0\xAF\xE0\xB0\xA8\xE0\xB1\x8D\x0C\0 \xE0\xB0\x9F\xE0\xB1\x8D\xE0\xB0\xB0\xE0\xB0\xBF\xE0\xB0\xB2\xE0\xB0\xBF\xE0\xB0\xAF\xE0\xB0\xA8\xE0\xB1\x8D\xE0\xB0\xB2\xE0\xB1\x81\x0C\0 \xE0\xB0\x9F\xE0\xB1\x8D\xE0\xB0\xB0\xE0\xB0\xBF\xE0\xB0\xB2\xE0\xB0\xBF\xE0\xB0\xAF\xE0\xB0\xA8\xE0\xB1\x8D\xE0\xB0\xB2\xE0\xB1\x81") })
                    },
                };
                static EL: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x06\x07\t\n\x0C\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0\x05\0\0\0\x06\0\0\0\x08\0\0\0\t\0\0\0\x0B\0\0\0\x0C\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x05\x05\x01\x05\x05\x01\x05\x05\x01\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x11\0$\x007\0P\0i\0\x82\0\xA1\0\xC0\0\xDF\0\0\x01!\x01\x03\0 \xCF\x87\xCE\xB9\xCE\xBB\xCE\xB9\xCE\xAC\xCE\xB4\xCE\xB1\x03\0 \xCF\x87\xCE\xB9\xCE\xBB\xCE\xB9\xCE\xAC\xCE\xB4\xCE\xB5\xCF\x82\x03\0 \xCF\x87\xCE\xB9\xCE\xBB\xCE\xB9\xCE\xAC\xCE\xB4\xCE\xB5\xCF\x82\x06\0 \xCE\xB5\xCE\xBA\xCE\xB1\xCF\x84\xCE\xBF\xCE\xBC\xCE\xBC\xCF\x8D\xCF\x81\xCE\xB9\xCE\xBF\x06\0 \xCE\xB5\xCE\xBA\xCE\xB1\xCF\x84\xCE\xBF\xCE\xBC\xCE\xBC\xCF\x8D\xCF\x81\xCE\xB9\xCE\xB1\x06\0 \xCE\xB5\xCE\xBA\xCE\xB1\xCF\x84\xCE\xBF\xCE\xBC\xCE\xBC\xCF\x8D\xCF\x81\xCE\xB9\xCE\xB1\t\0 \xCE\xB4\xCE\xB9\xCF\x83\xCE\xB5\xCE\xBA\xCE\xB1\xCF\x84\xCE\xBF\xCE\xBC\xCE\xBC\xCF\x8D\xCF\x81\xCE\xB9\xCE\xBF\t\0 \xCE\xB4\xCE\xB9\xCF\x83\xCE\xB5\xCE\xBA\xCE\xB1\xCF\x84\xCE\xBF\xCE\xBC\xCE\xBC\xCF\x8D\xCF\x81\xCE\xB9\xCE\xB1\t\0 \xCE\xB4\xCE\xB9\xCF\x83\xCE\xB5\xCE\xBA\xCE\xB1\xCF\x84\xCE\xBF\xCE\xBC\xCE\xBC\xCF\x8D\xCF\x81\xCE\xB9\xCE\xB1\x0C\0 \xCF\x84\xCF\x81\xCE\xB9\xCF\x83\xCE\xB5\xCE\xBA\xCE\xB1\xCF\x84\xCE\xBF\xCE\xBC\xCE\xBC\xCF\x8D\xCF\x81\xCE\xB9\xCE\xBF\x0C\0 \xCF\x84\xCF\x81\xCE\xB9\xCF\x83\xCE\xB5\xCE\xBA\xCE\xB1\xCF\x84\xCE\xBF\xCE\xBC\xCE\xBC\xCF\x8D\xCF\x81\xCE\xB9\xCE\xB1\x0C\0 \xCF\x84\xCF\x81\xCE\xB9\xCF\x83\xCE\xB5\xCE\xBA\xCE\xB1\xCF\x84\xCE\xBF\xCE\xBC\xCE\xBC\xCF\x8D\xCF\x81\xCE\xB9\xCE\xB1") })
                    },
                };
                static CS: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x06\x07\t\n\x0C\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\0\0\0\x05\0\0\0\t\0\0\0\x0B\0\0\0\x0F\0\0\0\x11\0\0\0\x15\0\0\0\x17\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x05\x04\x05\x01\x03\x04\x05\x04\x05\x01\x03\x04\x05\x04\x05\x01\x03\x04\x05\x04\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x17\0\0\0\0\0\n\0\x14\0\x1D\0'\x000\09\0C\0M\0X\0b\0m\0x\0\x83\0\x8E\0\x98\0\xA3\0\xAD\0\xB6\0\xC0\0\xCA\0\xD5\0\xDF\0\x03\0 tis\xC3\xADce\x03\0 tis\xC3\xADce\x03\0 tis\xC3\xADc\x03\0 tis\xC3\xADce\x03\0 tis\xC3\xADc\x06\0 milion\x06\0 miliony\x06\0 milionu\x06\0 milion\xC5\xAF\x06\0 milionu\x06\0 milion\xC5\xAF\t\0 miliarda\t\0 miliardy\t\0 miliardy\t\0 miliard\t\0 miliardy\t\0 miliard\x0C\0 bilion\x0C\0 biliony\x0C\0 bilionu\x0C\0 bilion\xC5\xAF\x0C\0 bilionu\x0C\0 bilion\xC5\xAF") })
                    },
                };
                static SK: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x06\x07\t\n\x0C\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\0\0\0\x05\0\0\0\t\0\0\0\x0B\0\0\0\x0F\0\0\0\x11\0\0\0\x15\0\0\0\x17\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x05\x04\x05\x01\x03\x04\x05\x04\x05\x01\x03\x04\x05\x04\x05\x01\x03\x04\x05\x04\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x17\0\0\0\0\0\n\0\x14\0\x1D\0'\x000\0:\0E\0P\0\\\0g\0s\0~\0\x89\0\x94\0\x9F\0\xAA\0\xB5\0\xBF\0\xCA\0\xD5\0\xE1\0\xEC\0\x03\0 tis\xC3\xADce\x03\0 tis\xC3\xADca\x03\0 tis\xC3\xADc\x03\0 tis\xC3\xADca\x03\0 tis\xC3\xADc\x06\0 mili\xC3\xB3n\x06\0 mili\xC3\xB3ny\x06\0 mili\xC3\xB3na\x06\0 mili\xC3\xB3nov\x06\0 mili\xC3\xB3na\x06\0 mili\xC3\xB3nov\t\0 miliarda\t\0 miliardy\t\0 miliardy\t\0 mili\xC3\xA1rd\t\0 miliardy\t\0 mili\xC3\xA1rd\x0C\0 bili\xC3\xB3n\x0C\0 bili\xC3\xB3ny\x0C\0 bili\xC3\xB3na\x0C\0 bili\xC3\xB3nov\x0C\0 bili\xC3\xB3na\x0C\0 bili\xC3\xB3nov") })
                    },
                };
                static LV: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x06\x07\t\n\x0C\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\0\0\0\x05\0\0\0\x08\0\0\0\n\0\0\0\r\0\0\0\x0F\0\0\0\x12\0\0\0\x14\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0\x01\x05\x01\x05\0\x01\x05\x01\x05\0\x01\x05\x01\x05\0\x01\x05\x01\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x14\0\0\0\0\0\r\0\x1A\0'\x004\0A\0K\0U\0_\0i\0s\0~\0\x89\0\x94\0\x9F\0\xAA\0\xB5\0\xC0\0\xCB\0\xD6\0\x03\0 t\xC5\xABksto\xC5\xA1u\x03\0 t\xC5\xABkstotis\x03\0 t\xC5\xABksto\xC5\xA1i\x03\0 t\xC5\xABkstotis\x03\0 t\xC5\xABksto\xC5\xA1i\x06\0 miljonu\x06\0 miljons\x06\0 miljoni\x06\0 miljons\x06\0 miljoni\t\0 miljardu\t\0 miljards\t\0 miljardi\t\0 miljards\t\0 miljardi\x0C\0 triljonu\x0C\0 triljons\x0C\0 triljoni\x0C\0 triljons\x0C\0 triljoni") })
                    },
                };
                static AST: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x06\x07\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0\x05\0\0\0\x06\0\0\0\x07\0\0\0\x08\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x05\x05\x01\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x08\0\0\0\0\0\t\0\x14\0\x1F\0)\x004\0?\0B\0\x03\0 millar\x03\0 millares\x03\0 millares\x06\0 mill\xC3\xB3n\x06\0 millones\x06\0 millones\t\0G\x0C\0T") })
                    },
                };
                static AR: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x06\x08\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0\x05\0\0\0\x06\0\0\0\x07\0\0\0\x08\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x05\x03\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x08\0\0\0\0\0\x0B\0\x14\0\x1D\0,\09\0F\0S\0\x03\0 \xD8\xA2\xD9\x84\xD8\xA7\xD9\x81\x03\0 \xD8\xA3\xD9\x84\xD9\x81\x03\0 \xD8\xA3\xD9\x84\xD9\x81\x06\0 \xD9\x85\xD9\x84\xD8\xA7\xD9\x8A\xD9\x8A\xD9\x86\x06\0 \xD9\x85\xD9\x84\xD9\x8A\xD9\x88\xD9\x86\x06\0 \xD9\x85\xD9\x84\xD9\x8A\xD9\x88\xD9\x86\t\0 \xD9\x85\xD9\x84\xD9\x8A\xD8\xA7\xD8\xB1\x0C\0 \xD8\xAA\xD8\xB1\xD9\x84\xD9\x8A\xD9\x88\xD9\x86") })
                    },
                };
                static YUE: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x08\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\n\0\x0F\0\x03\0\xE5\x8D\x83\x04\0\xE8\x90\xAC\x08\0\xE5\x84\x84\x0C\0\xE5\x85\x86") })
                    },
                };
                static KO: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\x08\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\n\0\x0F\0\x03\0\xEC\xB2\x9C\x04\0\xEB\xA7\x8C\x08\0\xEC\x96\xB5\x0C\0\xEC\xA1\xB0") })
                    },
                };
                static LO: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x0C\0\x18\0'\x003\0\x03\0 \xE0\xBA\x9E\xE0\xBA\xB1\xE0\xBA\x99\x05\0 \xE0\xBB\x81\xE0\xBA\xAA\xE0\xBA\x99\x06\0 \xE0\xBA\xA5\xE0\xBB\x89\xE0\xBA\xB2\xE0\xBA\x99\t\0 \xE0\xBA\x95\xE0\xBA\xB7\xE0\xBB\x89\x0C\0 \xE0\xBA\xA5\xE0\xBB\x89\xE0\xBA\xB2\xE0\xBA\x99\xE0\xBA\xA5\xE0\xBB\x89\xE0\xBA\xB2\xE0\xBA\x99") })
                    },
                };
                static KM: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x0F\0\x1D\0)\0>\0\x03\0 \xE1\x9E\x96\xE1\x9E\xB6\xE1\x9E\x93\xE1\x9F\x8B\x03\0\xE1\x9E\x96\xE1\x9E\xB6\xE1\x9E\x93\xE1\x9F\x8B\x06\0 \xE1\x9E\x9B\xE1\x9E\xB6\xE1\x9E\x93\t\0 \xE1\x9E\x94\xE1\x9F\x8A\xE1\x9E\xB8\xE1\x9E\x9B\xE1\x9E\xB6\xE1\x9E\x93\x0C\0 \xE1\x9E\x91\xE1\x9F\x92\xE1\x9E\x9A\xE1\x9E\xB8\xE1\x9E\x9B\xE1\x9E\xB6\xE1\x9E\x93") })
                    },
                };
                static AS: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x12\0\x1E\x000\0F\0\x03\0 \xE0\xA6\xB9\xE0\xA6\xBE\xE0\xA6\x9C\xE0\xA6\xBE\xE0\xA7\xB0\x05\0 \xE0\xA6\xB2\xE0\xA6\xBE\xE0\xA6\x96\x06\0 \xE0\xA6\xA8\xE0\xA6\xBF\xE0\xA6\xAF\xE0\xA7\x81\xE0\xA6\xA4\t\0 \xE0\xA6\xB6\xE0\xA6\xA4 \xE0\xA6\x95\xE0\xA7\x8B\xE0\xA6\x9F\xE0\xA6\xBF\x0C\0 \xE0\xA6\xB6\xE0\xA6\xA4 \xE0\xA6\xAA\xE0\xA7\xB0\xE0\xA6\xBE\xE0\xA7\xB0\xE0\xA7\x8D\xE0\xA6\xA6\xE0\xA7\x8D\xE0\xA6\xA7") })
                    },
                };
                static NE: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x06\t\x0C\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\x0F\0\x1B\0*\x006\0B\0\x03\0 \xE0\xA4\xB9\xE0\xA4\x9C\xE0\xA4\xBE\xE0\xA4\xB0\x05\0 \xE0\xA4\xB2\xE0\xA4\xBE\xE0\xA4\x96\x06\0 \xE0\xA4\x95\xE0\xA4\xB0\xE0\xA5\x8B\xE0\xA4\xA1\t\0 \xE0\xA4\x85\xE0\xA4\xB0\xE0\xA4\xAC\x0B\0 \xE0\xA4\x96\xE0\xA4\xB0\xE0\xA4\xAC\r\0 \xE0\xA4\xB6\xE0\xA4\x82\xE0\xA4\x96") })
                    },
                };
                static HI: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x07\t\x0B") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x12\0\x1E\x000\0<\0\x03\0 \xE0\xA4\xB9\xE0\xA4\x9C\xE0\xA4\xBC\xE0\xA4\xBE\xE0\xA4\xB0\x05\0 \xE0\xA4\xB2\xE0\xA4\xBE\xE0\xA4\x96\x07\0 \xE0\xA4\x95\xE0\xA4\xB0\xE0\xA5\x8B\xE0\xA4\xA1\xE0\xA4\xBC\t\0 \xE0\xA4\x85\xE0\xA4\xB0\xE0\xA4\xAC\x0B\0 \xE0\xA4\x96\xE0\xA4\xB0\xE0\xA4\xAC") })
                    },
                };
                static UR: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x07\t\x0B\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\x0B\0\x16\0!\0*\x005\0\x03\0 \xDB\x81\xD8\xB2\xD8\xA7\xD8\xB1\x05\0 \xD9\x84\xD8\xA7\xDA\xA9\xDA\xBE\x07\0 \xDA\xA9\xD8\xB1\xD9\x88\xDA\x91\t\0 \xD8\xA7\xD8\xB1\xD8\xA8\x0B\0 \xDA\xA9\xDA\xBE\xD8\xB1\xD8\xA8\x0C\0 \xD9\xB9\xD8\xB1\xDB\x8C\xD9\x84\xDB\x8C\xD9\x86") })
                    },
                };
                static MR: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x07\t\x0B\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\x0F\0\x1B\0*\09\0H\0\x03\0 \xE0\xA4\xB9\xE0\xA4\x9C\xE0\xA4\xBE\xE0\xA4\xB0\x05\0 \xE0\xA4\xB2\xE0\xA4\xBE\xE0\xA4\x96\x07\0 \xE0\xA4\x95\xE0\xA5\x8B\xE0\xA4\x9F\xE0\xA5\x80\t\0 \xE0\xA4\x85\xE0\xA4\xAC\xE0\xA5\x8D\xE0\xA4\x9C\x0B\0 \xE0\xA4\x96\xE0\xA4\xB0\xE0\xA5\x8D\xE0\xA4\xB5\r\0 \xE0\xA4\xAA\xE0\xA4\xA6\xE0\xA5\x8D\xE0\xA4\xAE") })
                    },
                };
                static PA: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x07\t\x0B\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\x12\0\x1E\0-\09\0E\0\x03\0 \xE0\xA8\xB9\xE0\xA8\x9C\xE0\xA8\xBC\xE0\xA8\xBE\xE0\xA8\xB0\x05\0 \xE0\xA8\xB2\xE0\xA9\xB1\xE0\xA8\x96\x07\0 \xE0\xA8\x95\xE0\xA8\xB0\xE0\xA9\x8B\xE0\xA9\x9C\t\0 \xE0\xA8\x85\xE0\xA8\xB0\xE0\xA8\xAC\x0B\0 \xE0\xA8\x96\xE0\xA8\xB0\xE0\xA8\xAC\r\0 \xE0\xA8\xA8\xE0\xA9\x80\xE0\xA8\xB2") })
                    },
                };
                static GU: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x07\t\x0B\x0C\r\x0E") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0\x07\0\0\0\x08\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x08\0\0\0\0\0\x0F\0\x1B\0*\x006\0K\0c\0r\0\x03\0 \xE0\xAA\xB9\xE0\xAA\x9C\xE0\xAA\xBE\xE0\xAA\xB0\x05\0 \xE0\xAA\xB2\xE0\xAA\xBE\xE0\xAA\x96\x07\0 \xE0\xAA\x95\xE0\xAA\xB0\xE0\xAB\x8B\xE0\xAA\xA1\t\0 \xE0\xAA\x85\xE0\xAA\xAC\xE0\xAA\x9C\x0B\0 \xE0\xAA\xA8\xE0\xAA\xBF\xE0\xAA\x96\xE0\xAA\xB0\xE0\xAB\x8D\xE0\xAA\xB5\x0C\0 \xE0\xAA\xAE\xE0\xAA\xB9\xE0\xAA\xBE\xE0\xAA\xAA\xE0\xAA\xA6\xE0\xAB\x8D\xE0\xAA\xAE\r\0 \xE0\xAA\xB6\xE0\xAA\x82\xE0\xAA\x95\xE0\xAB\x81\x0E\0 \xE0\xAA\x9C\xE0\xAA\xB2\xE0\xAA\xA7\xE0\xAA\xBF") })
                    },
                };
                static BN: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x07\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x12\0\x1E\0-\0\x03\0 \xE0\xA6\xB9\xE0\xA6\xBE\xE0\xA6\x9C\xE0\xA6\xBE\xE0\xA6\xB0\x05\0 \xE0\xA6\xB2\xE0\xA6\xBE\xE0\xA6\x96\x07\0 \xE0\xA6\x95\xE0\xA7\x8B\xE0\xA6\x9F\xE0\xA6\xBF\x0C\0 \xE0\xA6\xB2\xE0\xA6\xBE\xE0\xA6\x96 \xE0\xA6\x95\xE0\xA7\x8B\xE0\xA6\x9F\xE0\xA6\xBF") })
                    },
                };
                static LO_U_NU_LAOO: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0B\0\x19\x000\0\x03\0\xE0\xBA\x9E\xE0\xBA\xB1\xE0\xBA\x99\x06\0\xE0\xBA\xA5\xE0\xBB\x89\xE0\xBA\xB2\xE0\xBA\x99\t\0\xE0\xBA\x9E\xE0\xBA\xB1\xE0\xBA\x99\xE0\xBA\xA5\xE0\xBB\x89\xE0\xBA\xB2\xE0\xBA\x99\x0C\0\xE0\xBA\xA5\xE0\xBB\x89\xE0\xBA\xB2\xE0\xBA\x99\xE0\xBA\xA5\xE0\xBB\x89\xE0\xBA\xB2\xE0\xBA\x99") })
                    },
                };
                static AF: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\n\0\x14\0\x1E\0\x03\0 duisend\x06\0 miljoen\t\0 miljard\x0C\0 biljoen") })
                    },
                };
                static NL: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\n\0\x14\0\x1E\0\x03\0 duizend\x06\0 miljoen\t\0 miljard\x0C\0 biljoen") })
                    },
                };
                static PCM: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\n\0\x16\0\"\0\x03\0 Ta\xC3\xBAzan\x06\0 M\xC3\xADli\xE1\xBB\x8Dn\t\0 B\xC3\xADli\xE1\xBB\x8Dn\x0C\0 Tr\xC3\xADli\xE1\xBB\x8Dn") })
                    },
                };
                static MN: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\r\0\x16\0%\0\x03\0 \xD0\xBC\xD1\x8F\xD0\xBD\xD0\xB3\xD0\xB0\x06\0 \xD1\x81\xD0\xB0\xD1\x8F\t\0 \xD1\x82\xD1\x8D\xD1\x80\xD0\xB1\xD1\x83\xD0\xBC\x0C\0 \xD0\xB8\xD1\x85 \xD0\xBD\xD0\xB0\xD1\x8F\xD0\xB4") })
                    },
                };
                static HY: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\r\0\x1C\0-\0\x03\0 \xD5\xB0\xD5\xA1\xD5\xA6\xD5\xA1\xD6\x80\x06\0 \xD5\xB4\xD5\xAB\xD5\xAC\xD5\xAB\xD5\xB8\xD5\xB6\t\0 \xD5\xB4\xD5\xAB\xD5\xAC\xD5\xAB\xD5\xA1\xD6\x80\xD5\xA4\x0C\0 \xD5\xBF\xD6\x80\xD5\xAB\xD5\xAC\xD5\xAB\xD5\xB8\xD5\xB6") })
                    },
                };
                static TG: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\r\0\x1E\x001\0\x03\0 \xD2\xB3\xD0\xB0\xD0\xB7\xD0\xBE\xD1\x80\x06\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\t\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xB0\xD1\x80\xD0\xB4\x0C\0 \xD1\x82\xD1\x80\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD") })
                    },
                };
                static VI: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\t\0\x13\0\x1A\0\x03\0 ngh\xC3\xACn\x06\0 tri\xE1\xBB\x87u\t\0 t\xE1\xBB\xB7\x0C\0 ngh\xC3\xACn t\xE1\xBB\xB7") })
                    },
                };
                static TI: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\t\0\x18\0'\0\x03\0 \xE1\x88\xBD\xE1\x88\x95\x06\0 \xE1\x88\x9A\xE1\x88\x8D\xE1\x8B\xAE\xE1\x8A\x95\t\0 \xE1\x89\xA2\xE1\x88\x8D\xE1\x8B\xAE\xE1\x8A\x95\x0C\0 \xE1\x89\xB5\xE1\x88\xAA\xE1\x88\x8D\xE1\x8B\xAE\xE1\x8A\x95") })
                    },
                };
                static KY: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\t\0\x1A\0-\0\x03\0 \xD0\xBC\xD0\xB8\xD2\xA3\x06\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\t\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xB0\xD1\x80\xD0\xB4\x0C\0 \xD1\x82\xD1\x80\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD") })
                    },
                };
                static KK: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\t\0\x1A\0-\0\x03\0 \xD0\xBC\xD1\x8B\xD2\xA3\x06\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\t\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xB0\xD1\x80\xD0\xB4\x0C\0 \xD1\x82\xD1\x80\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD") })
                    },
                };
                static BGC: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
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
                static AZ: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x06\0\x0F\0\x19\0\x03\0 min\x06\0 milyon\t\0 milyard\x0C\0 trilyon") })
                    },
                };
                static KEA: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x06\0\x10\0\x1E\0\x03\0 mil\x06\0 milh\xC3\xA3u\t\0 mil milh\xC3\xA3u\x0C\0 bilh\xC3\xA3u") })
                    },
                };
                static AM: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x06\0\x15\0$\0\x03\0 \xE1\x88\xBA\x06\0 \xE1\x88\x9A\xE1\x88\x8A\xE1\x8B\xAE\xE1\x8A\x95\t\0 \xE1\x89\xA2\xE1\x88\x8A\xE1\x8B\xAE\xE1\x8A\x95\x0C\0 \xE1\x89\xB5\xE1\x88\xAA\xE1\x88\x8A\xE1\x8B\xAE\xE1\x8A\x95") })
                    },
                };
                static JV: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x07\0\x0E\0\x17\0\x03\0 \xC3\xA8wu\x06\0 yuta\t\0 milyar\x0C\0 trilyun") })
                    },
                };
                static MS: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x07\0\x0E\0\x17\0\x03\0 ribu\x06\0 juta\t\0 bilion\x0C\0 trilion") })
                    },
                };
                static ID: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x07\0\x0E\0\x17\0\x03\0 ribu\x06\0 juta\t\0 miliar\x0C\0 triliun") })
                    },
                };
                static HA: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x07\0\x11\0\x1B\0\x03\x05Dubu \x06\x08Miliyan \t\x08Biliyan \x0C\tTriliyan ") })
                    },
                };
                static SW: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x07\0\x11\0\x1B\0\x03\x05elfu \x06\x08milioni \t\x08bilioni \x0C\ttrilioni ") })
                    },
                };
                static UZ: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x07\0\x11\0\x1C\0\x03\0 ming\x06\0 million\t\0 milliard\x0C\0 trillion") })
                    },
                };
                static HU: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x07\0\x11\0\x1D\0\x03\0 ezer\x06\0 milli\xC3\xB3\t\0 milli\xC3\xA1rd\x0C\0 billi\xC3\xB3") })
                    },
                };
                static SQ: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x08\0\x11\0\x1B\0\x03\0 mij\xC3\xAB\x06\0 milion\t\0 miliard\x0C\0 bilion") })
                    },
                };
                static TK: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x08\0\x12\0\x1D\0\x03\0 m\xC3\xBC\xC5\x88\x06\0 million\t\0 milliard\x0C\0 trillion") })
                    },
                };
                static EN: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0B\0\x15\0\x1F\0\x03\0 thousand\x06\0 million\t\0 billion\x0C\0 trillion") })
                    },
                };
                static FA: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0B\0\x1A\0+\0\x03\0 \xD9\x87\xD8\xB2\xD8\xA7\xD8\xB1\x06\0 \xD9\x85\xDB\x8C\xD9\x84\xDB\x8C\xD9\x88\xD9\x86\t\0 \xD9\x85\xDB\x8C\xD9\x84\xDB\x8C\xD8\xA7\xD8\xB1\xD8\xAF\x0C\0 \xD9\x87\xD8\xB2\xD8\xA7\xD8\xB1\xD9\x85\xDB\x8C\xD9\x84\xDB\x8C\xD8\xA7\xD8\xB1\xD8\xAF") })
                    },
                };
                static UZ_CYRL: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0B\0\x1C\0/\0\x03\0 \xD0\xBC\xD0\xB8\xD0\xBD\xD0\xB3\x06\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\t\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xB0\xD1\x80\xD0\xB4\x0C\0 \xD1\x82\xD1\x80\xD0\xB8\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD") })
                    },
                };
                static SI: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0C\0!\x006\0\x03\n\xE0\xB6\xAF\xE0\xB7\x84\xE0\xB7\x83 \x06\x13\xE0\xB6\xB8\xE0\xB7\x92\xE0\xB6\xBD\xE0\xB7\x92\xE0\xB6\xBA\xE0\xB6\xB1 \t\x13\xE0\xB6\xB6\xE0\xB7\x92\xE0\xB6\xBD\xE0\xB7\x92\xE0\xB6\xBA\xE0\xB6\xB1 \x0C\x1C\xE0\xB6\xA7\xE0\xB7\x8A\xE2\x80\x8D\xE0\xB6\xBB\xE0\xB7\x92\xE0\xB6\xBD\xE0\xB7\x92\xE0\xB6\xBA\xE0\xB6\xB1 ") })
                    },
                };
                static HE: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0C\0\x1E\x002\0\x03\x03\xE2\x80\x8F \xD7\x90\xD7\x9C\xD7\xA3\x06\x03\xE2\x80\x8F \xD7\x9E\xD7\x99\xD7\x9C\xD7\x99\xD7\x95\xD7\x9F\t\x03\xE2\x80\x8F \xD7\x9E\xD7\x99\xD7\x9C\xD7\x99\xD7\x90\xD7\xA8\xD7\x93\x0C\x03\xE2\x80\x8F \xD7\x98\xD7\xA8\xD7\x99\xD7\x9C\xD7\x99\xD7\x95\xD7\x9F") })
                    },
                };
                static BRX: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0F\0!\x006\0\x03\0 \xE0\xA4\xB0\xE0\xA5\x8B\xE0\xA4\x9C\xE0\xA4\xBE\x06\0 \xE0\xA4\xA8\xE0\xA4\xBF\xE0\xA4\x9C\xE0\xA5\x81\xE0\xA4\xA4\t\0 \xE0\xA4\xAC\xE0\xA4\xBF\xE0\xA4\xB2\xE0\xA4\xBF\xE0\xA4\xAF\xE0\xA4\xA8\x0C\0 \xE0\xA4\xA4\xE0\xA5\x8D\xE0\xA4\xB0\xE0\xA4\xBF\xE0\xA4\xB2\xE0\xA4\xBF\xE0\xA4\xAF\xE0\xA4\xA8") })
                    },
                };
                static OR: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0F\0!\x006\0\x03\0 \xE0\xAC\xB9\xE0\xAC\x9C\xE0\xAC\xBE\xE0\xAC\xB0\x06\0 \xE0\xAC\xA8\xE0\xAC\xBF\xE0\xAD\x9F\xE0\xAD\x81\xE0\xAC\xA4\t\0 \xE0\xAC\xB6\xE0\xAC\xB9\xE0\xAC\x95\xE0\xAD\x8B\xE0\xAC\x9F\xE0\xAC\xBF\x0C\0 \xE0\xAC\xB2\xE0\xAC\x95\xE0\xAD\x8D\xE0\xAC\xB7\xE0\xAC\x95\xE0\xAD\x8B\xE0\xAC\x9F\xE0\xAC\xBF") })
                    },
                };
                static KOK: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0F\0$\x003\0\x03\0 \xE0\xA4\xB9\xE0\xA4\x9C\xE0\xA4\xBE\xE0\xA4\xB0\x06\0 \xE0\xA4\xA6\xE0\xA4\xB6\xE0\xA4\xB2\xE0\xA4\x95\xE0\xA5\x8D\xE0\xA4\xB7\t\0 \xE0\xA4\x85\xE0\xA4\xAC\xE0\xA5\x8D\xE0\xA4\x9C\x0C\0 \xE0\xA4\x9F\xE0\xA5\x8D\xE0\xA4\xB0\xE0\xA4\xBF\xE0\xA4\xB2\xE0\xA4\xBF\xE0\xA4\xAF\xE0\xA4\xA8") })
                    },
                };
                static ZU: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0F\0\x19\0,\0\x03\0 inkulungwane\x06\0 isigidi\t\0 isigidi sezigidi\x0C\0 isigidintathu") })
                    },
                };
                static YO_BJ: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x0F\0\x1F\0.\0\x03\0 \xC9\x9Bgb\xC9\x9B\xCC\x80r\xC3\xBAn\x06\0 m\xC3\xADl\xC3\xAD\xC9\x94\xCC\x80n\xC3\xB9\t\0 bil\xC3\xAD\xC9\x94\xCC\x80n\xC3\xB9\x0C\0 tirili\xC9\x94\xCC\x80n\xC3\xB9") })
                    },
                };
                static YO: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x11\0\"\x002\0\x03\0 \xE1\xBA\xB9gb\xE1\xBA\xB9\xCC\x80r\xC3\xBAn\x06\0 m\xC3\xADl\xC3\xAD\xE1\xBB\x8D\xCC\x80n\xC3\xB9\t\0 bil\xC3\xAD\xE1\xBB\x8D\xCC\x80n\xC3\xB9\x0C\0 tirili\xE1\xBB\x8D\xCC\x80n\xC3\xB9") })
                    },
                };
                static CHR: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x12\0'\0?\0\x03\0 \xE1\x8E\xA2\xE1\x8F\xAF\xE1\x8E\xA6\xE1\x8F\xB4\xE1\x8E\xB5\x06\0 \xE1\x8E\xA2\xE1\x8F\xB3\xE1\x8F\x86\xE1\x8F\x97\xE1\x8F\x85\xE1\x8F\x9B\t\0 \xE1\x8E\xA2\xE1\x8F\xAF\xE1\x8F\x94\xE1\x8E\xB3\xE1\x8F\x97\xE1\x8F\x85\xE1\x8F\x9B\x0C\0 \xE1\x8E\xA2\xE1\x8F\xAF\xE1\x8F\xA6\xE1\x8E\xA0\xE1\x8F\x97\xE1\x8F\x85\xE1\x8F\x9B") })
                    },
                };
                static KN: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x12\0*\0B\0\x03\0 \xE0\xB2\xB8\xE0\xB2\xBE\xE0\xB2\xB5\xE0\xB2\xBF\xE0\xB2\xB0\x06\0 \xE0\xB2\xAE\xE0\xB2\xBF\xE0\xB2\xB2\xE0\xB2\xBF\xE0\xB2\xAF\xE0\xB2\xA8\xE0\xB3\x8D\t\0 \xE0\xB2\xAC\xE0\xB2\xBF\xE0\xB2\xB2\xE0\xB2\xBF\xE0\xB2\xAF\xE0\xB2\xA8\xE0\xB3\x8D\x0C\0 \xE0\xB2\x9F\xE0\xB3\x8D\xE0\xB2\xB0\xE0\xB2\xBF\xE0\xB2\xB2\xE0\xB2\xBF\xE0\xB2\xAF\xE0\xB2\xA8\xE0\xB3\x8D\xE2\x80\x8C") })
                    },
                };
                static KA: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x12\0*\0E\0\x03\0 \xE1\x83\x90\xE1\x83\x97\xE1\x83\x90\xE1\x83\xA1\xE1\x83\x98\x06\0 \xE1\x83\x9B\xE1\x83\x98\xE1\x83\x9A\xE1\x83\x98\xE1\x83\x9D\xE1\x83\x9C\xE1\x83\x98\t\0 \xE1\x83\x9B\xE1\x83\x98\xE1\x83\x9A\xE1\x83\x98\xE1\x83\x90\xE1\x83\xA0\xE1\x83\x93\xE1\x83\x98\x0C\0 \xE1\x83\xA2\xE1\x83\xA0\xE1\x83\x98\xE1\x83\x9A\xE1\x83\x98\xE1\x83\x9D\xE1\x83\x9C\xE1\x83\x98") })
                    },
                };
                static ML: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x12\0*\0I\0\x03\0 \xE0\xB4\x86\xE0\xB4\xAF\xE0\xB4\xBF\xE0\xB4\xB0\xE0\xB4\x82\x06\0 \xE0\xB4\xA6\xE0\xB4\xB6\xE0\xB4\xB2\xE0\xB4\x95\xE0\xB5\x8D\xE0\xB4\xB7\xE0\xB4\x82\t\0 \xE0\xB4\xB2\xE0\xB4\x95\xE0\xB5\x8D\xE0\xB4\xB7\xE0\xB4\x82 \xE0\xB4\x95\xE0\xB5\x8B\xE0\xB4\x9F\xE0\xB4\xBF\x0C\0 \xE0\xB4\x9F\xE0\xB5\x8D\xE0\xB4\xB0\xE0\xB4\xBF\xE0\xB4\xB2\xE0\xB5\x8D\xE0\xB4\xAF\xE0\xB5\xBA") })
                    },
                };
                static TA: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x15\x003\0Q\0\x03\0 \xE0\xAE\x86\xE0\xAE\xAF\xE0\xAE\xBF\xE0\xAE\xB0\xE0\xAE\xAE\xE0\xAF\x8D\x06\0 \xE0\xAE\xAE\xE0\xAE\xBF\xE0\xAE\xB2\xE0\xAF\x8D\xE0\xAE\xB2\xE0\xAE\xBF\xE0\xAE\xAF\xE0\xAE\xA9\xE0\xAF\x8D\t\0 \xE0\xAE\xAA\xE0\xAE\xBF\xE0\xAE\xB2\xE0\xAF\x8D\xE0\xAE\xB2\xE0\xAE\xBF\xE0\xAE\xAF\xE0\xAE\xA9\xE0\xAF\x8D\x0C\0 \xE0\xAE\x9F\xE0\xAE\xBF\xE0\xAE\xB0\xE0\xAE\xBF\xE0\xAE\xB2\xE0\xAF\x8D\xE0\xAE\xB2\xE0\xAE\xBF\xE0\xAE\xAF\xE0\xAE\xA9\xE0\xAF\x8D") })
                    },
                };
                static FF_ADLM: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x1F\0B\0e\0\x03\0 \xF0\x9E\xA4\xB5\xF0\x9E\xA4\xB6\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xB2\xF0\x9E\xA4\xAB\xF0\x9E\xA4\xAA\xF0\x9E\xA4\xAB\x06\0 \xF0\x9E\xA4\xA3\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xA6\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xB2\xF0\x9E\xA4\xAB\xF0\x9E\xA4\xAA\xF0\x9E\xA4\xAB\t\0 \xF0\x9E\xA4\xA3\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xB6\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xB2\xF0\x9E\xA4\xAB\xF0\x9E\xA4\xAA\xF0\x9E\xA4\xAB\x0C\0 \xF0\x9E\xA4\xBC\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xB6\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xB2\xF0\x9E\xA4\xAB\xF0\x9E\xA4\xAA\xF0\x9E\xA4\xAB") })
                    },
                };
                static PT: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x03\0\0\0\x05\0\0\0\x07\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x01\x05\x01\x05\x01\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x10\0\x1B\0%\x000\0;\0\x03\0 mil\x06\0 milh\xC3\xA3o\x06\0 milh\xC3\xB5es\t\0 bilh\xC3\xA3o\t\0 bilh\xC3\xB5es\x0C\0 trilh\xC3\xA3o\x0C\0 trilh\xC3\xB5es") })
                    },
                };
                static KGP: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x03\0\0\0\x05\0\0\0\x07\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x01\x05\x01\x05\x01\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x10\0\x1D\0'\x004\0?\0\x03\0 mil\x06\0 milh\xC3\xA3o\x06\0 milh\xC3\xA3o ag\t\0 bilh\xC3\xA3o\t\0 bilh\xC3\xA3o ag\x0C\0 trilh\xC3\xA3o\x0C\0 trilh\xC3\xA3o ag") })
                    },
                };
                static YRL: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x03\0\0\0\x05\0\0\0\x07\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x01\x05\x01\x05\x01\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x10\0\x1E\0(\x006\0B\0\x03\0 miu\x06\0 mili\xC3\xA3u\x06\0 mili\xC3\xA3u-ita\t\0 bili\xC3\xA3u\t\0 bili\xC3\xA3u-ita\x0C\0 tirili\xC3\xA3u\x0C\0 tirili\xC3\xA3u-ita") })
                    },
                };
                static YRL_CO: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x03\0\0\0\x05\0\0\0\x07\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x01\x05\x01\x05\x01\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x10\0\x1E\0(\x006\0B\0\x03\0 miu\x06\0 mill\xC3\xB3n\x06\0 mill\xC3\xB3n-ita\t\0 bill\xC3\xB3n\t\0 bill\xC3\xB3n-ita\x0C\0 tirill\xC3\xB3n\x0C\0 tirill\xC3\xB3n-ita") })
                    },
                };
                static FR_CA: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x03\0\0\0\x05\0\0\0\x07\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x01\x05\x01\x05\x01\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x08\0\x12\0\x1D\0(\x004\0>\0\x03\0 mille\x06\0 million\x06\0 millions\t\0 milliard\t\0 milliards\x0C\0 billion\x0C\0 billions") })
                    },
                };
                static NN: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x03\0\0\0\x05\0\0\0\x07\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x01\x05\x01\x05\x01\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x08\0\x12\0\x1E\0)\x006\0@\0\x03\0 tusen\x06\0 million\x06\0 millionar\t\0 milliard\t\0 milliardar\x0C\0 billion\x0C\0 billionar") })
                    },
                };
                static IS: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x03\0\0\0\x05\0\0\0\x07\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x01\x05\x01\x05\x01\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0B\0\x16\0#\x001\0?\0J\0\x03\0 \xC3\xBE\xC3\xBAsund\x06\0 millj\xC3\xB3n\x06\0 millj\xC3\xB3nir\t\0 milljar\xC3\xB0ur\t\0 milljar\xC3\xB0ar\x0C\0 billj\xC3\xB3n\x0C\0 billj\xC3\xB3nir") })
                    },
                };
                static FIL: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x05\x01\x05\x01\x05\x01\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x08\0\0\0\0\0\x07\0\x11\0\x1A\0&\0/\0;\0E\0\x03\0 libo\x03\0 na libo\x06\0 milyon\x06\0 na milyon\t\0 bilyon\t\0 na bilyon\x0C\0 trilyon\x0C\0 na trilyon") })
                    },
                };
                static IA: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x05\x01\x05\x01\x05\x01\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x08\0\0\0\0\0\x08\0\x11\0\x1B\0'\x003\0@\0J\0\x03\0 mille\x03\0 milles\x06\0 million\x06\0 milliones\t\0 milliardo\t\0 milliardos\x0C\0 billion\x0C\0 billiones") })
                    },
                };
                static MK: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x05\x01\x05\x01\x05\x01\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x08\0\0\0\0\0\x0F\0\x1E\0-\0>\0S\0h\0w\0\x03\0 \xD0\xB8\xD0\xBB\xD1\x98\xD0\xB0\xD0\xB4\xD0\xB0\x03\0 \xD0\xB8\xD0\xBB\xD1\x98\xD0\xB0\xD0\xB4\xD0\xB8\x06\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\x06\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\xD0\xB8\t\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xB8\xD1\x98\xD0\xB0\xD1\x80\xD0\xB4\xD0\xB0\t\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xB8\xD1\x98\xD0\xB0\xD1\x80\xD0\xB4\xD0\xB8\x0C\0 \xD0\xB1\xD0\xB8\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\x0C\0 \xD0\xB1\xD0\xB8\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\xD0\xB8") })
                    },
                };
                static BS: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x07\0\0\0\t\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x01\x05\x01\x03\x05\x01\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\t\0\0\0\0\0\n\0\x14\0\x1D\0'\x003\0?\0K\0T\0\x03\0 hiljade\x03\0 hiljada\x06\0 milion\x06\0 miliona\t\0 milijarda\t\0 milijarde\t\0 milijardi\x0C\0 bilion\x0C\0 biliona") })
                    },
                };
                static HR: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x07\0\0\0\t\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x01\x05\x01\x03\x05\x01\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\t\0\0\0\0\0\n\0\x14\0\x1E\0)\x005\0A\0M\0W\0\x03\0 tisu\xC4\x87e\x03\0 tisu\xC4\x87a\x06\0 milijun\x06\0 milijuna\t\0 milijarda\t\0 milijarde\t\0 milijardi\x0C\0 bilijun\x0C\0 bilijuna") })
                    },
                };
                static SR: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x07\0\0\0\t\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x05\x01\x05\x01\x03\x05\x01\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\t\0\0\0\0\0\x0F\0\x1E\0-\0>\0S\0h\0}\0\x8C\0\x03\0 \xD1\x85\xD0\xB8\xD1\x99\xD0\xB0\xD0\xB4\xD0\xB5\x03\0 \xD1\x85\xD0\xB8\xD1\x99\xD0\xB0\xD0\xB4\xD0\xB0\x06\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\x06\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\xD0\xB0\t\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xB8\xD1\x98\xD0\xB0\xD1\x80\xD0\xB4\xD0\xB0\t\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xB8\xD1\x98\xD0\xB0\xD1\x80\xD0\xB4\xD0\xB5\t\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xB8\xD1\x98\xD0\xB0\xD1\x80\xD0\xB4\xD0\xB8\x0C\0 \xD0\xB1\xD0\xB8\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\x0C\0 \xD0\xB1\xD0\xB8\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\xD0\xB0") })
                    },
                };
                static RO: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\0\0\0\x06\0\0\0\t\0\0\0\x0C\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x03\x05\x01\x03\x05\x01\x03\x05\x01\x03\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x06\0\x0C\0\x15\0\x1E\0)\x007\0A\0L\0Z\0d\0p\0\x03\0 mie\x03\0 mii\x03\0 de mii\x06\0 milion\x06\0 milioane\x06\0 de milioane\t\0 miliard\t\0 miliarde\t\0 de miliarde\x0C\0 trilion\x0C\0 trilioane\x0C\0 de trilioane") })
                    },
                };
                static RU: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\0\0\0\x06\0\0\0\t\0\0\0\x0C\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x04\x05\x01\x04\x05\x01\x04\x05\x01\x04\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x0F\0\x1C\0+\0<\0Q\0d\0w\0\x8E\0\xA3\0\xB6\0\xCD\0\x03\0 \xD1\x82\xD1\x8B\xD1\x81\xD1\x8F\xD1\x87\xD0\xB0\x03\0 \xD1\x82\xD1\x8B\xD1\x81\xD1\x8F\xD1\x87\x03\0 \xD1\x82\xD1\x8B\xD1\x81\xD1\x8F\xD1\x87\xD0\xB8\x06\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\x06\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\xD0\xBE\xD0\xB2\x06\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\xD0\xB0\t\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xB0\xD1\x80\xD0\xB4\t\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xB0\xD1\x80\xD0\xB4\xD0\xBE\xD0\xB2\t\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xB0\xD1\x80\xD0\xB4\xD0\xB0\x0C\0 \xD1\x82\xD1\x80\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\x0C\0 \xD1\x82\xD1\x80\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\xD0\xBE\xD0\xB2\x0C\0 \xD1\x82\xD1\x80\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\xD0\xB0") })
                    },
                };
                static BR: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\0\0\0\x06\0\0\0\t\0\0\0\x0C\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\x04\x05\x02\x04\x05\x02\x04\x05\x02\x04\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\t\0\x17\0 \0+\0;\0D\0N\0]\0g\0r\0\x82\0\x03\0 viliad\x03\0 a viliado\xC3\xB9\x03\0 miliad\x06\0 v/milion\x06\0 a v/miliono\xC3\xB9\x06\0 milion\t\0 viliard\t\0 a viliardo\xC3\xB9\t\0 miliard\x0C\0 v/bilion\x0C\0 a v/biliono\xC3\xB9\x0C\0 bilion") })
                    },
                };
                static BE: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\0\0\0\x07\0\0\0\x0B\0\0\0\x0F\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x04\x05\x01\x03\x04\x05\x01\x03\x04\x05\x01\x03\x04\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0F\0\0\0\0\0\x0F\0\x1C\0+\0:\0K\0^\0o\0\x80\0\x93\0\xA8\0\xBB\0\xCC\0\xDF\0\xF4\0\x03\0 \xD1\x82\xD1\x8B\xD1\x81\xD1\x8F\xD1\x87\xD0\xB0\x03\0 \xD1\x82\xD1\x8B\xD1\x81\xD1\x8F\xD1\x87\x03\0 \xD1\x82\xD1\x8B\xD1\x81\xD1\x8F\xD1\x87\xD1\x8B\x06\0 \xD0\xBC\xD1\x96\xD0\xBB\xD1\x8C\xD1\x91\xD0\xBD\x06\0 \xD0\xBC\xD1\x96\xD0\xBB\xD1\x8C\xD1\x91\xD0\xBD\xD1\x8B\x06\0 \xD0\xBC\xD1\x96\xD0\xBB\xD1\x8C\xD1\x91\xD0\xBD\xD0\xB0\xD1\x9E\x06\0 \xD0\xBC\xD1\x96\xD0\xBB\xD1\x8C\xD1\x91\xD0\xBD\xD0\xB0\t\0 \xD0\xBC\xD1\x96\xD0\xBB\xD1\x8C\xD1\x8F\xD1\x80\xD0\xB4\t\0 \xD0\xBC\xD1\x96\xD0\xBB\xD1\x8C\xD1\x8F\xD1\x80\xD0\xB4\xD1\x8B\t\0 \xD0\xBC\xD1\x96\xD0\xBB\xD1\x8C\xD1\x8F\xD1\x80\xD0\xB4\xD0\xB0\xD1\x9E\t\0 \xD0\xBC\xD1\x96\xD0\xBB\xD1\x8C\xD1\x8F\xD1\x80\xD0\xB4\xD0\xB0\x0C\0 \xD1\x82\xD1\x80\xD1\x8B\xD0\xBB\xD1\x8C\xD1\x91\xD0\xBD\x0C\0 \xD1\x82\xD1\x80\xD1\x8B\xD0\xBB\xD1\x8C\xD1\x91\xD0\xBD\xD1\x8B\x0C\0 \xD1\x82\xD1\x80\xD1\x8B\xD0\xBB\xD1\x8C\xD1\x91\xD0\xBD\xD0\xB0\xD1\x9E\x0C\0 \xD1\x82\xD1\x80\xD1\x8B\xD0\xBB\xD1\x8C\xD1\x91\xD0\xBD\xD0\xB0") })
                    },
                };
                static UK: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\0\0\0\x07\0\0\0\x0B\0\0\0\x0F\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x04\x05\x01\x03\x04\x05\x01\x03\x04\x05\x01\x03\x04\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0F\0\0\0\0\0\x0F\0\x1C\0+\0<\0O\0d\0w\0\x88\0\x9B\0\xB0\0\xC3\0\xD6\0\xEB\0\x02\x01\x03\0 \xD1\x82\xD0\xB8\xD1\x81\xD1\x8F\xD1\x87\xD0\xB0\x03\0 \xD1\x82\xD0\xB8\xD1\x81\xD1\x8F\xD1\x87\x03\0 \xD1\x82\xD0\xB8\xD1\x81\xD1\x8F\xD1\x87\xD1\x96\x06\0 \xD0\xBC\xD1\x96\xD0\xBB\xD1\x8C\xD0\xB9\xD0\xBE\xD0\xBD\x06\0 \xD0\xBC\xD1\x96\xD0\xBB\xD1\x8C\xD0\xB9\xD0\xBE\xD0\xBD\xD0\xB8\x06\0 \xD0\xBC\xD1\x96\xD0\xBB\xD1\x8C\xD0\xB9\xD0\xBE\xD0\xBD\xD1\x96\xD0\xB2\x06\0 \xD0\xBC\xD1\x96\xD0\xBB\xD1\x8C\xD0\xB9\xD0\xBE\xD0\xBD\xD0\xB0\t\0 \xD0\xBC\xD1\x96\xD0\xBB\xD1\x8C\xD1\x8F\xD1\x80\xD0\xB4\t\0 \xD0\xBC\xD1\x96\xD0\xBB\xD1\x8C\xD1\x8F\xD1\x80\xD0\xB4\xD0\xB8\t\0 \xD0\xBC\xD1\x96\xD0\xBB\xD1\x8C\xD1\x8F\xD1\x80\xD0\xB4\xD1\x96\xD0\xB2\t\0 \xD0\xBC\xD1\x96\xD0\xBB\xD1\x8C\xD1\x8F\xD1\x80\xD0\xB4\xD0\xB0\x0C\0 \xD1\x82\xD1\x80\xD0\xB8\xD0\xBB\xD1\x8C\xD0\xB9\xD0\xBE\xD0\xBD\x0C\0 \xD1\x82\xD1\x80\xD0\xB8\xD0\xBB\xD1\x8C\xD0\xB9\xD0\xBE\xD0\xBD\xD0\xB8\x0C\0 \xD1\x82\xD1\x80\xD0\xB8\xD0\xBB\xD1\x8C\xD0\xB9\xD0\xBE\xD0\xBD\xD1\x96\xD0\xB2\x0C\0 \xD1\x82\xD1\x80\xD0\xB8\xD0\xBB\xD1\x8C\xD0\xB9\xD0\xBE\xD0\xBD\xD0\xB0") })
                    },
                };
                static GD: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x04\0\0\0\x08\0\0\0\x0C\0\0\0\x0F\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x02\x03\x05\x01\x02\x03\x05\x01\x02\x03\x05\x02\x03\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0F\0\0\0\0\0\t\0\x12\0\x1D\0%\x000\0;\0G\0Q\0\\\0g\0s\0}\0\x89\0\x96\0\x03\0 mh\xC3\xACle\x03\0 mh\xC3\xACle\x03\0 m\xC3\xACltean\x03\0 m\xC3\xACle\x06\0 mhillean\x06\0 mhillean\x06\0 milleanan\x06\0 millean\t\0 bhillean\t\0 bhillean\t\0 billeanan\t\0 billean\x0C\0 thrillean\x0C\0 trilleanan\x0C\0 trillean") })
                    },
                };
                static PL: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x04\0\0\0\x08\0\0\0\x0C\0\0\0\x10\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x03\x04\x05\x01\x03\x04\x05\x01\x03\x04\x05\x01\x03\x04\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x10\0\0\0\0\0\n\0\x15\0 \0+\x004\0>\0J\0T\0^\0i\0v\0\x81\0\x8A\0\x94\0\xA0\0\x03\0 tysi\xC4\x85c\x03\0 tysi\xC4\x85ce\x03\0 tysi\xC4\x99cy\x03\0 tysi\xC4\x85ca\x06\0 milion\x06\0 miliony\x06\0 milion\xC3\xB3w\x06\0 miliona\t\0 miliard\t\0 miliardy\t\0 miliard\xC3\xB3w\t\0 miliarda\x0C\0 bilion\x0C\0 biliony\x0C\0 bilion\xC3\xB3w\x0C\0 biliona") })
                    },
                };
                static LT: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x04\0\0\0\x08\0\0\0\x0C\0\0\0\x10\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x03\x04\x05\x01\x03\x04\x05\x01\x03\x04\x05\x01\x03\x04\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x10\0\0\0\0\0\x0E\0\x1E\0-\0=\0I\0U\0`\0l\0y\0\x86\0\x92\0\x9F\0\xAC\0\xB9\0\xC5\0\x03\0 t\xC5\xABkstantis\x03\0 t\xC5\xABkstan\xC4\x8Diai\x03\0 t\xC5\xABkstan\xC4\x8Dio\x03\0 t\xC5\xABkstan\xC4\x8Di\xC5\xB3\x06\0 milijonas\x06\0 milijonai\x06\0 milijono\x06\0 milijon\xC5\xB3\t\0 milijardas\t\0 milijardai\t\0 milijardo\t\0 milijard\xC5\xB3\x0C\0 trilijonas\x0C\0 trilijonai\x0C\0 trilijono\x0C\0 trilijon\xC5\xB3") })
                    },
                };
                static SC: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\x07\t\n\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x03\0\0\0\x04\0\0\0\x06\0\0\0\x07\0\0\0\x08\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x01\x05\x05\x01\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x08\0\0\0\0\0\t\0\x14\0 \0,\08\0E\0R\0\x03\0 m\xC3\xACgia\x06\0 millione\x06\0 milliones\x06\0 milliones\t\0 milliardu\t\0 milliardos\t\0 milliardos\x0C\0 m\xC3\xACgia milliardos") })
                    },
                };
                static DE: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\x07\t\n\x0C\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x03\0\0\0\x04\0\0\0\x06\0\0\0\x07\0\0\0\t\0\0\0\n\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x01\x05\x05\x01\x05\x05\x01\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\n\0\0\0\0\0\n\0\x14\0 \0,\08\0E\0R\0\\\0h\0\x03\0 Tausend\x06\0 Million\x06\0 Millionen\x06\0 Millionen\t\0 Milliarde\t\0 Milliarden\t\0 Milliarden\x0C\0 Billion\x0C\0 Billionen\x0C\0 Billionen") })
                    },
                };
                static FO: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\x07\t\n\x0C\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x03\0\0\0\x04\0\0\0\x06\0\0\0\x07\0\0\0\t\0\0\0\n\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x01\x05\x05\x01\x05\x05\x01\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\n\0\0\0\0\0\n\0\x15\0\"\0/\0:\0G\0T\0_\0l\0\x03\0 t\xC3\xBAsund\x06\0 milli\xC3\xB3n\x06\0 milli\xC3\xB3nir\x06\0 milli\xC3\xB3nir\t\0 milliard\t\0 milliardir\t\0 milliardir\x0C\0 billi\xC3\xB3n\x0C\0 billi\xC3\xB3nir\x0C\0 billi\xC3\xB3nir") })
                    },
                };
                static DA: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\x07\t\n\x0C\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x03\0\0\0\x04\0\0\0\x06\0\0\0\x07\0\0\0\t\0\0\0\n\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x01\x05\x05\x01\x05\x05\x01\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\n\0\0\0\0\0\t\0\x13\0\x1F\0+\x006\0C\0P\0Z\0f\0\x03\0 tusind\x06\0 million\x06\0 millioner\x06\0 millioner\t\0 milliard\t\0 milliarder\t\0 milliarder\x0C\0 billion\x0C\0 billioner\x0C\0 billioner") })
                    },
                };
                static ET: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\x07\t\n\x0C\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x03\0\0\0\x04\0\0\0\x06\0\0\0\x07\0\0\0\t\0\0\0\n\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x01\x05\x05\x01\x05\x05\x01\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\n\0\0\0\0\0\x08\0\x11\0\x1C\0'\x001\0=\0I\0S\0_\0\x03\0 tuhat\x06\0 miljon\x06\0 miljonit\x06\0 miljonit\t\0 miljard\t\0 miljardit\t\0 miljardit\x0C\0 triljon\x0C\0 triljonit\x0C\0 triljonit") })
                    },
                };
                static NB: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\x07\t\n\x0C\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x03\0\0\0\x04\0\0\0\x06\0\0\0\x07\0\0\0\t\0\0\0\n\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x01\x05\x05\x01\x05\x05\x01\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\n\0\0\0\0\0\x08\0\x12\0\x1E\0*\x005\0B\0O\0Y\0e\0\x03\0 tusen\x06\0 million\x06\0 millioner\x06\0 millioner\t\0 milliard\t\0 milliarder\t\0 milliarder\x0C\0 billion\x0C\0 billioner\x0C\0 billioner") })
                    },
                };
                static DSB: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\x07\t\n\x0C\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x05\0\0\0\x06\0\0\0\n\0\0\0\x0B\0\0\0\x0F\0\0\0\x10\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x01\x02\x03\x05\x05\x01\x02\x03\x05\x05\x01\x02\x03\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x10\0\0\0\0\0\x08\0\x11\0\x1B\0%\x000\0;\0F\0R\0]\0i\0u\0~\0\x88\0\x92\0\x9D\0\x03\0 tysac\x06\0 milion\x06\0 miliona\x06\0 miliony\x06\0 milionow\x06\0 milionow\t\0 miliarda\t\0 miliar\xC5\xBAe\t\0 miliardy\t\0 miliardow\t\0 miliardow\x0C\0 bilion\x0C\0 biliona\x0C\0 biliony\x0C\0 bilionow\x0C\0 bilionow") })
                    },
                };
                static HSB: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\x07\t\n\x0C\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x05\0\0\0\x06\0\0\0\n\0\0\0\x0B\0\0\0\x0F\0\0\0\x10\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x01\x02\x03\x05\x05\x01\x02\x03\x05\x05\x01\x02\x03\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x10\0\0\0\0\0\x08\0\x11\0\x1C\0&\x001\0<\0G\0T\0_\0k\0w\0\x80\0\x8B\0\x95\0\xA0\0\x03\0 tysac\x06\0 milion\x06\0 milionaj\x06\0 miliony\x06\0 milionow\x06\0 milionow\t\0 miliarda\t\0 miliard\xC5\xBAe\t\0 miliardy\t\0 miliardow\t\0 miliardow\x0C\0 bilion\x0C\0 bilionaj\x0C\0 biliony\x0C\0 bilionow\x0C\0 bilionow") })
                    },
                };
                static SL: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\x07\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x05\0\0\0\t\0\0\0\r\0\0\0\x11\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x01\x02\x03\x05\x01\x02\x03\x05\x01\x02\x03\x05\x01\x02\x03\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x11\0\0\0\0\0\t\0\x13\0\x1E\0)\x005\0?\0J\0U\0a\0m\0y\0\x85\0\x90\0\x9A\0\xA5\0\xB0\0\x03\0 tiso\xC4\x8D\x06\0 milijon\x06\0 milijona\x06\0 milijone\x06\0 milijonov\x06\0 milijon\x06\0 milijona\x06\0 milijoni\x06\0 milijonov\t\0 milijarda\t\0 milijardi\t\0 milijarde\t\0 milijard\x0C\0 bilijon\x0C\0 bilijona\x0C\0 bilijoni\x0C\0 bilijonov") })
                    },
                };
                static ES_419: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\x07\t\x0C\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0\x07\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x01\x05\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x10\0\x1B\0&\x005\0?\0\x03\0 mil\x06\0 mill\xC3\xB3n\x06\0 millones\x06\0 millones\t\0 mil millones\x0C\0 bill\xC3\xB3n\x0C\0 billones") })
                    },
                };
                static PT_AO: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\x07\t\x0C\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x07\0\0\0\x08\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x01\x05\x05\x05\x01\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x08\0\0\0\0\0\x06\0\x10\0\x1B\0&\x005\0?\0J\0\x03\0 mil\x06\0 milh\xC3\xA3o\x06\0 milh\xC3\xB5es\x06\0 milh\xC3\xB5es\t\0 mil milh\xC3\xB5es\x0C\0 bili\xC3\xA3o\x0C\0 bili\xC3\xB5es\x0C\0 bili\xC3\xB5es") })
                    },
                };
                static ES: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\x07\t\x0C\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x07\0\0\0\x08\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x01\x05\x05\x05\x01\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x08\0\0\0\0\0\x06\0\x10\0\x1B\0&\x005\0?\0J\0\x03\0 mil\x06\0 mill\xC3\xB3n\x06\0 millones\x06\0 millones\t\0 mil millones\x0C\0 bill\xC3\xB3n\x0C\0 billones\x0C\0 billones") })
                    },
                };
                static SV: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x06\x08\t\n\x0C\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x03\0\0\0\x04\0\0\0\x06\0\0\0\x07\0\0\0\t\0\0\0\n\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x01\x05\x05\x01\x05\x05\x01\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\n\0\0\0\0\0\x08\0\x11\0\x1C\0'\x001\0=\0I\0R\0]\0\x03\0 tusen\x06\0 miljon\x06\0 miljoner\x06\0 miljoner\t\0 miljard\t\0 miljarder\t\0 miljarder\x0C\0 biljon\x0C\0 biljoner\x0C\0 biljoner") })
                    },
                };
                static BS_CYRL: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x04\x06\t\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\t\0\x12\0\x1D\0\x03\0 \xD1\x85\xD0\xB8\xD1\x99\x06\0 \xD0\xBC\xD0\xB8\xD0\xBB\t\0 \xD0\xBC\xD0\xBB\xD1\x80\xD0\xB4\x0C\0 \xD0\xB1\xD0\xB8\xD0\xBB") })
                    },
                };
                static ZH: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x04\x08\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0\x04\0\xE4\xB8\x87\x08\0\xE4\xBA\xBF\x0C\0\xE4\xB8\x87\xE4\xBA\xBF") })
                    },
                };
                static YUE_HANS: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x04\x08\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0\x04\0\xE4\xB8\x87\x08\0\xE4\xBA\xBF\x0C\0\xE5\x85\x86") })
                    },
                };
                static ZH_HANT: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x04\x08\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x05\0\n\0\x04\0\xE8\x90\xAC\x08\0\xE5\x84\x84\x0C\0\xE5\x85\x86") })
                    },
                };
                static JA: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x04\x08\x0C\x10") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x05\0\n\0\x0F\0\x04\0\xE4\xB8\x87\x08\0\xE5\x84\x84\x0C\0\xE5\x85\x86\x10\0\xE4\xBA\xAC") })
                    },
                };
                static GL: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x06\x07\x0C\r") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0\x05\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x05\x05\x01\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\n\0\x15\0 \0*\x005\0\x06\0 mill\xC3\xB3n\x06\0 mill\xC3\xB3ns\x06\0 mill\xC3\xB3ns\x0C\0 bill\xC3\xB3n\x0C\0 bill\xC3\xB3ns\x0C\0 bill\xC3\xB3ns") })
                    },
                };
                static EU: <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
                    patterns: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x06\x0C") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x05") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\t\0\x06\0 milioi\x0C\0 bilioi") })
                    },
                };
                static VALUES: [&<icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::Yokeable; 538usize] = [&AF, &AF, &AM, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AS, &AS, &AST, &AZ, &AZ, &BE, &BE, &BG, &BGC, &BGC, &BGC, &BGC, &BN, &BN, &BN, &BN, &BR, &BRX, &BRX, &BS, &BS_CYRL, &BS, &CA, &CA, &CA, &CA, &CA, &BGC, &CHR, &CS, &BGC, &CY, &DA, &DA, &DE, &DE, &DE, &DE, &DE, &DE, &DE, &BGC, &BGC, &DSB, &EL, &EL, &EL, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &EN, &ES, &ES_419, &ES_419, &ES_419, &ES_419, &ES_419, &ES_419, &ES_419, &ES_419, &ES_419, &ES_419, &ES, &ES_419, &ES, &ES_419, &ES_419, &ES, &ES, &ES_419, &ES_419, &ES_419, &ES, &ES_419, &ES_419, &ES_419, &ES_419, &ES_419, &ES_419, &ET, &EU, &FA, &FA, &FA, &FA, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FI, &FIL, &FO, &FO, &FR, &FR, &FR, &FR, &FR, &FR, &FR_CA, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &FR, &GA, &GA, &GD, &GL, &GU, &GU, &HA, &HA, &HA, &HE, &HI, &EN, &HI, &HR, &HR, &HSB, &HU, &HY, &IA, &ID, &BGC, &IS, &IT, &IT, &IT, &IT, &JA, &JV, &JV, &KA, &KEA, &KGP, &KK, &KM, &KM, &KN, &KN, &KO, &KO, &KOK, &KOK, &BGC, &BGC, &BGC, &BGC, &BGC, &KY, &LO, &LO_U_NU_LAOO, &LT, &LV, &BGC, &BGC, &BGC, &MK, &ML, &ML, &MN, &BGC, &BGC, &BGC, &BGC, &MR, &MR, &MS, &MS, &MS, &MS, &MY, &MY, &NB, &NB, &NE, &NE, &NE, &NE, &NL, &NL, &NL, &NL, &NL, &NL, &NL, &NN, &NB, &OR, &OR, &PA, &PA, &PA, &PA, &PCM, &PL, &BGC, &BGC, &BGC, &BGC, &PT, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &PT_AO, &BGC, &BGC, &BGC, &BGC, &BGC, &BGC, &RO, &RO, &RU, &RU, &RU, &RU, &RU, &RU, &BGC, &BGC, &BGC, &BGC, &BGC, &BGC, &SC, &SD, &SD, &SD, &BGC, &SD, &SI, &SK, &SL, &SO, &SO, &SO, &SO, &SQ, &SQ, &SQ, &SR, &SR, &SR, &SR, &BS, &BS, &BS, &BS, &BGC, &BGC, &SV, &SV, &SV, &SW, &SW, &SW, &SW, &TA, &TA, &TA, &TA, &TA, &TA, &TA, &TA, &TE, &TE, &TG, &TH, &TH, &TI, &TI, &TK, &TO, &TR, &TR, &BGC, &UK, &BGC, &UR, &UR, &UR, &UR, &UZ, &UZ_CYRL, &UZ, &VI, &BGC, &BGC, &YO, &YO_BJ, &YRL, &YRL_CO, &YRL_CO, &YUE, &YUE_HANS, &YUE_HANS, &YUE, &YUE, &YUE, &ZH, &ZH, &ZH, &ZH, &ZH, &ZH_HANT, &ZH_HANT, &ZH_HANT, &ZH_HANT, &ZH_HANT, &ZH_HANT, &ZH, &ZU];
                static KEYS: [&str; 538usize] = ["af", "af-NA", "am", "ar", "ar-AE", "ar-AE-u-nu-arab", "ar-BH", "ar-BH-u-nu-latn", "ar-DJ", "ar-DJ-u-nu-latn", "ar-DZ", "ar-DZ-u-nu-arab", "ar-EG", "ar-EG-u-nu-latn", "ar-EH", "ar-EH-u-nu-arab", "ar-ER", "ar-ER-u-nu-latn", "ar-IL", "ar-IL-u-nu-latn", "ar-IQ", "ar-IQ-u-nu-latn", "ar-JO", "ar-JO-u-nu-latn", "ar-KM", "ar-KM-u-nu-latn", "ar-KW", "ar-KW-u-nu-latn", "ar-LB", "ar-LB-u-nu-latn", "ar-LY", "ar-LY-u-nu-arab", "ar-MA", "ar-MA-u-nu-arab", "ar-MR", "ar-MR-u-nu-latn", "ar-OM", "ar-OM-u-nu-latn", "ar-PS", "ar-PS-u-nu-latn", "ar-QA", "ar-QA-u-nu-latn", "ar-SA", "ar-SA-u-nu-latn", "ar-SD", "ar-SD-u-nu-latn", "ar-SO", "ar-SO-u-nu-latn", "ar-SS", "ar-SS-u-nu-latn", "ar-SY", "ar-SY-u-nu-latn", "ar-TD", "ar-TD-u-nu-latn", "ar-TN", "ar-TN-u-nu-arab", "ar-YE", "ar-YE-u-nu-latn", "ar-u-nu-latn", "as", "as-u-nu-latn", "ast", "az", "az-Latn", "be", "be-tarask", "bg", "bgc", "bgc-u-nu-latn", "bho", "bho-u-nu-latn", "bn", "bn-IN", "bn-IN-u-nu-latn", "bn-u-nu-latn", "br", "brx", "brx-u-nu-deva", "bs", "bs-Cyrl", "bs-Latn", "ca", "ca-AD", "ca-ES-valencia", "ca-FR", "ca-IT", "ceb", "chr", "cs", "cv", "cy", "da", "da-GL", "de", "de-AT", "de-BE", "de-CH", "de-IT", "de-LI", "de-LU", "doi", "doi-u-nu-deva", "dsb", "el", "el-CY", "el-polyton", "en", "en-001", "en-150", "en-AE", "en-AG", "en-AI", "en-AS", "en-AT", "en-AU", "en-BB", "en-BE", "en-BI", "en-BM", "en-BS", "en-BW", "en-BZ", "en-CA", "en-CC", "en-CH", "en-CK", "en-CM", "en-CX", "en-CY", "en-DE", "en-DG", "en-DK", "en-DM", "en-ER", "en-FI", "en-FJ", "en-FK", "en-FM", "en-GB", "en-GD", "en-GG", "en-GH", "en-GI", "en-GM", "en-GU", "en-GY", "en-HK", "en-IE", "en-IL", "en-IM", "en-IN", "en-IO", "en-JE", "en-JM", "en-KE", "en-KI", "en-KN", "en-KY", "en-LC", "en-LR", "en-LS", "en-MG", "en-MH", "en-MO", "en-MP", "en-MS", "en-MT", "en-MU", "en-MV", "en-MW", "en-MY", "en-NA", "en-NF", "en-NG", "en-NL", "en-NR", "en-NU", "en-NZ", "en-PG", "en-PH", "en-PK", "en-PN", "en-PR", "en-PW", "en-RW", "en-SB", "en-SC", "en-SD", "en-SE", "en-SG", "en-SH", "en-SI", "en-SL", "en-SS", "en-SX", "en-SZ", "en-TC", "en-TK", "en-TO", "en-TT", "en-TV", "en-TZ", "en-UG", "en-UM", "en-VC", "en-VG", "en-VI", "en-VU", "en-WS", "en-ZA", "en-ZM", "en-ZW", "es", "es-419", "es-AR", "es-BO", "es-BR", "es-BZ", "es-CL", "es-CO", "es-CR", "es-CU", "es-DO", "es-EA", "es-EC", "es-GQ", "es-GT", "es-HN", "es-IC", "es-MX", "es-NI", "es-PA", "es-PE", "es-PH", "es-PR", "es-PY", "es-SV", "es-US", "es-UY", "es-VE", "et", "eu", "fa", "fa-AF", "fa-AF-u-nu-latn", "fa-u-nu-latn", "ff-Adlm", "ff-Adlm-BF", "ff-Adlm-BF-u-nu-latn", "ff-Adlm-CM", "ff-Adlm-CM-u-nu-latn", "ff-Adlm-GH", "ff-Adlm-GH-u-nu-latn", "ff-Adlm-GM", "ff-Adlm-GM-u-nu-latn", "ff-Adlm-GW", "ff-Adlm-GW-u-nu-latn", "ff-Adlm-LR", "ff-Adlm-LR-u-nu-latn", "ff-Adlm-MR", "ff-Adlm-MR-u-nu-latn", "ff-Adlm-NE", "ff-Adlm-NE-u-nu-latn", "ff-Adlm-NG", "ff-Adlm-NG-u-nu-latn", "ff-Adlm-SL", "ff-Adlm-SL-u-nu-latn", "ff-Adlm-SN", "ff-Adlm-SN-u-nu-latn", "ff-Adlm-u-nu-latn", "fi", "fil", "fo", "fo-DK", "fr", "fr-BE", "fr-BF", "fr-BI", "fr-BJ", "fr-BL", "fr-CA", "fr-CD", "fr-CF", "fr-CG", "fr-CH", "fr-CI", "fr-CM", "fr-DJ", "fr-DZ", "fr-GA", "fr-GF", "fr-GN", "fr-GP", "fr-GQ", "fr-HT", "fr-KM", "fr-LU", "fr-MA", "fr-MC", "fr-MF", "fr-MG", "fr-ML", "fr-MQ", "fr-MR", "fr-MU", "fr-NC", "fr-NE", "fr-PF", "fr-PM", "fr-RE", "fr-RW", "fr-SC", "fr-SN", "fr-SY", "fr-TD", "fr-TG", "fr-TN", "fr-VU", "fr-WF", "fr-YT", "ga", "ga-GB", "gd", "gl", "gu", "gu-u-nu-gujr", "ha", "ha-GH", "ha-NE", "he", "hi", "hi-Latn", "hi-u-nu-deva", "hr", "hr-BA", "hsb", "hu", "hy", "ia", "id", "ig", "is", "it", "it-CH", "it-SM", "it-VA", "ja", "jv", "jv-u-nu-java", "ka", "kea", "kgp", "kk", "km", "km-u-nu-khmr", "kn", "kn-u-nu-knda", "ko", "ko-KP", "kok", "kok-u-nu-deva", "ks", "ks-Arab", "ks-Arab-u-nu-latn", "ks-Deva", "ks-u-nu-latn", "ky", "lo", "lo-u-nu-laoo", "lt", "lv", "mai", "mai-u-nu-deva", "mi", "mk", "ml", "ml-u-nu-mlym", "mn", "mni", "mni-Beng", "mni-Beng-u-nu-latn", "mni-u-nu-latn", "mr", "mr-u-nu-latn", "ms", "ms-BN", "ms-ID", "ms-SG", "my", "my-u-nu-latn", "nb", "nb-SJ", "ne", "ne-IN", "ne-IN-u-nu-latn", "ne-u-nu-latn", "nl", "nl-AW", "nl-BE", "nl-BQ", "nl-CW", "nl-SR", "nl-SX", "nn", "no", "or", "or-u-nu-orya", "pa", "pa-Guru", "pa-Guru-u-nu-guru", "pa-u-nu-guru", "pcm", "pl", "ps", "ps-PK", "ps-PK-u-nu-latn", "ps-u-nu-latn", "pt", "pt-AO", "pt-CH", "pt-CV", "pt-GQ", "pt-GW", "pt-LU", "pt-MO", "pt-MZ", "pt-PT", "pt-ST", "pt-TL", "qu", "qu-BO", "qu-EC", "raj", "raj-u-nu-latn", "rm", "ro", "ro-MD", "ru", "ru-BY", "ru-KG", "ru-KZ", "ru-MD", "ru-UA", "sa", "sa-u-nu-latn", "sat", "sat-Olck", "sat-Olck-u-nu-latn", "sat-u-nu-latn", "sc", "sd", "sd-Arab", "sd-Arab-u-nu-latn", "sd-Deva", "sd-u-nu-latn", "si", "sk", "sl", "so", "so-DJ", "so-ET", "so-KE", "sq", "sq-MK", "sq-XK", "sr", "sr-Cyrl", "sr-Cyrl-BA", "sr-Cyrl-XK", "sr-Latn", "sr-Latn-BA", "sr-Latn-ME", "sr-Latn-XK", "su", "su-Latn", "sv", "sv-AX", "sv-FI", "sw", "sw-CD", "sw-KE", "sw-UG", "ta", "ta-LK", "ta-LK-u-nu-tamldec", "ta-MY", "ta-MY-u-nu-tamldec", "ta-SG", "ta-SG-u-nu-tamldec", "ta-u-nu-tamldec", "te", "te-u-nu-telu", "tg", "th", "th-u-nu-thai", "ti", "ti-ER", "tk", "to", "tr", "tr-CY", "tt", "uk", "und", "ur", "ur-IN", "ur-IN-u-nu-latn", "ur-u-nu-arabext", "uz", "uz-Cyrl", "uz-Latn", "vi", "wo", "xh", "yo", "yo-BJ", "yrl", "yrl-CO", "yrl-VE", "yue", "yue-Hans", "yue-Hans-u-nu-hanidec", "yue-Hant", "yue-Hant-u-nu-hanidec", "yue-u-nu-hanidec", "zh", "zh-Hans", "zh-Hans-SG", "zh-Hans-SG-u-nu-hanidec", "zh-Hans-u-nu-hanidec", "zh-Hant", "zh-Hant-HK", "zh-Hant-HK-u-nu-hanidec", "zh-Hant-MO", "zh-Hant-MO-u-nu-hanidec", "zh-Hant-u-nu-hanidec", "zh-u-nu-hanidec", "zu"];
                let mut metadata = icu_provider::DataResponseMetadata::default();
                let payload = if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    payload
                } else {
                    let mut fallback_iterator = icu_locid_transform::fallback::LocaleFallbacker::new().fallback_for(<icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.into(), req.locale.clone());
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
