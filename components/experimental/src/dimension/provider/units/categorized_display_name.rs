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
    // Length
    UnitsNameLengthCoreV1, "Display name for core length";
    UnitsNameLengthExtendedV1, "Display name for extended length";
    UnitsNameLengthOutlierV1, "Display name for outlier length";
    // Mass
    UnitsNameMassCoreV1, "Display name for core mass";
    UnitsNameMassExtendedV1, "Display name for extended mass";
    UnitsNameMassOutlierV1, "Display name for outlier mass";
);
