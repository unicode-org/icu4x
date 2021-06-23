// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::deserializer;
use crate::error::Error;
use crate::manifest::Manifest;
use crate::manifest::MANIFEST_FILE;
use icu_provider::prelude::*;
use icu_provider::serde::*;
use icu_provider::yoke::trait_hack::YokeTraitHack;
use icu_provider::yoke::Yokeable;

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
        let manifest: Manifest =
            serde_json::from_str(&manifest_str).map_err(|e| (e, &manifest_path))?;
        Ok(Self {
            res_root: root_path_buf,
            manifest,
        })
    }

    fn get_reader(&self, req: &DataRequest) -> Result<(impl Read, PathBuf), DataError> {
        type Error = DataError;
        let mut path_buf = self.res_root.clone();
        path_buf.extend(req.resource_path.key.get_components().iter());
        if req.resource_path.options.is_empty() {
            path_buf.set_extension(self.manifest.syntax.get_file_extension());
        }
        if !path_buf.exists() {
            return Err(Error::UnsupportedResourceKey(req.resource_path.key));
        }
        if !req.resource_path.options.is_empty() {
            // TODO: Implement proper locale fallback
            path_buf.extend(req.resource_path.options.get_components().iter());
            path_buf.set_extension(self.manifest.syntax.get_file_extension());
        }
        if !path_buf.exists() {
            return Err(Error::UnavailableResourceOptions(req.clone()));
        }
        let file = match File::open(&path_buf) {
            Ok(file) => file,
            Err(err) => return Err(Error::Resource(Box::new(err))),
        };
        Ok((BufReader::new(file), path_buf))
    }

    fn get_rc_buffer(&self, req: &DataRequest) -> Result<(Rc<[u8]>, PathBuf), DataError> {
        let (mut reader, path_buf) = self.get_reader(req)?;
        let mut buffer = Vec::<u8>::new();
        reader
            .read_to_end(&mut buffer)
            .map_err(|e| DataError::Resource(Box::new(Error::Io(e, Some(path_buf.clone())))))?;
        let rc_buffer: Rc<[u8]> = buffer.into();
        Ok((rc_buffer, path_buf))
    }
}

impl<'d, 's, M> DataProvider<'d, 's, M> for FsDataProvider
where
    M: DataMarker<'s>,
    // Actual bound:
    //     for<'de> <M::Yokeable as Yokeable<'de>>::Output: serde::de::Deserialize<'de>,
    // Necessary workaround bound (see `yoke::trait_hack` docs):
    for<'de> YokeTraitHack<<M::Yokeable as Yokeable<'de>>::Output>: serde::de::Deserialize<'de>,
{
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'d, 's, M>, DataError> {
        let (rc_buffer, path_buf) = self.get_rc_buffer(req)?;
        Ok(DataResponse {
            metadata: DataResponseMetadata {
                data_langid: req.resource_path.options.langid.clone(),
            },
            payload: Some(
                DataPayload::try_from_rc_buffer(
                    rc_buffer,
                    deserializer::deserialize_zero_copy::<M>(&self.manifest.syntax),
                )
                .map_err(|e: deserializer::Error| e.into_resource_error(&path_buf))?,
            ),
        })
    }
}

impl SerdeDeDataProvider for FsDataProvider {
    fn load_to_receiver(
        &self,
        req: &DataRequest,
        receiver: &mut dyn SerdeDeDataReceiver,
    ) -> Result<DataResponseMetadata, DataError> {
        let (rc_buffer, path_buf) = self.get_rc_buffer(req)?;
        deserializer::deserialize_into_receiver(rc_buffer, &self.manifest.syntax, receiver)
            .map_err(|err| err.into_resource_error(&path_buf))?;
        Ok(DataResponseMetadata {
            data_langid: req.resource_path.options.langid.clone(),
        })
    }
}
