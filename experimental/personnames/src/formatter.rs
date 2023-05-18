// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::collections::btree_map::BTreeMap;
use std::mem::{discriminant, Discriminant};

use icu_locid::Locale;
use icu_provider::{DataError, DataLocale, DataPayload, DataProvider, DataRequest};

use crate::api::NameField;
use crate::api::PersonNamesFormatterError;
use crate::api::PreferredOrder;
use crate::provider::PersonNamesFormattingDefinitionV1Marker;

///
/// Default internal structure for person name.
///
pub struct PersonName {
    person_data: BTreeMap<NameField, String>,
    locale: Option<Locale>,
    preferred_order: Option<PreferredOrder>,
}

impl PersonName {
    /// Validate that the provided fields are valid.
    pub fn is_valid(&self) -> bool {
        self.all_provided_fields()
            .into_iter()
            .all(|field| field.is_valid())
            && (self.has_name_field(discriminant(&NameField::Given(None)))
                || self.has_name_field(discriminant(&NameField::Surname(None))))
    }

    /// Returns true if person have the name field, regardless of the modifier.
    pub fn has_name_field(&self, lookup_name_field: Discriminant<NameField>) -> bool {
        self.all_provided_fields()
            .into_iter()
            .any(|field| discriminant(field) == lookup_name_field)
    }

    /// Returns true if person have the name field matching the type and modifier.
    pub fn has_name_field_with_modifier(&self, lookup_name_field: &NameField) -> bool {
        self.all_provided_fields()
            .into_iter()
            .any(|field| field == lookup_name_field)
    }

    pub fn name_locale(&self) -> Option<&Locale> {
        self.locale.as_ref()
    }

    pub fn preferred_order(&self) -> Option<&PreferredOrder> {
        self.preferred_order.as_ref()
    }

    pub fn get(&self, field: &NameField) -> Option<&str> {
        self.person_data.get(field).map(String::as_ref)
    }

    pub fn all_provided_fields(&self) -> Vec<&NameField> {
        self.person_data.keys().collect()
    }

    ///
    /// Returns a new person name structure.
    ///
    pub fn try_new_unstable(
        person_data: BTreeMap<NameField, String>,
        locale: Option<Locale>,
        preferred_order: Option<PreferredOrder>,
    ) -> Result<PersonName, String> {
        let result = PersonName {
            person_data,
            locale,
            preferred_order,
        };
        if !result.is_valid() {
            return Err(String::from(
                "Trying to build an invalid DefaultPersonName !",
            ));
        }
        Ok(result)
    }
}

/// Placeholder for person name formatter options.
pub struct PersonNamesFormatterOptions;

/// Immutable structure loaded with formatter configs
pub struct PersonNamesFormatter {
    data_payload: DataPayload<PersonNamesFormattingDefinitionV1Marker>,
    options: PersonNamesFormatterOptions,
}

impl PersonNamesFormatter {
    pub fn try_new_unstable<D: DataProvider<PersonNamesFormattingDefinitionV1Marker>>(
        data_provider: &D,
        locale: &DataLocale,
        options: PersonNamesFormatterOptions,
    ) -> Result<PersonNamesFormatter, DataError>
    where
        D: DataProvider<PersonNamesFormattingDefinitionV1Marker>,
    {
        let data_payload = data_provider
            .load(DataRequest {
                locale,
                metadata: Default::default(),
            })?
            .take_payload()?;

        Ok(Self {
            data_payload,
            options,
        })
    }

    pub fn format(&self, _person_name: PersonName) -> Result<String, PersonNamesFormatterError> {
        Err(PersonNamesFormatterError::ParseError(String::from(
            "Unimplemented Person name formatter ",
        )))
    }
}
