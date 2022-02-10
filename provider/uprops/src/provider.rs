// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::fork::by_key::MultiForkByKeyProvider;
use icu_provider::iter::IterableDynProvider;
use icu_provider::serde::SerializeMarker;
use std::path::Path;

/// This data provider returns `UnicodeSet` data inside a `UnicodeProperty`
/// data struct. The source data is in the form of a directory of TOML file(s)
/// of data for the property(-ies) desired, as given by the ICU4C property data
/// exporter tool.
pub struct PropertiesDataProvider;
impl PropertiesDataProvider {
    pub fn try_new(
        root_dir: &Path,
    ) -> eyre::Result<MultiForkByKeyProvider<Box<dyn IterableDynProvider<SerializeMarker>>>> {
        Ok(MultiForkByKeyProvider {
            providers: vec![
                Box::new(
                    crate::enum_codepointtrie::EnumeratedPropertyCodePointTrieProvider::try_new(
                        root_dir,
                    )?,
                ),
                Box::new(crate::script::ScriptWithExtensionsPropertyProvider::try_new(root_dir)?),
                Box::new(
                    crate::enum_uniset::EnumeratedPropertyUnicodeSetDataProvider::try_new(
                        root_dir,
                    )?,
                ),
                // Has to go last as it matches all props/ keys.
                Box::new(
                    crate::bin_uniset::BinaryPropertyUnicodeSetDataProvider::try_new(root_dir)?,
                ),
            ],
        })
    }
}
