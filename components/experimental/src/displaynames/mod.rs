// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Display names for languages and regions.

// TODO: expand documentation

mod displaynames;
mod options;
pub mod provider;

pub use displaynames::DisplayNamesPreferences;
pub use displaynames::LanguageDisplayNames;
pub use displaynames::LocaleDisplayNamesFormatter;
pub use displaynames::RegionDisplayNames;
pub use displaynames::ScriptDisplayNames;
pub use displaynames::VariantDisplayNames;
pub use options::DisplayNamesOptions;
pub use options::Fallback;
pub use options::LanguageDisplay;
pub use options::Style;
