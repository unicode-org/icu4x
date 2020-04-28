use icu_util::datap;
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use std::prelude::v1::*;

#[derive(Serialize, Deserialize)]
struct DecimalJsonSchema {
    symbols_v1: datap::decimal::Payload,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct JsonSchema {
    decimal: DecimalJsonSchema,
}
