// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
//! Collection of data structures for `DataProvider`.
pub mod dates;
pub mod decimal;
pub mod plurals;

use crate::prelude::*;

/// Gets a locale-invariant default struct given a resource key in this module's category.
/// For example, if the resource key is `plurals/cardinal@1`, a Response with an object of type
/// `PluralRuleStringsV1` will be returned.
#[cfg(feature = "invariant")]
pub fn get_invariant<'d>(
    resc_key: &ResourceKey,
    receiver: &mut dyn DataReceiver<'d, 'static>,
) -> Result<(), DataError> {
    Err(DataError::UnsupportedResourceKey(*resc_key)) //
        .or_else(|_| decimal::get_invariant(resc_key, receiver)) //
        .or_else(|_| plurals::get_invariant(resc_key, receiver)) //
        .or_else(|_| dates::get_invariant(resc_key, receiver)) //
}

/// Gets a boxed DataReceiver capable of receiving any known data struct.
pub fn get_receiver<'d>(
    resc_key: &ResourceKey,
) -> Result<Box<dyn DataReceiver<'d, 'static> + 'd>, DataError> {
    None //
        .or_else(|| decimal::get_receiver(resc_key)) //
        .or_else(|| plurals::get_receiver(resc_key)) //
        .or_else(|| dates::get_receiver(resc_key)) //
        .ok_or_else(|| DataError::UnsupportedResourceKey(*resc_key)) //
}
