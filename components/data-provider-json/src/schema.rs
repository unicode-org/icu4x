use icu_data_provider as datap;
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use std::prelude::v1::*;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct DecimalJsonSchema {
    pub(crate) symbols_v1_a: datap::decimal::SymbolsV1,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct JsonSchema {
    pub(crate) decimal: DecimalJsonSchema,
}
