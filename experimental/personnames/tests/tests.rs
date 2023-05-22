// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::btree_map::BTreeMap;
use std::mem::discriminant;

use icu_locid::locale;
use icu_personnames::api::FieldModifier;
use icu_personnames::api::FieldModifierMask;
use icu_personnames::api::NameField;
use icu_personnames::api::PersonName;
use icu_personnames::api::PersonNamesFormatterError;
use icu_personnames::api::PreferredOrder;
use icu_personnames::provided_struct::DefaultPersonName;

#[test]
fn test_field_modifier_person_name_structure() -> Result<(), PersonNamesFormatterError> {
    let mut person_data: BTreeMap<NameField, String> = BTreeMap::new();
    person_data.insert(NameField::Given(None), String::from("Henry"));
    person_data.insert(NameField::Surname(None), String::from("Jekyll"));
    person_data.insert(
        NameField::Surname(Some(FieldModifier::Informal as FieldModifierMask)),
        String::from("Hide"),
    );

    let person_name: &dyn PersonName = &DefaultPersonName::try_new_unstable(
        person_data,
        Some(locale!("en")),
        Some(PreferredOrder::GivenFirst),
    )? as &dyn PersonName;

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
        .has_name_field_with_modifier(&NameField::Surname(Some(FieldModifier::AllCaps as FieldModifierMask))));

    // get
    assert_eq!(person_name.get(&NameField::Given(None)), Some("Henry"));
    assert_eq!(person_name.get(&NameField::Surname(None)), Some("Jekyll"));
    assert_eq!(
        person_name.get(&NameField::Surname(Some(FieldModifier::Informal as FieldModifierMask))),
        Some("Hide")
    );
    Ok(())
}

#[test]
fn test_field_modifier_person_name_should_have_given_or_surname() {
    let mut person_data: BTreeMap<NameField, String> = BTreeMap::new();
    person_data.insert(NameField::Title(None), String::from("Dr"));

    let person_name = DefaultPersonName::try_new_unstable(person_data, None, None);
    assert!(person_name.is_err());
}
