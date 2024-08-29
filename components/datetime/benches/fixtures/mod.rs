// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datetime::options::DateTimeFormatterOptions;
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
pub struct TestOptions {
    pub length: Option<icu_datetime::options::length::Bag>,
    #[cfg(feature = "experimental")]
    pub components: Option<icu_datetime::options::components::Bag>,
    #[cfg(feature = "experimental")]
    pub semantic: Option<icu_datetime::neo_skeleton::NeoSkeleton>,
    #[cfg(feature = "experimental")]
    pub preferences: Option<icu_datetime::options::preferences::Bag>,
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

#[allow(dead_code)]
pub fn get_options(input: &TestOptions) -> Option<DateTimeFormatterOptions> {
    if let Some(bag) = input.length {
        return Some(bag.into());
    }
    #[cfg(feature = "experimental")]
    if let Some(bag) = input.components {
        return Some(bag.into());
    }
    None
}
