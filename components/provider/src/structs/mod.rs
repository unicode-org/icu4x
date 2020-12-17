// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
//! Collection of data structures for `DataProvider`.
pub mod dates;
pub mod decimal;
pub mod plurals;

use crate::prelude::*;

/// Gets a locale-invariant default struct given a data key in this module's category.
/// For example, if the data key is `plurals/cardinal@1`, a Response with an object of type
/// `PluralRuleStringsV1` will be returned.
#[cfg(feature = "invariant")]
pub(crate) fn get_invariant(data_key: &DataKey) -> Option<DataResponse<'static>> {
    None //
        .or_else(|| decimal::get_invariant(data_key)) //
        .or_else(|| plurals::get_invariant(data_key)) //
        .or_else(|| dates::get_invariant(data_key)) //
}

pub fn get_receiver<'d>(data_key: &DataKey) -> Option<Box<dyn DataReceiver<'d, 'static> + 'd>> {
    None //
        .or_else(|| decimal::get_receiver(data_key)) //
        .or_else(|| plurals::get_receiver(data_key)) //
        .or_else(|| dates::get_receiver(data_key)) //
}
