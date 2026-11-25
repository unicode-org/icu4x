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
    DataRequest, DataRequestMetadata, ResultDataError,
};

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
///
/// There is one constructor for each width (long, short, narrow) and each tier of units
/// (tiny, light, heavy), such as [`Self::try_new_long_tiny`].
///
/// As in CLDR, a unit has a long or a narrow name only where it differs from the short
/// name, in the locale or in one of its parents. So the constructors of the long and the
/// narrow widths need the data of their width and of the short width.
#[derive(Debug)]
pub struct CategorizedFormatter<C: MeasureUnitCategory> {
    _category: PhantomData<C>,
    display_name: DisplayName,
    decimal_formatter: DecimalFormatter,
    plural_rules: PluralRules,
}

impl<C: MeasureUnitCategory> CategorizedFormatter<C> {
    /// Loads the display name of the unit from the marker `M`.
    ///
    /// A silent request does not log a name that the marker does not have.
    fn load_display_name<M, D>(
        provider: &D,
        prefs: CategorizedUnitsFormatterPreferences,
        categorized_unit: &CategorizedMeasureUnit<C>,
        silent: bool,
    ) -> Result<DisplayName, DataError>
    where
        M: DataMarker<DataStruct = UnitsDisplayNames<'static>>,
        D: ?Sized + DataProvider<M>,
    {
        let locale = M::make_locale(prefs.locale_preferences);
        let unit = DataMarkerAttributes::try_from_str(categorized_unit.cldr_id())
            .map_err(|_| DataError::custom("Failed to create a data marker"))?;
        let mut metadata = DataRequestMetadata::default();
        metadata.silent = silent;

        let response = DataProvider::<M>::load(
            provider,
            DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(unit, &locale),
                metadata,
            },
        )?;

