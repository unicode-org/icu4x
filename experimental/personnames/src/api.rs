// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::mem::Discriminant;

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

    /// Returns true if the provided field name is available.
    fn has_name_field(&self, lookup_name_field: Discriminant<NameField>) -> bool;

    /// Returns true if person have the name field matching the type and modifier.
    fn has_name_field_with_modifier(&self, lookup_name_field: &NameField) -> bool;
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
#[repr(u32)]
pub enum FieldModifier {
    Informal = 1 << 0,
    Prefix = 1 << 1,
    Core = 1 << 2,
    AllCaps = 1 << 3,
    InitialCap = 1 << 4,
    Initial = 1 << 5,
    Monogram = 1 << 6,
}

/// Field Modifiers Mask. (must be the same as FieldModifier repr)
pub type FieldModifierMask = u32;

/// List all incompatible combination of field modifier.
const MUTUALLY_EXCLUSIVE_FIELD_MODIFIERS: [FieldModifierMask; 3] = [
    FieldModifier::Prefix as FieldModifierMask | FieldModifier::Core as FieldModifierMask,
    FieldModifier::AllCaps as FieldModifierMask | FieldModifier::InitialCap as FieldModifierMask,
    FieldModifier::Initial as FieldModifierMask | FieldModifier::Monogram as FieldModifierMask,
];

/// Returns true if the field modifier is valid.
pub fn field_modifier_is_valid(mask: FieldModifierMask) -> bool {
    MUTUALLY_EXCLUSIVE_FIELD_MODIFIERS
        .into_iter()
        .all(|field| mask & field != field)
}

/// Name Fields defined by Unicode specifications.
///
/// https://www.unicode.org/reports/tr35/tr35-personNames.html#fields
#[derive(Ord, Eq, PartialEq, PartialOrd)]
pub enum NameField {
    Title(Option<FieldModifierMask>),
    Given(Option<FieldModifierMask>),
    Given2(Option<FieldModifierMask>),
    Surname(Option<FieldModifierMask>),
    Surname2(Option<FieldModifierMask>),
    Generation(Option<FieldModifierMask>),
    Credentials(Option<FieldModifierMask>),
}

/// NameField helper functions.
impl NameField {
    ///
    /// Returns the field modifier of the NameField.
    ///
    pub fn get_field_modifier(&self) -> Option<&FieldModifierMask> {
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
            Some(&field_modifier) => field_modifier_is_valid(field_modifier),
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
