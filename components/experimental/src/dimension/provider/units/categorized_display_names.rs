// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::dimension::provider::units::display_names::UnitsDisplayNames;

macro_rules! define_units_data_markers {
    ($($marker:ident, $doc:literal);* $(;)?) => {
        $(
            icu_provider::data_marker!(
                #[doc = $doc]
                /// The pluralized displayname of the unit.
                ///
                /// NOTE
                /// The data has a name for the long or the narrow width only where it differs
                /// from the short name, in the locale or in one of its parents. Where it has
                /// none, the short name is the name for that width.
                $marker,
                UnitsDisplayNames<'static>,
                #[cfg(feature = "datagen")]
                attributes_domain = "units"
            );
        )*
    };
}

define_units_data_markers!(
    // Area
    UnitsNamesAreaTinyV1,
    "Display names for area units defined by locale-specific preferences. The DataMarkerAttributes are the unit for the short width (e.g., meter), and the unit with /l or /n after it for the long or the narrow width (e.g., meter/l).";
    UnitsNamesAreaLightV1,
    "Display names for area units covering units from other locales\' preferences. The DataMarkerAttributes are the unit for the short width (e.g., meter), and the unit with /l or /n after it for the long or the narrow width (e.g., meter/l).";
    UnitsNamesAreaHeavyV1,
    "Display names for area units not specified by any locale\'s preferences. The DataMarkerAttributes are the unit for the short width (e.g., meter), and the unit with /l or /n after it for the long or the narrow width (e.g., meter/l).";

    // Duration
    UnitsNamesDurationTinyV1,
    "Display names for duration units defined by locale-specific preferences. The DataMarkerAttributes are the unit for the short width (e.g., meter), and the unit with /l or /n after it for the long or the narrow width (e.g., meter/l).";
    UnitsNamesDurationLightV1,
    "Display names for duration units covering units from other locales\' preferences. The DataMarkerAttributes are the unit for the short width (e.g., meter), and the unit with /l or /n after it for the long or the narrow width (e.g., meter/l).";
    UnitsNamesDurationHeavyV1,
    "Display names for duration units not specified by any locale\'s preferences. The DataMarkerAttributes are the unit for the short width (e.g., meter), and the unit with /l or /n after it for the long or the narrow width (e.g., meter/l).";

    // Length
    UnitsNamesLengthTinyV1,
    "Display names for length units defined by locale-specific preferences. The DataMarkerAttributes are the unit for the short width (e.g., meter), and the unit with /l or /n after it for the long or the narrow width (e.g., meter/l).";
    UnitsNamesLengthLightV1,
    "Display names for length units covering units from other locales\' preferences. The DataMarkerAttributes are the unit for the short width (e.g., meter), and the unit with /l or /n after it for the long or the narrow width (e.g., meter/l).";
    UnitsNamesLengthHeavyV1,
    "Display names for length units not specified by any locale\'s preferences. The DataMarkerAttributes are the unit for the short width (e.g., meter), and the unit with /l or /n after it for the long or the narrow width (e.g., meter/l).";

    // Mass
    UnitsNamesMassTinyV1,
    "Display names for mass units defined by locale-specific preferences. The DataMarkerAttributes are the unit for the short width (e.g., meter), and the unit with /l or /n after it for the long or the narrow width (e.g., meter/l).";
    UnitsNamesMassLightV1,
    "Display names for mass units covering units from other locales\' preferences. The DataMarkerAttributes are the unit for the short width (e.g., meter), and the unit with /l or /n after it for the long or the narrow width (e.g., meter/l).";
    UnitsNamesMassHeavyV1,
    "Display names for mass units not specified by any locale\'s preferences. The DataMarkerAttributes are the unit for the short width (e.g., meter), and the unit with /l or /n after it for the long or the narrow width (e.g., meter/l).";

    // Volume
    UnitsNamesVolumeTinyV1,
    "Display names for volume units defined by locale-specific preferences. The DataMarkerAttributes are the unit for the short width (e.g., meter), and the unit with /l or /n after it for the long or the narrow width (e.g., meter/l).";
    UnitsNamesVolumeLightV1,
    "Display names for volume units covering units from other locales\' preferences. The DataMarkerAttributes are the unit for the short width (e.g., meter), and the unit with /l or /n after it for the long or the narrow width (e.g., meter/l).";
    UnitsNamesVolumeHeavyV1,
    "Display names for volume units not specified by any locale\'s preferences. The DataMarkerAttributes are the unit for the short width (e.g., meter), and the unit with /l or /n after it for the long or the narrow width (e.g., meter/l).";
);
