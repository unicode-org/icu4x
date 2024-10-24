// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locale_core::{preferences::extensions::unicode::keywords, subtags::language};
use icu_provider::{DataIdentifierBorrowed, DataLocale};

#[derive(Clone, Debug)]
pub struct TimeLocaleData {
    pub data_locale: DataLocale,
    pub hour_cycle: keywords::HourCycle,
    pub time_format: &'static str,
    pub time_format_ampm: &'static str,
}

struct TimeData {
    pub und: TimeLocaleData,
    pub list: &'static [TimeLocaleData],
}

const TF_DATA: TimeData = TimeData {
    und: TimeLocaleData {
        data_locale: DataLocale::default(),
        hour_cycle: keywords::HourCycle::H23,
        time_format: "(und) [0]:00",
        time_format_ampm: "(und) [0]:00 [1]",
    },
    list: &[TimeLocaleData {
        data_locale: DataLocale::from_subtags(language!("en"), None, None, None, None),
        hour_cycle: keywords::HourCycle::H12,
        time_format: "(en) [0]:00",
        time_format_ampm: "(en) [0]:00 [1]",
    }],
};

pub fn get_payload(di: DataIdentifierBorrowed) -> TimeLocaleData {
    for item in TF_DATA.list {
        if item.data_locale.language == di.locale.language {
            return item.clone();
        }
    }
    TF_DATA.und.clone()
}
