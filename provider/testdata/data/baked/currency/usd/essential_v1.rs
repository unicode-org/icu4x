// @generated
#![cfg(feature = "icu_plurals")]
type DataStruct =
    <::icu_plurals::provider::CurrencyEssentialUsdV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_store_unchecked(&[
        ("ar", AR_AR_EG_BN_CCP_EN_001_EN_ZA_ES_ES_AR),
        ("ar-EG", AR_AR_EG_BN_CCP_EN_001_EN_ZA_ES_ES_AR),
        ("bn", AR_AR_EG_BN_CCP_EN_001_EN_ZA_ES_ES_AR),
        ("ccp", AR_AR_EG_BN_CCP_EN_001_EN_ZA_ES_ES_AR),
        ("en", EN_FIL_JA_RU_TR),
        ("en-001", AR_AR_EG_BN_CCP_EN_001_EN_ZA_ES_ES_AR),
        ("en-ZA", AR_AR_EG_BN_CCP_EN_001_EN_ZA_ES_ES_AR),
        ("es", AR_AR_EG_BN_CCP_EN_001_EN_ZA_ES_ES_AR),
        ("es-AR", AR_AR_EG_BN_CCP_EN_001_EN_ZA_ES_ES_AR),
        ("fil", EN_FIL_JA_RU_TR),
        ("fr", FR),
        ("ja", EN_FIL_JA_RU_TR),
        ("ru", EN_FIL_JA_RU_TR),
        ("sr", AR_AR_EG_BN_CCP_EN_001_EN_ZA_ES_ES_AR),
        ("sr-Cyrl", AR_AR_EG_BN_CCP_EN_001_EN_ZA_ES_ES_AR),
        ("sr-Latn", AR_AR_EG_BN_CCP_EN_001_EN_ZA_ES_ES_AR),
        ("th", AR_AR_EG_BN_CCP_EN_001_EN_ZA_ES_ES_AR),
        ("tr", EN_FIL_JA_RU_TR),
        ("und", AR_AR_EG_BN_CCP_EN_001_EN_ZA_ES_ES_AR),
    ]);
static AR_AR_EG_BN_CCP_EN_001_EN_ZA_ES_ES_AR: &DataStruct =
    &::icu_plurals::provider::CurrencyEssentialV1 {
        symbol: alloc::borrow::Cow::Borrowed("US$"),
        pattern: ::icu_singlenumberformatter::provider::CurrencyPattern {
            index: 0u8,
            pattern: alloc::borrow::Cow::Borrowed("not yet"),
        },
    };
static EN_FIL_JA_RU_TR: &DataStruct = &::icu_plurals::provider::CurrencyEssentialV1 {
    symbol: alloc::borrow::Cow::Borrowed("$"),
    pattern: ::icu_singlenumberformatter::provider::CurrencyPattern {
        index: 0u8,
        pattern: alloc::borrow::Cow::Borrowed("not yet"),
    },
};
static FR: &DataStruct = &::icu_plurals::provider::CurrencyEssentialV1 {
    symbol: alloc::borrow::Cow::Borrowed("$US"),
    pattern: ::icu_singlenumberformatter::provider::CurrencyPattern {
        index: 0u8,
        pattern: alloc::borrow::Cow::Borrowed("not yet"),
    },
};
