// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::collections::btree_map::BTreeMap;

use icu_locid::Locale;

use crate::api::NameField;
use crate::api::PersonName;
use crate::api::PersonNamesFormatter;
use crate::api::PersonNamesFormatterError;
use crate::api::PreferredOrder;
use crate::provider::PersonNamesFormattingDefinitionV1;

///
/// Default internal structure for person name.
///
pub struct DefaultPersonName {
    person_data: BTreeMap<NameField, String>,
    locale: Option<Locale>,
    preferred_order: Option<PreferredOrder>,
}

///
/// Returns a new person name structure.
///
pub fn new_person_name(
    person_data: BTreeMap<NameField, String>,
    locale: Option<Locale>,
    preferred_order: Option<PreferredOrder>,
) -> Result<Box<dyn PersonName>, String> {
    let result = DefaultPersonName {
        person_data,
        locale,
        preferred_order,
    };
    if !(&result as &dyn PersonName).is_valid() {
        return Err(String::from(
            "Trying to build an invalid DefaultPersonName !",
        ));
    }
    Ok(Box::new(result))
}

impl PersonName for DefaultPersonName {
    fn name_locale(&self) -> Option<&Locale> {
        self.locale.as_ref()
    }

    fn preferred_order(&self) -> Option<&PreferredOrder> {
        self.preferred_order.as_ref()
    }

    fn get(&self, field: &NameField) -> Option<&str> {
        self.person_data.get(field).map(String::as_ref)
    }

    fn all_provided_fields(&self) -> Vec<&NameField> {
        self.person_data.keys().collect()
    }
}

/// Immutable structure loaded with formatter configs
pub struct DefaultPersonNamesFormatter<'data> {
    config: PersonNamesFormattingDefinitionV1<'data>,
}

impl PersonNamesFormatter for DefaultPersonNamesFormatter<'_> {
    fn format(&self, _person_name: &dyn PersonName) -> Result<String, PersonNamesFormatterError> {
        let pattern_size = self.config.person_names_patterns.len();
        Err(PersonNamesFormatterError::ParseError(format!(
            "Unimplemented but formatter have {} patterns configured.",
            pattern_size
        )))
    }
}

pub fn new_formatter<'data>(
    config: PersonNamesFormattingDefinitionV1<'data>,
) -> Result<Box<dyn PersonNamesFormatter + 'data>, String> {
    Ok(Box::new(DefaultPersonNamesFormatter { config }))
}
