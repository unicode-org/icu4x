use icu_cldr_json_data_provider::{CldrJsonDataProvider, CldrPaths};
use icu_locale::LanguageIdentifier;
use icu_pluralrules::{PluralCategory, PluralRuleType, PluralRules};

#[test]
fn test_plural_rules() {
    let mut paths = CldrPaths::default();
    paths.cldr_core = Ok("./data/cldr-core".into());

    let dp = CldrJsonDataProvider::new(&paths);

    let lang: LanguageIdentifier = "en".parse().unwrap();

    let pr = PluralRules::try_new(lang, PluralRuleType::Cardinal, &dp).unwrap();

    assert_eq!(pr.select(5_usize), PluralCategory::Other);
}

#[test]
fn test_plural_rules_missing() {
    let mut paths = CldrPaths::default();
    paths.cldr_core = Ok("./data/cldr-core".into());

    let dp = CldrJsonDataProvider::new(&paths);

    let lang: LanguageIdentifier = "xx".parse().unwrap();

    let pr = PluralRules::try_new(lang, PluralRuleType::Cardinal, &dp);

    assert!(pr.is_err());
}

#[test]
fn test_plural_category_all() {
    let categories: Vec<&PluralCategory> = PluralCategory::all().collect();

    assert_eq!(categories.get(0), Some(&&PluralCategory::Zero));
}
