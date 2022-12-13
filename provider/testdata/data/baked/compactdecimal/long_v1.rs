// @generated
#![cfg(feature = "icu_compactdecimal")]
type DataStruct = < :: icu_compactdecimal :: provider :: LongCompactDecimalFormatDataV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
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
        ("th", TH_TH_U_NU_THAI),
        ("th-u-nu-thai", TH_TH_U_NU_THAI),
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
                        0u8, 6u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 11u8,
                        0u8, 0u8, 0u8, 12u8, 0u8, 0u8, 0u8, 13u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8,
                        0u8, 15u8, 0u8, 0u8, 0u8, 16u8, 0u8, 0u8, 0u8, 17u8, 0u8, 0u8, 0u8,
                    ])
                },
                unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        5u8, 5u8, 3u8, 5u8, 5u8, 5u8, 3u8, 5u8, 3u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8,
                        5u8, 5u8,
                    ])
                },
                unsafe {
                    ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                        17u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 4u8, 0u8, 15u8, 0u8, 24u8, 0u8,
                        33u8, 0u8, 42u8, 0u8, 57u8, 0u8, 70u8, 0u8, 85u8, 0u8, 98u8, 0u8, 111u8,
                        0u8, 124u8, 0u8, 137u8, 0u8, 150u8, 0u8, 165u8, 0u8, 180u8, 0u8, 0u8, 0u8,
                        0u8, 0u8, 0u8, 3u8, 32u8, 216u8, 162u8, 217u8, 132u8, 216u8, 167u8, 217u8,
                        129u8, 0u8, 3u8, 32u8, 216u8, 163u8, 217u8, 132u8, 217u8, 129u8, 0u8, 3u8,
                        32u8, 216u8, 163u8, 217u8, 132u8, 217u8, 129u8, 0u8, 3u8, 32u8, 216u8,
                        163u8, 217u8, 132u8, 217u8, 129u8, 0u8, 6u8, 32u8, 217u8, 133u8, 217u8,
                        132u8, 216u8, 167u8, 217u8, 138u8, 217u8, 138u8, 217u8, 134u8, 0u8, 6u8,
                        32u8, 217u8, 133u8, 217u8, 132u8, 217u8, 138u8, 217u8, 136u8, 217u8, 134u8,
                        0u8, 6u8, 32u8, 217u8, 133u8, 217u8, 132u8, 216u8, 167u8, 217u8, 138u8,
                        217u8, 138u8, 217u8, 134u8, 0u8, 6u8, 32u8, 217u8, 133u8, 217u8, 132u8,
                        217u8, 138u8, 217u8, 136u8, 217u8, 134u8, 0u8, 6u8, 32u8, 217u8, 133u8,
                        217u8, 132u8, 217u8, 138u8, 217u8, 136u8, 217u8, 134u8, 0u8, 9u8, 32u8,
                        217u8, 133u8, 217u8, 132u8, 217u8, 138u8, 216u8, 167u8, 216u8, 177u8, 0u8,
                        9u8, 32u8, 217u8, 133u8, 217u8, 132u8, 217u8, 138u8, 216u8, 167u8, 216u8,
                        177u8, 0u8, 9u8, 32u8, 217u8, 133u8, 217u8, 132u8, 217u8, 138u8, 216u8,
                        167u8, 216u8, 177u8, 0u8, 12u8, 32u8, 216u8, 170u8, 216u8, 177u8, 217u8,
                        132u8, 217u8, 138u8, 217u8, 136u8, 217u8, 134u8, 0u8, 12u8, 32u8, 216u8,
                        170u8, 216u8, 177u8, 217u8, 132u8, 217u8, 138u8, 217u8, 136u8, 217u8,
                        134u8, 0u8, 12u8, 32u8, 216u8, 170u8, 216u8, 177u8, 217u8, 132u8, 217u8,
                        138u8, 217u8, 136u8, 217u8, 134u8,
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
                        14u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 4u8, 0u8, 22u8, 0u8, 40u8, 0u8,
                        52u8, 0u8, 64u8, 0u8, 79u8, 0u8, 94u8, 0u8, 109u8, 0u8, 124u8, 0u8, 139u8,
                        0u8, 164u8, 0u8, 189u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 32u8, 224u8,
                        166u8, 185u8, 224u8, 166u8, 190u8, 224u8, 166u8, 156u8, 224u8, 166u8,
                        190u8, 224u8, 166u8, 176u8, 0u8, 3u8, 32u8, 224u8, 166u8, 185u8, 224u8,
                        166u8, 190u8, 224u8, 166u8, 156u8, 224u8, 166u8, 190u8, 224u8, 166u8,
                        176u8, 0u8, 5u8, 32u8, 224u8, 166u8, 178u8, 224u8, 166u8, 190u8, 224u8,
                        166u8, 150u8, 0u8, 5u8, 32u8, 224u8, 166u8, 178u8, 224u8, 166u8, 190u8,
                        224u8, 166u8, 150u8, 0u8, 7u8, 32u8, 224u8, 166u8, 149u8, 224u8, 167u8,
                        139u8, 224u8, 166u8, 159u8, 224u8, 166u8, 191u8, 0u8, 7u8, 32u8, 224u8,
                        166u8, 149u8, 224u8, 167u8, 139u8, 224u8, 166u8, 159u8, 224u8, 166u8,
                        191u8, 0u8, 7u8, 32u8, 224u8, 166u8, 149u8, 224u8, 167u8, 139u8, 224u8,
                        166u8, 159u8, 224u8, 166u8, 191u8, 0u8, 7u8, 32u8, 224u8, 166u8, 149u8,
                        224u8, 167u8, 139u8, 224u8, 166u8, 159u8, 224u8, 166u8, 191u8, 0u8, 7u8,
                        32u8, 224u8, 166u8, 149u8, 224u8, 167u8, 139u8, 224u8, 166u8, 159u8, 224u8,
                        166u8, 191u8, 0u8, 12u8, 32u8, 224u8, 166u8, 178u8, 224u8, 166u8, 190u8,
                        224u8, 166u8, 150u8, 32u8, 224u8, 166u8, 149u8, 224u8, 167u8, 139u8, 224u8,
                        166u8, 159u8, 224u8, 166u8, 191u8, 0u8, 12u8, 32u8, 224u8, 166u8, 178u8,
                        224u8, 166u8, 190u8, 224u8, 166u8, 150u8, 32u8, 224u8, 166u8, 149u8, 224u8,
                        167u8, 139u8, 224u8, 166u8, 159u8, 224u8, 166u8, 191u8, 0u8, 12u8, 32u8,
                        224u8, 166u8, 178u8, 224u8, 166u8, 190u8, 224u8, 166u8, 150u8, 32u8, 224u8,
                        166u8, 149u8, 224u8, 167u8, 139u8, 224u8, 166u8, 159u8, 224u8, 166u8,
                        191u8,
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
static EN_EN_001_EN_ZA: &DataStruct =
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
                        14u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 4u8, 0u8, 15u8, 0u8, 26u8, 0u8,
                        37u8, 0u8, 47u8, 0u8, 57u8, 0u8, 67u8, 0u8, 77u8, 0u8, 87u8, 0u8, 97u8,
                        0u8, 108u8, 0u8, 119u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 32u8, 116u8,
                        104u8, 111u8, 117u8, 115u8, 97u8, 110u8, 100u8, 0u8, 3u8, 32u8, 116u8,
                        104u8, 111u8, 117u8, 115u8, 97u8, 110u8, 100u8, 0u8, 3u8, 32u8, 116u8,
                        104u8, 111u8, 117u8, 115u8, 97u8, 110u8, 100u8, 0u8, 6u8, 32u8, 109u8,
                        105u8, 108u8, 108u8, 105u8, 111u8, 110u8, 0u8, 6u8, 32u8, 109u8, 105u8,
                        108u8, 108u8, 105u8, 111u8, 110u8, 0u8, 6u8, 32u8, 109u8, 105u8, 108u8,
                        108u8, 105u8, 111u8, 110u8, 0u8, 9u8, 32u8, 98u8, 105u8, 108u8, 108u8,
                        105u8, 111u8, 110u8, 0u8, 9u8, 32u8, 98u8, 105u8, 108u8, 108u8, 105u8,
                        111u8, 110u8, 0u8, 9u8, 32u8, 98u8, 105u8, 108u8, 108u8, 105u8, 111u8,
                        110u8, 0u8, 12u8, 32u8, 116u8, 114u8, 105u8, 108u8, 108u8, 105u8, 111u8,
                        110u8, 0u8, 12u8, 32u8, 116u8, 114u8, 105u8, 108u8, 108u8, 105u8, 111u8,
                        110u8, 0u8, 12u8, 32u8, 116u8, 114u8, 105u8, 108u8, 108u8, 105u8, 111u8,
                        110u8,
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
                    5u8, 0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 9u8, 0u8, 0u8, 0u8,
                    10u8, 0u8, 0u8, 0u8, 11u8, 0u8, 0u8, 0u8, 12u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8,
                    0u8, 15u8, 0u8, 0u8, 0u8, 16u8, 0u8, 0u8, 0u8,
                ])
            },
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    5u8, 5u8, 5u8, 5u8, 5u8, 1u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 1u8, 5u8, 5u8, 5u8,
                ])
            },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    16u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 4u8, 0u8, 10u8, 0u8, 16u8, 0u8, 22u8,
                    0u8, 32u8, 0u8, 43u8, 0u8, 54u8, 0u8, 65u8, 0u8, 80u8, 0u8, 95u8, 0u8, 110u8,
                    0u8, 120u8, 0u8, 131u8, 0u8, 142u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 32u8,
                    109u8, 105u8, 108u8, 0u8, 3u8, 32u8, 109u8, 105u8, 108u8, 0u8, 3u8, 32u8,
                    109u8, 105u8, 108u8, 0u8, 6u8, 32u8, 109u8, 105u8, 108u8, 108u8, 195u8, 179u8,
                    110u8, 0u8, 6u8, 32u8, 109u8, 105u8, 108u8, 108u8, 111u8, 110u8, 101u8, 115u8,
                    0u8, 6u8, 32u8, 109u8, 105u8, 108u8, 108u8, 111u8, 110u8, 101u8, 115u8, 0u8,
                    6u8, 32u8, 109u8, 105u8, 108u8, 108u8, 111u8, 110u8, 101u8, 115u8, 0u8, 9u8,
                    32u8, 109u8, 105u8, 108u8, 32u8, 109u8, 105u8, 108u8, 108u8, 111u8, 110u8,
                    101u8, 115u8, 0u8, 9u8, 32u8, 109u8, 105u8, 108u8, 32u8, 109u8, 105u8, 108u8,
                    108u8, 111u8, 110u8, 101u8, 115u8, 0u8, 9u8, 32u8, 109u8, 105u8, 108u8, 32u8,
                    109u8, 105u8, 108u8, 108u8, 111u8, 110u8, 101u8, 115u8, 0u8, 12u8, 32u8, 98u8,
                    105u8, 108u8, 108u8, 195u8, 179u8, 110u8, 0u8, 12u8, 32u8, 98u8, 105u8, 108u8,
                    108u8, 111u8, 110u8, 101u8, 115u8, 0u8, 12u8, 32u8, 98u8, 105u8, 108u8, 108u8,
                    111u8, 110u8, 101u8, 115u8, 0u8, 12u8, 32u8, 98u8, 105u8, 108u8, 108u8, 111u8,
                    110u8, 101u8, 115u8,
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
                    5u8, 0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 9u8, 0u8, 0u8, 0u8,
                    10u8, 0u8, 0u8, 0u8, 11u8, 0u8, 0u8, 0u8, 12u8, 0u8, 0u8, 0u8, 13u8, 0u8, 0u8,
                    0u8, 14u8, 0u8, 0u8, 0u8, 15u8, 0u8, 0u8, 0u8,
                ])
            },
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    5u8, 5u8, 5u8, 5u8, 5u8, 1u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8,
                ])
            },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    15u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 4u8, 0u8, 10u8, 0u8, 16u8, 0u8, 22u8,
                    0u8, 32u8, 0u8, 43u8, 0u8, 54u8, 0u8, 65u8, 0u8, 80u8, 0u8, 95u8, 0u8, 110u8,
                    0u8, 120u8, 0u8, 131u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 32u8, 109u8, 105u8,
                    108u8, 0u8, 3u8, 32u8, 109u8, 105u8, 108u8, 0u8, 3u8, 32u8, 109u8, 105u8,
                    108u8, 0u8, 6u8, 32u8, 109u8, 105u8, 108u8, 108u8, 195u8, 179u8, 110u8, 0u8,
                    6u8, 32u8, 109u8, 105u8, 108u8, 108u8, 111u8, 110u8, 101u8, 115u8, 0u8, 6u8,
                    32u8, 109u8, 105u8, 108u8, 108u8, 111u8, 110u8, 101u8, 115u8, 0u8, 6u8, 32u8,
                    109u8, 105u8, 108u8, 108u8, 111u8, 110u8, 101u8, 115u8, 0u8, 9u8, 32u8, 109u8,
                    105u8, 108u8, 32u8, 109u8, 105u8, 108u8, 108u8, 111u8, 110u8, 101u8, 115u8,
                    0u8, 9u8, 32u8, 109u8, 105u8, 108u8, 32u8, 109u8, 105u8, 108u8, 108u8, 111u8,
                    110u8, 101u8, 115u8, 0u8, 9u8, 32u8, 109u8, 105u8, 108u8, 32u8, 109u8, 105u8,
                    108u8, 108u8, 111u8, 110u8, 101u8, 115u8, 0u8, 12u8, 32u8, 98u8, 105u8, 108u8,
                    108u8, 195u8, 179u8, 110u8, 0u8, 12u8, 32u8, 98u8, 105u8, 108u8, 108u8, 111u8,
                    110u8, 101u8, 115u8, 0u8, 12u8, 32u8, 98u8, 105u8, 108u8, 108u8, 111u8, 110u8,
                    101u8, 115u8,
                ])
            },
        )
    },
};
static FIL: &DataStruct = &::icu_compactdecimal::provider::CompactDecimalPatternDataV1 {
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
                    1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8,
                    8u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 12u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8,
                    0u8, 16u8, 0u8, 0u8, 0u8, 18u8, 0u8, 0u8, 0u8, 20u8, 0u8, 0u8, 0u8, 22u8, 0u8,
                    0u8, 0u8, 24u8, 0u8, 0u8, 0u8, 26u8, 0u8, 0u8, 0u8,
                ])
            },
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    5u8, 5u8, 1u8, 5u8, 1u8, 5u8, 1u8, 5u8, 1u8, 5u8, 1u8, 5u8, 1u8, 5u8, 1u8, 5u8,
                    1u8, 5u8, 1u8, 5u8, 1u8, 5u8, 1u8, 5u8, 1u8, 5u8,
                ])
            },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    26u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 4u8, 0u8, 11u8, 0u8, 21u8, 0u8, 28u8,
                    0u8, 38u8, 0u8, 45u8, 0u8, 55u8, 0u8, 64u8, 0u8, 76u8, 0u8, 85u8, 0u8, 97u8,
                    0u8, 106u8, 0u8, 118u8, 0u8, 127u8, 0u8, 139u8, 0u8, 148u8, 0u8, 160u8, 0u8,
                    169u8, 0u8, 181u8, 0u8, 191u8, 0u8, 204u8, 0u8, 214u8, 0u8, 227u8, 0u8, 237u8,
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 32u8, 108u8, 105u8, 98u8, 111u8, 0u8, 3u8,
                    32u8, 110u8, 97u8, 32u8, 108u8, 105u8, 98u8, 111u8, 0u8, 3u8, 32u8, 108u8,
                    105u8, 98u8, 111u8, 0u8, 3u8, 32u8, 110u8, 97u8, 32u8, 108u8, 105u8, 98u8,
                    111u8, 0u8, 3u8, 32u8, 108u8, 105u8, 98u8, 111u8, 0u8, 3u8, 32u8, 110u8, 97u8,
                    32u8, 108u8, 105u8, 98u8, 111u8, 0u8, 6u8, 32u8, 109u8, 105u8, 108u8, 121u8,
                    111u8, 110u8, 0u8, 6u8, 32u8, 110u8, 97u8, 32u8, 109u8, 105u8, 108u8, 121u8,
                    111u8, 110u8, 0u8, 6u8, 32u8, 109u8, 105u8, 108u8, 121u8, 111u8, 110u8, 0u8,
                    6u8, 32u8, 110u8, 97u8, 32u8, 109u8, 105u8, 108u8, 121u8, 111u8, 110u8, 0u8,
                    6u8, 32u8, 109u8, 105u8, 108u8, 121u8, 111u8, 110u8, 0u8, 6u8, 32u8, 110u8,
                    97u8, 32u8, 109u8, 105u8, 108u8, 121u8, 111u8, 110u8, 0u8, 9u8, 32u8, 98u8,
                    105u8, 108u8, 121u8, 111u8, 110u8, 0u8, 9u8, 32u8, 110u8, 97u8, 32u8, 98u8,
                    105u8, 108u8, 121u8, 111u8, 110u8, 0u8, 9u8, 32u8, 98u8, 105u8, 108u8, 121u8,
                    111u8, 110u8, 0u8, 9u8, 32u8, 110u8, 97u8, 32u8, 98u8, 105u8, 108u8, 121u8,
                    111u8, 110u8, 0u8, 9u8, 32u8, 98u8, 105u8, 108u8, 121u8, 111u8, 110u8, 0u8,
                    9u8, 32u8, 110u8, 97u8, 32u8, 98u8, 105u8, 108u8, 121u8, 111u8, 110u8, 0u8,
                    12u8, 32u8, 116u8, 114u8, 105u8, 108u8, 121u8, 111u8, 110u8, 0u8, 12u8, 32u8,
                    110u8, 97u8, 32u8, 116u8, 114u8, 105u8, 108u8, 121u8, 111u8, 110u8, 0u8, 12u8,
                    32u8, 116u8, 114u8, 105u8, 108u8, 121u8, 111u8, 110u8, 0u8, 12u8, 32u8, 110u8,
                    97u8, 32u8, 116u8, 114u8, 105u8, 108u8, 121u8, 111u8, 110u8, 0u8, 12u8, 32u8,
                    116u8, 114u8, 105u8, 108u8, 121u8, 111u8, 110u8, 0u8, 12u8, 32u8, 110u8, 97u8,
                    32u8, 116u8, 114u8, 105u8, 108u8, 121u8, 111u8, 110u8,
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
                    1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8,
                    7u8, 0u8, 0u8, 0u8, 9u8, 0u8, 0u8, 0u8, 11u8, 0u8, 0u8, 0u8, 13u8, 0u8, 0u8,
                    0u8, 15u8, 0u8, 0u8, 0u8, 17u8, 0u8, 0u8, 0u8, 19u8, 0u8, 0u8, 0u8, 21u8, 0u8,
                    0u8, 0u8, 23u8, 0u8, 0u8, 0u8, 25u8, 0u8, 0u8, 0u8,
                ])
            },
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    5u8, 5u8, 1u8, 5u8, 6u8, 5u8, 5u8, 1u8, 5u8, 1u8, 5u8, 1u8, 5u8, 1u8, 5u8, 1u8,
                    5u8, 1u8, 5u8, 1u8, 5u8, 1u8, 5u8, 1u8, 5u8,
                ])
            },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    25u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 4u8, 0u8, 14u8, 0u8, 22u8, 0u8, 29u8,
                    0u8, 37u8, 0u8, 45u8, 0u8, 55u8, 0u8, 66u8, 0u8, 76u8, 0u8, 87u8, 0u8, 97u8,
                    0u8, 108u8, 0u8, 119u8, 0u8, 131u8, 0u8, 142u8, 0u8, 154u8, 0u8, 165u8, 0u8,
                    177u8, 0u8, 187u8, 0u8, 198u8, 0u8, 208u8, 0u8, 219u8, 0u8, 229u8, 0u8, 0u8,
                    0u8, 0u8, 0u8, 0u8, 3u8, 32u8, 109u8, 105u8, 108u8, 108u8, 105u8, 101u8, 114u8,
                    0u8, 3u8, 32u8, 109u8, 105u8, 108u8, 108u8, 101u8, 255u8, 3u8, 109u8, 105u8,
                    108u8, 108u8, 101u8, 0u8, 3u8, 32u8, 109u8, 105u8, 108u8, 108u8, 101u8, 0u8,
                    3u8, 32u8, 109u8, 105u8, 108u8, 108u8, 101u8, 0u8, 6u8, 32u8, 109u8, 105u8,
                    108u8, 108u8, 105u8, 111u8, 110u8, 0u8, 6u8, 32u8, 109u8, 105u8, 108u8, 108u8,
                    105u8, 111u8, 110u8, 115u8, 0u8, 6u8, 32u8, 109u8, 105u8, 108u8, 108u8, 105u8,
                    111u8, 110u8, 0u8, 6u8, 32u8, 109u8, 105u8, 108u8, 108u8, 105u8, 111u8, 110u8,
                    115u8, 0u8, 6u8, 32u8, 109u8, 105u8, 108u8, 108u8, 105u8, 111u8, 110u8, 0u8,
                    6u8, 32u8, 109u8, 105u8, 108u8, 108u8, 105u8, 111u8, 110u8, 115u8, 0u8, 9u8,
                    32u8, 109u8, 105u8, 108u8, 108u8, 105u8, 97u8, 114u8, 100u8, 0u8, 9u8, 32u8,
                    109u8, 105u8, 108u8, 108u8, 105u8, 97u8, 114u8, 100u8, 115u8, 0u8, 9u8, 32u8,
                    109u8, 105u8, 108u8, 108u8, 105u8, 97u8, 114u8, 100u8, 0u8, 9u8, 32u8, 109u8,
                    105u8, 108u8, 108u8, 105u8, 97u8, 114u8, 100u8, 115u8, 0u8, 9u8, 32u8, 109u8,
                    105u8, 108u8, 108u8, 105u8, 97u8, 114u8, 100u8, 0u8, 9u8, 32u8, 109u8, 105u8,
                    108u8, 108u8, 105u8, 97u8, 114u8, 100u8, 115u8, 0u8, 12u8, 32u8, 98u8, 105u8,
                    108u8, 108u8, 105u8, 111u8, 110u8, 0u8, 12u8, 32u8, 98u8, 105u8, 108u8, 108u8,
                    105u8, 111u8, 110u8, 115u8, 0u8, 12u8, 32u8, 98u8, 105u8, 108u8, 108u8, 105u8,
                    111u8, 110u8, 0u8, 12u8, 32u8, 98u8, 105u8, 108u8, 108u8, 105u8, 111u8, 110u8,
                    115u8, 0u8, 12u8, 32u8, 98u8, 105u8, 108u8, 108u8, 105u8, 111u8, 110u8, 0u8,
                    12u8, 32u8, 98u8, 105u8, 108u8, 108u8, 105u8, 111u8, 110u8, 115u8,
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
                    1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8,
                    11u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8, 17u8, 0u8, 0u8, 0u8, 20u8, 0u8, 0u8,
                    0u8, 23u8, 0u8, 0u8, 0u8, 26u8, 0u8, 0u8, 0u8, 29u8, 0u8, 0u8, 0u8, 32u8, 0u8,
                    0u8, 0u8, 35u8, 0u8, 0u8, 0u8, 38u8, 0u8, 0u8, 0u8,
                ])
            },
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    5u8, 5u8, 1u8, 4u8, 5u8, 1u8, 4u8, 5u8, 1u8, 4u8, 5u8, 1u8, 4u8, 5u8, 1u8, 4u8,
                    5u8, 1u8, 4u8, 5u8, 1u8, 4u8, 5u8, 1u8, 4u8, 5u8, 1u8, 4u8, 5u8, 1u8, 4u8, 5u8,
                    1u8, 4u8, 5u8, 1u8, 4u8, 5u8,
                ])
            },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    38u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 4u8, 0u8, 19u8, 0u8, 32u8, 0u8, 47u8,
                    0u8, 62u8, 0u8, 75u8, 0u8, 90u8, 0u8, 105u8, 0u8, 118u8, 0u8, 133u8, 0u8,
                    150u8, 0u8, 171u8, 0u8, 190u8, 0u8, 207u8, 0u8, 228u8, 0u8, 247u8, 0u8, 8u8,
                    1u8, 29u8, 1u8, 48u8, 1u8, 67u8, 1u8, 90u8, 1u8, 111u8, 1u8, 130u8, 1u8, 153u8,
                    1u8, 174u8, 1u8, 193u8, 1u8, 216u8, 1u8, 237u8, 1u8, 0u8, 2u8, 23u8, 2u8, 44u8,
                    2u8, 63u8, 2u8, 86u8, 2u8, 107u8, 2u8, 126u8, 2u8, 149u8, 2u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 3u8, 32u8, 209u8, 130u8, 209u8, 139u8, 209u8, 129u8, 209u8, 143u8,
                    209u8, 135u8, 208u8, 176u8, 0u8, 3u8, 32u8, 209u8, 130u8, 209u8, 139u8, 209u8,
                    129u8, 209u8, 143u8, 209u8, 135u8, 0u8, 3u8, 32u8, 209u8, 130u8, 209u8, 139u8,
                    209u8, 129u8, 209u8, 143u8, 209u8, 135u8, 208u8, 184u8, 0u8, 3u8, 32u8, 209u8,
                    130u8, 209u8, 139u8, 209u8, 129u8, 209u8, 143u8, 209u8, 135u8, 208u8, 176u8,
                    0u8, 3u8, 32u8, 209u8, 130u8, 209u8, 139u8, 209u8, 129u8, 209u8, 143u8, 209u8,
                    135u8, 0u8, 3u8, 32u8, 209u8, 130u8, 209u8, 139u8, 209u8, 129u8, 209u8, 143u8,
                    209u8, 135u8, 208u8, 184u8, 0u8, 3u8, 32u8, 209u8, 130u8, 209u8, 139u8, 209u8,
                    129u8, 209u8, 143u8, 209u8, 135u8, 208u8, 176u8, 0u8, 3u8, 32u8, 209u8, 130u8,
                    209u8, 139u8, 209u8, 129u8, 209u8, 143u8, 209u8, 135u8, 0u8, 3u8, 32u8, 209u8,
                    130u8, 209u8, 139u8, 209u8, 129u8, 209u8, 143u8, 209u8, 135u8, 208u8, 184u8,
                    0u8, 6u8, 32u8, 208u8, 188u8, 208u8, 184u8, 208u8, 187u8, 208u8, 187u8, 208u8,
                    184u8, 208u8, 190u8, 208u8, 189u8, 0u8, 6u8, 32u8, 208u8, 188u8, 208u8, 184u8,
                    208u8, 187u8, 208u8, 187u8, 208u8, 184u8, 208u8, 190u8, 208u8, 189u8, 208u8,
                    190u8, 208u8, 178u8, 0u8, 6u8, 32u8, 208u8, 188u8, 208u8, 184u8, 208u8, 187u8,
                    208u8, 187u8, 208u8, 184u8, 208u8, 190u8, 208u8, 189u8, 208u8, 176u8, 0u8, 6u8,
                    32u8, 208u8, 188u8, 208u8, 184u8, 208u8, 187u8, 208u8, 187u8, 208u8, 184u8,
                    208u8, 190u8, 208u8, 189u8, 0u8, 6u8, 32u8, 208u8, 188u8, 208u8, 184u8, 208u8,
                    187u8, 208u8, 187u8, 208u8, 184u8, 208u8, 190u8, 208u8, 189u8, 208u8, 190u8,
                    208u8, 178u8, 0u8, 6u8, 32u8, 208u8, 188u8, 208u8, 184u8, 208u8, 187u8, 208u8,
                    187u8, 208u8, 184u8, 208u8, 190u8, 208u8, 189u8, 208u8, 176u8, 0u8, 6u8, 32u8,
                    208u8, 188u8, 208u8, 184u8, 208u8, 187u8, 208u8, 187u8, 208u8, 184u8, 208u8,
                    190u8, 208u8, 189u8, 0u8, 6u8, 32u8, 208u8, 188u8, 208u8, 184u8, 208u8, 187u8,
                    208u8, 187u8, 208u8, 184u8, 208u8, 190u8, 208u8, 189u8, 208u8, 190u8, 208u8,
                    178u8, 0u8, 6u8, 32u8, 208u8, 188u8, 208u8, 184u8, 208u8, 187u8, 208u8, 187u8,
                    208u8, 184u8, 208u8, 190u8, 208u8, 189u8, 208u8, 176u8, 0u8, 9u8, 32u8, 208u8,
                    188u8, 208u8, 184u8, 208u8, 187u8, 208u8, 187u8, 208u8, 184u8, 208u8, 176u8,
                    209u8, 128u8, 208u8, 180u8, 0u8, 9u8, 32u8, 208u8, 188u8, 208u8, 184u8, 208u8,
                    187u8, 208u8, 187u8, 208u8, 184u8, 208u8, 176u8, 209u8, 128u8, 208u8, 180u8,
                    208u8, 190u8, 208u8, 178u8, 0u8, 9u8, 32u8, 208u8, 188u8, 208u8, 184u8, 208u8,
                    187u8, 208u8, 187u8, 208u8, 184u8, 208u8, 176u8, 209u8, 128u8, 208u8, 180u8,
                    208u8, 176u8, 0u8, 9u8, 32u8, 208u8, 188u8, 208u8, 184u8, 208u8, 187u8, 208u8,
                    187u8, 208u8, 184u8, 208u8, 176u8, 209u8, 128u8, 208u8, 180u8, 0u8, 9u8, 32u8,
                    208u8, 188u8, 208u8, 184u8, 208u8, 187u8, 208u8, 187u8, 208u8, 184u8, 208u8,
                    176u8, 209u8, 128u8, 208u8, 180u8, 208u8, 190u8, 208u8, 178u8, 0u8, 9u8, 32u8,
                    208u8, 188u8, 208u8, 184u8, 208u8, 187u8, 208u8, 187u8, 208u8, 184u8, 208u8,
                    176u8, 209u8, 128u8, 208u8, 180u8, 208u8, 176u8, 0u8, 9u8, 32u8, 208u8, 188u8,
                    208u8, 184u8, 208u8, 187u8, 208u8, 187u8, 208u8, 184u8, 208u8, 176u8, 209u8,
                    128u8, 208u8, 180u8, 0u8, 9u8, 32u8, 208u8, 188u8, 208u8, 184u8, 208u8, 187u8,
                    208u8, 187u8, 208u8, 184u8, 208u8, 176u8, 209u8, 128u8, 208u8, 180u8, 208u8,
                    190u8, 208u8, 178u8, 0u8, 9u8, 32u8, 208u8, 188u8, 208u8, 184u8, 208u8, 187u8,
                    208u8, 187u8, 208u8, 184u8, 208u8, 176u8, 209u8, 128u8, 208u8, 180u8, 208u8,
                    176u8, 0u8, 12u8, 32u8, 209u8, 130u8, 209u8, 128u8, 208u8, 184u8, 208u8, 187u8,
                    208u8, 187u8, 208u8, 184u8, 208u8, 190u8, 208u8, 189u8, 0u8, 12u8, 32u8, 209u8,
                    130u8, 209u8, 128u8, 208u8, 184u8, 208u8, 187u8, 208u8, 187u8, 208u8, 184u8,
                    208u8, 190u8, 208u8, 189u8, 208u8, 190u8, 208u8, 178u8, 0u8, 12u8, 32u8, 209u8,
                    130u8, 209u8, 128u8, 208u8, 184u8, 208u8, 187u8, 208u8, 187u8, 208u8, 184u8,
                    208u8, 190u8, 208u8, 189u8, 208u8, 176u8, 0u8, 12u8, 32u8, 209u8, 130u8, 209u8,
                    128u8, 208u8, 184u8, 208u8, 187u8, 208u8, 187u8, 208u8, 184u8, 208u8, 190u8,
                    208u8, 189u8, 0u8, 12u8, 32u8, 209u8, 130u8, 209u8, 128u8, 208u8, 184u8, 208u8,
                    187u8, 208u8, 187u8, 208u8, 184u8, 208u8, 190u8, 208u8, 189u8, 208u8, 190u8,
                    208u8, 178u8, 0u8, 12u8, 32u8, 209u8, 130u8, 209u8, 128u8, 208u8, 184u8, 208u8,
                    187u8, 208u8, 187u8, 208u8, 184u8, 208u8, 190u8, 208u8, 189u8, 208u8, 176u8,
                    0u8, 12u8, 32u8, 209u8, 130u8, 209u8, 128u8, 208u8, 184u8, 208u8, 187u8, 208u8,
                    187u8, 208u8, 184u8, 208u8, 190u8, 208u8, 189u8, 0u8, 12u8, 32u8, 209u8, 130u8,
                    209u8, 128u8, 208u8, 184u8, 208u8, 187u8, 208u8, 187u8, 208u8, 184u8, 208u8,
                    190u8, 208u8, 189u8, 208u8, 190u8, 208u8, 178u8, 0u8, 12u8, 32u8, 209u8, 130u8,
                    209u8, 128u8, 208u8, 184u8, 208u8, 187u8, 208u8, 187u8, 208u8, 184u8, 208u8,
                    190u8, 208u8, 189u8, 208u8, 176u8,
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
                    1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8,
                    8u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 12u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8,
                    0u8, 17u8, 0u8, 0u8, 0u8, 20u8, 0u8, 0u8, 0u8, 23u8, 0u8, 0u8, 0u8, 25u8, 0u8,
                    0u8, 0u8, 27u8, 0u8, 0u8, 0u8, 29u8, 0u8, 0u8, 0u8,
                ])
            },
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    5u8, 5u8, 3u8, 5u8, 3u8, 5u8, 3u8, 5u8, 1u8, 5u8, 1u8, 5u8, 1u8, 5u8, 1u8, 3u8,
                    5u8, 1u8, 3u8, 5u8, 1u8, 3u8, 5u8, 1u8, 5u8, 1u8, 5u8, 1u8, 5u8,
                ])
            },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    29u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 4u8, 0u8, 14u8, 0u8, 24u8, 0u8, 34u8,
                    0u8, 44u8, 0u8, 54u8, 0u8, 64u8, 0u8, 73u8, 0u8, 83u8, 0u8, 92u8, 0u8, 102u8,
                    0u8, 111u8, 0u8, 121u8, 0u8, 133u8, 0u8, 145u8, 0u8, 157u8, 0u8, 169u8, 0u8,
                    181u8, 0u8, 193u8, 0u8, 205u8, 0u8, 217u8, 0u8, 229u8, 0u8, 238u8, 0u8, 248u8,
                    0u8, 1u8, 1u8, 11u8, 1u8, 20u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 32u8, 104u8,
                    105u8, 108u8, 106u8, 97u8, 100u8, 101u8, 0u8, 3u8, 32u8, 104u8, 105u8, 108u8,
                    106u8, 97u8, 100u8, 97u8, 0u8, 3u8, 32u8, 104u8, 105u8, 108u8, 106u8, 97u8,
                    100u8, 101u8, 0u8, 3u8, 32u8, 104u8, 105u8, 108u8, 106u8, 97u8, 100u8, 97u8,
                    0u8, 3u8, 32u8, 104u8, 105u8, 108u8, 106u8, 97u8, 100u8, 101u8, 0u8, 3u8, 32u8,
                    104u8, 105u8, 108u8, 106u8, 97u8, 100u8, 97u8, 0u8, 6u8, 32u8, 109u8, 105u8,
                    108u8, 105u8, 111u8, 110u8, 0u8, 6u8, 32u8, 109u8, 105u8, 108u8, 105u8, 111u8,
                    110u8, 97u8, 0u8, 6u8, 32u8, 109u8, 105u8, 108u8, 105u8, 111u8, 110u8, 0u8,
                    6u8, 32u8, 109u8, 105u8, 108u8, 105u8, 111u8, 110u8, 97u8, 0u8, 6u8, 32u8,
                    109u8, 105u8, 108u8, 105u8, 111u8, 110u8, 0u8, 6u8, 32u8, 109u8, 105u8, 108u8,
                    105u8, 111u8, 110u8, 97u8, 0u8, 9u8, 32u8, 109u8, 105u8, 108u8, 105u8, 106u8,
                    97u8, 114u8, 100u8, 97u8, 0u8, 9u8, 32u8, 109u8, 105u8, 108u8, 105u8, 106u8,
                    97u8, 114u8, 100u8, 101u8, 0u8, 9u8, 32u8, 109u8, 105u8, 108u8, 105u8, 106u8,
                    97u8, 114u8, 100u8, 105u8, 0u8, 9u8, 32u8, 109u8, 105u8, 108u8, 105u8, 106u8,
                    97u8, 114u8, 100u8, 97u8, 0u8, 9u8, 32u8, 109u8, 105u8, 108u8, 105u8, 106u8,
                    97u8, 114u8, 100u8, 101u8, 0u8, 9u8, 32u8, 109u8, 105u8, 108u8, 105u8, 106u8,
                    97u8, 114u8, 100u8, 105u8, 0u8, 9u8, 32u8, 109u8, 105u8, 108u8, 105u8, 106u8,
                    97u8, 114u8, 100u8, 97u8, 0u8, 9u8, 32u8, 109u8, 105u8, 108u8, 105u8, 106u8,
                    97u8, 114u8, 100u8, 101u8, 0u8, 9u8, 32u8, 109u8, 105u8, 108u8, 105u8, 106u8,
                    97u8, 114u8, 100u8, 105u8, 0u8, 12u8, 32u8, 98u8, 105u8, 108u8, 105u8, 111u8,
                    110u8, 0u8, 12u8, 32u8, 98u8, 105u8, 108u8, 105u8, 111u8, 110u8, 97u8, 0u8,
                    12u8, 32u8, 98u8, 105u8, 108u8, 105u8, 111u8, 110u8, 0u8, 12u8, 32u8, 98u8,
                    105u8, 108u8, 105u8, 111u8, 110u8, 97u8, 0u8, 12u8, 32u8, 98u8, 105u8, 108u8,
                    105u8, 111u8, 110u8, 0u8, 12u8, 32u8, 98u8, 105u8, 108u8, 105u8, 111u8, 110u8,
                    97u8,
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
                    1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8,
                    8u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 12u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8,
                    0u8, 17u8, 0u8, 0u8, 0u8, 20u8, 0u8, 0u8, 0u8, 23u8, 0u8, 0u8, 0u8, 25u8, 0u8,
                    0u8, 0u8, 27u8, 0u8, 0u8, 0u8, 29u8, 0u8, 0u8, 0u8,
                ])
            },
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    5u8, 5u8, 3u8, 5u8, 3u8, 5u8, 3u8, 5u8, 1u8, 5u8, 1u8, 5u8, 1u8, 5u8, 1u8, 3u8,
                    5u8, 1u8, 3u8, 5u8, 1u8, 3u8, 5u8, 1u8, 5u8, 1u8, 5u8, 1u8, 5u8,
                ])
            },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    29u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 4u8, 0u8, 19u8, 0u8, 34u8, 0u8, 49u8,
                    0u8, 64u8, 0u8, 79u8, 0u8, 94u8, 0u8, 109u8, 0u8, 126u8, 0u8, 141u8, 0u8,
                    158u8, 0u8, 173u8, 0u8, 190u8, 0u8, 211u8, 0u8, 232u8, 0u8, 253u8, 0u8, 18u8,
                    1u8, 39u8, 1u8, 60u8, 1u8, 81u8, 1u8, 102u8, 1u8, 123u8, 1u8, 138u8, 1u8,
                    155u8, 1u8, 170u8, 1u8, 187u8, 1u8, 202u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8,
                    32u8, 209u8, 133u8, 208u8, 184u8, 209u8, 153u8, 208u8, 176u8, 208u8, 180u8,
                    208u8, 181u8, 0u8, 3u8, 32u8, 209u8, 133u8, 208u8, 184u8, 209u8, 153u8, 208u8,
                    176u8, 208u8, 180u8, 208u8, 176u8, 0u8, 3u8, 32u8, 209u8, 133u8, 208u8, 184u8,
                    209u8, 153u8, 208u8, 176u8, 208u8, 180u8, 208u8, 181u8, 0u8, 3u8, 32u8, 209u8,
                    133u8, 208u8, 184u8, 209u8, 153u8, 208u8, 176u8, 208u8, 180u8, 208u8, 176u8,
                    0u8, 3u8, 32u8, 209u8, 133u8, 208u8, 184u8, 209u8, 153u8, 208u8, 176u8, 208u8,
                    180u8, 208u8, 181u8, 0u8, 3u8, 32u8, 209u8, 133u8, 208u8, 184u8, 209u8, 153u8,
                    208u8, 176u8, 208u8, 180u8, 208u8, 176u8, 0u8, 6u8, 32u8, 208u8, 188u8, 208u8,
                    184u8, 208u8, 187u8, 208u8, 184u8, 208u8, 190u8, 208u8, 189u8, 0u8, 6u8, 32u8,
                    208u8, 188u8, 208u8, 184u8, 208u8, 187u8, 208u8, 184u8, 208u8, 190u8, 208u8,
                    189u8, 208u8, 176u8, 0u8, 6u8, 32u8, 208u8, 188u8, 208u8, 184u8, 208u8, 187u8,
                    208u8, 184u8, 208u8, 190u8, 208u8, 189u8, 0u8, 6u8, 32u8, 208u8, 188u8, 208u8,
                    184u8, 208u8, 187u8, 208u8, 184u8, 208u8, 190u8, 208u8, 189u8, 208u8, 176u8,
                    0u8, 6u8, 32u8, 208u8, 188u8, 208u8, 184u8, 208u8, 187u8, 208u8, 184u8, 208u8,
                    190u8, 208u8, 189u8, 0u8, 6u8, 32u8, 208u8, 188u8, 208u8, 184u8, 208u8, 187u8,
                    208u8, 184u8, 208u8, 190u8, 208u8, 189u8, 208u8, 176u8, 0u8, 9u8, 32u8, 208u8,
                    188u8, 208u8, 184u8, 208u8, 187u8, 208u8, 184u8, 209u8, 152u8, 208u8, 176u8,
                    209u8, 128u8, 208u8, 180u8, 208u8, 176u8, 0u8, 9u8, 32u8, 208u8, 188u8, 208u8,
                    184u8, 208u8, 187u8, 208u8, 184u8, 209u8, 152u8, 208u8, 176u8, 209u8, 128u8,
                    208u8, 180u8, 208u8, 181u8, 0u8, 9u8, 32u8, 208u8, 188u8, 208u8, 184u8, 208u8,
                    187u8, 208u8, 184u8, 209u8, 152u8, 208u8, 176u8, 209u8, 128u8, 208u8, 180u8,
                    208u8, 184u8, 0u8, 9u8, 32u8, 208u8, 188u8, 208u8, 184u8, 208u8, 187u8, 208u8,
                    184u8, 209u8, 152u8, 208u8, 176u8, 209u8, 128u8, 208u8, 180u8, 208u8, 176u8,
                    0u8, 9u8, 32u8, 208u8, 188u8, 208u8, 184u8, 208u8, 187u8, 208u8, 184u8, 209u8,
                    152u8, 208u8, 176u8, 209u8, 128u8, 208u8, 180u8, 208u8, 181u8, 0u8, 9u8, 32u8,
                    208u8, 188u8, 208u8, 184u8, 208u8, 187u8, 208u8, 184u8, 209u8, 152u8, 208u8,
                    176u8, 209u8, 128u8, 208u8, 180u8, 208u8, 184u8, 0u8, 9u8, 32u8, 208u8, 188u8,
                    208u8, 184u8, 208u8, 187u8, 208u8, 184u8, 209u8, 152u8, 208u8, 176u8, 209u8,
                    128u8, 208u8, 180u8, 208u8, 176u8, 0u8, 9u8, 32u8, 208u8, 188u8, 208u8, 184u8,
                    208u8, 187u8, 208u8, 184u8, 209u8, 152u8, 208u8, 176u8, 209u8, 128u8, 208u8,
                    180u8, 208u8, 181u8, 0u8, 9u8, 32u8, 208u8, 188u8, 208u8, 184u8, 208u8, 187u8,
                    208u8, 184u8, 209u8, 152u8, 208u8, 176u8, 209u8, 128u8, 208u8, 180u8, 208u8,
                    184u8, 0u8, 12u8, 32u8, 208u8, 177u8, 208u8, 184u8, 208u8, 187u8, 208u8, 184u8,
                    208u8, 190u8, 208u8, 189u8, 0u8, 12u8, 32u8, 208u8, 177u8, 208u8, 184u8, 208u8,
                    187u8, 208u8, 184u8, 208u8, 190u8, 208u8, 189u8, 208u8, 176u8, 0u8, 12u8, 32u8,
                    208u8, 177u8, 208u8, 184u8, 208u8, 187u8, 208u8, 184u8, 208u8, 190u8, 208u8,
                    189u8, 0u8, 12u8, 32u8, 208u8, 177u8, 208u8, 184u8, 208u8, 187u8, 208u8, 184u8,
                    208u8, 190u8, 208u8, 189u8, 208u8, 176u8, 0u8, 12u8, 32u8, 208u8, 177u8, 208u8,
                    184u8, 208u8, 187u8, 208u8, 184u8, 208u8, 190u8, 208u8, 189u8, 0u8, 12u8, 32u8,
                    208u8, 177u8, 208u8, 184u8, 208u8, 187u8, 208u8, 184u8, 208u8, 190u8, 208u8,
                    189u8, 208u8, 176u8,
                ])
            },
        )
    },
};
static TH_TH_U_NU_THAI: &DataStruct =
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
                        14u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 4u8, 0u8, 16u8, 0u8, 34u8, 0u8,
                        46u8, 0u8, 61u8, 0u8, 76u8, 0u8, 91u8, 0u8, 115u8, 0u8, 145u8, 0u8, 169u8,
                        0u8, 196u8, 0u8, 223u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 32u8, 224u8,
                        184u8, 158u8, 224u8, 184u8, 177u8, 224u8, 184u8, 153u8, 0u8, 4u8, 32u8,
                        224u8, 184u8, 171u8, 224u8, 184u8, 161u8, 224u8, 184u8, 183u8, 224u8,
                        185u8, 136u8, 224u8, 184u8, 153u8, 0u8, 5u8, 32u8, 224u8, 185u8, 129u8,
                        224u8, 184u8, 170u8, 224u8, 184u8, 153u8, 0u8, 6u8, 32u8, 224u8, 184u8,
                        165u8, 224u8, 185u8, 137u8, 224u8, 184u8, 178u8, 224u8, 184u8, 153u8, 0u8,
                        6u8, 32u8, 224u8, 184u8, 165u8, 224u8, 185u8, 137u8, 224u8, 184u8, 178u8,
                        224u8, 184u8, 153u8, 0u8, 6u8, 32u8, 224u8, 184u8, 165u8, 224u8, 185u8,
                        137u8, 224u8, 184u8, 178u8, 224u8, 184u8, 153u8, 0u8, 9u8, 32u8, 224u8,
                        184u8, 158u8, 224u8, 184u8, 177u8, 224u8, 184u8, 153u8, 224u8, 184u8,
                        165u8, 224u8, 185u8, 137u8, 224u8, 184u8, 178u8, 224u8, 184u8, 153u8, 0u8,
                        10u8, 32u8, 224u8, 184u8, 171u8, 224u8, 184u8, 161u8, 224u8, 184u8, 183u8,
                        224u8, 185u8, 136u8, 224u8, 184u8, 153u8, 224u8, 184u8, 165u8, 224u8,
                        185u8, 137u8, 224u8, 184u8, 178u8, 224u8, 184u8, 153u8, 0u8, 11u8, 32u8,
                        224u8, 185u8, 129u8, 224u8, 184u8, 170u8, 224u8, 184u8, 153u8, 224u8,
                        184u8, 165u8, 224u8, 185u8, 137u8, 224u8, 184u8, 178u8, 224u8, 184u8,
                        153u8, 0u8, 12u8, 32u8, 224u8, 184u8, 165u8, 224u8, 185u8, 137u8, 224u8,
                        184u8, 178u8, 224u8, 184u8, 153u8, 224u8, 184u8, 165u8, 224u8, 185u8,
                        137u8, 224u8, 184u8, 178u8, 224u8, 184u8, 153u8, 0u8, 12u8, 32u8, 224u8,
                        184u8, 165u8, 224u8, 185u8, 137u8, 224u8, 184u8, 178u8, 224u8, 184u8,
                        153u8, 224u8, 184u8, 165u8, 224u8, 185u8, 137u8, 224u8, 184u8, 178u8,
                        224u8, 184u8, 153u8, 0u8, 12u8, 32u8, 224u8, 184u8, 165u8, 224u8, 185u8,
                        137u8, 224u8, 184u8, 178u8, 224u8, 184u8, 153u8, 224u8, 184u8, 165u8,
                        224u8, 185u8, 137u8, 224u8, 184u8, 178u8, 224u8, 184u8, 153u8,
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
                    14u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 4u8, 0u8, 10u8, 0u8, 16u8, 0u8, 22u8,
                    0u8, 31u8, 0u8, 40u8, 0u8, 49u8, 0u8, 58u8, 0u8, 67u8, 0u8, 76u8, 0u8, 86u8,
                    0u8, 96u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 32u8, 98u8, 105u8, 110u8, 0u8,
                    3u8, 32u8, 98u8, 105u8, 110u8, 0u8, 3u8, 32u8, 98u8, 105u8, 110u8, 0u8, 6u8,
                    32u8, 109u8, 105u8, 108u8, 121u8, 111u8, 110u8, 0u8, 6u8, 32u8, 109u8, 105u8,
                    108u8, 121u8, 111u8, 110u8, 0u8, 6u8, 32u8, 109u8, 105u8, 108u8, 121u8, 111u8,
                    110u8, 0u8, 9u8, 32u8, 109u8, 105u8, 108u8, 121u8, 97u8, 114u8, 0u8, 9u8, 32u8,
                    109u8, 105u8, 108u8, 121u8, 97u8, 114u8, 0u8, 9u8, 32u8, 109u8, 105u8, 108u8,
                    121u8, 97u8, 114u8, 0u8, 12u8, 32u8, 116u8, 114u8, 105u8, 108u8, 121u8, 111u8,
                    110u8, 0u8, 12u8, 32u8, 116u8, 114u8, 105u8, 108u8, 121u8, 111u8, 110u8, 0u8,
                    12u8, 32u8, 116u8, 114u8, 105u8, 108u8, 121u8, 111u8, 110u8,
                ])
            },
        )
    },
};
