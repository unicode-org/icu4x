// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::SourceDataProvider;
use crate::cldr_serde;
use cldr_serde::units::preferences::UnitType;
use icu::experimental::dimension::provider::units::categorized_display_names::{
    UnitsNamesAreaLongHeavyV1, UnitsNamesAreaLongLightV1, UnitsNamesAreaLongTinyV1,
    UnitsNamesAreaShortNarrowHeavyV1, UnitsNamesAreaShortNarrowLightV1,
    UnitsNamesAreaShortNarrowTinyV1, UnitsNamesDurationLongHeavyV1, UnitsNamesDurationLongLightV1,
    UnitsNamesDurationLongTinyV1, UnitsNamesDurationShortNarrowHeavyV1,
    UnitsNamesDurationShortNarrowLightV1, UnitsNamesDurationShortNarrowTinyV1,
    UnitsNamesLengthLongHeavyV1, UnitsNamesLengthLongLightV1, UnitsNamesLengthLongTinyV1,
    UnitsNamesLengthShortNarrowHeavyV1, UnitsNamesLengthShortNarrowLightV1,
    UnitsNamesLengthShortNarrowTinyV1, UnitsNamesMassLongHeavyV1, UnitsNamesMassLongLightV1,
    UnitsNamesMassLongTinyV1, UnitsNamesMassShortNarrowHeavyV1, UnitsNamesMassShortNarrowLightV1,
    UnitsNamesMassShortNarrowTinyV1, UnitsNamesVolumeLongHeavyV1, UnitsNamesVolumeLongLightV1,
    UnitsNamesVolumeLongTinyV1, UnitsNamesVolumeShortNarrowHeavyV1,
    UnitsNamesVolumeShortNarrowLightV1, UnitsNamesVolumeShortNarrowTinyV1,
};
use icu::experimental::dimension::provider::units::display_names::UnitsDisplayNames;
use icu_provider::DataMarkerAttributes;
use icu_provider::prelude::*;
use std::collections::HashSet;

/// The widths of the unit names that a marker holds.
#[derive(Clone, Copy)]
enum Widths {
    /// The long names. The key of a unit is the unit, such as `meter`.
    Long,
    /// The short and the narrow names. The key of a unit is the width and the unit,
    /// such as `short-meter`.
    ShortNarrow,
}

impl Widths {
    /// Returns the data of each width of the marker, with the prefix of its keys.
    fn select(
        self,
        units: &cldr_serde::units::data::UnitsData,
    ) -> Vec<(&'static str, &cldr_serde::units::data::UnitsLengthData)> {
        match self {
            Self::Long => vec![("", &units.long)],
            Self::ShortNarrow => vec![("short-", &units.short), ("narrow-", &units.narrow)],
        }
    }
}

impl SourceDataProvider {
    fn get_display_name_payload<M>(
        &self,
        req: DataRequest,
        widths: Widths,
    ) -> Result<DataResponse<M>, DataError>
    where
        M: DataMarker<DataStruct = UnitsDisplayNames<'static>>,
    {
        let key = req.id.marker_attributes.as_str();

        let units_format_data: &cldr_serde::units::data::Resource = self
            .cldr()?
            .units()
            .read_and_parse(req.id.locale, "units.json")?;

        let unit_patterns = widths
            .select(&units_format_data.main.value.units)
            .into_iter()
            .find_map(|(prefix, width_data)| {
                let unit = key.strip_prefix(prefix)?;
                width_data
                    .categories
                    .iter()
                    .find_map(|(_, units_map)| units_map.get(unit))
            })
            .ok_or_else(|| DataErrorKind::IdentifierNotFound.with_req(M::INFO, req))?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(unit_patterns.try_into_plural_elements_packed_cow()?),
        })
    }

    fn get_display_name_iter_ids_cached(
        &self,
        unit_type: UnitType,
        category: &str,
        widths: Widths,
    ) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
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

            let units_format_data: &cldr_serde::units::data::Resource =
                self.cldr()?.units().read_and_parse(&locale, "units.json")?;
            for (prefix, width_data) in widths.select(&units_format_data.main.value.units) {
                let units_map = width_data.categories.get(category).ok_or_else(|| {
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
                        &self.cldr()?.extract_or_infer_region(&locale)?.to_string(),
                        &categorized_units_list,
                    ) != unit_type
                    {
                        continue;
                    }

                    data_locales.insert(DataIdentifierCow::from_owned(
                        DataMarkerAttributes::try_from_string(format!("{prefix}{unit}")).map_err(
                            |_| {
                                DataError::custom("Failed to parse the attribute")
                                    .with_debug_context(&unit)
                            },
                        )?,
                        locale,
                    ));
                }
            }
        }

        Ok(data_locales)
    }
}

