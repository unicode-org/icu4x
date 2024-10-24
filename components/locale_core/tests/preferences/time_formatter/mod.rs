// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod data_provider;
pub mod ecma402;
pub mod options;
mod preferences;

use data_provider::{get_payload, TimeLocaleData};
use icu_locale_core::preferences::extensions::unicode::keywords::HourCycle;
use icu_provider::{DataIdentifierCow, DataLocale};
pub use options::TimeFormatterOptions;
pub use preferences::TimeFormatterPreferences;

pub struct TimeFormatter {
    data: TimeLocaleData,
    raw_options: TFRawOptions,
}

impl TimeFormatter {
    pub fn new(prefs: TimeFormatterPreferences, options: TimeFormatterOptions) -> Self {
        let di = Self::get_data_identifier(&prefs);
        let data = get_payload(di.as_borrowed());

        Self::new_with_data(data, prefs, options)
    }

    pub fn new_with_data(
        data: TimeLocaleData,
        prefs: TimeFormatterPreferences,
        options: TimeFormatterOptions,
    ) -> Self {
        let raw_options = TFRawOptions::from(&prefs, &options);
        Self { data, raw_options }
    }

    pub fn format_to_string(&self, input: usize) -> String {
        use icu_locale_core::preferences::extensions::unicode::keywords::HourCycle;

        let hour_cycle = self.raw_options.hour_cycle.unwrap_or(self.data.hour_cycle);
        match hour_cycle {
            HourCycle::H11 | HourCycle::H12 => self
                .data
                .time_format_ampm
                .replace("[0]", &input.to_string())
                .replace("[1]", "am"),
            _ => self.data.time_format.replace("[0]", &input.to_string()),
        }
    }
    pub(crate) fn get_data_identifier(prefs: &TimeFormatterPreferences) -> DataIdentifierCow {
        let locale = DataLocale::from_subtags(
            prefs.language,
            prefs.script,
            prefs.region,
            prefs.variant,
            prefs.subdivision,
        );
        DataIdentifierCow::from_locale(locale)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TFRawOptions {
    pub(crate) hour_cycle: Option<HourCycle>,
    pub(crate) time_style: Option<options::TimeStyle>,
}

impl TFRawOptions {
    pub fn from(prefs: &TimeFormatterPreferences, options: &TimeFormatterOptions) -> Self {
        Self {
            hour_cycle: prefs.hour_cycle,
            time_style: options.time_style,
        }
    }
}
