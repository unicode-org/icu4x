// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::dimension::provider::units::display_names::UnitsDisplayNames;

/// Defines the data markers of one category.
///
/// Each tier has one marker for the long names, and one marker for the short and the narrow
/// names together. The short and the narrow names of a unit are often equal, and one marker
/// stores equal names once.
macro_rules! define_units_data_markers {
    (
        $category:literal,
        long: [$long_tiny:ident, $long_light:ident, $long_heavy:ident],
        short_narrow: [
            $short_narrow_tiny:ident,
            $short_narrow_light:ident,
            $short_narrow_heavy:ident
        ] $(,)?
    ) => {
        define_units_data_markers!(
            @widths $category, "Long", "the unit", "meter",
            $long_tiny, $long_light, $long_heavy
        );
        define_units_data_markers!(
            @widths $category, "Short and narrow", "the width and the unit", "short-meter",
            $short_narrow_tiny, $short_narrow_light, $short_narrow_heavy
        );
    };
    (
        @widths $category:literal, $widths:literal, $key:literal, $example:literal,
        $tiny:ident, $light:ident, $heavy:ident
    ) => {
        define_units_data_markers!(
            @marker $tiny, $category, $widths, $key, $example,
            "that are essential or defined by locale-specific preferences"
        );
        define_units_data_markers!(
            @marker $light, $category, $widths, $key, $example,
            "covering units from other locales' preferences"
        );
        define_units_data_markers!(
            @marker $heavy, $category, $widths, $key, $example,
            "not specified by any locale's preferences"
        );
    };
    (
        @marker $marker:ident, $category:literal, $widths:literal, $key:literal,
        $example:literal, $units:literal
    ) => {
        icu_provider::data_marker!(
            #[doc = concat!(
                $widths, " display names for ", $category, " units ", $units, ". ",
                "Access requires specifying ", $key, " in DataMarkerAttributes (e.g., ",
                $example, ")."
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
    long: [UnitsNamesAreaLongTinyV1, UnitsNamesAreaLongLightV1, UnitsNamesAreaLongHeavyV1],
    short_narrow: [
        UnitsNamesAreaShortNarrowTinyV1,
        UnitsNamesAreaShortNarrowLightV1,
        UnitsNamesAreaShortNarrowHeavyV1
    ],
);

define_units_data_markers!(
    "duration",
    long: [UnitsNamesDurationLongTinyV1, UnitsNamesDurationLongLightV1, UnitsNamesDurationLongHeavyV1],
    short_narrow: [
        UnitsNamesDurationShortNarrowTinyV1,
        UnitsNamesDurationShortNarrowLightV1,
        UnitsNamesDurationShortNarrowHeavyV1
    ],
);

define_units_data_markers!(
    "length",
    long: [UnitsNamesLengthLongTinyV1, UnitsNamesLengthLongLightV1, UnitsNamesLengthLongHeavyV1],
    short_narrow: [
        UnitsNamesLengthShortNarrowTinyV1,
        UnitsNamesLengthShortNarrowLightV1,
        UnitsNamesLengthShortNarrowHeavyV1
    ],
);

define_units_data_markers!(
    "mass",
    long: [UnitsNamesMassLongTinyV1, UnitsNamesMassLongLightV1, UnitsNamesMassLongHeavyV1],
    short_narrow: [
        UnitsNamesMassShortNarrowTinyV1,
        UnitsNamesMassShortNarrowLightV1,
        UnitsNamesMassShortNarrowHeavyV1
    ],
);

define_units_data_markers!(
    "volume",
    long: [UnitsNamesVolumeLongTinyV1, UnitsNamesVolumeLongLightV1, UnitsNamesVolumeLongHeavyV1],
    short_narrow: [
        UnitsNamesVolumeShortNarrowTinyV1,
        UnitsNamesVolumeShortNarrowLightV1,
        UnitsNamesVolumeShortNarrowHeavyV1
    ],
);
