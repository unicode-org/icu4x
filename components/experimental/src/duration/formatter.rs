// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::format::FormattedDuration;
use super::{provider, Duration};

pub use super::validated_options::ValidatedDurationFormatterOptions;
use icu_decimal::provider::DecimalSymbolsV1Marker;
use icu_decimal::FixedDecimalFormatter;
use icu_provider::prelude::*;

/// A formatter for [`Duration`](crate::duration::Duration)s.
///
/// [`DurationFormatter`] supports:
///
/// 1. Rendering with different styles for each unit
/// 2. Digital formatting style
/// 3. Positive and negative duraitons
///
/// Read more about the options in the [`options`] module.
///
/// See the crate-level documentation for examples.
#[derive(Debug)]
pub struct DurationFormatter {
    /// Options for configuring the formatter.
    pub(crate) options: ValidatedDurationFormatterOptions,
    pub(crate) digital: DataPayload<provider::DigitalDurationDataV1Marker>,
    pub(crate) fdf: FixedDecimalFormatter,
}

impl DurationFormatter {
    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<
        D: DataProvider<provider::DigitalDurationDataV1Marker>
            + DataProvider<DecimalSymbolsV1Marker>
            + ?Sized,
    >(
        provider: &D,
        locale: &DataLocale,
        options: ValidatedDurationFormatterOptions,
    ) -> Result<Self, DataError> {
        let digital = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(locale),
                ..Default::default()
            })?
            .payload;
        Ok(Self {
            digital,
            options,
            fdf: FixedDecimalFormatter::try_new_unstable(provider, locale, Default::default())?,
        })
    }

    /// Formats a [`Duration`](crate::duration::Duration) into a [`FormattedDuration`].
    pub fn format<'l>(&'l self, duration: &'l Duration) -> FormattedDuration<'l> {
        FormattedDuration {
            fmt: self,
            duration,
        }
    }
}
