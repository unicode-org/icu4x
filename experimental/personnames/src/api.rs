// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

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
    fn has_name_field_kind(&self, lookup_name_field: &NameFieldKind) -> bool;

    /// Returns true if person have the name field matching the type and modifier.
    fn has_name_field(&self, lookup_name_field: &NameField) -> bool;
}

///
/// Error handling for the person name formatter.
///
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum PersonNamesFormatterError {
    ParseError(String),
    InvalidPersonName,
}

/// Field Modifiers.
///
/// https://www.unicode.org/reports/tr35/tr35-personNames.html#modifiers
enum FieldModifier {
    None,
    Informal,
    Prefix,
    Core,
    AllCaps,
    InitialCap,
    Initial,
    Monogram,
}

impl FieldModifier {
    fn bit_value(&self) -> u32 {
        match &self {
            FieldModifier::None => 0,
            FieldModifier::Informal => 1 << 0,
            FieldModifier::Prefix => 1 << 1,
            FieldModifier::Core => 1 << 2,
            FieldModifier::AllCaps => 1 << 3,
            FieldModifier::InitialCap => 1 << 4,
            FieldModifier::Initial => 1 << 5,
            FieldModifier::Monogram => 1 << 6,
        }
    }
}

/// Field Modifiers Set. (must be the same as FieldModifier repr)
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct FieldModifierSet {
    value: u32,
}

pub enum FieldCapsStyle {
    Auto,
    AllCaps,
    InitialCap,
}

impl Into<FieldModifier> for FieldCapsStyle {
    fn into(self) -> FieldModifier {
        match self {
            FieldCapsStyle::Auto => FieldModifier::None,
            FieldCapsStyle::AllCaps => FieldModifier::AllCaps,
            FieldCapsStyle::InitialCap => FieldModifier::InitialCap,
        }
    }
}

pub enum FieldPart {
    Auto,
    Core,
    Prefix,
}

impl Into<FieldModifier> for FieldPart {
    fn into(self) -> FieldModifier {
        match self {
            FieldPart::Auto => FieldModifier::None,
            FieldPart::Core => FieldModifier::Core,
            FieldPart::Prefix => FieldModifier::Prefix,
        }
    }
}

pub enum FieldLength {
    Auto,
    Initial,
    Monogram,
}

impl Into<FieldModifier> for FieldLength {
    fn into(self) -> FieldModifier {
        match self {
            FieldLength::Auto => FieldModifier::None,
            FieldLength::Initial => FieldModifier::Initial,
            FieldLength::Monogram => FieldModifier::Monogram,
        }
    }
}

pub enum FieldFormality {
    Auto,
    Informal,
}

impl Into<FieldModifier> for FieldFormality {
    fn into(self) -> FieldModifier {
        match self {
            FieldFormality::Auto => FieldModifier::None,
            FieldFormality::Informal => FieldModifier::Informal,
        }
    }
}

impl FieldModifierSet {
    pub fn new(
        style: FieldCapsStyle,
        part: FieldPart,
        length: FieldLength,
        formality: FieldFormality,
    ) -> Self {
        let style_fm: FieldModifier = style.into();
        let part_fm: FieldModifier = part.into();
        let length_fm: FieldModifier = length.into();
        let formality_fm: FieldModifier = formality.into();
        FieldModifierSet {
            value: style_fm.bit_value()
                | part_fm.bit_value()
                | length_fm.bit_value()
                | formality_fm.bit_value(),
        }
    }
}

impl Default for FieldModifierSet {
    fn default() -> Self {
        Self::new(
            FieldCapsStyle::Auto,
            FieldPart::Auto,
            FieldLength::Auto,
            FieldFormality::Auto,
        )
    }
}

/// Name Fields defined by Unicode specifications.
///
/// https://www.unicode.org/reports/tr35/tr35-personNames.html#fields
#[derive(Eq, Ord, PartialOrd, PartialEq, Hash, Debug)]
pub struct NameField {
    pub kind: NameFieldKind,
    pub modifier: FieldModifierSet,
}

#[derive(Eq, Ord, PartialOrd, PartialEq, Hash, Debug)]
pub enum NameFieldKind {
    Title,
    Given,
    Given2,
    Surname,
    Surname2,
    Generation,
    Credentials,
}

/// An enum to specify the preferred field order for the name.
///
/// https://www.unicode.org/reports/tr35/tr35-personNames.html#person-name-object
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PreferredOrder {
    Default,
    GivenFirst,
    SurnameFirst,
}

/// Formatting Order
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum FormattingOrder {
    GivenFirst,
    SurnameFirst,
    Sorting,
}

/// Formatting Length
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum FormattingLength {
    Short,
    Medium,
    Long,
}

/// Formatting Usage
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum FormattingUsage {
    Addressing,
    Referring,
    Monogram,
}

/// Formatting Formality
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum FormattingFormality {
    Formal,
    Informal,
}

/// Person name formatter options.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct PersonNamesFormatterOptions {
    pub target_locale: Locale,
    pub order: FormattingOrder,
    pub length: FormattingLength,
    pub usage: FormattingUsage,
    pub formality: FormattingFormality,
}
