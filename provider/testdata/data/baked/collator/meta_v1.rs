// @generated
use icu_provider::prelude::*;
impl ResourceProvider<::icu_collator::provider::CollationMetadataV1Marker>
    for super::super::BakedDataProvider
{
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<::icu_collator::provider::CollationMetadataV1Marker>, DataError> {
        static VALUES: &[(&str, DataStruct)] = &[
            ("bn", BN_JA),
            ("es", ES_TR),
            ("ja", BN_JA),
            ("th", TH),
            ("tr", ES_TR),
            ("und", UND),
        ];
        #[allow(clippy::unwrap_used)]
        let value = VALUES
            .binary_search_by(|(k, _)| req.options.cmp_bytes(k.as_bytes()).reverse())
            .map(|i| VALUES.get(i).unwrap().1)
            .map_err(|_| {
                DataErrorKind::MissingResourceOptions.with_req(
                    <::icu_collator::provider::CollationMetadataV1Marker>::KEY,
                    req,
                )
            })?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(zerofrom::ZeroFrom::zero_from(
                value,
            ))),
        })
    }
}
type DataStruct =
    &'static <::icu_collator::provider::CollationMetadataV1Marker as DataMarker>::Yokeable;
static BN_JA: DataStruct = &::icu_collator::provider::CollationMetadataV1 { bits: 251658313u32 };
static ES_TR: DataStruct = &::icu_collator::provider::CollationMetadataV1 { bits: 251658249u32 };
static TH: DataStruct = &::icu_collator::provider::CollationMetadataV1 { bits: 251658825u32 };
static UND: DataStruct = &::icu_collator::provider::CollationMetadataV1 { bits: 251658241u32 };
