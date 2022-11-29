// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use fixed_decimal::FixedDecimal;
use icu_decimal::{
    options::FixedDecimalFormatterOptions, provider::DecimalSymbolsV1Marker, FixedDecimalFormatter,
};
use icu_plurals::{provider::CardinalV1Marker, PluralRules};
use icu_provider::{DataLocale, DataPayload, DataProvider, DataRequest};

use crate::format::FormattedRelativeTime;
use crate::provider::{ErasedRelativeTimeFormatV1Marker, LongDayRelativeTimeFormatDataV1Marker};
use crate::{options::RelativeTimeFormatterOptions, RelativeTimeError};

/// A formatter to render locale-sensitive relative time.
pub struct RelativeTimeFormatter {
    pub(crate) plural_rules: PluralRules,
    pub(crate) rt: DataPayload<ErasedRelativeTimeFormatV1Marker>,
    pub(crate) options: RelativeTimeFormatterOptions,
    pub(crate) fixed_decimal_format: FixedDecimalFormatter,
}

macro_rules! constructor {
    ($name: ident, $marker: ty) => {
        #[doc = concat!("Create a new [`RelativeTimeFormatter`]")]
        pub fn $name<D>(
            data_provider: &D,
            locale: &DataLocale,
            options: RelativeTimeFormatterOptions,
        ) -> Result<Self, RelativeTimeError>
        where
            D: DataProvider<CardinalV1Marker>
                + DataProvider<$marker>
                + DataProvider<DecimalSymbolsV1Marker>
                + ?Sized,
        {
            let plural_rules = PluralRules::try_new_cardinal_unstable(data_provider, locale)?;
            // Initialize FixedDecimalFormatter with default options
            let fixed_decimal_format = FixedDecimalFormatter::try_new_unstable(
                data_provider,
                locale,
                FixedDecimalFormatterOptions::default(),
            )?;
            let rt: DataPayload<$marker> = data_provider
                .load(DataRequest {
                    locale,
                    metadata: Default::default(),
                })?
                .take_payload()?;
            let rt = rt.cast();
            Ok(RelativeTimeFormatter {
                plural_rules,
                options,
                rt,
                fixed_decimal_format,
            })
        }
    };
}

impl RelativeTimeFormatter {
    constructor!(
        try_new_long_second_unstable,
        LongDayRelativeTimeFormatDataV1Marker
    );

    /// Format a `value` according to the locale and formatting options of
    /// [`RelativeTimeFormatter`].
    pub fn format(&self, value: FixedDecimal) -> FormattedRelativeTime<'_> {
        FormattedRelativeTime {
            options: &self.options,
            formatter: self,
            value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use writeable::assert_writeable_eq;

    fn test_basic() {
        let rtf = RelativeTimeFormatter::try_new_long_second_unstable(
            &icu_testdata::unstable_no_fallback(),
            &icu_locid::locale!("en").into(),
            RelativeTimeFormatterOptions::default(),
        )
        .expect("Unable to construct RelativeTimeFormatter");

        assert_writeable_eq!(rtf.format(FixedDecimal::from(0i8)), "Today");
    }
}
