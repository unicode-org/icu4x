// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locid::{subtags, Locale};
use icu_properties::script::ScriptWithExtensionsBorrowed;
use subtags::Script;

use crate::api::NameFieldKind::{Given, Surname};
use crate::api::{NameFieldKind, PersonName, PersonNamesFormatterError};

/// Override the formatting payload to use based on specification rules.
///
/// if name locale and formatting locale are incompatible, name locale takes precedence
/// it should dynamically load the name locale formatter using the data_provider given in constructor.
/// https://www.unicode.org/reports/tr35/tr35-personNames.html#switch-the-formatting-locale-if-necessary
///
/// The formatter locale and name locale must be maximized first.
///
pub(crate) fn effective_locale<'a>(
    formatter_locale: &'a Locale,
    person_name_locale: &'a Locale,
) -> Result<&'a Locale, PersonNamesFormatterError> {
    let name_script = person_name_locale.id.script.unwrap();
    let formatter_script = formatter_locale.id.script.unwrap();
    if !compatible_scripts(name_script, formatter_script) {
        return Ok(person_name_locale);
    }
    Ok(formatter_locale)
}

// TODO: proper handling of compatible scripts.
fn compatible_scripts(sc1: Script, sc2: Script) -> bool {
    let sc1_str = sc1.as_str();
    let sc2_str = sc2.as_str();
    let jpan_compatible = ["Hani", "Kana", "Hira"];
    if sc1_str == "Jpan" && jpan_compatible.contains(&sc2_str) {
        return true;
    }
    if sc2_str == "Jpan" && jpan_compatible.contains(&sc1_str) {
        return true;
    }
    sc1_str == sc2_str
}

/// https://www.unicode.org/reports/tr35/tr35-personNames.html#derive-the-name-locale
pub(crate) fn likely_person_name_locale<N>(
    person_name: &N,
) -> Result<Locale, PersonNamesFormatterError>
where
    N: PersonName,
{
    use icu_properties::Script;

    let swe = icu_properties::script::script_with_extensions();
    let mut found_name_script = find_script(person_name, swe, Surname);
    if found_name_script.is_none() {
        found_name_script = find_script(person_name, swe, Given);
    }
    let name_script = found_name_script.unwrap_or(Script::Unknown);

    let lookup = Script::enum_to_short_name_mapper().static_to_owned();
    let lookup = lookup.as_borrowed();

    person_name.name_locale().map_or_else(
        || {
            let default_language_str =
                "und-".to_owned() + lookup.get(name_script).unwrap().as_str();
            let effective_locale: Locale = default_language_str.parse().map_err(|_err| {
                PersonNamesFormatterError::ParseError(String::from(
                    "Cannot build default locale for parser",
                ))
            })?;
            Ok(effective_locale)
        },
        |locale| {
            let mut effective_locale = locale.clone();
            let effective_script = lookup
                .get(name_script)
                .unwrap()
                .as_str()
                .parse::<subtags::Script>()
                .map_err(|_err| PersonNamesFormatterError::InvalidPersonName)?;
            effective_locale.id.script = Some(effective_script);
            Ok(effective_locale)
        },
    )
}

fn find_script<N>(
    person_name: &N,
    swe: ScriptWithExtensionsBorrowed,
    kind: NameFieldKind,
) -> Option<icu_properties::Script>
where
    N: PersonName,
{
    use icu_properties::Script;

    person_name
        .available_name_fields()
        .iter()
        .filter(|&name_field| name_field.kind == kind)
        .find_map(|&name_field| {
            person_name.get(name_field).chars().find_map(|c| {
                let char_script = swe.get_script_val(c as u32);
                match char_script {
                    Script::Common | Script::Unknown | Script::Inherited => None,
                    _ => Some(char_script),
                }
            })
        })
}

#[cfg(test)]
mod tests {
    use icu_locid::locale;
    use icu_locid_transform::LocaleExpander;
    use litemap::LiteMap;

    use crate::api::{FieldModifierSet, NameField, NameFieldKind, PersonNamesFormatterError};
    use crate::derive_locale::{effective_locale, likely_person_name_locale};
    use crate::provided_struct::DefaultPersonName;

    #[test]
    fn test_effective_locale_matching_script() {
        let lc = LocaleExpander::new_extended();
        let mut locale = locale!("fr");
        lc.maximize(&mut locale);
        assert_eq!(
            effective_locale(&locale!("de_Latn_ch"), &locale),
            Ok(&locale!("de_Latn_ch"))
        );
        assert_eq!(
            effective_locale(&locale, &locale!("de_Latn_ch")),
            Ok(&locale!("fr_Latn_FR"))
        );
    }

    #[test]
    fn test_effective_locale_non_matching_script() {
        let lc = LocaleExpander::new_extended();
        let mut locale = locale!("ja");
        lc.maximize(&mut locale);
        assert_eq!(
            effective_locale(&locale!("de_Latn_ch"), &locale),
            Ok(&locale!("ja-Jpan-JP"))
        );
        assert_eq!(
            effective_locale(&locale, &locale!("de_Latn_ch")),
            Ok(&locale!("de-Latn-CH"))
        );
    }

    #[test]
    fn test_effective_locale_compatible_script() {
        let lc = LocaleExpander::new_extended();
        let mut locale = locale!("ja");
        lc.maximize(&mut locale);
        assert_eq!(
            effective_locale(&locale!("ja_Hani_JP"), &locale),
            Ok(&locale!("ja_Hani_JP"))
        );
        assert_eq!(
            effective_locale(&locale!("ja_Kana_JP"), &locale),
            Ok(&locale!("ja-Kana-JP"))
        );
        assert_eq!(
            effective_locale(&locale!("ja_Hira_JP"), &locale),
            Ok(&locale!("ja-Hira-JP"))
        );
        assert_eq!(
            effective_locale(&locale, &locale!("ja_Hani_JP")),
            Ok(&locale!("ja-Jpan-JP"))
        );
        assert_eq!(
            effective_locale(&locale, &locale!("ja_Kana_JP")),
            Ok(&locale!("ja-Jpan-JP"))
        );
        assert_eq!(
            effective_locale(&locale, &locale!("ja_Hira_JP")),
            Ok(&locale!("ja-Jpan-JP"))
        );
    }

    #[test]
    fn test_likely_person_names_locale() {
        assert_eq!(
            likely_person_name_locale(&person_name("Miyazaki", "Hayao").unwrap()),
            Ok(locale!("und_Latn"))
        );
        assert_eq!(
            likely_person_name_locale(&person_name("駿", "宮崎").unwrap()),
            Ok(locale!("und_Hani"))
        );
        assert_eq!(
            likely_person_name_locale(&person_name("하야오", "미야자키").unwrap()),
            Ok(locale!("und_Hang"))
        );
        assert_eq!(
            likely_person_name_locale(&person_name("アルベルト", "アインシュタイン").unwrap()),
            Ok(locale!("und_Kana"))
        );
    }

    fn person_name(
        surname: &str,
        first_name: &str,
    ) -> Result<DefaultPersonName, PersonNamesFormatterError> {
        let mut person_data: LiteMap<NameField, String> = LiteMap::new();
        person_data.insert(
            NameField {
                kind: NameFieldKind::Surname,
                modifier: FieldModifierSet::default(),
            },
            String::from(surname),
        );
        person_data.insert(
            NameField {
                kind: NameFieldKind::Given,
                modifier: FieldModifierSet::default(),
            },
            String::from(first_name),
        );

        DefaultPersonName::new(person_data, None, None)
    }
}
