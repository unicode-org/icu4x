// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::manifest::Manifest;
use icu_provider::prelude::*;
use std::fmt::Debug;
use std::fs;
use std::path::PathBuf;
use writeable::Writeable;

/// A data provider that reads ICU4X data from a filesystem directory.
///
/// [`FsDataProvider`] implements [`BufferProvider`], so it can be used in
/// `*_with_buffer_provider` constructors across ICU4X.
///
/// # Examples
///
/// ```
/// use icu_locid::locale;
/// use icu_provider::hello_world::HelloWorldFormatter;
/// use icu_provider_fs::FsDataProvider;
/// use writeable::assert_writeable_eq;
///
/// // Create a DataProvider from data files stored in a filesystem directory:
/// let provider = FsDataProvider::try_new(concat!(
///     env!("CARGO_MANIFEST_DIR"),
///     "/tests/data/json",
/// ))
/// .expect("Directory exists");
///
/// // Check that it works:
/// let formatter = HelloWorldFormatter::try_new_with_buffer_provider(
///     &provider,
///     &locale!("la").into(),
/// )
/// .expect("locale exists");
///
/// assert_writeable_eq!(formatter.format(), "Ave, munde");
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct FsDataProvider {
    root: PathBuf,
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
    pub fn try_new<T: Into<PathBuf>>(root: T) -> Result<Self, DataError> {
        let root = root.into();
        Ok(Self {
            manifest: Manifest::parse(&root)?,
            root,
        })
    }
}

impl BufferProvider for FsDataProvider {
    fn load_buffer(
        &self,
        key: DataKey,
        req: DataRequest,
    ) -> Result<DataResponse<BufferMarker>, DataError> {
        let mut path_buf = self.root.join(&*key.write_to_string());
        if !path_buf.exists() {
            return Err(DataErrorKind::MissingDataKey.with_req(key, req));
        }
        path_buf.push(&*req.locale.write_to_string());
        path_buf.set_extension(self.manifest.file_extension);
        if !path_buf.exists() {
            return Err(DataErrorKind::MissingLocale.with_req(key, req));
        }
        let buffer =
            fs::read(&path_buf).map_err(|e| DataError::from(e).with_path_context(&path_buf))?;
        let mut metadata = DataResponseMetadata::default();
        metadata.buffer_format = Some(self.manifest.buffer_format);
        Ok(DataResponse {
            metadata,
            payload: Some(DataPayload::from_owned_buffer(buffer.into_boxed_slice())),
        })
    }
}
