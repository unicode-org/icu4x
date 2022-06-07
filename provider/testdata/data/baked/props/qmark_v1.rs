// @generated
use icu_provider::prelude::*;
impl ResourceProvider<icu_properties::provider::QuotationMarkV1Marker>
    for super::super::BakedDataProvider
{
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<icu_properties::provider::QuotationMarkV1Marker>, DataError> {
        static VALUES: &[(&str, DataStruct)] = &[("und", UND)];
        #[allow(clippy::unwrap_used)]
        let value = VALUES
            .binary_search_by(|(k, _)| req.options.strict_cmp(k.as_bytes()).reverse())
            .map(|i| VALUES.get(i).unwrap().1)
            .map_err(|_| {
                DataErrorKind::MissingResourceOptions
                    .with_req(<icu_properties::provider::QuotationMarkV1Marker>::KEY, req)
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
    &'static <icu_properties::provider::QuotationMarkV1Marker as DataMarker>::Yokeable;
static UND: DataStruct = &::icu_properties::provider::UnicodePropertyV1 {
    inv_list: unsafe {
        #[allow(unused_unsafe)]
        ::icu_uniset::UnicodeSet::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    34u8, 0u8, 0u8, 0u8, 35u8, 0u8, 0u8, 0u8, 39u8, 0u8, 0u8, 0u8, 40u8, 0u8, 0u8,
                    0u8, 171u8, 0u8, 0u8, 0u8, 172u8, 0u8, 0u8, 0u8, 187u8, 0u8, 0u8, 0u8, 188u8,
                    0u8, 0u8, 0u8, 24u8, 32u8, 0u8, 0u8, 32u8, 32u8, 0u8, 0u8, 57u8, 32u8, 0u8,
                    0u8, 59u8, 32u8, 0u8, 0u8, 66u8, 46u8, 0u8, 0u8, 67u8, 46u8, 0u8, 0u8, 12u8,
                    48u8, 0u8, 0u8, 16u8, 48u8, 0u8, 0u8, 29u8, 48u8, 0u8, 0u8, 32u8, 48u8, 0u8,
                    0u8, 65u8, 254u8, 0u8, 0u8, 69u8, 254u8, 0u8, 0u8, 2u8, 255u8, 0u8, 0u8, 3u8,
                    255u8, 0u8, 0u8, 7u8, 255u8, 0u8, 0u8, 8u8, 255u8, 0u8, 0u8, 98u8, 255u8, 0u8,
                    0u8, 100u8, 255u8, 0u8, 0u8,
                ])
            },
            30usize,
        )
    },
};