        Ok(response.payload.cast())
    }

    /// Loads the display name of the unit from the first loader that has it.
    ///
    /// Each loader loads the name from one marker, with a silent request or not. A name
    /// that a marker does not have is expected, so only that error leads to the next
    /// loader, and the requests are silent except for the last one.
    fn load_first_display_name(
        first: &dyn Fn(bool) -> Result<DisplayName, DataError>,
        rest: &[&dyn Fn(bool) -> Result<DisplayName, DataError>],
    ) -> Result<DisplayName, DataError> {
        let mut load = first;
        for next in rest {
            if let Some(display_name) = load(true).allow_identifier_not_found()? {
                return Ok(display_name);
            }
            load = next;
        }
        load(false)
    }

    /// Creates the formatter from the display name of the unit, with compiled data for the rest.
    #[cfg(feature = "compiled_data")]
    fn try_new_with_display_name(
        prefs: CategorizedUnitsFormatterPreferences,
        display_name: DisplayName,
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

    /// Creates the formatter from the display name of the unit, and loads the rest from
    /// the provider.
    fn try_new_with_display_name_unstable<D>(
        provider: &D,
        prefs: CategorizedUnitsFormatterPreferences,
        display_name: DisplayName,
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

/// Implements the constructors of one width and one set of tiers: one with compiled data,
/// one with an unstable provider, and one with a buffer provider if it is named.
///
/// The constructors load the display name of the unit from the first of the `markers` that
/// has it. For the long and the narrow widths, the markers of the short width come after
/// the markers of the width: where the data has no name for the width, the short name is
/// the name for that width. The markers of the width come first for all the tiers, the way
/// CLDR goes through all the parents of the locale before it takes the short name.
///
/// A name that a marker does not have is expected. Only that error leads to the next
/// marker, and the requests are silent except for the last marker.
macro_rules! impl_constructors {
    (
        $width:literal,
        $tiers:literal,
        functions: [$new:ident, $new_unstable:ident $(, $new_buffer:ident)?],
        markers: [$first:ident $(, $rest:ident)* $(,)?] $(,)?
    ) => {
        impl<C: MeasureUnitCategory> CategorizedFormatter<C> {
            $(
                icu_provider::gen_buffer_data_constructors!(
                    (
                        prefs: CategorizedUnitsFormatterPreferences,
                        categorized_unit: CategorizedMeasureUnit<C>
                    ) -> error: DataError,
                    functions: [
                        $new: skip,
                        $new_buffer,
                        $new_unstable,
                        Self
                    ]
                );
            )?

            #[doc = concat!(
                "Creates a new [`CategorizedFormatter`] with the ", $width,
                " width from compiled locale data with the ", $tiers, " markers.",
            )]
            ///
            /// ✨ *Enabled with the `compiled_data` Cargo feature.*
            ///
            /// [📚 Help choosing a constructor](icu_provider::constructors)
            #[cfg(feature = "compiled_data")]
            pub fn $new(
                prefs: CategorizedUnitsFormatterPreferences,
                categorized_unit: CategorizedMeasureUnit<C>,
            ) -> Result<Self, DataError>
            where
                crate::provider::Baked: DataProvider<C::$first>,
                $(crate::provider::Baked: DataProvider<C::$rest>,)*
            {
                let provider = &crate::provider::Baked;

                // TODO(#8125): a marker might return the display name of a parent locale, even if the locale has its own one in a marker of a later tier.
                let display_name = Self::load_first_display_name(
                    &|silent| {
                        Self::load_display_name::<C::$first, _>(
                            provider,
                            prefs,
                            &categorized_unit,
                            silent,
                        )
                    },
                    &[$(&|silent| {
                        Self::load_display_name::<C::$rest, _>(
                            provider,
                            prefs,
                            &categorized_unit,
                            silent,
                        )
                    }),*],
                )?;

                Self::try_new_with_display_name(prefs, display_name)
            }

            #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::$new)]
            pub fn $new_unstable<D>(
                provider: &D,
                prefs: CategorizedUnitsFormatterPreferences,
                categorized_unit: CategorizedMeasureUnit<C>,
            ) -> Result<Self, DataError>
            where
                D: ?Sized
                    + DataProvider<C::$first>
                    $(+ DataProvider<C::$rest>)*
                    + DataProvider<icu_decimal::provider::DecimalSymbolsV1>
                    + DataProvider<icu_decimal::provider::DecimalDigitsV1>
                    + DataProvider<icu_plurals::provider::PluralsCardinalV1>,
            {
                // TODO(#8125): a marker might return the display name of a parent locale, even if the locale has its own one in a marker of a later tier.
                let display_name = Self::load_first_display_name(
                    &|silent| {
                        Self::load_display_name::<C::$first, _>(
                            provider,
                            prefs,
                            &categorized_unit,
                            silent,
                        )
                    },
                    &[$(&|silent| {
                        Self::load_display_name::<C::$rest, _>(
                            provider,
                            prefs,
                            &categorized_unit,
                            silent,
                        )
                    }),*],
                )?;

                Self::try_new_with_display_name_unstable(provider, prefs, display_name)
            }
        }
    };
}

impl_constructors!(
    "long",
    "tiny",
    functions: [
        try_new_long_tiny,
        try_new_long_tiny_unstable,
        try_new_long_tiny_with_buffer_provider
    ],
    markers: [DataMarkerLongTiny, DataMarkerShortTiny],
);
impl_constructors!(
    "long",
    "tiny and light",
    functions: [
        try_new_long_light,
        try_new_long_light_unstable,
        try_new_long_light_with_buffer_provider
    ],
    markers: [
        DataMarkerLongTiny,
        DataMarkerLongLight,
        DataMarkerShortTiny,
        DataMarkerShortLight,
    ],
);
impl_constructors!(
    "long",
    "tiny, light and heavy",
    functions: [try_new_long_heavy, try_new_long_heavy_unstable],
    markers: [
        DataMarkerLongTiny,
        DataMarkerLongLight,
        DataMarkerLongHeavy,
        DataMarkerShortTiny,
        DataMarkerShortLight,
        DataMarkerShortHeavy,
    ],
);

impl_constructors!(
    "short",
    "tiny",
    functions: [
        try_new_short_tiny,
        try_new_short_tiny_unstable,
        try_new_short_tiny_with_buffer_provider
    ],
    markers: [DataMarkerShortTiny],
);
impl_constructors!(
    "short",
    "tiny and light",
    functions: [
        try_new_short_light,
        try_new_short_light_unstable,
        try_new_short_light_with_buffer_provider
    ],
    markers: [DataMarkerShortTiny, DataMarkerShortLight],
);
impl_constructors!(
    "short",
    "tiny, light and heavy",
    functions: [try_new_short_heavy, try_new_short_heavy_unstable],
    markers: [DataMarkerShortTiny, DataMarkerShortLight, DataMarkerShortHeavy],
);

