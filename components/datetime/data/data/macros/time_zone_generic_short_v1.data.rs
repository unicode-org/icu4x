// @generated
/// Implement [`DataProvider<MetazoneGenericNamesShortV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_time_zone_generic_short_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker>, icu_provider::DataError> {
                static KGP: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"acreamazbras") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0ACTAMTBRT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static AF: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"afceafeaafsoafwe") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x03\0\x06\0\n\0CATEATSASTWAT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static EN_BW: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"afceafeaafsoafwemgmt") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x03\0\x06\0\n\0\r\0CATEATSASTWATGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static EN_CA: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"afghalasamceameaammoampaargeatlaaucwaueaauwebangbhutbrasbrunchatchricocoeasteatiecuafalkgalagreagulfguyahaalinceindiindoineainweiranmalamaldmgmtnepanewfnoropakiparaperupimiurugvene") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"-\0\0\0\0\0\x03\0\x06\0\x08\0\n\0\x0C\0\x0E\0\x11\0\x13\0\x17\0\x1A\0\x1E\0!\0$\0'\0*\0/\x002\x005\09\0<\0?\0B\0F\0I\0L\0O\0R\0V\0Y\0\\\0_\0b\0f\0i\0l\0o\0r\0t\0w\0z\0}\0\x80\0\x83\0\x86\0AFTAKTCTETMTPTARTATACWTAETAWSTBSTBTTBRTBNTCHASTCXTCCTEASTTLTECTFKTGALTEGTN/AGYTHATWITAISTICTWITWIBIRSTMYTMVTGMTNPTNTFNTPKTPYTPETPMTUYTVET") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0HSTUTC") })
                    },
                };
                static EN_GU: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlachamhaalmgmt") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\t\0\0\0\0\0\x03\0\x05\0\x07\0\t\0\x0B\0\r\0\x11\0\x14\0AKTCTETMTPTATChSTHATGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0HSTUTC") })
                    },
                };
                static ID: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlacubagreahaalinceineainwenewfpimi") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0E\0\0\0\0\0\x03\0\x05\0\x07\0\t\0\x0B\0\r\0\x16\0\x19\0\x1C\0 \0#\0&\0(\0AKTCTETMTPTATCT (Kuba)EGTHATWITAWITWIBNTPMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static CS: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlaeuce") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x07\0\t\0\x0B\0\r\0AKTCTETMTPTATSE\xC4\x8C") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0HSTUTC") })
                    },
                };
                static EU: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlaeuceeueaeuwehaalmgmt") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0B\0\0\0\0\0\x03\0\x05\0\x07\0\t\0\x0B\0\r\0\x10\0\x13\0\x16\0\x19\0AKTCTETMTPTATCETEETWETHATGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0HSTUTC") })
                    },
                };
                static NB: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlaeuceeueaeuwehaalmgmt") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0B\0\0\0\0\0\x03\0\x05\0\x07\0\t\0\x0B\0\r\0\x10\0\x13\0\x16\0\x19\0AKTCTETMTPTATCETEETWETHATGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static NL_SR: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlaeuceeueaeuwehaalmgmtsuri") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x03\0\x05\0\x07\0\t\0\x0B\0\r\0\x10\0\x13\0\x16\0\x19\0\x1C\0AKTCTETMTPTATCETEETWETHATGMTSRT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0HSTUTC") })
                    },
                };
                static VI: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlahaal") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x07\0\t\0\x0B\0\r\0AKTCTETMTPTATHAT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\t\0Gi\xE1\xBB\x9D HSTUTC") })
                    },
                };
                static CHR: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlahaal") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x07\0\t\0\x0B\0\r\0AKTCTETMTPTATHAT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0HSTUTC") })
                    },
                };
                static AST: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlahaalmgmt") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x08\0\0\0\0\0\x03\0\x05\0\x07\0\t\0\x0B\0\r\0\x10\0AKTCTETMTPTATHATGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0HSTUTC") })
                    },
                };
                static ZH_HANT: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlahaalmgmt") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x08\0\0\0\0\0\x03\0\x05\0\x07\0\t\0\x0B\0\r\0\x10\0AKTCTETMTPTATHATGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static SC: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlamgmt") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x07\0\n\0\x0C\0\x0E\0OAKOTOOOMPOPOAOMG") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0OIHTCU") })
                    },
                };
                static FR_CA: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"amceameaammoampanewf") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x02\0\x04\0\x06\0\x08\0HCHEHRHPHT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static GA: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ampaeuceeueaeuwemgmt") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0AACCETEETWETMAG") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static ES_AR: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"argearwe") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0ARTWART") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static EN_NZ: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"auceaucwaueaauwechatlohomgmtneze") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x08\0\0\0\0\0\x03\0\x07\0\n\0\r\0\x11\0\x14\0\x17\0ACTACWTAETAWTCHATLHTGMTNZT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static EN_AU: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"auceaucwaueaauwegulflohomgmtneze") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x08\0\0\0\0\0\x03\0\x07\0\n\0\r\0\x14\0\x17\0\x1A\0ACTACWTAETAWTGulf STLHTGMTNZT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static PT_CV: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"azor") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0AZOT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static PT_CH: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"azoreuceeueaeuwe") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x04\0\x07\0\n\0AZOTCETEETWET") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static ES_BO: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"boli") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0BOT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static ES_CL: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"chil") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0CLT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static ES_CO: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"colo") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0COT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static ES_EC: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ecua") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ECT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static SK: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"euce") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0SE\xC4\x8C") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0HSTUTC") })
                    },
                };
                static DA: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"euceeueaeuwe") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0CETEETWET") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static DE: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"euceeueaeuwe") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0MEZOEZWEZ") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static EN_GB: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"euceeueaeuwegulfmgmt") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x03\0\x06\0\t\0\x0C\0CETEETWETGTSGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static SV: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"euceeueaeuwemgmt") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x03\0\x06\0\t\0CETEETWETGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x0B\0HonolulutidUTC") })
                    },
                };
                static BS: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"euceeueaeuwemgmt") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x03\0\x06\0\t\0CETEETWETGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static KK: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"euea") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0EET") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static FR_GF: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"frgu") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0GFT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static AR: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"gulf") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0GST") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static HI_LATN: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"gulfindimgmt") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0GSTISTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0HSTUTC") })
                    },
                };
                static EN_IN: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"gulfindimgmt") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0GSTISTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static EN_AE: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"gulfmgmt") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0GSTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static EN_GY: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"guyamgmt") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0GYTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static SO: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"haal") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0HAT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0Waqtiga UTC") })
                    },
                };
                static EN_MO: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"hokomacamgmt") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0HKTMSTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static EN_HK: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"hokomgmt") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0HKTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static HI: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"indi") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0IST") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x12\0\xE0\xA4\x8F\xE0\xA4\x9A\xE0\xA4\x8F\xE0\xA4\xB8\xE0\xA4\x9F\xE0\xA5\x80UTC") })
                    },
                };
                static BN: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"indi") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0IST") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static AS: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"indi") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xA6\xAD\xE0\xA6\xBE. \xE0\xA6\xB8.") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static OR: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"indimgmt") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0ISTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static JA: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"japa") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0JST") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static EN_MY: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"malamgmt") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0MYTGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static EN_SG: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"malamgmtsing") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x06\0MYTGMTSGT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static TA_MY: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"malasing") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0MYTSGT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static EN_001: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"mgmt") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0GMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static ES_PE: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"peru") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0PET") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static ES_UY: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"urug") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UYT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static ES_VE: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"vene") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0VET") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static FF_ADLM: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(zerovec::ZeroVec::new(), zerovec::VarZeroVec::new())
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x0C\0\xF0\x9E\xA4\x91\xF0\x9E\xA4\x96\xF0\x9E\xA4\x96\xF0\x9E\xA4\x91\xF0\x9E\xA4\x96\xF0\x9E\xA4\x8A") })
                    },
                };
                static AM: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(zerovec::ZeroVec::new(), zerovec::VarZeroVec::new())
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static SD: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(zerovec::ZeroVec::new(), zerovec::VarZeroVec::new())
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD9\x85 \xD8\xB9 \xD9\x88") })
                    },
                };
                static VALUES: [&<icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable; 444usize] = [&AF, &AF, &AM, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AS, &AST, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &BN, &BN, &AM, &BN, &BS, &AM, &BS, &BS, &BS, &BS, &BS, &BS, &AM, &CHR, &CS, &AM, &BS, &DA, &DA, &DE, &DE, &DE, &DE, &DE, &DE, &DE, &AM, &DE, &DA, &DA, &DA, &AST, &EN_001, &BS, &EN_AE, &EN_001, &EN_001, &AST, &BS, &EN_AU, &EN_001, &BS, &AST, &EN_001, &EN_001, &EN_BW, &EN_001, &EN_CA, &EN_001, &BS, &EN_001, &EN_BW, &EN_001, &EN_001, &BS, &EN_001, &BS, &EN_001, &EN_BW, &BS, &EN_001, &EN_001, &EN_001, &EN_GB, &EN_001, &EN_001, &EN_BW, &EN_001, &EN_BW, &EN_GU, &EN_GY, &EN_HK, &EN_001, &EN_001, &EN_001, &EN_IN, &EN_001, &EN_001, &EN_001, &EN_BW, &EN_001, &EN_001, &EN_001, &EN_001, &EN_BW, &EN_BW, &EN_BW, &EN_001, &EN_MO, &EN_001, &EN_001, &EN_001, &EN_BW, &EN_001, &EN_BW, &EN_MY, &EN_BW, &EN_001, &EN_BW, &BS, &EN_001, &EN_001, &EN_NZ, &EN_001, &AST, &EN_001, &EN_001, &AST, &EN_001, &EN_BW, &EN_001, &EN_001, &EN_BW, &BS, &EN_SG, &EN_001, &BS, &EN_BW, &EN_BW, &EN_001, &EN_BW, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_BW, &EN_BW, &AST, &EN_001, &EN_001, &AST, &EN_001, &EN_001, &EN_BW, &EN_BW, &EN_BW, &BS, &AM, &ES_AR, &ES_BO, &AM, &AM, &ES_CL, &ES_CO, &AM, &AM, &AM, &BS, &ES_EC, &BS, &AM, &AM, &BS, &AM, &AM, &AM, &ES_PE, &BS, &AM, &AM, &AM, &CHR, &ES_UY, &ES_VE, &AM, &EU, &AM, &AM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &FR_CA, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &FR_GF, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &GA, &GA, &EU, &BS, &BN, &AM, &AM, &AM, &AM, &HI, &HI_LATN, &BS, &BS, &DE, &BS, &AM, &AM, &ID, &AM, &AM, &DA, &DA, &DA, &DA, &JA, &AM, &AM, &AM, &KGP, &KK, &AM, &BN, &AM, &AM, &BN, &AM, &AM, &AM, &AM, &AM, &AM, &DA, &AM, &AM, &DA, &BN, &AM, &AM, &AM, &BN, &EN_SG, &EN_SG, &EN_SG, &EN_SG, &AM, &NB, &NB, &AM, &BN, &EU, &EU, &EU, &EU, &EU, &NL_SR, &EU, &NB, &NB, &OR, &BN, &BN, &AM, &DA, &AM, &AM, &KGP, &AM, &PT_CH, &PT_CV, &PT_CH, &AM, &PT_CH, &AM, &AM, &PT_CH, &AM, &AM, &ES_PE, &ES_BO, &ES_EC, &AM, &AM, &BS, &BS, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &SC, &SD, &SD, &AM, &AM, &SK, &AM, &SO, &SO, &SO, &SO, &AM, &AM, &AM, &BS, &BS, &BS, &BS, &BS, &BS, &BS, &BS, &AM, &AM, &SV, &SV, &SV, &AM, &AM, &AM, &AM, &BN, &BN, &TA_MY, &TA_MY, &BN, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &AM, &VI, &AM, &AM, &AM, &AM, &KGP, &KGP, &KGP, &AM, &AM, &AM, &AM, &AM, &TA_MY, &ZH_HANT, &ZH_HANT, &ZH_HANT, &AM];
                static KEYS: [&str; 444usize] = ["af", "af-NA", "am", "ar", "ar-AE", "ar-BH", "ar-DJ", "ar-DZ", "ar-EG", "ar-EH", "ar-ER", "ar-IL", "ar-IQ", "ar-JO", "ar-KM", "ar-KW", "ar-LB", "ar-LY", "ar-MA", "ar-MR", "ar-OM", "ar-PS", "ar-QA", "ar-SA", "ar-SD", "ar-SO", "ar-SS", "ar-SY", "ar-TD", "ar-TN", "ar-YE", "as", "ast", "az", "az-Latn", "be", "be-tarask", "bg", "bgc", "bho", "bn", "bn-IN", "br", "brx", "bs", "bs-Cyrl", "bs-Latn", "ca", "ca-AD", "ca-ES-valencia", "ca-FR", "ca-IT", "ceb", "chr", "cs", "cv", "cy", "da", "da-GL", "de", "de-AT", "de-BE", "de-CH", "de-IT", "de-LI", "de-LU", "doi", "dsb", "el", "el-CY", "el-polyton", "en", "en-001", "en-150", "en-AE", "en-AG", "en-AI", "en-AS", "en-AT", "en-AU", "en-BB", "en-BE", "en-BI", "en-BM", "en-BS", "en-BW", "en-BZ", "en-CA", "en-CC", "en-CH", "en-CK", "en-CM", "en-CX", "en-CY", "en-DE", "en-DG", "en-DK", "en-DM", "en-ER", "en-FI", "en-FJ", "en-FK", "en-FM", "en-GB", "en-GD", "en-GG", "en-GH", "en-GI", "en-GM", "en-GU", "en-GY", "en-HK", "en-IE", "en-IL", "en-IM", "en-IN", "en-IO", "en-JE", "en-JM", "en-KE", "en-KI", "en-KN", "en-KY", "en-LC", "en-LR", "en-LS", "en-MG", "en-MH", "en-MO", "en-MP", "en-MS", "en-MT", "en-MU", "en-MV", "en-MW", "en-MY", "en-NA", "en-NF", "en-NG", "en-NL", "en-NR", "en-NU", "en-NZ", "en-PG", "en-PH", "en-PK", "en-PN", "en-PR", "en-PW", "en-RW", "en-SB", "en-SC", "en-SD", "en-SE", "en-SG", "en-SH", "en-SI", "en-SL", "en-SS", "en-SX", "en-SZ", "en-TC", "en-TK", "en-TO", "en-TT", "en-TV", "en-TZ", "en-UG", "en-UM", "en-VC", "en-VG", "en-VI", "en-VU", "en-WS", "en-ZA", "en-ZM", "en-ZW", "es", "es-419", "es-AR", "es-BO", "es-BR", "es-BZ", "es-CL", "es-CO", "es-CR", "es-CU", "es-DO", "es-EA", "es-EC", "es-GQ", "es-GT", "es-HN", "es-IC", "es-MX", "es-NI", "es-PA", "es-PE", "es-PH", "es-PR", "es-PY", "es-SV", "es-US", "es-UY", "es-VE", "et", "eu", "fa", "fa-AF", "ff-Adlm", "ff-Adlm-BF", "ff-Adlm-CM", "ff-Adlm-GH", "ff-Adlm-GM", "ff-Adlm-GW", "ff-Adlm-LR", "ff-Adlm-MR", "ff-Adlm-NE", "ff-Adlm-NG", "ff-Adlm-SL", "ff-Adlm-SN", "fi", "fil", "fo", "fo-DK", "fr", "fr-BE", "fr-BF", "fr-BI", "fr-BJ", "fr-BL", "fr-CA", "fr-CD", "fr-CF", "fr-CG", "fr-CH", "fr-CI", "fr-CM", "fr-DJ", "fr-DZ", "fr-GA", "fr-GF", "fr-GN", "fr-GP", "fr-GQ", "fr-HT", "fr-KM", "fr-LU", "fr-MA", "fr-MC", "fr-MF", "fr-MG", "fr-ML", "fr-MQ", "fr-MR", "fr-MU", "fr-NC", "fr-NE", "fr-PF", "fr-PM", "fr-RE", "fr-RW", "fr-SC", "fr-SN", "fr-SY", "fr-TD", "fr-TG", "fr-TN", "fr-VU", "fr-WF", "fr-YT", "ga", "ga-GB", "gd", "gl", "gu", "ha", "ha-GH", "ha-NE", "he", "hi", "hi-Latn", "hr", "hr-BA", "hsb", "hu", "hy", "ia", "id", "ig", "is", "it", "it-CH", "it-SM", "it-VA", "ja", "jv", "ka", "kea", "kgp", "kk", "km", "kn", "ko", "ko-KP", "kok", "ks", "ks-Arab", "ks-Deva", "ky", "lo", "lt", "lv", "mai", "mi", "mk", "ml", "mn", "mni", "mni-Beng", "mr", "ms", "ms-BN", "ms-ID", "ms-SG", "my", "nb", "nb-SJ", "ne", "ne-IN", "nl", "nl-AW", "nl-BE", "nl-BQ", "nl-CW", "nl-SR", "nl-SX", "nn", "no", "or", "pa", "pa-Guru", "pcm", "pl", "ps", "ps-PK", "pt", "pt-AO", "pt-CH", "pt-CV", "pt-GQ", "pt-GW", "pt-LU", "pt-MO", "pt-MZ", "pt-PT", "pt-ST", "pt-TL", "qu", "qu-BO", "qu-EC", "raj", "rm", "ro", "ro-MD", "ru", "ru-BY", "ru-KG", "ru-KZ", "ru-MD", "ru-UA", "sa", "sat", "sat-Olck", "sc", "sd", "sd-Arab", "sd-Deva", "si", "sk", "sl", "so", "so-DJ", "so-ET", "so-KE", "sq", "sq-MK", "sq-XK", "sr", "sr-Cyrl", "sr-Cyrl-BA", "sr-Cyrl-XK", "sr-Latn", "sr-Latn-BA", "sr-Latn-ME", "sr-Latn-XK", "su", "su-Latn", "sv", "sv-AX", "sv-FI", "sw", "sw-CD", "sw-KE", "sw-UG", "ta", "ta-LK", "ta-MY", "ta-SG", "te", "tg", "th", "ti", "ti-ER", "tk", "to", "tr", "tr-CY", "tt", "uk", "und", "ur", "ur-IN", "uz", "uz-Cyrl", "uz-Latn", "vi", "wo", "xh", "yo", "yo-BJ", "yrl", "yrl-CO", "yrl-VE", "yue", "yue-Hans", "yue-Hant", "zh", "zh-Hans", "zh-Hans-SG", "zh-Hant", "zh-Hant-HK", "zh-Hant-MO", "zu"];
                let mut metadata = icu_provider::DataResponseMetadata::default();
                let payload = if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    payload
                } else {
                    let mut fallback_iterator = icu_locid_transform::fallback::LocaleFallbacker::new().fallback_for(<icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::KeyedDataMarker>::KEY.into(), req.locale.clone());
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
