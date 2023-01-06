// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use bytes::Bytes;
use clap::{value_t, App, Arg};
use eyre::WrapErr;
use futures::stream::{self, StreamExt, TryStreamExt};
use simple_logger::SimpleLogger;
use std::io::Cursor;
use std::{io::Read, path::PathBuf};
use tokio::{fs, io::AsyncWriteExt};
use zip::ZipArchive;

#[derive(Debug, serde::Deserialize)]
struct GlobMetadata {
    cldr_json_glob: Vec<String>,
    icuexportdata_glob: Vec<String>,
}

fn load_sources() -> GlobMetadata {
    #[allow(clippy::unwrap_used)] // the TOML source is a constant
    toml::from_str(include_str!("../../globs.toml")).unwrap()
}

#[derive(Clone)]
struct CldrJsonDownloader<'a> {
    /// Repo owner and name, like "unicode-org/cldr-json"
    pub repo_owner_and_name: &'a str,
    /// Git tag or ref, like "39.0.0"
    pub tag: &'a str,
    /// Root directory to save downloaded files
    pub root_dir: PathBuf,
    /// Downloader client
    pub client: reqwest::Client,
}

fn expand_paths(in_paths: Vec<String>) -> Vec<String> {
    let mut paths = vec![];
    for pattern in in_paths {
        if pattern.contains("$LOCALES") {
            for locale in icu_testdata::locales().iter() {
                paths.push(pattern.replace("$LOCALES", &locale.to_string()));
            }
            // Also add "root" for older CLDRs
            paths.push(pattern.replace("$LOCALES", "root"));
        } else {
            // No variable in pattern
            paths.push(pattern)
        }
    }
    paths
}

impl CldrJsonDownloader<'_> {
    async fn fetch(&self, cldr_path: &str) -> eyre::Result<()> {
        let url = format!(
            "https://raw.githubusercontent.com/{}/{}/cldr-json/{}",
            self.repo_owner_and_name, self.tag, cldr_path
        );
        let response = self.client.get(&url).send().await?;
        let mut stream = match response.error_for_status() {
            Ok(res) => res.bytes_stream(),
            Err(err) => {
                // Missing files are expected. Print it as a warning and fail gracefully.
                if err.status() == Some(reqwest::StatusCode::NOT_FOUND) {
                    log::warn!("HTTP 404: {}", &url);
                    return Ok(());
                } else {
                    return Err(err.into());
                }
            }
        };
        let local_path = self.root_dir.join(cldr_path);
        fs::create_dir_all(local_path.parent().unwrap())
            .await
            .with_context(|| format!("Failed to create dir: {:?}", &local_path))?;
        let mut file = fs::File::create(&local_path)
            .await
            .with_context(|| format!("Failed to create file: {:?}", &local_path))?;
        while let Some(item) = stream.next().await {
            file.write_buf(&mut item?)
                .await
                .with_context(|| format!("Failed to write to file: {:?}", &local_path))?;
        }
        Ok(())
    }
}

struct IcuExportDataDownloader<'a> {
    /// Repo owner and name, like "unicode-org/cldr-json"
    pub repo_owner_and_name: &'a str,
    /// Git tag or ref, like "39.0.0"
    pub tag: &'a str,
    /// Root directory to save downloaded files
    pub root_dir: PathBuf,
}

impl IcuExportDataDownloader<'_> {
    /// Returns the reqwest client back to the caller for use later
    async fn download(self, client: &reqwest::Client) -> eyre::Result<IcuExportDataUnzipper> {
        let url = format!(
            "https://github.com/{}/releases/download/{}/icuexportdata_{}.zip",
            self.repo_owner_and_name,
            self.tag,
            self.tag.replace('/', "-")
        );
        log::info!("Downloading {url}");
        let bytes = client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;
        let cursor = Cursor::new(bytes);
        let zip_archive = ZipArchive::new(cursor)?;
        Ok(IcuExportDataUnzipper {
            root_dir: self.root_dir,
            zip_archive,
        })
    }
}

struct IcuExportDataUnzipper {
    /// Root directory to save downloaded files
    pub root_dir: PathBuf,
    /// The in-memory Zip archive
    pub zip_archive: ZipArchive<Cursor<Bytes>>,
}

