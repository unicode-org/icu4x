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
                static FI: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+H.mm"), alloc::borrow::Cow::Borrowed("-H.mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("UTC{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("UTC"),
                    region_format: alloc::borrow::Cow::Borrowed("aikavyöhyke: {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x0F\0{0} (kes\xC3\xA4aika){0} (normaaliaika)") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static CS: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+H:mm"), alloc::borrow::Cow::Borrowed("-H:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("časové pásmo {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x08\0{0} (+1){0} (+0)") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static ID: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH.mm"), alloc::borrow::Cow::Borrowed("-HH.mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("Waktu {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x15\0Waktu Musim Panas {0}Waktu Standar {0}") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static SL: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH.mm"), alloc::borrow::Cow::Borrowed("-HH.mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0} čas"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x10\0{0} poletni \xC4\x8Das{0} standardni \xC4\x8Das") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static DA: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH.mm"), alloc::borrow::Cow::Borrowed("-HH.mm")),
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
                static SI: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH.mm"), alloc::borrow::Cow::Borrowed("-HH.mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("ග\u{dca}\u{200d}ර\u{dd2}මවේ{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("ග\u{dca}\u{200d}ර\u{dd2}මවේ"),
                    region_format: alloc::borrow::Cow::Borrowed("{0} වේල\u{dcf}ව"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0,\0{0} \xE0\xB6\xAF\xE0\xB7\x92\xE0\xB7\x80\xE0\xB7\x8F\xE0\xB6\x86\xE0\xB6\xBD\xE0\xB7\x9D\xE0\xB6\x9A \xE0\xB7\x80\xE0\xB7\x9A\xE0\xB6\xBD\xE0\xB7\x8F\xE0\xB7\x80{0} \xE0\xB7\x83\xE0\xB6\xB8\xE0\xB7\x8A\xE0\xB6\xB8\xE0\xB6\xAD \xE0\xB7\x80\xE0\xB7\x9A\xE0\xB6\xBD\xE0\xB7\x8F\xE0\xB7\x80") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static BS: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed(" -HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT {0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x13\0{0}, ljetno vrijeme{0}, standardno vrijeme") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static HR: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed(" -HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x13\0{0}, ljetno vrijeme{0}, standardno vrijeme") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static SW: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT {0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("Saa za {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x14\0Saa za Mchana za {0}Saa za wastani za {0}") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static UR: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT {0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0} وقت"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x1A\0{0} \xDA\x88\xDB\x92 \xD9\x84\xD8\xA7\xD8\xA6\xD9\xB9 \xD9\xB9\xD8\xA7\xD8\xA6\xD9\x85{0} \xD9\x85\xD8\xB9\xDB\x8C\xD8\xA7\xD8\xB1\xDB\x8C \xD9\x88\xD9\x82\xD8\xAA") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
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
                static CY: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("Amser {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0Amser Haf {0}Amser Safonol {0}") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static VI: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("Giờ {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x12\0Gi\xE1\xBB\x9D m\xC3\xB9a h\xC3\xA8 {0}Gi\xE1\xBB\x9D chu\xE1\xBA\xA9n {0}") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static CA: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("Hora de: {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x13\0Hora d\xE2\x80\x99estiu, {0}Hora est\xC3\xA0ndard, {0}") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static PT: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("Horário {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x17\0Hor\xC3\xA1rio de Ver\xC3\xA3o: {0}Hor\xC3\xA1rio Padr\xC3\xA3o: {0}") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static ZU: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("Isikhathi sase-{0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x16\0{0} Isikhathi sasemini{0} isikhathi esivamile") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static LV: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("Laika josla: {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x12\0{0}: vasaras laiks{0}: standarta laiks") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{0} ({1})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static IG: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("Oge {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x0B\0Oge Ihe {0}Oge Izugbe {0}") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static RO: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("Ora din {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x14\0Ora de var\xC4\x83 din {0}Ora standard din {0}") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static IT: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("Ora {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x0F\0Ora legale: {0}Ora standard: {0}") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static SQ: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("Ora: {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x0F\0Ora verore: {0}Ora standarde: {0}") })
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
                static MS: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("Waktu {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x0F\0Waktu Siang {0}Waktu Piawai {0}") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static SO: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("Waqtiga {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x18\0Waqtiga Dharaarta ee {0}Waqtiga Caadiga Ah ee {0}") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static JV: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("Wektu {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x10\0Wektu Ketigo {0}Wektu Standar {0}") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static PL: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("czas: {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x10\0{0} (czas letni){0} (czas standardowy)") })
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
                static GL: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("hora de: {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x16\0hora de ver\xC3\xA1n de: {0}hora est\xC3\xA1ndar de: {0}") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static NN: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("tidssone for {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x11\0sommartid \xE2\x80\x93 {0}normaltid \xE2\x80\x93 {0}") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static NO: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("tidssone for {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x11\0sommertid \xE2\x80\x93 {0}normaltid \xE2\x80\x93 {0}") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static DE: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0} (Ortszeit)"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x10\0{0} (Sommerzeit){0} (Normalzeit)") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static HA: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0} Lokaci"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x10\0{0} Lokacin Rana{0} Daidaitaccen Lokaci") })
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
                static PCM: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0} Taim"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x11\0{0} D\xC3\xA9la\xC3\xADt Taim{0} F\xC3\xADksd Taim") })
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
                static AZ: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0} Vaxtı"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x0E\0{0} Yay Vaxt\xC4\xB1{0} Standart Vaxt\xC4\xB1") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static EU: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0} aldeko ordua"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x11\0{0} (udako ordua){0} aldeko ordu estandarra") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static HU: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0} idő"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x0F\0{0} ny\xC3\xA1ri id\xC5\x91{0} z\xC3\xB3naid\xC5\x91") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static TK: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0} wagty"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x11\0{0} tomusky wagty{0} standart wagty") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static KK: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0} уақыты"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x1B\0{0} \xD0\xB6\xD0\xB0\xD0\xB7\xD2\x93\xD1\x8B \xD1\x83\xD0\xB0\xD2\x9B\xD1\x8B\xD1\x82\xD1\x8B{0} \xD1\x81\xD1\x82\xD0\xB0\xD0\xBD\xD0\xB4\xD0\xB0\xD1\x80\xD1\x82\xD1\x82\xD1\x8B \xD1\x83\xD0\xB0\xD2\x9B\xD1\x8B\xD1\x82\xD1\x8B") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static KY: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0} убактысы"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x08\0{0} (+1){0} (+0)") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static SD: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0} وقت"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x1A\0{0} \xDA\x8F\xD9\x8A\xD9\x86\xD9\x87\xD9\x86 \xD8\xAC\xD9\x88 \xD9\x88\xD9\x82\xD8\xAA{0} \xD9\x85\xD8\xB9\xD9\x8A\xD8\xA7\xD8\xB1\xD9\x8A \xD9\x88\xD9\x82\xD8\xAA") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static KOK: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0} व\u{947}ळ"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0 \0{0} \xE0\xA4\xA1\xE0\xA5\x87\xE0\xA4\xB2\xE0\xA4\xBE\xE0\xA4\xAF\xE0\xA4\x9F \xE0\xA4\xB5\xE0\xA5\x87\xE0\xA4\xB3{0} \xE0\xA4\xAA\xE0\xA5\x8D\xE0\xA4\xB0\xE0\xA4\xAE\xE0\xA4\xBE\xE0\xA4\xA3\xE0\xA4\xBF\xE0\xA4\xA4 \xE0\xA4\xB5\xE0\xA5\x87\xE0\xA4\xB3") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static HI: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0} समय"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0 \0{0} \xE0\xA4\xA1\xE0\xA5\x87\xE0\xA4\xB2\xE0\xA4\xBE\xE0\xA4\x87\xE0\xA4\x9F \xE0\xA4\xB8\xE0\xA4\xAE\xE0\xA4\xAF{0} \xE0\xA4\xAE\xE0\xA4\xBE\xE0\xA4\xA8\xE0\xA4\x95 \xE0\xA4\xB8\xE0\xA4\xAE\xE0\xA4\xAF") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static NE: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0} समय"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\n\0{0} (+\xE0\xA5\xA7){0} (+\xE0\xA5\xA6)") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static AS: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0} সময\u{9bc}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0(\0{0} (+1) \xE0\xA6\xA1\xE0\xA7\x87\xE0\xA6\xB2\xE0\xA6\xBE\xE0\xA6\x87\xE0\xA6\x9F \xE0\xA6\xB8\xE0\xA6\xAE\xE0\xA6\xAF\xE0\xA6\xBC{0} (+0) \xE0\xA6\xAE\xE0\xA6\xBE\xE0\xA6\xA8 \xE0\xA6\xB8\xE0\xA6\xAE\xE0\xA6\xAF\xE0\xA6\xBC") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static PA: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0} ਵ\u{a47}ਲਾ"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0&\0{0} \xE0\xA8\xAA\xE0\xA9\x8D\xE0\xA8\xB0\xE0\xA8\x95\xE0\xA8\xBE\xE0\xA8\xB8\xE0\xA8\xBC \xE0\xA8\xB5\xE0\xA9\x87\xE0\xA8\xB2\xE0\xA8\xBE{0} \xE0\xA8\xAE\xE0\xA8\xBF\xE0\xA8\x86\xE0\xA8\xB0\xE0\xA9\x80 \xE0\xA8\xB5\xE0\xA9\x87\xE0\xA8\xB2\xE0\xA8\xBE") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static GU: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0} સમય"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x1A\0{0} \xE0\xAA\xA6\xE0\xAA\xBF\xE0\xAA\xB5\xE0\xAA\xB8 \xE0\xAA\xB8\xE0\xAA\xAE\xE0\xAA\xAF{0} \xE0\xAA\xAE\xE0\xAA\xBE\xE0\xAA\xA8\xE0\xAA\x95 \xE0\xAA\xB8\xE0\xAA\xAE\xE0\xAA\xAF") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static OR: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0} ସମୟ"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0#\0{0} \xE0\xAC\xA6\xE0\xAC\xBF\xE0\xAC\xAC\xE0\xAC\xBE\xE0\xAC\xB2\xE0\xAD\x8B\xE0\xAC\x95 \xE0\xAC\xB8\xE0\xAC\xAE\xE0\xAD\x9F{0} \xE0\xAC\xAE\xE0\xAC\xBE\xE0\xAC\xA8\xE0\xAC\xBE\xE0\xAC\x99\xE0\xAD\x8D\xE0\xAC\x95 \xE0\xAC\xB8\xE0\xAC\xAE\xE0\xAD\x9F") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static TA: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0} நேரம\u{bcd}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0&\0{0} \xE0\xAE\xAA\xE0\xAE\x95\xE0\xAE\xB2\xE0\xAF\x8A\xE0\xAE\xB3\xE0\xAE\xBF \xE0\xAE\xA8\xE0\xAF\x87\xE0\xAE\xB0\xE0\xAE\xAE\xE0\xAF\x8D{0} \xE0\xAE\xA8\xE0\xAE\xBF\xE0\xAE\xB2\xE0\xAF\x88\xE0\xAE\xAF\xE0\xAE\xBE\xE0\xAE\xA9 \xE0\xAE\xA8\xE0\xAF\x87\xE0\xAE\xB0\xE0\xAE\xAE\xE0\xAF\x8D") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static TE: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0} సమయం"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\x006\0{0} \xE0\xB0\xAA\xE0\xB0\x97\xE0\xB0\x9F\xE0\xB0\xBF \xE0\xB0\xB5\xE0\xB1\x86\xE0\xB0\xB2\xE0\xB1\x81\xE0\xB0\xA4\xE0\xB1\x81\xE0\xB0\xB0\xE0\xB1\x81 \xE0\xB0\xB8\xE0\xB0\xAE\xE0\xB0\xAF\xE0\xB0\x82{0} \xE0\xB0\xAA\xE0\xB1\x8D\xE0\xB0\xB0\xE0\xB0\xBE\xE0\xB0\xAE\xE0\xB0\xBE\xE0\xB0\xA3\xE0\xB0\xBF\xE0\xB0\x95 \xE0\xB0\xB8\xE0\xB0\xAE\xE0\xB0\xAF\xE0\xB0\x82") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static KN: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0} ಸಮಯ"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x1A\0{0} \xE0\xB2\xA6\xE0\xB2\xBF\xE0\xB2\xA8\xE0\xB2\xA6 \xE0\xB2\xB8\xE0\xB2\xAE\xE0\xB2\xAF{0} \xE0\xB2\xAA\xE0\xB3\x8D\xE0\xB2\xB0\xE0\xB2\xAE\xE0\xB2\xBE\xE0\xB2\xA3\xE0\xB2\xBF\xE0\xB2\xA4 \xE0\xB2\xB8\xE0\xB2\xAE\xE0\xB2\xAF") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static MY: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0} အချ\u{102d}န\u{103a}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0;\0{0} \xE1\x80\x94\xE1\x80\xBD\xE1\x80\xB1\xE1\x80\x9B\xE1\x80\xAC\xE1\x80\x9E\xE1\x80\xAE \xE1\x80\x85\xE1\x80\xB6\xE1\x80\x90\xE1\x80\xB1\xE1\x80\xAC\xE1\x80\xBA\xE1\x80\x81\xE1\x80\xBB\xE1\x80\xAD\xE1\x80\x94\xE1\x80\xBA{0} \xE1\x80\x85\xE1\x80\xB6\xE1\x80\x90\xE1\x80\xB1\xE1\x80\xAC\xE1\x80\xBA\xE1\x80\x81\xE1\x80\xBB\xE1\x80\xAD\xE1\x80\x94\xE1\x80\xBA") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static KO: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0} 시간"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x14\0{0} \xED\x95\x98\xEA\xB3\x84 \xED\x91\x9C\xEC\xA4\x80\xEC\x8B\x9C{0} \xED\x91\x9C\xEC\xA4\x80\xEC\x8B\x9C") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1}({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static HY: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
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
                static IS: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x10\0{0} (sumart\xC3\xADmi){0} (sta\xC3\xB0alt\xC3\xADmi)") })
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
                static GD: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x14\0T\xC3\xACde samhraidh: {0}Bun-\xC3\xA0m: {0}") })
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
                static NL: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0}-tijd"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0zomertijd {0}standaardtijd {0}") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static AF: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0}-tyd"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0{0}-dagligtyd{0}-standaardtyd") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static MN: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0}-н цаг"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x16\0{0}-\xD0\xBD \xD0\xB7\xD1\x83\xD0\xBD\xD1\x8B \xD1\x86\xD0\xB0\xD0\xB3{0}-\xD0\xBD \xD1\x81\xD1\x82\xD0\xB0\xD0\xBD\xD0\xB4\xD0\xB0\xD1\x80\xD1\x82 \xD1\x86\xD0\xB0\xD0\xB3") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static YUE_HANS: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0}时间"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x08\0{0} (+1){0} (+0)") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static ZH: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0}时间"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x0F\0{0}\xE5\xA4\x8F\xE4\xBB\xA4\xE6\x97\xB6\xE9\x97\xB4{0}\xE6\xA0\x87\xE5\x87\x86\xE6\x97\xB6\xE9\x97\xB4") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1}（{0}）"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static ZH_HANT: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0}時間"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x08\0{0} (+1){0} (+0)") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1}（{0}）"),
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
                static YUE: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0}時間"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x0F\0{0}\xE5\xA4\x8F\xE4\xBB\xA4\xE6\x99\x82\xE9\x96\x93{0}\xE6\xA8\x99\xE6\xBA\x96\xE6\x99\x82\xE9\x96\x93") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static SK: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("časové pásmo {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x08\0{0} (+1){0} (+0)") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static EL: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("Ώρα ({0})"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x19\0\xCE\x98\xCE\xB5\xCF\x81\xCE\xB9\xCE\xBD\xCE\xAE \xCF\x8E\xCF\x81\xCE\xB1 ({0})\xCE\xA7\xCE\xB5\xCE\xB9\xCE\xBC\xCE\xB5\xCF\x81\xCE\xB9\xCE\xBD\xCE\xAE \xCF\x8E\xCF\x81\xCE\xB1 ({0})") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("[{1} ({0})]"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static MK: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("Време во {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x08\0{0} (+1){0} (+0)") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static BE: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("Час: {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x16\0\xD0\x9B\xD0\xB5\xD1\x82\xD0\xBD\xD1\x96 \xD1\x87\xD0\xB0\xD1\x81: {0}\xD0\xA1\xD1\x82\xD0\xB0\xD0\xBD\xD0\xB4\xD0\xB0\xD1\x80\xD1\x82\xD0\xBD\xD1\x8B \xD1\x87\xD0\xB0\xD1\x81: {0}") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static UK: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("час: {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x19\0\xD1\x87\xD0\xB0\xD1\x81: {0}, \xD0\xBB\xD1\x96\xD1\x82\xD0\xBD\xD1\x96\xD0\xB9\xD1\x87\xD0\xB0\xD1\x81: {0}, \xD1\x81\xD1\x82\xD0\xB0\xD0\xBD\xD0\xB4\xD0\xB0\xD1\x80\xD1\x82\xD0\xBD\xD0\xB8\xD0\xB9") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static PS: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("د {0} په وخت"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x18\0{0} \xD8\xB1\xDA\xBC\xD8\xA7 \xD9\x88\xD8\xB1\xDA\x81 \xD9\x88\xD8\xAE\xD8\xAA{0} \xD9\x85\xD8\xB9\xDB\x8C\xD8\xA7\xD8\xB1\xDB\x8C \xD9\x88\xD8\xAE\xD8\xAA") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
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
                static LO: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("ເວລາ {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0%\0\xE0\xBB\x80\xE0\xBA\xA7\xE0\xBA\xA5\xE0\xBA\xB2\xE0\xBA\x81\xE0\xBA\xB2\xE0\xBA\x87\xE0\xBB\x80\xE0\xBA\xA7\xE0\xBA\xB1\xE0\xBA\x99 {0}\xE0\xBB\x80\xE0\xBA\xA7\xE0\xBA\xA5\xE0\xBA\xB2\xE0\xBA\xA1\xE0\xBA\xB2\xE0\xBA\x94\xE0\xBA\x95\xE0\xBA\xB0\xE0\xBA\x96\xE0\xBA\xB2\xE0\xBA\x99 {0}") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static KA: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("დრო: {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0&\0{0} \xE1\x83\x96\xE1\x83\x90\xE1\x83\xA4\xE1\x83\xAE\xE1\x83\xA3\xE1\x83\x9A\xE1\x83\x98\xE1\x83\xA1 \xE1\x83\x93\xE1\x83\xA0\xE1\x83\x9D{0} \xE1\x83\xA1\xE1\x83\xA2\xE1\x83\x90\xE1\x83\x9C\xE1\x83\x93\xE1\x83\x90\xE1\x83\xA0\xE1\x83\xA2\xE1\x83\xA3\xE1\x83\x9A\xE1\x83\x98 \xE1\x83\x93\xE1\x83\xA0\xE1\x83\x9D") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static GA: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("MAG{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("MAG"),
                    region_format: alloc::borrow::Cow::Borrowed("{0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x08\0{0} (+1){0} (+0)") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static YO: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("WAT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("WAT"),
                    region_format: alloc::borrow::Cow::Borrowed("Ìgbà {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x19\0{0} \xC3\x80k\xC3\xB3k\xC3\xB2 oj\xC3\xBAm\xE1\xBB\x8Dm\xE1\xBB\x8D{0} \xC3\x8Cl\xC3\xA0n\xC3\xA0 \xC3\x80k\xC3\xB3k\xC3\xB2") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static MR: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("[GMT]{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("[GMT]"),
                    region_format: alloc::borrow::Cow::Borrowed("{0} व\u{947}ळ"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0/\0{0} \xE0\xA4\xB8\xE0\xA5\x82\xE0\xA4\xB0\xE0\xA5\x8D\xE0\xA4\xAF\xE0\xA4\xAA\xE0\xA5\x8D\xE0\xA4\xB0\xE0\xA4\x95\xE0\xA4\xBE\xE0\xA4\xB6 \xE0\xA4\xB5\xE0\xA5\x87\xE0\xA4\xB3{0} \xE0\xA4\xAA\xE0\xA5\x8D\xE0\xA4\xB0\xE0\xA4\xAE\xE0\xA4\xBE\xE0\xA4\xA3 \xE0\xA4\xB5\xE0\xA5\x87\xE0\xA4\xB3") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static BG: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("Гринуич{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("Гринуич"),
                    region_format: alloc::borrow::Cow::Borrowed("{0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0*\0{0} \xE2\x80\x93 \xD0\xBB\xD1\x8F\xD1\x82\xD0\xBD\xD0\xBE \xD1\x87\xD0\xB0\xD1\x81\xD0\xBE\xD0\xB2\xD0\xBE \xD0\xB2\xD1\x80\xD0\xB5\xD0\xBC\xD0\xB5{0} \xE2\x80\x93 \xD1\x81\xD1\x82\xD0\xB0\xD0\xBD\xD0\xB4\xD0\xB0\xD1\x80\xD1\x82\xD0\xBD\xD0\xBE \xD0\xB2\xD1\x80\xD0\xB5\xD0\xBC\xD0\xB5") })
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
                static ML: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("ജിഎംടി {0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("ജിഎംടി"),
                    region_format: alloc::borrow::Cow::Borrowed("{0} സമയം"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0)\0{0} \xE0\xB4\xA1\xE0\xB5\x87\xE0\xB4\xB2\xE0\xB5\x88\xE0\xB4\xB1\xE0\xB5\x8D\xE0\xB4\xB1\xE0\xB5\x8D \xE0\xB4\xB8\xE0\xB4\xAE\xE0\xB4\xAF\xE0\xB4\x82{0} \xE0\xB4\xB8\xE0\xB5\x8D\xE0\xB4\xB1\xE0\xB5\x8D\xE0\xB4\xB1\xE0\xB4\xBE\xE0\xB5\xBB\xE0\xB4\xA1\xE0\xB5\x87\xE0\xB5\xBC\xE0\xB4\xA1\xE0\xB5\x8D \xE0\xB4\xB8\xE0\xB4\xAE\xE0\xB4\xAF\xE0\xB4\x82") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static KM: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("ម\u{17c9}ោង\u{200b}សកល {0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("ម\u{17c9}ោង\u{200b}សកល"),
                    region_format: alloc::borrow::Cow::Borrowed("ម\u{17c9}ោង\u{200b}នៅ\u{200b} {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\x007\0\xE1\x9E\x98\xE1\x9F\x89\xE1\x9F\x84\xE1\x9E\x84\xE2\x80\x8B\xE1\x9E\x96\xE1\x9F\x81\xE1\x9E\x9B\xE2\x80\x8B\xE1\x9E\x90\xE1\x9F\x92\xE1\x9E\x84\xE1\x9F\x83\xE2\x80\x8B\xE1\x9E\x93\xE1\x9F\x85\xE2\x80\x8B {0}\xE1\x9E\x98\xE1\x9F\x89\xE1\x9F\x84\xE1\x9E\x84\xE2\x80\x8B\xE1\x9E\x9F\xE1\x9F\x92\xE1\x9E\x8F\xE1\x9E\x84\xE1\x9F\x8B\xE1\x9E\x8A\xE1\x9E\xB6\xE1\x9E\x9A\xE2\x80\x8B\xE1\x9E\x93\xE1\x9F\x85 \xE2\x80\x8B{0}") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static ET: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("−HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT {0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("({0})"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x08\0{0} (+1){0} (+0)") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static LT: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("−HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("Laikas: {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x13\0Vasaros laikas: {0}\xC5\xBDiemos laikas: {0}") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static SV: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HH:mm"), alloc::borrow::Cow::Borrowed("−HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("{0}tid"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x0F\0{0} (sommartid){0} (normaltid)") })
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
                static AM: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("+HHmm"), alloc::borrow::Cow::Borrowed("-HHmm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("ጂ ኤም ቲ{0}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("ጂ ኤም ቲ"),
                    region_format: alloc::borrow::Cow::Borrowed("{0} ጊዜ"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0$\0{0} \xE1\x8B\xA8\xE1\x89\x80\xE1\x8A\x95 \xE1\x89\xA5\xE1\x88\xAD\xE1\x88\x83\xE1\x8A\x95 \xE1\x88\xB0\xE1\x8B\x93\xE1\x89\xB5{0} \xE1\x88\x98\xE1\x8B\xB0\xE1\x89\xA0\xE1\x8A\x9B \xE1\x88\xB0\xE1\x8B\x93\xE1\x89\xB5") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static HE: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("\u{200e}+HH:mm"), alloc::borrow::Cow::Borrowed("-HH:mm\u{200e}")),
                    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}\u{200e}"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
                    region_format: alloc::borrow::Cow::Borrowed("שעון {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x15\0\xD7\xA9\xD7\xA2\xD7\x95\xD7\x9F {0} (\xD7\xA7\xD7\x99\xD7\xA5)\xD7\xA9\xD7\xA2\xD7\x95\xD7\x9F {0} (\xD7\x97\xD7\x95\xD7\xA8\xD7\xA3)") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static FA: <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
                    hour_format: (alloc::borrow::Cow::Borrowed("\u{200e}+HH:mm"), alloc::borrow::Cow::Borrowed("\u{200e}−HH:mm")),
                    gmt_format: alloc::borrow::Cow::Borrowed("{0} گرینویچ"),
                    gmt_zero_format: alloc::borrow::Cow::Borrowed("گرینویچ"),
                    region_format: alloc::borrow::Cow::Borrowed("وقت {0}"),
                    region_format_variants: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"daylightstandard") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x1B\0\xD9\x88\xD9\x82\xD8\xAA \xD8\xAA\xD8\xA7\xD8\xA8\xD8\xB3\xD8\xAA\xD8\xA7\xD9\x86\xDB\x8C {0}\xD9\x88\xD9\x82\xD8\xAA \xD8\xB9\xD8\xA7\xD8\xAF\xDB\x8C {0}") })
                    },
                    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
                    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
                };
                static VALUES: [&<icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::Yokeable; 94usize] = [&AF, &AM, &AR, &AS, &AZ, &BE, &BG, &BN, &BS, &CA, &CS, &CY, &DA, &DE, &EL, &EN, &ES, &ET, &EU, &FA, &FI, &FIL, &FR, &GA, &GD, &GL, &GU, &HA, &HE, &HI, &EN, &HR, &HU, &HY, &ID, &IG, &IS, &IT, &JA, &JV, &KA, &KK, &KM, &KN, &KO, &KOK, &KY, &LO, &LT, &LV, &MK, &ML, &MN, &MR, &MS, &MY, &NE, &NL, &NN, &NO, &OR, &PA, &PCM, &PL, &PS, &PT, &RO, &RU, &SD, &SI, &SK, &SL, &SO, &SQ, &SR, &SR_LATN, &SV, &SW, &TA, &TE, &TH, &TK, &TR, &UK, &HY, &UR, &HY, &VI, &YO, &YUE, &YUE_HANS, &ZH, &ZH_HANT, &ZU];
                static KEYS: [&str; 94usize] = ["af", "am", "ar", "as", "az", "be", "bg", "bn", "bs", "ca", "cs", "cy", "da", "de", "el", "en", "es", "et", "eu", "fa", "fi", "fil", "fr", "ga", "gd", "gl", "gu", "ha", "he", "hi", "hi-Latn", "hr", "hu", "hy", "id", "ig", "is", "it", "ja", "jv", "ka", "kk", "km", "kn", "ko", "kok", "ky", "lo", "lt", "lv", "mk", "ml", "mn", "mr", "ms", "my", "ne", "nl", "nn", "no", "or", "pa", "pcm", "pl", "ps", "pt", "ro", "ru", "sd", "si", "sk", "sl", "so", "sq", "sr", "sr-Latn", "sv", "sw", "ta", "te", "th", "tk", "tr", "uk", "und", "ur", "uz", "vi", "yo", "yue", "yue-Hans", "zh", "zh-Hant", "zu"];
                let mut metadata = icu_provider::DataResponseMetadata::default();
                let payload = if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    payload
                } else {
                    let mut fallback_iterator = icu_locid_transform::fallback::LocaleFallbacker::new().fallback_for(<icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::KeyedDataMarker>::KEY.into(), req.locale.clone());
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
