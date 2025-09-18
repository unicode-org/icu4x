// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::dimension::provider::units::display_name::UnitsDisplayName;

macro_rules! define_units_data_markers {
    ($($marker:ident, $doc:literal);* $(;)?) => {
        $(
            icu_provider::data_marker!(
                #[doc = $doc]
                $marker,
                UnitsDisplayName<'static>,
                #[cfg(feature = "datagen")]
                attributes_domain = "units"
            );
        )*
    };
}

define_units_data_markers!(
    // Area
    UnitsNamesAreaCoreV1, "Display names for core area units, encompassing the units specified by unit preferences for the given locale";
    UnitsNamesAreaExtendedV1, "Display names for extended area units, encompassing the units specified by unit preferences for the other locales";
    UnitsNamesAreaOutlierV1, "Display names for outlier area units, encompassing the units which are not specified by unit preferences for any locale";
    // Duration
    UnitsNamesDurationCoreV1, "Display names for core duration units, encompassing the units specified by unit preferences for the given locale";
    UnitsNamesDurationExtendedV1, "Display names for extended duration units, encompassing the units specified by unit preferences for the other locales";
    UnitsNamesDurationOutlierV1, "Display names for outlier duration units, encompassing the units which are not specified by unit preferences for any locale";
    // Length
    UnitsNamesLengthCoreV1, "Display names for core length units, encompassing the units specified by unit preferences for the given locale";
    UnitsNamesLengthExtendedV1, "Display names for extended length units, encompassing the units specified by unit preferences for the other locales";
    UnitsNamesLengthOutlierV1, "Display names for outlier length units, encompassing the units which are not specified by unit preferences for any locale";
    // Mass
    UnitsNamesMassCoreV1, "Display names for core mass units, encompassing the units specified by unit preferences for the given locale";
    UnitsNamesMassExtendedV1, "Display names for extended mass units, encompassing the units specified by unit preferences for the other locales";
    UnitsNamesMassOutlierV1, "Display names for outlier mass units, encompassing the units which are not specified by unit preferences for any locale";
    // Volume
    UnitsNamesVolumeCoreV1, "Display names for core volume units, encompassing the units specified by unit preferences for the given locale";
    UnitsNamesVolumeExtendedV1, "Display names for extended volume units, encompassing the units specified by unit preferences for the other locales";
    UnitsNamesVolumeOutlierV1, "Display names for outlier volume units, encompassing the units which are not specified by unit preferences for any locale";
);
