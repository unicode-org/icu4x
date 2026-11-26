// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::SourceDataProvider;
use crate::cldr_serde;
use cldr_serde::units::preferences::UnitType;
use icu::experimental::dimension::provider::units::categorized_display_names::{
    UnitsNamesAreaLongEssentialV1, UnitsNamesAreaLongImperialV1, UnitsNamesAreaLongMetricV1,
    UnitsNamesAreaLongOutlierV1, UnitsNamesAreaNarrowEssentialV1, UnitsNamesAreaNarrowImperialV1,
    UnitsNamesAreaNarrowMetricV1, UnitsNamesAreaNarrowOutlierV1, UnitsNamesAreaShortEssentialV1,
    UnitsNamesAreaShortImperialV1, UnitsNamesAreaShortMetricV1, UnitsNamesAreaShortOutlierV1,
    UnitsNamesDurationLongEssentialV1, UnitsNamesDurationLongImperialV1,
    UnitsNamesDurationLongMetricV1, UnitsNamesDurationLongOutlierV1,
    UnitsNamesDurationNarrowEssentialV1, UnitsNamesDurationNarrowImperialV1,
    UnitsNamesDurationNarrowMetricV1, UnitsNamesDurationNarrowOutlierV1,
    UnitsNamesDurationShortEssentialV1, UnitsNamesDurationShortImperialV1,
    UnitsNamesDurationShortMetricV1, UnitsNamesDurationShortOutlierV1,
    UnitsNamesLengthLongEssentialV1, UnitsNamesLengthLongImperialV1, UnitsNamesLengthLongMetricV1,
    UnitsNamesLengthLongOutlierV1, UnitsNamesLengthNarrowEssentialV1,
    UnitsNamesLengthNarrowImperialV1, UnitsNamesLengthNarrowMetricV1,
    UnitsNamesLengthNarrowOutlierV1, UnitsNamesLengthShortEssentialV1,
    UnitsNamesLengthShortImperialV1, UnitsNamesLengthShortMetricV1, UnitsNamesLengthShortOutlierV1,
    UnitsNamesMassLongEssentialV1, UnitsNamesMassLongImperialV1, UnitsNamesMassLongMetricV1,
    UnitsNamesMassLongOutlierV1, UnitsNamesMassNarrowEssentialV1, UnitsNamesMassNarrowImperialV1,
    UnitsNamesMassNarrowMetricV1, UnitsNamesMassNarrowOutlierV1, UnitsNamesMassShortEssentialV1,
    UnitsNamesMassShortImperialV1, UnitsNamesMassShortMetricV1, UnitsNamesMassShortOutlierV1,
    UnitsNamesVolumeLongEssentialV1, UnitsNamesVolumeLongImperialV1, UnitsNamesVolumeLongMetricV1,
    UnitsNamesVolumeLongOutlierV1, UnitsNamesVolumeNarrowEssentialV1,
    UnitsNamesVolumeNarrowImperialV1, UnitsNamesVolumeNarrowMetricV1,
    UnitsNamesVolumeNarrowOutlierV1, UnitsNamesVolumeShortEssentialV1,
    UnitsNamesVolumeShortImperialV1, UnitsNamesVolumeShortMetricV1, UnitsNamesVolumeShortOutlierV1,
};
use icu::experimental::dimension::provider::units::display_names::UnitsDisplayNames;
use icu::locale::fallback::{LocaleFallbackIterator, LocaleFallbacker};
use icu_provider::DataMarkerAttributes;
use icu_provider::prelude::*;
use std::collections::HashSet;

/// The width of the unit names that a marker holds.
#[derive(Clone, Copy, PartialEq)]
enum Width {
    Long,
    Short,
    Narrow,
}

impl Width {
    /// Returns the data of this width from the units data of a locale.
    fn select(
        self,
        units: &cldr_serde::units::data::UnitsData,
    ) -> &cldr_serde::units::data::UnitsLengthData {
        match self {
            Self::Long => &units.long,
            Self::Short => &units.short,
            Self::Narrow => &units.narrow,
        }
    }
}

