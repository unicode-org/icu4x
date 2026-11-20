// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::SourceDataProvider;
use crate::cldr_serde;
use cldr_serde::units::preferences::UnitType;
use icu::experimental::dimension::provider::units::categorized_display_names::{
    UnitsNamesAreaLongHeavyV1, UnitsNamesAreaLongLightV1, UnitsNamesAreaLongTinyV1,
    UnitsNamesAreaNarrowHeavyV1, UnitsNamesAreaNarrowLightV1, UnitsNamesAreaNarrowTinyV1,
    UnitsNamesAreaShortHeavyV1, UnitsNamesAreaShortLightV1, UnitsNamesAreaShortTinyV1,
    UnitsNamesDurationLongHeavyV1, UnitsNamesDurationLongLightV1, UnitsNamesDurationLongTinyV1,
    UnitsNamesDurationNarrowHeavyV1, UnitsNamesDurationNarrowLightV1,
    UnitsNamesDurationNarrowTinyV1, UnitsNamesDurationShortHeavyV1, UnitsNamesDurationShortLightV1,
    UnitsNamesDurationShortTinyV1, UnitsNamesLengthLongHeavyV1, UnitsNamesLengthLongLightV1,
    UnitsNamesLengthLongTinyV1, UnitsNamesLengthNarrowHeavyV1, UnitsNamesLengthNarrowLightV1,
    UnitsNamesLengthNarrowTinyV1, UnitsNamesLengthShortHeavyV1, UnitsNamesLengthShortLightV1,
    UnitsNamesLengthShortTinyV1, UnitsNamesMassLongHeavyV1, UnitsNamesMassLongLightV1,
    UnitsNamesMassLongTinyV1, UnitsNamesMassNarrowHeavyV1, UnitsNamesMassNarrowLightV1,
    UnitsNamesMassNarrowTinyV1, UnitsNamesMassShortHeavyV1, UnitsNamesMassShortLightV1,
    UnitsNamesMassShortTinyV1, UnitsNamesVolumeLongHeavyV1, UnitsNamesVolumeLongLightV1,
    UnitsNamesVolumeLongTinyV1, UnitsNamesVolumeNarrowHeavyV1, UnitsNamesVolumeNarrowLightV1,
    UnitsNamesVolumeNarrowTinyV1, UnitsNamesVolumeShortHeavyV1, UnitsNamesVolumeShortLightV1,
    UnitsNamesVolumeShortTinyV1,
};
use icu::experimental::dimension::provider::units::display_names::UnitsDisplayNames;
use icu_provider::DataMarkerAttributes;
use icu_provider::prelude::*;
use std::collections::HashSet;

/// The width of the unit names that a marker holds.
#[derive(Clone, Copy)]
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

    fn get_display_name_iter_ids_cached(
        &self,
        unit_type: UnitType,
        category: &str,
        width: Width,
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
                    &self.cldr()?.extract_or_infer_region(&locale)?.to_string(),
                    &categorized_units_list,
                ) != unit_type
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
                self.get_display_name_iter_ids_cached($unit_type, $category, $width)
            }
        }
    };
}

/// Implements the providers of the markers of one category: one for each width and tier.
macro_rules! impl_units_display_names_providers {
    (
        $category:expr,
        long: [$long_tiny:ty, $long_light:ty, $long_heavy:ty],
        short: [$short_tiny:ty, $short_light:ty, $short_heavy:ty],
        narrow: [$narrow_tiny:ty, $narrow_light:ty, $narrow_heavy:ty] $(,)?
    ) => {
        impl_units_display_names_provider!($long_tiny, UnitType::Tiny, $category, Width::Long);
        impl_units_display_names_provider!($long_light, UnitType::Light, $category, Width::Long);
        impl_units_display_names_provider!($long_heavy, UnitType::Heavy, $category, Width::Long);
        impl_units_display_names_provider!($short_tiny, UnitType::Tiny, $category, Width::Short);
        impl_units_display_names_provider!($short_light, UnitType::Light, $category, Width::Short);
        impl_units_display_names_provider!($short_heavy, UnitType::Heavy, $category, Width::Short);
        impl_units_display_names_provider!($narrow_tiny, UnitType::Tiny, $category, Width::Narrow);
        impl_units_display_names_provider!(
            $narrow_light,
            UnitType::Light,
            $category,
            Width::Narrow
        );
        impl_units_display_names_provider!(
            $narrow_heavy,
            UnitType::Heavy,
            $category,
            Width::Narrow
        );
    };
}

impl_units_display_names_providers!(
    "area",
    long: [UnitsNamesAreaLongTinyV1, UnitsNamesAreaLongLightV1, UnitsNamesAreaLongHeavyV1],
    short: [UnitsNamesAreaShortTinyV1, UnitsNamesAreaShortLightV1, UnitsNamesAreaShortHeavyV1],
    narrow: [UnitsNamesAreaNarrowTinyV1, UnitsNamesAreaNarrowLightV1, UnitsNamesAreaNarrowHeavyV1],
);

impl_units_display_names_providers!(
    "duration",
    long: [UnitsNamesDurationLongTinyV1, UnitsNamesDurationLongLightV1, UnitsNamesDurationLongHeavyV1],
    short: [UnitsNamesDurationShortTinyV1, UnitsNamesDurationShortLightV1, UnitsNamesDurationShortHeavyV1],
    narrow: [UnitsNamesDurationNarrowTinyV1, UnitsNamesDurationNarrowLightV1, UnitsNamesDurationNarrowHeavyV1],
);

impl_units_display_names_providers!(
    "length",
    long: [UnitsNamesLengthLongTinyV1, UnitsNamesLengthLongLightV1, UnitsNamesLengthLongHeavyV1],
    short: [UnitsNamesLengthShortTinyV1, UnitsNamesLengthShortLightV1, UnitsNamesLengthShortHeavyV1],
    narrow: [UnitsNamesLengthNarrowTinyV1, UnitsNamesLengthNarrowLightV1, UnitsNamesLengthNarrowHeavyV1],
);

impl_units_display_names_providers!(
    "mass",
    long: [UnitsNamesMassLongTinyV1, UnitsNamesMassLongLightV1, UnitsNamesMassLongHeavyV1],
    short: [UnitsNamesMassShortTinyV1, UnitsNamesMassShortLightV1, UnitsNamesMassShortHeavyV1],
    narrow: [UnitsNamesMassNarrowTinyV1, UnitsNamesMassNarrowLightV1, UnitsNamesMassNarrowHeavyV1],
);

impl_units_display_names_providers!(
    "volume",
    long: [UnitsNamesVolumeLongTinyV1, UnitsNamesVolumeLongLightV1, UnitsNamesVolumeLongHeavyV1],
    short: [UnitsNamesVolumeShortTinyV1, UnitsNamesVolumeShortLightV1, UnitsNamesVolumeShortHeavyV1],
    narrow: [UnitsNamesVolumeNarrowTinyV1, UnitsNamesVolumeNarrowLightV1, UnitsNamesVolumeNarrowHeavyV1],
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

    let us_locale_short_meter: DataPayload<UnitsNamesLengthShortTinyV1> = provider
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

    let fr_locale: DataPayload<UnitsNamesLengthShortTinyV1> = provider
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
