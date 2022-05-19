// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::DatagenError;
use crate::transform::uprops::uprops_serde;
use elsa::sync::FrozenMap;
use std::fmt::Debug;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

pub(crate) struct UpropsPaths {
    root: PathBuf,
    binary_cache: Arc<FrozenMap<String, Box<uprops_serde::binary::BinaryProperty>>>,
    enum_cache: Arc<FrozenMap<String, Box<uprops_serde::enumerated::EnumeratedPropertyMap>>>,
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
            binary_cache: Arc::new(FrozenMap::new()),
            enum_cache: Arc::new(FrozenMap::new()),
        }
    }

    pub fn get_binary(
        &self,
        key: &str,
    ) -> Result<&uprops_serde::binary::BinaryProperty, DatagenError> {
        if self.binary_cache.get(key).is_none() {
            let toml_obj: uprops_serde::binary::Main =
                self.read_and_parse_toml(&self.root.join(key).with_extension("toml"))?;
            if let Some(v) = toml_obj.binary_property.into_iter().next() {
                self.binary_cache.insert(key.to_string(), Box::new(v));
            }
        }
        self.binary_cache
            .get(key)
            .ok_or_else(|| DatagenError::Custom("adsf".to_string(), None))
    }

    pub fn get_enumerated(
        &self,
        key: &str,
    ) -> Result<&uprops_serde::enumerated::EnumeratedPropertyMap, DatagenError> {
        if self.enum_cache.get(key).is_none() {
            let toml_obj: uprops_serde::enumerated::Main =
                self.read_and_parse_toml(&self.root.join(key).with_extension("toml"))?;
            if let Some(v) = toml_obj.enum_property.into_iter().next() {
                self.enum_cache.insert(key.to_string(), Box::new(v));
            }
        }
        self.enum_cache
            .get(key)
            .ok_or_else(|| DatagenError::Custom("adsf".to_string(), None))
    }

    pub fn get_script_extensions(
        &self,
    ) -> Result<uprops_serde::script_extensions::ScriptWithExtensionsProperty, DatagenError> {
        let path = self.root.join("scx.toml");
        let toml_obj: uprops_serde::script_extensions::Main = self.read_and_parse_toml(&path)?;

        toml_obj
            .script_extensions
            .into_iter()
            .next()
            .ok_or_else(|| {
                DatagenError::Custom(
                    format!(
                        "Could not parse Script_Extensions data from TOML {:?}",
                        path
                    ),
                    None,
                )
            })
    }

    #[cfg(feature = "experimental")]
    pub fn get_case(&self) -> Result<uprops_serde::case::Main, DatagenError> {
        self.read_and_parse_toml(&self.root.join("ucase.toml"))
    }

    fn read_and_parse_toml<D>(&self, path: &Path) -> Result<D, DatagenError>
    where
        for<'de> D: serde::Deserialize<'de> + 'static,
    {
        log::trace!("Reading: {:?}", path);
        let file = std::fs::read_to_string(path).map_err(|e| (e, path))?;
        toml::from_str(&file).map_err(|e| (e, path).into())
    }
}
