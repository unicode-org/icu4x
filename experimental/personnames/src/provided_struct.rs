use std::collections::BTreeMap;
use std::convert::identity;

use icu_locid::Locale;

use crate::api::{NameField, PersonName, PersonNamesFormatterError, PreferredOrder};

///
/// Simple person name, does not provide Field modifier support. All fields modifier are assumed to
/// be equivalent to None.
///
/// use FieldModifierSupportPersonName struct for field modifier support.
///
#[derive(Default)]
pub struct SimplePersonName {
    given: Option<String>,
    surname: Option<String>,
    title: Option<String>,
    given2: Option<String>,
    surname2: Option<String>,
    generation: Option<String>,
    credentials: Option<String>,
    locale: Option<Locale>,
    preferred_order: Option<PreferredOrder>,
    available_name_fields: Vec<NameField>,
}

impl SimplePersonName {
    pub fn try_new_unstable(
        locale: Option<Locale>,
        preferred_order: Option<PreferredOrder>,
        given: Option<String>,
        surname: Option<String>,
        title: Option<String>,
        given2: Option<String>,
        surname2: Option<String>,
        generation: Option<String>,
        credentials: Option<String>,
    ) -> Result<SimplePersonName, PersonNamesFormatterError> {
        if given.is_none() && surname.is_none() {
            return Err(PersonNamesFormatterError::InvalidPersonName);
        }
        let available_name_fields = vec![
            given.as_ref().map(|_value| NameField::Given(None)),
            surname.as_ref().map(|_value| NameField::Surname(None)),
            title.as_ref().map(|_value| NameField::Title(None)),
            given2.as_ref().map(|_value| NameField::Given2(None)),
            surname2.as_ref().map(|_value| NameField::Surname2(None)),
            generation
                .as_ref()
                .map(|_value| NameField::Generation(None)),
            credentials
                .as_ref()
                .map(|_value| NameField::Credentials(None)),
        ]
        .into_iter()
        .filter_map(identity)
        .collect();
        Ok(SimplePersonName {
            given,
            surname,
            title,
            given2,
            surname2,
            generation,
            credentials,
            locale,
            preferred_order,
            available_name_fields,
        })
    }
}

impl PersonName for SimplePersonName {
    fn name_locale(&self) -> Option<&Locale> {
        self.locale.as_ref()
    }

    fn preferred_order(&self) -> Option<&PreferredOrder> {
        self.preferred_order.as_ref()
    }

    fn get(&self, field: &NameField) -> Option<&str> {
        if field.get_field_modifier().is_some() {
            return None;
        }
        return match field {
            NameField::Title(_) => &self.title,
            NameField::Given(_) => &self.given,
            NameField::Given2(_) => &self.given2,
            NameField::Surname(_) => &self.surname,
            NameField::Surname2(_) => &self.surname2,
            NameField::Generation(_) => &self.generation,
            NameField::Credentials(_) => &self.credentials,
        }
        .as_ref()
        .map(String::as_str);
    }

    fn available_name_fields(&self) -> Vec<&NameField> {
        self.available_name_fields.iter().collect()
    }
}

///
/// FieldModifierSupportPersonName provide the full capability of PersonName as per specifications.
///
pub struct FieldModifierSupportPersonName {
    person_data: BTreeMap<NameField, String>,
    locale: Option<Locale>,
    preferred_order: Option<PreferredOrder>,
}

impl PersonName for FieldModifierSupportPersonName {
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
impl FieldModifierSupportPersonName {
    ///
    /// Returns a new person name structure.
    ///
    pub fn try_new_unstable(
        person_data: BTreeMap<NameField, String>,
        locale: Option<Locale>,
        preferred_order: Option<PreferredOrder>,
    ) -> Result<FieldModifierSupportPersonName, PersonNamesFormatterError> {
        let result = FieldModifierSupportPersonName {
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
