// An example application which uses icu_pluralrules to construct a correct
// sentence for English based on the numerical value in Ordinal category.
use icu_locale::LanguageIdentifier;
use icu_pluralrules::{PluralCategory, PluralRuleType, PluralRules};

const VALUES: &[usize] = &[0, 2, 25, 1, 3, 2, 4, 10, 7, 0];

fn print(_input: &str, _value: Option<usize>) {
    #[cfg(debug_assertions)]
    if let Some(value) = _value {
        println!("{}", _input.replace("{}", &value.to_string()));
    } else {
        println!("{}", _input);
    }
}

fn main() {
    let langid: LanguageIdentifier = "en".parse().expect("Failed to parse Language Identifier.");

    let provider = icu_testdata::get_provider();

    {
        print("\n====== Elevator Floor (en) example ============", None);
        let pr = PluralRules::try_new(langid, &provider, PluralRuleType::Ordinal)
            .expect("Failed to create a PluralRules instance.");

        for value in VALUES {
            match pr.select(*value) {
                PluralCategory::One => print("You are on the {}st floor.", Some(*value)),
                PluralCategory::Two => print("You are on the {}nd floor.", Some(*value)),
                PluralCategory::Few => print("You are on the {}rd floor.", Some(*value)),
                _ => print("You are on the {}th floor.", Some(*value)),
            }
        }
    }
}
