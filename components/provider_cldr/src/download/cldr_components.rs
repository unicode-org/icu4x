// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

//! Data downloader for CLDR JSON versions 37 and older. Prior to CLDR 38, the JSON for each
//! component was distributed separately as standalone zip files.

use super::error::Error;
use super::io_util;
use crate::CldrPaths;
use std::path::PathBuf;

/// Implementation of CldrPaths that downloads CLDR data directories on demand.
/// The download artifacts are saved in the user's cache directory; see
/// https://docs.rs/dirs/3.0.0/dirs/fn.cache_dir.html
///
/// Downloads separate zip files for each component, as was used in CLDR 37 and earlier.
///
/// # Example
///
/// ```
/// use icu_provider::prelude::*;
/// use icu_provider::structs;
/// use icu_provider_cldr::CldrPaths;
/// use icu_provider_cldr::download::cldr_components::CldrComponentsDownloader;
/// use icu_provider_cldr::transform::PluralsProvider;
/// use icu_locid_macros::langid;
/// use std::path::PathBuf;
///
/// let paths = CldrComponentsDownloader::try_from_github_tag("36.0.0")
///     .expect("Cache directory not found");
///
/// fn demo(paths: &dyn CldrPaths) {
///     use std::borrow::Cow;
///     use std::convert::TryFrom;
///     use icu_provider::prelude::*;
///     use icu_provider::structs;
///
///     let data_provider = PluralsProvider::try_from(paths)
///         .expect("The data should be well-formed after downloading");
///
///     let data: Cow<structs::plurals::PluralRuleStringsV1> = data_provider
///         .load_payload(&DataRequest {
///             resource_path: ResourcePath {
///                 key: structs::plurals::key::ORDINAL_V1,
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
pub struct CldrComponentsDownloader {
    /// Directory where downloaded files are stored.
    pub cache_dir: PathBuf,

    pub cldr_core: CldrComponentZipFileInfo,
    pub cldr_dates: CldrComponentZipFileInfo,
}

// TODO(#297): Implement this async.
impl CldrPaths for CldrComponentsDownloader {
    fn cldr_core(&self) -> Result<PathBuf, crate::error::Error> {
        self.cldr_core.download_and_unzip(&self)
    }
    fn cldr_dates(&self) -> Result<PathBuf, crate::error::Error> {
        self.cldr_dates.download_and_unzip(&self)
    }
}

impl CldrComponentsDownloader {
    /// Creates a CldrComponentsDownloader that downloads files to the system cache directory
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
            cldr_core: CldrComponentZipFileInfo {
                url: format!(
                    "https://github.com/unicode-cldr/cldr-core/archive/{}.zip",
                    github_tag
                ),
                top_dir: format!("cldr-core-{}", github_tag),
            },
            cldr_dates: CldrComponentZipFileInfo {
                url: format!(
                    "https://github.com/unicode-cldr/cldr-dates-full/archive/{}.zip",
                    github_tag
                ),
                top_dir: format!("cldr-dates-full-{}", github_tag),
            },
        })
    }
}

#[derive(Debug)]
pub struct CldrComponentZipFileInfo {
    /// The URL to the remote zip file
    pub url: String,
    /// The directory name in the unpacked zip fle
    pub top_dir: String,
}

impl CldrComponentZipFileInfo {
    fn download_and_unzip(
        &self,
        parent: &CldrComponentsDownloader,
    ) -> Result<PathBuf, crate::error::Error> {
        io_util::download_and_unzip(&self.url, &parent.cache_dir)
            .map(|p| p.join(&self.top_dir))
            .map_err(|e| e.into())
    }
}
