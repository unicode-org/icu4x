use icu::locid::macros::langid;
use icu::plurals::{PluralCategory, PluralRuleType, PluralRules};
use icu_provider_static::StaticDataProvider;

fn main() {
    let lid = langid!("en");

    let dp = StaticDataProvider::new();

    let pr = PluralRules::try_new(lid, &dp, PluralRuleType::Cardinal)
        .expect("Failed to construct a PluralRules struct.");

    assert_eq!(pr.select(5_usize), PluralCategory::Other);
}
