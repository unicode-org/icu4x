// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locid::locale;
use icu_personnames::api::FieldModifierSet;
use icu_personnames::api::NameFieldKind;
use icu_personnames::api::PersonNamesFormatterError;
use icu_personnames::api::PreferredOrder;
use icu_personnames::api::{FieldCapsStyle, NameField};
use icu_personnames::api::{FieldFormality, FieldLength, FieldPart, PersonName};
use icu_personnames::provided_struct::DefaultPersonName;
use litemap::LiteMap;

#[test]
fn test_field_modifier_person_name_structure() -> Result<(), PersonNamesFormatterError> {
    let mut person_data: LiteMap<NameField, String> = LiteMap::new();
    person_data.insert(
        NameField {
            kind: NameFieldKind::Given,
            modifier: FieldModifierSet::default(),
        },
        String::from("Henry"),
    );
    person_data.insert(
        NameField {
            kind: NameFieldKind::Surname,
            modifier: FieldModifierSet::default(),
        },
        String::from("Jekyll"),
    );
    person_data.insert(
        NameField {
            kind: NameFieldKind::Surname,
            modifier: FieldModifierSet::new(
                FieldCapsStyle::Auto,
                FieldPart::Auto,
                FieldLength::Auto,
                FieldFormality::Informal,
            ),
        },
        String::from("Hide"),
    );

    let person_name: &dyn PersonName = &DefaultPersonName::new(
        person_data,
        Some(locale!("en")),
        Some(PreferredOrder::GivenFirst),
    )? as &dyn PersonName;

    assert_eq!(person_name.name_locale(), Some(&locale!("en")));
    assert_eq!(
        person_name.preferred_order(),
        Some(&PreferredOrder::GivenFirst)
    );

    // has_name_field_kind tests
    assert!(person_name.has_name_field_kind(&NameFieldKind::Given));
    assert!(!person_name.has_name_field_kind(&NameFieldKind::Surname2));

    // has_name_field
    assert!(person_name.has_name_field(&NameField {
        kind: NameFieldKind::Given,
        modifier: FieldModifierSet::default(),
    }));
    assert!(!person_name.has_name_field(&NameField {
        kind: NameFieldKind::Surname2,
        modifier: FieldModifierSet::default(),
    }));
    assert!(!person_name.has_name_field(&NameField {
        kind: NameFieldKind::Surname,
        modifier: FieldModifierSet::new(
            FieldCapsStyle::AllCaps,
            FieldPart::Auto,
            FieldLength::Auto,
            FieldFormality::Auto,
        ),
    }));

    // get
    assert_eq!(
        person_name.get(&NameField {
            kind: NameFieldKind::Given,
            modifier: FieldModifierSet::default(),
        }),
        Some("Henry")
    );
    assert_eq!(
        person_name.get(&NameField {
            kind: NameFieldKind::Surname,
            modifier: FieldModifierSet::default(),
        }),
        Some("Jekyll")
    );
    assert_eq!(
        person_name.get(&NameField {
            kind: NameFieldKind::Surname,
            modifier: FieldModifierSet::new(
                FieldCapsStyle::Auto,
                FieldPart::Auto,
                FieldLength::Auto,
                FieldFormality::Informal,
            ),
        }),
        Some("Hide")
    );
    Ok(())
}

#[test]
fn test_field_modifier_person_name_should_have_given_or_surname() {
    let mut person_data: LiteMap<NameField, String> = LiteMap::new();
    person_data.insert(
        NameField {
            kind: NameFieldKind::Title,
            modifier: FieldModifierSet::default(),
        },
        String::from("Dr"),
    );

    let person_name = DefaultPersonName::new(person_data, None, None);
    assert!(person_name.is_err());
}
