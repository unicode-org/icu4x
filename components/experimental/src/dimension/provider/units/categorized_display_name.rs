// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::dimension::provider::units::display_name::UnitsDisplayName;

icu_provider::data_marker!(
    /// `AreaDisplayNameV1`
    AreaDisplayNameV1,
    UnitsDisplayName<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "units"
);

icu_provider::data_marker!(
    /// `DurationDisplayNameV1`
    DurationDisplayNameV1,
    UnitsDisplayName<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "units"
);

icu_provider::data_marker!(
    /// `LengthDisplayNameV1`
    LengthDisplayNameV1,
    UnitsDisplayName<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "units"
);

icu_provider::data_marker!(
    /// `MassDisplayNameV1`
    MassDisplayNameV1,
    UnitsDisplayName<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "units"
);

icu_provider::data_marker!(
    /// `VolumeDisplayNameV1`
    VolumeDisplayNameV1,
    UnitsDisplayName<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "units"
);
