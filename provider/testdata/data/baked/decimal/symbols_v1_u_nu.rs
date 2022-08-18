// @generated
type DataStruct =
    <::icu_decimal::provider::DecimalSymbolsV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_slice_unchecked(&[
        ("ar", AR_AR_EG),
        ("ar-EG", AR_AR_EG),
        ("ar-EG-u-nu-latn", AR_EG_U_NU_LATN_AR_U_NU_LATN),
        ("ar-u-nu-latn", AR_EG_U_NU_LATN_AR_U_NU_LATN),
        ("bn", BN),
        ("bn-u-nu-latn", BN_U_NU_LATN_CCP_U_NU_LATN),
        ("ccp", CCP),
        ("ccp-u-nu-latn", BN_U_NU_LATN_CCP_U_NU_LATN),
        ("en", EN_EN_001_FIL_JA_TH_UND),
        ("en-001", EN_EN_001_FIL_JA_TH_UND),
        ("en-ZA", EN_ZA_RU),
        ("es", ES),
        ("es-AR", ES_AR_SR_SR_CYRL_SR_LATN_TR),
        ("fil", EN_EN_001_FIL_JA_TH_UND),
        ("fr", FR),
        ("ja", EN_EN_001_FIL_JA_TH_UND),
        ("ru", EN_ZA_RU),
        ("sr", ES_AR_SR_SR_CYRL_SR_LATN_TR),
        ("sr-Cyrl", ES_AR_SR_SR_CYRL_SR_LATN_TR),
        ("sr-Latn", ES_AR_SR_SR_CYRL_SR_LATN_TR),
        ("th", EN_EN_001_FIL_JA_TH_UND),
        ("th-u-nu-thai", TH_U_NU_THAI),
        ("tr", ES_AR_SR_SR_CYRL_SR_LATN_TR),
        ("und", EN_EN_001_FIL_JA_TH_UND),
    ]);
