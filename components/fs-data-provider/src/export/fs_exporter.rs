use super::aliasing::{self, AliasCollection};
use super::serializers::Serializer;
use super::Error;
use icu_data_provider::iter::DataExporter;
use icu_data_provider::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AliasOption {
    /// Do not de-duplicate data.
    NoAliases,
    /// De-duplicate data by using filesystem symlinks.
    Symlink,
    // TODO: Alias based on a field in the JSON file
}

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

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Manifest {
    pub aliasing: AliasOption,
}

/// Options bag for initializing a FilesystemExporter.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Options {
    /// Directory in the filesystem to write output.
    pub root: PathBuf,
    /// Strategy for de-duplicating locale data.
    pub aliasing: AliasOption,
    /// Option for initializing the output directory.
    pub overwrite: OverwriteOption,
    /// Whether to print progress to stdout.
    pub verbose: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            root: PathBuf::from("icu4x_data"),
            aliasing: AliasOption::NoAliases,
            overwrite: OverwriteOption::CheckEmpty,
            verbose: false,
        }
    }
}

/// A data exporter that writes data to a filesystem hierarchy.
pub struct FilesystemExporter {
    root: PathBuf,
    manifest: Manifest,
    alias_collection: Option<AliasCollection<Vec<u8>>>,
    verbose: bool,
    serializer: Box<dyn Serializer>,
}

impl Drop for FilesystemExporter {
    fn drop(&mut self) {
        if self.alias_collection.is_some() {
            panic!("Please call flush before dropping FilesystemExporter");
        }
    }
}

impl DataExporter for FilesystemExporter {
    fn put(
        &mut self,
        req: &data_provider::Request,
        obj: &dyn erased_serde::Serialize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut path_buf = self.root.clone();
        path_buf.extend(req.data_key.get_components().iter());
        path_buf.extend(req.data_entry.get_components().iter());
        if self.verbose {
            println!("Initializing: {}", path_buf.to_string_lossy());
        }
        self.write_to_path(path_buf, obj)
    }
}

impl FilesystemExporter {
    pub fn try_new(serializer: Box<dyn Serializer>, options: &Options) -> Result<Self, Error> {
        let result = FilesystemExporter {
            root: options.root.to_path_buf(),
            manifest: Manifest {
                aliasing: options.aliasing,
            },
            alias_collection: None,
            verbose: options.verbose,
            serializer,
        };

        match options.overwrite {
            OverwriteOption::CheckEmpty => {
                if options.root.exists() {
                    fs::remove_dir(&options.root)?;
                }
            }
            OverwriteOption::RemoveAndReplace => {
                if options.root.exists() {
                    fs::remove_dir_all(&options.root)?;
                }
            }
        };
        fs::create_dir_all(&options.root)?;

        let mut manifest_path = result.root.to_path_buf();
        manifest_path.push("manifest");
        // NOTE: Always use JSON for the manifest, even if the serializer isn't JSON.
        // This way, we will always be able to start by reading manifest.json.
        manifest_path.set_extension("json");
        let manifest_file = fs::File::create(manifest_path)?;
        let mut manifest_writer = serde_json::Serializer::pretty(manifest_file);
        result.manifest.serialize(&mut manifest_writer)?;
        Ok(result)
    }

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
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file_extension = self.serializer.get_file_extension();
        match self.manifest.aliasing {
            AliasOption::NoAliases => {
                path_buf.set_extension(file_extension);
                if let Some(parent_dir) = path_buf.parent() {
                    fs::create_dir_all(&parent_dir)?;
                }
                let mut file = fs::File::create(&path_buf)?;
                self.serializer.serialize(obj, &mut file)?;
            }
            AliasOption::Symlink => {
                let mut buf: Vec<u8> = Vec::new();
                self.serializer.serialize(obj, &mut buf)?;
                let mut alias_root = path_buf.clone();
                assert!(alias_root.pop());
                self.alias_collection
                    .get_or_insert_with(|| {
                        AliasCollection::new(aliasing::Options {
                            root: alias_root,
                            symlink_file_extension: "l",
                            data_file_extension: file_extension,
                        })
                    })
                    .put(path_buf, buf);
            }
        }

        Ok(())
    }
}
