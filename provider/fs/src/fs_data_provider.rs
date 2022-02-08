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

    fn get_reader(
        &self,
        key: ResourceKey,
        req: &DataRequest,
    ) -> Result<(impl Read, PathBuf), DataError> {
        let mut path_buf = self.res_root.clone();
        path_buf.push(&*key.writeable_to_string());
        if req.options.is_empty() {
            path_buf.set_extension(self.manifest.get_file_extension());
        }
        if !path_buf.exists() {
            return Err(DataErrorKind::MissingResourceKey.with_req(key, req));
        }
        if !req.options.is_empty() {
            // TODO: Implement proper locale fallback
            path_buf.push(&*req.options.writeable_to_string());
            path_buf.set_extension(self.manifest.get_file_extension());
        }
        if !path_buf.exists() {
            return Err(DataErrorKind::MissingResourceOptions.with_req(key, req));
        }
        let file = File::open(&path_buf).map_err(|e| DataError::from(e).with_path(&path_buf))?;
        Ok((BufReader::new(file), path_buf))
    }

    fn get_rc_buffer(
        &self,
        key: ResourceKey,
        req: &DataRequest,
    ) -> Result<(Rc<[u8]>, PathBuf), DataError> {
        let (mut reader, path_buf) = self.get_reader(key, req)?;
        let mut buffer = Vec::<u8>::new();
        reader
            .read_to_end(&mut buffer)
            .map_err(|e| DataError::from(e).with_path(&path_buf))?;
        let rc_buffer: Rc<[u8]> = buffer.into();
        Ok((rc_buffer, path_buf))
    }
}

impl BufferProvider for FsDataProvider {
    fn load_buffer(
        &self,
        key: ResourceKey,
        req: &DataRequest,
    ) -> Result<DataResponse<BufferMarker>, DataError> {
        let (rc_buffer, _) = self.get_rc_buffer(key, req)?;
        let mut metadata = DataResponseMetadata::default();
        // TODO(#1109): Set metadata.data_langid correctly.
        metadata.buffer_format = Some(self.manifest.buffer_format);
        Ok(DataResponse {
            metadata,
            payload: Some(DataPayload::from_rc_buffer(rc_buffer)),
        })
    }
}

impl<M> ResourceProvider<M> for FsDataProvider
where
    M: ResourceMarker,
    // Actual bound:
    //     for<'de> <M::Yokeable as Yokeable<'de>>::Output: serde::de::Deserialize<'de>,
    // Necessary workaround bound (see `yoke::trait_hack` docs):
    for<'de> YokeTraitHack<<M::Yokeable as Yokeable<'de>>::Output>: serde::de::Deserialize<'de>,
{
    fn load_resource(&self, req: &DataRequest) -> Result<DataResponse<M>, DataError> {
        self.as_deserializing().load_resource(req)
    }
}

impl<M> DynProvider<M> for FsDataProvider
where
    M: DataMarker,
    // Actual bound:
    //     for<'de> <M::Yokeable as Yokeable<'de>>::Output: serde::de::Deserialize<'de>,
    // Necessary workaround bound (see `yoke::trait_hack` docs):
    for<'de> YokeTraitHack<<M::Yokeable as Yokeable<'de>>::Output>: serde::de::Deserialize<'de>,
{
    fn load_payload(
        &self,
        key: ResourceKey,
        req: &DataRequest,
    ) -> Result<DataResponse<M>, DataError> {
        self.as_deserializing().load_payload(key, req)
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
            options.push(ResourceOptions::default());
        }

        let key_root = key_root
            .canonicalize()
            .map_err(|e| DataErrorKind::Io(e.kind()).with_key(*resc_key))?;
        let key_root = key_root.to_str().ok_or_else(|| {
            DataErrorKind::Io(std::io::ErrorKind::InvalidData).with_key(*resc_key)
        })?;

        let pattern = format!("{}/**/*.{}", key_root, self.manifest.get_file_extension());
        let prefix_len = key_root.len() + 1;
        let suffix_len = 1 + self.manifest.get_file_extension().len();

        options.extend(
            glob::glob(&pattern)
                .expect("Failed to read glob pattern")
                .filter_map(|r| r.ok())
                .filter_map(|path| {
                    path.to_str()
                        .and_then(|s| s[prefix_len..s.len() - suffix_len].parse().ok())
                }),
        );

        Ok(Box::new(options.into_iter()))
    }
}
