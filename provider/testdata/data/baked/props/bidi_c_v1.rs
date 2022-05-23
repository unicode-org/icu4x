// @generated
use icu_provider::prelude::*;
impl ResourceProvider<icu_properties::provider::BidiControlV1Marker>
    for super::super::BakedDataProvider
{
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<icu_properties::provider::BidiControlV1Marker>, DataError> {
        static VALUES: &[(&str, DataStruct)] = &[("und", UND)];
        #[allow(clippy::unwrap_used)]
        let value = VALUES
            .binary_search_by(|(k, _)| req.options.cmp_bytes(k.as_bytes()).reverse())
            .map(|i| VALUES.get(i).unwrap().1)
            .map_err(|_| {
                DataErrorKind::MissingResourceOptions
                    .with_req(<icu_properties::provider::BidiControlV1Marker>::KEY, req)
            })?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(zerofrom::ZeroFrom::zero_from(
                value,
            ))),
        })
    }
}
type DataStruct = &'static <icu_properties::provider::BidiControlV1Marker as DataMarker>::Yokeable;
static UND: DataStruct = &::icu_properties::provider::UnicodePropertyV1 {
    inv_list: unsafe {
        #[allow(unused_unsafe)]
        ::icu_uniset::UnicodeSet::from_parts_unchecked(
            unsafe {
                static DATA: &[u8] = &[
                    28u8, 6u8, 0u8, 0u8, 29u8, 6u8, 0u8, 0u8, 14u8, 32u8, 0u8, 0u8, 16u8, 32u8,
                    0u8, 0u8, 42u8, 32u8, 0u8, 0u8, 47u8, 32u8, 0u8, 0u8, 102u8, 32u8, 0u8, 0u8,
                    106u8, 32u8, 0u8, 0u8,
                ];
                let (data, mut metadata): (usize, usize) = core::mem::transmute(DATA);
                metadata /= 4usize;
                zerovec::ZeroVec::Borrowed(core::mem::transmute((data, metadata)))
            },
            12usize,
        )
    },
};
