// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::dimension::provider::units::display_names::UnitsDisplayNames;
use crate::dimension::units::format::FormattedUnit;
use crate::measure::category::CategorizedMeasureUnit;
use crate::measure::category::MeasureUnitCategory;
use core::marker::PhantomData;
use fixed_decimal::Decimal;
use icu_decimal::DecimalFormatter;
use icu_decimal::DecimalFormatterPreferences;
use icu_decimal::options::DecimalFormatterOptions;
use icu_locale_core::preferences::{define_preferences, prefs_convert};
use icu_plurals::PluralRules;
use icu_plurals::PluralRulesPreferences;
use icu_provider::DataError;
use icu_provider::marker::DataMarkerExt;
use icu_provider::marker::ErasedMarker;
use icu_provider::{
    DataIdentifierBorrowed, DataMarker, DataMarkerAttributes, DataPayload, DataProvider,
    DataRequest,
};
use smallvec::SmallVec;

/// The names of one unit in one width, loaded from one of the unit names markers.
type UnitNames = DataPayload<ErasedMarker<UnitsDisplayNames<'static>>>;

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
///
/// There is one constructor for each width (long, short, narrow) and each tier of units
/// (tiny, light, heavy), such as [`Self::try_new_long_tiny`]. The long constructors need
/// only the long data. The short and the narrow constructors share their data, because
/// the short and the narrow names of a unit are often equal.
#[derive(Debug)]
pub struct CategorizedFormatter<C: MeasureUnitCategory> {
    _category: PhantomData<C>,
    display_name: UnitNames,
    decimal_formatter: DecimalFormatter,
    plural_rules: PluralRules,
}

impl<C: MeasureUnitCategory> CategorizedFormatter<C> {
    /// Loads the names of the unit from the marker `M`.
    ///
    /// The key of the unit is `key_prefix` and then the unit. The prefix is empty for the
    /// long markers, which hold one width. It is the width (`short-` or `narrow-`) for
    /// the markers that hold the short and the narrow names.
    fn load_names<M, D>(
        provider: &D,
        prefs: CategorizedUnitsFormatterPreferences,
        categorized_unit: &CategorizedMeasureUnit<C>,
        key_prefix: &str,
    ) -> Result<UnitNames, DataError>
    where
        M: DataMarker<DataStruct = UnitsDisplayNames<'static>>,
        D: ?Sized + DataProvider<M>,
    {
        let locale = M::make_locale(prefs.locale_preferences);
        let mut key: SmallVec<[u8; 32]> = SmallVec::new();
        key.extend_from_slice(key_prefix.as_bytes());
        key.extend_from_slice(categorized_unit.cldr_id().as_bytes());
        let key = DataMarkerAttributes::try_from_utf8(&key)
            .map_err(|_| DataError::custom("Failed to create a data marker"))?;

        let response = DataProvider::<M>::load(
            provider,
            DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(key, &locale),
                ..Default::default()
            },
        )?;

