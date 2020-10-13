// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use cargo_metadata::{self, MetadataCommand};
use icu_locale::LanguageIdentifier;
use serde::Deserialize;
use std::fmt;
use std::path::PathBuf;

pub enum Error {
    Cargo(cargo_metadata::Error),
    SerdeJson(serde_json::Error),
    PackageNotFound,
    MetadataNotFound,
}

impl From<cargo_metadata::Error> for Error {
    fn from(err: cargo_metadata::Error) -> Error {
        Error::Cargo(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::SerdeJson(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Cargo(error) => write!(f, "Cargo Error: {}", error),
            Error::SerdeJson(error) => write!(f, "Serde Error: {}", error),
            Error::PackageNotFound => write!(f, "{}: package not found", env!("CARGO_PKG_NAME")),
            Error::MetadataNotFound => write!(f, "package.metadata.icu4x_testdata not found"),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self as &dyn fmt::Display).fmt(f)
    }
}

#[derive(Debug, Deserialize)]
pub struct PackageMetadata {
    pub locales: Vec<LanguageIdentifier>,
    pub gitref: String,
}

#[derive(Debug)]
pub struct PackageInfo {
    pub target_directory: PathBuf,
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
    use icu_locale_macros::langid;
    let package_info = load().expect("Failed to load metadata");
    assert!(package_info
        .package_metadata
        .locales
        .contains(&(langid!("und"))));
    assert!(package_info.package_metadata.locales.len() > 10);
}
