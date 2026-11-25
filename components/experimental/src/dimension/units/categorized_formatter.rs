// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::dimension::provider::units::display_names::UnitsDisplayNames;
use crate::dimension::units::format::FormattedUnit;
use crate::dimension::units::options::Width;
use crate::measure::category::CategorizedMeasureUnit;
use crate::measure::category::MeasureUnitCategory;
use core::marker::PhantomData;
use fixed_decimal::Decimal;
use icu_decimal::DecimalFormatter;
use icu_decimal::DecimalFormatterPreferences;
use icu_decimal::options::DecimalFormatterOptions;
use icu_locale_core::DataLocale;
use icu_locale_core::preferences::{define_preferences, prefs_convert};
use icu_plurals::PluralRules;
use icu_plurals::PluralRulesPreferences;
use icu_provider::DataError;
use icu_provider::marker::DataMarkerExt;
use icu_provider::marker::ErasedMarker;
use icu_provider::{
    DataIdentifierBorrowed, DataMarkerAttributes, DataPayload, DataProvider, DataRequest,
    DataRequestMetadata, ResultDataError,
};
use smallvec::SmallVec;

/// Type alias for the formatting info needed by the formatter constructors.
type FormattingInfo = (DataLocale, DecimalFormatter, PluralRules);

/// The display name of a unit, loaded from one of the unit names markers.
type DisplayName = DataPayload<ErasedMarker<UnitsDisplayNames<'static>>>;

define_preferences!(
    /// The preferences for units formatting.
    [Copy]
    CategorizedUnitsFormatterPreferences,
    {
        /// The user's preferred numbering system.
        ///
        /// Corresponds to the `-u-nu` in Unicode Locale Identifier.
        numbering_system: crate::dimension::preferences::NumberingSystem
    }
);
prefs_convert!(
    CategorizedUnitsFormatterPreferences,
    DecimalFormatterPreferences,
    { numbering_system }
);
prefs_convert!(CategorizedUnitsFormatterPreferences, PluralRulesPreferences);

/// A [`CategorizedFormatter`] is used to format specific units.
///
/// This is useful for type inference and for ensuring that the correct units are used.
#[derive(Debug)]
pub struct CategorizedFormatter<C: MeasureUnitCategory> {
    _category: PhantomData<C>,
    display_name: DisplayName,
    decimal_formatter: DecimalFormatter,
    plural_rules: PluralRules,
}

impl<C: MeasureUnitCategory> CategorizedFormatter<C> {
    /// Returns the data marker attributes of a unit for a width: the unit for the short width,
    /// and the unit with `/l` or `/n` after it for the long or the narrow width.
    ///
    /// The width is after the unit so that the widths of a unit share the unit in the keys
    /// of the data.
    // TODO: Remove this function once we have separate markers for different widths.
    #[inline]
    fn attribute(width: Width, unit: &str) -> SmallVec<[u8; 32]> {
        let mut buffer: SmallVec<[u8; 32]> = SmallVec::new();
        let suffix = match width {
            Width::Short => "",
            Width::Narrow => "/n",
            Width::Long => "/l",
        };
        buffer.extend_from_slice(unit.as_bytes());
        buffer.extend_from_slice(suffix.as_bytes());
        buffer
    }