impl SourceDataProvider {
    fn get_display_name_payload<M>(
        &self,
        req: DataRequest,
        width: Width,
    ) -> Result<DataResponse<M>, DataError>
    where
        M: DataMarker<DataStruct = UnitsDisplayNames<'static>>,
    {
        let unit = req.id.marker_attributes.as_str();

        let units_format_data: &cldr_serde::units::data::Resource = self
            .cldr()?
            .units()
            .read_and_parse(req.id.locale, "units.json")?;

        let unit_patterns = width
            .select(&units_format_data.main.value.units)
            .categories
            .iter()
            .find_map(|(_, units_map)| units_map.get(unit))
            .ok_or_else(|| DataErrorKind::IdentifierNotFound.with_req(M::INFO, req))?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(unit_patterns.try_into_plural_elements_packed_cow()?),
        })
    }

    /// Returns whether the patterns of a unit for a width differ from its short patterns
    /// in a locale or in one of its parents. Only then does the data need them.
    ///
    /// As in CLDR, the formatter looks for the long or the narrow patterns in the locale and
    /// its parents, and takes the short patterns when it finds none. A parent counts,
    /// because the formatter would find its patterns before the short ones of the locale:
    /// the narrow `{0} noches` of `es-419` is also its short pattern, and its parent `es`
    /// has the narrow `{0}noches`.
    ///
    /// The CLDR JSON data is resolved, so this function compares the patterns.
    fn differs_from_short(
        &self,
        mut locales: LocaleFallbackIterator,
        width: Width,
        category: &str,
        unit: &str,
    ) -> Result<bool, DataError> {
        let units = self.cldr()?.units();
        loop {
            let locale = locales.get();
            if units.file_exists(locale, "units.json")? {
                let data: &cldr_serde::units::data::Resource =
                    units.read_and_parse(locale, "units.json")?;
                let data = &data.main.value.units;
                let patterns = |width: Width| {
                    width
                        .select(data)
                        .categories
                        .get(category)?
                        .get(unit)?
                        .try_into_plural_elements_packed_cow()
                        .ok()
                };
                let width_patterns = patterns(width);
                if width_patterns.is_some() && width_patterns != patterns(Width::Short) {
                    return Ok(true);
                }
            }
            if locale.is_unknown() {
                return Ok(false);
            }
            locales.step();
        }
    }

    fn get_display_name_iter_ids_cached<M: DataMarker>(
        &self,
        unit_type: UnitType,
        category: &str,
        width: Width,
    ) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        let fallbacker = LocaleFallbacker::try_new_unstable(self)?;
        let fallbacker = fallbacker.for_config(M::INFO.fallback_config);

        let mut data_locales = HashSet::new();
        let units = self.cldr()?.units();
        let locales = units.list_locales()?;
        for locale in locales {
            // Load and parse the unit constants from the supplemental data file.
            let preferences: &cldr_serde::units::preferences::Resource = self
                .cldr()?
                .core()
                .read_and_parse("supplemental/unitPreferenceData.json")?;

            let categorized_units_list = preferences.supplemental.categorized_units_list();

            // The tier of a unit depends on its unit systems.
            let units_info: &cldr_serde::units::info::Resource = self
                .cldr()?
                .core()
                .read_and_parse("supplemental/units.json")?;

            let units_format_data: &cldr_serde::units::data::Resource =
                self.cldr()?.units().read_and_parse(&locale, "units.json")?;
            let units_map = width
                .select(&units_format_data.main.value.units)
                .categories
                .get(category)
                .ok_or_else(|| {
                    DataErrorKind::InvalidRequest
                        .into_error()
                        .with_debug_context(category)
                })?;
            for (unit, patterns) in units_map {
                if patterns.other.is_none() {
                    continue;
                }

                if cldr_serde::units::preferences::Supplemental::unit_type(
                    category,
                    unit,
                    &categorized_units_list,
                    &units_info.supplemental.convert_units.convert_units,
                ) != unit_type
                {
                    continue;
                }

                // The formatter takes the short patterns when the data has no patterns
                // for the width, as CLDR does.
                if width != Width::Short
                    && !self.differs_from_short(
                        fallbacker.fallback_for(locale),
                        width,
                        category,
                        unit,
                    )?
                {
                    continue;
                }

                data_locales.insert(DataIdentifierCow::from_owned(
                    DataMarkerAttributes::try_from_string(unit.clone()).map_err(|_| {
                        DataError::custom("Failed to parse the attribute").with_debug_context(&unit)
                    })?,
                    locale,
                ));
            }
        }

        Ok(data_locales)
    }
}

