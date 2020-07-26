pub mod decimal;
pub mod plurals;

use crate::data_key::DataKey;
use std::any::TypeId;

/// Gets the expected type given a certain data key. For example, if the data key is
/// `plurals/cardinal@1`, the TypeId for PluralRuleStringsV1 will be returned.
///
/// # Example
///
/// ```
/// use icu_data_provider::icu_data_key;
/// use icu_data_provider::structs::get_type_id;
/// use icu_data_provider::structs::plurals::PluralRuleStringsV1;
/// use std::any::TypeId;
///
/// assert_eq!(
///     get_type_id(&icu_data_key!(plurals: cardinal@1)),
///     Some(TypeId::of::<PluralRuleStringsV1>())
/// );
/// ```
pub fn get_type_id(data_key: &DataKey) -> Option<TypeId> {
    if let Some(type_id) = decimal::get_type_id(data_key) {
        Some(type_id)
    } else if let Some(type_id) = plurals::get_type_id(data_key) {
        Some(type_id)
    } else {
        None
    }
}
