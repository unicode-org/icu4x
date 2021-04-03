// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locid::Locale;
use icu_provider::DataProvider;

use crate::{
    date::ZonedDateTimeInput,
    datetime::DateTimeFormat,
    format::zoned_datetime::{self, FormattedZonedDateTime},
    options::DateTimeFormatOptions,
    provider,
    timezone::TimeZoneFormat,
    DateTimeFormatError,
};

pub struct ZonedDateTimeFormat<'d> {
    pub(super) date_time_format: DateTimeFormat<'d>,
    pub(super) time_zone_format: TimeZoneFormat<'d>,
}

impl<'d> ZonedDateTimeFormat<'d> {
    pub fn try_new<L, DP, ZP>(
        locale: L,
        date_provider: &DP,
        zone_provider: &ZP,
        options: &DateTimeFormatOptions,
    ) -> Result<Self, DateTimeFormatError>
    where
        L: Into<Locale>,
        DP: DataProvider<'d, provider::gregory::DatesV1> + ?Sized,
        ZP: DataProvider<'d, provider::timezones::TimeZoneFormatsV1<'d>>
            + DataProvider<'d, provider::timezones::ExemplarCitiesV1<'d>>
            + DataProvider<'d, provider::timezones::MetaZoneGenericNamesLongV1<'d>>
            + DataProvider<'d, provider::timezones::MetaZoneGenericNamesShortV1<'d>>
            + DataProvider<'d, provider::timezones::MetaZoneSpecificNamesLongV1<'d>>
            + DataProvider<'d, provider::timezones::MetaZoneSpecificNamesShortV1<'d>>
            + ?Sized,
    {
        let date_time_format = DateTimeFormat::try_new(locale, date_provider, options)?;
        let time_zone_format = TimeZoneFormat::try_new(
            date_time_format.locale.clone(),
            date_time_format.pattern.clone(),
            zone_provider,
        )?;

        Ok(Self {
            date_time_format,
            time_zone_format,
        })
    }

    pub fn format<'s: 'd, T>(&'s self, value: &'s T) -> FormattedZonedDateTime<'s, T>
    where
        T: ZonedDateTimeInput,
    {
        FormattedZonedDateTime {
            zoned_date_time_format: self,
            zoned_datetime: value,
        }
    }

    pub fn format_to_write(
        &self,
        w: &mut impl std::fmt::Write,
        value: &impl ZonedDateTimeInput,
    ) -> std::fmt::Result {
        zoned_datetime::write_pattern(self, value, w).map_err(|_| std::fmt::Error)
    }

    pub fn format_to_string(&self, value: &impl ZonedDateTimeInput) -> String {
        let mut s = String::new();
        self.format_to_write(&mut s, value)
            .expect("Failed to write to a String.");
        s
    }
}