    /// Loads the display name of a unit for a width.
    ///
    /// `load` loads the display name for a request from the markers of the constructor.
    ///
    /// As in CLDR, a unit has a long or a narrow name only where it differs from the short
    /// name, in the locale or in one of its parents. So if `load` finds no name for the
    /// width, the short name is the name for that width.
    ///
    /// A long or a narrow name that the data does not have is expected: the request for it
    /// is silent, and only that error leads to the short name.
    fn load_display_name(
        locale: &DataLocale,
        categorized_unit: &CategorizedMeasureUnit<C>,
        width: Width,
        load: impl Fn(DataRequest) -> Result<DisplayName, DataError>,
    ) -> Result<DisplayName, DataError> {
        let load_width = |width, silent| {
            // TODO: Remove this allocation once we have separate markers for different widths.
            let attribute = Self::attribute(width, categorized_unit.cldr_id());
            let attribute = DataMarkerAttributes::try_from_utf8(&attribute)
                .map_err(|_| DataError::custom("Failed to create a data marker"))?;
            let mut metadata = DataRequestMetadata::default();
            metadata.silent = silent;
            load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(attribute, locale),
                metadata,
            })
        };

        if width != Width::Short
            && let Some(display_name) = load_width(width, true).allow_identifier_not_found()?
        {
            return Ok(display_name);
        }
        load_width(Width::Short, false)
    }

    icu_provider::gen_buffer_data_constructors!(
        (
            prefs: CategorizedUnitsFormatterPreferences,
            categorized_unit: CategorizedMeasureUnit<C>,
            options: super::options::UnitsFormatterOptions
        ) -> error: DataError,
        functions: [
            try_new_tiny: skip,
            try_new_tiny_with_buffer_provider,
            try_new_tiny_unstable,
            Self
        ]
    );

    icu_provider::gen_buffer_data_constructors!(
        (
            prefs: CategorizedUnitsFormatterPreferences,
            categorized_unit: CategorizedMeasureUnit<C>,
            options: super::options::UnitsFormatterOptions
        ) -> error: DataError,
        functions: [
            try_new_light: skip,
            try_new_light_with_buffer_provider,
            try_new_light_unstable,
            Self
        ]
    );

    /// Extracts the formatting info.
    ///
    /// This is a helper function for the constructors to avoid writing the same code multiple times.
    #[cfg(feature = "compiled_data")]
    fn extract_formatting_info(
        prefs: CategorizedUnitsFormatterPreferences,
    ) -> Result<FormattingInfo, DataError> {
        let locale = C::DataMarkerTiny::make_locale(prefs.locale_preferences);
        let decimal_formatter: DecimalFormatter =
            DecimalFormatter::try_new((&prefs).into(), DecimalFormatterOptions::default())?;

        let plural_rules = PluralRules::try_new_cardinal((&prefs).into())?;

        Ok((locale, decimal_formatter, plural_rules))
    }

    /// Extracts the formatting info for that needed to create an unstable creator of [`CategorizedFormatter`].
    ///
    /// This is a helper function for the constructors to avoid writing the same code multiple times.
    fn extract_formatting_info_unstable<D>(
        provider: &D,
        prefs: CategorizedUnitsFormatterPreferences,
    ) -> Result<FormattingInfo, DataError>
    where
        D: ?Sized
            + DataProvider<icu_decimal::provider::DecimalSymbolsV1>
            + DataProvider<icu_decimal::provider::DecimalDigitsV1>
            + DataProvider<icu_plurals::provider::PluralsCardinalV1>,
    {
        let locale = C::DataMarkerTiny::make_locale(prefs.locale_preferences);
        let decimal_formatter: DecimalFormatter = DecimalFormatter::try_new_unstable(
            provider,
            (&prefs).into(),
            DecimalFormatterOptions::default(),
        )?;

        let plural_rules = PluralRules::try_new_cardinal_unstable(provider, (&prefs).into())?;

        Ok((locale, decimal_formatter, plural_rules))
    }

    /// Creates a new [`CategorizedFormatter`] from compiled locale data and an options bag.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn try_new_tiny(
        prefs: CategorizedUnitsFormatterPreferences,
        categorized_unit: CategorizedMeasureUnit<C>,
        options: super::options::UnitsFormatterOptions,
    ) -> Result<Self, DataError>
    where
        crate::provider::Baked: DataProvider<C::DataMarkerTiny>,
    {
        let (locale, decimal_formatter, plural_rules) = Self::extract_formatting_info(prefs)?;

        let display_name =
            Self::load_display_name(&locale, &categorized_unit, options.width, |req| {
                <crate::provider::Baked as DataProvider<C::DataMarkerTiny>>::load(
                    &crate::provider::Baked,
                    req,
                )
                .map(|r| r.payload.cast())
            })?;

        Ok(Self {
            _category: PhantomData,
            display_name,
            decimal_formatter,
            plural_rules,
        })
    }

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_tiny)]
    pub fn try_new_tiny_unstable<D>(
        provider: &D,
        prefs: CategorizedUnitsFormatterPreferences,
        categorized_unit: CategorizedMeasureUnit<C>,
        options: super::options::UnitsFormatterOptions,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<C::DataMarkerTiny>
            + DataProvider<icu_decimal::provider::DecimalSymbolsV1>
            + DataProvider<icu_decimal::provider::DecimalDigitsV1>
            + DataProvider<icu_plurals::provider::PluralsCardinalV1>,
    {
        let (locale, decimal_formatter, plural_rules) =
            Self::extract_formatting_info_unstable(provider, prefs)?;

        let display_name =
            Self::load_display_name(&locale, &categorized_unit, options.width, |req| {
                DataProvider::<C::DataMarkerTiny>::load(provider, req).map(|r| r.payload.cast())
            })?;

        Ok(Self {
            _category: PhantomData,
            display_name,
            decimal_formatter,
            plural_rules,
        })
    }

    /// Creates a new [`CategorizedFormatter`] from compiled locale data with tiny and light markers and an options bag.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn try_new_light(
        prefs: CategorizedUnitsFormatterPreferences,
        categorized_unit: CategorizedMeasureUnit<C>,
        options: super::options::UnitsFormatterOptions,
    ) -> Result<Self, DataError>
    where
        crate::provider::Baked: DataProvider<C::DataMarkerTiny>,
        crate::provider::Baked: DataProvider<C::DataMarkerLight>,
    {
        let (locale, decimal_formatter, plural_rules) = Self::extract_formatting_info(prefs)?;

        let display_name =
            Self::load_display_name(&locale, &categorized_unit, options.width, |req| {
                <crate::provider::Baked as DataProvider<C::DataMarkerTiny>>::load(
                    &crate::provider::Baked,
                    req,
                )
                .map(|r| r.payload.cast())
                .or_else(|_| {
                    <crate::provider::Baked as DataProvider<C::DataMarkerLight>>::load(
                        &crate::provider::Baked,
                        req,
                    )
                    .map(|r| r.payload.cast())
                })
            })?;

        Ok(Self {
            _category: PhantomData,
            display_name,
            decimal_formatter,
            plural_rules,
        })
    }

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_light)]
    pub fn try_new_light_unstable<D>(
        provider: &D,
        prefs: CategorizedUnitsFormatterPreferences,
        categorized_unit: CategorizedMeasureUnit<C>,
        options: super::options::UnitsFormatterOptions,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<C::DataMarkerTiny>
            + DataProvider<C::DataMarkerLight>
            + DataProvider<icu_decimal::provider::DecimalSymbolsV1>
            + DataProvider<icu_decimal::provider::DecimalDigitsV1>
            + DataProvider<icu_plurals::provider::PluralsCardinalV1>,
    {
        let (locale, decimal_formatter, plural_rules) =
            Self::extract_formatting_info_unstable(provider, prefs)?;

        // TODO(#8125): this might return the display name of a parent locale from tiny, even if the locale has its own one in light.
        let display_name =
            Self::load_display_name(&locale, &categorized_unit, options.width, |req| {
                DataProvider::<C::DataMarkerTiny>::load(provider, req)
                    .map(|r| r.payload.cast())
                    .or_else(|_| {
                        DataProvider::<C::DataMarkerLight>::load(provider, req)
                            .map(|r| r.payload.cast())
                    })
            })?;

        Ok(Self {
            _category: PhantomData,
            display_name,
            decimal_formatter,
            plural_rules,
        })
    }

    /// Creates a new [`CategorizedFormatter`] from compiled locale data with tiny, light and heavy markers and an options bag.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn try_new_heavy(
        prefs: CategorizedUnitsFormatterPreferences,
        categorized_unit: CategorizedMeasureUnit<C>,
        options: super::options::UnitsFormatterOptions,
    ) -> Result<Self, DataError>
    where
        crate::provider::Baked: DataProvider<C::DataMarkerTiny>,
        crate::provider::Baked: DataProvider<C::DataMarkerLight>,
        crate::provider::Baked: DataProvider<C::DataMarkerHeavy>,
    {
        let (locale, decimal_formatter, plural_rules) = Self::extract_formatting_info(prefs)?;

        // TODO(#8125): this might return the display name of a parent locale from tiny or light, even if the locale has its own one in heavy.
        let display_name =
            Self::load_display_name(&locale, &categorized_unit, options.width, |req| {
                <crate::provider::Baked as DataProvider<C::DataMarkerTiny>>::load(
                    &crate::provider::Baked,
                    req,
                )
                .map(|r| r.payload.cast())
                .or_else(|_| {
                    <crate::provider::Baked as DataProvider<C::DataMarkerLight>>::load(
                        &crate::provider::Baked,
                        req,
                    )
                    .map(|r| r.payload.cast())
                })
                .or_else(|_| {
                    <crate::provider::Baked as DataProvider<C::DataMarkerHeavy>>::load(
                        &crate::provider::Baked,
                        req,
                    )
                    .map(|r| r.payload.cast())
                })
            })?;

        Ok(Self {
            _category: PhantomData,
            display_name,
            decimal_formatter,
            plural_rules,
        })
    }

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_heavy)]
    pub fn try_new_heavy_unstable<D>(
        provider: &D,
        prefs: CategorizedUnitsFormatterPreferences,
        categorized_unit: CategorizedMeasureUnit<C>,
        options: super::options::UnitsFormatterOptions,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<C::DataMarkerTiny>
            + DataProvider<C::DataMarkerLight>
            + DataProvider<C::DataMarkerHeavy>
            + DataProvider<icu_decimal::provider::DecimalSymbolsV1>
            + DataProvider<icu_decimal::provider::DecimalDigitsV1>
            + DataProvider<icu_plurals::provider::PluralsCardinalV1>,
    {
        let (locale, decimal_formatter, plural_rules) =
            Self::extract_formatting_info_unstable(provider, prefs)?;

        let display_name =
            Self::load_display_name(&locale, &categorized_unit, options.width, |req| {
                DataProvider::<C::DataMarkerTiny>::load(provider, req)
                    .map(|r| r.payload.cast())
                    .or_else(|_| {
                        DataProvider::<C::DataMarkerLight>::load(provider, req)
                            .map(|r| r.payload.cast())
                    })
                    .or_else(|_| {
                        DataProvider::<C::DataMarkerHeavy>::load(provider, req)
                            .map(|r| r.payload.cast())
                    })
            })?;

        Ok(Self {
            _category: PhantomData,
            display_name,
            decimal_formatter,
            plural_rules,
        })
    }

    /// Formats a [`Decimal`] value for the given unit.
    pub fn format_fixed_decimal<'l>(&'l self, value: &'l Decimal) -> FormattedUnit<'l> {
        FormattedUnit {
            value,
            display_name: self.display_name.get(),
            decimal_formatter: &self.decimal_formatter,
            plural_rules: &self.plural_rules,
        }
    }
}

