// @generated
#![cfg(feature = "icu_compactdecimal")]
type DataStruct = < :: icu_compactdecimal :: provider :: ShortCompactDecimalFormatDataV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_store_unchecked(&[
        ("ar", AR_AR_EG_AR_EG_U_NU_LATN_AR_U_NU_LATN),
        ("ar-EG", AR_AR_EG_AR_EG_U_NU_LATN_AR_U_NU_LATN),
        ("ar-EG-u-nu-latn", AR_AR_EG_AR_EG_U_NU_LATN_AR_U_NU_LATN),
        ("ar-u-nu-latn", AR_AR_EG_AR_EG_U_NU_LATN_AR_U_NU_LATN),
        ("bn", BN_BN_U_NU_LATN),
        ("bn-u-nu-latn", BN_BN_U_NU_LATN),
        ("ccp", CCP_CCP_U_NU_LATN_UND),
        ("ccp-u-nu-latn", CCP_CCP_U_NU_LATN_UND),
        ("en", EN_EN_001_EN_ZA_FIL_TH_TH_U_NU_THAI),
        ("en-001", EN_EN_001_EN_ZA_FIL_TH_TH_U_NU_THAI),
        ("en-ZA", EN_EN_001_EN_ZA_FIL_TH_TH_U_NU_THAI),
        ("es", ES),
        ("es-AR", ES_AR),
        ("fil", EN_EN_001_EN_ZA_FIL_TH_TH_U_NU_THAI),
        ("fr", FR),
        ("ja", JA),
        ("ru", RU),
        ("sr", SR_SR_CYRL),
        ("sr-Cyrl", SR_SR_CYRL),
        ("sr-Latn", SR_LATN),
        ("th", EN_EN_001_EN_ZA_FIL_TH_TH_U_NU_THAI),
        ("th-u-nu-thai", EN_EN_001_EN_ZA_FIL_TH_TH_U_NU_THAI),
        ("tr", TR),
        ("und", CCP_CCP_U_NU_LATN_UND),
    ]);
static AR_AR_EG_AR_EG_U_NU_LATN_AR_U_NU_LATN: &DataStruct =
    &::icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
        patterns: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap2d::from_parts_unchecked(
                unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8,
                    ])
                },
                unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8,
                        0u8, 6u8, 0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 9u8, 0u8,
                        0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 11u8, 0u8, 0u8, 0u8, 12u8, 0u8, 0u8, 0u8,
                        13u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8, 15u8, 0u8, 0u8, 0u8,
                    ])
                },
                unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        5u8, 5u8, 3u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        15u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 4u8, 0u8, 16u8, 0u8, 26u8, 0u8,
                        36u8, 0u8, 46u8, 0u8, 60u8, 0u8, 74u8, 0u8, 88u8, 0u8, 102u8, 0u8, 116u8,
                        0u8, 130u8, 0u8, 146u8, 0u8, 162u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8,
                        194u8, 160u8, 216u8, 162u8, 217u8, 132u8, 216u8, 167u8, 217u8, 129u8, 0u8,
                        3u8, 194u8, 160u8, 216u8, 163u8, 217u8, 132u8, 217u8, 129u8, 0u8, 3u8,
                        194u8, 160u8, 216u8, 163u8, 217u8, 132u8, 217u8, 129u8, 0u8, 3u8, 194u8,
                        160u8, 216u8, 163u8, 217u8, 132u8, 217u8, 129u8, 0u8, 6u8, 194u8, 160u8,
                        217u8, 133u8, 217u8, 132u8, 217u8, 138u8, 217u8, 136u8, 217u8, 134u8, 0u8,
                        6u8, 194u8, 160u8, 217u8, 133u8, 217u8, 132u8, 217u8, 138u8, 217u8, 136u8,
                        217u8, 134u8, 0u8, 6u8, 194u8, 160u8, 217u8, 133u8, 217u8, 132u8, 217u8,
                        138u8, 217u8, 136u8, 217u8, 134u8, 0u8, 9u8, 194u8, 160u8, 217u8, 133u8,
                        217u8, 132u8, 217u8, 138u8, 216u8, 167u8, 216u8, 177u8, 0u8, 9u8, 194u8,
                        160u8, 217u8, 133u8, 217u8, 132u8, 217u8, 138u8, 216u8, 167u8, 216u8,
                        177u8, 0u8, 9u8, 194u8, 160u8, 217u8, 133u8, 217u8, 132u8, 217u8, 138u8,
                        216u8, 167u8, 216u8, 177u8, 0u8, 12u8, 194u8, 160u8, 216u8, 170u8, 216u8,
                        177u8, 217u8, 132u8, 217u8, 138u8, 217u8, 136u8, 217u8, 134u8, 0u8, 12u8,
                        194u8, 160u8, 216u8, 170u8, 216u8, 177u8, 217u8, 132u8, 217u8, 138u8,
                        217u8, 136u8, 217u8, 134u8, 0u8, 12u8, 194u8, 160u8, 216u8, 170u8, 216u8,
                        177u8, 217u8, 132u8, 217u8, 138u8, 217u8, 136u8, 217u8, 134u8,
                    ])
                },
            )
        },
    };
