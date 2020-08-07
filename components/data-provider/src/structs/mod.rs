pub mod decimal;
pub mod plurals;

#[cfg(feature = "invariant")]
use crate::prelude::*;

/// Gets a locale-invariant default struct given a data key in this module's category.
/// For example, if the data key is `plurals/cardinal@1`, a Response with an object of type
/// PluralRuleStringsV1 will be returned.
#[cfg(feature = "invariant")]
pub(crate) fn get_invariant(data_key: &DataKey) -> Option<DataResponse<'static>> {
    if let Some(response) = decimal::get_invariant(data_key) {
        Some(response)
    } else if let Some(response) = plurals::get_invariant(data_key) {
        Some(response)
    } else {
        None
    }
}
