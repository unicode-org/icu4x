// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

extern crate alloc;

use icu_experimental::personnames::api::*;
use icu_experimental::personnames::provided_struct::DefaultPersonName;
use icu_experimental::personnames::PersonNamesFormatter;
use icu_locid::locale;
use litemap::LiteMap;
use PersonNamesFormatterError::ParseError;

mod baked {
    pub struct Baked;

    mod icu {
        pub use icu_collections as collections;
        pub use icu_experimental as experimental;
        pub use icu_locid_transform as locid_transform;
        pub use icu_properties as properties;
    }

    icu_experimental_data::make_provider!(Baked);
    icu_experimental_data::impl_personnames_personnames_v1!(Baked);
    icu_locid_transform_data::impl_fallback_supplement_co_v1!(Baked);
    icu_locid_transform_data::impl_fallback_parents_v1!(Baked);
    icu_locid_transform_data::impl_fallback_likelysubtags_v1!(Baked);
    icu_properties_data::impl_propnames_to_short_linear4_sc_v1!(Baked);
    icu_properties_data::impl_props_scx_v1!(Baked);
}

use baked::Baked as PersonNamesBakedProvider;

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
            modifier: FieldModifierSet::formality(FieldFormality::Informal),
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
        modifier: FieldModifierSet::style(FieldCapsStyle::AllCaps),
    }));

    // get
    assert_eq!(
        person_name.get(&NameField {
            kind: NameFieldKind::Given,
            modifier: FieldModifierSet::default(),
        }),
        "Henry"
    );
    assert_eq!(
        person_name.get(&NameField {
            kind: NameFieldKind::Surname,
            modifier: FieldModifierSet::default(),
        }),
        "Jekyll"
    );
    assert_eq!(
        person_name.get(&NameField {
            kind: NameFieldKind::Surname,
            modifier: FieldModifierSet::formality(FieldFormality::Informal),
        }),
        "Hide"
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

#[test]
fn test_formatter() -> Result<(), PersonNamesFormatterError> {
    let mut person_data: LiteMap<NameField, String> = LiteMap::new();
    person_data.insert(
        NameField {
            kind: NameFieldKind::Surname,
            modifier: FieldModifierSet::default(),
        },
        String::from("Wells"),
    );
    person_data.insert(
        NameField {
            kind: NameFieldKind::Given,
            modifier: FieldModifierSet::length(FieldLength::Initial),
        },
        String::from("H."),
    );
    person_data.insert(
        NameField {
            kind: NameFieldKind::Given2,
            modifier: FieldModifierSet::length(FieldLength::Initial),
        },
        String::from("G."),
    );

    let person_name = DefaultPersonName::new(person_data, Some(locale!("es")), None)?;

    let formatter = PersonNamesFormatter::try_new_unstable(
        &PersonNamesBakedProvider,
        PersonNamesFormatterOptions::new(
            locale!("es"),
            FormattingOrder::Sorting,
            FormattingLength::Short,
            FormattingUsage::Referring,
            FormattingFormality::Formal,
        ),
    )
    .map_err(|err| ParseError(err.to_string()))?;
    let value = formatter.format_to_string(&PersonNamesBakedProvider, &person_name)?;
    assert_eq!(&value, "Wells, H. G.");
    Ok(())
}

#[test]
fn test_formatter_switch_language_from_ja_to_es() -> Result<(), PersonNamesFormatterError> {
    let mut person_data: LiteMap<NameField, String> = LiteMap::new();
    person_data.insert(
        NameField {
            kind: NameFieldKind::Surname,
            modifier: FieldModifierSet::default(),
        },
        String::from("Wells"),
    );
    person_data.insert(
        NameField {
            kind: NameFieldKind::Given,
            modifier: FieldModifierSet::length(FieldLength::Initial),
        },
        String::from("H."),
    );
    person_data.insert(
        NameField {
            kind: NameFieldKind::Given2,
            modifier: FieldModifierSet::length(FieldLength::Initial),
        },
        String::from("G."),
    );

    let person_name = DefaultPersonName::new(person_data, Some(locale!("es")), None)?;

    let formatter = PersonNamesFormatter::try_new_unstable(
        &PersonNamesBakedProvider,
        PersonNamesFormatterOptions::new(
            locale!("ja"),
            FormattingOrder::Sorting,
            FormattingLength::Short,
            FormattingUsage::Referring,
            FormattingFormality::Formal,
        ),
    )
    .map_err(|err| ParseError(err.to_string()))?;
    let value = formatter.format_to_string(&PersonNamesBakedProvider, &person_name)?;
    // Since script are different, formatting should prioritise the name locale (es) over
    // formatting locale (ja).
    assert_eq!(&value, "Wells, H. G.");
    Ok(())
}

#[test]
fn test_space_replacement_spec_formatting_locale_ja() -> Result<(), PersonNamesFormatterError> {
    let mut person_data: LiteMap<NameField, String> = LiteMap::new();
    person_data.insert(
        NameField {
            kind: NameFieldKind::Surname,
            modifier: FieldModifierSet::default(),
        },
        String::from("Einstein"),
    );
    person_data.insert(
        NameField {
            kind: NameFieldKind::Given,
            modifier: FieldModifierSet::default(),
        },
        String::from("Albert"),
    );

    let person_name = DefaultPersonName::new(
        person_data,
        Some(locale!("de_Latn_CH")),
        Some(PreferredOrder::GivenFirst),
    )?;

    let formatter = PersonNamesFormatter::try_new_unstable(
        &PersonNamesBakedProvider,
        PersonNamesFormatterOptions::new(
            locale!("ja_JP"),
            FormattingOrder::GivenFirst,
            FormattingLength::Medium,
            FormattingUsage::Referring,
            FormattingFormality::Formal,
        ),
    )
    .map_err(|err| ParseError(err.to_string()))?;
    let value = formatter.format_to_string(&PersonNamesBakedProvider, &person_name)?;
    // ja formatting for sorting-short-referring-formal is {surname} {given}
    // since there's no given, only surname is displayed. the space is then trimmed.
    assert_eq!(&value, "Albert Einstein");
    Ok(())
}

#[test]
fn test_space_replacement_spec_formatting_locale_ja_jpan_script(
) -> Result<(), PersonNamesFormatterError> {
    let mut person_data: LiteMap<NameField, String> = LiteMap::new();
    person_data.insert(
        NameField {
            kind: NameFieldKind::Surname,
            modifier: FieldModifierSet::default(),
        },
        String::from("アインシュタイン"),
    );
    person_data.insert(
        NameField {
            kind: NameFieldKind::Given,
            modifier: FieldModifierSet::default(),
        },
        String::from("アルベルト"),
    );

    let person_name = DefaultPersonName::new(
        person_data,
        Some(locale!("de_Jpan_CH")),
        Some(PreferredOrder::GivenFirst),
    )?;

    let formatter = PersonNamesFormatter::try_new_unstable(
        &PersonNamesBakedProvider,
        PersonNamesFormatterOptions::new(
            locale!("ja_JP"),
            FormattingOrder::GivenFirst,
            FormattingLength::Medium,
            FormattingUsage::Referring,
            FormattingFormality::Formal,
        ),
    )
    .map_err(|err| ParseError(err.to_string()))?;
    let value = formatter.format_to_string(&PersonNamesBakedProvider, &person_name)?;
    // ja formatting for sorting-short-referring-formal is {surname} {given}
    // since there's no given, only surname is displayed. the space is then trimmed.
    assert_eq!(&value, "アルベルト・アインシュタイン");
    Ok(())
}

#[test]
fn test_space_replacement_spec_formatting_locale_ja_compatible(
) -> Result<(), PersonNamesFormatterError> {
    let mut person_data: LiteMap<NameField, String> = LiteMap::new();
    person_data.insert(
        NameField {
            kind: NameFieldKind::Surname,
            modifier: FieldModifierSet::default(),
        },
        String::from("宮崎"),
    );
    person_data.insert(
        NameField {
            kind: NameFieldKind::Given,
            modifier: FieldModifierSet::default(),
        },
        String::from("駿"),
    );

    let person_name = DefaultPersonName::new(
        person_data,
        Some(locale!("ja_Jpan_JP")),
        Some(PreferredOrder::SurnameFirst),
    )?;

    let formatter = PersonNamesFormatter::try_new_unstable(
        &PersonNamesBakedProvider,
        PersonNamesFormatterOptions::new(
            locale!("ja_JP"),
            FormattingOrder::GivenFirst,
            FormattingLength::Medium,
            FormattingUsage::Referring,
            FormattingFormality::Formal,
        ),
    )
    .map_err(|err| ParseError(err.to_string()))?;
    let value = formatter.format_to_string(&PersonNamesBakedProvider, &person_name)?;
    // ja formatting for sorting-short-referring-formal is {surname} {given}
    // since there's no given, only surname is displayed. the space is then trimmed.
    assert_eq!(&value, "宮崎駿");
    Ok(())
}

#[test]
fn test_space_replacement_spec_formatting_locale_de_compatible(
) -> Result<(), PersonNamesFormatterError> {
    let mut person_data: LiteMap<NameField, String> = LiteMap::new();
    person_data.insert(
        NameField {
            kind: NameFieldKind::Surname,
            modifier: FieldModifierSet::default(),
        },
        String::from("Einstein"),
    );
    person_data.insert(
        NameField {
            kind: NameFieldKind::Given,
            modifier: FieldModifierSet::default(),
        },
        String::from("Albert"),
    );

    let person_name = DefaultPersonName::new(
        person_data,
        Some(locale!("de_Latn_CH")),
        Some(PreferredOrder::GivenFirst),
    )?;

    let formatter = PersonNamesFormatter::try_new_unstable(
        &PersonNamesBakedProvider,
        PersonNamesFormatterOptions::new(
            locale!("de_CH"),
            FormattingOrder::GivenFirst,
            FormattingLength::Medium,
            FormattingUsage::Referring,
            FormattingFormality::Formal,
        ),
    )
    .map_err(|err| ParseError(err.to_string()))?;
    let value = formatter.format_to_string(&PersonNamesBakedProvider, &person_name)?;
    // ja formatting for sorting-short-referring-formal is {surname} {given}
    // since there's no given, only surname is displayed. the space is then trimmed.
    assert_eq!(&value, "Albert Einstein");
    Ok(())
}

#[test]
fn test_space_replacement_spec_formatting_locale_de_jpan_script(
) -> Result<(), PersonNamesFormatterError> {
    let mut person_data: LiteMap<NameField, String> = LiteMap::new();
    person_data.insert(
        NameField {
            kind: NameFieldKind::Surname,
            modifier: FieldModifierSet::default(),
        },
        String::from("アインシュタイン"),
    );
    person_data.insert(
        NameField {
            kind: NameFieldKind::Given,
            modifier: FieldModifierSet::default(),
        },
        String::from("アルベルト"),
    );

    let person_name = DefaultPersonName::new(
        person_data,
        Some(locale!("de_Jpan_CH")),
        Some(PreferredOrder::GivenFirst),
    )?;

    let formatter = PersonNamesFormatter::try_new_unstable(
        &PersonNamesBakedProvider,
        PersonNamesFormatterOptions::new(
            locale!("de_CH"),
            FormattingOrder::GivenFirst,
            FormattingLength::Medium,
            FormattingUsage::Referring,
            FormattingFormality::Formal,
        ),
    )
    .map_err(|err| ParseError(err.to_string()))?;
    let value = formatter.format_to_string(&PersonNamesBakedProvider, &person_name)?;
    // TODO: Double check specs
    // Specs example says that it should contain the dot,
    // Following specs, the name locale should be use if provided, in this case, "de".
    // assert_eq!(&value, "アルベルト・アインシュタイン");
    assert_eq!(&value, "アルベルト アインシュタイン");
    Ok(())
}

#[test]
fn test_space_replacement_spec_formatting_locale_und_latn_jp(
) -> Result<(), PersonNamesFormatterError> {
    let mut person_data: LiteMap<NameField, String> = LiteMap::new();
    person_data.insert(
        NameField {
            kind: NameFieldKind::Surname,
            modifier: FieldModifierSet::default(),
        },
        String::from("Miyazaki"),
    );
    person_data.insert(
        NameField {
            kind: NameFieldKind::Given,
            modifier: FieldModifierSet::default(),
        },
        String::from("Hayao"),
    );

    let person_name = DefaultPersonName::new(
        person_data,
        Some(locale!("und_Latn_JP")),
        Some(PreferredOrder::GivenFirst),
    )?;

    let formatter = PersonNamesFormatter::try_new_unstable(
        &PersonNamesBakedProvider,
        PersonNamesFormatterOptions::new(
            locale!("de_CH"),
            FormattingOrder::GivenFirst,
            FormattingLength::Medium,
            FormattingUsage::Referring,
            FormattingFormality::Formal,
        ),
    )
    .map_err(|err| ParseError(err.to_string()))?;
    let value = formatter.format_to_string(&PersonNamesBakedProvider, &person_name)?;
    // ja formatting for sorting-short-referring-formal is {surname} {given}
    // since there's no given, only surname is displayed. the space is then trimmed.
    assert_eq!(&value, "Hayao Miyazaki");
    Ok(())
}
