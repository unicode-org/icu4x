// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::dimension::provider::units::UnitsDisplayNameV1Marker;
use crate::dimension::units::formatter::UnitsFormatter;
use crate::dimension::units::options::{UnitsFormatterOptions, Width};
use crate::duration::options::FieldStyle;

use super::format::FormattedDuration;
use super::options::BaseStyle;
use super::validated_options::Unit;
use super::{provider, Duration};

pub use super::validated_options::ValidatedDurationFormatterOptions;
use icu_decimal::provider::DecimalSymbolsV1Marker;
use icu_decimal::FixedDecimalFormatter;
use icu_list::{ListFormatter, ListLength};
use icu_provider::prelude::*;

/// A formatter for [`Duration`](crate::duration::Duration)s.
///
/// [`DurationFormatter`] supports:
///
/// 1. Rendering with different styles for each unit
/// 2. Digital formatting style
/// 3. Positive and negative duraitons
///
/// Read more about the options in the [`options`](super::options) module.
///
/// See the crate-level documentation for examples.
pub struct DurationFormatter {
    /// Options for configuring the formatter.
    pub(crate) options: ValidatedDurationFormatterOptions,
    pub(crate) digital: DataPayload<provider::DigitalDurationDataV1Marker>,
    pub(crate) unit: DurationUnitFormatter,
    pub(crate) list: ListFormatter,
    pub(crate) fdf: FixedDecimalFormatter,
}

pub(crate) struct DurationUnitFormatter {
    pub(crate) year: UnitsFormatter,
    pub(crate) month: UnitsFormatter,
    pub(crate) week: UnitsFormatter,
    pub(crate) day: UnitsFormatter,
    pub(crate) hour: UnitsFormatter,
    pub(crate) minute: UnitsFormatter,
    pub(crate) second: UnitsFormatter,
    pub(crate) millisecond: UnitsFormatter,
    pub(crate) microsecond: UnitsFormatter,
    pub(crate) nanosecond: UnitsFormatter,
}

impl core::ops::Index<Unit> for DurationUnitFormatter {
    type Output = UnitsFormatter;

    fn index(&self, index: Unit) -> &Self::Output {
        match index {
            Unit::Year => &self.year,
            Unit::Month => &self.month,
            Unit::Week => &self.week,
            Unit::Day => &self.day,
            Unit::Hour => &self.hour,
            Unit::Minute => &self.minute,
            Unit::Second => &self.second,
            Unit::Millisecond => &self.millisecond,
            Unit::Microsecond => &self.microsecond,
            Unit::Nanosecond => &self.nanosecond,
        }
    }
}

impl DurationUnitFormatter {
    fn field_style_to_unit_width(style: FieldStyle, base: BaseStyle) -> Width {
        match style {
            FieldStyle::Long => Width::Long,
            FieldStyle::Short => Width::Short,
            FieldStyle::Narrow => Width::Narrow,
            _ => match base {
                BaseStyle::Long => Width::Long,
                BaseStyle::Short | BaseStyle::Digital => Width::Short,
                BaseStyle::Narrow => Width::Narrow,
            },
        }
    }

    #[cfg(feature = "compiled_data")]
    fn try_new(
        locale: &DataLocale,
        options: ValidatedDurationFormatterOptions,
    ) -> Result<Self, DataError> {
        let get_unit_formatter = |unit: Unit, style| {
            let w = DurationUnitFormatter::field_style_to_unit_width(style, options.base);
            let options = UnitsFormatterOptions { width: w };

            UnitsFormatter::try_new(locale, unit.as_unit_formatter_name(), options)
        };

        Ok(DurationUnitFormatter {
            year: get_unit_formatter(Unit::Year, options.year)?,
            month: get_unit_formatter(Unit::Month, options.month)?,
            week: get_unit_formatter(Unit::Week, options.week)?,
            day: get_unit_formatter(Unit::Day, options.day)?,
            hour: get_unit_formatter(Unit::Hour, options.hour)?,
            minute: get_unit_formatter(Unit::Minute, options.minute)?,
            second: get_unit_formatter(Unit::Second, options.second)?,
            millisecond: get_unit_formatter(Unit::Millisecond, options.millisecond)?,
            microsecond: get_unit_formatter(Unit::Microsecond, options.microsecond)?,
            nanosecond: get_unit_formatter(Unit::Nanosecond, options.nanosecond)?,
        })
    }

