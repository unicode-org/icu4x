use icu_locale::LanguageIdentifier;
use icu_pluralrules::PluralCategory;
use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct NumbersFixture {
    pub isize: Vec<i64>,
    pub usize: Vec<u64>,
    pub string: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct PluralsFixture {
    pub rules: HashMap<String, LocalePluralRulesFixture>,
    pub langs: Vec<LanguageIdentifier>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct LocalePluralRulesFixture {
    #[serde(rename = "pluralRule-count-zero")]
    pub zero: Option<String>,
    #[serde(rename = "pluralRule-count-one")]
    pub one: Option<String>,
    #[serde(rename = "pluralRule-count-two")]
    pub two: Option<String>,
    #[serde(rename = "pluralRule-count-few")]
    pub few: Option<String>,
    #[serde(rename = "pluralRule-count-many")]
    pub many: Option<String>,
}

impl LocalePluralRulesFixture {
    #[allow(dead_code)]
    pub(crate) fn get(&self, category: &PluralCategory) -> Option<&String> {
        match category {
            PluralCategory::Zero => self.zero.as_ref(),
            PluralCategory::One => self.one.as_ref(),
            PluralCategory::Two => self.two.as_ref(),
            PluralCategory::Few => self.few.as_ref(),
            PluralCategory::Many => self.many.as_ref(),
            PluralCategory::Other => None,
        }
    }
}
