// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::mem::{discriminant, Discriminant};

use bitmask_enum::bitmask;

use icu_locid::Locale;

/// Trait for providing person name data.
pub trait PersonName {
    /// Return the name locale of person name.
    fn name_locale(&self) -> Option<&Locale>;

    /// return the preferred order of person name.
    fn preferred_order(&self) -> Option<&PreferredOrder>;

    /// return the value of the given field name, it *must* match the name field.
    fn get(&self, field: &NameField) -> Option<&str>;

    /// Return all available name field.
    fn available_name_fields(&self) -> Vec<&NameField>;
}

/// Implementation for all PersonName struct.
///
/// see https://www.unicode.org/reports/tr35/tr35-personNames.html#person-name-object
impl dyn PersonName {
    /// Validate that the provided fields are valid.
    /// If the person name is not valid, it will not be formatted.
    ///
    /// This function will always be called before formatting.
    pub fn is_valid(&self) -> bool {
        self.available_name_fields()
            .into_iter()
            .all(|field| field.is_valid())
            && (self.has_name_field(discriminant(&NameField::Given(None)))
                || self.has_name_field(discriminant(&NameField::Surname(None))))
    }

    /// Returns true if person have the name field, regardless of the modifier.
    pub fn has_name_field(&self, lookup_name_field: Discriminant<NameField>) -> bool {
        self.available_name_fields()
            .into_iter()
            .any(|field| discriminant(field) == lookup_name_field)
    }

    /// Returns true if person have the name field matching the type and modifier.
    pub fn has_name_field_with_modifier(&self, lookup_name_field: &NameField) -> bool {
        self.available_name_fields()
            .into_iter()
            .any(|field| field == lookup_name_field)
    }
}

///
/// Error handling for the person name formatter.
///
#[derive(Debug)]
pub enum PersonNamesFormatterError {
    ParseError(String),
    InvalidPersonName,
}

/// Field Modifiers.
///
/// https://www.unicode.org/reports/tr35/tr35-personNames.html#modifiers
#[bitmask(u32)]
pub enum FieldModifier {
    Informal,
    Prefix,
    Core,
    AllCaps,
    InitialCap,
    Initial,
    Monogram,
}

/// List all incompatible combination of field modifier.
const MUTUALLY_EXCLUSIVE_FIELD_MODIFIERS: [&FieldModifier; 3] = [
    &FieldModifier::Prefix.or(FieldModifier::Core),
    &FieldModifier::AllCaps.or(FieldModifier::InitialCap),
    &FieldModifier::Initial.or(FieldModifier::Monogram),
];

impl FieldModifier {
    /// Returns true if the field modifier is valid.
    pub fn is_valid(&self) -> bool {
        MUTUALLY_EXCLUSIVE_FIELD_MODIFIERS
            .into_iter()
            .all(|field| !self.contains(*field))
    }
}

/// Name Fields defined by Unicode specifications.
///
/// https://www.unicode.org/reports/tr35/tr35-personNames.html#fields
#[derive(Ord, Eq, PartialEq, PartialOrd)]
pub enum NameField {
    Title(Option<FieldModifier>),
    Given(Option<FieldModifier>),
    Given2(Option<FieldModifier>),
    Surname(Option<FieldModifier>),
    Surname2(Option<FieldModifier>),
    Generation(Option<FieldModifier>),
    Credentials(Option<FieldModifier>),
}

/// NameField helper functions.
impl NameField {
    ///
    /// Returns the field modifier of the NameField.
    ///
    pub fn get_field_modifier(&self) -> Option<&FieldModifier> {
        match self {
            NameField::Title(field_modifier) => field_modifier,
            NameField::Given(field_modifier) => field_modifier,
            NameField::Given2(field_modifier) => field_modifier,
            NameField::Surname(field_modifier) => field_modifier,
            NameField::Surname2(field_modifier) => field_modifier,
            NameField::Generation(field_modifier) => field_modifier,
            NameField::Credentials(field_modifier) => field_modifier,
        }
        .as_ref()
    }
    ///
    /// Returns true if the name field is valid.
    ///
    pub fn is_valid(&self) -> bool {
        match self.get_field_modifier() {
            None => true,
            Some(field_modifier) => field_modifier.is_valid(),
        }
    }
}

/// An enum to specify the preferred field order for the name.
///
/// https://www.unicode.org/reports/tr35/tr35-personNames.html#person-name-object
#[derive(Debug, Eq, PartialEq)]
pub enum PreferredOrder {
    Default,
    GivenFirst,
    SurnameFirst,
}

/// Formatting Order
#[derive(Debug)]
pub enum FormattingOrder {
    GivenFirst,
    SurnameFirst,
    Sorting,
}

/// Formatting Length
#[derive(Debug)]
pub enum FormattingLength {
    Short,
    Medium,
    Long,
}

/// Formatting Usage
#[derive(Debug)]
pub enum FormattingUsage {
    Addressing,
    Referring,
    Monogram,
}

/// Formatting Formality
#[derive(Debug)]
pub enum FormattingFormality {
    Formal,
    Informal,
}

/// Person name formatter options.
#[derive(Debug)]
pub struct PersonNamesFormatterOptions {
    pub target_locale: Locale,
    pub order: FormattingOrder,
    pub length: FormattingLength,
    pub usage: FormattingUsage,
    pub formality: FormattingFormality,
}
