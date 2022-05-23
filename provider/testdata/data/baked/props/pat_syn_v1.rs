// @generated
use icu_provider::prelude::*;
impl ResourceProvider<icu_properties::provider::PatternSyntaxV1Marker>
    for super::super::BakedDataProvider
{
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<icu_properties::provider::PatternSyntaxV1Marker>, DataError> {
        static VALUES: &[(&str, DataStruct)] = &[("und", UND)];
        #[allow(clippy::unwrap_used)]
        let value = VALUES
            .binary_search_by(|(k, _)| req.options.cmp_bytes(k.as_bytes()).reverse())
            .map(|i| VALUES.get(i).unwrap().1)
            .map_err(|_| {
                DataErrorKind::MissingResourceOptions
                    .with_req(<icu_properties::provider::PatternSyntaxV1Marker>::KEY, req)
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
    &'static <icu_properties::provider::PatternSyntaxV1Marker as DataMarker>::Yokeable;
static UND: DataStruct = &::icu_properties::provider::UnicodePropertyV1 {
    inv_list: unsafe {
        #[allow(unused_unsafe)]
        ::icu_uniset::UnicodeSet::from_parts_unchecked(
            unsafe {
                static DATA: &[u8] = &[
                    33u8, 0u8, 0u8, 0u8, 48u8, 0u8, 0u8, 0u8, 58u8, 0u8, 0u8, 0u8, 65u8, 0u8, 0u8,
                    0u8, 91u8, 0u8, 0u8, 0u8, 95u8, 0u8, 0u8, 0u8, 96u8, 0u8, 0u8, 0u8, 97u8, 0u8,
                    0u8, 0u8, 123u8, 0u8, 0u8, 0u8, 127u8, 0u8, 0u8, 0u8, 161u8, 0u8, 0u8, 0u8,
                    168u8, 0u8, 0u8, 0u8, 169u8, 0u8, 0u8, 0u8, 170u8, 0u8, 0u8, 0u8, 171u8, 0u8,
                    0u8, 0u8, 173u8, 0u8, 0u8, 0u8, 174u8, 0u8, 0u8, 0u8, 175u8, 0u8, 0u8, 0u8,
                    176u8, 0u8, 0u8, 0u8, 178u8, 0u8, 0u8, 0u8, 182u8, 0u8, 0u8, 0u8, 183u8, 0u8,
                    0u8, 0u8, 187u8, 0u8, 0u8, 0u8, 188u8, 0u8, 0u8, 0u8, 191u8, 0u8, 0u8, 0u8,
                    192u8, 0u8, 0u8, 0u8, 215u8, 0u8, 0u8, 0u8, 216u8, 0u8, 0u8, 0u8, 247u8, 0u8,
                    0u8, 0u8, 248u8, 0u8, 0u8, 0u8, 16u8, 32u8, 0u8, 0u8, 40u8, 32u8, 0u8, 0u8,
                    48u8, 32u8, 0u8, 0u8, 63u8, 32u8, 0u8, 0u8, 65u8, 32u8, 0u8, 0u8, 84u8, 32u8,
                    0u8, 0u8, 85u8, 32u8, 0u8, 0u8, 95u8, 32u8, 0u8, 0u8, 144u8, 33u8, 0u8, 0u8,
                    96u8, 36u8, 0u8, 0u8, 0u8, 37u8, 0u8, 0u8, 118u8, 39u8, 0u8, 0u8, 148u8, 39u8,
                    0u8, 0u8, 0u8, 44u8, 0u8, 0u8, 0u8, 46u8, 0u8, 0u8, 128u8, 46u8, 0u8, 0u8, 1u8,
                    48u8, 0u8, 0u8, 4u8, 48u8, 0u8, 0u8, 8u8, 48u8, 0u8, 0u8, 33u8, 48u8, 0u8, 0u8,
                    48u8, 48u8, 0u8, 0u8, 49u8, 48u8, 0u8, 0u8, 62u8, 253u8, 0u8, 0u8, 64u8, 253u8,
                    0u8, 0u8, 69u8, 254u8, 0u8, 0u8, 71u8, 254u8, 0u8, 0u8,
                ];
                let (data, mut metadata): (usize, usize) = core::mem::transmute(DATA);
                metadata /= 4usize;
                zerovec::ZeroVec::Borrowed(core::mem::transmute((data, metadata)))
            },
            2760usize,
        )
    },
};
