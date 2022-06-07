// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::serializers::AbstractSerializer;
use crate::manifest::Manifest;
use icu_provider::datagen::*;
use icu_provider::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use writeable::Writeable;

#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OverwriteOption {
    /// If the directory doesn't exist, create it.
    /// If it does exist, remove it safely (rmdir) and re-create it.
    CheckEmpty,
    /// If the directory doesn't exist, create it.
    /// If it does exist, remove it aggressively (rm -rf) and re-create it.
    RemoveAndReplace,
}

/// Options bag for initializing a [`FilesystemExporter`].
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExporterOptions {
    /// Directory in the filesystem to write output.
    pub root: PathBuf,
    /// Option for initializing the output directory.
    pub overwrite: OverwriteOption,
}

impl Default for ExporterOptions {
    fn default() -> Self {
        Self {
            root: PathBuf::from("icu4x_data"),
            overwrite: OverwriteOption::CheckEmpty,
        }
    }
}

/// A data exporter that writes data to a filesystem hierarchy.
/// See the module-level docs for an example.
pub struct FilesystemExporter {
    root: PathBuf,
    manifest: Manifest,
    serializer: Box<dyn AbstractSerializer + Sync>,
}

impl FilesystemExporter {
    pub fn try_new(
        serializer: Box<dyn AbstractSerializer + Sync>,
        options: ExporterOptions,
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
}

impl DataExporter for FilesystemExporter {
    fn put_payload(
        &self,
        key: ResourceKey,
        options: &ResourceOptions,
        obj: &DataPayload<ExportMarker>,
    ) -> Result<(), DataError> {
        log::trace!("Writing: {}/{}", key, options);

        let mut path_buf = self.root.clone();
        path_buf.push(&*key.write_to_string());
        path_buf.push(&*options.write_to_string());
        path_buf.set_extension(self.manifest.file_extension);

        if let Some(parent_dir) = path_buf.parent() {
            fs::create_dir_all(&parent_dir)
                .map_err(|e| DataError::from(e).with_path_context(&parent_dir))?;
        }
        let mut file = fs::File::create(&path_buf)
            .map_err(|e| DataError::from(e).with_path_context(&path_buf))?;
        self.serializer
            .serialize(obj, &mut file)
            .map_err(|e| e.with_path_context(&path_buf))?;
        Ok(())
    }
}