        Ok(response.payload.cast())
    }

    /// Creates the formatter from the names of the unit, with compiled data for the rest.
    #[cfg(feature = "compiled_data")]
    fn try_new_with_names(
        prefs: CategorizedUnitsFormatterPreferences,
        display_name: UnitNames,
    ) -> Result<Self, DataError> {
        Ok(Self {
            _category: PhantomData,
            display_name,
            decimal_formatter: DecimalFormatter::try_new(
                (&prefs).into(),
                DecimalFormatterOptions::default(),
            )?,
            plural_rules: PluralRules::try_new_cardinal((&prefs).into())?,
        })
    }

    /// Creates the formatter from the names of the unit, and loads the rest from the provider.
    fn try_new_with_names_unstable<D>(
        provider: &D,
        prefs: CategorizedUnitsFormatterPreferences,
        display_name: UnitNames,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<icu_decimal::provider::DecimalSymbolsV1>
            + DataProvider<icu_decimal::provider::DecimalDigitsV1>
            + DataProvider<icu_plurals::provider::PluralsCardinalV1>,
    {
        Ok(Self {
            _category: PhantomData,
            display_name,
            decimal_formatter: DecimalFormatter::try_new_unstable(
                provider,
                (&prefs).into(),
                DecimalFormatterOptions::default(),
            )?,
            plural_rules: PluralRules::try_new_cardinal_unstable(provider, (&prefs).into())?,
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

/// Implements the constructors of one width.
///
/// Each tier has a constructor with compiled data and one with an unstable provider.
/// The tiny and light tiers also have one with a buffer provider.
///
/// `key_prefix` is what comes before the unit in the key of the markers of this width.
macro_rules! impl_constructors_for_width {
    (
        $width:literal,
        key_prefix: $key_prefix:literal,
        markers: [$tiny:ident, $light:ident, $heavy:ident],
        tiny: [$new_tiny:ident, $new_tiny_buffer:ident, $new_tiny_unstable:ident],
        light: [$new_light:ident, $new_light_buffer:ident, $new_light_unstable:ident],
        heavy: [$new_heavy:ident, $new_heavy_unstable:ident] $(,)?
    ) => {
        impl<C: MeasureUnitCategory> CategorizedFormatter<C> {
            icu_provider::gen_buffer_data_constructors!(
                (
                    prefs: CategorizedUnitsFormatterPreferences,
                    categorized_unit: CategorizedMeasureUnit<C>
                ) -> error: DataError,
                functions: [
                    $new_tiny: skip,
                    $new_tiny_buffer,
                    $new_tiny_unstable,
                    Self
                ]
            );

            icu_provider::gen_buffer_data_constructors!(
                (
                    prefs: CategorizedUnitsFormatterPreferences,
                    categorized_unit: CategorizedMeasureUnit<C>
                ) -> error: DataError,
                functions: [
                    $new_light: skip,
                    $new_light_buffer,
                    $new_light_unstable,
                    Self
                ]
            );

            #[doc = concat!(
                "Creates a new [`CategorizedFormatter`] with the ", $width,
                " width from compiled locale data with the tiny markers.",
            )]
            ///
            /// ✨ *Enabled with the `compiled_data` Cargo feature.*
            ///
            /// [📚 Help choosing a constructor](icu_provider::constructors)
            #[cfg(feature = "compiled_data")]
            pub fn $new_tiny(
                prefs: CategorizedUnitsFormatterPreferences,
                categorized_unit: CategorizedMeasureUnit<C>,
            ) -> Result<Self, DataError>
            where
                crate::provider::Baked: DataProvider<C::$tiny>,
            {
                let display_name = Self::load_names::<C::$tiny, _>(
                    &crate::provider::Baked,
                    prefs,
                    &categorized_unit,
                    $key_prefix,
                )?;

                Self::try_new_with_names(prefs, display_name)
            }

            #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::$new_tiny)]
            pub fn $new_tiny_unstable<D>(
                provider: &D,
                prefs: CategorizedUnitsFormatterPreferences,
                categorized_unit: CategorizedMeasureUnit<C>,
            ) -> Result<Self, DataError>
            where
                D: ?Sized
                    + DataProvider<C::$tiny>
                    + DataProvider<icu_decimal::provider::DecimalSymbolsV1>
                    + DataProvider<icu_decimal::provider::DecimalDigitsV1>
                    + DataProvider<icu_plurals::provider::PluralsCardinalV1>,
            {
                let display_name =
                    Self::load_names::<C::$tiny, _>(provider, prefs, &categorized_unit, $key_prefix)?;

                Self::try_new_with_names_unstable(provider, prefs, display_name)
            }

            #[doc = concat!(
                "Creates a new [`CategorizedFormatter`] with the ", $width,
                " width from compiled locale data with the tiny and light markers.",
            )]
            ///
            /// ✨ *Enabled with the `compiled_data` Cargo feature.*
            ///
            /// [📚 Help choosing a constructor](icu_provider::constructors)
            #[cfg(feature = "compiled_data")]
            pub fn $new_light(
                prefs: CategorizedUnitsFormatterPreferences,
                categorized_unit: CategorizedMeasureUnit<C>,
            ) -> Result<Self, DataError>
            where
                crate::provider::Baked: DataProvider<C::$tiny>,
                crate::provider::Baked: DataProvider<C::$light>,
            {
                let provider = &crate::provider::Baked;

                let display_name =
                    Self::load_names::<C::$tiny, _>(provider, prefs, &categorized_unit, $key_prefix).or_else(
                        |_| Self::load_names::<C::$light, _>(provider, prefs, &categorized_unit, $key_prefix),
                    )?;

                Self::try_new_with_names(prefs, display_name)
            }

            #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::$new_light)]
            pub fn $new_light_unstable<D>(
                provider: &D,
                prefs: CategorizedUnitsFormatterPreferences,
                categorized_unit: CategorizedMeasureUnit<C>,
            ) -> Result<Self, DataError>
            where
                D: ?Sized
                    + DataProvider<C::$tiny>
                    + DataProvider<C::$light>
                    + DataProvider<icu_decimal::provider::DecimalSymbolsV1>
                    + DataProvider<icu_decimal::provider::DecimalDigitsV1>
                    + DataProvider<icu_plurals::provider::PluralsCardinalV1>,
            {
                // TODO(#8125): this might return the root locale's name from tiny, even if the locale has its own name in light.
                let display_name =
                    Self::load_names::<C::$tiny, _>(provider, prefs, &categorized_unit, $key_prefix).or_else(
                        |_| Self::load_names::<C::$light, _>(provider, prefs, &categorized_unit, $key_prefix),
                    )?;

                Self::try_new_with_names_unstable(provider, prefs, display_name)
            }

            #[doc = concat!(
                "Creates a new [`CategorizedFormatter`] with the ", $width,
                " width from compiled locale data with the tiny, light and heavy markers.",
            )]
            ///
            /// ✨ *Enabled with the `compiled_data` Cargo feature.*
            ///
            /// [📚 Help choosing a constructor](icu_provider::constructors)
            #[cfg(feature = "compiled_data")]
            pub fn $new_heavy(
                prefs: CategorizedUnitsFormatterPreferences,
                categorized_unit: CategorizedMeasureUnit<C>,
            ) -> Result<Self, DataError>
            where
                crate::provider::Baked: DataProvider<C::$tiny>,
                crate::provider::Baked: DataProvider<C::$light>,
                crate::provider::Baked: DataProvider<C::$heavy>,
            {
                let provider = &crate::provider::Baked;

                // TODO(#8125): this might return the root locale's name from tiny or light, even if the locale has its own name in heavy.
                let display_name =
                    Self::load_names::<C::$tiny, _>(provider, prefs, &categorized_unit, $key_prefix)
                        .or_else(|_| {
                            Self::load_names::<C::$light, _>(provider, prefs, &categorized_unit, $key_prefix)
                        })
                        .or_else(|_| {
                            Self::load_names::<C::$heavy, _>(provider, prefs, &categorized_unit, $key_prefix)
                        })?;

                Self::try_new_with_names(prefs, display_name)
            }

            #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::$new_heavy)]
            pub fn $new_heavy_unstable<D>(
                provider: &D,
                prefs: CategorizedUnitsFormatterPreferences,
                categorized_unit: CategorizedMeasureUnit<C>,
            ) -> Result<Self, DataError>
            where
                D: ?Sized
                    + DataProvider<C::$tiny>
                    + DataProvider<C::$light>
                    + DataProvider<C::$heavy>
                    + DataProvider<icu_decimal::provider::DecimalSymbolsV1>
                    + DataProvider<icu_decimal::provider::DecimalDigitsV1>
                    + DataProvider<icu_plurals::provider::PluralsCardinalV1>,
            {
                let display_name =
                    Self::load_names::<C::$tiny, _>(provider, prefs, &categorized_unit, $key_prefix)
                        .or_else(|_| {
                            Self::load_names::<C::$light, _>(provider, prefs, &categorized_unit, $key_prefix)
                        })
                        .or_else(|_| {
                            Self::load_names::<C::$heavy, _>(provider, prefs, &categorized_unit, $key_prefix)
                        })?;

                Self::try_new_with_names_unstable(provider, prefs, display_name)
            }
        }
    };
}

