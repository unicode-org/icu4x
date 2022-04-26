// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::uprops::reader::*;

use crate::error::DatagenError;
use crate::uprops::uprops_serde;
use std::collections::HashMap;
use std::path::Path;

pub type TomlEnumerated = HashMap<String, uprops_serde::enumerated::EnumeratedPropertyMap>;
pub type TomlBinary = HashMap<String, uprops_serde::binary::BinaryProperty>;

pub fn load_binary_from_dir(root_dir: &Path) -> Result<TomlBinary, DatagenError> {
    let mut result = HashMap::new();
    for path in get_dir_contents(root_dir)? {
        let key: String = path
            .file_stem()
            .and_then(|p| p.to_str())
            .ok_or_else(|| DatagenError::Custom(format!("Invalid file name: {:?}", path), None))?
            .to_string();
        let toml_str = read_path_to_string(&path)?;
        let toml_obj: uprops_serde::binary::Main =
            toml::from_str(&toml_str).map_err(|e| (e, path))?;
        if let Some(v) = toml_obj.binary_property.into_iter().next() {
            result.insert(key, v);
        }
    }
    Ok(result)
}

pub fn load_enumerated_from_dir(root_dir: &Path) -> Result<TomlEnumerated, DatagenError> {
    let mut result = HashMap::new();
    for path in get_dir_contents(root_dir)? {
        let key: String = path
            .file_stem()
            .and_then(|p| p.to_str())
            .ok_or_else(|| DatagenError::Custom(format!("Invalid file name: {:?}", path), None))?
            .to_string();
        let toml_str = read_path_to_string(&path)?;
        let toml_obj: uprops_serde::enumerated::Main =
            toml::from_str(&toml_str).map_err(|e| (e, path))?;
        if let Some(v) = toml_obj.enum_property.into_iter().next() {
            result.insert(key, v);
        }
    }
    Ok(result)
}

pub fn load_script_extensions_from_dir(
    root_dir: &Path,
) -> Result<uprops_serde::script_extensions::ScriptWithExtensionsProperty, DatagenError> {
    let mut path = root_dir.join("scx");
    path.set_extension("toml");
    let toml_str = read_path_to_string(&path)?;
    let toml_obj: uprops_serde::script_extensions::Main =
        toml::from_str(&toml_str).map_err(|e| (e, &path))?;

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

pub fn get_last_component_no_version(key: icu_provider::ResourceKey) -> &'static str {
    #[allow(clippy::expect_used)] // TODO(#1668) Clippy exceptions need docs or fixingw.
    key.get_path()
        .split('/')
        .last()
        .expect("str::split doesn't return empty iterators")
        .split('@')
        .next()
        .expect("str::split doesn't return empty iterators")
}