static BN_BN_U_NU_LATN: &DataStruct =
    &::icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
        patterns: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap2d::from_parts_unchecked(
                unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8,
                    ])
                },
                unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8,
                        0u8, 5u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 8u8, 0u8,
                        0u8, 0u8, 9u8, 0u8, 0u8, 0u8, 11u8, 0u8, 0u8, 0u8, 12u8, 0u8, 0u8, 0u8,
                        13u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8, 15u8, 0u8, 0u8, 0u8,
                    ])
                },
                unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 1u8, 5u8, 5u8, 5u8, 5u8, 5u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        15u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 4u8, 0u8, 14u8, 0u8, 24u8, 0u8,
                        34u8, 0u8, 44u8, 0u8, 54u8, 0u8, 64u8, 0u8, 74u8, 0u8, 92u8, 0u8, 108u8,
                        0u8, 116u8, 0u8, 134u8, 0u8, 152u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8,
                        194u8, 160u8, 224u8, 166u8, 185u8, 224u8, 166u8, 190u8, 0u8, 3u8, 194u8,
                        160u8, 224u8, 166u8, 185u8, 224u8, 166u8, 190u8, 0u8, 5u8, 194u8, 160u8,
                        224u8, 166u8, 178u8, 224u8, 166u8, 190u8, 0u8, 5u8, 194u8, 160u8, 224u8,
                        166u8, 178u8, 224u8, 166u8, 190u8, 0u8, 7u8, 194u8, 160u8, 224u8, 166u8,
                        149u8, 224u8, 167u8, 139u8, 0u8, 7u8, 194u8, 160u8, 224u8, 166u8, 149u8,
                        224u8, 167u8, 139u8, 0u8, 7u8, 194u8, 160u8, 224u8, 166u8, 149u8, 224u8,
                        167u8, 139u8, 0u8, 9u8, 194u8, 160u8, 224u8, 166u8, 182u8, 224u8, 166u8,
                        164u8, 194u8, 160u8, 224u8, 166u8, 149u8, 224u8, 167u8, 139u8, 0u8, 9u8,
                        224u8, 166u8, 182u8, 224u8, 166u8, 164u8, 194u8, 160u8, 224u8, 166u8,
                        149u8, 224u8, 167u8, 139u8, 0u8, 9u8, 224u8, 166u8, 149u8, 224u8, 167u8,
                        139u8, 0u8, 12u8, 194u8, 160u8, 224u8, 166u8, 178u8, 224u8, 166u8, 190u8,
                        46u8, 224u8, 166u8, 149u8, 224u8, 167u8, 139u8, 46u8, 0u8, 12u8, 194u8,
                        160u8, 224u8, 166u8, 178u8, 224u8, 166u8, 190u8, 46u8, 224u8, 166u8, 149u8,
                        224u8, 167u8, 139u8, 46u8, 0u8, 12u8, 194u8, 160u8, 224u8, 166u8, 178u8,
                        224u8, 166u8, 190u8, 46u8, 224u8, 166u8, 149u8, 224u8, 167u8, 139u8, 46u8,
                    ])
                },
            )
        },
    };
