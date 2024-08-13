// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locale_core::subtags::script;
use icu_locale_core::{subtags, Locale};
use icu_properties::names::PropertyEnumToValueNameLinearTiny4MapperBorrowed;
use icu_properties::script::ScriptWithExtensionsBorrowed;

use crate::personnames::api::NameFieldKind::{Given, Surname};
use crate::personnames::api::{NameFieldKind, PersonName, PersonNamesFormatterError};

/// Override the formatting payload to use based on specification rules.
///
/// if name locale and formatting locale are incompatible, name locale takes precedence
/// it should dynamically load the name locale formatter using the data_provider given in constructor.
/// https://www.unicode.org/reports/tr35/tr35-personNames.html#switch-the-formatting-locale-if-necessary
///
/// The formatter locale and name locale must be maximized first.
pub fn effective_locale<'a>(
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
fn compatible_scripts(sc1: subtags::Script, sc2: subtags::Script) -> bool {
    let jpan_compatible = [script!("Hani"), script!("Kana"), script!("Hira")];
    if sc1 == script!("Jpan") && jpan_compatible.contains(&sc2) {
        return true;
    }
    if sc2 == script!("Jpan") && jpan_compatible.contains(&sc1) {
        return true;
    }
    sc1 == sc2
}

/// https://www.unicode.org/reports/tr35/tr35-personNames.html#derive-the-name-locale
pub fn likely_person_name_locale<N>(
    person_name: &N,
    swe: ScriptWithExtensionsBorrowed,
    scripts: PropertyEnumToValueNameLinearTiny4MapperBorrowed<icu_properties::Script>,
) -> Result<Locale, PersonNamesFormatterError>
where
    N: PersonName,
{
    let mut found_name_script = find_script(person_name, swe, Surname);
    if found_name_script.is_none() {
        found_name_script = find_script(person_name, swe, Given);
    }
    let name_script = found_name_script.unwrap_or(icu_properties::Script::Unknown);

    let locid_script = scripts
        .get(name_script)
        .unwrap()
        .as_str()
        .parse::<subtags::Script>()
        .map_err(|_err| PersonNamesFormatterError::InvalidPersonName)?;
    person_name.name_locale().map_or_else(
        || {
            let mut effective_locale = Locale::default();
            effective_locale.id.script = Some(locid_script);
            Ok(effective_locale)
        },
        |locale| {
            let mut effective_locale = locale.clone();
            effective_locale.id.script = Some(locid_script);
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
    use icu_locale::LocaleExpander;
    use icu_locale_core::locale;
    use litemap::LiteMap;

    use super::{effective_locale, likely_person_name_locale};
    use crate::personnames::api::{
        FieldModifierSet, NameField, NameFieldKind, PersonNamesFormatterError,
    };
    use crate::personnames::provided_struct::DefaultPersonName;

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
        let swe = icu_properties::script::script_with_extensions();
        let scripts = icu_properties::Script::enum_to_short_name_mapper();
        assert_eq!(
            likely_person_name_locale(&person_name("Miyazaki", "Hayao").unwrap(), swe, scripts),
            Ok(locale!("und_Latn"))
        );
        assert_eq!(
            likely_person_name_locale(&person_name("駿", "宮崎").unwrap(), swe, scripts),
            Ok(locale!("und_Hani"))
        );
        assert_eq!(
            likely_person_name_locale(&person_name("하야오", "미야자키").unwrap(), swe, scripts),
            Ok(locale!("und_Hang"))
        );
        assert_eq!(
            likely_person_name_locale(
                &person_name("アルベルト", "アインシュタイン").unwrap(),
                swe,
                scripts
            ),
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
