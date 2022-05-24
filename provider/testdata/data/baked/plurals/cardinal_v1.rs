// @generated
use icu_provider::prelude::*;
impl ResourceProvider<::icu_plurals::provider::CardinalV1Marker>
    for super::super::BakedDataProvider
{
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<::icu_plurals::provider::CardinalV1Marker>, DataError> {
        static VALUES: &[(&str, DataStruct)] = &[
            ("ar", AR),
            ("bn", BN),
            ("en", EN),
            ("es", ES),
            ("fil", FIL),
            ("fr", FR),
            ("ja", JA_TH_UND),
            ("ru", RU),
            ("sr", SR),
            ("th", JA_TH_UND),
            ("tr", TR),
            ("und", JA_TH_UND),
        ];
        #[allow(clippy::unwrap_used)]
        let value = VALUES
            .binary_search_by(|(k, _)| req.options.cmp_bytes(k.as_bytes()).reverse())
            .map(|i| VALUES.get(i).unwrap().1)
            .map_err(|_| {
                DataErrorKind::MissingResourceOptions
                    .with_req(<::icu_plurals::provider::CardinalV1Marker>::KEY, req)
            })?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(zerofrom::ZeroFrom::zero_from(
                value,
            ))),
        })
    }
}
type DataStruct = &'static <::icu_plurals::provider::CardinalV1Marker as DataMarker>::Yokeable;
static AR: DataStruct = &::icu_plurals::provider::PluralRulesV1 {
    zero: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        ::zerovec::VarZeroVec::from_bytes_unchecked(&[
            1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 192u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 0u8,
        ])
    })),
    one: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        ::zerovec::VarZeroVec::from_bytes_unchecked(&[
            1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 192u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8,
            1u8, 0u8, 0u8, 0u8,
        ])
    })),
    two: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        ::zerovec::VarZeroVec::from_bytes_unchecked(&[
            1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 192u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8,
            2u8, 0u8, 0u8, 0u8,
        ])
    })),
    few: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        ::zerovec::VarZeroVec::from_bytes_unchecked(&[
            1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 192u8, 100u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8,
            0u8, 10u8, 0u8, 0u8, 0u8,
        ])
    })),
    many: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        ::zerovec::VarZeroVec::from_bytes_unchecked(&[
            1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 192u8, 100u8, 0u8, 0u8, 0u8, 11u8, 0u8, 0u8,
            0u8, 99u8, 0u8, 0u8, 0u8,
        ])
    })),
};
static BN: DataStruct = &::icu_plurals::provider::PluralRulesV1 {
    zero: None,
    one: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        ::zerovec::VarZeroVec::from_bytes_unchecked(&[
            2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 13u8, 0u8, 0u8, 0u8, 193u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 64u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8,
            1u8, 0u8, 0u8, 0u8,
        ])
    })),
    two: None,
    few: None,
    many: None,
};
static EN: DataStruct = &::icu_plurals::provider::PluralRulesV1 {
    zero: None,
    one: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        ::zerovec::VarZeroVec::from_bytes_unchecked(&[
            2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 13u8, 0u8, 0u8, 0u8, 193u8, 0u8, 0u8, 0u8, 0u8,
            1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 194u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 0u8,
        ])
    })),
    two: None,
    few: None,
    many: None,
};
static ES: DataStruct = &::icu_plurals::provider::PluralRulesV1 {
    zero: None,
    one: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        ::zerovec::VarZeroVec::from_bytes_unchecked(&[
            1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 192u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8,
            1u8, 0u8, 0u8, 0u8,
        ])
    })),
    two: None,
    few: None,
    many: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        ::zerovec::VarZeroVec::from_bytes_unchecked(&[
            5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 13u8, 0u8, 0u8, 0u8, 26u8, 0u8, 0u8, 0u8, 39u8,
            0u8, 0u8, 0u8, 52u8, 0u8, 0u8, 0u8, 199u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 129u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            193u8, 64u8, 66u8, 15u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 194u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 5u8, 0u8, 0u8, 0u8,
        ])
    })),
};
static FIL: DataStruct = &::icu_plurals::provider::PluralRulesV1 {
    zero: None,
    one: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        ::zerovec::VarZeroVec::from_bytes_unchecked(&[
            6u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 13u8, 0u8, 0u8, 0u8, 42u8, 0u8, 0u8, 0u8, 55u8,
            0u8, 0u8, 0u8, 84u8, 0u8, 0u8, 0u8, 97u8, 0u8, 0u8, 0u8, 194u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 193u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8,
            1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 3u8,
            0u8, 0u8, 0u8, 66u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 129u8,
            10u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 6u8,
            0u8, 0u8, 0u8, 9u8, 0u8, 0u8, 0u8, 9u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 132u8, 10u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 4u8,
            0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 9u8, 0u8, 0u8, 0u8, 9u8, 0u8,
            0u8, 0u8,
        ])
    })),
    two: None,
    few: None,
    many: None,
};
static FR: DataStruct = &::icu_plurals::provider::PluralRulesV1 {
    zero: None,
    one: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        ::zerovec::VarZeroVec::from_bytes_unchecked(&[
            1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 193u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8,
        ])
    })),
    two: None,
    few: None,
    many: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        ::zerovec::VarZeroVec::from_bytes_unchecked(&[
            5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 13u8, 0u8, 0u8, 0u8, 26u8, 0u8, 0u8, 0u8, 39u8,
            0u8, 0u8, 0u8, 52u8, 0u8, 0u8, 0u8, 199u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 129u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            193u8, 64u8, 66u8, 15u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 194u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 5u8, 0u8, 0u8, 0u8,
        ])
    })),
};
static JA_TH_UND: DataStruct = &::icu_plurals::provider::PluralRulesV1 {
    zero: None,
    one: None,
    two: None,
    few: None,
    many: None,
};
static RU: DataStruct = &::icu_plurals::provider::PluralRulesV1 {
    zero: None,
    one: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        ::zerovec::VarZeroVec::from_bytes_unchecked(&[
            3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 13u8, 0u8, 0u8, 0u8, 26u8, 0u8, 0u8, 0u8,
            194u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 193u8, 10u8, 0u8,
            0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 129u8, 100u8, 0u8, 0u8, 0u8, 11u8,
            0u8, 0u8, 0u8, 11u8, 0u8, 0u8, 0u8,
        ])
    })),
    two: None,
    few: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        ::zerovec::VarZeroVec::from_bytes_unchecked(&[
            3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 13u8, 0u8, 0u8, 0u8, 26u8, 0u8, 0u8, 0u8,
            194u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 193u8, 10u8, 0u8,
            0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 129u8, 100u8, 0u8, 0u8, 0u8, 12u8,
            0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8,
        ])
    })),
    many: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        ::zerovec::VarZeroVec::from_bytes_unchecked(&[
            6u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 13u8, 0u8, 0u8, 0u8, 26u8, 0u8, 0u8, 0u8, 39u8,
            0u8, 0u8, 0u8, 52u8, 0u8, 0u8, 0u8, 65u8, 0u8, 0u8, 0u8, 194u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 193u8, 10u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 0u8, 66u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            193u8, 10u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 9u8, 0u8, 0u8, 0u8, 66u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 193u8, 100u8, 0u8, 0u8, 0u8, 11u8,
            0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8,
        ])
    })),
};
static SR: DataStruct = &::icu_plurals::provider::PluralRulesV1 {
    zero: None,
    one: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        ::zerovec::VarZeroVec::from_bytes_unchecked(&[
            5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 13u8, 0u8, 0u8, 0u8, 26u8, 0u8, 0u8, 0u8, 39u8,
            0u8, 0u8, 0u8, 52u8, 0u8, 0u8, 0u8, 194u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 193u8, 10u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8,
            129u8, 100u8, 0u8, 0u8, 0u8, 11u8, 0u8, 0u8, 0u8, 11u8, 0u8, 0u8, 0u8, 68u8, 10u8, 0u8,
            0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 132u8, 100u8, 0u8, 0u8, 0u8, 11u8,
            0u8, 0u8, 0u8, 11u8, 0u8, 0u8, 0u8,
        ])
    })),
    two: None,
    few: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        ::zerovec::VarZeroVec::from_bytes_unchecked(&[
            5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 13u8, 0u8, 0u8, 0u8, 26u8, 0u8, 0u8, 0u8, 39u8,
            0u8, 0u8, 0u8, 52u8, 0u8, 0u8, 0u8, 194u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 193u8, 10u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8,
            129u8, 100u8, 0u8, 0u8, 0u8, 12u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8, 68u8, 10u8, 0u8,
            0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 132u8, 100u8, 0u8, 0u8, 0u8, 12u8,
            0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8,
        ])
    })),
    many: None,
};
static TR: DataStruct = &::icu_plurals::provider::PluralRulesV1 {
    zero: None,
    one: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        ::zerovec::VarZeroVec::from_bytes_unchecked(&[
            1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 192u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8,
            1u8, 0u8, 0u8, 0u8,
        ])
    })),
    two: None,
    few: None,
    many: None,
};
