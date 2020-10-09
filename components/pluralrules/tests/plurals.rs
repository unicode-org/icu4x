use icu_locale::LanguageIdentifier;
use icu_pluralrules::{PluralCategory, PluralRuleType, PluralRules};

#[test]
fn test_plural_rules() {
    let provider = icu_testdata::get_provider();

    let lang: LanguageIdentifier = "en".parse().unwrap();

    let pr = PluralRules::try_new(lang, &provider, PluralRuleType::Cardinal).unwrap();

    assert_eq!(pr.select(5_usize), PluralCategory::Other);
}

#[test]
fn test_plural_rules_missing() {
    let provider = icu_testdata::get_provider();

    let lang: LanguageIdentifier = "xx".parse().unwrap();

    let pr = PluralRules::try_new(lang, &provider, PluralRuleType::Cardinal);

    assert!(pr.is_err());
}

#[test]
fn test_plural_category_all() {
    let categories: Vec<&PluralCategory> = PluralCategory::all().collect();

    assert_eq!(categories.get(0), Some(&&PluralCategory::Zero));
}
