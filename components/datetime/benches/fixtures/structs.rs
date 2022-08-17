// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datetime::options;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fixture(pub Vec<Test>);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Test {
    pub setups: Vec<TestInput>,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestInput {
    pub locale: String,
    pub options: TestOptions,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TestOptions {
    #[serde(rename = "length")]
    Length(options::length::Bag),
    #[serde(rename = "components")]
    #[cfg(feature = "experimental")]
    Components(options::components::Bag),
    #[serde(rename = "components")]
    #[cfg(not(feature = "experimental"))]
    Components(serde_json::Value),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestOptionsLength {
    pub date: Option<TestLength>,
    pub time: Option<TestLength>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TestLength {
    #[serde(rename = "short")]
    Short,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "long")]
    Long,
    #[serde(rename = "full")]
    Full,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatternsFixture(pub Vec<String>);
