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

/// Determines the type of unit
#[derive(PartialEq)]
pub(crate) enum UnitType {
    Core,
    Extended,
    Outlier,
}

/// Maps a category (e.g., "length", "mass", "duration") to a map from each region to a set of units.
/// This excludes usages; it only checks if the unit is present for a given country in a specific category and region.
pub(crate) type CategorizedUnitsList = BTreeMap<String, BTreeMap<String, HashSet<String>>>;

impl Supplemental {
    /// Helper function to check if a target unit matches any unit in the set,
    /// including as a subunit in compound units (e.g., "foot" matches "foot-and-inch")
    fn unit_matches_any(target_unit: &str, units: &HashSet<String>) -> bool {
        units.iter().any(|u| u.contains(target_unit))
    }

    /// Returns a map from each category (e.g., "length", "mass", "duration") to a map from each region to a set of units.
    /// This excludes usages; it only checks if the unit is present for a given country in a specific category and region.
    /// NOTE:
    ///   The units can be present as compound units (e.g., "square-meter") or mixed units (e.g., "foot-and-inch").
    pub(crate) fn categorized_units_list(&self) -> CategorizedUnitsList {
        let mut result = BTreeMap::new();

        // Iterate through each category in the unit preference data
        for (category, usage_prefs) in &self.unit_preference_data {
            let mut region_units_map = BTreeMap::new();

            // For each usage in this category (e.g., "default", "person", etc.)
            for region_prefs in usage_prefs.values() {
                // For each region in this usage
                for (region_str, unit_prefs) in region_prefs {
                    // Get or create the units set for this region (using string key)
                    let units_set = region_units_map
                        .entry(region_str.clone())
                        .or_insert_with(HashSet::new);

                    // Add all units for this region (HashSet automatically handles duplicates)
                    for unit_pref in unit_prefs {
                        units_set.insert(unit_pref.unit.clone());
                    }
                }
            }

            result.insert(category.clone(), region_units_map);
        }

        result
    }

    /// Determines the type of unit based on the region and category.
    /// If the unit is present in the unit preferences and the region is present in the unit preferences, it is a core unit.
    /// If the unit is present in the unit preferences but not in the region preferences, it is an extended unit.
    /// If the unit is not present in the unit preferences, it is an outlier unit.
    ///
    /// If the specific region is not found, falls back to region "001" (world/default region).
    ///
    /// The matching includes subunit matching: a unit like "foot" will match compound units like "foot-and-inch".
    pub(crate) fn unit_type(
        category: &str,
        unit: &str,
        region: &str,
        categorized_units_list: &CategorizedUnitsList,
    ) -> UnitType {
        let region_units = match categorized_units_list.get(category) {
            Some(units) => units,
            None => return UnitType::Outlier,
        };

        let found_in_region = region_units
            .get(region)
            .or_else(|| region_units.get("001"))
            .is_some_and(|units| Self::unit_matches_any(unit, units));

        let found_anywhere = region_units
            .values()
            .any(|units| Self::unit_matches_any(unit, units));

        if found_in_region {
            UnitType::Core
        } else if found_anywhere {
            UnitType::Extended
        } else {
            UnitType::Outlier
        }
    }
}
