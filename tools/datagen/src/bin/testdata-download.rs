// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use anyhow::Context;
use clap::{value_t, App, Arg, ArgMatches};
use futures::stream::{self, StreamExt, TryStreamExt};
use icu_testdata::metadata::{self, PackageInfo};
use simple_logger::SimpleLogger;
use std::path::PathBuf;
use tokio::fs;

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

impl CldrJsonDownloader<'_> {
    async fn fetch(&self, cldr_path: &str) -> anyhow::Result<()> {
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
        {
            use tokio::io::AsyncWriteExt;
            while let Some(item) = stream.next().await {
                file.write_buf(&mut item?)
                    .await
                    .with_context(|| format!("Failed to write to file: {:?}", &local_path))?;
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cldr_json_root = icu_testdata::paths::cldr_json_root();

    let args = App::new("ICU4X Test Data Downloader")
        .version("0.0.1")
        .author("The ICU4X Project Developers")
        .about("Download data from CLDR for ICU4X testing")
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
                    "Path to output data directory. The directory will be overwritten. Omit this option to write data into the package tree.",
                )
                .takes_value(true)
                .default_value_os(cldr_json_root.as_os_str()),
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
        0 => SimpleLogger::from_env().init().unwrap(),
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
        _ => anyhow::bail!("Only -v, -vv, and -vvv are supported"),
    }

    let metadata = metadata::load()?;
    log::debug!("Package metadata: {:?}", metadata);

    fs::remove_dir_all(&cldr_json_root)
        .await
        .with_context(|| format!("Failed to delete directory: {:?}", &cldr_json_root))?;

    download_cldr(&args, &metadata).await?;

    Ok(())
}

async fn download_cldr(args: &ArgMatches<'_>, metadata: &PackageInfo) -> anyhow::Result<()> {
    let output_path = PathBuf::from(
        args.value_of_os("OUTPUT")
            .expect("Option has a default value"),
    );
    let http_concurrency: usize =
        value_t!(args, "HTTP_CONCURRENCY", usize).expect("Option has a default value");

    let downloader = &CldrJsonDownloader {
        repo_owner_and_name: "unicode-org/cldr-json",
        tag: &metadata.package_metadata.gitref,
        root_dir: output_path,
        client: reqwest::ClientBuilder::new()
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .build()?,
    };

    let all_paths = metadata.package_metadata.get_all_cldr_paths();
    stream::iter(all_paths)
        .map(Ok)
        .try_for_each_concurrent(http_concurrency, |path| async move {
            log::info!("Downloading: {}", path);
            downloader.fetch(&path).await
        })
        .await?;
    Ok(())
}
