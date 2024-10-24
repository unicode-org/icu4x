// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locale_core::{preferences::extensions::unicode::keywords::HourCycle, Locale};

use super::{
    data_provider::{get_payload, TimeLocaleData},
    options::{TimeFormatterOptions, TimeStyle},
    preferences::TimeFormatterPreferences,
    TimeFormatter as ICU4XTimeFormatter,
};

pub struct TimeFormatter {
    dtf: ICU4XTimeFormatter,
    ecma402_resolved_options: ECMA402TimeFormatterResolvedOptions,
}

impl TimeFormatter {
    pub fn new(prefs: TimeFormatterPreferences, options: TimeFormatterOptions) -> Self {
        let di = ICU4XTimeFormatter::get_data_identifier(&prefs);
        let data = get_payload(di.as_borrowed());

        let ecma402_resolved_options =
            ECMA402TimeFormatterResolvedOptions::from(&data, &prefs, &options);

        let dtf = ICU4XTimeFormatter::new_with_data(data, prefs, options);

        Self {
            dtf,
            ecma402_resolved_options,
        }
    }

    pub fn format_to_string(&self, input: usize) -> String {
        self.dtf.format_to_string(input)
    }

    pub fn resolved_options(&self) -> &ECMA402TimeFormatterResolvedOptions {
        &self.ecma402_resolved_options
    }
}

pub struct ECMA402TimeFormatterResolvedOptions {
    pub locale: Locale,
    pub hour_cycle: HourCycle,
    pub time_style: TimeStyle,
}

impl ECMA402TimeFormatterResolvedOptions {
    pub fn from(
        data: &TimeLocaleData,
        prefs: &TimeFormatterPreferences,
        options: &TimeFormatterOptions,
    ) -> Self {
        Self {
            locale: data.data_locale.clone().into_locale(),
            hour_cycle: prefs.hour_cycle.unwrap_or(data.hour_cycle),
            time_style: options.time_style.unwrap_or_default(),
        }
    }
}
