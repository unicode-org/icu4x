// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data downloader for CLDR JSON versions 38 and later. Starting in CLDR 38, the JSON for all
//! components is shipped in a single zip file.

use super::error::Error;
use super::io_util;
use crate::CldrPathsAllInOne;
use std::path::PathBuf;

/// Implementation of CldrPaths that downloads CLDR data directories on demand.
/// The download artifacts are saved in the user's [cache directory](
/// https://docs.rs/dirs/3.0.0/dirs/fn.cache_dir.html).
///
/// Downloads a single zip file for all components, as used in CLDR 38 and later.
///
/// # Examples
///
/// ```
/// use icu_provider::prelude::*;
/// use icu_provider_cldr::CldrPaths;
/// use icu_provider_cldr::download::CldrAllInOneDownloader;
/// use icu_provider_cldr::transform::PluralsProvider;
/// use icu_locid_macros::langid;
/// use std::path::PathBuf;
///
/// let downloader = CldrAllInOneDownloader::try_new_from_github("38.1.0", "modern")
///     .expect("Cache directory not found");
///
/// fn demo(downloader: CldrAllInOneDownloader) {
///     use std::borrow::Cow;
///     use std::convert::TryFrom;
///     use icu_provider::prelude::*;
///
///     let paths = downloader.download(None)
///         .expect("The data should download successfully");
///
///     let data_provider = PluralsProvider::try_from(&paths as &dyn CldrPaths)
///         .expect("The data should be well-formed after downloading");
///
///     let data: DataPayload<icu_plurals::provider::PluralRulesV1Marker> = data_provider
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
///     let rule = "n % 10 = 3 and n % 100 != 13".parse()
///         .expect("Failed to parse plural rule");
///     assert_eq!(data.get().few, Some(rule));
/// }
///
/// // Calling demo(downloader) will cause the data to actually get downloaded.
/// //demo(downloader);
/// ```
#[derive(Debug)]
pub struct CldrAllInOneDownloader {
    /// Directory where downloaded files are stored
    pub cache_dir: PathBuf,

    /// The URL to the remote zip file
    pub url: String,

    /// CLDR JSON locale subset: "full" or "modern"
    pub locale_subset: String,
}

impl CldrAllInOneDownloader {
    /// Creates a [`CldrAllInOneDownloader`] that downloads files to the system cache directory as
    /// determined by [`dirs::cache_dir()`](dirs::cache_dir()).
    ///
    /// Arguments:
    ///
    /// - `github_tag`: a [tag in the CLDR JSON repositories](https://github.com/unicode-cldr/cldr-json/tags),
    ///    such as "38.1.0"
    /// - `locale_subset`: either "modern" (fewer locales, smaller download) or "full" (more
    ///   locales, larger download)
    pub fn try_new_from_github(github_tag: &str, locale_subset: &str) -> Result<Self, Error> {
        Ok(Self {
            cache_dir: dirs::cache_dir()
                .ok_or(Error::NoCacheDir)?
                .join("icu4x")
                .join("cldr"),
            url: format!(
                "https://github.com/unicode-org/cldr-json/releases/download/{}/cldr-{}-json-{}.zip",
                github_tag, github_tag, locale_subset
            ),
            locale_subset: locale_subset.to_string(),
        })
    }

    pub fn download(self, uprops_root: Option<PathBuf>) -> Result<CldrPathsAllInOne, Error> {
        // TODO(#297): Implement this async.
        let downloaded = io_util::download_and_unzip(&self.url, &self.cache_dir)?;
        Ok(CldrPathsAllInOne {
            cldr_json_root: downloaded,
            locale_subset: self.locale_subset,
            uprops_root,
        })
    }
}
