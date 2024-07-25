// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg(feature = "serde")]

use icu_datetime::DateTimeFormatterOptions;
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
pub struct TestOutput {
    // Key is locale, and value is expected test output.
    pub values: HashMap<String, String>,
}

pub fn get_options(input: &TestOptions) -> Option<DateTimeFormatterOptions> {
    if let Some(bag) = input.length {
        return Some(bag.into());
    }
    #[cfg(feature = "experimental")]
    if let Some(mut bag) = input.components {
        bag.preferences = input.preferences;
        return Some(bag.into());
    }
    None
}
