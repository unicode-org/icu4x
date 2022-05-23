// @generated
use icu_provider::prelude::*;
impl ResourceProvider<icu_properties::provider::WhiteSpaceV1Marker>
    for super::super::BakedDataProvider
{
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<icu_properties::provider::WhiteSpaceV1Marker>, DataError> {
        static VALUES: &[(&str, DataStruct)] = &[("und", UND)];
        #[allow(clippy::unwrap_used)]
        let value = VALUES
            .binary_search_by(|(k, _)| req.options.cmp_bytes(k.as_bytes()).reverse())
            .map(|i| VALUES.get(i).unwrap().1)
            .map_err(|_| {
                DataErrorKind::MissingResourceOptions
                    .with_req(<icu_properties::provider::WhiteSpaceV1Marker>::KEY, req)
            })?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(zerofrom::ZeroFrom::zero_from(
                value,
            ))),
        })
    }
}
type DataStruct = &'static <icu_properties::provider::WhiteSpaceV1Marker as DataMarker>::Yokeable;
static UND: DataStruct = &::icu_properties::provider::UnicodePropertyV1 {
    inv_list: unsafe {
        #[allow(unused_unsafe)]
        ::icu_uniset::UnicodeSet::from_parts_unchecked(
            unsafe {
                static DATA: &[u8] = &[
                    9u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8, 32u8, 0u8, 0u8, 0u8, 33u8, 0u8, 0u8,
                    0u8, 133u8, 0u8, 0u8, 0u8, 134u8, 0u8, 0u8, 0u8, 160u8, 0u8, 0u8, 0u8, 161u8,
                    0u8, 0u8, 0u8, 128u8, 22u8, 0u8, 0u8, 129u8, 22u8, 0u8, 0u8, 0u8, 32u8, 0u8,
                    0u8, 11u8, 32u8, 0u8, 0u8, 40u8, 32u8, 0u8, 0u8, 42u8, 32u8, 0u8, 0u8, 47u8,
                    32u8, 0u8, 0u8, 48u8, 32u8, 0u8, 0u8, 95u8, 32u8, 0u8, 0u8, 96u8, 32u8, 0u8,
                    0u8, 0u8, 48u8, 0u8, 0u8, 1u8, 48u8, 0u8, 0u8,
                ];
                let (data, mut metadata): (usize, usize) = core::mem::transmute(DATA);
                metadata /= 4usize;
                zerovec::ZeroVec::Borrowed(core::mem::transmute((data, metadata)))
            },
            25usize,
        )
    },
};
