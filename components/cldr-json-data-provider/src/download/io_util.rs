use super::error::DownloadError;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::time::Instant;
use unzip::Unzipper;

macro_rules! map_io_err {
    ($path_ref:ident) => {
        |err| DownloadError::Io(err, $path_ref.to_owned())
    };
}

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
fn download_sync(url: &str, destination: &Path) -> Result<(), DownloadError> {
    log::info!("Downloading: {}", url);
    let start = Instant::now();
    let mut response = reqwest::blocking::get(url)?;
    if !response.status().is_success() {
        return Err(DownloadError::HttpStatus(
            response.status(),
            url.to_string(),
        ));
    }
    log::info!("Status: {}", response.status());
    let mut file = File::create(destination).map_err(map_io_err!(destination))?;
    response.copy_to(&mut file)?;
    log::info!("Finished in {:.2} seconds", start.elapsed().as_secs_f64());
    Ok(())
}

#[test]
fn test_download_sync() -> Result<(), DownloadError> {
    let temp_file = mktemp::Temp::new_file()?;
    download_sync(
        "https://www.w3.org/WAI/ER/tests/xhtml/testfiles/resources/pdf/dummy.pdf",
        &temp_file,
    )?;
    assert_files_eq(&PathBuf::from("./tests/testdata/dummy.pdf"), &temp_file);
    Ok(())
}

/// Synchronously unpack a zip file into a destination directory.
// TODO(#297): Implement this async.
fn unzip_sync(zip_path: &Path, dir_path: &Path) -> Result<(), DownloadError> {
    let reader = File::open(zip_path).map_err(map_io_err!(zip_path))?;
    log::info!("Unzipping...");
    let start = Instant::now();
    Unzipper::new(reader, dir_path)
        .unzip()
        .map_err(map_io_err!(dir_path))?;
    log::info!("Unzipped in {:.2} seconds", start.elapsed().as_secs_f64());
    Ok(())
}

#[test]
fn test_unzip_sync() -> Result<(), DownloadError> {
    let temp_dir = mktemp::Temp::new_dir()?;
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
pub fn download_and_unzip(zip_file_url: &str, cache_dir: &Path) -> Result<PathBuf, DownloadError> {
    fs::create_dir_all(cache_dir).map_err(map_io_err!(cache_dir))?;

    let zip_dir = cache_dir.to_path_buf().join("zips");
    fs::create_dir_all(&zip_dir).map_err(map_io_err!(zip_dir))?;

    let data_dir = cache_dir.to_path_buf().join("data");
    fs::create_dir_all(&data_dir).map_err(map_io_err!(data_dir))?;

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
