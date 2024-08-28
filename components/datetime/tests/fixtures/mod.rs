// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg(feature = "serde")]

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
    pub values: HashMap<String, TestOutputItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TestOutputItem {
    ExpectedString(String),
    ExpectedStringAndPattern { formatted: String, pattern: String },
}

impl TestOutputItem {
    pub fn expectation(&self) -> &str {
        match self {
            Self::ExpectedString(s) => s,
            Self::ExpectedStringAndPattern { formatted, .. } => formatted,
        }
    }

    pub fn pattern(&self) -> Option<&str> {
        match self {
            Self::ExpectedString(_) => None,
            Self::ExpectedStringAndPattern { pattern, .. } => Some(pattern),
        }
    }
}
