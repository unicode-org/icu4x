use super::io_util;
use crate::error::Error;
use crate::CldrPaths;
use std::path::PathBuf;

/// Implementation of CldrPaths that downloads CLDR data directories on demand.
///
/// # Example
///
/// ```
/// use icu_cldr_json_data_provider::download::CldrPathsDownload;
/// use icu_cldr_json_data_provider::CldrJsonDataProvider;
/// use std::path::PathBuf;
///
/// let paths = CldrPathsDownload::from_github_tag("36.0.0");
///
/// let data_provider = CldrJsonDataProvider::new(&paths);
///
/// fn demo<'d>(data_provider: &'d CldrJsonDataProvider<'d, 'd>) {
///     use std::borrow::Cow;
///     use icu_data_provider::prelude::*;
///     use icu_data_provider::structs::plurals::PluralRuleStringsV1;
///
///     let data: Cow<PluralRuleStringsV1> = data_provider
///         .load(&DataRequest {
///             data_entry: DataEntry {
///                 langid: "uk".parse().unwrap(),
///                 variant: None,
///             },
///             data_key: icu_data_key!(plurals: ordinal@1),
///         })
///         .unwrap()
///         .take_payload()
///         .unwrap();
///     assert_eq!(data.few, Some(Cow::Borrowed("n % 10 = 3 and n % 100 != 13")));
/// }
///
/// // Calling demo(&data_provider) will cause the data to actually get downloaded.
/// //demo(&data_provider);
/// ```
pub struct CldrPathsDownload {
    /// Directory where downloaded files are stored.
    pub cache_dir: PathBuf,

    pub cldr_core: CldrZipFileInfo,
    pub cldr_dates: CldrZipFileInfo,
}

// TODO(#297): Implement this async.
impl CldrPaths for CldrPathsDownload {
    fn cldr_core(&self) -> Result<PathBuf, Error> {
        self.cldr_core.download_and_unzip(&self)
    }
    fn cldr_dates(&self) -> Result<PathBuf, Error> {
        self.cldr_dates.download_and_unzip(&self)
    }
}

impl CldrPathsDownload {
    /// Creates a CldrPathsDownload that downloads files to the system cache directory
    /// as determined by dirs::cache_dir().
    ///
    /// github_tag should be a tag in the CLDR JSON repositories, such as "36.0.0":
    /// https://github.com/unicode-cldr/cldr-core/tags
    pub fn from_github_tag(github_tag: &str) -> Self {
        Self {
            cache_dir: dirs::cache_dir().unwrap().join("icu4x").join("cldr"),
            cldr_core: CldrZipFileInfo {
                url: format!(
                    "https://github.com/unicode-cldr/cldr-core/archive/{}.zip",
                    github_tag
                ),
                top_dir: format!("cldr-core-{}", github_tag),
            },
            cldr_dates: CldrZipFileInfo {
                url: format!(
                    "https://github.com/unicode-cldr/cldr-dates-modern/archive/{}.zip",
                    github_tag
                ),
                top_dir: format!("cldr-dates-modern-{}", github_tag),
            },
        }
    }
}

pub struct CldrZipFileInfo {
    /// The URL to the remote zip file
    pub url: String,
    /// The directory name in the unpacked zip fle
    pub top_dir: String,
}

impl CldrZipFileInfo {
    fn download_and_unzip(&self, parent: &CldrPathsDownload) -> Result<PathBuf, Error> {
        io_util::download_and_unzip(&self.url, &parent.cache_dir)
            .map(|p| p.join(&self.top_dir))
            .map_err(|e| e.into())
    }
}