impl_constructors!(
    "narrow",
    "tiny",
    functions: [
        try_new_narrow_tiny,
        try_new_narrow_tiny_unstable,
        try_new_narrow_tiny_with_buffer_provider
    ],
    markers: [DataMarkerNarrowTiny, DataMarkerShortTiny],
);
impl_constructors!(
    "narrow",
    "tiny and light",
    functions: [
        try_new_narrow_light,
        try_new_narrow_light_unstable,
        try_new_narrow_light_with_buffer_provider
    ],
    markers: [
        DataMarkerNarrowTiny,
        DataMarkerNarrowLight,
        DataMarkerShortTiny,
        DataMarkerShortLight,
    ],
);
impl_constructors!(
    "narrow",
    "tiny, light and heavy",
    functions: [try_new_narrow_heavy, try_new_narrow_heavy_unstable],
    markers: [
        DataMarkerNarrowTiny,
        DataMarkerNarrowLight,
        DataMarkerNarrowHeavy,
        DataMarkerShortTiny,
        DataMarkerShortLight,
        DataMarkerShortHeavy,
    ],
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
            // TODO(#8125): `square-meter` is not a tiny unit in `en`, so this is the short name of root.
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
                "1,000 square meters",
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
                "1,000 square meters",
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

    /// The long and the narrow names of a unit fall back to its short name, as in CLDR.
    #[test]
    fn test_long_and_narrow_fall_back_to_short() {
        let value = Decimal::from_str("1000").unwrap();

        let long: Constructor<Duration> = CategorizedFormatter::<Duration>::try_new_long_tiny;
        let short: Constructor<Duration> = CategorizedFormatter::<Duration>::try_new_short_tiny;
        let narrow: Constructor<Duration> = CategorizedFormatter::<Duration>::try_new_narrow_tiny;

        let test_cases = [
            // In English, the long name of `day` is also its short name.
            (locale!("en-US"), long, "1,000 days"),
            (locale!("en-US"), short, "1,000 days"),
            (locale!("en-US"), narrow, "1,000d"),
            // Root has only short names.
            (locale!("und"), long, "1,000 d"),
            (locale!("und"), narrow, "1,000 d"),
            // The narrow name of `day` in `es-CO` is also its short name.
            // Its parent `es-419` has another narrow name, which `es-CO` must not take.
            (locale!("es-419"), narrow, "1,000dd."),
            (locale!("es-CO"), short, "1.000 días"),
            (locale!("es-CO"), narrow, "1.000 días"),
        ];

        for (locale, constructor, expected) in test_cases {
            let formatter = constructor(locale.into(), Duration::day()).unwrap();
            assert_writeable_eq!(formatter.format_fixed_decimal(&value), expected);
        }
    }

    /// Only a name that a marker does not have leads to the next marker. The requests are
    /// silent except for the last one, so a name that is missing as expected is not logged.
    #[test]
    fn test_only_a_missing_name_leads_to_the_next_marker() {
        use core::cell::RefCell;
        use icu_provider::DataErrorKind;

        // No marker has the name.
        let requests = RefCell::new(Vec::new());
        let missing = |silent| {
            requests.borrow_mut().push(silent);
            Err(DataErrorKind::IdentifierNotFound.into_error())
        };
        let result = CategorizedFormatter::<Duration>::load_first_display_name(
            &missing,
            &[&missing, &missing],
        );
        assert_eq!(result.unwrap_err().kind, DataErrorKind::IdentifierNotFound);
        assert_eq!(requests.into_inner(), [true, true, false]);

        // Another error is returned, and the next marker is not asked.
        let requests = RefCell::new(0);
        let fails = |_silent| {
            *requests.borrow_mut() += 1;
            Err(DataError::custom("another error"))
        };
        let result = CategorizedFormatter::<Duration>::load_first_display_name(&fails, &[&fails]);
        assert_eq!(result.unwrap_err().kind, DataErrorKind::Custom);
        assert_eq!(requests.into_inner(), 1);
    }
}
