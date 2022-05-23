// @generated
use icu_provider::prelude::*;
impl ResourceProvider<icu_properties::provider::ExtenderV1Marker>
    for super::super::BakedDataProvider
{
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<icu_properties::provider::ExtenderV1Marker>, DataError> {
        static VALUES: &[(&str, DataStruct)] = &[("und", UND)];
        #[allow(clippy::unwrap_used)]
        let value = VALUES
            .binary_search_by(|(k, _)| req.options.cmp_bytes(k.as_bytes()).reverse())
            .map(|i| VALUES.get(i).unwrap().1)
            .map_err(|_| {
                DataErrorKind::MissingResourceOptions
                    .with_req(<icu_properties::provider::ExtenderV1Marker>::KEY, req)
            })?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(zerofrom::ZeroFrom::zero_from(
                value,
            ))),
        })
    }
}
type DataStruct = &'static <icu_properties::provider::ExtenderV1Marker as DataMarker>::Yokeable;
static UND: DataStruct = &::icu_properties::provider::UnicodePropertyV1 {
    inv_list: unsafe {
        #[allow(unused_unsafe)]
        ::icu_uniset::UnicodeSet::from_parts_unchecked(
            unsafe {
                static DATA: &[u8] = &[
                    183u8, 0u8, 0u8, 0u8, 184u8, 0u8, 0u8, 0u8, 208u8, 2u8, 0u8, 0u8, 210u8, 2u8,
                    0u8, 0u8, 64u8, 6u8, 0u8, 0u8, 65u8, 6u8, 0u8, 0u8, 250u8, 7u8, 0u8, 0u8,
                    251u8, 7u8, 0u8, 0u8, 85u8, 11u8, 0u8, 0u8, 86u8, 11u8, 0u8, 0u8, 70u8, 14u8,
                    0u8, 0u8, 71u8, 14u8, 0u8, 0u8, 198u8, 14u8, 0u8, 0u8, 199u8, 14u8, 0u8, 0u8,
                    10u8, 24u8, 0u8, 0u8, 11u8, 24u8, 0u8, 0u8, 67u8, 24u8, 0u8, 0u8, 68u8, 24u8,
                    0u8, 0u8, 167u8, 26u8, 0u8, 0u8, 168u8, 26u8, 0u8, 0u8, 54u8, 28u8, 0u8, 0u8,
                    55u8, 28u8, 0u8, 0u8, 123u8, 28u8, 0u8, 0u8, 124u8, 28u8, 0u8, 0u8, 5u8, 48u8,
                    0u8, 0u8, 6u8, 48u8, 0u8, 0u8, 49u8, 48u8, 0u8, 0u8, 54u8, 48u8, 0u8, 0u8,
                    157u8, 48u8, 0u8, 0u8, 159u8, 48u8, 0u8, 0u8, 252u8, 48u8, 0u8, 0u8, 255u8,
                    48u8, 0u8, 0u8, 21u8, 160u8, 0u8, 0u8, 22u8, 160u8, 0u8, 0u8, 12u8, 166u8, 0u8,
                    0u8, 13u8, 166u8, 0u8, 0u8, 207u8, 169u8, 0u8, 0u8, 208u8, 169u8, 0u8, 0u8,
                    230u8, 169u8, 0u8, 0u8, 231u8, 169u8, 0u8, 0u8, 112u8, 170u8, 0u8, 0u8, 113u8,
                    170u8, 0u8, 0u8, 221u8, 170u8, 0u8, 0u8, 222u8, 170u8, 0u8, 0u8, 243u8, 170u8,
                    0u8, 0u8, 245u8, 170u8, 0u8, 0u8, 112u8, 255u8, 0u8, 0u8, 113u8, 255u8, 0u8,
                    0u8, 129u8, 7u8, 1u8, 0u8, 131u8, 7u8, 1u8, 0u8, 93u8, 19u8, 1u8, 0u8, 94u8,
                    19u8, 1u8, 0u8, 198u8, 21u8, 1u8, 0u8, 201u8, 21u8, 1u8, 0u8, 152u8, 26u8, 1u8,
                    0u8, 153u8, 26u8, 1u8, 0u8, 66u8, 107u8, 1u8, 0u8, 68u8, 107u8, 1u8, 0u8,
                    224u8, 111u8, 1u8, 0u8, 226u8, 111u8, 1u8, 0u8, 227u8, 111u8, 1u8, 0u8, 228u8,
                    111u8, 1u8, 0u8, 60u8, 225u8, 1u8, 0u8, 62u8, 225u8, 1u8, 0u8, 68u8, 233u8,
                    1u8, 0u8, 71u8, 233u8, 1u8, 0u8,
                ];
                let (data, mut metadata): (usize, usize) = core::mem::transmute(DATA);
                metadata /= 4usize;
                zerovec::ZeroVec::Borrowed(core::mem::transmute((data, metadata)))
            },
            50usize,
        )
    },
};
