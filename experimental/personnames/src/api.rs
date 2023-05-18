// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use bitmask_enum::bitmask;

///
/// Error handling for the person name formatter.
///
pub enum PersonNamesFormatterError {
    ParseError(String),
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
        self.bits() & field_modifier.bits() == field_modifier.bits()
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
