// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::dimension::provider::units::display_names::UnitsDisplayNames;

macro_rules! define_units_data_markers {
    ($($marker:ident, $doc:literal);* $(;)?) => {
        $(
            icu_provider::data_marker!(
                #[doc = $doc]
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
    UnitsNamesAreaCoreV1,
    "Display names for area units defined by locale-specific preferences. Access requires specifying the width's first letter and the unit's ID in DataMarkerAttributes (e.g., s85 for meter).";
    UnitsNamesAreaExtendedV1,
    "Display names for area units covering units from other locales\' preferences. Access requires specifying the width's first letter and the unit's ID in DataMarkerAttributes (e.g., s85 for meter).";
    UnitsNamesAreaOutlierV1,
    "Display names for area units not specified by any locale\'s preferences. Access requires specifying the width's first letter and the unit's ID in DataMarkerAttributes (e.g., s85 for meter).";

    // Duration
    UnitsNamesDurationCoreV1,
    "Display names for duration units defined by locale-specific preferences. Access requires specifying the width's first letter and the unit's ID in DataMarkerAttributes (e.g., s85 for meter).";
    UnitsNamesDurationExtendedV1,
    "Display names for duration units covering units from other locales\' preferences. Access requires specifying the width's first letter and the unit's ID in DataMarkerAttributes (e.g., s85 for meter).";
    UnitsNamesDurationOutlierV1,
    "Display names for duration units not specified by any locale\'s preferences. Access requires specifying the width's first letter and the unit's ID in DataMarkerAttributes (e.g., s85 for meter).";

    // Length
    UnitsNamesLengthCoreV1,
    "Display names for length units defined by locale-specific preferences. Access requires specifying the width's first letter and the unit's ID in DataMarkerAttributes (e.g., s85 for meter).";
    UnitsNamesLengthExtendedV1,
    "Display names for length units covering units from other locales\' preferences. Access requires specifying the width's first letter and the unit's ID in DataMarkerAttributes (e.g., s85 for meter).";
    UnitsNamesLengthOutlierV1,
    "Display names for length units not specified by any locale\'s preferences. Access requires specifying the width's first letter and the unit's ID in DataMarkerAttributes (e.g., s85 for meter).";

    // Mass
    UnitsNamesMassCoreV1,
    "Display names for mass units defined by locale-specific preferences. Access requires specifying the width's first letter and the unit's ID in DataMarkerAttributes (e.g., s85 for meter).";
    UnitsNamesMassExtendedV1,
    "Display names for mass units covering units from other locales\' preferences. Access requires specifying the width's first letter and the unit's ID in DataMarkerAttributes (e.g., s85 for meter).";
    UnitsNamesMassOutlierV1,
    "Display names for mass units not specified by any locale\'s preferences. Access requires specifying the width's first letter and the unit's ID in DataMarkerAttributes (e.g., s85 for meter).";

    // Volume
    UnitsNamesVolumeCoreV1,
    "Display names for volume units defined by locale-specific preferences. Access requires specifying the width's first letter and the unit's ID in DataMarkerAttributes (e.g., s85 for meter).";
    UnitsNamesVolumeExtendedV1,
    "Display names for volume units covering units from other locales\' preferences. Access requires specifying the width's first letter and the unit's ID in DataMarkerAttributes (e.g., s85 for meter).";
    UnitsNamesVolumeOutlierV1,
    "Display names for volume units not specified by any locale\'s preferences. Access requires specifying the width's first letter and the unit's ID in DataMarkerAttributes (e.g., s85 for meter).";
);
