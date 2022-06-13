// @generated
use icu_provider::prelude::*;
impl ResourceProvider<icu_properties::provider::SoftDottedV1Marker>
    for super::super::BakedDataProvider
{
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<icu_properties::provider::SoftDottedV1Marker>, DataError> {
        static VALUES: &[(&str, DataStruct)] = &[("und", UND)];
        #[allow(clippy::unwrap_used)]
        let value = VALUES
            .binary_search_by(|(k, _)| req.options.strict_cmp(k.as_bytes()).reverse())
            .map(|i| VALUES.get(i).unwrap().1)
            .map_err(|_| {
                DataErrorKind::MissingResourceOptions
                    .with_req(<icu_properties::provider::SoftDottedV1Marker>::KEY, req)
            })?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(zerofrom::ZeroFrom::zero_from(
                value,
            ))),
        })
    }
}
type DataStruct = &'static <icu_properties::provider::SoftDottedV1Marker as DataMarker>::Yokeable;
static UND: DataStruct = &::icu_properties::provider::UnicodePropertyV1 {
    inv_list: unsafe {
        #[allow(unused_unsafe)]
        ::icu_uniset::UnicodeSet::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    105u8, 0u8, 0u8, 0u8, 107u8, 0u8, 0u8, 0u8, 47u8, 1u8, 0u8, 0u8, 48u8, 1u8,
                    0u8, 0u8, 73u8, 2u8, 0u8, 0u8, 74u8, 2u8, 0u8, 0u8, 104u8, 2u8, 0u8, 0u8,
                    105u8, 2u8, 0u8, 0u8, 157u8, 2u8, 0u8, 0u8, 158u8, 2u8, 0u8, 0u8, 178u8, 2u8,
                    0u8, 0u8, 179u8, 2u8, 0u8, 0u8, 243u8, 3u8, 0u8, 0u8, 244u8, 3u8, 0u8, 0u8,
                    86u8, 4u8, 0u8, 0u8, 87u8, 4u8, 0u8, 0u8, 88u8, 4u8, 0u8, 0u8, 89u8, 4u8, 0u8,
                    0u8, 98u8, 29u8, 0u8, 0u8, 99u8, 29u8, 0u8, 0u8, 150u8, 29u8, 0u8, 0u8, 151u8,
                    29u8, 0u8, 0u8, 164u8, 29u8, 0u8, 0u8, 165u8, 29u8, 0u8, 0u8, 168u8, 29u8, 0u8,
                    0u8, 169u8, 29u8, 0u8, 0u8, 45u8, 30u8, 0u8, 0u8, 46u8, 30u8, 0u8, 0u8, 203u8,
                    30u8, 0u8, 0u8, 204u8, 30u8, 0u8, 0u8, 113u8, 32u8, 0u8, 0u8, 114u8, 32u8, 0u8,
                    0u8, 72u8, 33u8, 0u8, 0u8, 74u8, 33u8, 0u8, 0u8, 124u8, 44u8, 0u8, 0u8, 125u8,
                    44u8, 0u8, 0u8, 34u8, 212u8, 1u8, 0u8, 36u8, 212u8, 1u8, 0u8, 86u8, 212u8, 1u8,
                    0u8, 88u8, 212u8, 1u8, 0u8, 138u8, 212u8, 1u8, 0u8, 140u8, 212u8, 1u8, 0u8,
                    190u8, 212u8, 1u8, 0u8, 192u8, 212u8, 1u8, 0u8, 242u8, 212u8, 1u8, 0u8, 244u8,
                    212u8, 1u8, 0u8, 38u8, 213u8, 1u8, 0u8, 40u8, 213u8, 1u8, 0u8, 90u8, 213u8,
                    1u8, 0u8, 92u8, 213u8, 1u8, 0u8, 142u8, 213u8, 1u8, 0u8, 144u8, 213u8, 1u8,
                    0u8, 194u8, 213u8, 1u8, 0u8, 196u8, 213u8, 1u8, 0u8, 246u8, 213u8, 1u8, 0u8,
                    248u8, 213u8, 1u8, 0u8, 42u8, 214u8, 1u8, 0u8, 44u8, 214u8, 1u8, 0u8, 94u8,
                    214u8, 1u8, 0u8, 96u8, 214u8, 1u8, 0u8, 146u8, 214u8, 1u8, 0u8, 148u8, 214u8,
                    1u8, 0u8, 26u8, 223u8, 1u8, 0u8, 27u8, 223u8, 1u8, 0u8,
                ])
            },
            47usize,
        )
    },
};
