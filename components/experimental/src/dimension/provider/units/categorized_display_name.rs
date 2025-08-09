// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::dimension::provider::units::display_name::UnitsDisplayName;
use icu_provider::prelude::*;

icu_provider::data_marker!(
    /// `LengthDisplayNameV1`
    LengthDisplayNameV1,
    LengthDisplayName<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "units"
);

/// Represents the display name data for length units.
#[derive(Clone, PartialEq, Debug, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct LengthDisplayName<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub display_name: UnitsDisplayName<'data>,
}
