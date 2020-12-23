// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
// Decimal types
use crate::prelude::*;
use serde::{Deserialize, Serialize};
use smallstr::SmallString;

pub mod key {
    use crate::resource::ResourceKey;
    pub const SYMBOLS_V1: ResourceKey = resource_key!(decimal, "symbols", 1);
}

/// Gets a locale-invariant default struct given a data key in this module's category.
#[cfg(feature = "invariant")]
pub fn get_invariant<'d>(
    resc_key: &ResourceKey,
    receiver: &mut dyn DataReceiver<'d, 'static>,
) -> Result<(), DataError> {
    match *resc_key {
        key::SYMBOLS_V1 => receiver.receive_invariant::<SymbolsV1>(),
        _ => Err(DataError::UnsupportedResourceKey(*resc_key)),
    }
}

/// Gets a boxed DataReceiver capable of receiving a data key in this module's category.
pub fn get_receiver<'d>(resc_key: &ResourceKey) -> Option<Box<dyn DataReceiver<'d, 'static> + 'd>> {
    match *resc_key {
        key::SYMBOLS_V1 => Some(DataReceiverForType::<SymbolsV1>::new_boxed()),
        _ => None,
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct SymbolsV1 {
    pub zero_digit: char,
    pub decimal_separator: SmallString<[u8; 8]>,
    pub grouping_separator: SmallString<[u8; 8]>,
}

#[cfg(feature = "invariant")]
impl Default for SymbolsV1 {
    fn default() -> Self {
        Self {
            zero_digit: '0',
            decimal_separator: ".".into(),
            grouping_separator: ",".into(),
        }
    }
}
