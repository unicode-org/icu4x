// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::manifest::Manifest;
use icu_provider::prelude::*;
use icu_provider::DynamicDryDataProvider;
use std::fmt::Debug;
use std::fmt::Write;
use std::fs;
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
///     FsDataProvider::try_new("tests/data/json".into()).expect("Directory exists");
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
    /// let provider = FsDataProvider::try_new("/path/to/data/directory".into())
    ///     .expect_err("Specify a real directory in the line above");
    /// ```
    pub fn try_new(root: PathBuf) -> Result<Self, DataError> {
        Ok(Self {
            manifest: Manifest::parse(&root)?,
            root,
        })
    }

    fn dry_load_internal(
        &self,
        marker: DataMarkerInfo,
        req: DataRequest,
    ) -> Result<(DataResponseMetadata, PathBuf), DataError> {
        if marker.is_singleton && !req.id.locale.is_und() {
            return Err(DataErrorKind::InvalidRequest.with_req(marker, req));
        }
        let mut path = self.root.join(marker.path.as_str());
        if !path.exists() {
            return Err(DataErrorKind::MarkerNotFound.with_req(marker, req));
        }
        if !req.id.marker_attributes.is_empty() {
            if req.metadata.attributes_prefix_match {
                path.push(
                    std::fs::read_dir(&path)?
                        .filter_map(|e| e.ok()?.file_name().into_string().ok())
                        .filter(|c| c.starts_with(req.id.marker_attributes.as_str()))
                        .min()
                        .ok_or(DataErrorKind::IdentifierNotFound.with_req(marker, req))?,
                );
            } else {
                path.push(req.id.marker_attributes.as_str());
            }
        }
        if !marker.is_singleton {
            let mut string_path = path.into_os_string();
            write!(&mut string_path, "/{}", req.id.locale).expect("infallible");
            path = PathBuf::from(string_path);
        }
        path.set_extension(self.manifest.file_extension);
        if !path.exists() {
            return Err(DataErrorKind::IdentifierNotFound.with_req(marker, req));
        }
        let mut metadata = DataResponseMetadata::default();
        metadata.buffer_format = Some(self.manifest.buffer_format);
        Ok((metadata, path))
    }
}

impl DynamicDataProvider<BufferMarker> for FsDataProvider {
    fn load_data(
        &self,
        marker: DataMarkerInfo,
        req: DataRequest,
    ) -> Result<DataResponse<BufferMarker>, DataError> {
        let (metadata, path) = self.dry_load_internal(marker, req)?;
        let buffer = fs::read(&path).map_err(|e| DataError::from(e).with_path_context(&path))?;
        Ok(DataResponse {
            metadata,
            payload: DataPayload::from_owned_buffer(buffer.into_boxed_slice()),
        })
    }
}

impl DynamicDryDataProvider<BufferMarker> for FsDataProvider {
    fn dry_load_data(
        &self,
        marker: DataMarkerInfo,
        req: DataRequest,
    ) -> Result<DataResponseMetadata, DataError> {
        Ok(self.dry_load_internal(marker, req)?.0)
    }
}
