// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::dimension::provider::units::display_name::UnitsDisplayName;

icu_provider::data_marker!(
    /// Display name for core length
    UnitsNameLengthCoreV1,
    UnitsDisplayName<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "units"
);

icu_provider::data_marker!(
    /// Display name for extended length
    UnitsNameLengthExtendedV1,
    UnitsDisplayName<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "units"
);

icu_provider::data_marker!(
    /// Display name for outlier length
    UnitsNameLengthOutlierV1,
    UnitsDisplayName<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "units"
);

icu_provider::data_marker!(
    /// Display name for core mass
    UnitsNameMassCoreV1,
    UnitsDisplayName<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "units"
);

icu_provider::data_marker!(
    /// Display name for extended mass
    UnitsNameMassExtendedV1,
    UnitsDisplayName<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "units"
);

icu_provider::data_marker!(
    /// Display name for outlier mass
    UnitsNameMassOutlierV1,
    UnitsDisplayName<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "units"
);
