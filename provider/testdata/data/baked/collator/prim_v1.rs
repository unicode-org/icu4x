// @generated
use icu_provider::prelude::*;
impl ResourceProvider<::icu_collator::provider::CollationSpecialPrimariesV1Marker>
    for super::super::BakedDataProvider
{
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<::icu_collator::provider::CollationSpecialPrimariesV1Marker>, DataError>
    {
        static VALUES: &[(&str, DataStruct)] = &[("und", UND)];
        #[allow(clippy::unwrap_used)]
        let value = VALUES
            .binary_search_by(|(k, _)| req.options.strict_cmp(k.as_bytes()).reverse())
            .map(|i| VALUES.get(i).unwrap().1)
            .map_err(|_| {
                DataErrorKind::MissingResourceOptions.with_req(
                    <::icu_collator::provider::CollationSpecialPrimariesV1Marker>::KEY,
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
    &'static <::icu_collator::provider::CollationSpecialPrimariesV1Marker as DataMarker>::Yokeable;
static UND: DataStruct = &::icu_collator::provider::CollationSpecialPrimariesV1 {
    last_primaries: unsafe {
        ::zerovec::ZeroVec::from_bytes_unchecked(&[6u8, 5u8, 0u8, 12u8, 137u8, 13u8, 0u8, 14u8])
    },
    numeric_primary: 15u8,
};