macro_rules! impl_units_display_names_provider {
    ($marker:ty, $unit_type:expr, $category:expr, $width:expr) => {
        impl DataProvider<$marker> for SourceDataProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.check_req::<$marker>(req)?;
                self.get_display_name_payload::<$marker>(req, $width)
            }
        }

        impl crate::IterableDataProviderCached<$marker> for SourceDataProvider {
            fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                self.get_display_name_iter_ids_cached::<$marker>($unit_type, $category, $width)
            }
        }
    };
}

/// Implements the providers of the markers of one category: one for each width and tier.
macro_rules! impl_units_display_names_providers {
    (
        $category:expr,
        long: [$long_essential:ty, $long_imperial:ty, $long_metric:ty, $long_outlier:ty $(,)?],
        short: [$short_essential:ty, $short_imperial:ty, $short_metric:ty, $short_outlier:ty $(,)?],
        narrow: [$narrow_essential:ty, $narrow_imperial:ty, $narrow_metric:ty, $narrow_outlier:ty $(,)?] $(,)?
    ) => {
        impl_units_display_names_providers!(
            @width $category, Width::Long,
            $long_essential, $long_imperial, $long_metric, $long_outlier
        );
        impl_units_display_names_providers!(
            @width $category, Width::Short,
            $short_essential, $short_imperial, $short_metric, $short_outlier
        );
        impl_units_display_names_providers!(
            @width $category, Width::Narrow,
            $narrow_essential, $narrow_imperial, $narrow_metric, $narrow_outlier
        );
    };
    (@width $category:expr, $width:expr, $essential:ty, $imperial:ty, $metric:ty, $outlier:ty) => {
        impl_units_display_names_provider!($essential, UnitType::Essential, $category, $width);
        impl_units_display_names_provider!($imperial, UnitType::Imperial, $category, $width);
        impl_units_display_names_provider!($metric, UnitType::Metric, $category, $width);
        impl_units_display_names_provider!($outlier, UnitType::Outlier, $category, $width);
    };
}

impl_units_display_names_providers!(
    "area",
    long: [
        UnitsNamesAreaLongEssentialV1,
        UnitsNamesAreaLongImperialV1,
        UnitsNamesAreaLongMetricV1,
        UnitsNamesAreaLongOutlierV1,
    ],
    short: [
        UnitsNamesAreaShortEssentialV1,
        UnitsNamesAreaShortImperialV1,
        UnitsNamesAreaShortMetricV1,
        UnitsNamesAreaShortOutlierV1,
    ],
    narrow: [
        UnitsNamesAreaNarrowEssentialV1,
        UnitsNamesAreaNarrowImperialV1,
        UnitsNamesAreaNarrowMetricV1,
        UnitsNamesAreaNarrowOutlierV1,
    ],
);

impl_units_display_names_providers!(
    "duration",
    long: [
        UnitsNamesDurationLongEssentialV1,
        UnitsNamesDurationLongImperialV1,
        UnitsNamesDurationLongMetricV1,
        UnitsNamesDurationLongOutlierV1,
    ],
    short: [
        UnitsNamesDurationShortEssentialV1,
        UnitsNamesDurationShortImperialV1,
        UnitsNamesDurationShortMetricV1,
        UnitsNamesDurationShortOutlierV1,
    ],
    narrow: [
        UnitsNamesDurationNarrowEssentialV1,
        UnitsNamesDurationNarrowImperialV1,
        UnitsNamesDurationNarrowMetricV1,
        UnitsNamesDurationNarrowOutlierV1,
    ],
);

