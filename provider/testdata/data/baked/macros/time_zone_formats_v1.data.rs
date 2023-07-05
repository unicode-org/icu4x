// @generated
/// Implement [`DataProvider<TimeZoneFormatsV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_time_zone_formats_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker>, icu_provider::DataError> {
                static BN: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT {0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0} সময\u{9bc}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0&\0{0} \xE0\xA6\xA6\xE0\xA6\xBF\xE0\xA6\xAC\xE0\xA6\xBE\xE0\xA6\xB2\xE0\xA7\x8B\xE0\xA6\x95 \xE0\xA6\xB8\xE0\xA6\xAE\xE0\xA6\xAF\xE0\xA6\xBC{0} \xE0\xA6\xAE\xE0\xA6\xBE\xE0\xA6\xA8\xE0\xA6\x95 \xE0\xA6\xB8\xE0\xA6\xAE\xE0\xA6\xAF\xE0\xA6\xBC") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static CCP: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT {0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0} 𑄃\u{11127}𑄇\u{11134}𑄖\u{11127}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0I\0{0} \xF0\x91\x84\x98\xF0\x91\x84\xA8\xF0\x91\x84\x9D\xF0\x91\x84\xAA\xF0\x91\x84\x8C\xF0\x91\x84\xB4\xF0\x91\x84\x8E\xF0\x91\x84\xB3\xF0\x91\x84\xA0 \xF0\x91\x84\x83\xF0\x91\x84\xA7\xF0\x91\x84\x87\xF0\x91\x84\xB4\xF0\x91\x84\x96\xF0\x91\x84\xA7\xF0\x91\x84\x96\xF0\x91\x84\xB4{0} \xF0\x91\x84\x9F\xF0\x91\x84\x9A\xF0\x91\x84\xA7\xF0\x91\x84\x87\xF0\x91\x84\xB4 \xF0\x91\x84\x83\xF0\x91\x84\xA7\xF0\x91\x84\x87\xF0\x91\x84\xB4\xF0\x91\x84\x96\xF0\x91\x84\xA7\xF0\x91\x84\x96\xF0\x91\x84\xB4") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static FIL: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("Oras sa {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x14\0Daylight Time ng {0}Standard na Oras sa {0}") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static ES_AR: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("hora de {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x15\0hora de verano de {0}hora est\xC3\xA1ndar de {0}") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static ES: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("hora de {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x18\0horario de verano de {0}horario est\xC3\xA1ndar de {0}") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static TR: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0} Saati"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0{0} Yaz Saati{0} Standart Saati") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static EN: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0} Time"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x11\0{0} Daylight Time{0} Standard Time") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static UND: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x08\0{0} (+1){0} (+0)") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static SR_LATN: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x11\0{0}, letnje vreme{0}, standardno vreme") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static SR: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x1A\0{0}, \xD0\xBB\xD0\xB5\xD1\x82\xD1\x9A\xD0\xB5 \xD0\xB2\xD1\x80\xD0\xB5\xD0\xBC\xD0\xB5{0}, \xD1\x81\xD1\x82\xD0\xB0\xD0\xBD\xD0\xB4\xD0\xB0\xD1\x80\xD0\xB4\xD0\xBD\xD0\xBE \xD0\xB2\xD1\x80\xD0\xB5\xD0\xBC\xD0\xB5") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static RU: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x1C\0{0}, \xD0\xBB\xD0\xB5\xD1\x82\xD0\xBD\xD0\xB5\xD0\xB5 \xD0\xB2\xD1\x80\xD0\xB5\xD0\xBC\xD1\x8F{0}, \xD1\x81\xD1\x82\xD0\xB0\xD0\xBD\xD0\xB4\xD0\xB0\xD1\x80\xD1\x82\xD0\xBD\xD0\xBE\xD0\xB5 \xD0\xB2\xD1\x80\xD0\xB5\xD0\xBC\xD1\x8F") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static JA: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0}時間"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x0C\0{0}\xE5\xA4\x8F\xE6\x99\x82\xE9\x96\x93{0}\xE6\xA8\x99\xE6\xBA\x96\xE6\x99\x82") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1}（{0}）"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static TH: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("เวลา{0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0!\0\xE0\xB9\x80\xE0\xB8\xA7\xE0\xB8\xA5\xE0\xB8\xB2\xE0\xB8\xAD\xE0\xB8\xAD\xE0\xB8\xA1\xE0\xB9\x81\xE0\xB8\xAA\xE0\xB8\x87{0}\xE0\xB9\x80\xE0\xB8\xA7\xE0\xB8\xA5\xE0\xB8\xB2\xE0\xB8\xA1\xE0\xB8\xB2\xE0\xB8\x95\xE0\xB8\xA3\xE0\xB8\x90\xE0\xB8\xB2\xE0\xB8\x99{0}") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static AR: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("غرينتش{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("غرينتش"),
                    region_format: alloc::borrow::Cow::Borrowed("توقيت {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x1B\0\xD8\xAA\xD9\x88\xD9\x82\xD9\x8A\xD8\xAA {0} \xD8\xA7\xD9\x84\xD8\xB5\xD9\x8A\xD9\x81\xD9\x8A\xD8\xAA\xD9\x88\xD9\x82\xD9\x8A\xD8\xAA {0} \xD8\xA7\xD9\x84\xD8\xB1\xD8\xB3\xD9\x85\xD9\x8A") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static FR: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("−HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("UTC{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("UTC"),
                    region_format: alloc::borrow::Cow::Borrowed("heure : {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x15\0{0} (heure d\xE2\x80\x99\xC3\xA9t\xC3\xA9){0} (heure standard)") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static VALUES: [&<icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable; 19usize] = [&AR, &AR, &BN, &CCP, &EN, &EN, &EN, &ES, &ES_AR, &FIL, &FR, &JA, &RU, &SR, &SR, &SR_LATN, &TH, &TR, &UND];
                static KEYS: [&str; 19usize] = ["ar", "ar-EG", "bn", "ccp", "en", "en-001", "en-ZA", "es", "es-AR", "fil", "fr", "ja", "ru", "sr", "sr-Cyrl", "sr-Latn", "th", "tr", "und"];
                if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(payload)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::MissingLocale.with_req(<icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
