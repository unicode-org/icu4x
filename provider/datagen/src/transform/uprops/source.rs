// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::uprops::uprops_serde;
use elsa::sync::FrozenMap;
use icu_provider::DataError;
use std::fmt::Debug;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

pub(crate) struct UpropsPaths {
    root: PathBuf,
    cache: Arc<FrozenMap<PathBuf, Box<dyn Any + Send + Sync>>>,
}

impl Debug for UpropsPaths {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UpropsPaths")
            .field("root", &self.root)
            // skip formatting the cache
            .finish()
    }
}

impl UpropsPaths {
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            cache: Arc::new(FrozenMap::new()),
        }
    }

    pub fn read_and_parse_toml<D, P: AsRef<Path>>(&self, path: P) -> Result<D, DataError>
    where
        for<'de> D: serde::Deserialize<'de> + 'static,
    {
        let path = self.root.join(path);

        if cache.get(path).is_none() {
            log::trace!("Reading: {:?}", path);

            let file = std::fs::read_to_string(path)
            .map_err(|e| DataError::from(e).with_path_context(path))?;
            let file: S = toml::from_str(&file)
            .map_err(|e| crate::error::data_error_from_toml(e).with_path_context(path));
            cache.insert(path.to_path_buf(), Box::new(file));
        }
        cache
            .get(path)
            .unwrap()
            .downcast_ref::<S>()
            .ok_or_else(|| DataError::for_type::<S>())
    }
}
