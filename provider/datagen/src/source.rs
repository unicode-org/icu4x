// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use elsa::sync::FrozenMap;
use icu_provider::prelude::*;
use std::any::Any;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::fmt::Debug;
#[cfg(feature = "networking")]
use std::fs::File;
#[cfg(feature = "networking")]
use std::io::BufWriter;
use std::io::Cursor;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use zip::ZipArchive;

pub(crate) struct SerdeCache {
    pub(crate) root: AbstractFs,
    cache: FrozenMap<String, Box<dyn Any + Send + Sync>>,
    list_cache: RwLock<Option<HashSet<String>>>,
}

impl Debug for SerdeCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SerdeCache")
            .field("root", &self.root)
            // skip formatting the cache
            .finish()
    }
}

impl SerdeCache {
    pub fn new(root: AbstractFs) -> Self {
        Self {
            root,
            cache: FrozenMap::new(),
            list_cache: Default::default(),
        }
    }

    fn read_and_parse<S>(
        &self,
        path: &str,
        parser: fn(&[u8]) -> Result<S, DataError>,
    ) -> Result<&S, DataError>
    where
        for<'de> S: serde::Deserialize<'de> + 'static + Send + Sync,
    {
        match self.cache.get(path) {
            Some(x) => x,
            None => self.cache.insert(
                path.to_string(),
                Box::new(
                    parser(&self.root.read_to_buf(path)?)
                        .map_err(|e| e.with_path_context(&path))?,
                ),
            ),
        }
        .downcast_ref::<S>()
        .ok_or_else(|| DataError::custom("Cache error").with_type_context::<S>())
    }

    pub fn read_and_parse_json<S>(&self, path: &str) -> Result<&S, DataError>
    where
        for<'de> S: serde::Deserialize<'de> + 'static + Send + Sync,
    {
        self.read_and_parse(path, |bytes| {
            serde_json::from_slice(bytes)
                .map_err(|e| DataError::custom("JSON deserialize").with_display_context(&e))
        })
    }

    pub fn read_and_parse_toml<S>(&self, path: &str) -> Result<&S, DataError>
    where
        for<'de> S: serde::Deserialize<'de> + 'static + Send + Sync,
    {
        self.read_and_parse(path, |bytes| {
            toml::from_slice(bytes)
                .map_err(|e| DataError::custom("TOML deserialize").with_display_context(&e))
        })
    }

    pub fn list(&self, path: &str) -> Result<impl Iterator<Item = String>, DataError> {
        let hash_set = self
            .list_cache
            .read()
            .expect("poison")
            .as_ref()
            .unwrap()
            .iter()
            .filter_map(|p| p.strip_prefix(path))
            .filter_map(|suffix| suffix.split('/').find(|s| !s.is_empty()))
            .map(String::from)
            .collect::<HashSet<_>>();
        Ok(hash_set.into_iter())
    }

    pub fn file_exists(&self, path: &str) -> Result<bool, DataError> {
        self.populate_list_cache();
        let file_exists = self
            .list_cache
            .read()
            .expect("poison")
            .as_ref()
            .unwrap()
            .contains(path);
        Ok(file_exists)
    }

    fn populate_list_cache(&self) -> Result<(), DataError> {
        let mut list_cache = self.list_cache.write().expect("poison");
        if list_cache.is_none() {
            let list = self.root.list_all()?;
            list_cache.replace(list);
        }
        Ok(())
    }
}

pub(crate) struct ZipData {
    archive: ZipArchive<Cursor<Vec<u8>>>,
    file_list: HashSet<String>,
}

pub(crate) enum AbstractFs {
    Fs(PathBuf),
    Zip(RwLock<Result<ZipData, String>>),
    Memory(BTreeMap<&'static str, &'static [u8]>),
}

impl Debug for AbstractFs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AbstractFs").finish()
    }
}

impl AbstractFs {
    pub fn new<P: AsRef<Path>>(root: P) -> Result<Self, DataError> {
        if std::fs::metadata(root.as_ref())
            .map_err(|e| DataError::from(e).with_path_context(root.as_ref()))?
            .is_dir()
        {
            Ok(Self::Fs(root.as_ref().to_path_buf()))
        } else {
            let archive = ZipArchive::new(Cursor::new(std::fs::read(&root)?)).map_err(|e| {
                DataError::custom("Invalid ZIP file")
                    .with_display_context(&e)
                    .with_path_context(&root)
            })?;
            let file_list = archive.file_names().map(String::from).collect();
            Ok(Self::Zip(RwLock::new(Ok(ZipData { archive, file_list }))))
        }
    }

