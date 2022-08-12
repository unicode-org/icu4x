// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg(feature = "serde")]

//! This file contains the serde representaitons of the JSON files located in
//! components/datetime/tests/fixtures/tests

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fixture(pub Vec<Test>);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Test {
    pub description: Option<String>,
    pub input: TestInput,
    pub output: TestOutput,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestInput {
    pub value: String,
    pub options: TestOptions,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TestOptions {
    #[serde(rename = "length")]
    Length(icu_datetime::options::length::Bag),
    #[serde(rename = "components")]
    #[cfg(feature = "experimental")]
    Components(icu_datetime::options::components::Bag),
    #[serde(rename = "components")]
    #[cfg(not(feature = "experimental"))]
    Components(serde_json::Value),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestOutput {
    // Key is locale, and value is expected test output.
    pub values: HashMap<String, String>,
}
