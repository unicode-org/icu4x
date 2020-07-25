use icu_data_provider::structs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct DecimalJsonSchema {
    pub(crate) symbols_v1_a: structs::decimal::SymbolsV1,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct JsonSchema {
    pub(crate) decimal: DecimalJsonSchema,
}
