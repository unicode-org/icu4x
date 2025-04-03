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
use std::sync::OnceLock;
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
                        .map_err(|e| e.with_path_context(std::path::Path::new(path)))?,
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
            toml::from_str(
                std::str::from_utf8(bytes)
                    .map_err(|e| DataError::custom("TOML UTF8").with_display_context(&e))?,
            )
            .map_err(|e| DataError::custom("TOML deserialize").with_display_context(&e))
        })
    }

    pub fn list(&self, path: &str) -> Result<impl Iterator<Item = String>, DataError> {
        self.root.list(path)
    }

    pub fn file_exists(&self, path: &str) -> Result<bool, DataError> {
        self.root.file_exists(path)
    }
}

pub(crate) struct ZipData {
    archive: ZipArchive<Cursor<Vec<u8>>>,
    file_list: HashSet<String>,
}

pub(crate) struct TarArchive {
    archive: Vec<u8>,
    file_list: HashSet<String>,
}

pub(crate) enum AbstractFs {
    Fs(PathBuf),
    Zip(RwLock<Result<ZipData, String>>),
    Tar(RwLock<Result<TarArchive, String>>),
    Memory(BTreeMap<&'static str, &'static [u8]>),
}

impl Debug for AbstractFs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AbstractFs").finish()
    }
}

impl AbstractFs {
    pub fn new(root: &Path) -> Result<Self, DataError> {
        if std::fs::metadata(root)
            .map_err(|e| DataError::from(e).with_path_context(root))?
            .is_dir()
        {
            Ok(Self::Fs(root.to_path_buf()))
        } else if root.extension().is_some_and(|ext| ext == "zip") {
            let archive = ZipArchive::new(Cursor::new(std::fs::read(root)?)).map_err(|e| {
                DataError::custom("Invalid ZIP file")
                    .with_display_context(&e)
                    .with_path_context(root)
            })?;
            let file_list = archive.file_names().map(String::from).collect();
            Ok(Self::Zip(RwLock::new(Ok(ZipData { archive, file_list }))))
        } else if root.ends_with(".tar.gz") {
            use std::io::Read;
            let mut data = Vec::new();
            flate2::read::GzDecoder::new(Cursor::new(std::fs::read(root)?))
                .read_to_end(&mut data)?;

            let file_list = tar::Archive::new(Cursor::new(&data))
                .entries_with_seek()
                .map(|e| {
                    e.into_iter().filter_map(|e| {
                        Some(e.ok()?.path().ok()?.as_os_str().to_str()?.to_string())
                    })
                })?
                .collect::<HashSet<_>>();

            Ok(Self::Tar(RwLock::new(Ok(TarArchive {
                archive: data,
                file_list,
            }))))
        } else {
            Err(DataError::custom("unsupported archive type").with_display_context(&root.display()))
        }
    }

    #[cfg(feature = "networking")]
    pub fn new_from_url(path: String) -> Self {
        if path.ends_with(".zip") {
            Self::Zip(RwLock::new(Err(path)))
        } else {
            Self::Tar(RwLock::new(Err(path)))
        }
    }

