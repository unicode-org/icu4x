// @generated
use icu_provider::prelude::*;
impl ResourceProvider<icu_properties::provider::EmojiComponentV1Marker>
    for super::super::BakedDataProvider
{
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<icu_properties::provider::EmojiComponentV1Marker>, DataError> {
        static VALUES: &[(&str, DataStruct)] = &[("und", UND)];
        #[allow(clippy::unwrap_used)]
        let value = VALUES
            .binary_search_by(|(k, _)| req.options.strict_cmp(k.as_bytes()).reverse())
            .map(|i| VALUES.get(i).unwrap().1)
            .map_err(|_| {
                DataErrorKind::MissingResourceOptions
                    .with_req(<icu_properties::provider::EmojiComponentV1Marker>::KEY, req)
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
    &'static <icu_properties::provider::EmojiComponentV1Marker as DataMarker>::Yokeable;
static UND: DataStruct = &::icu_properties::provider::UnicodePropertyV1::InversionList(unsafe {
    #[allow(unused_unsafe)]
    ::icu_uniset::UnicodeSet::from_parts_unchecked(
        unsafe {
            ::zerovec::ZeroVec::from_bytes_unchecked(&[
                35u8, 0u8, 0u8, 0u8, 36u8, 0u8, 0u8, 0u8, 42u8, 0u8, 0u8, 0u8, 43u8, 0u8, 0u8, 0u8,
                48u8, 0u8, 0u8, 0u8, 58u8, 0u8, 0u8, 0u8, 13u8, 32u8, 0u8, 0u8, 14u8, 32u8, 0u8,
                0u8, 227u8, 32u8, 0u8, 0u8, 228u8, 32u8, 0u8, 0u8, 15u8, 254u8, 0u8, 0u8, 16u8,
                254u8, 0u8, 0u8, 230u8, 241u8, 1u8, 0u8, 0u8, 242u8, 1u8, 0u8, 251u8, 243u8, 1u8,
                0u8, 0u8, 244u8, 1u8, 0u8, 176u8, 249u8, 1u8, 0u8, 180u8, 249u8, 1u8, 0u8, 32u8,
                0u8, 14u8, 0u8, 128u8, 0u8, 14u8, 0u8,
            ])
        },
        146usize,
    )
});
