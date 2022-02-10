// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::prelude::*;

/// A data provider whose supported keys are known statically at compile time.
///
/// Implementing this trait means that a provider is built to support a specific set of
/// keys; for example, by transforming those keys from an external data source.
///
/// TODO(#442): Think about a better way to do this. This is not fully supported.
/// TODO: When const_trait_impl is stable, most implementations of this trait should be const.
pub trait KeyedDataProvider {
    fn supported_keys() -> Vec<ResourceKey>;
}
