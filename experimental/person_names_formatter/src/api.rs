// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::mem::{discriminant, Discriminant};

use bitmask_enum::bitmask;

use icu_locid::Locale;

///
/// Error handling for the person name formatter.
///
pub enum PersonNamesFormatterError {
    ParseError(String),
}

/// Public trait for the person name formatter.
///
/// A PersonNameFormatter is expected to only handle a single target locale.
pub trait PersonNamesFormatter {
    ///
    /// Format the provided person name into the (optional target locale)
    ///
    /// Returns the formatted string when successful, an error otherwise.
    fn format(&self, person_name: &dyn PersonName) -> Result<String, PersonNamesFormatterError>;
}

/// Field Modifiers.
///
/// https://www.unicode.org/reports/tr35/tr35-personNames.html#modifiers
#[bitmask(u8)]
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
            .all(|field| !self.has(field))
    }

    /// Returns true if field modifier has the fields.
    fn has(&self, field_modifier: &FieldModifier) -> bool {
        return self.bits() & field_modifier.bits() == field_modifier.bits();
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

/// Person name trait.
pub trait PersonName {
    /// Returns the locale of the name-- that is, the language or country of origin for the person being named.
    /// An implementing class is allowed to return None here to indicate the name's locale is unknown.
    fn name_locale(&self) -> Option<&Locale> {
        None
    }

    /// Returns the preferred field order for the name.
    ///
    /// PersonName objects should generally return NONE, allowing the PersonNameFormatter to deduce the proper field order based on the locales of the name
    /// and the formatter. But this can be used to force a particular field order, generally in cases
    /// where the deduction logic in PersonNameFormatter would guess wrong.
    fn preferred_order(&self) -> Option<&PreferredOrder> {
        None
    }

    /// Returns the data associated with the name field.
    fn get(&self, field: &NameField) -> Option<&str>;

    /// Returns all the provided field from the underlying implementation.
    fn all_provided_fields(&self) -> Vec<&NameField>;
}

impl dyn PersonName {
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
}
