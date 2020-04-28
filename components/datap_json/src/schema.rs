use icu_util::datap;
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use std::prelude::v1::*;

#[derive(Serialize, Deserialize)]
pub(crate) struct DecimalJsonSchema {
    pub(crate) symbols_v1: datap::decimal::Payload,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct JsonSchema {
    pub(crate) decimal: DecimalJsonSchema,
}
