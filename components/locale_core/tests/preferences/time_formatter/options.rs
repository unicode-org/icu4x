// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locale_core::preferences::options;

options!(
    TimeFormatterOptions,
    {
        time_style => TimeStyle
    }
);

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub enum TimeStyle {
    Short,
    #[default]
    Medium,
}
