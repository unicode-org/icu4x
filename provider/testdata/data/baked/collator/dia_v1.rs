// @generated
use icu_provider::prelude::*;
impl ResourceProvider<::icu_collator::provider::CollationDiacriticsV1Marker>
    for super::super::BakedDataProvider
{
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<::icu_collator::provider::CollationDiacriticsV1Marker>, DataError>
    {
        static VALUES: &[(&str, DataStruct)] = &[("und", UND)];
        #[allow(clippy::unwrap_used)]
        let value = VALUES
            .binary_search_by(|(k, _)| req.options.cmp_bytes(k.as_bytes()).reverse())
            .map(|i| VALUES.get(i).unwrap().1)
            .map_err(|_| {
                DataErrorKind::MissingResourceOptions.with_req(
                    <::icu_collator::provider::CollationDiacriticsV1Marker>::KEY,
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
    &'static <::icu_collator::provider::CollationDiacriticsV1Marker as DataMarker>::Yokeable;
static UND: DataStruct = &::icu_collator::provider::CollationDiacriticsV1 {
    ce32s: unsafe {
        ::zerovec::ZeroVec::from_bytes_unchecked(&[
            5u8, 138u8, 0u8, 0u8, 5u8, 136u8, 0u8, 0u8, 5u8, 142u8, 0u8, 0u8, 5u8, 154u8, 0u8, 0u8,
            5u8, 164u8, 0u8, 0u8, 5u8, 180u8, 0u8, 0u8, 5u8, 140u8, 0u8, 0u8, 5u8, 156u8, 0u8, 0u8,
            5u8, 150u8, 0u8, 0u8, 5u8, 182u8, 0u8, 0u8, 5u8, 146u8, 0u8, 0u8, 5u8, 152u8, 0u8, 0u8,
            5u8, 144u8, 0u8, 0u8, 5u8, 166u8, 0u8, 0u8, 5u8, 166u8, 0u8, 0u8, 5u8, 184u8, 0u8, 0u8,
            5u8, 186u8, 0u8, 0u8, 5u8, 188u8, 0u8, 0u8, 5u8, 166u8, 0u8, 0u8, 5u8, 132u8, 0u8, 0u8,
            5u8, 134u8, 0u8, 0u8, 5u8, 166u8, 0u8, 0u8, 5u8, 168u8, 0u8, 0u8, 5u8, 168u8, 0u8, 0u8,
            5u8, 168u8, 0u8, 0u8, 5u8, 168u8, 0u8, 0u8, 5u8, 166u8, 0u8, 0u8, 5u8, 190u8, 0u8, 0u8,
            5u8, 168u8, 0u8, 0u8, 5u8, 168u8, 0u8, 0u8, 5u8, 168u8, 0u8, 0u8, 5u8, 168u8, 0u8, 0u8,
            5u8, 168u8, 0u8, 0u8, 5u8, 192u8, 0u8, 0u8, 5u8, 194u8, 0u8, 0u8, 5u8, 196u8, 0u8, 0u8,
            5u8, 198u8, 0u8, 0u8, 5u8, 200u8, 0u8, 0u8, 5u8, 202u8, 0u8, 0u8, 5u8, 160u8, 0u8, 0u8,
            5u8, 162u8, 0u8, 0u8, 5u8, 168u8, 0u8, 0u8, 5u8, 168u8, 0u8, 0u8, 5u8, 168u8, 0u8, 0u8,
            5u8, 168u8, 0u8, 0u8, 5u8, 204u8, 0u8, 0u8, 5u8, 206u8, 0u8, 0u8, 5u8, 168u8, 0u8, 0u8,
            5u8, 208u8, 0u8, 0u8, 5u8, 210u8, 0u8, 0u8, 5u8, 130u8, 0u8, 0u8, 5u8, 168u8, 0u8, 0u8,
            5u8, 212u8, 0u8, 0u8, 5u8, 178u8, 0u8, 0u8, 5u8, 170u8, 0u8, 0u8, 5u8, 170u8, 0u8, 0u8,
            5u8, 158u8, 0u8, 0u8, 5u8, 214u8, 0u8, 0u8, 5u8, 168u8, 0u8, 0u8, 5u8, 168u8, 0u8, 0u8,
            5u8, 168u8, 0u8, 0u8, 5u8, 166u8, 0u8, 0u8, 5u8, 166u8, 0u8, 0u8, 5u8, 166u8, 0u8, 0u8,
            5u8, 138u8, 0u8, 0u8, 5u8, 136u8, 0u8, 0u8, 5u8, 148u8, 0u8, 0u8, 5u8, 132u8, 0u8, 0u8,
            1u8, 0u8, 0u8, 0u8, 5u8, 216u8, 0u8, 0u8, 5u8, 166u8, 0u8, 0u8, 5u8, 168u8, 0u8, 0u8,
            5u8, 168u8, 0u8, 0u8, 5u8, 168u8, 0u8, 0u8, 5u8, 166u8, 0u8, 0u8, 5u8, 166u8, 0u8, 0u8,
            5u8, 166u8, 0u8, 0u8, 5u8, 168u8, 0u8, 0u8, 5u8, 168u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            5u8, 166u8, 0u8, 0u8, 5u8, 166u8, 0u8, 0u8, 5u8, 166u8, 0u8, 0u8, 5u8, 168u8, 0u8, 0u8,
            5u8, 168u8, 0u8, 0u8, 5u8, 168u8, 0u8, 0u8, 5u8, 168u8, 0u8, 0u8, 5u8, 166u8, 0u8, 0u8,
            5u8, 218u8, 0u8, 0u8, 5u8, 168u8, 0u8, 0u8, 5u8, 168u8, 0u8, 0u8, 5u8, 166u8, 0u8, 0u8,
            5u8, 168u8, 0u8, 0u8, 5u8, 166u8, 0u8, 0u8, 5u8, 166u8, 0u8, 0u8, 5u8, 168u8, 0u8, 0u8,
            5u8, 220u8, 0u8, 0u8, 5u8, 222u8, 0u8, 0u8, 5u8, 168u8, 0u8, 0u8, 19u8, 5u8, 0u8, 42u8,
            19u8, 5u8, 0u8, 50u8, 19u8, 5u8, 0u8, 58u8, 19u8, 5u8, 0u8, 70u8, 19u8, 5u8, 0u8, 82u8,
            19u8, 5u8, 0u8, 46u8, 19u8, 5u8, 0u8, 48u8, 19u8, 5u8, 0u8, 56u8, 19u8, 5u8, 0u8, 66u8,
            19u8, 5u8, 0u8, 76u8, 19u8, 5u8, 0u8, 80u8, 19u8, 5u8, 0u8, 84u8, 19u8, 5u8, 0u8, 88u8,
        ])
    },
};
