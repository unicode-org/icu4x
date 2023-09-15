// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use elsa::sync::FrozenMap;
use icu_provider::prelude::*;
use std::any::Any;
#[cfg(feature = "legacy_api")]
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
use zip::ZipArchive;

pub(crate) struct SerdeCache {
    pub(crate) root: AbstractFs,
    cache: FrozenMap<String, Box<dyn Any + Send + Sync>>,
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
        self.root.list(path, false)
    }

    pub fn file_exists(&self, path: &str) -> Result<bool, DataError> {
        self.root.file_exists(path)
    }
}

pub(crate) struct ZipData {
    archive: ZipArchive<Cursor<Vec<u8>>>,
    file_list: HashSet<String>,
}

pub(crate) enum AbstractFs {
    Fs(PathBuf),
    Zip(RwLock<Result<ZipData, String>>),
    #[cfg(feature = "legacy_api")]
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

    /// Returns the size, in bytes, of the entry at `path`.
    pub fn size(&self, path: &str) -> Result<u64, DataError> {
        self.init()?;
        match self {
            AbstractFs::Fs(root) => root
                .join(path)
                .metadata()
                .map(|metadata| metadata.len())
                .map_err(Into::into),
            AbstractFs::Zip(zip) => zip
                .write()
                .expect("poison")
                .as_mut()
                .ok()
                .unwrap() // init called
                .archive
                .by_name(path)
                .map(|entry| entry.size())
                .map_err(|e| {
                    DataError::custom("Zip")
                        .with_display_context(&e)
                        .with_display_context(path)
                }),
            #[cfg(feature = "legacy_api")]
            AbstractFs::Memory(map) => {
                map.get(path)
                    .copied()
                    .map(|b| b.len() as u64)
                    .ok_or_else(|| {
                        DataError::custom("Not found in icu4x-datagen's data/")
                            .with_display_context(path)
                    })
            }
        }
    }

    /// Returns [`true`] if the entry at `path` is a file, otherwise [`false`]
    pub fn is_file(&self, path: &str) -> Result<bool, DataError> {
        self.init()?;
        match self {
            AbstractFs::Fs(root) => root
                .join(path)
                .metadata()
                .map(|metadata| metadata.is_file())
                .map_err(Into::into),
            AbstractFs::Zip(zip) => zip
                .write()
                .expect("poison")
                .as_mut()
                .ok()
                .unwrap() // init called
                .archive
                .by_name(path)
                .map(|entry| entry.is_file())
                .map_err(|e| {
                    DataError::custom("Zip")
                        .with_display_context(&e)
                        .with_display_context(path)
                }),
            #[cfg(feature = "legacy_api")]
            AbstractFs::Memory(map) => Ok(map.contains_key(path)),
        }
    }

    pub fn read_to_buf(&self, path: &str) -> Result<Vec<u8>, DataError> {
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
            #[cfg(feature = "legacy_api")]
            Self::Memory(map) => map.get(path).copied().map(Vec::from).ok_or_else(|| {
                DataError::custom("Not found in icu4x-datagen's data/").with_display_context(path)
            }),
        }
    }

    pub fn read_to_buf_exact(&self, n: usize, path: &str) -> Result<Vec<u8>, DataError> {
        self.init()?;
        match self {
            Self::Fs(root) => {
                log::trace!("Reading: {}/{}", root.display(), path);
                let mut buf = vec![0; n];
                let mut file = std::fs::File::open(root.join(path))?;
                file.read_exact(&mut buf)
                    .map(|_| buf)
                    .map_err(|e| DataError::from(e).with_path_context(&root.join(path)))
            }
            Self::Zip(zip) => {
                log::trace!("Reading: <zip>/{}", path);
                let mut buf = vec![0; n];
                zip.write()
                    .expect("poison")
                    .as_mut()
                    .ok()
                    .unwrap() // init called
                    .archive
                    .by_name(path)
                    .map_err(|e| {
                        DataError::custom("Zip")
                            .with_display_context(&e)
                            .with_display_context(path)
                    })?
                    .read_exact(&mut buf)?;
                Ok(buf)
            }
            #[cfg(feature = "legacy_api")]
            Self::Memory(map) => map
                .get(path)
                .copied()
                .and_then(|d| d.get(..n).map(Vec::from))
                .ok_or_else(|| {
                    DataError::custom("Not found in icu4x-datagen's data/")
                        .with_display_context(path)
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

    pub fn list(
        &self,
        path: &str,
        recursive: bool,
    ) -> Result<impl Iterator<Item = String>, DataError> {
        self.init()?;
        Ok(match self {
            Self::Fs(root) => {
                let path = root.join(path);
                if recursive {
                    walkdir::WalkDir::new(&path)
                        .follow_links(true)
                        .into_iter()
                        .flatten()
                        .filter(|entry| entry.file_type().is_file())
                        .map(|file| {
                            file.into_path()
                                .strip_prefix(&path)
                                .unwrap()
                                .to_string_lossy()
                                .into_owned()
                        })
                        .collect::<HashSet<_>>()
                        .into_iter()
                } else {
                    std::fs::read_dir(&path)
                        .map_err(|e| DataError::from(e).with_display_context(&path.display()))?
                        .map(|e| -> Result<_, DataError> {
                            Ok(e?.file_name().to_string_lossy().into_owned())
                        })
                        .collect::<Result<HashSet<_>, DataError>>()
                        .map(HashSet::into_iter)?
                }
            }
            Self::Zip(zip) => zip
                .read()
                .expect("poison")
                .as_ref()
                .ok()
                .unwrap() // init called
                .file_list
                .iter()
                .filter_map(|p| p.strip_prefix(path))
                .filter_map(|suffix| {
                    if recursive {
                        Some(suffix)
                    } else {
                        suffix.split('/').find(|s| !s.is_empty())
                    }
                })
                .map(ToOwned::to_owned)
                .collect::<HashSet<_>>()
                .into_iter(),
            #[cfg(feature = "legacy_api")]
            Self::Memory(map) => map
                .keys()
                .filter_map(|p| p.strip_prefix(path))
                .filter_map(|suffix| {
                    if recursive {
                        Some(suffix)
                    } else {
                        suffix.split('/').find(|s| !s.is_empty())
                    }
                })
                .map(ToOwned::to_owned)
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
            #[cfg(feature = "legacy_api")]
            Self::Memory(map) => map.contains_key(path),
        })
    }
}
