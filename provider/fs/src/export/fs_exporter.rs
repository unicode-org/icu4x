// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::aliasing::{self, AliasCollection};
use super::serializers::{json, AbstractSerializer};
use crate::error::Error;
use crate::manifest::AliasOption;
use crate::manifest::Manifest;
use crate::manifest::MANIFEST_FILE;
use icu_provider::export::DataExporter;
use icu_provider::prelude::*;
use icu_provider::serde::SerializeMarker;
use serde::{Deserialize, Serialize};
use std::collections::btree_map::BTreeMap;
use std::fs;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Mutex;
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
    /// Strategy for de-duplicating locale data.
    pub aliasing: AliasOption,
    /// Option for initializing the output directory.
    pub overwrite: OverwriteOption,
}

impl Default for ExporterOptions {
    fn default() -> Self {
        Self {
            root: PathBuf::from("icu4x_data"),
            aliasing: AliasOption::NoAliases,
            overwrite: OverwriteOption::CheckEmpty,
        }
    }
}

/// A data exporter that writes data to a filesystem hierarchy.
/// See the module-level docs for an example.
pub struct FilesystemExporter {
    root: PathBuf,
    manifest: Manifest,
    // The presence of the option does not change for the lifetime of the object and
    // therefore doesn't need to be locked. This way a non-aliasing instance is lock-free.
    alias_collections: Option<Mutex<BTreeMap<ResourceKeyHash, AliasCollection<Vec<u8>>>>>,
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
                aliasing: options.aliasing,
                buffer_format: serializer.get_buffer_format(),
            },
            alias_collections: match options.aliasing {
                AliasOption::NoAliases => None,
                AliasOption::Symlink => Some(Mutex::new(BTreeMap::new())),
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
        let mut key_root = self.root.clone();
        key_root.push(&*key.writeable_to_string());
        let mut path_buf = key_root.clone();
        path_buf.push(&*options.writeable_to_string());

        log::trace!("Writing: {}/{}", key, options);

        match self.alias_collections.as_ref() {
            None => {
                path_buf.set_extension(self.manifest.get_file_extension());
                if let Some(parent_dir) = path_buf.parent() {
                    fs::create_dir_all(&parent_dir).map_err(|e| Error::from((e, parent_dir)))?;
                }
                let mut file =
                    fs::File::create(&path_buf).map_err(|e| Error::from((e, &path_buf)))?;
                self.serializer
                    .serialize(obj.get().deref(), &mut file)
                    .map_err(|e| Error::from((e, &path_buf)))?;
            }
            Some(alias_collections) => {
                let mut buf: Vec<u8> = Vec::new();
                self.serializer
                    .serialize(obj.get().deref(), &mut buf)
                    .map_err(Error::from_serializers_error)?;
                alias_collections
                    .lock()
                    .unwrap()
                    .entry(key.get_hash())
                    .or_insert_with(|| {
                        AliasCollection::new(aliasing::Options {
                            root: key_root,
                            symlink_file_extension: "l",
                            data_file_prefix: "data",
                            data_file_extension: self.manifest.get_file_extension(),
                        })
                    })
                    .put(path_buf, buf);
            }
        }
        Ok(())
    }

    fn flush(&self, key: ResourceKey) -> Result<(), DataError> {
        // If we're aliasing we have to flush the alias collection for this key
        if let Some(alias_collections) = self.alias_collections.as_ref() {
            if let Some(alias_collection) =
                alias_collections.lock().unwrap().get_mut(&key.get_hash())
            {
                alias_collection.flush()?;
            }
        }
        Ok(())
    }

    fn close(&mut self) -> Result<(), DataError> {
        // If we're aliasing we have to make sure all alias collections were flushed
        if let Some(mut alias_collections) = self.alias_collections.take() {
            for (_, alias_collection) in alias_collections.get_mut().unwrap().iter_mut() {
                // Flushing is idempotent
                alias_collection.flush()?;
            }
        }
        Ok(())
    }
}
