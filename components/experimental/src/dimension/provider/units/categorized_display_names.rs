// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::dimension::provider::units::display_names::UnitsDisplayNames;

macro_rules! define_units_data_markers {
    ($($marker:ident, $doc:literal);* $(;)?) => {
        $(
            icu_provider::data_marker!(
                #[doc = $doc]
                ///
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
    "Display names for area units defined by locale-specific preferences. Access requires specifying width and unit in DataMarkerAttributes (e.g., short-meter).";
    UnitsNamesAreaLightV1,
    "Display names for area units covering units from other locales\' preferences. Access requires specifying width and unit in DataMarkerAttributes (e.g., short-meter).";
    UnitsNamesAreaHeavyV1,
    "Display names for area units not specified by any locale\'s preferences. Access requires specifying width and unit in DataMarkerAttributes (e.g., short-meter).";

    // Duration
    UnitsNamesDurationTinyV1,
    "Display names for duration units defined by locale-specific preferences. Access requires specifying width and unit in DataMarkerAttributes (e.g., short-meter).";
    UnitsNamesDurationLightV1,
    "Display names for duration units covering units from other locales\' preferences. Access requires specifying width and unit in DataMarkerAttributes (e.g., short-meter).";
    UnitsNamesDurationHeavyV1,
    "Display names for duration units not specified by any locale\'s preferences. Access requires specifying width and unit in DataMarkerAttributes (e.g., short-meter).";

    // Length
    UnitsNamesLengthTinyV1,
    "Display names for length units defined by locale-specific preferences. Access requires specifying width and unit in DataMarkerAttributes (e.g., short-meter).";
    UnitsNamesLengthLightV1,
    "Display names for length units covering units from other locales\' preferences. Access requires specifying width and unit in DataMarkerAttributes (e.g., short-meter).";
    UnitsNamesLengthHeavyV1,
    "Display names for length units not specified by any locale\'s preferences. Access requires specifying width and unit in DataMarkerAttributes (e.g., short-meter).";

    // Mass
    UnitsNamesMassTinyV1,
    "Display names for mass units defined by locale-specific preferences. Access requires specifying width and unit in DataMarkerAttributes (e.g., short-meter).";
    UnitsNamesMassLightV1,
    "Display names for mass units covering units from other locales\' preferences. Access requires specifying width and unit in DataMarkerAttributes (e.g., short-meter).";
    UnitsNamesMassHeavyV1,
    "Display names for mass units not specified by any locale\'s preferences. Access requires specifying width and unit in DataMarkerAttributes (e.g., short-meter).";

    // Volume
    UnitsNamesVolumeTinyV1,
    "Display names for volume units defined by locale-specific preferences. Access requires specifying width and unit in DataMarkerAttributes (e.g., short-meter).";
    UnitsNamesVolumeLightV1,
    "Display names for volume units covering units from other locales\' preferences. Access requires specifying width and unit in DataMarkerAttributes (e.g., short-meter).";
    UnitsNamesVolumeHeavyV1,
    "Display names for volume units not specified by any locale\'s preferences. Access requires specifying width and unit in DataMarkerAttributes (e.g., short-meter).";
);
