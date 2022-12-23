// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use displaydoc::Display;
use icu_locid::LanguageIdentifier;
use serde::Deserialize;

static SOURCES_TOML: &str = include_str!("../sources.toml");

#[derive(Debug, Deserialize)]
#[non_exhaustive]
pub struct SourceMetadata {
    pub locales: Vec<LanguageIdentifier>,
    pub cldr_json_glob: Vec<String>,
    pub cldr_json_gitref: String,
    pub icuexportdata_glob: Vec<String>,
    pub icuexportdata_gitref: String,
}

impl SourceMetadata {
    /// Expands `cldr_json_glob` to the list of all included CLDR JSON paths.
    // TODO: Consider making this a Generator.
    pub fn get_all_cldr_paths(&self) -> Vec<String> {
        self.expand_paths(&self.cldr_json_glob)
    }

    pub fn get_all_icuexportdata_paths(&self) -> Vec<String> {
        self.expand_paths(&self.icuexportdata_glob)
    }

    fn expand_paths(&self, in_paths: &[String]) -> Vec<String> {
        let mut paths = vec![];
        for pattern in in_paths.iter() {
            if pattern.contains("$LOCALES") {
                for locale in self.locales.iter() {
                    let locale_str = writeable::Writeable::write_to_string(locale);
                    paths.push(pattern.replace("$LOCALES", &locale_str));
                }
                // Also add "root" for older CLDRs
                paths.push(pattern.replace("$LOCALES", "root"));
            } else {
                // No variable in pattern
                paths.push(pattern.clone())
            }
        }
        paths
    }
}

pub fn load() -> SourceMetadata {
    #[allow(clippy::unwrap_used)] // the TOML source is a constant
    toml::from_str(SOURCES_TOML).unwrap()
}

#[test]
fn test_metadata() {
    let package_info = load();
    assert!(package_info.locales.contains(&(LanguageIdentifier::UND)));
    assert!(package_info.locales.len() > 10);
}
