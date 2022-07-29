// @generated
type DataStruct =
    <::icu_plurals::provider::OrdinalV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_slice_unchecked(&[
        ("ar", AR_ES_JA_RU_SR_TH_TR_UND),
        ("bn", BN),
        ("en", EN),
        ("es", AR_ES_JA_RU_SR_TH_TR_UND),
        ("fil", FIL_FR),
        ("fr", FIL_FR),
        ("ja", AR_ES_JA_RU_SR_TH_TR_UND),
        ("ru", AR_ES_JA_RU_SR_TH_TR_UND),
        ("sr", AR_ES_JA_RU_SR_TH_TR_UND),
        ("th", AR_ES_JA_RU_SR_TH_TR_UND),
        ("tr", AR_ES_JA_RU_SR_TH_TR_UND),
        ("und", AR_ES_JA_RU_SR_TH_TR_UND),
    ]);
static AR_ES_JA_RU_SR_TH_TR_UND: &DataStruct = &::icu_plurals::provider::PluralRulesV1 {
    zero: None,
    one: None,
    two: None,
    few: None,
    many: None,
};
static BN: &DataStruct = &::icu_plurals::provider::PluralRulesV1 {
    zero: None,
    one: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        ::zerovec::VarZeroVec::from_bytes_unchecked(&[
            1u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 192u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8,
            0u8, 1u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 0u8,
            7u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 9u8, 0u8, 0u8, 0u8, 9u8,
            0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8,
        ])
    })),
    two: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        ::zerovec::VarZeroVec::from_bytes_unchecked(&[
            1u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 192u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8,
            0u8, 2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8,
        ])
    })),
    few: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        ::zerovec::VarZeroVec::from_bytes_unchecked(&[
            1u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 192u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8,
            0u8, 4u8, 0u8, 0u8, 0u8,
        ])
    })),
    many: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        ::zerovec::VarZeroVec::from_bytes_unchecked(&[
            1u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 192u8, 0u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8,
            0u8, 6u8, 0u8, 0u8, 0u8,
        ])
    })),
};
static EN: &DataStruct = &::icu_plurals::provider::PluralRulesV1 {
    zero: None,
    one: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        ::zerovec::VarZeroVec::from_bytes_unchecked(&[
            2u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 13u8, 0u8, 0u8, 0u8, 192u8, 10u8, 0u8,
            0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 128u8, 100u8, 0u8, 0u8, 0u8, 11u8,
            0u8, 0u8, 0u8, 11u8, 0u8, 0u8, 0u8,
        ])
    })),
    two: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        ::zerovec::VarZeroVec::from_bytes_unchecked(&[
            2u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 13u8, 0u8, 0u8, 0u8, 192u8, 10u8, 0u8,
            0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 128u8, 100u8, 0u8, 0u8, 0u8, 12u8,
            0u8, 0u8, 0u8, 12u8, 0u8, 0u8, 0u8,
        ])
    })),
    few: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        ::zerovec::VarZeroVec::from_bytes_unchecked(&[
            2u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 13u8, 0u8, 0u8, 0u8, 192u8, 10u8, 0u8,
            0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 128u8, 100u8, 0u8, 0u8, 0u8, 13u8,
            0u8, 0u8, 0u8, 13u8, 0u8, 0u8, 0u8,
        ])
    })),
    many: None,
};
static FIL_FR: &DataStruct = &::icu_plurals::provider::PluralRulesV1 {
    zero: None,
    one: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        ::zerovec::VarZeroVec::from_bytes_unchecked(&[
            1u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 192u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8,
            0u8, 1u8, 0u8, 0u8, 0u8,
        ])
    })),
    two: None,
    few: None,
    many: None,
};
