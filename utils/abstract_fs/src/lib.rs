// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::BTreeMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::io::{Cursor, Read, Error, ErrorKind};
use std::path::Path;
use std::path::PathBuf;
use std::sync::RwLock;
use zip::ZipArchive;

#[derive(Debug)]
pub struct AbstractFs(AbstractFsInner);

#[derive(Debug)]
enum AbstractFsInner {
    Fs(PathBuf),
    Zip(RwLock<Result<ZipData, String>>),
    Memory(BTreeMap<&'static str, &'static [u8]>),
}

#[derive(Debug)]
struct ZipData {
    archive: ZipArchive<Cursor<Vec<u8>>>,
    file_list: HashSet<String>,
}

impl AbstractFs {
    pub fn new_from_fs<P: AsRef<Path>>(root: P) -> Result<Self, Error> {
        if std::fs::metadata(root.as_ref())?.is_dir() {
            Ok(Self(AbstractFsInner::Fs(root.as_ref().to_path_buf())))
        } else {
            let archive = ZipArchive::new(Cursor::new(std::fs::read(&root)?))
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
            let file_list = archive.file_names().map(String::from).collect();
            Ok(Self(AbstractFsInner::Zip(RwLock::new(Ok(ZipData {
                archive,
                file_list,
            })))))
        }
    }

    pub fn new_memory(data: impl IntoIterator<Item = (&'static str, &'static [u8])>) -> Self {
        Self(AbstractFsInner::Memory(data.into_iter().collect()))
    }

    #[cfg(feature = "networking")]
    pub fn new_from_url(path: String) -> Self {
        Self(AbstractFsInner::Zip(RwLock::new(Err(path))))
    }

    
    fn init(&self) -> Result<(), Error> {
        #[cfg(feature = "networking")]
        if let AbstractFsInner::Zip(lock) = &self.0 {
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
                        .map_err(|e| Error::new(ErrorKind::Other, e))?
                        .into_reader(),
                    &mut std::io::BufWriter::new(std::fs::File::create(&root)?),
                )?;
            }

            let archive = ZipArchive::new(Cursor::new(std::fs::read(&root)?))
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

            let file_list = archive.file_names().map(String::from).collect();

            *lock = Ok(ZipData { archive, file_list });
        }
        Ok(())
    }

    pub fn read_to_buf(&self, path: &str) -> Result<Vec<u8>, Error> {
        self.init()?;
        match &self.0 {
            AbstractFsInner::Fs(root) => std::fs::read(root.join(path)),
            AbstractFsInner::Zip(zip) => {
                let mut buf = Vec::new();
                zip.write()
                    .expect("poison")
                    .as_mut()
                    .ok()
                    .unwrap() // init called
                    .archive
                    .by_name(path)
                    .map_err(|e| Error::new(ErrorKind::NotFound, e))?
                    .read_to_end(&mut buf)?;
                Ok(buf)
            }
            AbstractFsInner::Memory(map) => map
                .get(path)
                .copied()
                .map(Vec::from)
                .ok_or(ErrorKind::NotFound.into()),
        }
    }

    pub fn read_to_string(&self, path: &str) -> Result<String, Error> {
        let vec = self.read_to_buf(path)?;
        let s = String::from_utf8(vec).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        Ok(s)
    }

    pub fn list(&self, path: &str) -> Result<impl Iterator<Item = String>, Error> {
        self.init()?;
        Ok(match &self.0 {
            AbstractFsInner::Fs(root) => std::fs::read_dir(root.join(path))?
                .map(|e| -> Result<_, Error> { Ok(e?.file_name().into_string().unwrap()) })
                .collect::<Result<HashSet<_>, Error>>()
                .map(HashSet::into_iter)?,
            AbstractFsInner::Zip(zip) => zip
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
            AbstractFsInner::Memory(map) => map
                .keys()
                .copied()
                .filter_map(|p| p.strip_prefix(path))
                .filter_map(|suffix| suffix.split('/').find(|s| !s.is_empty()))
                .map(String::from)
                .collect::<HashSet<_>>()
                .into_iter(),
        })
    }

    pub fn file_exists(&self, path: &str) -> Result<bool, Error> {
        self.init()?;
        Ok(match &self.0 {
            AbstractFsInner::Fs(root) => root.join(path).is_file(),
            AbstractFsInner::Zip(zip) => zip
                .read()
                .expect("poison")
                .as_ref()
                .ok()
                .unwrap() // init called
                .file_list
                .contains(path),
            AbstractFsInner::Memory(map) => map.contains_key(path),
        })
    }
}
