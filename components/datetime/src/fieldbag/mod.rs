// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Field-specific formatting options for better interop with ECMA-402 and ICU4C.

use icu_locale_core::preferences::extensions::unicode::keywords::HourCycle;

use crate::{DateTimeFormatterPreferences, fieldsets::builder::FieldSetBuilder};

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub struct DateTimeFieldBag {}

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub struct DateTimeFieldPreferences {
    hour_cycle: Option<HourCycle>,
}

#[derive(Debug, Copy, Clone)]
#[allow(clippy::exhaustive_structs)]
pub struct DateTimeFieldBagWithPreferences {
    pub bag: DateTimeFieldBag,
    pub preferences: DateTimeFieldPreferences,
}

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum DateTimeFieldBagParseError {}

impl DateTimeFieldBagWithPreferences {
    pub fn try_from_skeleton(
        skeleton: &str,
    ) -> Result<DateTimeFieldBagWithPreferences, DateTimeFieldBagParseError> {
        todo!()
    }
}

impl FieldSetBuilder {
    pub fn from_field_bag(bag: &DateTimeFieldBag) -> Self {
        todo!()
    }
}

impl DateTimeFormatterPreferences {
    pub fn merge_field_preferences(self, field_preferences: DateTimeFieldPreferences) -> Self {
        todo!()
    }
}
