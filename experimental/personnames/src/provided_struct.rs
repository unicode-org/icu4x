// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::BTreeMap;

use icu_locid::Locale;

use crate::api::{NameField, PersonName, PersonNamesFormatterError, PreferredOrder};

///
/// FieldModifierSupportPersonName provide the full capability of PersonName as per specifications.
///
pub struct DefaultPersonName {
    person_data: BTreeMap<NameField, String>,
    locale: Option<Locale>,
    preferred_order: Option<PreferredOrder>,
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

    fn available_name_fields(&self) -> Vec<&NameField> {
        self.person_data.keys().collect()
    }
}

///
/// Default person name functions.
///
impl DefaultPersonName {
    ///
    /// Returns a new person name structure.
    ///
    pub fn try_new_unstable(
        person_data: BTreeMap<NameField, String>,
        locale: Option<Locale>,
        preferred_order: Option<PreferredOrder>,
    ) -> Result<DefaultPersonName, PersonNamesFormatterError> {
        let result = DefaultPersonName {
            person_data,
            locale,
            preferred_order,
        };
        if !(&result as &dyn PersonName).is_valid() {
            return Err(PersonNamesFormatterError::InvalidPersonName);
        }
        Ok(result)
    }
}
