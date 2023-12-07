// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub(crate) use abstract_fs::AbstractFs;
use elsa::sync::FrozenMap;
use icu_provider::prelude::*;
use std::any::Any;
use std::fmt::Debug;

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
            None => {
                log::debug!("Reading: {}", path);
                let bytes = self
                    .root
                    .read_to_buf(path)
                    .map_err(|e| DataError::from(e).with_path_context(&path))?;
                let parsed = parser(&bytes).map_err(|e| e.with_path_context(&path))?;
                self.cache.insert(path.to_string(), Box::new(parsed))
            }
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
        Ok(self.root.list(path)?)
    }

    pub fn file_exists(&self, path: &str) -> Result<bool, DataError> {
        Ok(self.root.file_exists(path)?)
    }
}