impl_units_display_names_providers!(
    "length",
    long: [
        UnitsNamesLengthLongEssentialV1,
        UnitsNamesLengthLongImperialV1,
        UnitsNamesLengthLongMetricV1,
        UnitsNamesLengthLongOutlierV1,
    ],
    short: [
        UnitsNamesLengthShortEssentialV1,
        UnitsNamesLengthShortImperialV1,
        UnitsNamesLengthShortMetricV1,
        UnitsNamesLengthShortOutlierV1,
    ],
    narrow: [
        UnitsNamesLengthNarrowEssentialV1,
        UnitsNamesLengthNarrowImperialV1,
        UnitsNamesLengthNarrowMetricV1,
        UnitsNamesLengthNarrowOutlierV1,
    ],
);

impl_units_display_names_providers!(
    "mass",
    long: [
        UnitsNamesMassLongEssentialV1,
        UnitsNamesMassLongImperialV1,
        UnitsNamesMassLongMetricV1,
        UnitsNamesMassLongOutlierV1,
    ],
    short: [
        UnitsNamesMassShortEssentialV1,
        UnitsNamesMassShortImperialV1,
        UnitsNamesMassShortMetricV1,
        UnitsNamesMassShortOutlierV1,
    ],
    narrow: [
        UnitsNamesMassNarrowEssentialV1,
        UnitsNamesMassNarrowImperialV1,
        UnitsNamesMassNarrowMetricV1,
        UnitsNamesMassNarrowOutlierV1,
    ],
);

impl_units_display_names_providers!(
    "volume",
    long: [
        UnitsNamesVolumeLongEssentialV1,
        UnitsNamesVolumeLongImperialV1,
        UnitsNamesVolumeLongMetricV1,
        UnitsNamesVolumeLongOutlierV1,
    ],
    short: [
        UnitsNamesVolumeShortEssentialV1,
        UnitsNamesVolumeShortImperialV1,
        UnitsNamesVolumeShortMetricV1,
        UnitsNamesVolumeShortOutlierV1,
    ],
    narrow: [
        UnitsNamesVolumeNarrowEssentialV1,
        UnitsNamesVolumeNarrowImperialV1,
        UnitsNamesVolumeNarrowMetricV1,
        UnitsNamesVolumeNarrowOutlierV1,
    ],
);

#[test]
fn test_basic() {
    use icu::locale::{data_locale, locale};
    use icu::plurals::PluralRules;
    use icu_provider::prelude::*;
    use writeable::assert_writeable_eq;

    let provider = SourceDataProvider::new_testing();

    let us_locale_long_meter: DataPayload<UnitsNamesLengthLongEssentialV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("meter"),
                &data_locale!("en"),
            ),
            ..Default::default()
        })
        .unwrap()
        .payload;

    let en_rules = PluralRules::try_new_cardinal_unstable(&provider, locale!("en").into()).unwrap();
    let long = us_locale_long_meter
        .get()
        .get(1.into(), &en_rules)
        .interpolate([1]);
    assert_writeable_eq!(long, "1 meter");

    let us_locale_short_meter: DataPayload<UnitsNamesLengthShortEssentialV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("meter"),
                &data_locale!("en"),
            ),
            ..Default::default()
        })
        .unwrap()
        .payload;

    let short = us_locale_short_meter
        .get()
        .get(5.into(), &en_rules)
        .interpolate([5]);
    assert_writeable_eq!(short, "5 m");

    let ar_eg_locale: DataPayload<UnitsNamesLengthLongEssentialV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("meter"),
                &data_locale!("ar-EG"),
            ),
            ..Default::default()
        })
        .unwrap()
        .payload;

    let ar_rules = PluralRules::try_new_cardinal_unstable(&provider, locale!("ar").into()).unwrap();
    let long = ar_eg_locale.get().get(1.into(), &ar_rules).interpolate([1]);
    assert_writeable_eq!(long, "متر");

    let fr_locale: DataPayload<UnitsNamesLengthShortEssentialV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("meter"),
                &data_locale!("fr"),
            ),
            ..Default::default()
        })
        .unwrap()
        .payload;

    let fr_rules = PluralRules::try_new_cardinal_unstable(&provider, locale!("fr").into()).unwrap();
    let short = fr_locale.get().get(5.into(), &fr_rules).interpolate([5]);
    assert_writeable_eq!(short, "5 m");
}

