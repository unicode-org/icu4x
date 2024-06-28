// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::manifest::Manifest;
use icu_provider::prelude::*;
use std::fmt::Debug;
use std::fmt::Write;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

/// A data provider that reads ICU4X data from a filesystem directory.
///
/// [`FsDataProvider`] implements [`BufferProvider`], so it can be used in
/// `*_with_buffer_provider` constructors across ICU4X.
///
/// # Examples
///
/// ```
/// use icu_locale_core::locale;
/// use icu_provider::hello_world::HelloWorldFormatter;
/// use icu_provider_fs::FsDataProvider;
/// use writeable::assert_writeable_eq;
///
/// // Create a DataProvider from data files stored in a filesystem directory:
/// let provider =
///     FsDataProvider::try_new("tests/data/json").expect("Directory exists");
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

impl DynamicDataProvider<BufferMarker> for FsDataProvider {
    fn load_data(
        &self,
        marker: DataMarkerInfo,
        req: DataRequest,
    ) -> Result<DataResponse<BufferMarker>, DataError> {
        if marker.is_singleton && !req.id.locale.is_empty() {
            return Err(DataErrorKind::InvalidRequest.with_req(marker, req));
        }
        let mut path = self.root.clone().into_os_string();
        write!(&mut path, "/{}", marker.path.as_str()).expect("infallible");
        if !Path::new(&path).exists() {
            return Err(DataErrorKind::MarkerNotFound.with_req(marker, req));
        }
        if !req.id.marker_attributes.is_empty() {
            write!(&mut path, "/{}", req.id.marker_attributes as &str).expect("infallible");
        }
        write!(&mut path, "/{}", req.id.locale).expect("infallible");
        write!(&mut path, ".{}", self.manifest.file_extension).expect("infallible");
        if !Path::new(&path).exists() {
            return Err(DataErrorKind::IdentifierNotFound.with_req(marker, req));
        }
        let buffer = fs::read(&path).map_err(|e| DataError::from(e).with_path_context(&path))?;
        let mut metadata = DataResponseMetadata::default();
        metadata.buffer_format = Some(self.manifest.buffer_format);
        Ok(DataResponse {
            metadata,
            payload: DataPayload::from_owned_buffer(buffer.into_boxed_slice()),
        })
    }
}
