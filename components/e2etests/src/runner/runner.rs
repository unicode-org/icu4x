use super::*;

use icu_locale::Locale;
use icu_locale::subtags::Region;
use icu_locale::subtags::Variant;
use icu_locale::subtags::Variants;

impl From<&Locale> for LocaleTestOutput {
    fn from(input: &Locale) -> Self {
        let language = input.language;
        let script = input.script;
        let variants = &input.variants;

        let region: Option<Region> =
            input
            .region;
        let region: &str =
            match &region {
                Some(r) => r.as_str(),
                None    => "",
            };
        let region: String = String::from(region);

        let lang: String = String::from(language.as_str());

        let script: String = match script {
            Some(s) => String::from(s.as_str()),
            None    => String::from(""),
        };

        let variants: Vec<Variant> = variants.to_vec();
        let variants: Vec<String> = variants.into_iter()
            .map(|v: Variant| String::from(v.as_str()))
            .collect::<Vec<String>>();
        let subtags: Vec<Vec<String>> = vec![variants];

        let locale_test_output = LocaleTestOutput {
            lang,
            region,
            subtags
        };

        locale_test_output
    }
}


pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

//
// tests
//

#[cfg(test)]
#[path = "./runner_test.rs"]
mod runner_tests;
