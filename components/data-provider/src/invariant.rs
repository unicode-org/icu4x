// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use crate::error::Error;
use crate::iter::DataEntryCollection;
use crate::prelude::*;
use crate::structs;
use icu_locale::LanguageIdentifier;
use std::fmt;

/// Package a data struct T implementing Default as a DataResponse.
pub(crate) fn make_inv_response<T>() -> Option<DataResponse<'static>>
where
    T: 'static + Clone + erased_serde::Serialize + fmt::Debug + Default,
{
    Some(
        DataResponseBuilder {
            data_langid: LanguageIdentifier::default(),
        }
        .with_owned_payload(T::default()),
    )
}

/// A locale-invariant data provider. Sometimes useful for testing. Not intended to be used in
/// production environments.
///
/// The objects returned by InvariantDataProvider are guaranteed to conform to the correct struct
/// definition, so InvariantDataProvider can also be used to validate unknown data providers.
///
/// # Example
///
/// ```
/// use icu_data_provider::prelude::*;
/// use icu_data_provider::InvariantDataProvider;
/// use icu_data_provider::iter::DataEntryCollection;
/// use icu_locale_macros::langid;
///
/// let provider = InvariantDataProvider;
/// let expected_entries = vec![DataEntry {
///     variant: None,
///     langid: langid!("und"),
/// }];
/// let actual_entries: Vec<DataEntry> = provider
///     .iter_for_key(&icu_data_key!(plurals: cardinal@1))
///     .unwrap()
///     .collect();
/// assert_eq!(&expected_entries, &actual_entries);
/// ```
pub struct InvariantDataProvider;

impl DataProvider<'static> for InvariantDataProvider {
    fn load<'a>(&'a self, req: &DataRequest) -> Result<DataResponse<'static>, Error> {
        structs::get_invariant(&req.data_key).ok_or(Error::UnsupportedDataKey(req.data_key))
    }
}

impl DataEntryCollection for InvariantDataProvider {
    fn iter_for_key(
        &self,
        data_key: &DataKey,
    ) -> Result<Box<dyn Iterator<Item = DataEntry>>, Error> {
        structs::get_invariant(data_key).ok_or(Error::UnsupportedDataKey(*data_key))?;
        let list: Vec<DataEntry> = vec![DataEntry {
            variant: None,
            langid: LanguageIdentifier::default(),
        }];
        Ok(Box::new(list.into_iter()))
    }
}

#[test]
fn test_basic() {
    let provider = InvariantDataProvider;
    let response = provider
        .load(&DataRequest {
            data_key: icu_data_key!(plurals: cardinal@1),
            data_entry: DataEntry {
                variant: None,
                langid: LanguageIdentifier::default(),
            },
        })
        .unwrap();
    let plurals_data: &structs::plurals::PluralRuleStringsV1 = response.borrow_payload().unwrap();
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
