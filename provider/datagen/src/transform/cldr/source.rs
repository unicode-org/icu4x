// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::source::AbstractFs;
use elsa::sync::FrozenMap;
use icu_locid::LanguageIdentifier;
use icu_provider::DataError;
use std::any::Any;
use std::fmt::Debug;
use std::str::FromStr;
use std::sync::Arc;

pub(crate) struct CldrCache {
    root: AbstractFs,
    locale_subset: String,
    cache: Arc<FrozenMap<String, Box<dyn Any + Send + Sync>>>,
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
    pub(crate) fn new(root: AbstractFs, locale_subset: String) -> Self {
        Self {
            root,
            locale_subset,
            cache: Arc::new(FrozenMap::new()),
        }
    }

    pub(crate) fn cldr_core(&self) -> CldrDirNoLang<'_> {
        CldrDirNoLang(self, "cldr-core".to_string())
    }

    pub(crate) fn cldr_numbers(&self) -> CldrDirLang<'_> {
        CldrDirLang(self, format!("cldr-numbers-{}/main", self.locale_subset))
    }

    pub(crate) fn cldr_misc(&self) -> CldrDirLang<'_> {
        CldrDirLang(self, format!("cldr-misc-{}/main", self.locale_subset))
    }

    pub(crate) fn cldr_bcp47(&self) -> CldrDirNoLang<'_> {
        CldrDirNoLang(self, "cldr-bcp47/bcp47".to_string())
    }

    pub(crate) fn cldr_dates(&self, cal: &str) -> CldrDirLang<'_> {
        CldrDirLang(
            self,
            if cal == "gregorian" {
                format!("cldr-dates-{}/main", self.locale_subset)
            } else {
                format!("cldr-cal-{}-{}/main", cal, self.locale_subset)
            },
        )
    }
}

pub(crate) struct CldrDirNoLang<'a>(&'a CldrCache, String);

impl<'a> CldrDirNoLang<'a> {
    pub(crate) fn read_and_parse<S>(&self, file_name: &str) -> Result<&'a S, DataError>
    where
        for<'de> S: serde::Deserialize<'de> + 'static + Send + Sync,
    {
        read_and_parse_json(self.0, &format!("{}/{}", self.1, file_name))
    }
}

pub(crate) struct CldrDirLang<'a>(&'a CldrCache, String);

impl<'a> CldrDirLang<'a> {
    pub(crate) fn read_and_parse<S>(
        &self,
        lang: &LanguageIdentifier,
        file_name: &str,
    ) -> Result<&'a S, DataError>
    where
        for<'de> S: serde::Deserialize<'de> + 'static + Send + Sync,
    {
        read_and_parse_json(self.0, &format!("{}/{}/{}", self.1, lang, file_name))
    }

    pub(crate) fn list_langs(&self) -> Result<impl Iterator<Item = LanguageIdentifier>, DataError> {
        Ok(self.0.root.list(&self.1)?.into_iter().map(|path| {
            LanguageIdentifier::from_str(&path.file_name().unwrap().to_string_lossy()).unwrap()
        }))
    }
}

fn read_and_parse_json<'a, S>(paths: &'a CldrCache, path: &str) -> Result<&'a S, DataError>
where
    for<'de> S: serde::Deserialize<'de> + 'static + Send + Sync,
{
    match paths.cache.get(path) {
        Some(x) => x,
        None => {
            let file = paths.root.read_to_buf(path)?;
            let file: S = serde_json::from_slice(&file)
                .map_err(|e| DataError::from(e).with_path_context(&path))?;
            paths.cache.insert(path.to_string(), Box::new(file))
        }
    }
    .downcast_ref::<S>()
    .ok_or_else(|| DataError::custom("CLDR JSON error").with_type_context::<S>())
}
