// @generated
use icu_provider::prelude::*;
impl ResourceProvider<icu_properties::provider::DefaultIgnorableCodePointV1Marker>
    for super::super::BakedDataProvider
{
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<icu_properties::provider::DefaultIgnorableCodePointV1Marker>, DataError>
    {
        static VALUES: &[(&str, DataStruct)] = &[("und", UND)];
        #[allow(clippy::unwrap_used)]
        let value = VALUES
            .binary_search_by(|(k, _)| req.options.cmp_bytes(k.as_bytes()).reverse())
            .map(|i| VALUES.get(i).unwrap().1)
            .map_err(|_| {
                DataErrorKind::MissingResourceOptions.with_req(
                    <icu_properties::provider::DefaultIgnorableCodePointV1Marker>::KEY,
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
    &'static <icu_properties::provider::DefaultIgnorableCodePointV1Marker as DataMarker>::Yokeable;
static UND: DataStruct = &::icu_properties::provider::UnicodePropertyV1 {
    inv_list: unsafe {
        #[allow(unused_unsafe)]
        ::icu_uniset::UnicodeSet::from_parts_unchecked(
            unsafe {
                static DATA: &[u8] = &[
                    173u8, 0u8, 0u8, 0u8, 174u8, 0u8, 0u8, 0u8, 79u8, 3u8, 0u8, 0u8, 80u8, 3u8,
                    0u8, 0u8, 28u8, 6u8, 0u8, 0u8, 29u8, 6u8, 0u8, 0u8, 95u8, 17u8, 0u8, 0u8, 97u8,
                    17u8, 0u8, 0u8, 180u8, 23u8, 0u8, 0u8, 182u8, 23u8, 0u8, 0u8, 11u8, 24u8, 0u8,
                    0u8, 16u8, 24u8, 0u8, 0u8, 11u8, 32u8, 0u8, 0u8, 16u8, 32u8, 0u8, 0u8, 42u8,
                    32u8, 0u8, 0u8, 47u8, 32u8, 0u8, 0u8, 96u8, 32u8, 0u8, 0u8, 112u8, 32u8, 0u8,
                    0u8, 100u8, 49u8, 0u8, 0u8, 101u8, 49u8, 0u8, 0u8, 0u8, 254u8, 0u8, 0u8, 16u8,
                    254u8, 0u8, 0u8, 255u8, 254u8, 0u8, 0u8, 0u8, 255u8, 0u8, 0u8, 160u8, 255u8,
                    0u8, 0u8, 161u8, 255u8, 0u8, 0u8, 240u8, 255u8, 0u8, 0u8, 249u8, 255u8, 0u8,
                    0u8, 160u8, 188u8, 1u8, 0u8, 164u8, 188u8, 1u8, 0u8, 115u8, 209u8, 1u8, 0u8,
                    123u8, 209u8, 1u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 16u8, 14u8, 0u8,
                ];
                let (data, mut metadata): (usize, usize) = core::mem::transmute(DATA);
                metadata /= 4usize;
                zerovec::ZeroVec::Borrowed(core::mem::transmute((data, metadata)))
            },
            4174usize,
        )
    },
};
