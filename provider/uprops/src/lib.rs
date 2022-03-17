// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_provider_uprops` contains implementations of the [`ICU4X`]
//! [data provider] interface backed by TOML files exported by the
//! ICU4C icuwriteuprops tool. Create a directory containing TOML files for
//! the necessary Unicode properties and then pass the path to the desired
//! provider.
//!
//! **Important:** This data provider implementation is not optimized
//! for production use.  It is much more efficient if you use
//! [`FsDataProvider`] or [`StaticDataProvider`] instead.
//!
//! [`ICU4X`]: ../icu/index.html
//! [data provider]: icu_provider
//! [`FsDataProvider`]: ../icu_provider_fs/struct.FsDataProvider.html
//! [`StaticDataProvider`]: ../icu_provider_blob/struct.StaticDataProvider.html

mod bin_uniset;
mod enum_codepointtrie;
mod enum_uniset;
mod reader;
mod script;
mod uprops_helpers;
mod uprops_serde;

pub use bin_uniset::BinaryPropertyUnicodeSetDataProvider;
#[cfg(feature = "casemapping")]
pub mod casemapping;

pub use enum_codepointtrie::EnumeratedPropertyCodePointTrieProvider;
pub use enum_uniset::EnumeratedPropertyUnicodeSetDataProvider;
pub use script::ScriptWithExtensionsPropertyProvider;

use icu_provider::datagen::OmnibusDatagenProvider;
use icu_provider::fork::by_key::MultiForkByKeyProvider;
use icu_provider::DataMarker;
use std::path::Path;

pub fn create_exportable_provider<T: DataMarker>(
    root_dir: &Path,
) -> eyre::Result<MultiForkByKeyProvider<Box<dyn OmnibusDatagenProvider<T> + Sync>>>
where
    EnumeratedPropertyCodePointTrieProvider: OmnibusDatagenProvider<T>,
    ScriptWithExtensionsPropertyProvider: OmnibusDatagenProvider<T>,
    EnumeratedPropertyUnicodeSetDataProvider: OmnibusDatagenProvider<T>,
    BinaryPropertyUnicodeSetDataProvider: OmnibusDatagenProvider<T>,
{
    Ok(MultiForkByKeyProvider {
        providers: vec![
            Box::new(EnumeratedPropertyCodePointTrieProvider::try_new(root_dir)?),
            Box::new(ScriptWithExtensionsPropertyProvider::try_new(root_dir)?),
            Box::new(EnumeratedPropertyUnicodeSetDataProvider::try_new(root_dir)?),
            // Has to go last as it matches all props/ keys.
            Box::new(BinaryPropertyUnicodeSetDataProvider::try_new(root_dir)?),
        ],
    })
}
