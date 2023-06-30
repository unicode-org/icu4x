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
                static KGP: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
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
                static EN_BW: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"afceafeaafsoafwemgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x05\0\0\0\x06\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"stststdtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\x03\0\x06\0\n\0\x0E\0\x11\0CATEATSASTWASTWATGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static EN_CA: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"afghalasamceameaammoampaatlaaucwaueaauwebangbhutbrasbrunchatchricococoloeasteatiecuafalkgalagulfguyahaalinceindiindoineainweiranmalamaldmgmtnepanewfpakiparapimiurugvene") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x03\0\0\0\x05\0\0\0\x07\0\0\0\t\0\0\0\x0B\0\0\0\r\0\0\0\x0F\0\0\0\x11\0\0\0\x13\0\0\0\x14\0\0\0\x15\0\0\0\x16\0\0\0\x17\0\0\0\x19\0\0\0\x1A\0\0\0\x1B\0\0\0\x1C\0\0\0\x1E\0\0\0\x1F\0\0\0 \0\0\0!\0\0\0\"\0\0\0#\0\0\0$\0\0\0&\0\0\0'\0\0\0(\0\0\0)\0\0\0*\0\0\0+\0\0\0-\0\0\0.\0\0\0/\0\0\x000\0\0\x001\0\0\x003\0\0\x004\0\0\x005\0\0\x007\0\0\09\0\0\0:\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"stdtstdtstdtstdtstdtstdtstdtstdtstdtstststdtstdtstststdtdtstststdtstststdtststststststdtstststststdtststdtdtstdtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b":\0\0\0\0\0\x03\0\x07\0\x0B\0\x0E\0\x11\0\x14\0\x17\0\x1A\0\x1D\0 \0#\0&\0)\0.\x003\x007\0;\0?\0C\0F\0I\0M\0P\0U\0Z\0]\0`\0d\0i\0m\0p\0s\0w\0{\0~\0\x81\0\x85\0\x89\0\x8D\0\x90\0\x93\0\x96\0\x99\0\x9D\0\xA1\0\xA4\0\xA7\0\xAA\0\xAD\0\xB0\0\xB3\0\xB6\0\xBA\0\xBE\0\xC2\0\xC6\0\xC9\0AFTAKDTAKSTCDTCSTEDTESTMDTMSTPDTPSTADTASTACWDTACWSTAEDTAESTAWDTAWSTBSTBTTBRSTBNTCHADTCHASTCXTCCTCOSTEASSTEASTTLTECTFKSTGALTN/AGYTHADTHASTWITAISTICTWITWIBIRDTIRSTMYTMVTGMTNPTNDTNSTPKTPYSTPMDTPMSTUYSTUYTVET") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0HDTHSTUTC") })
                    },
                };
                static EN_GU: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlachamhaalmgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\n\0\0\0\x0C\0\0\0\r\0\0\0\x0F\0\0\0\x10\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtstdtstdtststdtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x10\0\0\0\0\0\x04\0\x08\0\x0B\0\x0E\0\x11\0\x14\0\x17\0\x1A\0\x1D\0 \0#\0&\0*\0.\x002\0AKDTAKSTCDTCSTEDTESTMDTMSTPDTPSTADTASTChSTHADTHASTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0HDTHSTUTC") })
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
                static NB: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlaeuceeueaeuwehaalmgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\n\0\0\0\x0C\0\0\0\x0E\0\0\0\x10\0\0\0\x12\0\0\0\x14\0\0\0\x15\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtstdtstdtstdtstdtstdtstdtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x15\0\0\0\0\0\x04\0\x08\0\x0B\0\x0E\0\x11\0\x14\0\x17\0\x1A\0\x1D\0 \0#\0&\0*\0-\x001\x004\08\0;\0?\0C\0AKDTAKSTCDTCSTEDTESTMDTMSTPDTPSTADTASTCESTCETEESTEETWESTWETHADTHASTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static NL_SR: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlaeuceeueaeuwehaalmgmtsuri") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\n\0\0\0\x0C\0\0\0\x0E\0\0\0\x10\0\0\0\x12\0\0\0\x14\0\0\0\x15\0\0\0\x16\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtstdtstdtstdtstdtstdtstdtststst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x16\0\0\0\0\0\x04\0\x08\0\x0B\0\x0E\0\x11\0\x14\0\x17\0\x1A\0\x1D\0 \0#\0&\0*\0-\x001\x004\08\0;\0?\0C\0F\0AKDTAKSTCDTCSTEDTESTMDTMSTPDTPSTADTASTCESTCETEESTEETWESTWETHADTHASTGMTSRT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0HDTHSTUTC") })
                    },
                };
                static CHR: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlahaal") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\n\0\0\0\x0C\0\0\0\x0E\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtstdtstdtstdtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0E\0\0\0\0\0\x04\0\x08\0\x0B\0\x0E\0\x11\0\x14\0\x17\0\x1A\0\x1D\0 \0#\0&\0*\0AKDTAKSTCDTCSTEDTESTMDTMSTPDTPSTADTASTHADTHAST") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0HDTHSTUTC") })
                    },
                };
                static AST: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
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
                static SC: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlamgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\n\0\0\0\x0C\0\0\0\r\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtstdtstdtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\r\0\0\0\0\0\x04\0\x08\0\x0B\0\x0E\0\x11\0\x14\0\x18\0\x1C\0\x1F\0\"\0%\0(\0OLAKOIAKOLTOITOLOOIOOLMPOIMPOLPOIPOLAOIAOMG") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0OLHOIHTCU") })
                    },
                };
                static FR_CA: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"amceameaammoampanewf") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\n\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtstdtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\n\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0\x0F\0\x12\0\x15\0\x18\0\x1B\0HACHNCHAEHNEHARHNRHAPHNPHATHNT") })
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
                static ES_AR: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"argearwe") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x04\0\x07\0\x0C\0ARSTARTWARSTWART") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static EN_NZ: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"auceaucwaueaauwechatlohomgmtneze") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\n\0\0\0\x0C\0\0\0\r\0\0\0\x0F\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtstdtstdtststdtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0F\0\0\0\0\0\x04\0\x08\0\r\0\x12\0\x16\0\x1A\0\x1E\0\"\0'\0,\x000\x004\x007\0;\0ACDTACSTACWDTACWSTAEDTAESTAWDTAWSTCHADTCHASTLHDTLHSTGMTNZDTNZST") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static EN_AU: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"auceaucwaueaauwegulflohomgmtneze") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\t\0\0\0\x0B\0\0\0\x0C\0\0\0\x0E\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtststdtststdtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0E\0\0\0\0\0\x04\0\x08\0\r\0\x12\0\x16\0\x1A\0\x1E\0\"\0)\0-\x001\x004\08\0ACDTACSTACWDTACWSTAEDTAESTAWDTAWSTGulf STLHDTLHSTGMTNZDTNZST") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static PT_CH: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"azoreuceeueaeuwe") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtstdtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x08\0\0\0\0\0\x05\0\t\0\r\0\x10\0\x14\0\x17\0\x1B\0AZOSTAZOTCESTCETEESTEETWESTWET") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static ES_BO: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"boli") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0BOT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static ES_CL: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"chil") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0CLSTCLT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static ES_CO: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"colo") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0COSTCOT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static ES_EC: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ecua") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ECT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
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
                static EN_GB: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"euceeueaeuwegulfmgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x07\0\0\0\x08\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstdtststst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x08\0\0\0\0\0\x04\0\x07\0\x0B\0\x0E\0\x12\0\x15\0\x18\0CESTCETEESTEETWESTWETGTSGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"gblon\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0BSTUTC") })
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
                static FR_GF: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"frgu") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0GFT") })
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
                static EN_IN: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"gulfindimgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ststst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0GSTISTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static EN_AE: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"gulfmgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"stst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0GSTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static EN_GY: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"guyamgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"stst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0GYTGMT") })
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
                static EN_MO: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"hokomacamgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x04\0\0\0\x05\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstdtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x04\0\x07\0\n\0\r\0HKSTHKTMDTMSTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static EN_HK: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"hokomgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x04\0\x07\0HKSTHKTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
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
                static EN_MY: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"malamgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"stst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0MYTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static EN_SG: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"malamgmtsing") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ststst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0MYTGMTSGT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static TA_MY: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"malasing") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"stst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0MYTSGT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static EN_IE: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"mgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0GMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"iedub\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0ISTUTC") })
                    },
                };
                static EN_001: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"mgmt") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0GMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static ES_PE: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"peru") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0PESTPET") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static ES_UY: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"urug") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0UYSTUYT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static ES_VE: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"vene") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0VET") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"st") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static FF_ADLM: <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(zerovec::ZeroVec::new(), zerovec::ZeroVec::new(), zerovec::ZeroVec::new(), zerovec::VarZeroVec::new())
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0\x03\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"dtstst") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x0C\0\x18\0\xF0\x9E\xA4\x91\xF0\x9E\xA4\x95\xF0\x9E\xA4\x96\xF0\x9E\xA4\x91\xF0\x9E\xA4\x96\xF0\x9E\xA4\x96\xF0\x9E\xA4\x91\xF0\x9E\xA4\x96\xF0\x9E\xA4\x8A") })
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
                static VALUES: [&<icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::Yokeable; 444usize] = [&AF, &AF, &AM, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AS, &AST, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &BN, &BN, &AM, &BN, &BS, &AM, &BS, &BS, &BS, &BS, &BS, &BS, &AM, &CHR, &CS, &AM, &BS, &DA, &DA, &DE, &DE, &DE, &DE, &DE, &DE, &DE, &AM, &DE, &DA, &DA, &DA, &AST, &EN_001, &BS, &EN_AE, &EN_001, &EN_001, &AST, &BS, &EN_AU, &EN_001, &BS, &AST, &EN_001, &EN_001, &EN_BW, &EN_001, &EN_CA, &EN_001, &BS, &EN_001, &EN_BW, &EN_001, &EN_001, &BS, &EN_001, &BS, &EN_001, &EN_BW, &BS, &EN_001, &EN_001, &EN_001, &EN_GB, &EN_001, &EN_001, &EN_BW, &EN_001, &EN_BW, &EN_GU, &EN_GY, &EN_HK, &EN_IE, &EN_001, &EN_001, &EN_IN, &EN_001, &EN_001, &EN_001, &EN_BW, &EN_001, &EN_001, &EN_001, &EN_001, &EN_BW, &EN_BW, &EN_BW, &EN_001, &EN_MO, &EN_001, &EN_001, &EN_001, &EN_BW, &EN_001, &EN_BW, &EN_MY, &EN_BW, &EN_001, &EN_BW, &BS, &EN_001, &EN_001, &EN_NZ, &EN_001, &AST, &EN_001, &EN_001, &AST, &EN_001, &EN_BW, &EN_001, &EN_001, &EN_BW, &BS, &EN_SG, &EN_001, &BS, &EN_BW, &EN_BW, &EN_001, &EN_BW, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_BW, &EN_BW, &AST, &EN_001, &EN_001, &AST, &EN_001, &EN_001, &EN_BW, &EN_BW, &EN_BW, &BS, &AM, &ES_AR, &ES_BO, &AM, &AM, &ES_CL, &ES_CO, &AM, &AM, &AM, &BS, &ES_EC, &BS, &AM, &AM, &BS, &AM, &AM, &AM, &ES_PE, &BS, &AM, &AM, &AM, &CHR, &ES_UY, &ES_VE, &AM, &EU, &AM, &AM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &FR_CA, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &FR_GF, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &GA, &GA, &GD, &BS, &BN, &AM, &AM, &AM, &AM, &HI, &EN_IN, &BS, &BS, &DE, &BS, &AM, &AM, &ID, &AM, &AM, &DA, &DA, &DA, &DA, &JA, &AM, &AM, &AM, &KGP, &KK, &AM, &BN, &AM, &AM, &BN, &AM, &AM, &AM, &AM, &AM, &AM, &DA, &AM, &AM, &DA, &BN, &AM, &AM, &AM, &BN, &EN_SG, &EN_SG, &EN_SG, &EN_SG, &AM, &NB, &NB, &AM, &BN, &EU, &EU, &EU, &EU, &EU, &NL_SR, &EU, &NB, &NB, &OR, &BN, &BN, &AM, &DA, &AM, &AM, &KGP, &AM, &PT_CH, &AM, &PT_CH, &AM, &PT_CH, &AM, &AM, &PT_CH, &AM, &AM, &ES_PE, &ES_BO, &ES_EC, &AM, &AM, &BS, &BS, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &SC, &SD, &SD, &AM, &AM, &SK, &AM, &SO, &SO, &SO, &SO, &AM, &AM, &AM, &BS, &BS, &BS, &BS, &BS, &BS, &BS, &BS, &AM, &AM, &SV, &SV, &SV, &AM, &AM, &AM, &AM, &BN, &BN, &TA_MY, &TA_MY, &BN, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &CHR, &AM, &AM, &AM, &AM, &KGP, &KGP, &KGP, &AM, &AM, &AM, &AM, &AM, &TA_MY, &ZH_HANT, &ZH_HANT, &ZH_HANT, &AM];
                static KEYS: [&str; 444usize] = ["af", "af-NA", "am", "ar", "ar-AE", "ar-BH", "ar-DJ", "ar-DZ", "ar-EG", "ar-EH", "ar-ER", "ar-IL", "ar-IQ", "ar-JO", "ar-KM", "ar-KW", "ar-LB", "ar-LY", "ar-MA", "ar-MR", "ar-OM", "ar-PS", "ar-QA", "ar-SA", "ar-SD", "ar-SO", "ar-SS", "ar-SY", "ar-TD", "ar-TN", "ar-YE", "as", "ast", "az", "az-Latn", "be", "be-tarask", "bg", "bgc", "bho", "bn", "bn-IN", "br", "brx", "bs", "bs-Cyrl", "bs-Latn", "ca", "ca-AD", "ca-ES-valencia", "ca-FR", "ca-IT", "ceb", "chr", "cs", "cv", "cy", "da", "da-GL", "de", "de-AT", "de-BE", "de-CH", "de-IT", "de-LI", "de-LU", "doi", "dsb", "el", "el-CY", "el-polyton", "en", "en-001", "en-150", "en-AE", "en-AG", "en-AI", "en-AS", "en-AT", "en-AU", "en-BB", "en-BE", "en-BI", "en-BM", "en-BS", "en-BW", "en-BZ", "en-CA", "en-CC", "en-CH", "en-CK", "en-CM", "en-CX", "en-CY", "en-DE", "en-DG", "en-DK", "en-DM", "en-ER", "en-FI", "en-FJ", "en-FK", "en-FM", "en-GB", "en-GD", "en-GG", "en-GH", "en-GI", "en-GM", "en-GU", "en-GY", "en-HK", "en-IE", "en-IL", "en-IM", "en-IN", "en-IO", "en-JE", "en-JM", "en-KE", "en-KI", "en-KN", "en-KY", "en-LC", "en-LR", "en-LS", "en-MG", "en-MH", "en-MO", "en-MP", "en-MS", "en-MT", "en-MU", "en-MV", "en-MW", "en-MY", "en-NA", "en-NF", "en-NG", "en-NL", "en-NR", "en-NU", "en-NZ", "en-PG", "en-PH", "en-PK", "en-PN", "en-PR", "en-PW", "en-RW", "en-SB", "en-SC", "en-SD", "en-SE", "en-SG", "en-SH", "en-SI", "en-SL", "en-SS", "en-SX", "en-SZ", "en-TC", "en-TK", "en-TO", "en-TT", "en-TV", "en-TZ", "en-UG", "en-UM", "en-VC", "en-VG", "en-VI", "en-VU", "en-WS", "en-ZA", "en-ZM", "en-ZW", "es", "es-419", "es-AR", "es-BO", "es-BR", "es-BZ", "es-CL", "es-CO", "es-CR", "es-CU", "es-DO", "es-EA", "es-EC", "es-GQ", "es-GT", "es-HN", "es-IC", "es-MX", "es-NI", "es-PA", "es-PE", "es-PH", "es-PR", "es-PY", "es-SV", "es-US", "es-UY", "es-VE", "et", "eu", "fa", "fa-AF", "ff-Adlm", "ff-Adlm-BF", "ff-Adlm-CM", "ff-Adlm-GH", "ff-Adlm-GM", "ff-Adlm-GW", "ff-Adlm-LR", "ff-Adlm-MR", "ff-Adlm-NE", "ff-Adlm-NG", "ff-Adlm-SL", "ff-Adlm-SN", "fi", "fil", "fo", "fo-DK", "fr", "fr-BE", "fr-BF", "fr-BI", "fr-BJ", "fr-BL", "fr-CA", "fr-CD", "fr-CF", "fr-CG", "fr-CH", "fr-CI", "fr-CM", "fr-DJ", "fr-DZ", "fr-GA", "fr-GF", "fr-GN", "fr-GP", "fr-GQ", "fr-HT", "fr-KM", "fr-LU", "fr-MA", "fr-MC", "fr-MF", "fr-MG", "fr-ML", "fr-MQ", "fr-MR", "fr-MU", "fr-NC", "fr-NE", "fr-PF", "fr-PM", "fr-RE", "fr-RW", "fr-SC", "fr-SN", "fr-SY", "fr-TD", "fr-TG", "fr-TN", "fr-VU", "fr-WF", "fr-YT", "ga", "ga-GB", "gd", "gl", "gu", "ha", "ha-GH", "ha-NE", "he", "hi", "hi-Latn", "hr", "hr-BA", "hsb", "hu", "hy", "ia", "id", "ig", "is", "it", "it-CH", "it-SM", "it-VA", "ja", "jv", "ka", "kea", "kgp", "kk", "km", "kn", "ko", "ko-KP", "kok", "ks", "ks-Arab", "ks-Deva", "ky", "lo", "lt", "lv", "mai", "mi", "mk", "ml", "mn", "mni", "mni-Beng", "mr", "ms", "ms-BN", "ms-ID", "ms-SG", "my", "nb", "nb-SJ", "ne", "ne-IN", "nl", "nl-AW", "nl-BE", "nl-BQ", "nl-CW", "nl-SR", "nl-SX", "nn", "no", "or", "pa", "pa-Guru", "pcm", "pl", "ps", "ps-PK", "pt", "pt-AO", "pt-CH", "pt-CV", "pt-GQ", "pt-GW", "pt-LU", "pt-MO", "pt-MZ", "pt-PT", "pt-ST", "pt-TL", "qu", "qu-BO", "qu-EC", "raj", "rm", "ro", "ro-MD", "ru", "ru-BY", "ru-KG", "ru-KZ", "ru-MD", "ru-UA", "sa", "sat", "sat-Olck", "sc", "sd", "sd-Arab", "sd-Deva", "si", "sk", "sl", "so", "so-DJ", "so-ET", "so-KE", "sq", "sq-MK", "sq-XK", "sr", "sr-Cyrl", "sr-Cyrl-BA", "sr-Cyrl-XK", "sr-Latn", "sr-Latn-BA", "sr-Latn-ME", "sr-Latn-XK", "su", "su-Latn", "sv", "sv-AX", "sv-FI", "sw", "sw-CD", "sw-KE", "sw-UG", "ta", "ta-LK", "ta-MY", "ta-SG", "te", "tg", "th", "ti", "ti-ER", "tk", "to", "tr", "tr-CY", "tt", "uk", "und", "ur", "ur-IN", "uz", "uz-Cyrl", "uz-Latn", "vi", "wo", "xh", "yo", "yo-BJ", "yrl", "yrl-CO", "yrl-VE", "yue", "yue-Hans", "yue-Hant", "zh", "zh-Hans", "zh-Hans-SG", "zh-Hant", "zh-Hant-HK", "zh-Hant-MO", "zu"];
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
