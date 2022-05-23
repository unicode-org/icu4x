// @generated
use icu_provider::prelude::*;
impl ResourceProvider<::icu_plurals::provider::OrdinalV1Marker>
    for super::super::BakedDataProvider
{
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<::icu_plurals::provider::OrdinalV1Marker>, DataError> {
        static VALUES: &[(&str, DataStruct)] = &[
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
        ];
        #[allow(clippy::unwrap_used)]
        let value = VALUES
            .binary_search_by(|(k, _)| req.options.cmp_bytes(k.as_bytes()).reverse())
            .map(|i| VALUES.get(i).unwrap().1)
            .map_err(|_| {
                DataErrorKind::MissingResourceOptions
                    .with_req(<::icu_plurals::provider::OrdinalV1Marker>::KEY, req)
            })?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(zerofrom::ZeroFrom::zero_from(
                value,
            ))),
        })
    }
}
type DataStruct = &'static <::icu_plurals::provider::OrdinalV1Marker as DataMarker>::Yokeable;
static AR_ES_JA_RU_SR_TH_TR_UND: DataStruct = &::icu_plurals::provider::PluralRulesV1 {
    zero: None,
    one: None,
    two: None,
    few: None,
    many: None,
};
static BN: DataStruct = &::icu_plurals::provider::PluralRulesV1 {
    zero: None,
    one: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        static BYTES: &[u8] = &[
            1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 192u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8,
            1u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 7u8,
            0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 9u8, 0u8, 0u8, 0u8, 9u8, 0u8,
            0u8, 0u8, 10u8, 0u8, 0u8, 0u8, 10u8, 0u8, 0u8, 0u8,
        ];
        ::zerovec::VarZeroVec::Borrowed(core::mem::transmute(BYTES))
    })),
    two: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        static BYTES: &[u8] = &[
            1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 192u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8,
            2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8,
        ];
        ::zerovec::VarZeroVec::Borrowed(core::mem::transmute(BYTES))
    })),
    few: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        static BYTES: &[u8] = &[
            1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 192u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8,
            4u8, 0u8, 0u8, 0u8,
        ];
        ::zerovec::VarZeroVec::Borrowed(core::mem::transmute(BYTES))
    })),
    many: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        static BYTES: &[u8] = &[
            1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 192u8, 0u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8,
            6u8, 0u8, 0u8, 0u8,
        ];
        ::zerovec::VarZeroVec::Borrowed(core::mem::transmute(BYTES))
    })),
};
static EN: DataStruct = &::icu_plurals::provider::PluralRulesV1 {
    zero: None,
    one: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        static BYTES: &[u8] = &[
            2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 13u8, 0u8, 0u8, 0u8, 192u8, 10u8, 0u8, 0u8,
            0u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 128u8, 100u8, 0u8, 0u8, 0u8, 11u8, 0u8,
            0u8, 0u8, 11u8, 0u8, 0u8, 0u8,
        ];
        ::zerovec::VarZeroVec::Borrowed(core::mem::transmute(BYTES))
    })),
    two: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        static BYTES: &[u8] = &[
            2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 13u8, 0u8, 0u8, 0u8, 192u8, 10u8, 0u8, 0u8,
            0u8, 2u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 128u8, 100u8, 0u8, 0u8, 0u8, 12u8, 0u8,
            0u8, 0u8, 12u8, 0u8, 0u8, 0u8,
        ];
        ::zerovec::VarZeroVec::Borrowed(core::mem::transmute(BYTES))
    })),
    few: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        static BYTES: &[u8] = &[
            2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 13u8, 0u8, 0u8, 0u8, 192u8, 10u8, 0u8, 0u8,
            0u8, 3u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 128u8, 100u8, 0u8, 0u8, 0u8, 13u8, 0u8,
            0u8, 0u8, 13u8, 0u8, 0u8, 0u8,
        ];
        ::zerovec::VarZeroVec::Borrowed(core::mem::transmute(BYTES))
    })),
    many: None,
};
static FIL_FR: DataStruct = &::icu_plurals::provider::PluralRulesV1 {
    zero: None,
    one: Some(::icu_plurals::rules::runtime::ast::Rule(unsafe {
        static BYTES: &[u8] = &[
            1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 192u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8,
            1u8, 0u8, 0u8, 0u8,
        ];
        ::zerovec::VarZeroVec::Borrowed(core::mem::transmute(BYTES))
    })),
    two: None,
    few: None,
    many: None,
};
