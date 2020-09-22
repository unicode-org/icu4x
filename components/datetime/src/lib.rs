pub mod date;
mod error;
pub mod fields;
mod format;
pub mod options;
pub mod pattern;
mod provider;

use date::DateTime;
pub use error::DateTimeFormatError;
use format::write_pattern;
pub use format::FormattedDateTime;
use icu_data_provider::{icu_data_key, structs, DataEntry, DataProvider, DataRequest};
use icu_locale::LanguageIdentifier;
use options::DateTimeFormatOptions;
use pattern::Pattern;
use provider::DateTimeDates;
use std::borrow::Cow;

pub struct DateTimeFormat<'d> {
    _langid: LanguageIdentifier,
    pattern: Pattern,
    data: Cow<'d, structs::dates::gregory::DatesV1>,
}

impl<'d> DateTimeFormat<'d> {
    pub fn try_new<D: DataProvider<'d>>(
        langid: LanguageIdentifier,
        data_provider: &D,
        options: &DateTimeFormatOptions,
    ) -> Result<Self, DateTimeFormatError> {
        let data_key = icu_data_key!(dates: gregory@1);
        let response = data_provider
            .load(&DataRequest {
                data_key,
                data_entry: DataEntry {
                    variant: None,
                    langid: langid.clone(),
                },
            })
            .unwrap();
        let data: Cow<structs::dates::gregory::DatesV1> = response.take_payload()?;

        let pattern = data
            .get_pattern_for_options(options)?
            .ok_or(error::DateTimeFormatError::MissingData)?;

        Ok(Self {
            _langid: langid,
            pattern,
            data,
        })
    }

    pub fn format<'s>(&'s self, value: &'s DateTime) -> FormattedDateTime<'s> {
        FormattedDateTime {
            pattern: &self.pattern,
            data: &self.data,
            date_time: value,
        }
    }

    pub fn format_to_write(
        &self,
        value: &DateTime,
        w: &mut impl std::fmt::Write,
    ) -> std::fmt::Result {
        write_pattern(&self.pattern, &self.data, value, w)
    }

    pub fn format_to_string(&self, value: &DateTime) -> String {
        let mut s = String::new();
        self.format_to_write(value, &mut s)
            .expect("Failed to write to a String.");
        s
    }
}
