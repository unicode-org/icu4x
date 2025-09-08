// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::prelude::*;
// use icu::datetime::provider::neo::*;
use icu::decimal::provider::*;

/// A data provider that returns _something_ for structs it knows about.
///
/// Similar to EmptyProvider but returns Ok with a default value.
pub(crate) struct DebugProvider;

impl<M: DataMarker> DataProvider<M> for DebugProvider
where
    M::DataStruct: Clone,
{
    fn load(&self, _req: DataRequest) -> Result<DataResponse<M>, DataError> {
        // Note: would use TypeId::of but it isn't yet stable as a const fn
        use core::any::{type_name, Any, TypeId};
        use icu::datetime::provider::neo::*;
        let type_id = TypeId::of::<M::DataStruct>();
        let data: Box<dyn Any> = if type_id == TypeId::of::<YearNames>() {
            Box::new(YearNames::FixedEras(Default::default()))
        } else if type_id == TypeId::of::<MonthNames>() {
            Box::new(MonthNames::Linear(Default::default()))
        } else if type_id == TypeId::of::<LinearNames>() {
            Box::new(LinearNames {
                names: Default::default(),
            })
        } else if type_id == TypeId::of::<[char; 10]>() {
            Box::new(['\0'; 10])
        } else if type_id == TypeId::of::<DecimalSymbols>() {
            Box::new(DecimalSymbols::new_en_for_testing())
        } else {
            panic!(
                "Don't how how to create for debug: {}",
                type_name::<M::DataStruct>()
            );
        };
        let data: Box<M::DataStruct> = data.downcast().unwrap();
        // TODO: Use Box::into_inner when stabilized
        let data: M::DataStruct = (*data).clone();
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(data),
        })
    }
}
