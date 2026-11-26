// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON unitPreferenceData.json file.
//!
//! The file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-core/supplemental/unitPreferenceData.json>

use serde::Deserialize;
use std::collections::{BTreeMap, HashSet};

/// Represents a single unit preference entry with an optional threshold
#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct UnitPreference {
    /// The unit identifier (e.g., "square-kilometer", "hectare")
    pub(crate) unit: String,

    /// Optional threshold value - unit is preferred when the value is greater than or equal to this
    #[serde(rename = "geq")]
    pub(crate) greater_or_equal: Option<f64>,
}

/// Maps region country codes (e.g., "US", "GB", "DE") to lists of unit preferences
pub(crate) type RegionPreferences = BTreeMap<String, Vec<UnitPreference>>;

/// Maps usage contexts (e.g., "default", "person-height", etc.) to region preferences  
pub(crate) type UsagePreferences = BTreeMap<String, RegionPreferences>;

/// Maps measurement categories (e.g., "length", "mass", "duration") to usage preferences
pub(crate) type CategoryPreferences = BTreeMap<String, UsagePreferences>;

/// The supplemental data containing unit preferences
#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Supplemental {
    /// Version information for the CLDR data
    #[serde(rename = "version")]
    pub(crate) version: Option<BTreeMap<String, String>>,

    /// The main unit preference data organized by category -> usage -> region -> preferences
    #[serde(rename = "unitPreferenceData")]
    pub(crate) unit_preference_data: CategoryPreferences,
}

/// Root structure for the unitPreferenceData.json file
#[derive(Deserialize)]
pub(crate) struct Resource {
    /// The supplemental data section
    pub(crate) supplemental: Supplemental,
}

/// The tier of the names of a unit. A unit is in the same tier in every locale.
#[derive(Debug, PartialEq)]
pub(crate) enum UnitType {
    /// An essential unit, such as `second` or `meter`.
    Essential,
    /// A unit of the US or the UK system that the unit preferences use, such as `foot`.
    Imperial,
    /// Any other unit that the unit preferences use, such as `kilometer`.
    Metric,
    /// A unit that the unit preferences don't use, such as `furlong`.
    Outlier,
}

/// Maps a category (e.g., "length", "mass", "duration") to a map from each region to a set of units.
/// This excludes usages; it only checks if the unit is present for a given country in a specific category and region.
pub(crate) type CategorizedUnitsList = BTreeMap<String, BTreeMap<String, HashSet<String>>>;
