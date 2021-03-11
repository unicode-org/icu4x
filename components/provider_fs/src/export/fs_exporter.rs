// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).
use super::aliasing::{self, AliasCollection};
use super::serializers::AbstractSerializer;
use crate::error::Error;
use crate::manifest::AliasOption;
use crate::manifest::LocalesOption;
use crate::manifest::Manifest;
use crate::manifest::MANIFEST_FILE;
use icu_provider::erased::ErasedDataStruct;
use icu_provider::iter::DataExporter;
use icu_provider::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

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

/// Options bag for initializing a FilesystemExporter.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExporterOptions {
    /// Directory in the filesystem to write output.
    pub root: PathBuf,
    /// Strategy for including locales.
    pub locales: LocalesOption,
    /// Strategy for de-duplicating locale data.
    pub aliasing: AliasOption,
    /// Option for initializing the output directory.
    pub overwrite: OverwriteOption,
}

impl Default for ExporterOptions {
    fn default() -> Self {
        Self {
            root: PathBuf::from("icu4x_data"),
            locales: LocalesOption::IncludeAll,
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
    alias_collection: Option<AliasCollection<Vec<u8>>>,
    serializer: Box<dyn AbstractSerializer>,
}

impl Drop for FilesystemExporter {
    fn drop(&mut self) {
        if self.alias_collection.is_some() {
            panic!("Please call flush before dropping FilesystemExporter");
        }
    }
}

impl DataExporter for FilesystemExporter {
    fn put_payload(
        &mut self,
        req: &DataRequest,
        obj: &dyn ErasedDataStruct,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut path_buf = self.root.clone();
        path_buf.extend(req.resource_path.key.get_components().iter());
        path_buf.extend(req.resource_path.options.get_components().iter());
        log::trace!("Writing: {}", req);
        self.write_to_path(path_buf, obj.as_serialize())?;
        Ok(())
    }

    fn include_resource_options(&self, resc_options: &ResourceOptions) -> bool {
        match self.manifest.locales {
            LocalesOption::IncludeAll => true,
            LocalesOption::IncludeList(ref list) => match &resc_options.langid {
                Some(langid) => list.contains(&langid),
                None => true,
            },
        }
    }
}

impl FilesystemExporter {
    pub fn try_new(
        serializer: Box<dyn AbstractSerializer>,
        options: ExporterOptions,
    ) -> Result<Self, Error> {
        let result = FilesystemExporter {
            root: options.root,
            manifest: Manifest {
                aliasing: options.aliasing,
                locales: options.locales,
                syntax: serializer.deref().clone(),
            },
            alias_collection: None,
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

        let mut manifest_path = result.root.to_path_buf();
        manifest_path.push(MANIFEST_FILE);
        let mut manifest_file =
            fs::File::create(&manifest_path).map_err(|e| (e, &manifest_path))?;
        let mut manifest_writer = serde_json::Serializer::pretty(&mut manifest_file);
        result
            .manifest
            .serialize(&mut manifest_writer)
            .map_err(|e| (e, &manifest_path))?;
        writeln!(&mut manifest_file).map_err(|e| (e, &manifest_path))?;
        Ok(result)
    }

    /// This function must be called before the FilesystemExporter leaves scope.
    /// It is recommended to flush after exporting each ResourceKey.
    pub fn flush(&mut self) -> Result<(), Error> {
        if let Some(mut alias_collection) = self.alias_collection.take() {
            alias_collection.flush()?;
        }
        Ok(())
    }

    fn write_to_path(
        &mut self,
        mut path_buf: PathBuf,
        obj: &dyn erased_serde::Serialize,
    ) -> Result<(), Error> {
        let file_extension = self.serializer.get_file_extension();
        match self.manifest.aliasing {
            AliasOption::NoAliases => {
                path_buf.set_extension(file_extension);
                if let Some(parent_dir) = path_buf.parent() {
                    fs::create_dir_all(&parent_dir).map_err(|e| (e, parent_dir))?;
                }
                let mut file = fs::File::create(&path_buf).map_err(|e| (e, &path_buf))?;
                self.serializer
                    .serialize(obj, &mut file)
                    .map_err(|e| (e, &path_buf))?;
            }
            AliasOption::Symlink => {
                let mut buf: Vec<u8> = Vec::new();
                self.serializer
                    .serialize(obj, &mut buf)
                    .map_err(Error::from_serializers_error)?;
                let mut alias_root = path_buf.clone();
                assert!(alias_root.pop());
                self.alias_collection
                    .get_or_insert_with(|| {
                        AliasCollection::new(aliasing::Options {
                            root: alias_root,
                            symlink_file_extension: "l",
                            data_file_prefix: "data",
                            data_file_extension: file_extension,
                        })
                    })
                    .put(path_buf, buf);
            }
        }

        Ok(())
    }
}