static CCP_CCP_U_NU_LATN_UND: &DataStruct =
    &::icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
        patterns: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap2d::from_parts_unchecked(
                unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8,
                    ])
                },
                unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8,
                        0u8, 5u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 8u8, 0u8,
                        0u8, 0u8, 9u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 11u8, 0u8, 0u8, 0u8,
                        12u8, 0u8, 0u8, 0u8, 13u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8,
                    ])
                },
                unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        14u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 4u8, 0u8, 7u8, 0u8, 10u8, 0u8,
                        13u8, 0u8, 16u8, 0u8, 19u8, 0u8, 22u8, 0u8, 25u8, 0u8, 28u8, 0u8, 31u8,
                        0u8, 34u8, 0u8, 37u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 75u8, 0u8, 3u8,
                        75u8, 0u8, 3u8, 75u8, 0u8, 6u8, 77u8, 0u8, 6u8, 77u8, 0u8, 6u8, 77u8, 0u8,
                        9u8, 71u8, 0u8, 9u8, 71u8, 0u8, 9u8, 71u8, 0u8, 12u8, 84u8, 0u8, 12u8,
                        84u8, 0u8, 12u8, 84u8,
                    ])
                },
            )
        },
    };
static EN_EN_001_EN_ZA_FIL_TH_TH_U_NU_THAI: &DataStruct =
    &::icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
        patterns: unsafe {
            #[allow(unused_unsafe)]
            ::zerovec::ZeroMap2d::from_parts_unchecked(
                unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8,
                    ])
                },
                unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8,
                        0u8, 5u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 8u8, 0u8,
                        0u8, 0u8, 9u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 11u8, 0u8, 0u8, 0u8,
                        12u8, 0u8, 0u8, 0u8, 13u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8,
                    ])
                },
                unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        14u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 4u8, 0u8, 7u8, 0u8, 10u8, 0u8,
                        13u8, 0u8, 16u8, 0u8, 19u8, 0u8, 22u8, 0u8, 25u8, 0u8, 28u8, 0u8, 31u8,
                        0u8, 34u8, 0u8, 37u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 75u8, 0u8, 3u8,
                        75u8, 0u8, 3u8, 75u8, 0u8, 6u8, 77u8, 0u8, 6u8, 77u8, 0u8, 6u8, 77u8, 0u8,
                        9u8, 66u8, 0u8, 9u8, 66u8, 0u8, 9u8, 66u8, 0u8, 12u8, 84u8, 0u8, 12u8,
                        84u8, 0u8, 12u8, 84u8,
                    ])
                },
            )
        },
    };
