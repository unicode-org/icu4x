// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

pub mod cldr_allinone;
pub mod cldr_components;

mod error;
mod io_util;

pub use error::Error;

use crate::CldrPaths;

/// An enum representing either the All-in-One or the Components style of CLDR JSON zip files.
#[derive(Debug, PartialEq, Eq)]
pub enum CldrJsonDistributionType {
    Components,
    AllInOne,
}

/// Returns the correct zip file style for the given tag.
///
/// # Example
///
/// ```
/// use icu_provider_cldr::download::CldrJsonDistributionType;
/// use icu_provider_cldr::download::check_github_tag;
///
/// assert!(matches!(
///     check_github_tag("37.0.0"),
///     Ok(CldrJsonDistributionType::Components)
/// ));
/// assert!(matches!(
///     check_github_tag("38.1.0-BETA4"),
///     Ok(CldrJsonDistributionType::AllInOne)
/// ));
/// assert!(matches!(
///     check_github_tag("foo"),
///     Err(_)
/// ));
/// ```
pub fn check_github_tag(github_tag: &str) -> Result<CldrJsonDistributionType, Error> {
    let period_pos = github_tag
        .chars()
        .position(|c| c == '.')
        .ok_or(Error::InvalidGitHubTag)?;
    let version: usize = github_tag[..period_pos]
        .parse()
        .map_err(|_| Error::InvalidGitHubTag)?;
    Ok(if version < 38 {
        CldrJsonDistributionType::Components
    } else {
        CldrJsonDistributionType::AllInOne
    })
}

/// Returns a boxed CldrPaths for the given GitHub tag.
pub fn try_from_github_tag(github_tag: &str) -> Result<Box<dyn CldrPaths>, Error> {
    match check_github_tag(github_tag)? {
        CldrJsonDistributionType::Components => {
            let downloader =
                cldr_components::CldrComponentsDownloader::try_from_github_tag(github_tag)?;
            Ok(Box::new(downloader))
        }
        CldrJsonDistributionType::AllInOne => {
            let downloader =
                cldr_allinone::CldrAllInOneDownloader::try_from_github_tag(github_tag)?;
            Ok(Box::new(downloader))
        }
    }
}
