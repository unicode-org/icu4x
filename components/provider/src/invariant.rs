// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use crate::data_receiver::DataReceiverThrowAway;
use crate::error::Error;
use crate::iter::DataEntryCollection;
use crate::prelude::*;
use crate::structs;
use icu_locid::LanguageIdentifier;

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
/// use icu_provider::iter::DataEntryCollection;
/// use icu_locid_macros::langid;
///
/// let provider = InvariantDataProvider;
/// let expected_entries = vec![DataEntry {
///     variant: None,
///     langid: langid!("und"),
/// }];
/// let actual_entries: Vec<DataEntry> = provider
///     .iter_for_key(&structs::plurals::key::CARDINAL_V1)
///     .unwrap()
///     .collect();
/// assert_eq!(&expected_entries, &actual_entries);
/// ```
pub struct InvariantDataProvider;

impl<'d> DataProviderV2<'d> for InvariantDataProvider {
    fn load_to_receiver(
        &self,
        req: &DataRequest,
        receiver: &mut dyn DataReceiver<'d, 'static>,
    ) -> Result<DataResponse, Error> {
        structs::get_invariant(&req.data_key, receiver)?;
        Ok(DataResponse {
            data_langid: LanguageIdentifier::default(),
        })
    }
}

impl DataEntryCollection for InvariantDataProvider {
    fn iter_for_key(
        &self,
        data_key: &DataKey,
    ) -> Result<Box<dyn Iterator<Item = DataEntry>>, Error> {
        let mut receiver = DataReceiverThrowAway::default();
        structs::get_invariant(data_key, &mut receiver)?;
        let list: Vec<DataEntry> = vec![DataEntry {
            variant: None,
            langid: LanguageIdentifier::default(),
        }];
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
                data_key: structs::plurals::key::CARDINAL_V1,
                data_entry: DataEntry {
                    variant: None,
                    langid: LanguageIdentifier::default(),
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