impl_constructors_for_width!(
    "long",
    key_prefix: "",
    markers: [DataMarkerLongTiny, DataMarkerLongLight, DataMarkerLongHeavy],
    tiny: [
        try_new_long_tiny,
        try_new_long_tiny_with_buffer_provider,
        try_new_long_tiny_unstable
    ],
    light: [
        try_new_long_light,
        try_new_long_light_with_buffer_provider,
        try_new_long_light_unstable
    ],
    heavy: [try_new_long_heavy, try_new_long_heavy_unstable],
);

impl_constructors_for_width!(
    "short",
    key_prefix: "short-",
    markers: [
        DataMarkerShortNarrowTiny,
        DataMarkerShortNarrowLight,
        DataMarkerShortNarrowHeavy
    ],
    tiny: [
        try_new_short_tiny,
        try_new_short_tiny_with_buffer_provider,
        try_new_short_tiny_unstable
    ],
    light: [
        try_new_short_light,
        try_new_short_light_with_buffer_provider,
        try_new_short_light_unstable
    ],
    heavy: [try_new_short_heavy, try_new_short_heavy_unstable],
);

impl_constructors_for_width!(
    "narrow",
    key_prefix: "narrow-",
    markers: [
        DataMarkerShortNarrowTiny,
        DataMarkerShortNarrowLight,
        DataMarkerShortNarrowHeavy
    ],
    tiny: [
        try_new_narrow_tiny,
        try_new_narrow_tiny_with_buffer_provider,
        try_new_narrow_tiny_unstable
    ],
    light: [
        try_new_narrow_light,
        try_new_narrow_light_with_buffer_provider,
        try_new_narrow_light_unstable
    ],
    heavy: [try_new_narrow_heavy, try_new_narrow_heavy_unstable],
);

