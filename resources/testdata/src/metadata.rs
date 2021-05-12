// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use cargo_metadata::{self, camino::Utf8PathBuf, MetadataCommand};
use icu_locid::LanguageIdentifier;
use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Cargo Error: {0}")]
    Cargo(#[from] cargo_metadata::Error),
    #[error("Serde Error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("{0}: package not found", env!("CARGO_PKG_NAME"))]
    PackageNotFound,
    #[error("package.metadata.icu4x_testdata not found")]
    MetadataNotFound,
}

#[derive(Debug, Deserialize)]
pub struct PackageMetadata {
    pub locales: Vec<LanguageIdentifier>,
    pub cldr_json_glob: Vec<String>,
    pub gitref: String,
}

impl PackageMetadata {
    /// Expands `cldr_json_glob` to the list of all included CLDR JSON paths.
    // TODO: Consider making this a Generator.
    pub fn get_all_cldr_paths(&self) -> Vec<String> {
        let mut paths = vec![];
        for pattern in self.cldr_json_glob.iter() {
            if pattern.contains("$LOCALES") {
                for locale in self.locales.iter() {
                    let locale_str = writeable::Writeable::writeable_to_string(locale);
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
pub struct PackageInfo {
    pub target_directory: Utf8PathBuf,
    pub package_metadata: PackageMetadata,
}

pub fn load() -> Result<PackageInfo, Error> {
    let metadata = MetadataCommand::new().no_deps().exec()?;

    let target_directory = metadata.target_directory;

    let mut icu_testdata_pkg = metadata
        .packages
        // into_iter() rather than iter() to take ownership of the result
        .into_iter()
        .find(|x| x.name == env!("CARGO_PKG_NAME"))
        .ok_or(Error::PackageNotFound)?;

    let package_metadata: PackageMetadata = serde_json::from_value(
        icu_testdata_pkg
            .metadata
            // Use mut functions so that we can call .take() at the end
            .as_object_mut()
            .ok_or(Error::MetadataNotFound)?
            .get_mut("icu4x_testdata")
            .ok_or(Error::MetadataNotFound)?
            .take(),
    )?;

    Ok(PackageInfo {
        target_directory,
        package_metadata,
    })
}

#[test]
fn test_metadata() {
    let package_info = load().expect("Failed to load metadata");
    assert!(package_info
        .package_metadata
        .locales
        .contains(&(LanguageIdentifier::und())));
    assert!(package_info.package_metadata.locales.len() > 10);
}
