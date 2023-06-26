// @generated
/// Implement [`DataProvider<MetazoneSpecificNamesShortV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_time_zone_specific_short_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker>, icu_provider::DataError> {
                static PT: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"acreamazbras") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\x04\0\x07\0\x0B\0\x0E\0\x12\0ACSTACTAMSTAMTBRSTBRT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static AF: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"afceafeaafsoafwe") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x05\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"stststdtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x03\0\x06\0\n\0\x0E\0CATEATSASTWASTWAT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static ID: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlacubagreahaalinceineainwenewfpimi") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\n\0\0\0\x0C\0\0\0\x0E\0\0\0\x10\0\0\0\x12\0\0\0\x13\0\0\0\x14\0\0\0\x15\0\0\0\x17\0\0\0\x19\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtstdtstdtstdtstdtstdtststststdtstdtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x19\0\0\0\0\0\x04\0\x08\0\x0B\0\x0E\0\x11\0\x14\0\x17\0\x1A\0\x1D\0 \0#\0&\x000\0:\0>\0B\0F\0J\0N\0Q\0T\0W\0Z\0^\0AKDTAKSTCDTCSTEDTESTMDTMSTPDTPSTADTASTCDT (Kuba)CST (Kuba)EGDTEGSTHADTHASTWITAWITWIBNDTNSTPMDTPMST") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static CS: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlaeuce") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\n\0\0\0\x0C\0\0\0\x0E\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtstdtstdtstdtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0E\0\0\0\0\0\x04\0\x08\0\x0B\0\x0E\0\x11\0\x14\0\x17\0\x1A\0\x1D\0 \0#\0&\0+\0AKDTAKSTCDTCSTEDTESTMDTMSTPDTPSTADTASTSEL\xC4\x8CSE\xC4\x8C") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0HDTHSTUTC") })
                    },
                };
                static GD: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlaeuceeueaeuwehaalmgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\n\0\0\0\x0C\0\0\0\x0E\0\0\0\x10\0\0\0\x12\0\0\0\x14\0\0\0\x15\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtstdtstdtstdtstdtstdtstdtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x15\0\0\0\0\0\x04\0\x08\0\x0B\0\x0E\0\x11\0\x14\0\x17\0\x1A\0\x1D\0 \0#\0&\0*\0-\x001\x004\08\0;\0?\0C\0AKDTAKSTCDTCSTEDTESTMDTMSTPDTPSTADTASTCESTCETEESTEETWESTWETHADTHASTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"gblon\0\0\0iedub\0\0\0ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x04\0\0\0\x05\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtdtdtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x03\0\x11\0\x14\0\x17\0TSBTS\xC3\x88 (\xC3\x88irinn)HDTHSTUTC") })
                    },
                };
                static EU: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlaeuceeueaeuwehaalmgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\n\0\0\0\x0C\0\0\0\x0E\0\0\0\x10\0\0\0\x12\0\0\0\x14\0\0\0\x15\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtstdtstdtstdtstdtstdtstdtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x15\0\0\0\0\0\x04\0\x08\0\x0B\0\x0E\0\x11\0\x14\0\x17\0\x1A\0\x1D\0 \0#\0&\0*\0-\x001\x004\08\0;\0?\0C\0AKDTAKSTCDTCSTEDTESTMDTMSTPDTPSTADTASTCESTCETEESTEETWESTWETHADTHASTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0HDTHSTUTC") })
                    },
                };
                static NN: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlaeuceeueaeuwehaalmgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\n\0\0\0\x0C\0\0\0\x0E\0\0\0\x10\0\0\0\x12\0\0\0\x14\0\0\0\x15\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtstdtstdtstdtstdtstdtstdtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x15\0\0\0\0\0\x04\0\x08\0\x0B\0\x0E\0\x11\0\x14\0\x17\0\x1A\0\x1D\0 \0#\0&\0*\0-\x001\x004\08\0;\0?\0C\0AKDTAKSTCDTCSTEDTESTMDTMSTPDTPSTADTASTCESTCETEESTEETWESTWETHADTHASTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static VI: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlahaal") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\n\0\0\0\x0C\0\0\0\x0E\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtstdtstdtstdtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0E\0\0\0\0\0\x04\0\x08\0\x0B\0\x0E\0\x11\0\x14\0\x17\0\x1A\0\x1D\0 \0#\0&\0*\0AKDTAKSTCDTCSTEDTESTMDTMSTPDTPSTADTASTHADTHAST") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0HDTHSTUTC") })
                    },
                };
                static EN: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlahaalmgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\n\0\0\0\x0C\0\0\0\x0E\0\0\0\x0F\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtstdtstdtstdtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0F\0\0\0\0\0\x04\0\x08\0\x0B\0\x0E\0\x11\0\x14\0\x17\0\x1A\0\x1D\0 \0#\0&\0*\0.\0AKDTAKSTCDTCSTEDTESTMDTMSTPDTPSTADTASTHADTHASTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0HDTHSTUTC") })
                    },
                };
                static ZH_HANT: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlahaalmgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\n\0\0\0\x0C\0\0\0\x0E\0\0\0\x0F\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtstdtstdtstdtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0F\0\0\0\0\0\x04\0\x08\0\x0B\0\x0E\0\x11\0\x14\0\x17\0\x1A\0\x1D\0 \0#\0&\0*\0.\0AKDTAKSTCDTCSTEDTESTMDTMSTPDTPSTADTASTHADTHASTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static GA: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ampaeuceeueaeuwemgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\t\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\t\0\0\0\0\0\x04\0\x08\0\x0C\0\x0F\0\x13\0\x16\0\x1A\0\x1D\0ASACACACCESTCETEESTEETWESTWETMAG") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"gblon\0\0\0iedub\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtdtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x07\0ASBAC\xC3\x89UTC") })
                    },
                };
                static SK: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"euce") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x05\0SEL\xC4\x8CSE\xC4\x8C") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0HDTHSTUTC") })
                    },
                };
                static DA: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"euceeueaeuwe") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\x04\0\x07\0\x0B\0\x0E\0\x12\0CESTCETEESTEETWESTWET") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static DE: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"euceeueaeuwe") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\x04\0\x07\0\x0B\0\x0E\0\x12\0MESZMEZOESZOEZWESZWEZ") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static SV: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"euceeueaeuwemgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x07\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x04\0\x07\0\x0B\0\x0E\0\x12\0\x15\0CESTCETEESTEETWESTWETGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x11\0\"\0HonolulusommartidHonolulunormaltidUTC") })
                    },
                };
                static BS: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"euceeueaeuwemgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x07\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x04\0\x07\0\x0B\0\x0E\0\x12\0\x15\0CESTCETEESTEETWESTWETGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static KK: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"euea") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0EESTEET") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static AR: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"gulf") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0GST") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static HI_LATN: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"gulfindimgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ststst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0GSTISTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static SO: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"haal") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0HADTHAST") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0Waqtiga UTC") })
                    },
                };
                static HI: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"indi") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0IST") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x15\0HST\xE0\xA4\x8F\xE0\xA4\x9A\xE0\xA4\x8F\xE0\xA4\xB8\xE0\xA4\x9F\xE0\xA5\x80UTC") })
                    },
                };
                static BN: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"indi") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0IST") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static AS: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"indi") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xA6\xAD\xE0\xA6\xBE. \xE0\xA6\xB8.") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static OR: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"indimgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"stst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0ISTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static JA: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"japa") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0JDTJST") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static MS: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"malamgmtsing") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ststst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0MYTGMTSGT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static AM: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(zerovec::ZeroVec::new(), zerovec::ZeroVec::new(), zerovec::ZeroVec::new(), zerovec::VarZeroVec::new())
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static SD: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(zerovec::ZeroVec::new(), zerovec::ZeroVec::new(), zerovec::ZeroVec::new(), zerovec::VarZeroVec::new())
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD9\x85 \xD8\xB9 \xD9\x88") })
                    },
                };
                static VALUES: [&<icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable; 94usize] = [&AF, &AM, &AR, &AS, &AM, &AM, &AM, &BN, &BS, &BS, &CS, &BS, &DA, &DE, &DA, &EN, &BS, &AM, &EU, &AM, &AM, &AM, &AM, &GA, &GD, &BS, &BN, &AM, &AM, &HI, &HI_LATN, &BS, &BS, &AM, &ID, &AM, &AM, &DA, &JA, &AM, &AM, &KK, &AM, &BN, &AM, &BN, &AM, &AM, &AM, &DA, &DA, &BN, &AM, &BN, &MS, &AM, &AM, &EU, &NN, &NN, &OR, &BN, &AM, &DA, &AM, &PT, &BS, &AM, &SD, &AM, &SK, &AM, &SO, &AM, &BS, &BS, &SV, &AM, &BN, &BN, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &VI, &AM, &AM, &AM, &AM, &ZH_HANT, &AM];
                static KEYS: [&str; 94usize] = ["af", "am", "ar", "as", "az", "be", "bg", "bn", "bs", "ca", "cs", "cy", "da", "de", "el", "en", "es", "et", "eu", "fa", "fi", "fil", "fr", "ga", "gd", "gl", "gu", "ha", "he", "hi", "hi-Latn", "hr", "hu", "hy", "id", "ig", "is", "it", "ja", "jv", "ka", "kk", "km", "kn", "ko", "kok", "ky", "lo", "lt", "lv", "mk", "ml", "mn", "mr", "ms", "my", "ne", "nl", "nn", "no", "or", "pa", "pcm", "pl", "ps", "pt", "ro", "ru", "sd", "si", "sk", "sl", "so", "sq", "sr", "sr-Latn", "sv", "sw", "ta", "te", "th", "tk", "tr", "uk", "und", "ur", "uz", "vi", "yo", "yue", "yue-Hans", "zh", "zh-Hant", "zu"];
                let mut metadata = icu_provider::DataResponseMetadata::default();
                let payload = if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    payload
                } else {
                    let mut fallback_iterator = icu_locid_transform::fallback::LocaleFallbacker::new().fallback_for(<icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::KeyedDataMarker>::KEY.into(), req.locale.clone());
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
