// GENERATED MODULE. DO NOT EDIT

use ::icu_provider::prelude::*;

pub struct StaticDataProvider {
    list_and_1: &'static [(
        &'static str,
        &'static <::icu_list::provider::AndListV1Marker as DataMarker>::Yokeable,
    )],
    list_or_1: &'static [(
        &'static str,
        &'static <::icu_list::provider::OrListV1Marker as DataMarker>::Yokeable,
    )],
    list_unit_1: &'static [(
        &'static str,
        &'static <::icu_list::provider::UnitListV1Marker as DataMarker>::Yokeable,
    )],
}
mod list_and_1;
mod list_or_1;
mod list_unit_1;
pub static PROVIDER: &StaticDataProvider = &StaticDataProvider {
    list_and_1: list_and_1::VALUES,
    list_or_1: list_or_1::VALUES,
    list_unit_1: list_unit_1::VALUES,
};
macro_rules! provider_impl {
    ($ marker : ty , $ field_name : ident) => {
        impl ResourceProvider<$marker> for &'static StaticDataProvider {
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
