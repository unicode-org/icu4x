// @generated
use icu_provider::prelude::*;
impl ResourceProvider<icu_properties::provider::NoncharacterCodePointV1Marker>
    for super::super::BakedDataProvider
{
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<icu_properties::provider::NoncharacterCodePointV1Marker>, DataError>
    {
        static VALUES: &[(&str, DataStruct)] = &[("und", UND)];
        #[allow(clippy::unwrap_used)]
        let value = VALUES
            .binary_search_by(|(k, _)| req.options.strict_cmp(k.as_bytes()).reverse())
            .map(|i| VALUES.get(i).unwrap().1)
            .map_err(|_| {
                DataErrorKind::MissingResourceOptions.with_req(
                    <icu_properties::provider::NoncharacterCodePointV1Marker>::KEY,
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
    &'static <icu_properties::provider::NoncharacterCodePointV1Marker as DataMarker>::Yokeable;
static UND: DataStruct = &::icu_properties::provider::UnicodePropertyV1::InversionList(unsafe {
    #[allow(unused_unsafe)]
    ::icu_uniset::UnicodeSet::from_parts_unchecked(
        unsafe {
            ::zerovec::ZeroVec::from_bytes_unchecked(&[
                208u8, 253u8, 0u8, 0u8, 240u8, 253u8, 0u8, 0u8, 254u8, 255u8, 0u8, 0u8, 0u8, 0u8,
                1u8, 0u8, 254u8, 255u8, 1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 254u8, 255u8, 2u8, 0u8, 0u8,
                0u8, 3u8, 0u8, 254u8, 255u8, 3u8, 0u8, 0u8, 0u8, 4u8, 0u8, 254u8, 255u8, 4u8, 0u8,
                0u8, 0u8, 5u8, 0u8, 254u8, 255u8, 5u8, 0u8, 0u8, 0u8, 6u8, 0u8, 254u8, 255u8, 6u8,
                0u8, 0u8, 0u8, 7u8, 0u8, 254u8, 255u8, 7u8, 0u8, 0u8, 0u8, 8u8, 0u8, 254u8, 255u8,
                8u8, 0u8, 0u8, 0u8, 9u8, 0u8, 254u8, 255u8, 9u8, 0u8, 0u8, 0u8, 10u8, 0u8, 254u8,
                255u8, 10u8, 0u8, 0u8, 0u8, 11u8, 0u8, 254u8, 255u8, 11u8, 0u8, 0u8, 0u8, 12u8,
                0u8, 254u8, 255u8, 12u8, 0u8, 0u8, 0u8, 13u8, 0u8, 254u8, 255u8, 13u8, 0u8, 0u8,
                0u8, 14u8, 0u8, 254u8, 255u8, 14u8, 0u8, 0u8, 0u8, 15u8, 0u8, 254u8, 255u8, 15u8,
                0u8, 0u8, 0u8, 16u8, 0u8,
            ])
        },
        64usize,
    )
});