impl IcuExportDataUnzipper {
    pub async fn unzip(&mut self, path: &str) -> eyre::Result<()> {
        let mut zip_file = self
            .zip_archive
            .by_name(path)
            .with_context(|| format!("Did not find file in zip: {:?}", &path))?;
        let local_path = self.root_dir.join(path);
        fs::create_dir_all(local_path.parent().unwrap())
            .await
            .with_context(|| format!("Failed to create dir: {:?}", &local_path))?;
        let mut file = fs::File::create(&local_path)
            .await
            .with_context(|| format!("Failed to create file: {:?}", &local_path))?;
        // the `zip` crate is not async, so unzip the file into a buffer first
        let mut file_buf = Vec::new();
        zip_file.read_to_end(&mut file_buf)?;
        file.write_all(&file_buf).await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let testdata_data_root = icu_testdata::paths::data_root();

    let args = App::new("ICU4X Test Data Downloader")
        .version("0.0.1")
        .author("The ICU4X Project Developers")
        .about("Download data from CLDR and ICU for ICU4X testing")
        .arg(
            Arg::with_name("VERBOSE")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help("Sets the level of verbosity (-v, -vv, or -vvv)"),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .short("o")
                .long("out")
                .help(
                    "Path to output data directory. The subdirectories 'cldr' and 'icuexport' will be overwritten. Omit this option to write data into the package tree.",
                )
                .takes_value(true)
                .default_value_os(testdata_data_root.as_os_str()),
        )
        .arg(
            Arg::with_name("HTTP_CONCURRENCY")
                .short("j")
                .long("http-concurrency")
                .help(
                    "Maximum number of concurrent HTTP requests",
                )
                .takes_value(true)
                .default_value("4"),
        )
        .get_matches();

    match args.occurrences_of("VERBOSE") {
        0 => SimpleLogger::new()
            .env()
            .with_level(log::LevelFilter::Info)
            .init()
            .unwrap(),
        1 => SimpleLogger::new()
            .with_level(log::LevelFilter::Info)
            .init()
            .unwrap(),
        2 => SimpleLogger::new()
            .with_level(log::LevelFilter::Debug)
            .init()
            .unwrap(),
        3 => SimpleLogger::new()
            .with_level(log::LevelFilter::Trace)
            .init()
            .unwrap(),
        _ => eyre::bail!("Only -v, -vv, and -vvv are supported"),
    }

    let output_path = PathBuf::from(
        args.value_of_os("OUTPUT")
            .expect("Option has a default value"),
    );

    let http_concurrency: usize =
        value_t!(args, "HTTP_CONCURRENCY", usize).expect("Option has a default value");

    let client = reqwest::ClientBuilder::new()
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        ))
        .build()?;

    let tag = icu_testdata::versions::cldr_tag();

    let cldr_downloader = &CldrJsonDownloader {
        repo_owner_and_name: "unicode-org/cldr-json",
        tag: &tag,
        root_dir: output_path.clone().join("cldr"),
        client: client.clone(),
    };
    fs::remove_dir_all(&cldr_downloader.root_dir)
        .await
        .with_context(|| {
            format!(
                "Failed to delete directory: {:?}",
                &cldr_downloader.root_dir
            )
        })?;

    stream::iter(expand_paths(load_sources().cldr_json_glob))
        .map(Ok)
        .try_for_each_concurrent(http_concurrency, |path| async move {
            log::info!("Downloading: {}", path);
            cldr_downloader.fetch(&path).await
        })
        .await?;

    let tag = icu_testdata::versions::icu_tag();

    let icued_downloader = IcuExportDataDownloader {
        repo_owner_and_name: "unicode-org/icu",
        tag: &tag,
        root_dir: output_path.clone().join("icuexport"),
    };
    fs::remove_dir_all(&icued_downloader.root_dir)
        .await
        .with_context(|| {
            format!(
                "Failed to delete directory: {:?}",
                &icued_downloader.root_dir
            )
        })?;

    let mut icued_unzipper = icued_downloader.download(&client).await?;

    let all_paths = expand_paths(load_sources().icuexportdata_glob);
    for path in all_paths {
        log::info!("Unzipping: {}", path);
        icued_unzipper.unzip(&path).await?;
    }

    Ok(())
}
