// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Display names for languages and regions.
//!
//! There are currently two designs for how to use this component:
//!
//! 1. [`multi`]: Load multiple display names at once.
//! 2. [`single`]: Load a single display name at a time.
//!
//! See the documentation for those submodules for more information.
//!
//! ### Comparison of Single and Multi
//!
//! The [`multi`] module lets you load multiple names at once, whereas [`single`]
//! loads one name at a time.
//!
//! Currently, the data between the two modules is NOT shared, but we hope to
//! make it shared in the future.
//!
//! We are not sure which design is better for users. If you have any feedback
//! on the design of this component, please let us know at
//! <https://github.com/unicode-org/icu4x/issues/7824>.

// TODO: expand documentation

mod displaynames;
mod options;
pub mod provider;
mod singular;

pub mod multi {
    //! Types for loading multiple display names at once.
    //!
    //! This submodule is useful for applications that need to display multiple names
    //! of the same type, such as a list of regions or scripts.
    //!
    //! ### Status
    //!
    //! Currently, this module has limited support. It supports regions and scripts,
    //! but support for languages, locales, and variants is currently missing.
    //! More features are on their way.
    //!
    //! If you have any feedback, please let us know at
    //! <https://github.com/unicode-org/icu4x/issues/7825>.
    //!
    //! See [`mod@super`] for a comparison of single and multi.
    use super::displaynames;
    pub use displaynames::LanguageDisplayNames;
    pub use displaynames::LocaleDisplayNamesFormatter;
    pub use displaynames::RegionDisplayNames;
    pub use displaynames::ScriptDisplayNames;
    pub use displaynames::VariantDisplayNames;
}

pub mod single {
    //! Types for loading a single display name at a time.
    //!
    //! This submodule is useful for applications that only need to display one or
    //! two specific names, such as the name of the current region.
    //!
    //! See [`mod@super`] for a comparison of single and multi.
    use super::singular;
    pub use singular::RegionDisplayName;
    pub use singular::ScriptDisplayName;
}

pub use displaynames::DisplayNamesPreferences;
pub use options::DisplayNamesOptions;
pub use options::Fallback;
pub use options::LanguageDisplay;
pub use options::Style;
