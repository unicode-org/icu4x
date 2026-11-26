// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::dimension::provider::units::display_names::UnitsDisplayNames;

/// Defines the data markers of one category: one marker for each width and each tier.
macro_rules! define_units_data_markers {
    (
        $category:literal,
        long: [$long_essential:ident, $long_imperial:ident, $long_metric:ident, $long_outlier:ident $(,)?],
        short: [$short_essential:ident, $short_imperial:ident, $short_metric:ident, $short_outlier:ident $(,)?],
        narrow: [$narrow_essential:ident, $narrow_imperial:ident, $narrow_metric:ident, $narrow_outlier:ident $(,)?] $(,)?
    ) => {
        define_units_data_markers!(
            @width $category, "Long", $long_essential, $long_imperial, $long_metric, $long_outlier,
            "\n\nNOTE\nThe data has a long name only where it differs from the short name, in the locale or in one of its parents. Where it has none, the short name is also the long name."
        );
        define_units_data_markers!(
            @width $category, "Short", $short_essential, $short_imperial, $short_metric, $short_outlier,
            ""
        );
        define_units_data_markers!(
            @width $category, "Narrow", $narrow_essential, $narrow_imperial, $narrow_metric, $narrow_outlier,
            "\n\nNOTE\nThe data has a narrow name only where it differs from the short name, in the locale or in one of its parents. Where it has none, the short name is also the narrow name."
        );
    };
    (@width $category:literal, $width:literal, $essential:ident, $imperial:ident, $metric:ident, $outlier:ident, $fallback:literal) => {
        define_units_data_markers!(
            @marker $essential, $category, $width, $fallback,
            "that are essential"
        );
        define_units_data_markers!(
            @marker $imperial, $category, $width, $fallback,
            "of the US and the UK systems that the unit preferences of some region use"
        );
        define_units_data_markers!(
            @marker $metric, $category, $width, $fallback,
            "of the other systems that the unit preferences of some region use"
        );
        define_units_data_markers!(
            @marker $outlier, $category, $width, $fallback,
            "that the unit preferences of no region use"
        );
    };
    (@marker $marker:ident, $category:literal, $width:literal, $fallback:literal, $units:literal) => {
        icu_provider::data_marker!(
            #[doc = concat!(
                $width, " display names for ", $category, " units ", $units, ". ",
                "A unit is in the same marker in every locale. ",
                "Access requires specifying the unit in DataMarkerAttributes (e.g., meter). ",
                "The pluralized displayname of the unit.",
                $fallback
            )]
            $marker,
            UnitsDisplayNames<'static>,
            #[cfg(feature = "datagen")]
            attributes_domain = "units"
        );
    };
}

define_units_data_markers!(
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

define_units_data_markers!(
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

define_units_data_markers!(
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

define_units_data_markers!(
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

define_units_data_markers!(
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