    fn try_new_unstable<
        D: ?Sized
            + DataProvider<UnitsDisplayNameV1Marker>
            + DataProvider<icu_decimal::provider::DecimalSymbolsV1Marker>
            + DataProvider<icu_plurals::provider::CardinalV1Marker>,
    >(
        provider: &D,
        locale: &DataLocale,
        options: ValidatedDurationFormatterOptions,
    ) -> Result<Self, DataError> {
        let get_unit_formatter = |unit: Unit, style| {
            let w = DurationUnitFormatter::field_style_to_unit_width(style, options.base);
            let options = UnitsFormatterOptions { width: w };

            UnitsFormatter::try_new_unstable(
                provider,
                locale,
                unit.as_unit_formatter_name(),
                options,
            )
        };

        Ok(DurationUnitFormatter {
            year: get_unit_formatter(Unit::Year, options.year)?,
            month: get_unit_formatter(Unit::Month, options.month)?,
            week: get_unit_formatter(Unit::Week, options.week)?,
            day: get_unit_formatter(Unit::Day, options.day)?,
            hour: get_unit_formatter(Unit::Hour, options.hour)?,
            minute: get_unit_formatter(Unit::Minute, options.minute)?,
            second: get_unit_formatter(Unit::Second, options.second)?,
            millisecond: get_unit_formatter(Unit::Millisecond, options.millisecond)?,
            microsecond: get_unit_formatter(Unit::Microsecond, options.microsecond)?,
            nanosecond: get_unit_formatter(Unit::Nanosecond, options.nanosecond)?,
        })
    }
}

impl From<BaseStyle> for icu_list::ListLength {
    fn from(style: BaseStyle) -> Self {
        // Section 1.1.13
        // 1. Let lfOpts be OrdinaryObjectCreate(null).
        // 2. Perform ! CreateDataPropertyOrThrow(lfOpts, "type", "unit").
        // 3. Let listStyle be durationFormat.[[Style]].
        // 4. If listStyle is "digital", then
        //     a. Set listStyle to "short".
        // 5. Perform ! CreateDataPropertyOrThrow(lfOpts, "style", listStyle).
        // 6. Let lf be ! Construct(%ListFormat%, « durationFormat.[[Locale]], lfOpts »).
        match style {
            BaseStyle::Long => ListLength::Wide,
            BaseStyle::Short | BaseStyle::Digital => ListLength::Short,
            BaseStyle::Narrow => ListLength::Narrow,
        }
    }
}

impl DurationFormatter {
    icu_provider::gen_any_buffer_data_constructors!(
        (locale, options: ValidatedDurationFormatterOptions) -> error: DataError,
        functions: [
            try_new: skip,
            try_new_with_any_provider,
            try_new_with_buffer_provider,
            try_new_unstable,
            Self
        ]
    );

    /// Creates a new [`DurationFormatter`] from compiled locale data and an options bag.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn try_new(
        locale: &DataLocale,
        options: ValidatedDurationFormatterOptions,
    ) -> Result<Self, DataError> {
        let digital = crate::provider::Baked
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(locale),
                ..Default::default()
            })?
            .payload;

        Ok(Self {
            digital,
            options,
            unit: DurationUnitFormatter::try_new(locale, options)?,
            list: ListFormatter::try_new_unit_with_length(locale, options.base.into())?,
            fdf: FixedDecimalFormatter::try_new(locale, Default::default())?,
        })
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<
        D: DataProvider<provider::DigitalDurationDataV1Marker>
            + DataProvider<UnitsDisplayNameV1Marker>
            + DataProvider<DecimalSymbolsV1Marker>
            + DataProvider<icu_plurals::provider::CardinalV1Marker>
            + DataProvider<icu_list::provider::UnitListV2Marker>
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
            unit: DurationUnitFormatter::try_new_unstable(provider, locale, options)?,
            list: ListFormatter::try_new_unit_with_length_unstable(
                provider,
                locale,
                options.base.into(),
            )?,
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
