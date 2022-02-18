// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::serializers::{json, AbstractSerializer};
use crate::error::Error;
use crate::manifest::Manifest;
use crate::manifest::MANIFEST_FILE;
use icu_provider::export::DataExporter;
use icu_provider::prelude::*;
use icu_provider::serde::SerializeMarker;
use serde::{Deserialize, Serialize};
use std::fs;
use std::ops::Deref;
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
    ) -> Result<Self, Error> {
        let result = FilesystemExporter {
            root: options.root,
            manifest: Manifest {
                buffer_format: serializer.get_buffer_format(),
            },
            serializer,
        };

        match options.overwrite {
            OverwriteOption::CheckEmpty => {
                if result.root.exists() {
                    fs::remove_dir(&result.root).map_err(|e| (e, &result.root))?;
                }
            }
            OverwriteOption::RemoveAndReplace => {
                if result.root.exists() {
                    fs::remove_dir_all(&result.root).map_err(|e| (e, &result.root))?;
                }
            }
        };
        fs::create_dir_all(&result.root).map_err(|e| (e, &result.root))?;

        let manifest_path = result.root.join(MANIFEST_FILE);
        let mut manifest_file =
            fs::File::create(&manifest_path).map_err(|e| (e, &manifest_path))?;
        let manifest_serializer = json::Serializer::new(json::Options {
            style: json::StyleOption::Pretty,
        });
        manifest_serializer
            .serialize(&result.manifest, &mut manifest_file)
            .map_err(|e| (e, manifest_path))?;
        Ok(result)
    }
}

impl DataExporter<SerializeMarker> for FilesystemExporter {
    fn put_payload(
        &self,
        key: ResourceKey,
        options: ResourceOptions,
        obj: DataPayload<SerializeMarker>,
    ) -> Result<(), DataError> {
        log::trace!("Writing: {}/{}", key, options);

        let mut path_buf = self.root.clone();
        path_buf.push(&*key.writeable_to_string());
        path_buf.push(&*options.writeable_to_string());
        path_buf.set_extension(self.manifest.get_file_extension());

        if let Some(parent_dir) = path_buf.parent() {
            fs::create_dir_all(&parent_dir).map_err(|e| Error::from((e, parent_dir)))?;
        }
        let mut file = fs::File::create(&path_buf).map_err(|e| Error::from((e, &path_buf)))?;
        self.serializer
            .serialize(obj.get().deref(), &mut file)
            .map_err(|e| Error::from((e, &path_buf)))?;
        Ok(())
    }
}