#[cfg(feature = "compiled_data")]
#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use fixed_decimal::Decimal;
    use icu_locale_core::locale;
    use writeable::assert_writeable_eq;

    use crate::dimension::units::categorized_formatter::CategorizedFormatter;
    use crate::dimension::units::options::{UnitsFormatterOptions, Width};
    use crate::measure::category::{Area, CategorizedMeasureUnit, Duration, Length};

    #[test]
    fn test_area_categorized_tiny_formatter() {
        let test_cases = vec![
            (
                locale!("en-US"),
                Area::square_meter(),
                "1000",
                UnitsFormatterOptions::default(),
                "1,000 m²",
            ),
            // TODO(#8125): `square-meter` is not a tiny unit in `en`, so this is the short name of root.
            (
                locale!("en-US"),
                Area::square_meter(),
                "1000",
                UnitsFormatterOptions {
                    width: Width::Long,
                    ..Default::default()
                },
                "1,000 m²",
            ),
            (
                locale!("fr-FR"),
                Area::square_meter(),
                "1000",
                UnitsFormatterOptions {
                    width: Width::Long,
                    ..Default::default()
                },
                "1\u{202f}000\u{a0}mètres carrés",
            ),
            (
                locale!("ar-EG"),
                Area::square_meter(),
                "1000",
                UnitsFormatterOptions {
                    width: Width::Long,
                    ..Default::default()
                },
                "١٬٠٠٠ متر مربع",
            ),
        ];

        for (locale, categorized_unit, value_str, options, expected) in test_cases {
            let formatter = CategorizedFormatter::<Area>::try_new_tiny(
                locale.clone().into(),
                categorized_unit,
                options,
            );

            let formatter = formatter.unwrap();
            let signed_decimal = Decimal::from_str(value_str).unwrap();
            let formatted = formatter.format_fixed_decimal(&signed_decimal);

            assert_writeable_eq!(formatted, expected);
        }
    }

    #[test]
    fn test_area_categorized_light_formatter() {
        let test_cases = vec![
            (
                locale!("en-US"),
                Area::square_meter(),
                "1000",
                UnitsFormatterOptions::default(),
                "1,000 m²",
            ),
            (
                locale!("en-US"),
                Area::square_meter(),
                "1000",
                UnitsFormatterOptions {
                    width: Width::Long,
                    ..Default::default()
                },
                "1,000 square meters",
            ),
            (
                locale!("fr-FR"),
                Area::square_meter(),
                "1000",
                UnitsFormatterOptions {
                    width: Width::Long,
                    ..Default::default()
                },
                "1\u{202f}000\u{a0}mètres carrés",
            ),
            (
                locale!("ar-EG"),
                Area::square_meter(),
                "1000",
                UnitsFormatterOptions {
                    width: Width::Long,
                    ..Default::default()
                },
                "١٬٠٠٠ متر مربع",
            ),
        ];

        for (locale, categorized_unit, value_str, options, expected) in test_cases {
            let formatter = CategorizedFormatter::<Area>::try_new_light(
                locale.clone().into(),
                categorized_unit,
                options,
            );
            let formatter = formatter.unwrap();
            let signed_decimal = Decimal::from_str(value_str).unwrap();
            let formatted = formatter.format_fixed_decimal(&signed_decimal);

            assert_writeable_eq!(formatted, expected);
        }
    }

    #[test]
    fn test_area_categorized_heavy_formatter() {
        let test_cases = vec![
            // TODO: Uncomment this test case once we have danum units in the Area category.
            //    (
            //     locale!("en-US"),
            //     Area::danum(),
            //     "1000",
            //     UnitsFormatterOptions::default(),
            //     "TBD",
            //    ),
            (
                locale!("en-US"),
                Area::square_meter(),
                "1000",
                UnitsFormatterOptions::default(),
                "1,000 m²",
            ),
            (
                locale!("en-US"),
                Area::square_meter(),
                "1000",
                UnitsFormatterOptions {
                    width: Width::Long,
                    ..Default::default()
                },
                "1,000 square meters",
            ),
            (
                locale!("fr-FR"),
                Area::square_meter(),
                "1000",
                UnitsFormatterOptions {
                    width: Width::Long,
                    ..Default::default()
                },
                "1\u{202f}000\u{a0}mètres carrés",
            ),
            (
                locale!("ar-EG"),
                Area::square_meter(),
                "1000",
                UnitsFormatterOptions {
                    width: Width::Long,
                    ..Default::default()
                },
                "١٬٠٠٠ متر مربع",
            ),
        ];

        for (locale, categorized_unit, value_str, options, expected) in test_cases {
            let formatter = CategorizedFormatter::<Area>::try_new_heavy(
                locale.clone().into(),
                categorized_unit,
                options,
            );
            let formatter = formatter.unwrap();
            let signed_decimal = Decimal::from_str(value_str).unwrap();
            let formatted = formatter.format_fixed_decimal(&signed_decimal);

            assert_writeable_eq!(formatted, expected);
        }
    }

    #[test]
    fn test_duration_categorized_tiny_formatter() {
        let test_cases = vec![
            (
                locale!("en-US"),
                Duration::second(),
                "1000",
                UnitsFormatterOptions::default(),
                "1,000 sec",
            ),
            (
                locale!("en-US"),
                Duration::second(),
                "1000",
                UnitsFormatterOptions {
                    width: Width::Long,
                    ..Default::default()
                },
                "1,000 seconds",
            ),
            (
                locale!("fr-FR"),
                Duration::second(),
                "1000",
                UnitsFormatterOptions {
                    width: Width::Long,
                    ..Default::default()
                },
                "1\u{202f}000\u{a0}secondes",
            ),
            (
                locale!("ar-EG"),
                Duration::second(),
                "1000",
                UnitsFormatterOptions {
                    width: Width::Long,
                    ..Default::default()
                },
                "١٬٠٠٠ ثانية",
            ),
        ];

        for (locale, categorized_unit, value_str, options, expected) in test_cases {
            let formatter = CategorizedFormatter::<Duration>::try_new_tiny(
                locale.into(),
                categorized_unit,
                options,
            )
            .unwrap();
            let signed_decimal = Decimal::from_str(value_str).unwrap();
            let formatted = formatter.format_fixed_decimal(&signed_decimal);
            assert_writeable_eq!(formatted, expected);
        }
    }

    /// The essential units are in the tiny data of every locale,
    /// also when the unit preferences of the locale's region don't use them.
    #[test]
    fn test_essential_units_tiny_formatter() {
        let value = Decimal::from_str("1000").unwrap();
        let long = UnitsFormatterOptions::from(Width::Long);

        // The unit preferences of the United States don't list `meter`.
        let formatter = CategorizedFormatter::<Length>::try_new_tiny(
            locale!("en-US").into(),
            Length::meter(),
            long,
        )
        .unwrap();
        assert_writeable_eq!(formatter.format_fixed_decimal(&value), "1,000 meters");

        // The unit preferences don't list `week`,
        // and they list `year` and `month` in the `year-duration` category.
        let test_cases = [
            (Duration::week(), "1,000 weeks"),
            (Duration::month(), "1,000 months"),
            (Duration::year(), "1,000 years"),
        ];
        for (unit, expected) in test_cases {
            let formatter =
                CategorizedFormatter::<Duration>::try_new_tiny(locale!("en-US").into(), unit, long)
                    .unwrap();
            assert_writeable_eq!(formatter.format_fixed_decimal(&value), expected);
        }

        // Every unit of the duration formatter loads from the tiny data alone.
        let duration_units: [fn() -> CategorizedMeasureUnit<Duration>; 10] = [
            Duration::year,
            Duration::month,
            Duration::week,
            Duration::day,
            Duration::hour,
            Duration::minute,
            Duration::second,
            Duration::millisecond,
            Duration::microsecond,
            Duration::nanosecond,
        ];
        for locale in [
            locale!("en-US"),
            locale!("fr-FR"),
            locale!("ar-EG"),
            locale!("ja"),
        ] {
            for width in [Width::Long, Width::Short, Width::Narrow] {
                for unit in duration_units {
                    let unit = unit();
                    let cldr_id = unit.cldr_id().to_string();
                    let formatter = CategorizedFormatter::<Duration>::try_new_tiny(
                        locale.clone().into(),
                        unit,
                        width.into(),
                    );
                    assert!(formatter.is_ok(), "{locale} {width:?} {cldr_id}");
                }
            }
        }
    }

    /// The long and the narrow names of a unit fall back to its short name, as in CLDR.
    #[test]
    fn test_long_and_narrow_fall_back_to_short() {
        let value = Decimal::from_str("1000").unwrap();

        let test_cases = [
            // In English, the long name of `day` is also its short name.
            (locale!("en-US"), Width::Long, "1,000 days"),
            (locale!("en-US"), Width::Short, "1,000 days"),
            (locale!("en-US"), Width::Narrow, "1,000d"),
            // Root has only short names.
            (locale!("und"), Width::Long, "1,000 d"),
            (locale!("und"), Width::Narrow, "1,000 d"),
            // The narrow name of `day` in `es-CO` is also its short name.
            // Its parent `es-419` has another narrow name, which `es-CO` must not take.
            (locale!("es-419"), Width::Narrow, "1,000dd."),
            (locale!("es-CO"), Width::Short, "1.000 días"),
            (locale!("es-CO"), Width::Narrow, "1.000 días"),
        ];

        for (locale, width, expected) in test_cases {
            let formatter = CategorizedFormatter::<Duration>::try_new_tiny(
                locale.into(),
                Duration::day(),
                width.into(),
            )
            .unwrap();
            assert_writeable_eq!(formatter.format_fixed_decimal(&value), expected);
        }
    }

    /// Only a long or a narrow name that the data does not have leads to the short name.
    /// The request for it is silent, so a name that is missing as expected is not logged.
    #[test]
    fn test_only_a_missing_name_falls_back_to_short() {
        use crate::dimension::provider::units::categorized_display_names::UnitsNamesDurationTinyV1;
        use core::cell::RefCell;
        use icu_locale_core::DataLocale;
        use icu_provider::{DataError, DataErrorKind, DataMarker};

        let locale = DataLocale::from(locale!("en"));
        let unit = Duration::day();

        // The data has no name for any width.
        let requests = RefCell::new(Vec::new());
        let result = CategorizedFormatter::<Duration>::load_display_name(
            &locale,
            &unit,
            Width::Long,
            |req| {
                requests.borrow_mut().push((
                    req.id.marker_attributes.as_str().to_string(),
                    req.metadata.silent,
                ));
                Err(DataErrorKind::IdentifierNotFound.with_req(UnitsNamesDurationTinyV1::INFO, req))
            },
        );
        assert_eq!(result.unwrap_err().kind, DataErrorKind::IdentifierNotFound);
        assert_eq!(
            requests.into_inner(),
            [("day/l".to_string(), true), ("day".to_string(), false)]
        );

        // Another error is returned, and the short name is not loaded.
        let requests = RefCell::new(0);
        let result = CategorizedFormatter::<Duration>::load_display_name(
            &locale,
            &unit,
            Width::Long,
            |_| {
                *requests.borrow_mut() += 1;
                Err(DataError::custom("another error"))
            },
        );
        assert_eq!(result.unwrap_err().kind, DataErrorKind::Custom);
        assert_eq!(requests.into_inner(), 1);
    }
}
