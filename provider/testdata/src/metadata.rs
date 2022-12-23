// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use displaydoc::Display;
use icu_locid::LanguageIdentifier;
use serde::Deserialize;

static SOURCES_TOML: &str = include_str!("../sources.toml");

#[derive(Display, Debug)]
#[non_exhaustive]
pub enum Error {
    #[displaydoc("Serde Error: {0}")]
    SerdeToml(toml::de::Error),
}

impl std::error::Error for Error {}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Error::SerdeToml(e)
    }
}

#[derive(Debug, Deserialize)]
#[non_exhaustive]
pub struct PackageMetadata {
    pub locales: Vec<LanguageIdentifier>,
    pub cldr_json_glob: Vec<String>,
    pub cldr_json_gitref: String,
    pub icuexportdata_glob: Vec<String>,
    pub icuexportdata_gitref: String,
}

impl PackageMetadata {
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

#[derive(Debug)]
#[non_exhaustive]
pub struct PackageInfo {
    pub package_metadata: PackageMetadata,
}

pub fn load() -> Result<PackageInfo, Error> {
    let package_metadata: PackageMetadata = toml::from_str(SOURCES_TOML).unwrap();

    Ok(PackageInfo {
        package_metadata,
    })
}

#[test]
fn test_metadata() {
    let package_info = load().expect("Failed to load metadata");
    assert!(package_info
        .package_metadata
        .locales
        .contains(&(LanguageIdentifier::UND)));
    assert!(package_info.package_metadata.locales.len() > 10);
}
