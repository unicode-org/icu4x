// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).


use lazy_static::lazy_static;
use regex::CaptureMatches;
use regex::Regex;

use crate::api::{FieldLength, FieldModifier, NameField, PersonName};

const CLDR_INITIAL_INITIAL_PATTERN: &str = r"\{(?P<initial_position>\d+)}(?P<trailing>[^{]+)?";
const CLDR_INITIAL_SEQUENCE_PATTERN: &str = r"\{(?P<initial_position>\d+)}(?P<trailing>[^{]+)?";

lazy_static! {
    static ref INITIAL_PATTERN: Regex = Regex::new(CLDR_INITIAL_INITIAL_PATTERN).unwrap();
    static ref INITIAL_PATTERN_SEQUENCE: Regex = Regex::new(CLDR_INITIAL_SEQUENCE_PATTERN).unwrap();
}

///
/// Derive missing initials from the name
///
/// https://www.unicode.org/reports/tr35/tr35-personNames.html#derive-initials
///
pub(crate) fn derive_missing_initials(
    person_name: &dyn PersonName,
    requested_field: &NameField,
    initial_pattern: &str,
    initial_sequence_pattern: &str,
) -> String {
    if person_name.has_name_field(requested_field) {
        return person_name.get(requested_field).to_string();
    }
    if requested_field.modifier.has_field(FieldModifier::Initial) {
        let full_value = Vec::from_iter(
            person_name
                .get(&NameField {
                    kind: requested_field.kind,
                    modifier: requested_field.modifier.with_length(FieldLength::Auto),
                })
                .split(' ')
                .map(|s| s.trim()),
        );
        let captures: CaptureMatches = INITIAL_PATTERN.captures_iter(initial_pattern);
        let captures_sequence: CaptureMatches =
            INITIAL_PATTERN_SEQUENCE.captures_iter(initial_sequence_pattern);

        let mut initials = vec![];
        for capture in captures {
            let value_iter = full_value.iter();
            let position: usize = capture
                .name("initial_position")
                .map(|m| m.as_str().parse().unwrap())
                .unwrap();

            let trailing = capture.name("trailing").map_or("", |m| m.as_str());
            for name_part in value_iter {
                let initial = name_part
                    .chars()
                    .nth(position)
                    .map(|i| i.to_string() + trailing)
                    // Default to empty string if not found.
                    .unwrap_or(String::from(""));
                initials.push(initial);
            }
        }
        let mut build_initials = vec![];

        for capture_sequence in captures_sequence {
            let position: usize = capture_sequence
                .name("initial_position")
                .map(|m| m.as_str().parse().unwrap())
                .unwrap();
            let trailing_sequence = capture_sequence.name("trailing").map_or("", |m| m.as_str());
            build_initials.push(
                initials
                    .get(position)
                    .map(|s| s.to_string() + trailing_sequence)
                    .unwrap_or(trailing_sequence.to_string()),
            )
        }
        return build_initials.join("").trim().to_string();
    }
    // If it had the field, it would have been returned earlier.
    String::from("")
}

#[cfg(test)]
mod tests {
    use icu_locid::locale;
    use litemap::LiteMap;

    use crate::api::NameFieldKind::Given;
    use crate::api::{FieldLength, FieldModifierSet, NameField, PersonNamesFormatterError};
    use crate::provided_struct::DefaultPersonName;

    #[test]
    fn test_single_initial() -> Result<(), PersonNamesFormatterError> {
        let mut person_data = LiteMap::new();
        person_data.insert(
            NameField {
                kind: Given,
                modifier: FieldModifierSet::default(),
            },
            String::from("Henry"),
        );
        let person_name = DefaultPersonName::new(person_data, Some(locale!("en")), None)?;
        let requested_field = NameField {
            kind: Given,
            modifier: FieldModifierSet::length(FieldLength::Initial),
        };
        let result = crate::derive_missing_initials::derive_missing_initials(
            &person_name,
            &requested_field,
            "{0}.",
            "{0} {1}",
        );
        assert_eq!(result, "H.");
        Ok(())
    }

    #[test]
    fn test_multi_initial() -> Result<(), PersonNamesFormatterError> {
        let mut person_data = LiteMap::new();
        person_data.insert(
            NameField {
                kind: Given,
                modifier: FieldModifierSet::default(),
            },
            String::from("Mary Jane"),
        );
        let person_name = DefaultPersonName::new(person_data, Some(locale!("en")), None)?;
        let requested_field = NameField {
            kind: Given,
            modifier: FieldModifierSet::length(FieldLength::Initial),
        };
        let result = crate::derive_missing_initials::derive_missing_initials(
            &person_name,
            &requested_field,
            "{0}.",
            "{0} {1}",
        );
        assert_eq!(result, "M. J.");
        Ok(())
    }

    #[test]
    fn test_multi_3_initial_should_still_only_be_2() -> Result<(), PersonNamesFormatterError> {
        let mut person_data = LiteMap::new();
        person_data.insert(
            NameField {
                kind: Given,
                modifier: FieldModifierSet::default(),
            },
            String::from("Mary Jane Anne"),
        );
        let person_name = DefaultPersonName::new(person_data, Some(locale!("en")), None)?;
        let requested_field = NameField {
            kind: Given,
            modifier: FieldModifierSet::length(FieldLength::Initial),
        };
        let result = crate::derive_missing_initials::derive_missing_initials(
            &person_name,
            &requested_field,
            "{0}.",
            "{0} {1}",
        );
        assert_eq!(result, "M. J.");
        Ok(())
    }
}
