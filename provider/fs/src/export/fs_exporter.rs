// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::serializers::AbstractSerializer;
use crate::datapath::marker_to_path;
use crate::manifest::Manifest;
use icu_provider::export::*;
use icu_provider::prelude::*;
use std::fmt::Write;
use std::fs;
use std::io::Write as _;
use std::path::PathBuf;

/// Choices of what to do if [`FilesystemExporter`] tries to write to a pre-existing directory.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum OverwriteOption {
    /// If the directory doesn't exist, create it.
    /// If it does exist, remove it safely (`rmdir`) and re-create it.
    CheckEmpty,
    /// If the directory doesn't exist, create it.
    /// If it does exist, remove it aggressively (`rm -rf`) and re-create it.
    RemoveAndReplace,
}

impl Default for OverwriteOption {
    fn default() -> Self {
        Self::CheckEmpty
    }
}

/// Options bag for initializing a [`FilesystemExporter`].
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Options {
    /// Directory in the filesystem to write output.
    pub root: PathBuf,
    /// Option for initializing the output directory.
    pub overwrite: OverwriteOption,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            root: PathBuf::from("icu4x_data"),
            overwrite: Default::default(),
        }
    }
}

impl From<PathBuf> for Options {
    fn from(root: PathBuf) -> Self {
        Options {
            root,
            ..Default::default()
        }
    }
}

/// A data exporter that writes data to a filesystem hierarchy.
/// See the module-level docs for an example.
#[derive(Debug)]
pub struct FilesystemExporter {
    root: PathBuf,
    manifest: Manifest,
    serializer: Box<dyn AbstractSerializer + Sync>,
}

impl FilesystemExporter {
    /// Creates a new [`FilesystemExporter`] with a [serializer] and [options].
    ///
    /// See the module-level docs for an example.
    ///
    /// [serializer]: crate::export::serializers
    /// [options]: Options
    pub fn try_new(
        serializer: Box<dyn AbstractSerializer + Sync>,
        options: Options,
    ) -> Result<Self, DataError> {
        let result = FilesystemExporter {
            root: options.root,
            manifest: Manifest::for_format(serializer.get_buffer_format())?,
            serializer,
        };

        match options.overwrite {
            OverwriteOption::CheckEmpty if result.root.exists() => fs::remove_dir(&result.root),
            OverwriteOption::RemoveAndReplace if result.root.exists() => {
                fs::remove_dir_all(&result.root)
            }
            _ => Ok(()),
        }
        .and_then(|_| fs::create_dir_all(&result.root))
        .map_err(|e| DataError::from(e).with_path_context(&result.root))?;

        result.manifest.write(&result.root)?;
        Ok(result)
    }

    fn setup_file(&self, mut path_buf: PathBuf) -> Result<Box<dyn std::io::Write>, DataError> {
        path_buf.set_extension(self.manifest.file_extension);
        let file: Box<dyn std::io::Write> = if self.serializer.is_text_format() {
            Box::new(crlify::BufWriterWithLineEndingFix::new(
                fs::File::create(&path_buf)
                    .map_err(|e| DataError::from(e).with_path_context(&path_buf))?,
            ))
        } else {
            Box::new(std::io::BufWriter::new(
                fs::File::create(&path_buf)
                    .map_err(|e| DataError::from(e).with_path_context(&path_buf))?,
            ))
        };
        Ok(file)
    }
}

impl DataExporter for FilesystemExporter {
    fn put_payload(
        &self,
        marker: DataMarkerInfo,
        id: DataIdentifierBorrowed,
        payload: &DataPayload<ExportMarker>,
    ) -> Result<(), DataError> {
        let mut path_buf = marker_to_path(marker.id, &self.root);
        if !id.marker_attributes.is_empty() {
            path_buf.push(id.marker_attributes.as_str());
        }

        #[expect(clippy::unwrap_used)] // has parent by construction
        let parent_dir = path_buf.parent().unwrap();

        fs::create_dir_all(parent_dir)
            .map_err(|e| DataError::from(e).with_path_context(parent_dir))?;

        fs::create_dir_all(&path_buf)
            .map_err(|e| DataError::from(e).with_path_context(&path_buf))?;
        let mut string_path = path_buf.into_os_string();
        write!(&mut string_path, "/{}", id.locale).expect("infallible");
        path_buf = PathBuf::from(string_path);

        let mut file = self.setup_file(path_buf)?;
        self.serializer.serialize(payload, &mut file)
    }

    fn flush(&self, marker: DataMarkerInfo, metadata: FlushMetadata) -> Result<(), DataError> {
        let path_buf = marker_to_path(marker.id, &self.root);

        if !path_buf.exists() {
            fs::create_dir_all(&path_buf)
                .map_err(|e| DataError::from(e).with_path_context(&path_buf))?;
            fs::File::create(path_buf.join(".empty"))?;
        } else if let Some(checksum) = metadata.checksum {
            write!(
                &mut fs::File::create(path_buf.join(".checksum"))?,
                "{checksum}"
            )?;
        }

        Ok(())
    }

    fn flush_singleton(
        &self,
        marker: DataMarkerInfo,
        payload: &DataPayload<ExportMarker>,
        metadata: FlushMetadata,
    ) -> Result<(), DataError> {
        let path_buf = marker_to_path(marker.id, &self.root);

        #[expect(clippy::unwrap_used)] // has parent by construction
        let parent_dir = path_buf.parent().unwrap();

        fs::create_dir_all(parent_dir)
            .map_err(|e| DataError::from(e).with_path_context(parent_dir))?;

        if let Some(checksum) = metadata.checksum {
            write!(
                &mut fs::File::create(format!("{}_checksum", path_buf.display()))?,
                "{checksum}"
            )
            .unwrap();
        }
        let mut file = self.setup_file(path_buf)?;

        self.serializer.serialize(payload, &mut file)
    }
}
