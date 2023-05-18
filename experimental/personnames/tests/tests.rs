// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::btree_map::BTreeMap;
use std::mem::discriminant;

use icu_locid::locale;
use icu_person_names_formatter::api::{FieldModifier, NameField, PreferredOrder};

#[test]
fn test_person_name_structure() -> Result<(), String> {
    let mut person_data: BTreeMap<NameField, String> = BTreeMap::new();
    person_data.insert(NameField::Given(None), String::from("Henry"));
    person_data.insert(NameField::Surname(None), String::from("Jekyll"));
    person_data.insert(
        NameField::Surname(Some(FieldModifier::Informal)),
        String::from("Hide"),
    );

    let person_name = icu_person_names_formatter::PersonName::try_new_unstable(
        person_data,
        Some(locale!("en")),
        Some(PreferredOrder::GivenFirst),
    )?;

    assert_eq!(person_name.name_locale(), Some(&locale!("en")));
    assert_eq!(
        person_name.preferred_order(),
        Some(&PreferredOrder::GivenFirst)
    );

    // has_name_field tests
    assert!(person_name.has_name_field(discriminant(&NameField::Surname(None))));
    assert!(!person_name.has_name_field(discriminant(&NameField::Surname2(None))));

    // has_name_field_with_modifier
    assert!(person_name.has_name_field_with_modifier(&NameField::Given(None)));
    assert!(!person_name.has_name_field_with_modifier(&NameField::Surname2(None)));
    assert!(!person_name
        .has_name_field_with_modifier(&NameField::Surname(Some(FieldModifier::AllCaps))));

    // get
    assert_eq!(person_name.get(&NameField::Surname(None)), Some("Jekyll"));
    assert_eq!(
        person_name.get(&NameField::Surname(Some(FieldModifier::Informal))),
        Some("Hide")
    );
    Ok(())
}

#[test]
fn test_person_name_should_have_given_or_surname() -> Result<(), String> {
    let mut person_data: BTreeMap<NameField, String> = BTreeMap::new();
    person_data.insert(NameField::Title(None), String::from("Dr"));

    let person_name =
        icu_person_names_formatter::PersonName::try_new_unstable(person_data, None, None);
    assert!(person_name.is_err());
    Ok(())
}
