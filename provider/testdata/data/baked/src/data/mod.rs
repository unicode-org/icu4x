// GENERATED MODULE. DO NOT EDIT
type DataStruct<M> = &'static <M as ::icu_provider::DataMarker>::Yokeable;
type Options = &'static str;
type Data<M> = &'static [(Options, DataStruct<M>)];
pub struct BakedDataProvider {
    list_and_1: Data<::icu_list::provider::AndListV1Marker>,
    list_or_1: Data<::icu_list::provider::OrListV1Marker>,
    list_unit_1: Data<::icu_list::provider::UnitListV1Marker>,
}
mod list;
pub static PROVIDER: &BakedDataProvider = &BakedDataProvider {
    list_and_1: list::and_1::VALUES,
    list_or_1: list::or_1::VALUES,
    list_unit_1: list::unit_1::VALUES,
};
use icu_provider::prelude::*;
macro_rules! provider_impl {
    ($ marker : ty , $ field_name : ident) => {
        impl ResourceProvider<$marker> for &'static BakedDataProvider {
            fn load_resource(&self, req: &DataRequest) -> Result<DataResponse<$marker>, DataError> {
                let value = self
                    .$field_name
                    .binary_search_by(|(k, _)| req.options.cmp_bytes(k.as_bytes()).reverse())
                    .map(|i| self.$field_name.get(i).unwrap().1)
                    .map_err(|_| {
                        DataErrorKind::MissingResourceOptions.with_req(<$marker>::KEY, req)
                    })?;
                Ok(DataResponse {
                    metadata: DataResponseMetadata::default(),
                    payload: Some(DataPayload::from_owned(zerofrom::ZeroFrom::zero_from(
                        value,
                    ))),
                })
            }
        }
    };
}
provider_impl!(::icu_list::provider::AndListV1Marker, list_and_1);
provider_impl!(::icu_list::provider::OrListV1Marker, list_or_1);
provider_impl!(::icu_list::provider::UnitListV1Marker, list_unit_1);