    #[cfg(feature = "networking")]
    pub fn new_from_url(path: String) -> Self {
        Self::Zip(RwLock::new(Err(path)))
    }

    fn init(&self) -> Result<(), DataError> {
        #[cfg(feature = "networking")]
        if let Self::Zip(lock) = self {
            if lock.read().expect("poison").is_ok() {
                return Ok(());
            }
            let mut lock = lock.write().expect("poison");
            let resource = if let Err(resource) = &*lock {
                resource
            } else {
                return Ok(());
            };

            let root = std::env::var_os("ICU4X_SOURCE_CACHE")
                .map(PathBuf::from)
                .unwrap_or_else(|| std::env::temp_dir().join("icu4x-source-cache/"))
                .join(resource.rsplit("//").next().unwrap());

            if !root.exists() {
                log::info!("Downloading {resource}");
                std::fs::create_dir_all(root.parent().unwrap())?;
                std::io::copy(
                    &mut ureq::get(resource)
                        .call()
                        .map_err(|e| DataError::custom("Download").with_display_context(&e))?
                        .into_reader(),
                    &mut BufWriter::new(File::create(&root)?),
                )?;
            }

            let archive = ZipArchive::new(Cursor::new(std::fs::read(&root)?)).map_err(|e| {
                DataError::custom("Invalid ZIP file")
                    .with_display_context(&e)
                    .with_path_context(&root)
            })?;

            let file_list = archive.file_names().map(String::from).collect();

            *lock = Ok(ZipData { archive, file_list });
        }
        Ok(())
    }

    fn read_to_buf(&self, path: &str) -> Result<Vec<u8>, DataError> {
        self.init()?;
        match self {
            Self::Fs(root) => {
                log::debug!("Reading: {}/{}", root.display(), path);
                std::fs::read(root.join(path))
                    .map_err(|e| DataError::from(e).with_path_context(&root.join(path)))
            }
            Self::Zip(zip) => {
                log::debug!("Reading: <zip>/{}", path);
                let mut buf = Vec::new();
                zip.write()
                    .expect("poison")
                    .as_mut()
                    .ok()
                    .unwrap() // init called
                    .archive
                    .by_name(path)
                    .map_err(|e| {
                        DataErrorKind::Io(std::io::ErrorKind::NotFound)
                            .into_error()
                            .with_display_context(&e)
                            .with_display_context(path)
                    })?
                    .read_to_end(&mut buf)?;
                Ok(buf)
            }
            Self::Memory(map) => map.get(path).copied().map(Vec::from).ok_or_else(|| {
                DataError::custom("Not found in icu4x-datagen's data/").with_display_context(path)
            }),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn read_to_string(&self, path: &str) -> Result<String, DataError> {
        let vec = self.read_to_buf(path)?;
        let s = String::from_utf8(vec)
            .map_err(|e| DataError::custom("Invalid UTF-8").with_display_context(&e))?;
        Ok(s)
    }

    fn list(&self, path: &str) -> Result<impl Iterator<Item = String>, DataError> {
        self.init()?;
        Ok(match self {
            Self::Fs(root) => std::fs::read_dir(root.join(path))
                .map_err(|e| DataError::from(e).with_display_context(path))?
                .map(|e| -> Result<_, DataError> { Ok(e?.file_name().into_string().unwrap()) })
                .collect::<Result<HashSet<_>, DataError>>()
                .map(HashSet::into_iter)?,
            Self::Zip(zip) => zip
                .read()
                .expect("poison")
                .as_ref()
                .ok()
                .unwrap() // init called
                .file_list
                .iter()
                .filter_map(|p| p.strip_prefix(path))
                .filter_map(|suffix| suffix.split('/').find(|s| !s.is_empty()))
                .map(String::from)
                .collect::<HashSet<_>>()
                .into_iter(),
            Self::Memory(map) => map
                .keys()
                .copied()
                .filter_map(|p| p.strip_prefix(path))
                .filter_map(|suffix| suffix.split('/').find(|s| !s.is_empty()))
                .map(String::from)
                .collect::<HashSet<_>>()
                .into_iter(),
        })
    }

    fn file_exists(&self, path: &str) -> Result<bool, DataError> {
        self.init()?;
        Ok(match self {
            Self::Fs(root) => root.join(path).is_file(),
            Self::Zip(zip) => zip
                .read()
                .expect("poison")
                .as_ref()
                .ok()
                .unwrap() // init called
                .file_list
                .contains(path),
            Self::Memory(map) => map.contains_key(path),
        })
    }

    fn list_all(&self) -> Result<HashSet<String>, DataError> {
        self.init()?;
        Ok(match self {
            Self::Fs(root) => fs_readdir_recursive(root)?
                .into_iter()
                .map(|path| {
                    path.strip_prefix(root)
                        .unwrap()
                        .to_string_lossy()
                        .into_owned()
                })
                .collect(),
            Self::Zip(zip) => zip
                .read()
                .expect("poison")
                .as_ref()
                .ok()
                .unwrap()
                .file_list
                .iter()
                .filter(|path| !path.ends_with('/'))
                .map(|s| s.to_string())
                .collect(),
            Self::Memory(map) => map.keys().copied().map(String::from).collect(),
        })
    }
}

fn fs_readdir_recursive(root: &Path) -> Result<HashSet<PathBuf>, DataError> {
    let mut all_files = HashSet::new();
    let mut entries =
        std::fs::read_dir(root).map_err(|e| DataError::from(e).with_debug_context(root))?;
    for entry in entries {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            all_files.extend(fs_readdir_recursive(&entry.path())?);
        } else {
            all_files.insert(entry.path());
        }
    }
    Ok(all_files)
}

#[cfg(test)]
#[cfg(feature = "networking")]
mod tests {
    use super::*;

