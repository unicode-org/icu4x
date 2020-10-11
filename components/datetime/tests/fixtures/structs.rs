// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fixture(pub Vec<Test>);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Test {
    pub input: TestInput,
    pub output: TestOutput,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestInput {
    pub locale: String,
    pub value: String,
    pub options: TestOptions,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestOptions {
    pub style: TestOptionsStyle,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestOptionsStyle {
    pub date: Option<TestStyleWidth>,
    pub time: Option<TestStyleWidth>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TestStyleWidth {
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
pub struct TestOutput {
    pub value: String,
}
