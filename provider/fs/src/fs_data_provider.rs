// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::Error;
use crate::manifest::Manifest;
use crate::manifest::MANIFEST_FILE;
use icu_provider::iter::IterableProvider;
use icu_provider::prelude::*;
use icu_provider::serde::*;
use icu_provider::yoke::trait_hack::YokeTraitHack;
use icu_provider::yoke::Yokeable;
use writeable::Writeable;

use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::PathBuf;
use std::rc::Rc;

/// A data provider that reads ICU4X data from a filesystem directory.
///
/// # Examples
///
/// ```
/// use icu_provider_fs::FsDataProvider;
///
/// let provider = FsDataProvider::try_new("/path/to/data/directory")
///     .expect_err("Specify a real directory in the line above");
/// ```
#[derive(Debug, PartialEq)]
pub struct FsDataProvider {
    res_root: PathBuf,
    manifest: Manifest,
}

impl FsDataProvider {
    /// Create a new [`FsDataProvider`] given a filesystem directory.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider_fs::FsDataProvider;
    ///
    /// let provider = FsDataProvider::try_new("/path/to/data/directory")
    ///     .expect_err("Specify a real directory in the line above");
    /// ```
    pub fn try_new<T: Into<PathBuf>>(root: T) -> Result<Self, Error> {
        let root_path_buf: PathBuf = root.into();
        let manifest_path = root_path_buf.join(MANIFEST_FILE);
        let manifest_str = fs::read_to_string(&manifest_path).map_err(|e| (e, &manifest_path))?;
        let manifest: Manifest = serde_json_core::from_str(&manifest_str)
            .map(|(obj, _)| obj)
            .map_err(|e| (e, &manifest_path))?;
        check_format_supported(manifest.buffer_format)?;
        Ok(Self {
            res_root: root_path_buf,
            manifest,
        })
    }

    fn get_reader(&self, req: &DataRequest) -> Result<(impl Read, PathBuf), DataError> {
        let mut path_buf = self.res_root.clone();
        path_buf.push(&*req.resource_path.key.writeable_to_string());
        if req.resource_path.options.is_empty() {
            path_buf.set_extension(self.manifest.get_file_extension());
        }
        if !path_buf.exists() {
            return Err(DataErrorKind::MissingResourceKey.with_req(req));
        }
        if !req.resource_path.options.is_empty() {
            // TODO: Implement proper locale fallback
            path_buf.push(&*req.resource_path.options.writeable_to_string());
            path_buf.set_extension(self.manifest.get_file_extension());
        }
        if !path_buf.exists() {
            return Err(DataErrorKind::MissingResourceOptions.with_req(req));
        }
        let file = File::open(&path_buf).map_err(|e| DataError::from(e).with_path(&path_buf))?;
        Ok((BufReader::new(file), path_buf))
    }

    fn get_rc_buffer(&self, req: &DataRequest) -> Result<(Rc<[u8]>, PathBuf), DataError> {
        let (mut reader, path_buf) = self.get_reader(req)?;
        let mut buffer = Vec::<u8>::new();
        reader
            .read_to_end(&mut buffer)
            .map_err(|e| DataError::from(e).with_path(&path_buf))?;
        let rc_buffer: Rc<[u8]> = buffer.into();
        Ok((rc_buffer, path_buf))
    }
}

impl BufferProvider for FsDataProvider {
    fn load_buffer(&self, req: &DataRequest) -> Result<DataResponse<BufferMarker>, DataError> {
        let (rc_buffer, _) = self.get_rc_buffer(req)?;
        let mut metadata = DataResponseMetadata::default();
        // TODO(#1109): Set metadata.data_langid correctly.
        metadata.buffer_format = Some(self.manifest.buffer_format);
        Ok(DataResponse {
            metadata,
            payload: Some(DataPayload::from_rc_buffer(rc_buffer)),
        })
    }
}

impl<M> DataProvider<M> for FsDataProvider
where
    M: DataMarker,
    // Actual bound:
    //     for<'de> <M::Yokeable as Yokeable<'de>>::Output: serde::de::Deserialize<'de>,
    // Necessary workaround bound (see `yoke::trait_hack` docs):
    for<'de> YokeTraitHack<<M::Yokeable as Yokeable<'de>>::Output>: serde::de::Deserialize<'de>,
{
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<M>, DataError> {
        self.as_deserializing().load_payload(req)
    }
}

impl IterableProvider for FsDataProvider {
    fn supported_options_for_key(
        &self,
        resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions> + '_>, DataError> {
        let mut key_root = self.res_root.clone();
        key_root.push(resc_key.get_path());

        let mut options = Vec::new();

        if key_root
            .with_extension(self.manifest.get_file_extension())
            .exists()
        {
            options.push(ResourceOptions::from_parts(core::iter::empty()));
        }

        let is_data_file = |entry: &std::fs::DirEntry| {
            let t = entry.file_type().expect("IO");
            (t.is_file()
                || (t.is_symlink()
                    && self.manifest.aliasing == crate::manifest::AliasOption::Symlink))
                && entry
                    .file_name()
                    .to_string_lossy()
                    .ends_with(self.manifest.get_file_extension())
        };

        if key_root.exists() && key_root.is_dir() {
            for level1 in fs::read_dir(key_root).expect("IO") {
                let level1 = level1.expect("IO");

                if is_data_file(&level1) {
                    options.push(ResourceOptions::from_parts(core::iter::once(
                        level1
                            .file_name()
                            .to_string_lossy()
                            .strip_suffix(self.manifest.get_file_extension())
                            .unwrap()
                            .strip_suffix('.')
                            .unwrap(),
                    )))
                } else if level1.file_type().unwrap().is_dir() {
                    for level2 in fs::read_dir(level1.path()).expect("IO") {
                        let level2 = level2.expect("IO");
                        if is_data_file(&level2) {
                            options.push(ResourceOptions::from_parts(IntoIterator::into_iter([
                                &*level1.file_name().to_string_lossy(),
                                level2
                                    .file_name()
                                    .to_string_lossy()
                                    .strip_suffix(self.manifest.get_file_extension())
                                    .unwrap()
                                    .strip_suffix('.')
                                    .unwrap(),
                            ])));
                        }
                    }
                }
            }
        }

        Ok(Box::new(options.into_iter()))
    }
}