/// Loads the names of a unit for a locale from the marker `M`.
/// Returns `None` if the data has no such names.
#[cfg(test)]
fn load_names<M>(provider: &SourceDataProvider, locale: &str, unit: &str) -> Option<DataPayload<M>>
where
    M: DataMarker,
    SourceDataProvider: DataProvider<M>,
{
    provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic(unit),
                &locale.parse().unwrap(),
            ),
            ..Default::default()
        })
        .allow_identifier_not_found()
        .unwrap()
        .map(|response| response.payload)
}

/// The data has the long and the narrow patterns of a unit only where the formatter
/// can't take the short ones.
#[test]
fn test_long_and_narrow_differ_from_short() {
    let provider = &SourceDataProvider::new_testing();
    let long_essential =
        |locale, unit| load_names::<UnitsNamesDurationLongEssentialV1>(provider, locale, unit);
    let short_essential =
        |locale, unit| load_names::<UnitsNamesDurationShortEssentialV1>(provider, locale, unit);
    let narrow_essential =
        |locale, unit| load_names::<UnitsNamesDurationNarrowEssentialV1>(provider, locale, unit);
    let long_outlier =
        |locale, unit| load_names::<UnitsNamesDurationLongOutlierV1>(provider, locale, unit);
    let short_outlier =
        |locale, unit| load_names::<UnitsNamesDurationShortOutlierV1>(provider, locale, unit);
    let narrow_outlier =
        |locale, unit| load_names::<UnitsNamesDurationNarrowOutlierV1>(provider, locale, unit);

    // In root, the long and the narrow patterns are aliases of the short ones.
    assert!(short_essential("und", "day").is_some());
    assert!(long_essential("und", "day").is_none());
    assert!(narrow_essential("und", "day").is_none());

    // In English, the long patterns of `day` are the short ones (`{0} day`, `{0} days`),
    // and the narrow pattern is `{0}d`.
    assert!(short_essential("en", "day").is_some());
    assert!(long_essential("en", "day").is_none());
    assert!(narrow_essential("en", "day").is_some());

    // The narrow patterns of `night` in `es-AR` are its short ones (`{0} noches`).
    // But `es`, a parent of `es-AR`, has the narrow `{0}noches`, and the formatter
    // would find it for `es-AR`. So `es-AR` keeps its narrow patterns.
    let es_ar_narrow = narrow_outlier("es-AR", "night").unwrap();
    let es_ar_short = short_outlier("es-AR", "night").unwrap();
    let es_narrow = narrow_outlier("es", "night").unwrap();
    assert_eq!(es_ar_narrow.get(), es_ar_short.get());
    assert_ne!(es_ar_narrow.get(), es_narrow.get());

    // The long patterns of `night` are the short ones in `es-AR`, in `es`, and in root.
    assert!(long_outlier("es-AR", "night").is_none());
    assert!(long_outlier("es", "night").is_none());
}

/// A unit is in the same tier in every locale, so a locale and its parents have the
/// names of the unit in the same marker.
#[test]
fn test_unit_tier_does_not_depend_on_locale() {
    let provider = &SourceDataProvider::new_testing();
    let essential =
        |locale, unit| load_names::<UnitsNamesLengthShortEssentialV1>(provider, locale, unit);
    let imperial =
        |locale, unit| load_names::<UnitsNamesLengthShortImperialV1>(provider, locale, unit);
    let metric = |locale, unit| load_names::<UnitsNamesLengthShortMetricV1>(provider, locale, unit);
    let outlier =
        |locale, unit| load_names::<UnitsNamesLengthShortOutlierV1>(provider, locale, unit);

    // The unit preferences of the United States list `foot`, and not `kilometer`.
    for locale in ["und", "en", "en-001", "fr", "ja"] {
        assert!(imperial(locale, "foot").is_some());
        assert!(essential(locale, "foot").is_none());
        assert!(metric(locale, "foot").is_none());
        assert!(outlier(locale, "foot").is_none());

        assert!(metric(locale, "kilometer").is_some());
        assert!(essential(locale, "kilometer").is_none());
        assert!(imperial(locale, "kilometer").is_none());
        assert!(outlier(locale, "kilometer").is_none());

        assert!(essential(locale, "meter").is_some());
        assert!(outlier(locale, "furlong").is_some());
    }
}
