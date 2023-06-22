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
                static EN_ZA: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"afceafeaafsoafwemgmt") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x03\0\x06\0\n\0\r\0CATEATSASTWATGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static EN: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alasamceameaammoampaatlahaalmgmt") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x08\0\0\0\0\0\x03\0\x05\0\x07\0\t\0\x0B\0\r\0\x10\0AKTCTETMTPTATHATGMT") })
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ushnl\0\0\0utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0HSTUTC") })
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
                static ES: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"euceeueaeuwemgmt") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x03\0\x06\0\t\0CETEETWETGMT") })
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
                static FIL: <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1 {
                    defaults: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(zerovec::ZeroVec::new(), zerovec::VarZeroVec::new())
                    },
                    overrides: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"utc\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0UTC") })
                    },
                };
                static VALUES: [&<icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::Yokeable; 19usize] = [&AR, &AR, &BN, &BN, &EN, &EN_001, &EN_ZA, &ES, &ES_AR, &FIL, &FIL, &JA, &FIL, &ES, &ES, &ES, &FIL, &FIL, &FIL];
                static KEYS: [&str; 19usize] = ["ar", "ar-EG", "bn", "ccp", "en", "en-001", "en-ZA", "es", "es-AR", "fil", "fr", "ja", "ru", "sr", "sr-Cyrl", "sr-Latn", "th", "tr", "und"];
                if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(payload)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::MissingLocale.with_req(<icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
