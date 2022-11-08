// @generated
#![cfg(feature = "icu_relativetime")]
type DataStruct = < :: icu_relativetime :: provider :: StandardRelativeTimeV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_store_unchecked(&[
        ("ar", AR_AR_EG),
        ("ar-EG", AR_AR_EG),
        ("bn", BN),
        ("ccp", CCP),
        ("en", EN_EN_001_EN_ZA),
        ("en-001", EN_EN_001_EN_ZA),
        ("en-ZA", EN_EN_001_EN_ZA),
        ("es", ES_ES_AR),
        ("es-AR", ES_ES_AR),
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
    ]);
static AR_AR_EG: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternsV1([
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("الثواني")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 216u8, 167u8, 217u8, 132u8, 216u8, 162u8,
                        217u8, 134u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("الدقائق")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 217u8, 135u8, 216u8, 176u8, 217u8, 135u8,
                        32u8, 216u8, 167u8, 217u8, 132u8, 216u8, 175u8, 217u8, 130u8, 217u8, 138u8,
                        217u8, 130u8, 216u8, 169u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("الساعات")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 216u8, 167u8, 217u8, 132u8, 216u8, 179u8,
                        216u8, 167u8, 216u8, 185u8, 216u8, 169u8, 32u8, 216u8, 167u8, 217u8, 132u8,
                        216u8, 173u8, 216u8, 167u8, 217u8, 132u8, 217u8, 138u8, 216u8, 169u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("يوم")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[254u8, 255u8, 0u8, 1u8, 2u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 13u8, 0u8, 19u8, 0u8, 29u8, 0u8, 37u8, 0u8,
                        216u8, 163u8, 217u8, 136u8, 217u8, 132u8, 32u8, 216u8, 163u8, 217u8, 133u8,
                        216u8, 179u8, 216u8, 163u8, 217u8, 133u8, 216u8, 179u8, 216u8, 167u8,
                        217u8, 132u8, 217u8, 138u8, 217u8, 136u8, 217u8, 133u8, 216u8, 186u8,
                        216u8, 175u8, 217u8, 139u8, 216u8, 167u8, 216u8, 168u8, 216u8, 185u8,
                        216u8, 175u8, 32u8, 216u8, 167u8, 217u8, 132u8, 216u8, 186u8, 216u8, 175u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("الأسبوع")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 27u8, 0u8, 48u8, 0u8, 216u8, 167u8, 217u8,
                        132u8, 216u8, 163u8, 216u8, 179u8, 216u8, 168u8, 217u8, 136u8, 216u8,
                        185u8, 32u8, 216u8, 167u8, 217u8, 132u8, 217u8, 133u8, 216u8, 167u8, 216u8,
                        182u8, 217u8, 138u8, 217u8, 135u8, 216u8, 176u8, 216u8, 167u8, 32u8, 216u8,
                        167u8, 217u8, 132u8, 216u8, 163u8, 216u8, 179u8, 216u8, 168u8, 217u8,
                        136u8, 216u8, 185u8, 216u8, 167u8, 217u8, 132u8, 216u8, 163u8, 216u8,
                        179u8, 216u8, 168u8, 217u8, 136u8, 216u8, 185u8, 32u8, 216u8, 167u8, 217u8,
                        132u8, 217u8, 130u8, 216u8, 167u8, 216u8, 175u8, 217u8, 133u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("الشهر")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 23u8, 0u8, 40u8, 0u8, 216u8, 167u8, 217u8,
                        132u8, 216u8, 180u8, 217u8, 135u8, 216u8, 177u8, 32u8, 216u8, 167u8, 217u8,
                        132u8, 217u8, 133u8, 216u8, 167u8, 216u8, 182u8, 217u8, 138u8, 217u8,
                        135u8, 216u8, 176u8, 216u8, 167u8, 32u8, 216u8, 167u8, 217u8, 132u8, 216u8,
                        180u8, 217u8, 135u8, 216u8, 177u8, 216u8, 167u8, 217u8, 132u8, 216u8,
                        180u8, 217u8, 135u8, 216u8, 177u8, 32u8, 216u8, 167u8, 217u8, 132u8, 217u8,
                        130u8, 216u8, 167u8, 216u8, 175u8, 217u8, 133u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("ربع السنة")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 23u8, 0u8, 40u8, 0u8, 216u8, 167u8, 217u8,
                        132u8, 216u8, 177u8, 216u8, 168u8, 216u8, 185u8, 32u8, 216u8, 167u8, 217u8,
                        132u8, 216u8, 163u8, 216u8, 174u8, 217u8, 138u8, 216u8, 177u8, 217u8,
                        135u8, 216u8, 176u8, 216u8, 167u8, 32u8, 216u8, 167u8, 217u8, 132u8, 216u8,
                        177u8, 216u8, 168u8, 216u8, 185u8, 216u8, 167u8, 217u8, 132u8, 216u8,
                        177u8, 216u8, 168u8, 216u8, 185u8, 32u8, 216u8, 167u8, 217u8, 132u8, 217u8,
                        130u8, 216u8, 167u8, 216u8, 175u8, 217u8, 133u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("السنة")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 25u8, 0u8, 50u8, 0u8, 216u8, 167u8, 217u8,
                        132u8, 216u8, 179u8, 217u8, 134u8, 216u8, 169u8, 32u8, 216u8, 167u8, 217u8,
                        132u8, 217u8, 133u8, 216u8, 167u8, 216u8, 182u8, 217u8, 138u8, 216u8,
                        169u8, 216u8, 167u8, 217u8, 132u8, 216u8, 179u8, 217u8, 134u8, 216u8,
                        169u8, 32u8, 216u8, 167u8, 217u8, 132u8, 216u8, 173u8, 216u8, 167u8, 217u8,
                        132u8, 217u8, 138u8, 216u8, 169u8, 216u8, 167u8, 217u8, 132u8, 216u8,
                        179u8, 217u8, 134u8, 216u8, 169u8, 32u8, 216u8, 167u8, 217u8, 132u8, 217u8,
                        130u8, 216u8, 167u8, 216u8, 175u8, 217u8, 133u8, 216u8, 169u8,
                    ])
                },
            )
        },
    },
]);
static BN: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternsV1([
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("সেকেন\u{9cd}ড")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 224u8, 166u8, 143u8, 224u8, 166u8, 150u8,
                        224u8, 166u8, 168u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("মিনিট")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 224u8, 166u8, 143u8, 224u8, 166u8, 135u8,
                        32u8, 224u8, 166u8, 174u8, 224u8, 166u8, 191u8, 224u8, 166u8, 168u8, 224u8,
                        166u8, 191u8, 224u8, 166u8, 159u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("ঘণ\u{9cd}ট\u{9be}")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 224u8, 166u8, 143u8, 224u8, 166u8, 135u8,
                        32u8, 224u8, 166u8, 152u8, 224u8, 166u8, 163u8, 224u8, 167u8, 141u8, 224u8,
                        166u8, 159u8, 224u8, 166u8, 190u8, 224u8, 166u8, 175u8, 224u8, 166u8,
                        188u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("দিন")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[254u8, 255u8, 0u8, 1u8, 2u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 19u8, 0u8, 34u8, 0u8, 40u8, 0u8, 64u8, 0u8,
                        224u8, 166u8, 151u8, 224u8, 166u8, 164u8, 32u8, 224u8, 166u8, 170u8, 224u8,
                        166u8, 176u8, 224u8, 166u8, 182u8, 224u8, 167u8, 129u8, 224u8, 166u8,
                        151u8, 224u8, 166u8, 164u8, 224u8, 166u8, 149u8, 224u8, 166u8, 190u8,
                        224u8, 166u8, 178u8, 224u8, 166u8, 134u8, 224u8, 166u8, 156u8, 224u8,
                        166u8, 134u8, 224u8, 166u8, 151u8, 224u8, 166u8, 190u8, 224u8, 166u8,
                        174u8, 224u8, 167u8, 128u8, 224u8, 166u8, 149u8, 224u8, 166u8, 190u8,
                        224u8, 166u8, 178u8, 224u8, 166u8, 134u8, 224u8, 166u8, 151u8, 224u8,
                        166u8, 190u8, 224u8, 166u8, 174u8, 224u8, 167u8, 128u8, 32u8, 224u8, 166u8,
                        170u8, 224u8, 166u8, 176u8, 224u8, 166u8, 182u8, 224u8, 167u8, 129u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("সপ\u{9cd}ত\u{9be}হ")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 25u8, 0u8, 50u8, 0u8, 224u8, 166u8, 151u8,
                        224u8, 166u8, 164u8, 32u8, 224u8, 166u8, 184u8, 224u8, 166u8, 170u8, 224u8,
                        167u8, 141u8, 224u8, 166u8, 164u8, 224u8, 166u8, 190u8, 224u8, 166u8,
                        185u8, 224u8, 166u8, 143u8, 224u8, 166u8, 135u8, 32u8, 224u8, 166u8, 184u8,
                        224u8, 166u8, 170u8, 224u8, 167u8, 141u8, 224u8, 166u8, 164u8, 224u8,
                        166u8, 190u8, 224u8, 166u8, 185u8, 224u8, 166u8, 170u8, 224u8, 166u8,
                        176u8, 224u8, 167u8, 135u8, 224u8, 166u8, 176u8, 32u8, 224u8, 166u8, 184u8,
                        224u8, 166u8, 170u8, 224u8, 167u8, 141u8, 224u8, 166u8, 164u8, 224u8,
                        166u8, 190u8, 224u8, 166u8, 185u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("ম\u{9be}স")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 16u8, 0u8, 32u8, 0u8, 224u8, 166u8, 151u8,
                        224u8, 166u8, 164u8, 32u8, 224u8, 166u8, 174u8, 224u8, 166u8, 190u8, 224u8,
                        166u8, 184u8, 224u8, 166u8, 143u8, 224u8, 166u8, 135u8, 32u8, 224u8, 166u8,
                        174u8, 224u8, 166u8, 190u8, 224u8, 166u8, 184u8, 224u8, 166u8, 170u8,
                        224u8, 166u8, 176u8, 224u8, 167u8, 135u8, 224u8, 166u8, 176u8, 32u8, 224u8,
                        166u8, 174u8, 224u8, 166u8, 190u8, 224u8, 166u8, 184u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("ত\u{9cd}রৈম\u{9be}সিক")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 34u8, 0u8, 68u8, 0u8, 224u8, 166u8, 151u8,
                        224u8, 166u8, 164u8, 32u8, 224u8, 166u8, 164u8, 224u8, 167u8, 141u8, 224u8,
                        166u8, 176u8, 224u8, 167u8, 136u8, 224u8, 166u8, 174u8, 224u8, 166u8,
                        190u8, 224u8, 166u8, 184u8, 224u8, 166u8, 191u8, 224u8, 166u8, 149u8,
                        224u8, 166u8, 143u8, 224u8, 166u8, 135u8, 32u8, 224u8, 166u8, 164u8, 224u8,
                        167u8, 141u8, 224u8, 166u8, 176u8, 224u8, 167u8, 136u8, 224u8, 166u8,
                        174u8, 224u8, 166u8, 190u8, 224u8, 166u8, 184u8, 224u8, 166u8, 191u8,
                        224u8, 166u8, 149u8, 224u8, 166u8, 170u8, 224u8, 166u8, 176u8, 224u8,
                        167u8, 135u8, 224u8, 166u8, 176u8, 32u8, 224u8, 166u8, 164u8, 224u8, 167u8,
                        141u8, 224u8, 166u8, 176u8, 224u8, 167u8, 136u8, 224u8, 166u8, 174u8,
                        224u8, 166u8, 190u8, 224u8, 166u8, 184u8, 224u8, 166u8, 191u8, 224u8,
                        166u8, 149u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("বছর")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 16u8, 0u8, 32u8, 0u8, 224u8, 166u8, 151u8,
                        224u8, 166u8, 164u8, 32u8, 224u8, 166u8, 172u8, 224u8, 166u8, 155u8, 224u8,
                        166u8, 176u8, 224u8, 166u8, 143u8, 224u8, 166u8, 135u8, 32u8, 224u8, 166u8,
                        172u8, 224u8, 166u8, 155u8, 224u8, 166u8, 176u8, 224u8, 166u8, 170u8,
                        224u8, 166u8, 176u8, 224u8, 167u8, 135u8, 224u8, 166u8, 176u8, 32u8, 224u8,
                        166u8, 172u8, 224u8, 166u8, 155u8, 224u8, 166u8, 176u8,
                    ])
                },
            )
        },
    },
]);
static CCP: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternsV1([
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("𑄥𑄬𑄉𑄬𑄚\u{11134}")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 240u8, 145u8, 132u8, 131u8, 240u8, 145u8,
                        132u8, 168u8, 240u8, 145u8, 132u8, 135u8, 240u8, 145u8, 132u8, 180u8,
                        240u8, 145u8, 132u8, 133u8, 240u8, 145u8, 132u8, 154u8, 240u8, 145u8,
                        132u8, 170u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed(
            "𑄟\u{11128}𑄚\u{11128}𑄖\u{11134}",
        )),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 240u8, 145u8, 132u8, 131u8, 240u8, 145u8,
                        132u8, 179u8, 240u8, 145u8, 132u8, 134u8, 240u8, 145u8, 132u8, 172u8, 32u8,
                        240u8, 145u8, 132u8, 159u8, 240u8, 145u8, 132u8, 168u8, 240u8, 145u8,
                        132u8, 154u8, 240u8, 145u8, 132u8, 168u8, 240u8, 145u8, 132u8, 150u8,
                        240u8, 145u8, 132u8, 180u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("𑄊\u{1112e}𑄚\u{11134}𑄓")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 240u8, 145u8, 132u8, 131u8, 240u8, 145u8,
                        132u8, 179u8, 240u8, 145u8, 132u8, 134u8, 240u8, 145u8, 132u8, 172u8, 32u8,
                        240u8, 145u8, 132u8, 138u8, 240u8, 145u8, 132u8, 174u8, 240u8, 145u8,
                        132u8, 154u8, 240u8, 145u8, 132u8, 180u8, 240u8, 145u8, 132u8, 147u8,
                        240u8, 145u8, 132u8, 160u8, 240u8, 145u8, 132u8, 180u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("𑄘\u{11128}𑄚\u{11134}")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[254u8, 255u8, 0u8, 1u8, 2u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 49u8, 0u8, 101u8, 0u8, 129u8, 0u8, 189u8,
                        0u8, 240u8, 145u8, 132u8, 137u8, 240u8, 145u8, 132u8, 172u8, 240u8, 145u8,
                        132u8, 163u8, 240u8, 145u8, 132u8, 167u8, 240u8, 145u8, 132u8, 152u8,
                        240u8, 145u8, 132u8, 172u8, 32u8, 240u8, 145u8, 132u8, 155u8, 240u8, 145u8,
                        132u8, 167u8, 240u8, 145u8, 132u8, 162u8, 240u8, 145u8, 132u8, 180u8,
                        240u8, 145u8, 132u8, 165u8, 240u8, 145u8, 132u8, 170u8, 240u8, 145u8,
                        132u8, 137u8, 240u8, 145u8, 132u8, 172u8, 240u8, 145u8, 132u8, 163u8,
                        240u8, 145u8, 132u8, 180u8, 240u8, 145u8, 132u8, 163u8, 240u8, 145u8,
                        132u8, 179u8, 240u8, 145u8, 132u8, 160u8, 240u8, 145u8, 132u8, 135u8,
                        240u8, 145u8, 132u8, 172u8, 240u8, 145u8, 132u8, 163u8, 240u8, 145u8,
                        132u8, 179u8, 240u8, 145u8, 132u8, 160u8, 240u8, 145u8, 132u8, 172u8,
                        240u8, 145u8, 132u8, 131u8, 240u8, 145u8, 132u8, 172u8, 240u8, 145u8,
                        132u8, 140u8, 240u8, 145u8, 132u8, 180u8, 240u8, 145u8, 132u8, 165u8,
                        240u8, 145u8, 132u8, 179u8, 240u8, 145u8, 132u8, 160u8, 240u8, 145u8,
                        132u8, 131u8, 240u8, 145u8, 132u8, 172u8, 240u8, 145u8, 132u8, 142u8,
                        240u8, 145u8, 132u8, 172u8, 240u8, 145u8, 132u8, 150u8, 240u8, 145u8,
                        132u8, 180u8, 240u8, 145u8, 132u8, 150u8, 240u8, 145u8, 132u8, 179u8,
                        240u8, 145u8, 132u8, 160u8, 240u8, 145u8, 132u8, 135u8, 240u8, 145u8,
                        132u8, 172u8, 240u8, 145u8, 132u8, 163u8, 240u8, 145u8, 132u8, 179u8,
                        240u8, 145u8, 132u8, 160u8, 240u8, 145u8, 132u8, 172u8, 240u8, 145u8,
                        132u8, 131u8, 240u8, 145u8, 132u8, 172u8, 240u8, 145u8, 132u8, 142u8,
                        240u8, 145u8, 132u8, 172u8, 240u8, 145u8, 132u8, 150u8, 240u8, 145u8,
                        132u8, 180u8, 240u8, 145u8, 132u8, 150u8, 240u8, 145u8, 132u8, 179u8,
                        240u8, 145u8, 132u8, 160u8, 240u8, 145u8, 132u8, 172u8, 32u8, 240u8, 145u8,
                        132u8, 155u8, 240u8, 145u8, 132u8, 167u8, 240u8, 145u8, 132u8, 162u8,
                        240u8, 145u8, 132u8, 180u8, 240u8, 145u8, 132u8, 165u8, 240u8, 145u8,
                        132u8, 170u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("𑄥𑄛\u{11134}𑄖")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 41u8, 0u8, 74u8, 0u8, 240u8, 145u8, 132u8,
                        137u8, 240u8, 145u8, 132u8, 172u8, 240u8, 145u8, 132u8, 163u8, 240u8,
                        145u8, 132u8, 167u8, 240u8, 145u8, 132u8, 152u8, 240u8, 145u8, 132u8,
                        172u8, 32u8, 240u8, 145u8, 132u8, 165u8, 240u8, 145u8, 132u8, 155u8, 240u8,
                        145u8, 132u8, 180u8, 240u8, 145u8, 132u8, 150u8, 240u8, 145u8, 132u8,
                        131u8, 240u8, 145u8, 132u8, 179u8, 240u8, 145u8, 132u8, 134u8, 240u8,
                        145u8, 132u8, 172u8, 32u8, 240u8, 145u8, 132u8, 165u8, 240u8, 145u8, 132u8,
                        155u8, 240u8, 145u8, 132u8, 180u8, 240u8, 145u8, 132u8, 150u8, 240u8,
                        145u8, 132u8, 155u8, 240u8, 145u8, 132u8, 167u8, 240u8, 145u8, 132u8,
                        162u8, 240u8, 145u8, 132u8, 172u8, 32u8, 240u8, 145u8, 132u8, 165u8, 240u8,
                        145u8, 132u8, 155u8, 240u8, 145u8, 132u8, 180u8, 240u8, 145u8, 132u8,
                        150u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("𑄟𑄏\u{11134}")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 37u8, 0u8, 66u8, 0u8, 240u8, 145u8, 132u8,
                        137u8, 240u8, 145u8, 132u8, 172u8, 240u8, 145u8, 132u8, 163u8, 240u8,
                        145u8, 132u8, 167u8, 240u8, 145u8, 132u8, 152u8, 240u8, 145u8, 132u8,
                        172u8, 32u8, 240u8, 145u8, 132u8, 159u8, 240u8, 145u8, 132u8, 143u8, 240u8,
                        145u8, 132u8, 180u8, 240u8, 145u8, 132u8, 131u8, 240u8, 145u8, 132u8,
                        179u8, 240u8, 145u8, 132u8, 134u8, 240u8, 145u8, 132u8, 172u8, 32u8, 240u8,
                        145u8, 132u8, 159u8, 240u8, 145u8, 132u8, 143u8, 240u8, 145u8, 132u8,
                        180u8, 240u8, 145u8, 132u8, 155u8, 240u8, 145u8, 132u8, 167u8, 240u8,
                        145u8, 132u8, 162u8, 240u8, 145u8, 132u8, 172u8, 32u8, 240u8, 145u8, 132u8,
                        159u8, 240u8, 145u8, 132u8, 143u8, 240u8, 145u8, 132u8, 180u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed(
            "𑄖\u{11128}𑄚\u{11134}𑄟𑄏\u{11127}𑄢\u{11134}",
        )),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 61u8, 0u8, 114u8, 0u8, 240u8, 145u8, 132u8,
                        137u8, 240u8, 145u8, 132u8, 172u8, 240u8, 145u8, 132u8, 163u8, 240u8,
                        145u8, 132u8, 179u8, 240u8, 145u8, 132u8, 160u8, 240u8, 145u8, 132u8,
                        172u8, 32u8, 240u8, 145u8, 132u8, 150u8, 240u8, 145u8, 132u8, 168u8, 240u8,
                        145u8, 132u8, 154u8, 240u8, 145u8, 132u8, 180u8, 240u8, 145u8, 132u8,
                        159u8, 240u8, 145u8, 132u8, 143u8, 240u8, 145u8, 132u8, 167u8, 240u8,
                        145u8, 132u8, 162u8, 240u8, 145u8, 132u8, 180u8, 240u8, 145u8, 132u8,
                        131u8, 240u8, 145u8, 132u8, 179u8, 240u8, 145u8, 132u8, 134u8, 240u8,
                        145u8, 132u8, 172u8, 32u8, 240u8, 145u8, 132u8, 150u8, 240u8, 145u8, 132u8,
                        168u8, 240u8, 145u8, 132u8, 154u8, 240u8, 145u8, 132u8, 180u8, 240u8,
                        145u8, 132u8, 159u8, 240u8, 145u8, 132u8, 143u8, 240u8, 145u8, 132u8,
                        167u8, 240u8, 145u8, 132u8, 162u8, 240u8, 145u8, 132u8, 180u8, 240u8,
                        145u8, 132u8, 155u8, 240u8, 145u8, 132u8, 167u8, 240u8, 145u8, 132u8,
                        162u8, 240u8, 145u8, 132u8, 172u8, 32u8, 240u8, 145u8, 132u8, 150u8, 240u8,
                        145u8, 132u8, 168u8, 240u8, 145u8, 132u8, 154u8, 240u8, 145u8, 132u8,
                        180u8, 240u8, 145u8, 132u8, 159u8, 240u8, 145u8, 132u8, 143u8, 240u8,
                        145u8, 132u8, 167u8, 240u8, 145u8, 132u8, 162u8, 240u8, 145u8, 132u8,
                        180u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed(
            "𑄝\u{11127}𑄏\u{11127}𑄢\u{11134}",
        )),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 49u8, 0u8, 82u8, 0u8, 240u8, 145u8, 132u8,
                        137u8, 240u8, 145u8, 132u8, 172u8, 240u8, 145u8, 132u8, 163u8, 240u8,
                        145u8, 132u8, 179u8, 240u8, 145u8, 132u8, 160u8, 240u8, 145u8, 132u8,
                        172u8, 32u8, 240u8, 145u8, 132u8, 157u8, 240u8, 145u8, 132u8, 167u8, 240u8,
                        145u8, 132u8, 143u8, 240u8, 145u8, 132u8, 167u8, 240u8, 145u8, 132u8,
                        162u8, 240u8, 145u8, 132u8, 180u8, 240u8, 145u8, 132u8, 131u8, 240u8,
                        145u8, 132u8, 172u8, 32u8, 240u8, 145u8, 132u8, 157u8, 240u8, 145u8, 132u8,
                        167u8, 240u8, 145u8, 132u8, 143u8, 240u8, 145u8, 132u8, 167u8, 240u8,
                        145u8, 132u8, 162u8, 240u8, 145u8, 132u8, 180u8, 240u8, 145u8, 132u8,
                        142u8, 240u8, 145u8, 132u8, 172u8, 240u8, 145u8, 132u8, 162u8, 240u8,
                        145u8, 132u8, 167u8, 32u8, 240u8, 145u8, 132u8, 157u8, 240u8, 145u8, 132u8,
                        167u8, 240u8, 145u8, 132u8, 143u8, 240u8, 145u8, 132u8, 167u8, 240u8,
                        145u8, 132u8, 162u8, 240u8, 145u8, 132u8, 180u8,
                    ])
                },
            )
        },
    },
]);
static EN_EN_001_EN_ZA: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternsV1([
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("second")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 110u8, 111u8, 119u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("minute")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 104u8, 105u8, 115u8, 32u8, 109u8,
                        105u8, 110u8, 117u8, 116u8, 101u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("hour")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 104u8, 105u8, 115u8, 32u8, 104u8,
                        111u8, 117u8, 114u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("day")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 9u8, 0u8, 14u8, 0u8, 121u8, 101u8, 115u8,
                        116u8, 101u8, 114u8, 100u8, 97u8, 121u8, 116u8, 111u8, 100u8, 97u8, 121u8,
                        116u8, 111u8, 109u8, 111u8, 114u8, 114u8, 111u8, 119u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("week")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 9u8, 0u8, 18u8, 0u8, 108u8, 97u8, 115u8,
                        116u8, 32u8, 119u8, 101u8, 101u8, 107u8, 116u8, 104u8, 105u8, 115u8, 32u8,
                        119u8, 101u8, 101u8, 107u8, 110u8, 101u8, 120u8, 116u8, 32u8, 119u8, 101u8,
                        101u8, 107u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("month")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 10u8, 0u8, 20u8, 0u8, 108u8, 97u8, 115u8,
                        116u8, 32u8, 109u8, 111u8, 110u8, 116u8, 104u8, 116u8, 104u8, 105u8, 115u8,
                        32u8, 109u8, 111u8, 110u8, 116u8, 104u8, 110u8, 101u8, 120u8, 116u8, 32u8,
                        109u8, 111u8, 110u8, 116u8, 104u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("quarter")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 12u8, 0u8, 24u8, 0u8, 108u8, 97u8, 115u8,
                        116u8, 32u8, 113u8, 117u8, 97u8, 114u8, 116u8, 101u8, 114u8, 116u8, 104u8,
                        105u8, 115u8, 32u8, 113u8, 117u8, 97u8, 114u8, 116u8, 101u8, 114u8, 110u8,
                        101u8, 120u8, 116u8, 32u8, 113u8, 117u8, 97u8, 114u8, 116u8, 101u8, 114u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("year")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 9u8, 0u8, 18u8, 0u8, 108u8, 97u8, 115u8,
                        116u8, 32u8, 121u8, 101u8, 97u8, 114u8, 116u8, 104u8, 105u8, 115u8, 32u8,
                        121u8, 101u8, 97u8, 114u8, 110u8, 101u8, 120u8, 116u8, 32u8, 121u8, 101u8,
                        97u8, 114u8,
                    ])
                },
            )
        },
    },
]);
static ES_ES_AR: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternsV1([
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("segundo")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 104u8, 111u8, 114u8, 97u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("minuto")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 101u8, 115u8, 116u8, 101u8, 32u8, 109u8,
                        105u8, 110u8, 117u8, 116u8, 111u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("hora")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 101u8, 115u8, 116u8, 97u8, 32u8, 104u8,
                        111u8, 114u8, 97u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("día")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[254u8, 255u8, 0u8, 1u8, 2u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 8u8, 0u8, 12u8, 0u8, 15u8, 0u8, 22u8, 0u8,
                        97u8, 110u8, 116u8, 101u8, 97u8, 121u8, 101u8, 114u8, 97u8, 121u8, 101u8,
                        114u8, 104u8, 111u8, 121u8, 109u8, 97u8, 195u8, 177u8, 97u8, 110u8, 97u8,
                        112u8, 97u8, 115u8, 97u8, 100u8, 111u8, 32u8, 109u8, 97u8, 195u8, 177u8,
                        97u8, 110u8, 97u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("semana")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 16u8, 0u8, 27u8, 0u8, 108u8, 97u8, 32u8,
                        115u8, 101u8, 109u8, 97u8, 110u8, 97u8, 32u8, 112u8, 97u8, 115u8, 97u8,
                        100u8, 97u8, 101u8, 115u8, 116u8, 97u8, 32u8, 115u8, 101u8, 109u8, 97u8,
                        110u8, 97u8, 108u8, 97u8, 32u8, 112u8, 114u8, 195u8, 179u8, 120u8, 105u8,
                        109u8, 97u8, 32u8, 115u8, 101u8, 109u8, 97u8, 110u8, 97u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("mes")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 13u8, 0u8, 21u8, 0u8, 101u8, 108u8, 32u8,
                        109u8, 101u8, 115u8, 32u8, 112u8, 97u8, 115u8, 97u8, 100u8, 111u8, 101u8,
                        115u8, 116u8, 101u8, 32u8, 109u8, 101u8, 115u8, 101u8, 108u8, 32u8, 112u8,
                        114u8, 195u8, 179u8, 120u8, 105u8, 109u8, 111u8, 32u8, 109u8, 101u8, 115u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("trimestre")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 19u8, 0u8, 33u8, 0u8, 101u8, 108u8, 32u8,
                        116u8, 114u8, 105u8, 109u8, 101u8, 115u8, 116u8, 114u8, 101u8, 32u8, 112u8,
                        97u8, 115u8, 97u8, 100u8, 111u8, 101u8, 115u8, 116u8, 101u8, 32u8, 116u8,
                        114u8, 105u8, 109u8, 101u8, 115u8, 116u8, 114u8, 101u8, 101u8, 108u8, 32u8,
                        112u8, 114u8, 195u8, 179u8, 120u8, 105u8, 109u8, 111u8, 32u8, 116u8, 114u8,
                        105u8, 109u8, 101u8, 115u8, 116u8, 114u8, 101u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("año")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 14u8, 0u8, 23u8, 0u8, 101u8, 108u8, 32u8,
                        97u8, 195u8, 177u8, 111u8, 32u8, 112u8, 97u8, 115u8, 97u8, 100u8, 111u8,
                        101u8, 115u8, 116u8, 101u8, 32u8, 97u8, 195u8, 177u8, 111u8, 101u8, 108u8,
                        32u8, 112u8, 114u8, 195u8, 179u8, 120u8, 105u8, 109u8, 111u8, 32u8, 97u8,
                        195u8, 177u8, 111u8,
                    ])
                },
            )
        },
    },
]);
static FIL: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternsV1([
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("segundo")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 110u8, 103u8, 97u8, 121u8, 111u8, 110u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("minuto")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 32u8, 109u8, 105u8, 110u8,
                        117u8, 116u8, 111u8, 110u8, 103u8, 32u8, 105u8, 116u8, 111u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("oras")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 110u8, 103u8, 97u8, 121u8, 111u8, 110u8,
                        103u8, 32u8, 111u8, 114u8, 97u8, 115u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("araw")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[254u8, 255u8, 0u8, 1u8, 2u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 21u8, 0u8, 28u8, 0u8, 40u8, 0u8, 45u8, 0u8,
                        65u8, 114u8, 97u8, 119u8, 32u8, 98u8, 97u8, 103u8, 111u8, 32u8, 97u8,
                        110u8, 103u8, 32u8, 107u8, 97u8, 104u8, 97u8, 112u8, 111u8, 110u8, 107u8,
                        97u8, 104u8, 97u8, 112u8, 111u8, 110u8, 110u8, 103u8, 97u8, 121u8, 111u8,
                        110u8, 103u8, 32u8, 97u8, 114u8, 97u8, 119u8, 98u8, 117u8, 107u8, 97u8,
                        115u8, 83u8, 97u8, 109u8, 97u8, 107u8, 97u8, 108u8, 97u8, 119u8, 97u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("linggo")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 19u8, 0u8, 34u8, 0u8, 110u8, 97u8, 107u8,
                        97u8, 108u8, 105u8, 112u8, 97u8, 115u8, 32u8, 110u8, 97u8, 32u8, 108u8,
                        105u8, 110u8, 103u8, 103u8, 111u8, 115u8, 97u8, 32u8, 108u8, 105u8, 110u8,
                        103u8, 103u8, 111u8, 110u8, 103u8, 32u8, 105u8, 116u8, 111u8, 115u8, 117u8,
                        115u8, 117u8, 110u8, 111u8, 100u8, 32u8, 110u8, 97u8, 32u8, 108u8, 105u8,
                        110u8, 103u8, 103u8, 111u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("buwan")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 15u8, 0u8, 28u8, 0u8, 110u8, 97u8, 107u8,
                        97u8, 114u8, 97u8, 97u8, 110u8, 103u8, 32u8, 98u8, 117u8, 119u8, 97u8,
                        110u8, 110u8, 103u8, 97u8, 121u8, 111u8, 110u8, 103u8, 32u8, 98u8, 117u8,
                        119u8, 97u8, 110u8, 115u8, 117u8, 115u8, 117u8, 110u8, 111u8, 100u8, 32u8,
                        110u8, 97u8, 32u8, 98u8, 117u8, 119u8, 97u8, 110u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("quarter")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 17u8, 0u8, 32u8, 0u8, 110u8, 97u8, 107u8,
                        97u8, 114u8, 97u8, 97u8, 110u8, 103u8, 32u8, 113u8, 117u8, 97u8, 114u8,
                        116u8, 101u8, 114u8, 110u8, 103u8, 97u8, 121u8, 111u8, 110u8, 103u8, 32u8,
                        113u8, 117u8, 97u8, 114u8, 116u8, 101u8, 114u8, 115u8, 117u8, 115u8, 117u8,
                        110u8, 111u8, 100u8, 32u8, 110u8, 97u8, 32u8, 113u8, 117u8, 97u8, 114u8,
                        116u8, 101u8, 114u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("taon")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 14u8, 0u8, 26u8, 0u8, 110u8, 97u8, 107u8,
                        97u8, 114u8, 97u8, 97u8, 110u8, 103u8, 32u8, 116u8, 97u8, 111u8, 110u8,
                        110u8, 103u8, 97u8, 121u8, 111u8, 110u8, 103u8, 32u8, 116u8, 97u8, 111u8,
                        110u8, 115u8, 117u8, 115u8, 117u8, 110u8, 111u8, 100u8, 32u8, 110u8, 97u8,
                        32u8, 116u8, 97u8, 111u8, 110u8,
                    ])
                },
            )
        },
    },
]);
static FR: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternsV1([
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("seconde")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 109u8, 97u8, 105u8, 110u8, 116u8, 101u8,
                        110u8, 97u8, 110u8, 116u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("minute")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 99u8, 101u8, 116u8, 116u8, 101u8, 32u8,
                        109u8, 105u8, 110u8, 117u8, 116u8, 101u8, 45u8, 99u8, 105u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("heure")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 99u8, 101u8, 116u8, 116u8, 101u8, 32u8,
                        104u8, 101u8, 117u8, 114u8, 101u8, 45u8, 99u8, 105u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("jour")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[254u8, 255u8, 0u8, 1u8, 2u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 10u8, 0u8, 14u8, 0u8, 27u8, 0u8, 33u8, 0u8,
                        97u8, 118u8, 97u8, 110u8, 116u8, 45u8, 104u8, 105u8, 101u8, 114u8, 104u8,
                        105u8, 101u8, 114u8, 97u8, 117u8, 106u8, 111u8, 117u8, 114u8, 100u8, 226u8,
                        128u8, 153u8, 104u8, 117u8, 105u8, 100u8, 101u8, 109u8, 97u8, 105u8, 110u8,
                        97u8, 112u8, 114u8, 195u8, 168u8, 115u8, 45u8, 100u8, 101u8, 109u8, 97u8,
                        105u8, 110u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("semaine")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 20u8, 0u8, 33u8, 0u8, 108u8, 97u8, 32u8,
                        115u8, 101u8, 109u8, 97u8, 105u8, 110u8, 101u8, 32u8, 100u8, 101u8, 114u8,
                        110u8, 105u8, 195u8, 168u8, 114u8, 101u8, 99u8, 101u8, 116u8, 116u8, 101u8,
                        32u8, 115u8, 101u8, 109u8, 97u8, 105u8, 110u8, 101u8, 108u8, 97u8, 32u8,
                        115u8, 101u8, 109u8, 97u8, 105u8, 110u8, 101u8, 32u8, 112u8, 114u8, 111u8,
                        99u8, 104u8, 97u8, 105u8, 110u8, 101u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("mois")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 15u8, 0u8, 25u8, 0u8, 108u8, 101u8, 32u8,
                        109u8, 111u8, 105u8, 115u8, 32u8, 100u8, 101u8, 114u8, 110u8, 105u8, 101u8,
                        114u8, 99u8, 101u8, 32u8, 109u8, 111u8, 105u8, 115u8, 45u8, 99u8, 105u8,
                        108u8, 101u8, 32u8, 109u8, 111u8, 105u8, 115u8, 32u8, 112u8, 114u8, 111u8,
                        99u8, 104u8, 97u8, 105u8, 110u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("trimestre")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 20u8, 0u8, 32u8, 0u8, 108u8, 101u8, 32u8,
                        116u8, 114u8, 105u8, 109u8, 101u8, 115u8, 116u8, 114u8, 101u8, 32u8, 100u8,
                        101u8, 114u8, 110u8, 105u8, 101u8, 114u8, 99u8, 101u8, 32u8, 116u8, 114u8,
                        105u8, 109u8, 101u8, 115u8, 116u8, 114u8, 101u8, 108u8, 101u8, 32u8, 116u8,
                        114u8, 105u8, 109u8, 101u8, 115u8, 116u8, 114u8, 101u8, 32u8, 112u8, 114u8,
                        111u8, 99u8, 104u8, 97u8, 105u8, 110u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("année")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 20u8, 0u8, 32u8, 0u8, 108u8, 226u8, 128u8,
                        153u8, 97u8, 110u8, 110u8, 195u8, 169u8, 101u8, 32u8, 100u8, 101u8, 114u8,
                        110u8, 105u8, 195u8, 168u8, 114u8, 101u8, 99u8, 101u8, 116u8, 116u8, 101u8,
                        32u8, 97u8, 110u8, 110u8, 195u8, 169u8, 101u8, 108u8, 226u8, 128u8, 153u8,
                        97u8, 110u8, 110u8, 195u8, 169u8, 101u8, 32u8, 112u8, 114u8, 111u8, 99u8,
                        104u8, 97u8, 105u8, 110u8, 101u8,
                    ])
                },
            )
        },
    },
]);
static JA: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternsV1([
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("秒")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 228u8, 187u8, 138u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("分")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 49u8, 32u8, 229u8, 136u8, 134u8, 228u8,
                        187u8, 165u8, 229u8, 134u8, 133u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("時")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 49u8, 32u8, 230u8, 153u8, 130u8, 233u8,
                        150u8, 147u8, 228u8, 187u8, 165u8, 229u8, 134u8, 133u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("日")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[254u8, 255u8, 0u8, 1u8, 2u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 9u8, 0u8, 15u8, 0u8, 21u8, 0u8, 27u8, 0u8,
                        228u8, 184u8, 128u8, 230u8, 152u8, 168u8, 230u8, 151u8, 165u8, 230u8,
                        152u8, 168u8, 230u8, 151u8, 165u8, 228u8, 187u8, 138u8, 230u8, 151u8,
                        165u8, 230u8, 152u8, 142u8, 230u8, 151u8, 165u8, 230u8, 152u8, 142u8,
                        229u8, 190u8, 140u8, 230u8, 151u8, 165u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("週")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 6u8, 0u8, 12u8, 0u8, 229u8, 133u8, 136u8,
                        233u8, 128u8, 177u8, 228u8, 187u8, 138u8, 233u8, 128u8, 177u8, 230u8,
                        157u8, 165u8, 233u8, 128u8, 177u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("月")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 6u8, 0u8, 12u8, 0u8, 229u8, 133u8, 136u8,
                        230u8, 156u8, 136u8, 228u8, 187u8, 138u8, 230u8, 156u8, 136u8, 230u8,
                        157u8, 165u8, 230u8, 156u8, 136u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("四半期")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 12u8, 0u8, 24u8, 0u8, 229u8, 137u8, 141u8,
                        229u8, 155u8, 155u8, 229u8, 141u8, 138u8, 230u8, 156u8, 159u8, 228u8,
                        187u8, 138u8, 229u8, 155u8, 155u8, 229u8, 141u8, 138u8, 230u8, 156u8,
                        159u8, 231u8, 191u8, 140u8, 229u8, 155u8, 155u8, 229u8, 141u8, 138u8,
                        230u8, 156u8, 159u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("年")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 6u8, 0u8, 12u8, 0u8, 230u8, 152u8, 168u8,
                        229u8, 185u8, 180u8, 228u8, 187u8, 138u8, 229u8, 185u8, 180u8, 230u8,
                        157u8, 165u8, 229u8, 185u8, 180u8,
                    ])
                },
            )
        },
    },
]);
static RU: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternsV1([
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("секунда")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 209u8, 129u8, 208u8, 181u8, 208u8, 185u8,
                        209u8, 135u8, 208u8, 176u8, 209u8, 129u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("минута")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 208u8, 178u8, 32u8, 209u8, 141u8, 209u8,
                        130u8, 209u8, 131u8, 32u8, 208u8, 188u8, 208u8, 184u8, 208u8, 189u8, 209u8,
                        131u8, 209u8, 130u8, 209u8, 131u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("час")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 208u8, 178u8, 32u8, 209u8, 141u8, 209u8,
                        130u8, 208u8, 190u8, 209u8, 130u8, 32u8, 209u8, 135u8, 208u8, 176u8, 209u8,
                        129u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("день")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[254u8, 255u8, 0u8, 1u8, 2u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 18u8, 0u8, 28u8, 0u8, 42u8, 0u8, 54u8, 0u8,
                        208u8, 191u8, 208u8, 190u8, 208u8, 183u8, 208u8, 176u8, 208u8, 178u8,
                        209u8, 135u8, 208u8, 181u8, 209u8, 128u8, 208u8, 176u8, 208u8, 178u8,
                        209u8, 135u8, 208u8, 181u8, 209u8, 128u8, 208u8, 176u8, 209u8, 129u8,
                        208u8, 181u8, 208u8, 179u8, 208u8, 190u8, 208u8, 180u8, 208u8, 189u8,
                        209u8, 143u8, 208u8, 183u8, 208u8, 176u8, 208u8, 178u8, 209u8, 130u8,
                        209u8, 128u8, 208u8, 176u8, 208u8, 191u8, 208u8, 190u8, 209u8, 129u8,
                        208u8, 187u8, 208u8, 181u8, 208u8, 183u8, 208u8, 176u8, 208u8, 178u8,
                        209u8, 130u8, 209u8, 128u8, 208u8, 176u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("неделя")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 32u8, 0u8, 58u8, 0u8, 208u8, 189u8, 208u8,
                        176u8, 32u8, 208u8, 191u8, 209u8, 128u8, 208u8, 190u8, 209u8, 136u8, 208u8,
                        187u8, 208u8, 190u8, 208u8, 185u8, 32u8, 208u8, 189u8, 208u8, 181u8, 208u8,
                        180u8, 208u8, 181u8, 208u8, 187u8, 208u8, 181u8, 208u8, 189u8, 208u8,
                        176u8, 32u8, 209u8, 141u8, 209u8, 130u8, 208u8, 190u8, 208u8, 185u8, 32u8,
                        208u8, 189u8, 208u8, 181u8, 208u8, 180u8, 208u8, 181u8, 208u8, 187u8,
                        208u8, 181u8, 208u8, 189u8, 208u8, 176u8, 32u8, 209u8, 129u8, 208u8, 187u8,
                        208u8, 181u8, 208u8, 180u8, 209u8, 131u8, 209u8, 142u8, 209u8, 137u8,
                        208u8, 181u8, 208u8, 185u8, 32u8, 208u8, 189u8, 208u8, 181u8, 208u8, 180u8,
                        208u8, 181u8, 208u8, 187u8, 208u8, 181u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("месяц")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 30u8, 0u8, 54u8, 0u8, 208u8, 178u8, 32u8,
                        208u8, 191u8, 209u8, 128u8, 208u8, 190u8, 209u8, 136u8, 208u8, 187u8,
                        208u8, 190u8, 208u8, 188u8, 32u8, 208u8, 188u8, 208u8, 181u8, 209u8, 129u8,
                        209u8, 143u8, 209u8, 134u8, 208u8, 181u8, 208u8, 178u8, 32u8, 209u8, 141u8,
                        209u8, 130u8, 208u8, 190u8, 208u8, 188u8, 32u8, 208u8, 188u8, 208u8, 181u8,
                        209u8, 129u8, 209u8, 143u8, 209u8, 134u8, 208u8, 181u8, 208u8, 178u8, 32u8,
                        209u8, 129u8, 208u8, 187u8, 208u8, 181u8, 208u8, 180u8, 209u8, 131u8,
                        209u8, 142u8, 209u8, 137u8, 208u8, 181u8, 208u8, 188u8, 32u8, 208u8, 188u8,
                        208u8, 181u8, 209u8, 129u8, 209u8, 143u8, 209u8, 134u8, 208u8, 181u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("квартал")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 34u8, 0u8, 68u8, 0u8, 208u8, 178u8, 32u8,
                        208u8, 191u8, 209u8, 128u8, 208u8, 190u8, 209u8, 136u8, 208u8, 187u8,
                        208u8, 190u8, 208u8, 188u8, 32u8, 208u8, 186u8, 208u8, 178u8, 208u8, 176u8,
                        209u8, 128u8, 209u8, 130u8, 208u8, 176u8, 208u8, 187u8, 208u8, 181u8,
                        208u8, 178u8, 32u8, 209u8, 130u8, 208u8, 181u8, 208u8, 186u8, 209u8, 131u8,
                        209u8, 137u8, 208u8, 181u8, 208u8, 188u8, 32u8, 208u8, 186u8, 208u8, 178u8,
                        208u8, 176u8, 209u8, 128u8, 209u8, 130u8, 208u8, 176u8, 208u8, 187u8,
                        208u8, 181u8, 208u8, 178u8, 32u8, 209u8, 129u8, 208u8, 187u8, 208u8, 181u8,
                        208u8, 180u8, 209u8, 131u8, 209u8, 142u8, 209u8, 137u8, 208u8, 181u8,
                        208u8, 188u8, 32u8, 208u8, 186u8, 208u8, 178u8, 208u8, 176u8, 209u8, 128u8,
                        209u8, 130u8, 208u8, 176u8, 208u8, 187u8, 208u8, 181u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("год")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 26u8, 0u8, 46u8, 0u8, 208u8, 178u8, 32u8,
                        208u8, 191u8, 209u8, 128u8, 208u8, 190u8, 209u8, 136u8, 208u8, 187u8,
                        208u8, 190u8, 208u8, 188u8, 32u8, 208u8, 179u8, 208u8, 190u8, 208u8, 180u8,
                        209u8, 131u8, 208u8, 178u8, 32u8, 209u8, 141u8, 209u8, 130u8, 208u8, 190u8,
                        208u8, 188u8, 32u8, 208u8, 179u8, 208u8, 190u8, 208u8, 180u8, 209u8, 131u8,
                        208u8, 178u8, 32u8, 209u8, 129u8, 208u8, 187u8, 208u8, 181u8, 208u8, 180u8,
                        209u8, 131u8, 209u8, 142u8, 209u8, 137u8, 208u8, 181u8, 208u8, 188u8, 32u8,
                        208u8, 179u8, 208u8, 190u8, 208u8, 180u8, 209u8, 131u8,
                    ])
                },
            )
        },
    },
]);
static SR_LATN: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternsV1([
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("sekund")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 97u8, 100u8, 97u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("minut")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 111u8, 118u8, 111u8, 103u8, 32u8, 109u8,
                        105u8, 110u8, 117u8, 116u8, 97u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("sat")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 111u8, 118u8, 111u8, 103u8, 32u8, 115u8,
                        97u8, 116u8, 97u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("dan")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[254u8, 255u8, 0u8, 1u8, 2u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 9u8, 0u8, 14u8, 0u8, 19u8, 0u8, 24u8, 0u8,
                        112u8, 114u8, 101u8, 107u8, 106u8, 117u8, 196u8, 141u8, 101u8, 106u8,
                        117u8, 196u8, 141u8, 101u8, 100u8, 97u8, 110u8, 97u8, 115u8, 115u8, 117u8,
                        116u8, 114u8, 97u8, 112u8, 114u8, 101u8, 107u8, 111u8, 115u8, 117u8, 116u8,
                        114u8, 97u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("nedelja")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 15u8, 0u8, 26u8, 0u8, 112u8, 114u8, 111u8,
                        197u8, 161u8, 108u8, 101u8, 32u8, 110u8, 101u8, 100u8, 101u8, 108u8, 106u8,
                        101u8, 111u8, 118u8, 101u8, 32u8, 110u8, 101u8, 100u8, 101u8, 108u8, 106u8,
                        101u8, 115u8, 108u8, 101u8, 100u8, 101u8, 196u8, 135u8, 101u8, 32u8, 110u8,
                        101u8, 100u8, 101u8, 108u8, 106u8, 101u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("mesec")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 15u8, 0u8, 26u8, 0u8, 112u8, 114u8, 111u8,
                        197u8, 161u8, 108u8, 111u8, 103u8, 32u8, 109u8, 101u8, 115u8, 101u8, 99u8,
                        97u8, 111u8, 118u8, 111u8, 103u8, 32u8, 109u8, 101u8, 115u8, 101u8, 99u8,
                        97u8, 115u8, 108u8, 101u8, 100u8, 101u8, 196u8, 135u8, 101u8, 103u8, 32u8,
                        109u8, 101u8, 115u8, 101u8, 99u8, 97u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("kvartal")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 17u8, 0u8, 30u8, 0u8, 112u8, 114u8, 111u8,
                        197u8, 161u8, 108u8, 111u8, 103u8, 32u8, 107u8, 118u8, 97u8, 114u8, 116u8,
                        97u8, 108u8, 97u8, 111u8, 118u8, 111u8, 103u8, 32u8, 107u8, 118u8, 97u8,
                        114u8, 116u8, 97u8, 108u8, 97u8, 115u8, 108u8, 101u8, 100u8, 101u8, 196u8,
                        135u8, 101u8, 103u8, 32u8, 107u8, 118u8, 97u8, 114u8, 116u8, 97u8, 108u8,
                        97u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("godina")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 14u8, 0u8, 24u8, 0u8, 112u8, 114u8, 111u8,
                        197u8, 161u8, 108u8, 101u8, 32u8, 103u8, 111u8, 100u8, 105u8, 110u8, 101u8,
                        111u8, 118u8, 101u8, 32u8, 103u8, 111u8, 100u8, 105u8, 110u8, 101u8, 115u8,
                        108u8, 101u8, 100u8, 101u8, 196u8, 135u8, 101u8, 32u8, 103u8, 111u8, 100u8,
                        105u8, 110u8, 101u8,
                    ])
                },
            )
        },
    },
]);
static SR_SR_CYRL: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternsV1([
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("секунд")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 209u8, 129u8, 208u8, 176u8, 208u8, 180u8,
                        208u8, 176u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("минут")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 208u8, 190u8, 208u8, 178u8, 208u8, 190u8,
                        208u8, 179u8, 32u8, 208u8, 188u8, 208u8, 184u8, 208u8, 189u8, 209u8, 131u8,
                        209u8, 130u8, 208u8, 176u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("сат")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 208u8, 190u8, 208u8, 178u8, 208u8, 190u8,
                        208u8, 179u8, 32u8, 209u8, 129u8, 208u8, 176u8, 209u8, 130u8, 208u8, 176u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("дан")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[254u8, 255u8, 0u8, 1u8, 2u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 16u8, 0u8, 24u8, 0u8, 34u8, 0u8, 44u8, 0u8,
                        208u8, 191u8, 209u8, 128u8, 208u8, 181u8, 208u8, 186u8, 209u8, 152u8,
                        209u8, 131u8, 209u8, 135u8, 208u8, 181u8, 209u8, 152u8, 209u8, 131u8,
                        209u8, 135u8, 208u8, 181u8, 208u8, 180u8, 208u8, 176u8, 208u8, 189u8,
                        208u8, 176u8, 209u8, 129u8, 209u8, 129u8, 209u8, 131u8, 209u8, 130u8,
                        209u8, 128u8, 208u8, 176u8, 208u8, 191u8, 209u8, 128u8, 208u8, 181u8,
                        208u8, 186u8, 208u8, 190u8, 209u8, 129u8, 209u8, 131u8, 209u8, 130u8,
                        209u8, 128u8, 208u8, 176u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("недеља")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 25u8, 0u8, 44u8, 0u8, 208u8, 191u8, 209u8,
                        128u8, 208u8, 190u8, 209u8, 136u8, 208u8, 187u8, 208u8, 181u8, 32u8, 208u8,
                        189u8, 208u8, 181u8, 208u8, 180u8, 208u8, 181u8, 209u8, 153u8, 208u8,
                        181u8, 208u8, 190u8, 208u8, 178u8, 208u8, 181u8, 32u8, 208u8, 189u8, 208u8,
                        181u8, 208u8, 180u8, 208u8, 181u8, 209u8, 153u8, 208u8, 181u8, 209u8,
                        129u8, 208u8, 187u8, 208u8, 181u8, 208u8, 180u8, 208u8, 181u8, 209u8,
                        155u8, 208u8, 181u8, 32u8, 208u8, 189u8, 208u8, 181u8, 208u8, 180u8, 208u8,
                        181u8, 209u8, 153u8, 208u8, 181u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("месец")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 27u8, 0u8, 48u8, 0u8, 208u8, 191u8, 209u8,
                        128u8, 208u8, 190u8, 209u8, 136u8, 208u8, 187u8, 208u8, 190u8, 208u8,
                        179u8, 32u8, 208u8, 188u8, 208u8, 181u8, 209u8, 129u8, 208u8, 181u8, 209u8,
                        134u8, 208u8, 176u8, 208u8, 190u8, 208u8, 178u8, 208u8, 190u8, 208u8,
                        179u8, 32u8, 208u8, 188u8, 208u8, 181u8, 209u8, 129u8, 208u8, 181u8, 209u8,
                        134u8, 208u8, 176u8, 209u8, 129u8, 208u8, 187u8, 208u8, 181u8, 208u8,
                        180u8, 208u8, 181u8, 209u8, 155u8, 208u8, 181u8, 208u8, 179u8, 32u8, 208u8,
                        188u8, 208u8, 181u8, 209u8, 129u8, 208u8, 181u8, 209u8, 134u8, 208u8,
                        176u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("квартал")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 31u8, 0u8, 56u8, 0u8, 208u8, 191u8, 209u8,
                        128u8, 208u8, 190u8, 209u8, 136u8, 208u8, 187u8, 208u8, 190u8, 208u8,
                        179u8, 32u8, 208u8, 186u8, 208u8, 178u8, 208u8, 176u8, 209u8, 128u8, 209u8,
                        130u8, 208u8, 176u8, 208u8, 187u8, 208u8, 176u8, 208u8, 190u8, 208u8,
                        178u8, 208u8, 190u8, 208u8, 179u8, 32u8, 208u8, 186u8, 208u8, 178u8, 208u8,
                        176u8, 209u8, 128u8, 209u8, 130u8, 208u8, 176u8, 208u8, 187u8, 208u8,
                        176u8, 209u8, 129u8, 208u8, 187u8, 208u8, 181u8, 208u8, 180u8, 208u8,
                        181u8, 209u8, 155u8, 208u8, 181u8, 208u8, 179u8, 32u8, 208u8, 186u8, 208u8,
                        178u8, 208u8, 176u8, 209u8, 128u8, 209u8, 130u8, 208u8, 176u8, 208u8,
                        187u8, 208u8, 176u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("година")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 25u8, 0u8, 44u8, 0u8, 208u8, 191u8, 209u8,
                        128u8, 208u8, 190u8, 209u8, 136u8, 208u8, 187u8, 208u8, 181u8, 32u8, 208u8,
                        179u8, 208u8, 190u8, 208u8, 180u8, 208u8, 184u8, 208u8, 189u8, 208u8,
                        181u8, 208u8, 190u8, 208u8, 178u8, 208u8, 181u8, 32u8, 208u8, 179u8, 208u8,
                        190u8, 208u8, 180u8, 208u8, 184u8, 208u8, 189u8, 208u8, 181u8, 209u8,
                        129u8, 208u8, 187u8, 208u8, 181u8, 208u8, 180u8, 208u8, 181u8, 209u8,
                        155u8, 208u8, 181u8, 32u8, 208u8, 179u8, 208u8, 190u8, 208u8, 180u8, 208u8,
                        184u8, 208u8, 189u8, 208u8, 181u8,
                    ])
                },
            )
        },
    },
]);
static TH: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternsV1([
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("ว\u{e34}นาท\u{e35}")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 224u8, 184u8, 130u8, 224u8, 184u8, 147u8,
                        224u8, 184u8, 176u8, 224u8, 184u8, 153u8, 224u8, 184u8, 181u8, 224u8,
                        185u8, 137u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("นาท\u{e35}")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 224u8, 184u8, 153u8, 224u8, 184u8, 178u8,
                        224u8, 184u8, 151u8, 224u8, 184u8, 181u8, 224u8, 184u8, 153u8, 224u8,
                        184u8, 181u8, 224u8, 185u8, 137u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("ช\u{e31}\u{e48}วโมง")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 224u8, 184u8, 138u8, 224u8, 184u8, 177u8,
                        224u8, 185u8, 136u8, 224u8, 184u8, 167u8, 224u8, 185u8, 130u8, 224u8,
                        184u8, 161u8, 224u8, 184u8, 135u8, 224u8, 184u8, 153u8, 224u8, 184u8,
                        181u8, 224u8, 185u8, 137u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("ว\u{e31}น")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[254u8, 255u8, 0u8, 1u8, 2u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 33u8, 0u8, 57u8, 0u8, 75u8, 0u8, 99u8, 0u8,
                        224u8, 185u8, 128u8, 224u8, 184u8, 161u8, 224u8, 184u8, 183u8, 224u8,
                        185u8, 136u8, 224u8, 184u8, 173u8, 224u8, 184u8, 167u8, 224u8, 184u8,
                        178u8, 224u8, 184u8, 153u8, 224u8, 184u8, 139u8, 224u8, 184u8, 183u8,
                        224u8, 184u8, 153u8, 224u8, 185u8, 128u8, 224u8, 184u8, 161u8, 224u8,
                        184u8, 183u8, 224u8, 185u8, 136u8, 224u8, 184u8, 173u8, 224u8, 184u8,
                        167u8, 224u8, 184u8, 178u8, 224u8, 184u8, 153u8, 224u8, 184u8, 167u8,
                        224u8, 184u8, 177u8, 224u8, 184u8, 153u8, 224u8, 184u8, 153u8, 224u8,
                        184u8, 181u8, 224u8, 185u8, 137u8, 224u8, 184u8, 158u8, 224u8, 184u8,
                        163u8, 224u8, 184u8, 184u8, 224u8, 185u8, 136u8, 224u8, 184u8, 135u8,
                        224u8, 184u8, 153u8, 224u8, 184u8, 181u8, 224u8, 185u8, 137u8, 224u8,
                        184u8, 161u8, 224u8, 184u8, 176u8, 224u8, 184u8, 163u8, 224u8, 184u8,
                        183u8, 224u8, 184u8, 153u8, 224u8, 184u8, 153u8, 224u8, 184u8, 181u8,
                        224u8, 185u8, 137u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("ส\u{e31}ปดาห\u{e4c}")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 42u8, 0u8, 72u8, 0u8, 224u8, 184u8, 170u8,
                        224u8, 184u8, 177u8, 224u8, 184u8, 155u8, 224u8, 184u8, 148u8, 224u8,
                        184u8, 178u8, 224u8, 184u8, 171u8, 224u8, 185u8, 140u8, 224u8, 184u8,
                        151u8, 224u8, 184u8, 181u8, 224u8, 185u8, 136u8, 224u8, 185u8, 129u8,
                        224u8, 184u8, 165u8, 224u8, 185u8, 137u8, 224u8, 184u8, 167u8, 224u8,
                        184u8, 170u8, 224u8, 184u8, 177u8, 224u8, 184u8, 155u8, 224u8, 184u8,
                        148u8, 224u8, 184u8, 178u8, 224u8, 184u8, 171u8, 224u8, 185u8, 140u8,
                        224u8, 184u8, 153u8, 224u8, 184u8, 181u8, 224u8, 185u8, 137u8, 224u8,
                        184u8, 170u8, 224u8, 184u8, 177u8, 224u8, 184u8, 155u8, 224u8, 184u8,
                        148u8, 224u8, 184u8, 178u8, 224u8, 184u8, 171u8, 224u8, 185u8, 140u8,
                        224u8, 184u8, 171u8, 224u8, 184u8, 153u8, 224u8, 185u8, 137u8, 224u8,
                        184u8, 178u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("เด\u{e37}อน")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 36u8, 0u8, 60u8, 0u8, 224u8, 185u8, 128u8,
                        224u8, 184u8, 148u8, 224u8, 184u8, 183u8, 224u8, 184u8, 173u8, 224u8,
                        184u8, 153u8, 224u8, 184u8, 151u8, 224u8, 184u8, 181u8, 224u8, 185u8,
                        136u8, 224u8, 185u8, 129u8, 224u8, 184u8, 165u8, 224u8, 185u8, 137u8,
                        224u8, 184u8, 167u8, 224u8, 185u8, 128u8, 224u8, 184u8, 148u8, 224u8,
                        184u8, 183u8, 224u8, 184u8, 173u8, 224u8, 184u8, 153u8, 224u8, 184u8,
                        153u8, 224u8, 184u8, 181u8, 224u8, 185u8, 137u8, 224u8, 185u8, 128u8,
                        224u8, 184u8, 148u8, 224u8, 184u8, 183u8, 224u8, 184u8, 173u8, 224u8,
                        184u8, 153u8, 224u8, 184u8, 171u8, 224u8, 184u8, 153u8, 224u8, 185u8,
                        137u8, 224u8, 184u8, 178u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("ไตรมาส")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 39u8, 0u8, 66u8, 0u8, 224u8, 185u8, 132u8,
                        224u8, 184u8, 149u8, 224u8, 184u8, 163u8, 224u8, 184u8, 161u8, 224u8,
                        184u8, 178u8, 224u8, 184u8, 170u8, 224u8, 184u8, 151u8, 224u8, 184u8,
                        181u8, 224u8, 185u8, 136u8, 224u8, 185u8, 129u8, 224u8, 184u8, 165u8,
                        224u8, 185u8, 137u8, 224u8, 184u8, 167u8, 224u8, 185u8, 132u8, 224u8,
                        184u8, 149u8, 224u8, 184u8, 163u8, 224u8, 184u8, 161u8, 224u8, 184u8,
                        178u8, 224u8, 184u8, 170u8, 224u8, 184u8, 153u8, 224u8, 184u8, 181u8,
                        224u8, 185u8, 137u8, 224u8, 185u8, 132u8, 224u8, 184u8, 149u8, 224u8,
                        184u8, 163u8, 224u8, 184u8, 161u8, 224u8, 184u8, 178u8, 224u8, 184u8,
                        170u8, 224u8, 184u8, 171u8, 224u8, 184u8, 153u8, 224u8, 185u8, 137u8,
                        224u8, 184u8, 178u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("ป\u{e35}")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 27u8, 0u8, 42u8, 0u8, 224u8, 184u8, 155u8,
                        224u8, 184u8, 181u8, 224u8, 184u8, 151u8, 224u8, 184u8, 181u8, 224u8,
                        185u8, 136u8, 224u8, 185u8, 129u8, 224u8, 184u8, 165u8, 224u8, 185u8,
                        137u8, 224u8, 184u8, 167u8, 224u8, 184u8, 155u8, 224u8, 184u8, 181u8,
                        224u8, 184u8, 153u8, 224u8, 184u8, 181u8, 224u8, 185u8, 137u8, 224u8,
                        184u8, 155u8, 224u8, 184u8, 181u8, 224u8, 184u8, 171u8, 224u8, 184u8,
                        153u8, 224u8, 185u8, 137u8, 224u8, 184u8, 178u8,
                    ])
                },
            )
        },
    },
]);
static TR: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternsV1([
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("saniye")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 197u8, 159u8, 105u8, 109u8, 100u8, 105u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("dakika")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8, 117u8, 32u8, 100u8, 97u8, 107u8, 105u8,
                        107u8, 97u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("saat")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 98u8, 117u8, 32u8, 115u8, 97u8, 97u8, 116u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("gün")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[254u8, 255u8, 0u8, 1u8, 2u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 12u8, 0u8, 16u8, 0u8, 22u8, 0u8, 28u8, 0u8,
                        101u8, 118u8, 118u8, 101u8, 108u8, 115u8, 105u8, 32u8, 103u8, 195u8, 188u8,
                        110u8, 100u8, 195u8, 188u8, 110u8, 98u8, 117u8, 103u8, 195u8, 188u8, 110u8,
                        121u8, 97u8, 114u8, 196u8, 177u8, 110u8, 195u8, 182u8, 98u8, 195u8, 188u8,
                        114u8, 32u8, 103u8, 195u8, 188u8, 110u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("hafta")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 12u8, 0u8, 20u8, 0u8, 103u8, 101u8, 195u8,
                        167u8, 101u8, 110u8, 32u8, 104u8, 97u8, 102u8, 116u8, 97u8, 98u8, 117u8,
                        32u8, 104u8, 97u8, 102u8, 116u8, 97u8, 103u8, 101u8, 108u8, 101u8, 99u8,
                        101u8, 107u8, 32u8, 104u8, 97u8, 102u8, 116u8, 97u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("ay")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 9u8, 0u8, 14u8, 0u8, 103u8, 101u8, 195u8,
                        167u8, 101u8, 110u8, 32u8, 97u8, 121u8, 98u8, 117u8, 32u8, 97u8, 121u8,
                        103u8, 101u8, 108u8, 101u8, 99u8, 101u8, 107u8, 32u8, 97u8, 121u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("çeyrek")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 14u8, 0u8, 24u8, 0u8, 103u8, 101u8, 195u8,
                        167u8, 101u8, 110u8, 32u8, 195u8, 167u8, 101u8, 121u8, 114u8, 101u8, 107u8,
                        98u8, 117u8, 32u8, 195u8, 167u8, 101u8, 121u8, 114u8, 101u8, 107u8, 103u8,
                        101u8, 108u8, 101u8, 99u8, 101u8, 107u8, 32u8, 195u8, 167u8, 101u8, 121u8,
                        114u8, 101u8, 107u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("yıl")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 11u8, 0u8, 18u8, 0u8, 103u8, 101u8, 195u8,
                        167u8, 101u8, 110u8, 32u8, 121u8, 196u8, 177u8, 108u8, 98u8, 117u8, 32u8,
                        121u8, 196u8, 177u8, 108u8, 103u8, 101u8, 108u8, 101u8, 99u8, 101u8, 107u8,
                        32u8, 121u8, 196u8, 177u8, 108u8,
                    ])
                },
            )
        },
    },
]);
static UND: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternsV1([
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("Second")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 110u8, 111u8, 119u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("Minute")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 104u8, 105u8, 115u8, 32u8, 109u8,
                        105u8, 110u8, 117u8, 116u8, 101u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("Hour")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[0u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 104u8, 105u8, 115u8, 32u8, 104u8,
                        111u8, 117u8, 114u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("Day")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 9u8, 0u8, 14u8, 0u8, 121u8, 101u8, 115u8,
                        116u8, 101u8, 114u8, 100u8, 97u8, 121u8, 116u8, 111u8, 100u8, 97u8, 121u8,
                        116u8, 111u8, 109u8, 111u8, 114u8, 114u8, 111u8, 119u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("Week")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 9u8, 0u8, 18u8, 0u8, 108u8, 97u8, 115u8,
                        116u8, 32u8, 119u8, 101u8, 101u8, 107u8, 116u8, 104u8, 105u8, 115u8, 32u8,
                        119u8, 101u8, 101u8, 107u8, 110u8, 101u8, 120u8, 116u8, 32u8, 119u8, 101u8,
                        101u8, 107u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("Month")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 10u8, 0u8, 20u8, 0u8, 108u8, 97u8, 115u8,
                        116u8, 32u8, 109u8, 111u8, 110u8, 116u8, 104u8, 116u8, 104u8, 105u8, 115u8,
                        32u8, 109u8, 111u8, 110u8, 116u8, 104u8, 110u8, 101u8, 120u8, 116u8, 32u8,
                        109u8, 111u8, 110u8, 116u8, 104u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("Quarter")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 12u8, 0u8, 24u8, 0u8, 108u8, 97u8, 115u8,
                        116u8, 32u8, 113u8, 117u8, 97u8, 114u8, 116u8, 101u8, 114u8, 116u8, 104u8,
                        105u8, 115u8, 32u8, 113u8, 117u8, 97u8, 114u8, 116u8, 101u8, 114u8, 110u8,
                        101u8, 120u8, 116u8, 32u8, 113u8, 117u8, 97u8, 114u8, 116u8, 101u8, 114u8,
                    ])
                },
            )
        },
    },
    ::icu_relativetime::provider::RelativeTimePattern {
        display_name: Some(alloc::borrow::Cow::Borrowed("Year")),
        relatives: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap::from_parts_unchecked(
                unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 9u8, 0u8, 18u8, 0u8, 108u8, 97u8, 115u8,
                        116u8, 32u8, 121u8, 101u8, 97u8, 114u8, 116u8, 104u8, 105u8, 115u8, 32u8,
                        121u8, 101u8, 97u8, 114u8, 110u8, 101u8, 120u8, 116u8, 32u8, 121u8, 101u8,
                        97u8, 114u8,
                    ])
                },
            )
        },
    },
]);
