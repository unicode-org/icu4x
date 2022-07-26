// @generated
type DataStruct = & 'static < :: icu_datetime :: provider :: time_zones :: TimeZoneFormatsV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub static DATA: &[(&str, DataStruct)] = &[
    ("ar", AR_AR_EG),
    ("ar-EG", AR_AR_EG),
    ("bn", BN),
    ("ccp", CCP),
    ("en", EN_EN_001_EN_ZA),
    ("en-001", EN_EN_001_EN_ZA),
    ("en-ZA", EN_EN_001_EN_ZA),
    ("es", ES),
    ("es-AR", ES_AR),
    ("fil", FIL),
    ("fr", FR),
    ("ja", JA),
    ("ru", RU),
    ("sr", SR_SR_CYRL),
    ("sr-Cyrl", SR_SR_CYRL),
    ("sr-Latn", SR_LATN),
    ("th", TH),
    ("tr", TR),
    ("und", UND),
];
static AR_AR_EG: DataStruct = &::icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
    hour_format: (
        alloc::borrow::Cow::Borrowed("+HH:mm"),
        alloc::borrow::Cow::Borrowed("-HH:mm"),
    ),
    gmt_format: alloc::borrow::Cow::Borrowed("غرينتش{0}"),
    gmt_zero_format: alloc::borrow::Cow::Borrowed("غرينتش"),
    region_format: alloc::borrow::Cow::Borrowed("توقيت {0}"),
    region_format_variants: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    100u8, 97u8, 121u8, 108u8, 105u8, 103u8, 104u8, 116u8, 115u8, 116u8, 97u8,
                    110u8, 100u8, 97u8, 114u8, 100u8,
                ])
            },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 27u8, 0u8, 0u8, 0u8, 216u8, 170u8,
                    217u8, 136u8, 217u8, 130u8, 217u8, 138u8, 216u8, 170u8, 32u8, 123u8, 48u8,
                    125u8, 32u8, 216u8, 167u8, 217u8, 132u8, 216u8, 181u8, 217u8, 138u8, 217u8,
                    129u8, 217u8, 138u8, 216u8, 170u8, 217u8, 136u8, 217u8, 130u8, 217u8, 138u8,
                    216u8, 170u8, 32u8, 123u8, 48u8, 125u8, 32u8, 216u8, 167u8, 217u8, 132u8,
                    216u8, 177u8, 216u8, 179u8, 217u8, 133u8, 217u8, 138u8,
                ])
            },
        )
    },
    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
};
static BN: DataStruct = &::icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
    hour_format: (
        alloc::borrow::Cow::Borrowed("+HH:mm"),
        alloc::borrow::Cow::Borrowed("-HH:mm"),
    ),
    gmt_format: alloc::borrow::Cow::Borrowed("GMT {0}"),
    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
    region_format: alloc::borrow::Cow::Borrowed("{0} সময\u{9bc}"),
    region_format_variants: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    100u8, 97u8, 121u8, 108u8, 105u8, 103u8, 104u8, 116u8, 115u8, 116u8, 97u8,
                    110u8, 100u8, 97u8, 114u8, 100u8,
                ])
            },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 38u8, 0u8, 0u8, 0u8, 123u8, 48u8,
                    125u8, 32u8, 224u8, 166u8, 166u8, 224u8, 166u8, 191u8, 224u8, 166u8, 172u8,
                    224u8, 166u8, 190u8, 224u8, 166u8, 178u8, 224u8, 167u8, 139u8, 224u8, 166u8,
                    149u8, 32u8, 224u8, 166u8, 184u8, 224u8, 166u8, 174u8, 224u8, 166u8, 175u8,
                    224u8, 166u8, 188u8, 123u8, 48u8, 125u8, 32u8, 224u8, 166u8, 174u8, 224u8,
                    166u8, 190u8, 224u8, 166u8, 168u8, 224u8, 166u8, 149u8, 32u8, 224u8, 166u8,
                    184u8, 224u8, 166u8, 174u8, 224u8, 166u8, 175u8, 224u8, 166u8, 188u8,
                ])
            },
        )
    },
    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
};
static CCP: DataStruct = &::icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
    hour_format: (
        alloc::borrow::Cow::Borrowed("+HH:mm"),
        alloc::borrow::Cow::Borrowed("-HH:mm"),
    ),
    gmt_format: alloc::borrow::Cow::Borrowed("GMT {0}"),
    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
    region_format: alloc::borrow::Cow::Borrowed("{0} 𑄃\u{11127}𑄇\u{11134}𑄖\u{11127}"),
    region_format_variants: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    100u8, 97u8, 121u8, 108u8, 105u8, 103u8, 104u8, 116u8, 115u8, 116u8, 97u8,
                    110u8, 100u8, 97u8, 114u8, 100u8,
                ])
            },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 73u8, 0u8, 0u8, 0u8, 123u8, 48u8,
                    125u8, 32u8, 240u8, 145u8, 132u8, 152u8, 240u8, 145u8, 132u8, 168u8, 240u8,
                    145u8, 132u8, 157u8, 240u8, 145u8, 132u8, 170u8, 240u8, 145u8, 132u8, 140u8,
                    240u8, 145u8, 132u8, 180u8, 240u8, 145u8, 132u8, 142u8, 240u8, 145u8, 132u8,
                    179u8, 240u8, 145u8, 132u8, 160u8, 32u8, 240u8, 145u8, 132u8, 131u8, 240u8,
                    145u8, 132u8, 167u8, 240u8, 145u8, 132u8, 135u8, 240u8, 145u8, 132u8, 180u8,
                    240u8, 145u8, 132u8, 150u8, 240u8, 145u8, 132u8, 167u8, 240u8, 145u8, 132u8,
                    150u8, 240u8, 145u8, 132u8, 180u8, 123u8, 48u8, 125u8, 32u8, 240u8, 145u8,
                    132u8, 159u8, 240u8, 145u8, 132u8, 154u8, 240u8, 145u8, 132u8, 167u8, 240u8,
                    145u8, 132u8, 135u8, 240u8, 145u8, 132u8, 180u8, 32u8, 240u8, 145u8, 132u8,
                    131u8, 240u8, 145u8, 132u8, 167u8, 240u8, 145u8, 132u8, 135u8, 240u8, 145u8,
                    132u8, 180u8, 240u8, 145u8, 132u8, 150u8, 240u8, 145u8, 132u8, 167u8, 240u8,
                    145u8, 132u8, 150u8, 240u8, 145u8, 132u8, 180u8,
                ])
            },
        )
    },
    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
};
static EN_EN_001_EN_ZA: DataStruct = &::icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
    hour_format: (
        alloc::borrow::Cow::Borrowed("+HH:mm"),
        alloc::borrow::Cow::Borrowed("-HH:mm"),
    ),
    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
    region_format: alloc::borrow::Cow::Borrowed("{0} Time"),
    region_format_variants: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    100u8, 97u8, 121u8, 108u8, 105u8, 103u8, 104u8, 116u8, 115u8, 116u8, 97u8,
                    110u8, 100u8, 97u8, 114u8, 100u8,
                ])
            },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 17u8, 0u8, 0u8, 0u8, 123u8, 48u8,
                    125u8, 32u8, 68u8, 97u8, 121u8, 108u8, 105u8, 103u8, 104u8, 116u8, 32u8, 84u8,
                    105u8, 109u8, 101u8, 123u8, 48u8, 125u8, 32u8, 83u8, 116u8, 97u8, 110u8, 100u8,
                    97u8, 114u8, 100u8, 32u8, 84u8, 105u8, 109u8, 101u8,
                ])
            },
        )
    },
    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
};
static ES: DataStruct = &::icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
    hour_format: (
        alloc::borrow::Cow::Borrowed("+HH:mm"),
        alloc::borrow::Cow::Borrowed("-HH:mm"),
    ),
    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
    region_format: alloc::borrow::Cow::Borrowed("hora de {0}"),
    region_format_variants: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    100u8, 97u8, 121u8, 108u8, 105u8, 103u8, 104u8, 116u8, 115u8, 116u8, 97u8,
                    110u8, 100u8, 97u8, 114u8, 100u8,
                ])
            },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 24u8, 0u8, 0u8, 0u8, 104u8, 111u8,
                    114u8, 97u8, 114u8, 105u8, 111u8, 32u8, 100u8, 101u8, 32u8, 118u8, 101u8,
                    114u8, 97u8, 110u8, 111u8, 32u8, 100u8, 101u8, 32u8, 123u8, 48u8, 125u8, 104u8,
                    111u8, 114u8, 97u8, 114u8, 105u8, 111u8, 32u8, 101u8, 115u8, 116u8, 195u8,
                    161u8, 110u8, 100u8, 97u8, 114u8, 32u8, 100u8, 101u8, 32u8, 123u8, 48u8, 125u8,
                ])
            },
        )
    },
    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
};
static ES_AR: DataStruct = &::icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
    hour_format: (
        alloc::borrow::Cow::Borrowed("+HH:mm"),
        alloc::borrow::Cow::Borrowed("-HH:mm"),
    ),
    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
    region_format: alloc::borrow::Cow::Borrowed("hora de {0}"),
    region_format_variants: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    100u8, 97u8, 121u8, 108u8, 105u8, 103u8, 104u8, 116u8, 115u8, 116u8, 97u8,
                    110u8, 100u8, 97u8, 114u8, 100u8,
                ])
            },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 21u8, 0u8, 0u8, 0u8, 104u8, 111u8,
                    114u8, 97u8, 32u8, 100u8, 101u8, 32u8, 118u8, 101u8, 114u8, 97u8, 110u8, 111u8,
                    32u8, 100u8, 101u8, 32u8, 123u8, 48u8, 125u8, 104u8, 111u8, 114u8, 97u8, 32u8,
                    101u8, 115u8, 116u8, 195u8, 161u8, 110u8, 100u8, 97u8, 114u8, 32u8, 100u8,
                    101u8, 32u8, 123u8, 48u8, 125u8,
                ])
            },
        )
    },
    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
};
static FIL: DataStruct = &::icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
    hour_format: (
        alloc::borrow::Cow::Borrowed("+HH:mm"),
        alloc::borrow::Cow::Borrowed("-HH:mm"),
    ),
    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
    region_format: alloc::borrow::Cow::Borrowed("Oras sa {0}"),
    region_format_variants: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    100u8, 97u8, 121u8, 108u8, 105u8, 103u8, 104u8, 116u8, 115u8, 116u8, 97u8,
                    110u8, 100u8, 97u8, 114u8, 100u8,
                ])
            },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 20u8, 0u8, 0u8, 0u8, 68u8, 97u8, 121u8,
                    108u8, 105u8, 103u8, 104u8, 116u8, 32u8, 84u8, 105u8, 109u8, 101u8, 32u8,
                    110u8, 103u8, 32u8, 123u8, 48u8, 125u8, 83u8, 116u8, 97u8, 110u8, 100u8, 97u8,
                    114u8, 100u8, 32u8, 110u8, 97u8, 32u8, 79u8, 114u8, 97u8, 115u8, 32u8, 115u8,
                    97u8, 32u8, 123u8, 48u8, 125u8,
                ])
            },
        )
    },
    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
};
static FR: DataStruct = &::icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
    hour_format: (
        alloc::borrow::Cow::Borrowed("+HH:mm"),
        alloc::borrow::Cow::Borrowed("−HH:mm"),
    ),
    gmt_format: alloc::borrow::Cow::Borrowed("UTC{0}"),
    gmt_zero_format: alloc::borrow::Cow::Borrowed("UTC"),
    region_format: alloc::borrow::Cow::Borrowed("heure : {0}"),
    region_format_variants: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    100u8, 97u8, 121u8, 108u8, 105u8, 103u8, 104u8, 116u8, 115u8, 116u8, 97u8,
                    110u8, 100u8, 97u8, 114u8, 100u8,
                ])
            },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 21u8, 0u8, 0u8, 0u8, 123u8, 48u8,
                    125u8, 32u8, 40u8, 104u8, 101u8, 117u8, 114u8, 101u8, 32u8, 100u8, 226u8,
                    128u8, 153u8, 195u8, 169u8, 116u8, 195u8, 169u8, 41u8, 123u8, 48u8, 125u8,
                    32u8, 40u8, 104u8, 101u8, 117u8, 114u8, 101u8, 32u8, 115u8, 116u8, 97u8, 110u8,
                    100u8, 97u8, 114u8, 100u8, 41u8,
                ])
            },
        )
    },
    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
};
static JA: DataStruct = &::icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
    hour_format: (
        alloc::borrow::Cow::Borrowed("+HH:mm"),
        alloc::borrow::Cow::Borrowed("-HH:mm"),
    ),
    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
    region_format: alloc::borrow::Cow::Borrowed("{0}時間"),
    region_format_variants: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    100u8, 97u8, 121u8, 108u8, 105u8, 103u8, 104u8, 116u8, 115u8, 116u8, 97u8,
                    110u8, 100u8, 97u8, 114u8, 100u8,
                ])
            },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 12u8, 0u8, 0u8, 0u8, 123u8, 48u8,
                    125u8, 229u8, 164u8, 143u8, 230u8, 153u8, 130u8, 233u8, 150u8, 147u8, 123u8,
                    48u8, 125u8, 230u8, 168u8, 153u8, 230u8, 186u8, 150u8, 230u8, 153u8, 130u8,
                ])
            },
        )
    },
    fallback_format: alloc::borrow::Cow::Borrowed("{1}（{0}）"),
    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
};
static RU: DataStruct = &::icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
    hour_format: (
        alloc::borrow::Cow::Borrowed("+HH:mm"),
        alloc::borrow::Cow::Borrowed("-HH:mm"),
    ),
    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
    region_format: alloc::borrow::Cow::Borrowed("{0}"),
    region_format_variants: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    100u8, 97u8, 121u8, 108u8, 105u8, 103u8, 104u8, 116u8, 115u8, 116u8, 97u8,
                    110u8, 100u8, 97u8, 114u8, 100u8,
                ])
            },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 28u8, 0u8, 0u8, 0u8, 123u8, 48u8,
                    125u8, 44u8, 32u8, 208u8, 187u8, 208u8, 181u8, 209u8, 130u8, 208u8, 189u8,
                    208u8, 181u8, 208u8, 181u8, 32u8, 208u8, 178u8, 209u8, 128u8, 208u8, 181u8,
                    208u8, 188u8, 209u8, 143u8, 123u8, 48u8, 125u8, 44u8, 32u8, 209u8, 129u8,
                    209u8, 130u8, 208u8, 176u8, 208u8, 189u8, 208u8, 180u8, 208u8, 176u8, 209u8,
                    128u8, 209u8, 130u8, 208u8, 189u8, 208u8, 190u8, 208u8, 181u8, 32u8, 208u8,
                    178u8, 209u8, 128u8, 208u8, 181u8, 208u8, 188u8, 209u8, 143u8,
                ])
            },
        )
    },
    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
};
static SR_LATN: DataStruct = &::icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
    hour_format: (
        alloc::borrow::Cow::Borrowed("+HH:mm"),
        alloc::borrow::Cow::Borrowed("-HH:mm"),
    ),
    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
    region_format: alloc::borrow::Cow::Borrowed("{0}"),
    region_format_variants: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    100u8, 97u8, 121u8, 108u8, 105u8, 103u8, 104u8, 116u8, 115u8, 116u8, 97u8,
                    110u8, 100u8, 97u8, 114u8, 100u8,
                ])
            },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 17u8, 0u8, 0u8, 0u8, 123u8, 48u8,
                    125u8, 44u8, 32u8, 108u8, 101u8, 116u8, 110u8, 106u8, 101u8, 32u8, 118u8,
                    114u8, 101u8, 109u8, 101u8, 123u8, 48u8, 125u8, 44u8, 32u8, 115u8, 116u8, 97u8,
                    110u8, 100u8, 97u8, 114u8, 100u8, 110u8, 111u8, 32u8, 118u8, 114u8, 101u8,
                    109u8, 101u8,
                ])
            },
        )
    },
    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
};
static SR_SR_CYRL: DataStruct = &::icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
    hour_format: (
        alloc::borrow::Cow::Borrowed("+HH:mm"),
        alloc::borrow::Cow::Borrowed("-HH:mm"),
    ),
    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
    region_format: alloc::borrow::Cow::Borrowed("{0}"),
    region_format_variants: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    100u8, 97u8, 121u8, 108u8, 105u8, 103u8, 104u8, 116u8, 115u8, 116u8, 97u8,
                    110u8, 100u8, 97u8, 114u8, 100u8,
                ])
            },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 26u8, 0u8, 0u8, 0u8, 123u8, 48u8,
                    125u8, 44u8, 32u8, 208u8, 187u8, 208u8, 181u8, 209u8, 130u8, 209u8, 154u8,
                    208u8, 181u8, 32u8, 208u8, 178u8, 209u8, 128u8, 208u8, 181u8, 208u8, 188u8,
                    208u8, 181u8, 123u8, 48u8, 125u8, 44u8, 32u8, 209u8, 129u8, 209u8, 130u8,
                    208u8, 176u8, 208u8, 189u8, 208u8, 180u8, 208u8, 176u8, 209u8, 128u8, 208u8,
                    180u8, 208u8, 189u8, 208u8, 190u8, 32u8, 208u8, 178u8, 209u8, 128u8, 208u8,
                    181u8, 208u8, 188u8, 208u8, 181u8,
                ])
            },
        )
    },
    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
};
static TH: DataStruct = &::icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
    hour_format: (
        alloc::borrow::Cow::Borrowed("+HH:mm"),
        alloc::borrow::Cow::Borrowed("-HH:mm"),
    ),
    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
    region_format: alloc::borrow::Cow::Borrowed("เวลา{0}"),
    region_format_variants: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    100u8, 97u8, 121u8, 108u8, 105u8, 103u8, 104u8, 116u8, 115u8, 116u8, 97u8,
                    110u8, 100u8, 97u8, 114u8, 100u8,
                ])
            },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 33u8, 0u8, 0u8, 0u8, 224u8, 185u8,
                    128u8, 224u8, 184u8, 167u8, 224u8, 184u8, 165u8, 224u8, 184u8, 178u8, 224u8,
                    184u8, 173u8, 224u8, 184u8, 173u8, 224u8, 184u8, 161u8, 224u8, 185u8, 129u8,
                    224u8, 184u8, 170u8, 224u8, 184u8, 135u8, 123u8, 48u8, 125u8, 224u8, 185u8,
                    128u8, 224u8, 184u8, 167u8, 224u8, 184u8, 165u8, 224u8, 184u8, 178u8, 224u8,
                    184u8, 161u8, 224u8, 184u8, 178u8, 224u8, 184u8, 149u8, 224u8, 184u8, 163u8,
                    224u8, 184u8, 144u8, 224u8, 184u8, 178u8, 224u8, 184u8, 153u8, 123u8, 48u8,
                    125u8,
                ])
            },
        )
    },
    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
};
static TR: DataStruct = &::icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
    hour_format: (
        alloc::borrow::Cow::Borrowed("+HH:mm"),
        alloc::borrow::Cow::Borrowed("-HH:mm"),
    ),
    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
    region_format: alloc::borrow::Cow::Borrowed("{0} Saati"),
    region_format_variants: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    100u8, 97u8, 121u8, 108u8, 105u8, 103u8, 104u8, 116u8, 115u8, 116u8, 97u8,
                    110u8, 100u8, 97u8, 114u8, 100u8,
                ])
            },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 13u8, 0u8, 0u8, 0u8, 123u8, 48u8,
                    125u8, 32u8, 89u8, 97u8, 122u8, 32u8, 83u8, 97u8, 97u8, 116u8, 105u8, 123u8,
                    48u8, 125u8, 32u8, 83u8, 116u8, 97u8, 110u8, 100u8, 97u8, 114u8, 116u8, 32u8,
                    83u8, 97u8, 97u8, 116u8, 105u8,
                ])
            },
        )
    },
    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
};
static UND: DataStruct = &::icu_datetime::provider::time_zones::TimeZoneFormatsV1 {
    hour_format: (
        alloc::borrow::Cow::Borrowed("+HH:mm"),
        alloc::borrow::Cow::Borrowed("-HH:mm"),
    ),
    gmt_format: alloc::borrow::Cow::Borrowed("GMT{0}"),
    gmt_zero_format: alloc::borrow::Cow::Borrowed("GMT"),
    region_format: alloc::borrow::Cow::Borrowed("{0}"),
    region_format_variants: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    100u8, 97u8, 121u8, 108u8, 105u8, 103u8, 104u8, 116u8, 115u8, 116u8, 97u8,
                    110u8, 100u8, 97u8, 114u8, 100u8,
                ])
            },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 123u8, 48u8, 125u8,
                    32u8, 40u8, 43u8, 49u8, 41u8, 123u8, 48u8, 125u8, 32u8, 40u8, 43u8, 48u8, 41u8,
                ])
            },
        )
    },
    fallback_format: alloc::borrow::Cow::Borrowed("{1} ({0})"),
    gmt_offset_fallback: alloc::borrow::Cow::Borrowed("GMT+?"),
};
