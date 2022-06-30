// @generated
use icu_provider::prelude::*;
impl ResourceProvider<icu_properties::provider::LogicalOrderExceptionV1Marker>
    for super::super::BakedDataProvider
{
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<icu_properties::provider::LogicalOrderExceptionV1Marker>, DataError>
    {
        static VALUES: &[(&str, DataStruct)] = &[("und", UND)];
        #[allow(clippy::unwrap_used)]
        let value = VALUES
            .binary_search_by(|(k, _)| req.options.strict_cmp(k.as_bytes()).reverse())
            .map(|i| VALUES.get(i).unwrap().1)
            .map_err(|_| {
                DataErrorKind::MissingResourceOptions.with_req(
                    <icu_properties::provider::LogicalOrderExceptionV1Marker>::KEY,
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
    &'static <icu_properties::provider::LogicalOrderExceptionV1Marker as DataMarker>::Yokeable;
static UND: DataStruct = &::icu_properties::provider::UnicodePropertyV1::InversionList(unsafe {
    #[allow(unused_unsafe)]
    ::icu_uniset::UnicodeSet::from_parts_unchecked(
        unsafe {
            ::zerovec::ZeroVec::from_bytes_unchecked(&[
                64u8, 14u8, 0u8, 0u8, 69u8, 14u8, 0u8, 0u8, 192u8, 14u8, 0u8, 0u8, 197u8, 14u8,
                0u8, 0u8, 181u8, 25u8, 0u8, 0u8, 184u8, 25u8, 0u8, 0u8, 186u8, 25u8, 0u8, 0u8,
                187u8, 25u8, 0u8, 0u8, 181u8, 170u8, 0u8, 0u8, 183u8, 170u8, 0u8, 0u8, 185u8,
                170u8, 0u8, 0u8, 186u8, 170u8, 0u8, 0u8, 187u8, 170u8, 0u8, 0u8, 189u8, 170u8, 0u8,
                0u8,
            ])
        },
        19usize,
    )
});
