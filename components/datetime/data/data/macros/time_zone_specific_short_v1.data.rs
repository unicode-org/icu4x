// @generated
/// Implement `DataProvider<MetazoneSpecificNamesShortV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_time_zone_specific_short_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.65"]
        impl icu_provider::DataProvider<icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker>, icu_provider::DataError> {
                static KGP: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"acreamazbras") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\x04\0\x07\0\x0B\0\x0E\0\x12\0ACSTACTAMSTAMTBRSTBRT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static AF: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"afceafeaafsoafwe") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x05\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"stststdtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x03\0\x06\0\n\0\x0E\0CATEATSASTWASTWAT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static EN_BW: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"afceafeaafsoafwemgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x05\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"stststdtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\x03\0\x06\0\n\0\x0E\0\x11\0CATEATSASTWASTWATGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static EN_CA: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"afghalasamceameaammoampaatlaaucwaueaauwebangbhutbrasbrunchatchricococoloeasteatiecuafalkgalagulfguyahaalinceindiindoineainweiranmalamaldmgmtnepanewfpakiparapimiurugvene") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x03\0\0\0\x05\0\0\0\x07\0\0\0\t\0\0\0\x0B\0\0\0\r\0\0\0\x0F\0\0\0\x11\0\0\0\x13\0\0\0\x14\0\0\0\x15\0\0\0\x16\0\0\0\x17\0\0\0\x19\0\0\0\x1A\0\0\0\x1B\0\0\0\x1C\0\0\0\x1E\0\0\0\x1F\0\0\0 \0\0\0!\0\0\0\"\0\0\0#\0\0\0$\0\0\0&\0\0\0'\0\0\0(\0\0\0)\0\0\0*\0\0\0+\0\0\0-\0\0\0.\0\0\0/\0\0\x000\0\0\x001\0\0\x003\0\0\x004\0\0\x005\0\0\x007\0\0\09\0\0\0:\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"stdtstdtstdtstdtstdtstdtstdtstdtstdtstststdtstdtstststdtdtstststdtstststdtststststststdtstststststdtststdtdtstdtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b":\0\0\0\0\0\x03\0\x07\0\x0B\0\x0E\0\x11\0\x14\0\x17\0\x1A\0\x1D\0 \0#\0&\0)\0.\x003\x007\0;\0?\0C\0F\0I\0M\0P\0U\0Z\0]\0`\0d\0i\0m\0p\0s\0w\0{\0~\0\x81\0\x85\0\x89\0\x8D\0\x90\0\x93\0\x96\0\x99\0\x9D\0\xA1\0\xA4\0\xA7\0\xAA\0\xAD\0\xB0\0\xB3\0\xB6\0\xBA\0\xBE\0\xC2\0\xC6\0\xC9\0AFTAKDTAKSTCDTCSTEDTESTMDTMSTPDTPSTADTASTACWDTACWSTAEDTAESTAWDTAWSTBSTBTTBRSTBNTCHADTCHASTCXTCCTCOSTEASSTEASTTLTECTFKSTGALTN/AGYTHADTHASTWITAISTICTWITWIBIRDTIRSTMYTMVTGMTNPTNDTNSTPKTPYSTPMDTPMSTUYSTUYTVET") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0HDTHSTUTC") })
                    },
                };
                static EN_GU: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlachamhaalmgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\n\0\0\0\x0C\0\0\0\r\0\0\0\x0F\0\0\0\x10\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtstdtstdtststdtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x10\0\0\0\0\0\x04\0\x08\0\x0B\0\x0E\0\x11\0\x14\0\x17\0\x1A\0\x1D\0 \0#\0&\0*\0.\x002\0AKDTAKSTCDTCSTEDTESTMDTMSTPDTPSTADTASTChSTHADTHASTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0HDTHSTUTC") })
                    },
                };
                static ID: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlacubagreahaalinceineainwenewfpimi") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\n\0\0\0\x0C\0\0\0\x0E\0\0\0\x10\0\0\0\x12\0\0\0\x13\0\0\0\x14\0\0\0\x15\0\0\0\x17\0\0\0\x19\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtstdtstdtstdtstdtstdtststststdtstdtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x19\0\0\0\0\0\x04\0\x08\0\x0B\0\x0E\0\x11\0\x14\0\x17\0\x1A\0\x1D\0 \0#\0&\x000\0:\0>\0B\0F\0J\0N\0Q\0T\0W\0Z\0^\0AKDTAKSTCDTCSTEDTESTMDTMSTPDTPSTADTASTCDT (Kuba)CST (Kuba)EGDTEGSTHADTHASTWITAWITWIBNDTNSTPMDTPMST") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static CS: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlaeuce") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\n\0\0\0\x0C\0\0\0\x0E\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtstdtstdtstdtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0E\0\0\0\0\0\x04\0\x08\0\x0B\0\x0E\0\x11\0\x14\0\x17\0\x1A\0\x1D\0 \0#\0&\0+\0AKDTAKSTCDTCSTEDTESTMDTMSTPDTPSTADTASTSEL\xC4\x8CSE\xC4\x8C") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0HDTHSTUTC") })
                    },
                };
                static GD: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlaeuceeueaeuwehaalmgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\n\0\0\0\x0C\0\0\0\x0E\0\0\0\x10\0\0\0\x12\0\0\0\x14\0\0\0\x15\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtstdtstdtstdtstdtstdtstdtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x15\0\0\0\0\0\x04\0\x08\0\x0B\0\x0E\0\x11\0\x14\0\x17\0\x1A\0\x1D\0 \0#\0&\0*\0-\x001\x004\08\0;\0?\0C\0AKDTAKSTCDTCSTEDTESTMDTMSTPDTPSTADTASTCESTCETEESTEETWESTWETHADTHASTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"gblon\0\0\0iedub\0\0\0ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x04\0\0\0\x05\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtdtdtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x03\0\x11\0\x14\0\x17\0TSBTS\xC3\x88 (\xC3\x88irinn)HDTHSTUTC") })
                    },
                };
                static EU: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlaeuceeueaeuwehaalmgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\n\0\0\0\x0C\0\0\0\x0E\0\0\0\x10\0\0\0\x12\0\0\0\x14\0\0\0\x15\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtstdtstdtstdtstdtstdtstdtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x15\0\0\0\0\0\x04\0\x08\0\x0B\0\x0E\0\x11\0\x14\0\x17\0\x1A\0\x1D\0 \0#\0&\0*\0-\x001\x004\08\0;\0?\0C\0AKDTAKSTCDTCSTEDTESTMDTMSTPDTPSTADTASTCESTCETEESTEETWESTWETHADTHASTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0HDTHSTUTC") })
                    },
                };
                static NO: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlaeuceeueaeuwehaalmgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\n\0\0\0\x0C\0\0\0\x0E\0\0\0\x10\0\0\0\x12\0\0\0\x14\0\0\0\x15\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtstdtstdtstdtstdtstdtstdtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x15\0\0\0\0\0\x04\0\x08\0\x0B\0\x0E\0\x11\0\x14\0\x17\0\x1A\0\x1D\0 \0#\0&\0*\0-\x001\x004\08\0;\0?\0C\0AKDTAKSTCDTCSTEDTESTMDTMSTPDTPSTADTASTCESTCETEESTEETWESTWETHADTHASTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static NL_SR: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlaeuceeueaeuwehaalmgmtsuri") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\n\0\0\0\x0C\0\0\0\x0E\0\0\0\x10\0\0\0\x12\0\0\0\x14\0\0\0\x15\0\0\0\x16\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtstdtstdtstdtstdtstdtstdtststst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x16\0\0\0\0\0\x04\0\x08\0\x0B\0\x0E\0\x11\0\x14\0\x17\0\x1A\0\x1D\0 \0#\0&\0*\0-\x001\x004\08\0;\0?\0C\0F\0AKDTAKSTCDTCSTEDTESTMDTMSTPDTPSTADTASTCESTCETEESTEETWESTWETHADTHASTGMTSRT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0HDTHSTUTC") })
                    },
                };
                static CHR: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlahaal") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\n\0\0\0\x0C\0\0\0\x0E\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtstdtstdtstdtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0E\0\0\0\0\0\x04\0\x08\0\x0B\0\x0E\0\x11\0\x14\0\x17\0\x1A\0\x1D\0 \0#\0&\0*\0AKDTAKSTCDTCSTEDTESTMDTMSTPDTPSTADTASTHADTHAST") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0HDTHSTUTC") })
                    },
                };
                static AST: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlahaalmgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\n\0\0\0\x0C\0\0\0\x0E\0\0\0\x0F\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtstdtstdtstdtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0F\0\0\0\0\0\x04\0\x08\0\x0B\0\x0E\0\x11\0\x14\0\x17\0\x1A\0\x1D\0 \0#\0&\0*\0.\0AKDTAKSTCDTCSTEDTESTMDTMSTPDTPSTADTASTHADTHASTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0HDTHSTUTC") })
                    },
                };
                static ZH_HANT: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlahaalmgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\n\0\0\0\x0C\0\0\0\x0E\0\0\0\x0F\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtstdtstdtstdtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0F\0\0\0\0\0\x04\0\x08\0\x0B\0\x0E\0\x11\0\x14\0\x17\0\x1A\0\x1D\0 \0#\0&\0*\0.\0AKDTAKSTCDTCSTEDTESTMDTMSTPDTPSTADTASTHADTHASTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static SC: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlamgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\n\0\0\0\x0C\0\0\0\r\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtstdtstdtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x04\0\x08\0\x0B\0\x0E\0\x11\0\x14\0\x18\0\x1C\0\x1F\0\"\0%\0(\0OLAKOIAKOLTOITOLOOIOOLMPOIMPOLPOIPOLAOIAOMG") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0OLHOIHTCU") })
                    },
                };
                static FR_CA: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"amceameaammoampanewf") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\n\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtstdtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\n\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0HACHNCHAEHNEHARHNRHAPHNPHATHNT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static GA: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ampaeuceeueaeuwemgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\t\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\t\0\0\0\0\0\x04\0\x08\0\x0C\0\x0F\0\x13\0\x16\0\x1A\0\x1D\0ASACACACCESTCETEESTEETWESTWETMAG") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"gblon\0\0\0iedub\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtdtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x07\0ASBAC\xC3\x89UTC") })
                    },
                };
                static ES_AR: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"argearwe") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x04\0\x07\0\x0C\0ARSTARTWARSTWART") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static EN_NZ: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"auceaucwaueaauwechatlohomgmtneze") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\n\0\0\0\x0C\0\0\0\r\0\0\0\x0F\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtstdtstdtststdtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0F\0\0\0\0\0\x04\0\x08\0\r\0\x12\0\x16\0\x1A\0\x1E\0\"\0'\0,\x000\x004\x007\0;\0ACDTACSTACWDTACWSTAEDTAESTAWDTAWSTCHADTCHASTLHDTLHSTGMTNZDTNZST") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static EN_AU: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"auceaucwaueaauwegulflohomgmtneze") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\t\0\0\0\x0B\0\0\0\x0C\0\0\0\x0E\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtststdtststdtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0E\0\0\0\0\0\x04\0\x08\0\r\0\x12\0\x16\0\x1A\0\x1E\0\"\0)\0-\x001\x004\08\0ACDTACSTACWDTACWSTAEDTAESTAWDTAWSTGulf STLHDTLHSTGMTNZDTNZST") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static PT_PT: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"azoreuceeueaeuwe") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x08\0\0\0\0\0\x05\0\t\0\r\0\x10\0\x14\0\x17\0\x1B\0AZOSTAZOTCESTCETEESTEETWESTWET") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static ES_BO: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"boli") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0BOT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static ES_CL: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"chil") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0CLSTCLT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static ES_CO: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"colo") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0COSTCOT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static ES_EC: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ecua") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ECT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static SK: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"euce") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x05\0SEL\xC4\x8CSE\xC4\x8C") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0HDTHSTUTC") })
                    },
                };
                static DA: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"euceeueaeuwe") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\x04\0\x07\0\x0B\0\x0E\0\x12\0CESTCETEESTEETWESTWET") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static DE: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"euceeueaeuwe") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\x04\0\x07\0\x0B\0\x0E\0\x12\0MESZMEZOESZOEZWESZWEZ") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static EN_GB: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"euceeueaeuwegulfmgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x07\0\0\0\x08\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtststst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x08\0\0\0\0\0\x04\0\x07\0\x0B\0\x0E\0\x12\0\x15\0\x18\0CESTCETEESTEETWESTWETGTSGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"gblon\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0BSTUTC") })
                    },
                };
                static SV: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"euceeueaeuwemgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x07\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x04\0\x07\0\x0B\0\x0E\0\x12\0\x15\0CESTCETEESTEETWESTWETGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x11\0\"\0HonolulusommartidHonolulunormaltidUTC") })
                    },
                };
                static BS: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"euceeueaeuwemgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x07\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x04\0\x07\0\x0B\0\x0E\0\x12\0\x15\0CESTCETEESTEETWESTWETGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static KK: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"euea") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0EESTEET") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static FR_GF: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"frgu") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0GFT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static AR: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"gulf") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0GST") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static EN_IN: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"gulfindimgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ststst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0GSTISTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static EN_AE: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"gulfmgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"stst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0GSTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static EN_GY: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"guyamgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"stst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0GYTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static SO: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"haal") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0HADTHAST") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0Waqtiga UTC") })
                    },
                };
                static EN_MO: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"hokomacamgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x05\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x04\0\x07\0\n\0\r\0HKSTHKTMDTMSTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static EN_HK: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"hokomgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x07\0HKSTHKTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static HI: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"indi") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0IST") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x15\0HST\xE0\xA4\x8F\xE0\xA4\x9A\xE0\xA4\x8F\xE0\xA4\xB8\xE0\xA4\x9F\xE0\xA5\x80UTC") })
                    },
                };
                static BN: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"indi") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0IST") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static AS: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"indi") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xA6\xAD\xE0\xA6\xBE. \xE0\xA6\xB8.") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static OR: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"indimgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"stst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0ISTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static JA: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"japa") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0JDTJST") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static EN_MY: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"malamgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"stst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0MYTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static EN_SG: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"malamgmtsing") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ststst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0MYTGMTSGT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static TA_MY: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"malasing") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"stst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0MYTSGT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static EN_IE: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"mgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0GMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"iedub\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0ISTUTC") })
                    },
                };
                static EN_001: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"mgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0GMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static ES_PE: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"peru") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0PESTPET") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static ES_UY: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"urug") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0UYSTUYT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static ES_VE: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"vene") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0VET") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static FF_ADLM: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(zerovec::ZeroVec::new(), zerovec::ZeroVec::new(), zerovec::ZeroVec::new(), zerovec::VarZeroVec::new())
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x18\0\xF0\x9E\xA4\x91\xF0\x9E\xA4\x95\xF0\x9E\xA4\x96\xF0\x9E\xA4\x91\xF0\x9E\xA4\x96\xF0\x9E\xA4\x96\xF0\x9E\xA4\x91\xF0\x9E\xA4\x96\xF0\x9E\xA4\x8A") })
                    },
                };
                static UND: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(zerovec::ZeroVec::new(), zerovec::ZeroVec::new(), zerovec::ZeroVec::new(), zerovec::VarZeroVec::new())
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static SD: <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(zerovec::ZeroVec::new(), zerovec::ZeroVec::new(), zerovec::ZeroVec::new(), zerovec::VarZeroVec::new())
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD9\x85 \xD8\xB9 \xD9\x88") })
                    },
                };
                static VALUES: [&<icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable; 119usize] = [&AF, &AR, &AS, &AST, &BN, &BN, &BS, &BS, &CHR, &CS, &BS, &DA, &DE, &DE, &DA, &AST, &EN_001, &BS, &EN_AE, &EN_AU, &EN_BW, &EN_CA, &EN_BW, &EN_BW, &EN_GB, &EN_BW, &EN_BW, &EN_GU, &EN_GY, &EN_HK, &EN_IE, &EN_IN, &EN_BW, &EN_BW, &EN_BW, &EN_BW, &EN_001, &EN_MO, &EN_001, &EN_BW, &EN_BW, &EN_MY, &EN_BW, &EN_BW, &EN_NZ, &EN_BW, &EN_BW, &EN_SG, &EN_BW, &EN_BW, &EN_BW, &EN_BW, &EN_BW, &EN_BW, &EN_BW, &EN_BW, &BS, &ES_AR, &ES_BO, &ES_CL, &ES_CO, &ES_EC, &ES_PE, &CHR, &ES_UY, &ES_VE, &EU, &FF_ADLM, &FR_CA, &FR_GF, &GA, &GD, &BS, &BN, &HI, &BS, &DE, &BS, &ID, &DA, &JA, &KGP, &KK, &BN, &BN, &DA, &DA, &BN, &BN, &EN_SG, &BN, &EU, &NL_SR, &NO, &OR, &BN, &DA, &KGP, &PT_PT, &ES_PE, &ES_BO, &ES_EC, &BS, &SC, &SD, &SK, &SO, &BS, &BS, &SV, &BN, &TA_MY, &TA_MY, &BN, &UND, &CHR, &KGP, &ZH_HANT, &TA_MY];
                static KEYS: [&str; 119usize] = ["af", "ar", "as", "ast", "bn", "brx", "bs", "ca", "chr", "cs", "cy", "da", "de", "dsb", "el", "en", "en-001", "en-150", "en-AE", "en-AU", "en-BW", "en-CA", "en-CM", "en-ER", "en-GB", "en-GH", "en-GM", "en-GU", "en-GY", "en-HK", "en-IE", "en-IN", "en-KE", "en-LR", "en-LS", "en-MG", "en-MH", "en-MO", "en-MP", "en-MU", "en-MW", "en-MY", "en-NA", "en-NG", "en-NZ", "en-RW", "en-SD", "en-SG", "en-SL", "en-SS", "en-SZ", "en-TZ", "en-UG", "en-ZA", "en-ZM", "en-ZW", "es", "es-AR", "es-BO", "es-CL", "es-CO", "es-EC", "es-PE", "es-US", "es-UY", "es-VE", "eu", "ff-Adlm", "fr-CA", "fr-GF", "ga", "gd", "gl", "gu", "hi", "hr", "hsb", "hu", "id", "it", "ja", "kgp", "kk", "kn", "kok", "lv", "mk", "ml", "mr", "ms", "ne-IN", "nl", "nl-SR", "no", "or", "pa", "pl", "pt", "pt-PT", "qu", "qu-BO", "qu-EC", "ro", "sc", "sd", "sk", "so", "sr", "sr-Latn", "sv", "ta", "ta-MY", "ta-SG", "te", "und", "vi", "yrl", "zh-Hant", "zh-SG"];
                let mut metadata = icu_provider::DataResponseMetadata::default();
                let payload = if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    payload
                } else {
                    const FALLBACKER: icu::locid_transform::fallback::LocaleFallbackerWithConfig<'static> = icu::locid_transform::fallback::LocaleFallbacker::new().for_config(<icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::KeyedDataMarker>::KEY.fallback_config());
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
