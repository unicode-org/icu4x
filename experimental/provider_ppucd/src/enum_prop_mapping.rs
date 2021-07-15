// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_uniset::enum_props::*;

//
// Single getter function for enumerated property name:
// Enum prop name string -> Rust enum
//

fn get_enum_property_enum(name: &str) -> Option<EnumeratedProperty> {
    Some(match name {
        "gc" => EnumeratedProperty::GeneralCategory,
        _ => return None,
    })
}

//
// Getter function per enumerated property:
// Enum prop val string -> Rust enum
//

fn get_general_category_enum(name: &str) -> Option<GeneralCategory> {
    Some(match name {
        // "C" => GeneralCategory::Other,
        "Cc" => GeneralCategory::Control,
        "Cf" => GeneralCategory::Format,
        "Cn" => GeneralCategory::Unassigned,
        "Co" => GeneralCategory::PrivateUse,
        "Cs" => GeneralCategory::Surrogate,
        // "L" => GeneralCategory::Letter,
        // "LC" => GeneralCategory::CasedLetter,
        "Ll" => GeneralCategory::LowercaseLetter,
        "Lm" => GeneralCategory::ModifierLetter,
        "Lo" => GeneralCategory::OtherLetter,
        "Lt" => GeneralCategory::TitlecaseLetter,
        "Lu" => GeneralCategory::UppercaseLetter,
        // "M" => GeneralCategory::CombiningMark,
        "Mc" => GeneralCategory::SpacingMark,
        "Me" => GeneralCategory::EnclosingMark,
        "Mn" => GeneralCategory::NonspacingMark,
        // "N" => GeneralCategory::Number,
        "Nd" => GeneralCategory::Digit,
        "Nl" => GeneralCategory::LetterNumber,
        "No" => GeneralCategory::OtherNumber,
        // "P" => GeneralCategory::Punctutation,
        "Pc" => GeneralCategory::ConnectorPunctuation,
        "Pd" => GeneralCategory::DashPunctuation,
        "Pe" => GeneralCategory::ClosePunctuation,
        "Pf" => GeneralCategory::FinalPunctuation,
        "Pi" => GeneralCategory::InitialPunctuation,
        "Po" => GeneralCategory::OtherPunctuation,
        "Ps" => GeneralCategory::OpenPunctuation,
        // "S" => GeneralCategory::Symbol,
        "Sc" => GeneralCategory::CurrencySymbol,
        "Sk" => GeneralCategory::ModifierSymbol,
        "Sm" => GeneralCategory::MathSymbol,
        "So" => GeneralCategory::OtherSymbol,
        // "Z" => GeneralCategory::Separator,
        "Zl" => GeneralCategory::LineSeparator,
        "Zp" => GeneralCategory::ParagraphSeparator,
        "Zs" => GeneralCategory::SpaceSeparator,
        _ => return None,
    })
}