macro_rules! impl_units_display_names_provider {
    ($marker:ty, $unit_type:expr, $category:expr, $widths:expr) => {
        impl DataProvider<$marker> for SourceDataProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.check_req::<$marker>(req)?;
                self.get_display_name_payload::<$marker>(req, $widths)
            }
        }

        impl crate::IterableDataProviderCached<$marker> for SourceDataProvider {
            fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                self.get_display_name_iter_ids_cached($unit_type, $category, $widths)
            }
        }
    };
}

/// Implements the providers of the markers of one category: for each tier, one marker
/// for the long names, and one for the short and the narrow names.
macro_rules! impl_units_display_names_providers {
    (
        $category:expr,
        long: [$long_tiny:ty, $long_light:ty, $long_heavy:ty],
        short_narrow: [$short_narrow_tiny:ty, $short_narrow_light:ty, $short_narrow_heavy:ty] $(,)?
    ) => {
        impl_units_display_names_provider!($long_tiny, UnitType::Tiny, $category, Widths::Long);
        impl_units_display_names_provider!($long_light, UnitType::Light, $category, Widths::Long);
        impl_units_display_names_provider!($long_heavy, UnitType::Heavy, $category, Widths::Long);
        impl_units_display_names_provider!(
            $short_narrow_tiny,
            UnitType::Tiny,
            $category,
            Widths::ShortNarrow
        );
        impl_units_display_names_provider!(
            $short_narrow_light,
            UnitType::Light,
            $category,
            Widths::ShortNarrow
        );
        impl_units_display_names_provider!(
            $short_narrow_heavy,
            UnitType::Heavy,
            $category,
            Widths::ShortNarrow
        );
    };
}

impl_units_display_names_providers!(
    "area",
    long: [UnitsNamesAreaLongTinyV1, UnitsNamesAreaLongLightV1, UnitsNamesAreaLongHeavyV1],
    short_narrow: [UnitsNamesAreaShortNarrowTinyV1, UnitsNamesAreaShortNarrowLightV1, UnitsNamesAreaShortNarrowHeavyV1],
);

impl_units_display_names_providers!(
    "duration",
    long: [UnitsNamesDurationLongTinyV1, UnitsNamesDurationLongLightV1, UnitsNamesDurationLongHeavyV1],
    short_narrow: [UnitsNamesDurationShortNarrowTinyV1, UnitsNamesDurationShortNarrowLightV1, UnitsNamesDurationShortNarrowHeavyV1],
);

impl_units_display_names_providers!(
    "length",
    long: [UnitsNamesLengthLongTinyV1, UnitsNamesLengthLongLightV1, UnitsNamesLengthLongHeavyV1],
    short_narrow: [UnitsNamesLengthShortNarrowTinyV1, UnitsNamesLengthShortNarrowLightV1, UnitsNamesLengthShortNarrowHeavyV1],
);

impl_units_display_names_providers!(
    "mass",
    long: [UnitsNamesMassLongTinyV1, UnitsNamesMassLongLightV1, UnitsNamesMassLongHeavyV1],
    short_narrow: [UnitsNamesMassShortNarrowTinyV1, UnitsNamesMassShortNarrowLightV1, UnitsNamesMassShortNarrowHeavyV1],
);

impl_units_display_names_providers!(
    "volume",
    long: [UnitsNamesVolumeLongTinyV1, UnitsNamesVolumeLongLightV1, UnitsNamesVolumeLongHeavyV1],
    short_narrow: [UnitsNamesVolumeShortNarrowTinyV1, UnitsNamesVolumeShortNarrowLightV1, UnitsNamesVolumeShortNarrowHeavyV1],
);

#[test]
fn test_basic() {
    use icu::locale::{data_locale, locale};
    use icu::plurals::PluralRules;
    use icu_provider::prelude::*;
    use writeable::assert_writeable_eq;

    let provider = SourceDataProvider::new_testing();

    let us_locale_long_meter: DataPayload<UnitsNamesLengthLongTinyV1> = provider
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

    let us_locale_short_meter: DataPayload<UnitsNamesLengthShortNarrowTinyV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("short-meter"),
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

    let ar_eg_locale: DataPayload<UnitsNamesLengthLongTinyV1> = provider
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

    let fr_locale: DataPayload<UnitsNamesLengthShortNarrowTinyV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("short-meter"),
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
