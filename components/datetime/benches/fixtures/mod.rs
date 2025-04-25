// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datetime::{fieldsets::builder::FieldSetBuilder, provider::fields::components};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct Fixture {
    pub(crate) setups: Vec<TestInput>,
    pub(crate) values: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct TestInput {
    pub(crate) locale: String,
    pub(crate) options: TestOptions,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct TestOptions {
    pub(crate) length: Option<TestOptionsLength>,
    pub(crate) components: Option<TestComponentsBag>,
    pub(crate) semantic: Option<FieldSetBuilder>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct TestOptionsLength {
    pub(crate) date: Option<TestLength>,
    pub(crate) time: Option<TestLength>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum TestLength {
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
pub(crate) struct TestComponentsBag {
    pub(crate) era: Option<components::Text>,
    pub(crate) year: Option<components::Year>,
    pub(crate) month: Option<components::Month>,
    pub(crate) week: Option<components::Week>,
    pub(crate) day: Option<components::Day>,
    pub(crate) weekday: Option<components::Text>,

    pub(crate) hour: Option<components::Numeric>,
    pub(crate) minute: Option<components::Numeric>,
    pub(crate) second: Option<components::Numeric>,
    pub(crate) subsecond: Option<u8>,

    pub(crate) time_zone_name: Option<components::TimeZoneName>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct PatternsFixture(pub(crate) Vec<String>);