#[cfg(feature = "compiled_data")]
#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use fixed_decimal::Decimal;
    use icu_locale_core::locale;
    use icu_provider::DataError;
    use writeable::assert_writeable_eq;

    use crate::dimension::units::categorized_formatter::{
        CategorizedFormatter, CategorizedUnitsFormatterPreferences,
    };
    use crate::measure::category::{Area, CategorizedMeasureUnit, Duration, Length};

    /// A constructor of [`CategorizedFormatter`] with compiled data.
    type Constructor<C> = fn(
        CategorizedUnitsFormatterPreferences,
        CategorizedMeasureUnit<C>,
    ) -> Result<CategorizedFormatter<C>, DataError>;

    const AREA_SHORT_TINY: Constructor<Area> = CategorizedFormatter::<Area>::try_new_short_tiny;
    const AREA_LONG_TINY: Constructor<Area> = CategorizedFormatter::<Area>::try_new_long_tiny;
    const AREA_SHORT_LIGHT: Constructor<Area> = CategorizedFormatter::<Area>::try_new_short_light;
    const AREA_LONG_LIGHT: Constructor<Area> = CategorizedFormatter::<Area>::try_new_long_light;
    const AREA_SHORT_HEAVY: Constructor<Area> = CategorizedFormatter::<Area>::try_new_short_heavy;
    const AREA_LONG_HEAVY: Constructor<Area> = CategorizedFormatter::<Area>::try_new_long_heavy;

    #[test]
    fn test_area_categorized_tiny_formatter() {
        let test_cases = vec![
            (
                locale!("en-US"),
                Area::square_meter(),
                "1000",
                AREA_SHORT_TINY,
                "1,000 m²",
            ),
            (
                locale!("en-US"),
                Area::square_meter(),
                "1000",
                AREA_LONG_TINY,
                "1,000 m²",
            ),
            (
                locale!("fr-FR"),
                Area::square_meter(),
                "1000",
                AREA_LONG_TINY,
                "1\u{202f}000\u{a0}mètres carrés",
            ),
            (
                locale!("ar-EG"),
                Area::square_meter(),
                "1000",
                AREA_LONG_TINY,
                "١٬٠٠٠ متر مربع",
            ),
        ];

        for (locale, categorized_unit, value_str, constructor, expected) in test_cases {
            let formatter = constructor(locale.clone().into(), categorized_unit);

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
                AREA_SHORT_LIGHT,
                "1,000 m²",
            ),
            (
                locale!("en-US"),
                Area::square_meter(),
                "1000",
                AREA_LONG_LIGHT,
                "1,000 m²",
            ),
            (
                locale!("fr-FR"),
                Area::square_meter(),
                "1000",
                AREA_LONG_LIGHT,
                "1\u{202f}000\u{a0}mètres carrés",
            ),
            (
                locale!("ar-EG"),
                Area::square_meter(),
                "1000",
                AREA_LONG_LIGHT,
                "١٬٠٠٠ متر مربع",
            ),
        ];

        for (locale, categorized_unit, value_str, constructor, expected) in test_cases {
            let formatter = constructor(locale.clone().into(), categorized_unit);
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
            //     AREA_SHORT_HEAVY,
            //     "TBD",
            //    ),
            (
                locale!("en-US"),
                Area::square_meter(),
                "1000",
                AREA_SHORT_HEAVY,
                "1,000 m²",
            ),
            (
                locale!("en-US"),
                Area::square_meter(),
                "1000",
                AREA_LONG_HEAVY,
                "1,000 m²",
            ),
            (
                locale!("fr-FR"),
                Area::square_meter(),
                "1000",
                AREA_LONG_HEAVY,
                "1\u{202f}000\u{a0}mètres carrés",
            ),
            (
                locale!("ar-EG"),
                Area::square_meter(),
                "1000",
                AREA_LONG_HEAVY,
                "١٬٠٠٠ متر مربع",
            ),
        ];

        for (locale, categorized_unit, value_str, constructor, expected) in test_cases {
            let formatter = constructor(locale.clone().into(), categorized_unit);
            let formatter = formatter.unwrap();
            let signed_decimal = Decimal::from_str(value_str).unwrap();
            let formatted = formatter.format_fixed_decimal(&signed_decimal);

            assert_writeable_eq!(formatted, expected);
        }
    }

    #[test]
    fn test_duration_categorized_tiny_formatter() {
        let short: Constructor<Duration> = CategorizedFormatter::<Duration>::try_new_short_tiny;
        let long: Constructor<Duration> = CategorizedFormatter::<Duration>::try_new_long_tiny;

        let test_cases = vec![
            (
                locale!("en-US"),
                Duration::second(),
                "1000",
                short,
                "1,000 sec",
            ),
            (
                locale!("en-US"),
                Duration::second(),
                "1000",
                long,
                "1,000 seconds",
            ),
            (
                locale!("fr-FR"),
                Duration::second(),
                "1000",
                long,
                "1\u{202f}000\u{a0}secondes",
            ),
            (
                locale!("ar-EG"),
                Duration::second(),
                "1000",
                long,
                "١٬٠٠٠ ثانية",
            ),
        ];

        for (locale, categorized_unit, value_str, constructor, expected) in test_cases {
            let formatter = constructor(locale.into(), categorized_unit).unwrap();
            let signed_decimal = Decimal::from_str(value_str).unwrap();
            let formatted = formatter.format_fixed_decimal(&signed_decimal);
            assert_writeable_eq!(formatted, expected);
        }
    }

    /// Each width has its own data: the same unit and locale format differently
    /// with the constructor of each width.
    #[test]
    fn test_widths_tiny_formatter() {
        let value = Decimal::from_str("1").unwrap();

        let test_cases: [(Constructor<Duration>, &str); 3] = [
            (
                CategorizedFormatter::<Duration>::try_new_long_tiny,
                "1 hour",
            ),
            (CategorizedFormatter::<Duration>::try_new_short_tiny, "1 hr"),
            (CategorizedFormatter::<Duration>::try_new_narrow_tiny, "1h"),
        ];

        for (constructor, expected) in test_cases {
            let formatter = constructor(locale!("en-US").into(), Duration::hour()).unwrap();
            assert_writeable_eq!(formatter.format_fixed_decimal(&value), expected);
        }
    }

    /// The essential units are in the tiny data of every locale,
    /// also when the unit preferences of the locale's region don't use them.
    #[test]
    fn test_essential_units_tiny_formatter() {
        let value = Decimal::from_str("1000").unwrap();

        // The unit preferences of the United States don't list `meter`.
        let formatter = CategorizedFormatter::<Length>::try_new_long_tiny(
            locale!("en-US").into(),
            Length::meter(),
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
                CategorizedFormatter::<Duration>::try_new_long_tiny(locale!("en-US").into(), unit)
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
        let constructors: [(Constructor<Duration>, &str); 3] = [
            (CategorizedFormatter::<Duration>::try_new_long_tiny, "long"),
            (
                CategorizedFormatter::<Duration>::try_new_short_tiny,
                "short",
            ),
            (
                CategorizedFormatter::<Duration>::try_new_narrow_tiny,
                "narrow",
            ),
        ];
        for locale in [
            locale!("en-US"),
            locale!("fr-FR"),
            locale!("ar-EG"),
            locale!("ja"),
        ] {
            for (constructor, width) in constructors {
                for unit in duration_units {
                    let unit = unit();
                    let cldr_id = unit.cldr_id().to_string();
                    let formatter = constructor(locale.clone().into(), unit);
                    assert!(formatter.is_ok(), "{locale} {width} {cldr_id}");
                }
            }
        }
    }
}
