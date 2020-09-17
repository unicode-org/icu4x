// An example application which uses icu_pluralrules to construct the correct
// sentence for English based on the numerical value in Cardinal and Ordinal categories.
use icu_data_provider::InvariantDataProvider;
use icu_locale::LanguageIdentifier;
use icu_pluralrules::{PluralCategory, PluralRuleType, PluralRules};

static SILENT: bool = false;

fn print(input: &str, value: Option<usize>) {
    if SILENT {
        return;
    }
    if let Some(value) = value {
        println!("{}", input.replace("{}", &value.to_string()));
    } else {
        println!("{}", input);
    }
}

fn main() {
    let langid = LanguageIdentifier::default();
    let values: &[usize] = &[0, 2, 25, 1, 3, 2, 4, 10, 7, 0];
    let dtp = InvariantDataProvider;

    {
        print("\n====== Unread Emails (en) example ============", None);
        let pr = PluralRules::try_new(langid.clone(), PluralRuleType::Cardinal, &dtp)
            .expect("Failed to create a PluralRules instance.");

        for value in values {
            match pr.select(*value) {
                PluralCategory::One => print("You have one unread message.", None),
                _ => print("You have {} unread messages.", Some(*value)),
            }
        }
    }

    {
        print("\n====== Elevator Floor (en) example ============", None);
        let pr = PluralRules::try_new(langid, PluralRuleType::Ordinal, &dtp)
            .expect("Failed to create a PluralRules instance.");

        for value in values {
            match pr.select(*value) {
                PluralCategory::One => print("You are on the {}st floor.", Some(*value)),
                PluralCategory::Two => print("You are on the {}nd floor.", Some(*value)),
                PluralCategory::Few => print("You are on the {}rd floor.", Some(*value)),
                _ => print("You are on the {}th floor.", Some(*value)),
            }
        }
    }
}
