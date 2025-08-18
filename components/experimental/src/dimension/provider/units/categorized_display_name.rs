// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::dimension::provider::units::display_name::UnitsDisplayName;

icu_provider::data_marker!(
    /// Display name for core length
    CoreUnitsNameLengthV1,
    UnitsDisplayName<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "units"
);

icu_provider::data_marker!(
    /// Display name for extended length
    ExtendedUnitsNameLengthV1,
    UnitsDisplayName<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "units"
);

icu_provider::data_marker!(
    /// Display name for outlier length
    OutlierUnitsNameLengthV1,
    UnitsDisplayName<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "units"
);

icu_provider::data_marker!(
    /// Display name for core mass
    CoreUnitsNameMassV1,
    UnitsDisplayName<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "units"
);

icu_provider::data_marker!(
    /// Display name for extended mass
    ExtendedUnitsNameMassV1,
    UnitsDisplayName<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "units"
);

icu_provider::data_marker!(
    /// Display name for outlier mass
    OutlierUnitsNameMassV1,
    UnitsDisplayName<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "units"
);
