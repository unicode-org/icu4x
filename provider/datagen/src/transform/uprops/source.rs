// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use elsa::sync::FrozenMap;
use icu_provider::{DataError, DataErrorKind};
use std::any::Any;
use std::fmt::Debug;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

pub(crate) struct TomlPaths {
    root: PathBuf,
    cache: Arc<FrozenMap<PathBuf, Box<dyn Any + Send + Sync>>>,
}

impl Debug for TomlPaths {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TomlPaths")
            .field("root", &self.root)
            // skip formatting the cache
            .finish()
    }
}

impl TomlPaths {
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            cache: Arc::new(FrozenMap::new()),
        }
    }

    pub fn read_and_parse_toml<S, P: AsRef<Path>>(&self, path: P) -> Result<&S, DataError>
    where
        for<'de> S: serde::Deserialize<'de> + 'static + Send + Sync,
    {
        let path = self.root.join(path);

        if self.cache.get(&path).is_none() {
            log::trace!("Reading: {:?}", &path);
            let file = std::fs::read_to_string(&path).map_err(|_| {
                DataErrorKind::MissingResourceKey
                    .into_error()
                    .with_path_context(&path)
            })?; // treat missing files as key errors
            let file: S = toml::from_str(&file)
                .map_err(|e| crate::error::data_error_from_toml(e).with_path_context(&path))?;
            self.cache.insert(path.clone(), Box::new(file));
        }
        self.cache
            .get(&path)
            .unwrap()
            .downcast_ref::<S>()
            .ok_or_else(|| DataError::custom("Uprops TOML error").with_type_context::<S>())
    }

    #[cfg_attr(not(feature = "experimental"), allow(dead_code))]
    pub fn list(&self) -> Result<impl Iterator<Item = PathBuf>, DataError> {
        let mut result = vec![];
        for entry in std::fs::read_dir(&self.root)
            .map_err(|e| DataError::from(e).with_path_context(&self.root))?
        {
            let entry = entry.map_err(|e| DataError::from(e).with_path_context(&self.root))?;
            let path = entry.path();
            result.push(path);
        }
        Ok(result.into_iter())
    }
}
