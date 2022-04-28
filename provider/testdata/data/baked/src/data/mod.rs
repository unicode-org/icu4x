// GENERATED MODULE. DO NOT EDIT

use icu_provider::prelude::*;
use writeable::Writeable;
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
impl ResourceProvider<::icu_list::provider::AndListV1Marker> for &'static StaticDataProvider {
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<::icu_list::provider::AndListV1Marker>, DataError> {
        let index = self
            .list_and_1
            .binary_search_by_key(&&*req.options.write_to_string(), |(k, _)| k)
            .map_err(|_| DataErrorKind::MissingResourceOptions.into_error())?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(zerofrom::ZeroFrom::zero_from(
                self.list_and_1[index].1,
            ))),
        })
    }
}
impl ResourceProvider<::icu_list::provider::OrListV1Marker> for &'static StaticDataProvider {
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<::icu_list::provider::OrListV1Marker>, DataError> {
        let index = self
            .list_or_1
            .binary_search_by_key(&&*req.options.write_to_string(), |(k, _)| k)
            .map_err(|_| DataErrorKind::MissingResourceOptions.into_error())?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(zerofrom::ZeroFrom::zero_from(
                self.list_or_1[index].1,
            ))),
        })
    }
}
impl ResourceProvider<::icu_list::provider::UnitListV1Marker> for &'static StaticDataProvider {
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<::icu_list::provider::UnitListV1Marker>, DataError> {
        let index = self
            .list_unit_1
            .binary_search_by_key(&&*req.options.write_to_string(), |(k, _)| k)
            .map_err(|_| DataErrorKind::MissingResourceOptions.into_error())?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(zerofrom::ZeroFrom::zero_from(
                self.list_unit_1[index].1,
            ))),
        })
    }
}
