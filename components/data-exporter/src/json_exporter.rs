use crate::aliasing::{self, AliasCollection};
use crate::Error;
use crate::FileWriter;
use serde::{Deserialize, Serialize};

use std::fs;
use std::path::Path;
use std::path::PathBuf;

#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AliasOption {
    NoAliases,
    Symlink,
}

#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OverwriteOption {
    RemoveAndReplace,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Manifest {
    pub aliasing: AliasOption,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Options {
    pub root: PathBuf,
    pub aliasing: AliasOption,
    pub overwrite: OverwriteOption,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            root: PathBuf::from("icu4x_data"),
            aliasing: AliasOption::NoAliases,
            overwrite: OverwriteOption::RemoveAndReplace,
        }
    }
}

pub struct JsonFileWriter {
    root: PathBuf,
    manifest: Manifest,
    alias_collection: Option<AliasCollection<Vec<u8>>>,
}

impl Drop for JsonFileWriter {
    fn drop(&mut self) {
        if self.alias_collection.is_some() {
            panic!("Please call flush before dropping JsonFileWriter");
        }
    }
}

impl JsonFileWriter {
    pub fn try_new(options: &Options) -> Result<Self, Error> {
        let result = JsonFileWriter {
            root: options.root.to_path_buf(),
            manifest: Manifest {
                aliasing: options.aliasing,
            },
            alias_collection: None,
        };

        match options.overwrite {
            OverwriteOption::RemoveAndReplace => {
                fs::remove_dir_all(&options.root)?;
            }
        };
        fs::create_dir_all(&options.root)?;

        let mut manifest_path = result.root.to_path_buf();
        manifest_path.push("manifest");
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
}

impl FileWriter for JsonFileWriter {
    fn write_to_path(
        &mut self,
        path_without_extension: &Path,
        obj: &dyn erased_serde::Serialize,
    ) -> Result<(), Error> {
        let mut path_buf: std::path::PathBuf = self.root.clone();
        path_buf.extend(path_without_extension);
        path_buf.set_extension("json");

        if let Some(parent_dir) = path_buf.parent() {
            fs::create_dir_all(&parent_dir)?;
        }

        match self.manifest.aliasing {
            AliasOption::NoAliases => {
                let file = fs::File::create(&path_buf)?;
                let mut json = serde_json::Serializer::new(file);
                obj.erased_serialize(&mut erased_serde::Serializer::erase(&mut json))?;
            }
            AliasOption::Symlink => {
                let mut buf: Vec<u8> = Vec::new();
                let mut json = serde_json::Serializer::new(&mut buf);
                obj.erased_serialize(&mut erased_serde::Serializer::erase(&mut json))?;
                let mut alias_root = path_buf.clone();
                assert!(alias_root.pop());
                self.alias_collection
                    .get_or_insert_with(|| {
                        AliasCollection::new(aliasing::Options {
                            root: alias_root,
                            symlink_file_extension: "l",
                            data_file_extension: "json",
                        })
                    })
                    .put(path_buf, buf);
            }
        }

        Ok(())
    }
}
