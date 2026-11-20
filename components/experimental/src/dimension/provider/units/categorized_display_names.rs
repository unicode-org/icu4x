// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::dimension::provider::units::display_names::UnitsDisplayNames;

/// Defines the data markers of one category: one marker for each width and each tier.
macro_rules! define_units_data_markers {
    (
        $category:literal,
        long: [$long_tiny:ident, $long_light:ident, $long_heavy:ident],
        short: [$short_tiny:ident, $short_light:ident, $short_heavy:ident],
        narrow: [$narrow_tiny:ident, $narrow_light:ident, $narrow_heavy:ident] $(,)?
    ) => {
        define_units_data_markers!(@width $category, "Long", $long_tiny, $long_light, $long_heavy);
        define_units_data_markers!(@width $category, "Short", $short_tiny, $short_light, $short_heavy);
        define_units_data_markers!(@width $category, "Narrow", $narrow_tiny, $narrow_light, $narrow_heavy);
    };
    (@width $category:literal, $width:literal, $tiny:ident, $light:ident, $heavy:ident) => {
        define_units_data_markers!(
            @marker $tiny, $category, $width,
            "that are essential or defined by locale-specific preferences"
        );
        define_units_data_markers!(
            @marker $light, $category, $width,
            "covering units from other locales' preferences"
        );
        define_units_data_markers!(
            @marker $heavy, $category, $width,
            "not specified by any locale's preferences"
        );
    };
    (@marker $marker:ident, $category:literal, $width:literal, $units:literal) => {
        icu_provider::data_marker!(
            #[doc = concat!(
                $width, " display names for ", $category, " units ", $units, ". ",
                "Access requires specifying the unit in DataMarkerAttributes (e.g., meter)."
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
    short: [UnitsNamesAreaShortTinyV1, UnitsNamesAreaShortLightV1, UnitsNamesAreaShortHeavyV1],
    narrow: [UnitsNamesAreaNarrowTinyV1, UnitsNamesAreaNarrowLightV1, UnitsNamesAreaNarrowHeavyV1],
);

define_units_data_markers!(
    "duration",
    long: [UnitsNamesDurationLongTinyV1, UnitsNamesDurationLongLightV1, UnitsNamesDurationLongHeavyV1],
    short: [UnitsNamesDurationShortTinyV1, UnitsNamesDurationShortLightV1, UnitsNamesDurationShortHeavyV1],
    narrow: [UnitsNamesDurationNarrowTinyV1, UnitsNamesDurationNarrowLightV1, UnitsNamesDurationNarrowHeavyV1],
);

define_units_data_markers!(
    "length",
    long: [UnitsNamesLengthLongTinyV1, UnitsNamesLengthLongLightV1, UnitsNamesLengthLongHeavyV1],
    short: [UnitsNamesLengthShortTinyV1, UnitsNamesLengthShortLightV1, UnitsNamesLengthShortHeavyV1],
    narrow: [UnitsNamesLengthNarrowTinyV1, UnitsNamesLengthNarrowLightV1, UnitsNamesLengthNarrowHeavyV1],
);

define_units_data_markers!(
    "mass",
    long: [UnitsNamesMassLongTinyV1, UnitsNamesMassLongLightV1, UnitsNamesMassLongHeavyV1],
    short: [UnitsNamesMassShortTinyV1, UnitsNamesMassShortLightV1, UnitsNamesMassShortHeavyV1],
    narrow: [UnitsNamesMassNarrowTinyV1, UnitsNamesMassNarrowLightV1, UnitsNamesMassNarrowHeavyV1],
);

define_units_data_markers!(
    "volume",
    long: [UnitsNamesVolumeLongTinyV1, UnitsNamesVolumeLongLightV1, UnitsNamesVolumeLongHeavyV1],
    short: [UnitsNamesVolumeShortTinyV1, UnitsNamesVolumeShortLightV1, UnitsNamesVolumeShortHeavyV1],
    narrow: [UnitsNamesVolumeNarrowTinyV1, UnitsNamesVolumeNarrowLightV1, UnitsNamesVolumeNarrowHeavyV1],
);
