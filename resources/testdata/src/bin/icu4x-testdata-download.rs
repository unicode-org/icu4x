// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}

struct CldrJsonDownloader<'a> {
    /// Repo owner and name, like "unicode-org/cldr-json"
    pub repo_owner_and_name: &'a str,
    /// Git tag or ref, like "39.0.0"
    pub tag: &'a str,
    /// Downloader client
    pub client: reqwest::Client,
}

#[derive(Debug, serde::Deserialize)]
struct GitHubFileListEntry {
    pub name: String,
    pub path: String,
}

impl CldrJsonDownloader<'_> {
    async fn list_github_dir(&self, dir: &str) -> Result<Vec<GitHubFileListEntry>, Error> {
        let url = format!(
            "https://api.github.com/repos/{}/contents/cldr-json/{}?ref={}",
            self.repo_owner_and_name, dir, self.tag
        );
        let response = self
            .client
            .get(url)
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    static APP_USER_AGENT: &str = concat!(
        env!("CARGO_PKG_NAME"),
        "/",
        env!("CARGO_PKG_VERSION"),
    );
    let downloader = CldrJsonDownloader {
        repo_owner_and_name: "unicode-org/cldr-json",
        tag: "39.0.0",
        client: reqwest::ClientBuilder::new()
            .user_agent(APP_USER_AGENT)
            .build()?,
    };
    println!("{:?}", downloader.list_github_dir("").await?);
    todo!()
}