static AR_AR_EG: &DataStruct = &::icu_decimal::provider::DecimalSymbolsV1 {
    minus_sign_affixes: ::icu_decimal::provider::AffixesV1 {
        prefix: alloc::borrow::Cow::Borrowed("\u{61c}-"),
        suffix: alloc::borrow::Cow::Borrowed(""),
    },
    plus_sign_affixes: ::icu_decimal::provider::AffixesV1 {
        prefix: alloc::borrow::Cow::Borrowed("\u{61c}+"),
        suffix: alloc::borrow::Cow::Borrowed(""),
    },
    decimal_separator: alloc::borrow::Cow::Borrowed("٫"),
    grouping_separator: alloc::borrow::Cow::Borrowed("٬"),
    grouping_sizes: ::icu_decimal::provider::GroupingSizesV1 {
        primary: 3u8,
        secondary: 3u8,
        min_grouping: 1u8,
    },
    digits: ['٠', '١', '٢', '٣', '٤', '٥', '٦', '٧', '٨', '٩'],
};
static AR_EG_U_NU_LATN_AR_U_NU_LATN: &DataStruct = &::icu_decimal::provider::DecimalSymbolsV1 {
    minus_sign_affixes: ::icu_decimal::provider::AffixesV1 {
        prefix: alloc::borrow::Cow::Borrowed("\u{200e}-"),
        suffix: alloc::borrow::Cow::Borrowed(""),
    },
    plus_sign_affixes: ::icu_decimal::provider::AffixesV1 {
        prefix: alloc::borrow::Cow::Borrowed("\u{200e}+"),
        suffix: alloc::borrow::Cow::Borrowed(""),
    },
    decimal_separator: alloc::borrow::Cow::Borrowed("."),
    grouping_separator: alloc::borrow::Cow::Borrowed(","),
    grouping_sizes: ::icu_decimal::provider::GroupingSizesV1 {
        primary: 3u8,
        secondary: 3u8,
        min_grouping: 1u8,
    },
    digits: ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
};
static BN: &DataStruct = &::icu_decimal::provider::DecimalSymbolsV1 {
    minus_sign_affixes: ::icu_decimal::provider::AffixesV1 {
        prefix: alloc::borrow::Cow::Borrowed("-"),
        suffix: alloc::borrow::Cow::Borrowed(""),
    },
    plus_sign_affixes: ::icu_decimal::provider::AffixesV1 {
        prefix: alloc::borrow::Cow::Borrowed("+"),
        suffix: alloc::borrow::Cow::Borrowed(""),
    },
    decimal_separator: alloc::borrow::Cow::Borrowed("."),
    grouping_separator: alloc::borrow::Cow::Borrowed(","),
    grouping_sizes: ::icu_decimal::provider::GroupingSizesV1 {
        primary: 3u8,
        secondary: 2u8,
        min_grouping: 1u8,
    },
    digits: ['০', '১', '২', '৩', '৪', '৫', '৬', '৭', '৮', '৯'],
};
static BN_U_NU_LATN_CCP_U_NU_LATN: &DataStruct = &::icu_decimal::provider::DecimalSymbolsV1 {
    minus_sign_affixes: ::icu_decimal::provider::AffixesV1 {
        prefix: alloc::borrow::Cow::Borrowed("-"),
        suffix: alloc::borrow::Cow::Borrowed(""),
    },
    plus_sign_affixes: ::icu_decimal::provider::AffixesV1 {
        prefix: alloc::borrow::Cow::Borrowed("+"),
        suffix: alloc::borrow::Cow::Borrowed(""),
    },
    decimal_separator: alloc::borrow::Cow::Borrowed("."),
    grouping_separator: alloc::borrow::Cow::Borrowed(","),
    grouping_sizes: ::icu_decimal::provider::GroupingSizesV1 {
        primary: 3u8,
        secondary: 2u8,
        min_grouping: 1u8,
    },
    digits: ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
};
static CCP: &DataStruct = &::icu_decimal::provider::DecimalSymbolsV1 {
    minus_sign_affixes: ::icu_decimal::provider::AffixesV1 {
        prefix: alloc::borrow::Cow::Borrowed("-"),
        suffix: alloc::borrow::Cow::Borrowed(""),
    },
    plus_sign_affixes: ::icu_decimal::provider::AffixesV1 {
        prefix: alloc::borrow::Cow::Borrowed("+"),
        suffix: alloc::borrow::Cow::Borrowed(""),
    },
    decimal_separator: alloc::borrow::Cow::Borrowed("."),
    grouping_separator: alloc::borrow::Cow::Borrowed(","),
    grouping_sizes: ::icu_decimal::provider::GroupingSizesV1 {
        primary: 3u8,
        secondary: 2u8,
        min_grouping: 1u8,
    },
    digits: ['𑄶', '𑄷', '𑄸', '𑄹', '𑄺', '𑄻', '𑄼', '𑄽', '𑄾', '𑄿'],
};
static EN_EN_001_FIL_JA_TH_UND: &DataStruct = &::icu_decimal::provider::DecimalSymbolsV1 {
    minus_sign_affixes: ::icu_decimal::provider::AffixesV1 {
        prefix: alloc::borrow::Cow::Borrowed("-"),
        suffix: alloc::borrow::Cow::Borrowed(""),
    },
    plus_sign_affixes: ::icu_decimal::provider::AffixesV1 {
        prefix: alloc::borrow::Cow::Borrowed("+"),
        suffix: alloc::borrow::Cow::Borrowed(""),
    },
    decimal_separator: alloc::borrow::Cow::Borrowed("."),
    grouping_separator: alloc::borrow::Cow::Borrowed(","),
    grouping_sizes: ::icu_decimal::provider::GroupingSizesV1 {
        primary: 3u8,
        secondary: 3u8,
        min_grouping: 1u8,
    },
    digits: ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
};
static EN_ZA_RU: &DataStruct = &::icu_decimal::provider::DecimalSymbolsV1 {
    minus_sign_affixes: ::icu_decimal::provider::AffixesV1 {
        prefix: alloc::borrow::Cow::Borrowed("-"),
        suffix: alloc::borrow::Cow::Borrowed(""),
    },
    plus_sign_affixes: ::icu_decimal::provider::AffixesV1 {
        prefix: alloc::borrow::Cow::Borrowed("+"),
        suffix: alloc::borrow::Cow::Borrowed(""),
    },
    decimal_separator: alloc::borrow::Cow::Borrowed(","),
    grouping_separator: alloc::borrow::Cow::Borrowed("\u{a0}"),
    grouping_sizes: ::icu_decimal::provider::GroupingSizesV1 {
        primary: 3u8,
        secondary: 3u8,
        min_grouping: 1u8,
    },
    digits: ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
};
static ES: &DataStruct = &::icu_decimal::provider::DecimalSymbolsV1 {
    minus_sign_affixes: ::icu_decimal::provider::AffixesV1 {
        prefix: alloc::borrow::Cow::Borrowed("-"),
        suffix: alloc::borrow::Cow::Borrowed(""),
    },
    plus_sign_affixes: ::icu_decimal::provider::AffixesV1 {
        prefix: alloc::borrow::Cow::Borrowed("+"),
        suffix: alloc::borrow::Cow::Borrowed(""),
    },
    decimal_separator: alloc::borrow::Cow::Borrowed(","),
    grouping_separator: alloc::borrow::Cow::Borrowed("."),
    grouping_sizes: ::icu_decimal::provider::GroupingSizesV1 {
        primary: 3u8,
        secondary: 3u8,
        min_grouping: 2u8,
    },
    digits: ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
};
static ES_AR_SR_SR_CYRL_SR_LATN_TR: &DataStruct = &::icu_decimal::provider::DecimalSymbolsV1 {
    minus_sign_affixes: ::icu_decimal::provider::AffixesV1 {
        prefix: alloc::borrow::Cow::Borrowed("-"),
        suffix: alloc::borrow::Cow::Borrowed(""),
    },
    plus_sign_affixes: ::icu_decimal::provider::AffixesV1 {
        prefix: alloc::borrow::Cow::Borrowed("+"),
        suffix: alloc::borrow::Cow::Borrowed(""),
    },
    decimal_separator: alloc::borrow::Cow::Borrowed(","),
    grouping_separator: alloc::borrow::Cow::Borrowed("."),
    grouping_sizes: ::icu_decimal::provider::GroupingSizesV1 {
        primary: 3u8,
        secondary: 3u8,
        min_grouping: 1u8,
    },
    digits: ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
};
static FR: &DataStruct = &::icu_decimal::provider::DecimalSymbolsV1 {
    minus_sign_affixes: ::icu_decimal::provider::AffixesV1 {
        prefix: alloc::borrow::Cow::Borrowed("-"),
        suffix: alloc::borrow::Cow::Borrowed(""),
    },
    plus_sign_affixes: ::icu_decimal::provider::AffixesV1 {
        prefix: alloc::borrow::Cow::Borrowed("+"),
        suffix: alloc::borrow::Cow::Borrowed(""),
    },
    decimal_separator: alloc::borrow::Cow::Borrowed(","),
    grouping_separator: alloc::borrow::Cow::Borrowed("\u{202f}"),
    grouping_sizes: ::icu_decimal::provider::GroupingSizesV1 {
        primary: 3u8,
        secondary: 3u8,
        min_grouping: 1u8,
    },
    digits: ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
};
static TH_U_NU_THAI: &DataStruct = &::icu_decimal::provider::DecimalSymbolsV1 {
    minus_sign_affixes: ::icu_decimal::provider::AffixesV1 {
        prefix: alloc::borrow::Cow::Borrowed("-"),
        suffix: alloc::borrow::Cow::Borrowed(""),
    },
    plus_sign_affixes: ::icu_decimal::provider::AffixesV1 {
        prefix: alloc::borrow::Cow::Borrowed("+"),
        suffix: alloc::borrow::Cow::Borrowed(""),
    },
    decimal_separator: alloc::borrow::Cow::Borrowed("."),
    grouping_separator: alloc::borrow::Cow::Borrowed(","),
    grouping_sizes: ::icu_decimal::provider::GroupingSizesV1 {
        primary: 3u8,
        secondary: 3u8,
        min_grouping: 1u8,
    },
    digits: ['๐', '๑', '๒', '๓', '๔', '๕', '๖', '๗', '๘', '๙'],
};