    #[test]
    fn test_list_all() {
        for abstract_fs in [get_zip_fs(), get_fs_fs(), get_memory_fs()] {
            let list_all = abstract_fs.list_all().unwrap();
            // Convert to a Vec and sort for testing purposes.
            // Also selectively drop files not in the test fs.
            let mut list_all = list_all
                .into_iter()
                .filter(|file_name| {
                    !file_name.contains("variables")
                        && !file_name.contains("weights.npy")
                        && !file_name.contains("saved_model.pb")
                        && !file_name.contains("keras")
                        && !file_name.contains("model5")
                        && !file_name.contains("model7")
                        && !file_name.contains("genvec")
                        && !file_name.contains("Burmese_graphclust")
                })
                .collect::<Vec<_>>();
            list_all.sort();
            list_all.truncate(5);
            assert_eq!(
                &[
                    "Burmese_codepoints_exclusive_model4_heavy/weights.json",
                    "Khmer_codepoints_exclusive_model4_heavy/weights.json",
                    "Lao_codepoints_exclusive_model4_heavy/weights.json",
                    "Thai_codepoints_exclusive_model4_heavy/weights.json",
                    "Thai_graphclust_model4_heavy/weights.json"
                ],
                &*list_all
            );
        }
    }

    fn get_memory_fs() -> AbstractFs {
        AbstractFs::Memory(
            [
                (
                    "Burmese_codepoints_exclusive_model4_heavy/weights.json",
                    b"".as_slice(),
                ),
                (
                    "Khmer_codepoints_exclusive_model4_heavy/weights.json",
                    b"".as_slice(),
                ),
                (
                    "Lao_codepoints_exclusive_model4_heavy/weights.json",
                    b"".as_slice(),
                ),
                (
                    "Thai_codepoints_exclusive_model4_heavy/weights.json",
                    b"".as_slice(),
                ),
                ("Thai_graphclust_model4_heavy/weights.json", b"".as_slice()),
            ]
            .into_iter()
            .collect(),
        )
    }

    fn get_zip_fs() -> AbstractFs {
        AbstractFs::new_from_url("https://github.com/unicode-org/lstm_word_segmentation/releases/download/v0.1.0/models.zip".to_string())
    }

    fn get_fs_fs() -> AbstractFs {
        AbstractFs::new("tests/data/lstm".to_string()).unwrap()
    }
}
