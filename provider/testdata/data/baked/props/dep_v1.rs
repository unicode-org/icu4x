// @generated
use icu_provider::prelude::*;
impl ResourceProvider<icu_properties::provider::DeprecatedV1Marker>
    for super::super::BakedDataProvider
{
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<icu_properties::provider::DeprecatedV1Marker>, DataError> {
        static VALUES: &[(&str, DataStruct)] = &[("und", UND)];
        #[allow(clippy::unwrap_used)]
        let value = VALUES
            .binary_search_by(|(k, _)| req.options.strict_cmp(k.as_bytes()).reverse())
            .map(|i| VALUES.get(i).unwrap().1)
            .map_err(|_| {
                DataErrorKind::MissingResourceOptions
                    .with_req(<icu_properties::provider::DeprecatedV1Marker>::KEY, req)
            })?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(zerofrom::ZeroFrom::zero_from(
                value,
            ))),
        })
    }
}
type DataStruct = &'static <icu_properties::provider::DeprecatedV1Marker as DataMarker>::Yokeable;
static UND: DataStruct = &::icu_properties::provider::UnicodePropertyV1 {
    inv_list: unsafe {
        #[allow(unused_unsafe)]
        ::icu_uniset::UnicodeSet::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    73u8, 1u8, 0u8, 0u8, 74u8, 1u8, 0u8, 0u8, 115u8, 6u8, 0u8, 0u8, 116u8, 6u8,
                    0u8, 0u8, 119u8, 15u8, 0u8, 0u8, 120u8, 15u8, 0u8, 0u8, 121u8, 15u8, 0u8, 0u8,
                    122u8, 15u8, 0u8, 0u8, 163u8, 23u8, 0u8, 0u8, 165u8, 23u8, 0u8, 0u8, 106u8,
                    32u8, 0u8, 0u8, 112u8, 32u8, 0u8, 0u8, 41u8, 35u8, 0u8, 0u8, 43u8, 35u8, 0u8,
                    0u8, 1u8, 0u8, 14u8, 0u8, 2u8, 0u8, 14u8, 0u8,
                ])
            },
            15usize,
        )
    },
};
