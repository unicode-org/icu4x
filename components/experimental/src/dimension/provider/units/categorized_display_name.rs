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
    UnitsNameAreaCoreV1, "Display names for core area units, encompassing the primary area-related units specified by unit preferences for the given locale";
    UnitsNameAreaExtendedV1, "Display names for extended area units, encompassing the secondary area-related units specified by unit preferences for the other locales";
    UnitsNameAreaOutlierV1, "Display names for outlier area units, encompassing the tertiary area-related units which are not specified by unit preferences for any locale";
    // Duration
    UnitsNameDurationCoreV1, "Display names for core duration units, encompassing the primary duration-related units specified by unit preferences for the given locale";
    UnitsNameDurationExtendedV1, "Display names for extended duration units, encompassing the secondary duration-related units specified by unit preferences for the other locales";
    UnitsNameDurationOutlierV1, "Display names for outlier duration units, encompassing the tertiary duration-related units which are not specified by unit preferences for any locale";
    // Length
    UnitsNameLengthCoreV1, "Display names for core length units, encompassing the primary length-related units specified by unit preferences for the given locale";
    UnitsNameLengthExtendedV1, "Display names for extended length units, encompassing the secondary length-related units specified by unit preferences for the other locales";
    UnitsNameLengthOutlierV1, "Display names for outlier length units, encompassing the tertiary length-related units which are not specified by unit preferences for any locale";
    // Mass
    UnitsNameMassCoreV1, "Display names for core mass units, encompassing the primary mass-related units specified by unit preferences for the given locale";
    UnitsNameMassExtendedV1, "Display names for extended mass units, encompassing the secondary mass-related units specified by unit preferences for the other locales";
    UnitsNameMassOutlierV1, "Display names for outlier mass units, encompassing the tertiary mass-related units which are not specified by unit preferences for any locale";
    // Volume
    UnitsNameVolumeCoreV1, "Display names for core volume units, encompassing the primary volume-related units specified by unit preferences for the given locale";
    UnitsNameVolumeExtendedV1, "Display names for extended volume units, encompassing the secondary volume-related units specified by unit preferences for the other locales";
    UnitsNameVolumeOutlierV1, "Display names for outlier volume units, encompassing the tertiary volume-related units which are not specified by unit preferences for any locale";
);
