// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use elsa::sync::FrozenMap;
use icu_locid::LanguageIdentifier;
use icu_provider::DataError;
use std::any::Any;
use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;

pub(crate) struct CldrCache {
    root: PathBuf,
    locale_subset: String,
    cache: Arc<FrozenMap<PathBuf, Box<dyn Any + Send + Sync>>>,
}

impl Debug for CldrCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CldrCache")
            .field("root", &self.root)
            .field("locale_subset", &self.locale_subset)
            // skip formatting the cache
            .finish()
    }
}

impl CldrCache {
    pub(crate) fn new(root: PathBuf, locale_subset: String) -> Self {
        Self {
            root,
            locale_subset,
            cache: Arc::new(FrozenMap::new()),
        }
    }

    pub(crate) fn cldr_core(&self) -> CldrDirNoLang<'_> {
        CldrDirNoLang(self.root.join("cldr-core"), self.cache.as_ref())
    }

    pub(crate) fn cldr_numbers(&self) -> CldrDirLang<'_> {
        CldrDirLang(
            self.root
                .join(format!("cldr-numbers-{}", self.locale_subset))
                .join("main"),
            self.cache.as_ref(),
        )
    }

    pub(crate) fn cldr_misc(&self) -> CldrDirLang<'_> {
        CldrDirLang(
            self.root
                .join(format!("cldr-misc-{}", self.locale_subset))
                .join("main"),
            self.cache.as_ref(),
        )
    }

    pub(crate) fn cldr_bcp47(&self) -> CldrDirNoLang<'_> {
        CldrDirNoLang(
            self.root.join("cldr-bcp47").join("bcp47"),
            self.cache.as_ref(),
        )
    }

    pub(crate) fn cldr_dates(&self, cal: &str) -> CldrDirLang<'_> {
        CldrDirLang(
            if cal == "gregorian" {
                self.root.join(format!("cldr-dates-{}", self.locale_subset))
            } else {
                self.root
                    .join(format!("cldr-cal-{}-{}", cal, self.locale_subset))
            }
            .join("main"),
            self.cache.as_ref(),
        )
    }
}

pub(crate) struct CldrDirNoLang<'a>(PathBuf, &'a FrozenMap<PathBuf, Box<dyn Any + Send + Sync>>);

impl<'a> CldrDirNoLang<'a> {
    pub(crate) fn read_and_parse<S>(&self, file_name: &str) -> Result<&'a S, DataError>
    where
        for<'de> S: serde::Deserialize<'de> + 'static + Send + Sync,
    {
        read_and_parse_json(&self.0.join(file_name), self.1)
    }
}

pub(crate) struct CldrDirLang<'a>(PathBuf, &'a FrozenMap<PathBuf, Box<dyn Any + Send + Sync>>);

impl<'a> CldrDirLang<'a> {
    pub(crate) fn read_and_parse<S>(
        &self,
        lang: &LanguageIdentifier,
        file_name: &str,
    ) -> Result<&'a S, DataError>
    where
        for<'de> S: serde::Deserialize<'de> + 'static + Send + Sync,
    {
        read_and_parse_json(&self.0.join(lang.to_string()).join(file_name), self.1)
    }

    pub(crate) fn list_langs(&self) -> Result<impl Iterator<Item = LanguageIdentifier>, DataError> {
        let mut result = vec![];
        for entry in
            fs::read_dir(&self.0).map_err(|e| DataError::from(e).with_path_context(&self.0))?
        {
            let entry = entry.map_err(|e| DataError::from(e).with_path_context(&self.0.clone()))?;
            let path = entry.path();
            result.push(path);
        }
        Ok(result.into_iter().map(|path| {
            LanguageIdentifier::from_str(&path.file_name().unwrap().to_string_lossy()).unwrap()
        }))
    }
}

fn read_and_parse_json<'a, S>(
    path: &Path,
    cache: &'a FrozenMap<PathBuf, Box<dyn Any + Send + Sync>>,
) -> Result<&'a S, DataError>
where
    for<'de> S: serde::Deserialize<'de> + 'static + Send + Sync,
{
    if cache.get(path).is_none() {
        log::trace!("Reading: {:?}", path);
        let file = File::open(path).map_err(|e| DataError::from(e).with_path_context(&path))?;
        let file: S = serde_json::from_reader(BufReader::new(file))
            .map_err(|e| DataError::from(e).with_path_context(&path))?;
        cache.insert(path.to_path_buf(), Box::new(file));
    }
    cache
        .get(path)
        .unwrap()
        .downcast_ref::<S>()
        .ok_or_else(|| DataError::custom("CLDR JSON error").with_type_context::<S>())
}
