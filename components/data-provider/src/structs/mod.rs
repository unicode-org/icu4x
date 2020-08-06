pub mod decimal;
pub mod plurals;

/// Gets a locale-invariant default struct given a data key in this module's category.
/// For example, if the data key is `plurals/cardinal@1`, a Response with an object of type
/// PluralRuleStringsV1 will be returned.
///
/// # Example
///
/// ```
/// use icu_data_provider::prelude::*;
/// use icu_data_provider::structs::get_invariant;
/// use icu_data_provider::structs::plurals::PluralRuleStringsV1;
/// use std::any::TypeId;
///
/// assert_eq!(
///     get_invariant(&icu_data_key!(plurals: cardinal@1))
///         .map(|response| response.get_payload_type_id()),
///     Some(TypeId::of::<PluralRuleStringsV1>())
/// );
/// ```
#[cfg(feature = "invariant")]
pub fn get_invariant(
    data_key: &crate::data_key::DataKey,
) -> Option<crate::data_provider::Response<'static>> {
    if let Some(response) = decimal::get_invariant(data_key) {
        Some(response)
    } else if let Some(response) = plurals::get_invariant(data_key) {
        Some(response)
    } else {
        None
    }
}
