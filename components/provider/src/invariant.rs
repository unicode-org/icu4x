// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use crate::data_receiver::DataReceiverThrowAway;
use crate::error::Error;
use crate::iter::IterableDataProvider;
use crate::prelude::*;
use crate::structs;
use std::borrow::Cow;
use std::fmt::Debug;

/// A locale-invariant data provider. Sometimes useful for testing. Not intended to be used in
/// production environments.
///
/// The objects returned by `InvariantDataProvider` are guaranteed to conform to the correct struct
/// definition, so `InvariantDataProvider` can also be used to validate unknown data providers.
///
/// # Example
///
/// ```
/// use icu_provider::prelude::*;
/// use icu_provider::structs;
/// use icu_provider::InvariantDataProvider;
/// use icu_locid_macros::langid;
///
/// let provider = InvariantDataProvider;
/// let expected_entries = vec![ResourceOptions::default()];
/// let actual_entries: Vec<ResourceOptions> = provider
///     .supported_options_for_key(&structs::plurals::key::CARDINAL_V1)
///     .unwrap()
///     .collect();
/// assert_eq!(&expected_entries, &actual_entries);
/// ```
pub struct InvariantDataProvider;

impl<'d, T> DataProvider<'d, 'static, T> for InvariantDataProvider
where
    T: serde::Deserialize<'static> + serde::Serialize + Clone + Debug + Default,
{
    fn load_payload(&self, _req: &DataRequest) -> Result<DataResponseWithPayload<'d, T>, Error> {
        Ok(DataResponseWithPayload {
            response: DataResponse::default(),
            payload: Some(Cow::Owned(T::default())),
        })
    }
}

impl<'d> ErasedDataProvider<'d> for InvariantDataProvider {
    fn load_to_receiver(
        &self,
        req: &DataRequest,
        receiver: &mut dyn DataReceiver<'d, 'static>,
    ) -> Result<DataResponse, Error> {
        structs::get_invariant(&req.resource_path.key, receiver)?;
        Ok(DataResponse::default())
    }
}

impl IterableDataProvider<'_> for InvariantDataProvider {
    fn supported_options_for_key(
        &self,
        resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, Error> {
        let mut receiver = DataReceiverThrowAway::default();
        structs::get_invariant(resc_key, &mut receiver)?;
        let list: Vec<ResourceOptions> = vec![ResourceOptions::default()];
        Ok(Box::new(list.into_iter()))
    }
}

#[test]
fn test_v2() {
    let provider = InvariantDataProvider;
    let mut receiver =
        DataReceiverForType::<structs::plurals::PluralRuleStringsV1> { payload: None };
    provider
        .load_to_receiver(
            &DataRequest {
                resource_path: ResourcePath {
                    key: structs::plurals::key::CARDINAL_V1,
                    options: Default::default(),
                },
            },
            &mut receiver,
        )
        .unwrap();
    let plurals_data: &structs::plurals::PluralRuleStringsV1 = &(receiver.payload.unwrap());
    assert_eq!(
        plurals_data,
        &structs::plurals::PluralRuleStringsV1 {
            zero: None,
            one: None,
            two: None,
            few: None,
            many: None,
        }
    );
}
