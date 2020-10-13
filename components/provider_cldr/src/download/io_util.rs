// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use super::error::Error;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::time::Instant;
use unzip::Unzipper;

#[cfg(test)]
fn assert_files_eq(expected_file_path: &Path, actual_file_path: &Path) {
    use std::io::Read;
    let mut expected_buf = Vec::new();
    File::open(expected_file_path)
        .unwrap()
        .read_to_end(&mut expected_buf)
        .unwrap();
    let mut actual_buf = Vec::new();
    File::open(&actual_file_path)
        .unwrap()
        .read_to_end(&mut actual_buf)
        .unwrap();
    assert_eq!(expected_buf, actual_buf);
}

// Synchronously download url and save it to destination.
// TODO(#297): Implement this async.
fn download_sync(url: &str, destination: &Path) -> Result<(), Error> {
    log::info!("Downloading: {}", url);
    let start = Instant::now();
    let mut response = reqwest::blocking::get(url)?;
    if !response.status().is_success() {
        return Err(Error::HttpStatus(response.status(), url.to_string()));
    }
    log::info!("Status: {}", response.status());
    let mut file = File::create(destination).map_err(|e| (e, destination))?;
    response.copy_to(&mut file)?;
    log::info!("Finished in {:.2} seconds", start.elapsed().as_secs_f64());
    Ok(())
}

// TODO(#333): re-enable with caching to prevent flakiness
#[test]
#[ignore]
fn test_download_sync() -> Result<(), Error> {
    let temp_file = mktemp::Temp::new_file().map_err(|e| Error::Io(e, None))?;
    download_sync(
        "https://www.w3.org/WAI/ER/tests/xhtml/testfiles/resources/pdf/dummy.pdf",
        &temp_file,
    )?;
    assert_files_eq(&PathBuf::from("./tests/testdata/dummy.pdf"), &temp_file);
    Ok(())
}

/// Synchronously unpack a zip file into a destination directory.
// TODO(#297): Implement this async.
fn unzip_sync(zip_path: &Path, dir_path: &Path) -> Result<(), Error> {
    let reader = File::open(zip_path).map_err(|e| (e, zip_path))?;
    log::info!("Unzipping...");
    let start = Instant::now();
    Unzipper::new(reader, dir_path)
        .unzip()
        .map_err(|e| (e, dir_path))?;
    log::info!("Unzipped in {:.2} seconds", start.elapsed().as_secs_f64());
    Ok(())
}

#[test]
fn test_unzip_sync() -> Result<(), Error> {
    let temp_dir = mktemp::Temp::new_dir().map_err(|e| Error::Io(e, None))?;
    unzip_sync(&PathBuf::from("./tests/testdata/dummy.zip"), &temp_dir)?;
    assert_files_eq(
        &PathBuf::from("./tests/testdata/dummy.pdf"),
        &temp_dir.to_path_buf().join("dummy.pdf"),
    );
    Ok(())
}

/// Downloads and unpacks a zip file, returning the path to the unpacked directory.
///
/// `cache_dir` is a directory where both the zip file and the unpacked directory will be
/// saved. If the zip file has already been downloaded, it will not be downloaded again.
pub fn download_and_unzip(zip_file_url: &str, cache_dir: &Path) -> Result<PathBuf, Error> {
    fs::create_dir_all(cache_dir).map_err(|e| (e, cache_dir))?;

    let zip_dir = cache_dir.to_path_buf().join("zips");
    fs::create_dir_all(&zip_dir).map_err(|e| (e, &zip_dir))?;

    let data_dir = cache_dir.to_path_buf().join("data");
    fs::create_dir_all(&data_dir).map_err(|e| (e, &data_dir))?;

    let basename = urlencoding::encode(zip_file_url);
    let mut zip_path = zip_dir.join(&basename);
    zip_path.set_extension("zip");
    let dir_path = data_dir.join(&basename);

    if !zip_path.exists() {
        download_sync(zip_file_url, &zip_path)?;
    }

    if !dir_path.exists() {
        unzip_sync(&zip_path, &dir_path)?;
    }

    Ok(dir_path)
}
