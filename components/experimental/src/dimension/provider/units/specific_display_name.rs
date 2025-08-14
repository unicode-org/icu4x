// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::dimension::provider::units::display_name::UnitsDisplayName;

icu_provider::data_marker!(
    /// Display name for area units
    UnitsNameMeterV1,
    UnitsDisplayName<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "units"
);

// macro_rules! units_specific_display_name_marker {
//     ($($name:ident),+) => {
//         $(
//             icu_provider::data_marker!(
//                 $name,
//                 UnitsDisplayName<'static>,
//                 #[cfg(feature = "datagen")]
//                 attributes_domain = "units"
//             );
//         )+
//     };
// }

// units_specific_display_name_marker!(
//     UnitsNameFootV1,
//     UnitsNameKilometerV1
// );