static ES: &DataStruct = &::icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
    patterns: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap2d::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8,
                ])
            },
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8,
                    5u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8,
                    9u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 11u8, 0u8, 0u8, 0u8, 12u8, 0u8, 0u8,
                    0u8, 13u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8,
                ])
            },
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8,
                ])
            },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    14u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 4u8, 0u8, 11u8, 0u8, 18u8, 0u8, 25u8,
                    0u8, 30u8, 0u8, 35u8, 0u8, 40u8, 0u8, 45u8, 0u8, 55u8, 0u8, 65u8, 0u8, 70u8,
                    0u8, 75u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 194u8, 160u8, 109u8, 105u8,
                    108u8, 0u8, 3u8, 194u8, 160u8, 109u8, 105u8, 108u8, 0u8, 3u8, 194u8, 160u8,
                    109u8, 105u8, 108u8, 0u8, 6u8, 194u8, 160u8, 77u8, 0u8, 6u8, 194u8, 160u8,
                    77u8, 0u8, 6u8, 194u8, 160u8, 77u8, 0u8, 6u8, 194u8, 160u8, 77u8, 0u8, 9u8,
                    194u8, 160u8, 109u8, 105u8, 108u8, 194u8, 160u8, 77u8, 0u8, 9u8, 194u8, 160u8,
                    109u8, 105u8, 108u8, 194u8, 160u8, 77u8, 0u8, 12u8, 194u8, 160u8, 66u8, 0u8,
                    12u8, 194u8, 160u8, 66u8, 0u8, 12u8, 194u8, 160u8, 66u8,
                ])
            },
        )
    },
};
static ES_AR: &DataStruct = &::icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
    patterns: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap2d::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8,
                ])
            },
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8,
                    5u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8,
                    9u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 11u8, 0u8, 0u8, 0u8, 12u8, 0u8, 0u8,
                    0u8, 13u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8,
                ])
            },
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8,
                ])
            },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    14u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 4u8, 0u8, 9u8, 0u8, 14u8, 0u8, 19u8,
                    0u8, 24u8, 0u8, 29u8, 0u8, 34u8, 0u8, 39u8, 0u8, 49u8, 0u8, 59u8, 0u8, 64u8,
                    0u8, 69u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 194u8, 160u8, 75u8, 0u8, 3u8,
                    194u8, 160u8, 107u8, 0u8, 3u8, 194u8, 160u8, 107u8, 0u8, 6u8, 194u8, 160u8,
                    77u8, 0u8, 6u8, 194u8, 160u8, 77u8, 0u8, 6u8, 194u8, 160u8, 77u8, 0u8, 6u8,
                    194u8, 160u8, 77u8, 0u8, 9u8, 194u8, 160u8, 109u8, 105u8, 108u8, 194u8, 160u8,
                    77u8, 0u8, 9u8, 194u8, 160u8, 109u8, 105u8, 108u8, 194u8, 160u8, 77u8, 0u8,
                    12u8, 194u8, 160u8, 66u8, 0u8, 12u8, 194u8, 160u8, 66u8, 0u8, 12u8, 194u8,
                    160u8, 66u8,
                ])
            },
        )
    },
};
static FR: &DataStruct = &::icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
    patterns: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap2d::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8,
                ])
            },
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8,
                    5u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8,
                    9u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 11u8, 0u8, 0u8, 0u8, 12u8, 0u8, 0u8,
                    0u8, 13u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8,
                ])
            },
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8,
                ])
            },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    14u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 4u8, 0u8, 9u8, 0u8, 14u8, 0u8, 19u8,
                    0u8, 24u8, 0u8, 29u8, 0u8, 34u8, 0u8, 40u8, 0u8, 46u8, 0u8, 52u8, 0u8, 58u8,
                    0u8, 64u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 194u8, 160u8, 107u8, 0u8, 3u8,
                    194u8, 160u8, 107u8, 0u8, 3u8, 194u8, 160u8, 107u8, 0u8, 6u8, 194u8, 160u8,
                    77u8, 0u8, 6u8, 194u8, 160u8, 77u8, 0u8, 6u8, 194u8, 160u8, 77u8, 0u8, 9u8,
                    194u8, 160u8, 77u8, 100u8, 0u8, 9u8, 194u8, 160u8, 77u8, 100u8, 0u8, 9u8,
                    194u8, 160u8, 77u8, 100u8, 0u8, 12u8, 194u8, 160u8, 66u8, 110u8, 0u8, 12u8,
                    194u8, 160u8, 66u8, 110u8, 0u8, 12u8, 194u8, 160u8, 66u8, 110u8,
                ])
            },
        )
    },
};
static JA: &DataStruct = &::icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
    patterns: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap2d::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8,
                ])
            },
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8,
                    5u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8,
                    9u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 11u8, 0u8, 0u8, 0u8, 12u8, 0u8, 0u8,
                    0u8, 13u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8,
                ])
            },
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8,
                ])
            },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    14u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 4u8, 0u8, 6u8, 0u8, 11u8, 0u8, 16u8,
                    0u8, 21u8, 0u8, 26u8, 0u8, 31u8, 0u8, 36u8, 0u8, 41u8, 0u8, 46u8, 0u8, 51u8,
                    0u8, 56u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 4u8, 228u8, 184u8, 135u8,
                    0u8, 4u8, 228u8, 184u8, 135u8, 0u8, 4u8, 228u8, 184u8, 135u8, 0u8, 4u8, 228u8,
                    184u8, 135u8, 0u8, 8u8, 229u8, 132u8, 132u8, 0u8, 8u8, 229u8, 132u8, 132u8,
                    0u8, 8u8, 229u8, 132u8, 132u8, 0u8, 8u8, 229u8, 132u8, 132u8, 0u8, 12u8, 229u8,
                    133u8, 134u8, 0u8, 12u8, 229u8, 133u8, 134u8, 0u8, 12u8, 229u8, 133u8, 134u8,
                ])
            },
        )
    },
};
static RU: &DataStruct = &::icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
    patterns: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap2d::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8,
                ])
            },
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8,
                    5u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8,
                    9u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 11u8, 0u8, 0u8, 0u8, 12u8, 0u8, 0u8,
                    0u8, 13u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8,
                ])
            },
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8,
                ])
            },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    14u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 4u8, 0u8, 15u8, 0u8, 26u8, 0u8, 37u8,
                    0u8, 47u8, 0u8, 57u8, 0u8, 67u8, 0u8, 79u8, 0u8, 91u8, 0u8, 103u8, 0u8, 115u8,
                    0u8, 127u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 194u8, 160u8, 209u8, 130u8,
                    209u8, 139u8, 209u8, 129u8, 46u8, 0u8, 3u8, 194u8, 160u8, 209u8, 130u8, 209u8,
                    139u8, 209u8, 129u8, 46u8, 0u8, 3u8, 194u8, 160u8, 209u8, 130u8, 209u8, 139u8,
                    209u8, 129u8, 46u8, 0u8, 6u8, 194u8, 160u8, 208u8, 188u8, 208u8, 187u8, 208u8,
                    189u8, 0u8, 6u8, 194u8, 160u8, 208u8, 188u8, 208u8, 187u8, 208u8, 189u8, 0u8,
                    6u8, 194u8, 160u8, 208u8, 188u8, 208u8, 187u8, 208u8, 189u8, 0u8, 9u8, 194u8,
                    160u8, 208u8, 188u8, 208u8, 187u8, 209u8, 128u8, 208u8, 180u8, 0u8, 9u8, 194u8,
                    160u8, 208u8, 188u8, 208u8, 187u8, 209u8, 128u8, 208u8, 180u8, 0u8, 9u8, 194u8,
                    160u8, 208u8, 188u8, 208u8, 187u8, 209u8, 128u8, 208u8, 180u8, 0u8, 12u8,
                    194u8, 160u8, 209u8, 130u8, 209u8, 128u8, 208u8, 187u8, 208u8, 189u8, 0u8,
                    12u8, 194u8, 160u8, 209u8, 130u8, 209u8, 128u8, 208u8, 187u8, 208u8, 189u8,
                    0u8, 12u8, 194u8, 160u8, 209u8, 130u8, 209u8, 128u8, 208u8, 187u8, 208u8,
                    189u8,
                ])
            },
        )
    },
};
static SR_LATN: &DataStruct = &::icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
    patterns: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap2d::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8,
                ])
            },
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8,
                    5u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8,
                    9u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 11u8, 0u8, 0u8, 0u8, 12u8, 0u8, 0u8,
                    0u8, 13u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8,
                ])
            },
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8,
                ])
            },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    14u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 4u8, 0u8, 13u8, 0u8, 22u8, 0u8, 31u8,
                    0u8, 39u8, 0u8, 47u8, 0u8, 55u8, 0u8, 64u8, 0u8, 73u8, 0u8, 82u8, 0u8, 90u8,
                    0u8, 98u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 194u8, 160u8, 104u8, 105u8,
                    108u8, 106u8, 46u8, 0u8, 3u8, 194u8, 160u8, 104u8, 105u8, 108u8, 106u8, 46u8,
                    0u8, 3u8, 194u8, 160u8, 104u8, 105u8, 108u8, 106u8, 46u8, 0u8, 6u8, 194u8,
                    160u8, 109u8, 105u8, 108u8, 46u8, 0u8, 6u8, 194u8, 160u8, 109u8, 105u8, 108u8,
                    46u8, 0u8, 6u8, 194u8, 160u8, 109u8, 105u8, 108u8, 46u8, 0u8, 9u8, 194u8,
                    160u8, 109u8, 108u8, 114u8, 100u8, 46u8, 0u8, 9u8, 194u8, 160u8, 109u8, 108u8,
                    114u8, 100u8, 46u8, 0u8, 9u8, 194u8, 160u8, 109u8, 108u8, 114u8, 100u8, 46u8,
                    0u8, 12u8, 194u8, 160u8, 98u8, 105u8, 108u8, 46u8, 0u8, 12u8, 194u8, 160u8,
                    98u8, 105u8, 108u8, 46u8, 0u8, 12u8, 194u8, 160u8, 98u8, 105u8, 108u8, 46u8,
                ])
            },
        )
    },
};
static SR_SR_CYRL: &DataStruct = &::icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
    patterns: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap2d::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8,
                ])
            },
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8,
                    5u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8,
                    9u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 11u8, 0u8, 0u8, 0u8, 12u8, 0u8, 0u8,
                    0u8, 13u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8,
                ])
            },
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8,
                ])
            },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    14u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 4u8, 0u8, 15u8, 0u8, 26u8, 0u8, 37u8,
                    0u8, 48u8, 0u8, 59u8, 0u8, 70u8, 0u8, 83u8, 0u8, 96u8, 0u8, 109u8, 0u8, 120u8,
                    0u8, 131u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 194u8, 160u8, 209u8, 133u8,
                    208u8, 184u8, 209u8, 153u8, 46u8, 0u8, 3u8, 194u8, 160u8, 209u8, 133u8, 208u8,
                    184u8, 209u8, 153u8, 46u8, 0u8, 3u8, 194u8, 160u8, 209u8, 133u8, 208u8, 184u8,
                    209u8, 153u8, 46u8, 0u8, 6u8, 194u8, 160u8, 208u8, 188u8, 208u8, 184u8, 208u8,
                    187u8, 46u8, 0u8, 6u8, 194u8, 160u8, 208u8, 188u8, 208u8, 184u8, 208u8, 187u8,
                    46u8, 0u8, 6u8, 194u8, 160u8, 208u8, 188u8, 208u8, 184u8, 208u8, 187u8, 46u8,
                    0u8, 9u8, 194u8, 160u8, 208u8, 188u8, 208u8, 187u8, 209u8, 128u8, 208u8, 180u8,
                    46u8, 0u8, 9u8, 194u8, 160u8, 208u8, 188u8, 208u8, 187u8, 209u8, 128u8, 208u8,
                    180u8, 46u8, 0u8, 9u8, 194u8, 160u8, 208u8, 188u8, 208u8, 187u8, 209u8, 128u8,
                    208u8, 180u8, 46u8, 0u8, 12u8, 194u8, 160u8, 208u8, 177u8, 208u8, 184u8, 208u8,
                    187u8, 46u8, 0u8, 12u8, 194u8, 160u8, 208u8, 177u8, 208u8, 184u8, 208u8, 187u8,
                    46u8, 0u8, 12u8, 194u8, 160u8, 208u8, 177u8, 208u8, 184u8, 208u8, 187u8, 46u8,
                ])
            },
        )
    },
};
static TR: &DataStruct = &::icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
    patterns: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap2d::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8,
                ])
            },
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8,
                    5u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8,
                    9u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 11u8, 0u8, 0u8, 0u8, 12u8, 0u8, 0u8,
                    0u8, 13u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8,
                ])
            },
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8,
                ])
            },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    14u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 4u8, 0u8, 9u8, 0u8, 14u8, 0u8, 19u8,
                    0u8, 25u8, 0u8, 31u8, 0u8, 37u8, 0u8, 43u8, 0u8, 49u8, 0u8, 55u8, 0u8, 61u8,
                    0u8, 67u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 194u8, 160u8, 66u8, 0u8, 3u8,
                    194u8, 160u8, 66u8, 0u8, 3u8, 194u8, 160u8, 66u8, 0u8, 6u8, 194u8, 160u8, 77u8,
                    110u8, 0u8, 6u8, 194u8, 160u8, 77u8, 110u8, 0u8, 6u8, 194u8, 160u8, 77u8,
                    110u8, 0u8, 9u8, 194u8, 160u8, 77u8, 114u8, 0u8, 9u8, 194u8, 160u8, 77u8,
                    114u8, 0u8, 9u8, 194u8, 160u8, 77u8, 114u8, 0u8, 12u8, 194u8, 160u8, 84u8,
                    110u8, 0u8, 12u8, 194u8, 160u8, 84u8, 110u8, 0u8, 12u8, 194u8, 160u8, 84u8,
                    110u8,
                ])
            },
        )
    },
};