    fn init(&self) -> Result<(), DataError> {
        #[cfg(feature = "networking")]
        fn download(resource: &String) -> Result<PathBuf, DataError> {
            let root = std::env::var_os("ICU4X_SOURCE_CACHE")
                .map(PathBuf::from)
                .unwrap_or_else(|| std::env::temp_dir().join("icu4x-source-cache/"))
                .join(resource.rsplit("//").next().unwrap());
            if !root.exists() {
                log::info!("Downloading {resource}");
                std::fs::create_dir_all(root.parent().unwrap())?;
                let mut retry = 5;
                let mut response = loop {
                    match ureq::get(resource).call() {
                        Ok(r) => break r.into_body().into_reader(),
                        Err(e) if retry > 0 => {
                            log::warn!("Download error {e:?}, retrying...");
                            std::thread::sleep(std::time::Duration::from_secs(2));
                            retry -= 1;
                        }
                        Err(e) => {
                            return Err(DataError::custom("Download").with_display_context(&e))
                        }
                    }
                };
                std::io::copy(&mut response, &mut BufWriter::new(File::create(&root)?))?;
            }
            Ok(root)
        }

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

            let root = download(resource)?;

            let archive = ZipArchive::new(Cursor::new(std::fs::read(&root)?)).map_err(|e| {
                DataError::custom("Invalid ZIP file")
                    .with_display_context(&e)
                    .with_path_context(&root)
            })?;

            let file_list = archive.file_names().map(String::from).collect();

            *lock = Ok(ZipData { archive, file_list });
        } else if let Self::Tar(lock) = self {
            if lock.read().expect("poison").is_ok() {
                return Ok(());
            }
            let mut lock = lock.write().expect("poison");
            let resource = if let Err(resource) = &*lock {
                resource
            } else {
                return Ok(());
            };

            use std::io::Read;
            let mut data = Vec::new();
            flate2::read::GzDecoder::new(Cursor::new(std::fs::read(&download(resource)?)?))
                .read_to_end(&mut data)?;

            let file_list = tar::Archive::new(Cursor::new(&data))
                .entries_with_seek()
                .map(|e| {
                    e.into_iter().filter_map(|e| {
                        Some(e.ok()?.path().ok()?.as_os_str().to_str()?.to_string())
                    })
                })?
                .collect::<HashSet<_>>();

            *lock = Ok(TarArchive {
                archive: data,
                file_list,
            })
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
            Self::Tar(tar) => {
                log::debug!("Reading: <tar>/{}", path);
                tar::Archive::new(Cursor::new(
                    &tar.read().expect("poison").as_ref().unwrap().archive,
                )) // init called
                .entries_with_seek()
                .and_then(|e| {
                    for e in e {
                        let mut e = e?;
                        if e.path()?.as_os_str() == path {
                            let mut vec = vec![];
                            e.read_to_end(&mut vec)?;
                            return Ok(vec);
                        }
                    }
                    Err(std::io::ErrorKind::NotFound.into())
                })
                .map_err(|e| {
                    DataErrorKind::Io(e.kind())
                        .into_error()
                        .with_display_context(&e)
                        .with_display_context(path)
                })
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
            Self::Tar(tar) => tar
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
            Self::Tar(tar) => tar
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
}

#[derive(Debug)]
pub(crate) struct TzdbCache {
    pub(crate) root: AbstractFs,
    pub(crate) transitions: OnceLock<Result<parse_zoneinfo::table::Table, DataError>>,
}

impl TzdbCache {
    pub(crate) fn transitions(&self) -> Result<&parse_zoneinfo::table::Table, DataError> {
        use parse_zoneinfo::line::{Line, LineParser};
        use parse_zoneinfo::table::TableBuilder;

        self.transitions
            .get_or_init(|| {
                let tzfiles = [
                    "africa",
                    "antarctica",
                    "asia",
                    "australasia",
                    "backward",
                    "etcetera",
                    "europe",
                    "northamerica",
                    "southamerica",
                ];

                let mut lines = Vec::<String>::new();

                for file in tzfiles {
                    lines.extend(
                        self.root
                            .read_to_string(file)?
                            .lines()
                            .map(ToOwned::to_owned),
                    );
                }

                enum Section {
                    Normal,
                    Vanguard,
                    Rearguard,
                }
                let mut i = 0;
                let mut section = Section::Normal;

                while i < lines.len() {
                    match section {
                        Section::Normal => {
                            if lines[i].starts_with("# Vanguard section") {
                                lines.remove(i);
                                section = Section::Vanguard;
                            } else if lines[i].starts_with('#') {
                                lines.remove(i);
                            } else {
                                i += 1;
                            }
                        }
                        Section::Vanguard => {
                            if lines[i].starts_with("# Rearguard section") {
                                section = Section::Rearguard;
                            }
                            lines.remove(i);
                        }
                        Section::Rearguard => {
                            if lines[i].starts_with("# End of rearguard section") {
                                section = Section::Normal;
                                lines.remove(i);
                            } else {
                                // Rearguard lines mighht start with a # not followed by a space (that's a comment), or
                                // they might not ¯\_(ツ)_/¯.
                                if (lines[i].starts_with('#') && !lines[i].starts_with("# "))
                                    || !lines[i].contains('#')
                                {
                                    lines[i] =
                                        lines[i].strip_prefix('#').unwrap_or(&lines[i]).into();
                                    i += 1;
                                } else {
                                    lines.remove(i);
                                }
                            }
                        }
                    }
                }

                // Morocco doesn't have have rearguard data in the text file, so we have to replicate the transform from
                // ziguard.awk: https://github.com/eggert/tz/blob/271a5784a59e454b659d85948b5e65c17c11516a/ziguard.awk#L261-L299
                for line in lines.iter_mut() {
                    if line.starts_with("Rule\tMorocco") {
                        let mut parts = line.split('\t').skip(2);
                        let from = parts.next().unwrap();
                        let to = parts.next().unwrap();
                        let _type = parts.next().unwrap();
                        let month = parts.next().unwrap();
                        let _day = parts.next().unwrap();
                        let _time = parts.next().unwrap();
                        let save = parts.next().unwrap();
                        if to == "2018" && month == "Oct" {
                            *line = line.replace("2018", "2017");
                        } else if from.parse::<i32>().unwrap() >= 2019 {
                            if save.trim() == "0" {
                                *line = line.replace("\t0\t", "\t1:00\t");
                            } else {
                                *line = line.replace("\t-1:00\t", "\t0\t");
                            }
                        }
                    }
                    *line = line.replace("1:00\tMorocco\t%z", "0:00\tMorocco\t+00/+01");
                }

                #[allow(deprecated)] // no alternative?!
                let parser = LineParser::new();
                let mut table = TableBuilder::new();

                for line in lines {
                    match parser.parse_str(&line).unwrap() {
                        Line::Zone(zone) => table.add_zone_line(zone).unwrap(),
                        Line::Continuation(cont) => table.add_continuation_line(cont).unwrap(),
                        Line::Rule(rule) => table.add_rule_line(rule).unwrap(),
                        Line::Link(link) => table.add_link_line(link).unwrap(),
                        Line::Space => {}
                    }
                }

                Ok(table.build())
            })
            .as_ref()
            .map_err(|&e| e)
    }
}
