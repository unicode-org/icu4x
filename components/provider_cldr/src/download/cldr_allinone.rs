// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

//! Data downloader for CLDR JSON versions 38 and later. Starting in CLDR 38, the JSON for all
//! components is shipped in a single zip file.

use super::error::Error;
use super::io_util;
use crate::CldrPaths;
use std::path::PathBuf;

/// Implementation of CldrPaths that downloads CLDR data directories on demand.
/// The download artifacts are saved in the user's cache directory; see
/// https://docs.rs/dirs/3.0.0/dirs/fn.cache_dir.html
///
/// Downloads a single zip file for all components, as used in CLDR 38 and later.
///
/// # Example
///
/// ```
/// use icu_provider::prelude::*;
/// use icu_provider_cldr::CldrPaths;
/// use icu_provider_cldr::download::CldrAllInOneDownloader;
/// use icu_provider_cldr::transform::PluralsProvider;
/// use icu_locid_macros::langid;
/// use std::path::PathBuf;
///
/// let paths = CldrAllInOneDownloader::try_from_github_tag("38.1.0-BETA4")
///     .expect("Cache directory not found");
///
/// fn demo(paths: &dyn CldrPaths) {
///     use std::borrow::Cow;
///     use std::convert::TryFrom;
///     use icu_provider::prelude::*;
///
///     let data_provider = PluralsProvider::try_from(paths)
///         .expect("The data should be well-formed after downloading");
///
///     let data: Cow<icu_plurals::provider::PluralRuleStringsV1> = data_provider
///         .load_payload(&DataRequest {
///             resource_path: ResourcePath {
///                 key: icu_plurals::provider::key::ORDINAL_V1,
///                 options: ResourceOptions {
///                     langid: Some(langid!("uk")),
///                     variant: None,
///                 },
///             },
///         })
///         .unwrap()
///         .take_payload()
///         .unwrap();
///     assert_eq!(data.few, Some(Cow::Borrowed("n % 10 = 3 and n % 100 != 13")));
/// }
///
/// // Calling demo(&paths) will cause the data to actually get downloaded.
/// //demo(&paths);
/// ```
#[derive(Debug)]
pub struct CldrAllInOneDownloader {
    /// Directory where downloaded files are stored
    pub cache_dir: PathBuf,

    /// The URL to the remote zip file
    pub url: String,

    /// CLDR JSON directory suffix: probably either "modern" or "full"
    pub suffix: String,
}

// TODO(#297): Implement this async.
impl CldrPaths for CldrAllInOneDownloader {
    fn cldr_core(&self) -> Result<PathBuf, crate::error::Error> {
        self.download_and_unzip()
            .map(|p| p.join("cldr-core".to_string()))
    }
    fn cldr_dates(&self) -> Result<PathBuf, crate::error::Error> {
        self.download_and_unzip()
            .map(|p| p.join(format!("cldr-dates-{}", self.suffix)))
    }
}

impl CldrAllInOneDownloader {
    /// Creates a CldrAllInOneDownloader that downloads files to the system cache directory
    /// as determined by dirs::cache_dir().
    ///
    /// github_tag should be a tag in the CLDR JSON repositories, such as "36.0.0":
    /// https://github.com/unicode-cldr/cldr-core/tags
    pub fn try_from_github_tag(github_tag: &str) -> Result<Self, Error> {
        Ok(Self {
            cache_dir: dirs::cache_dir()
                .ok_or(Error::NoCacheDir)?
                .join("icu4x")
                .join("cldr"),
            url: format!(
                "https://github.com/unicode-org/cldr-json/releases/download/{}/cldr-{}-json-full.zip",
                github_tag, github_tag
            ),
            suffix: "full".to_string(),
        })
    }

    fn download_and_unzip(&self) -> Result<PathBuf, crate::error::Error> {
        io_util::download_and_unzip(&self.url, &self.cache_dir).map_err(|e| e.into())
    }
}
