// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! The fixed IDs of the pre-built units.
//!
//! A pre-built unit is a unit that CLDR names directly, such as `meter` or `kilometer-per-hour`.
//! The unit names data is keyed by these IDs.

use crate::measure::provider::single_unit::UnitID;

/// Returns the fixed ID of a pre-built unit, given its CLDR ID.
pub fn prebuilt_unit_id(cldr_id: &str) -> Option<UnitID> {
    let index = PREBUILT_UNITS
        .binary_search_by_key(&cldr_id, |&(name, _)| name)
        .ok()?;
    PREBUILT_UNITS.get(index).map(|&(_, id)| id)
}

/// Returns the CLDR ID of a pre-built unit, given its fixed ID.
pub fn prebuilt_unit_cldr_id(id: UnitID) -> Option<&'static str> {
    PREBUILT_UNITS
        .iter()
        .find(|&&(_, unit_id)| unit_id == id)
        .map(|&(name, _)| name)
}

/// Every unit that CLDR 49 names or has conversion data for, sorted by CLDR ID.
///
/// A simple unit has the same ID as in [`crate::measure::parser::ids`]. The other units are
/// numbered from 160, in alphabetical order. An ID must never change or be reused, because the
/// unit names data is keyed by it.
// TODO(#8678): generate this table from a tracker file instead of writing it by hand.
const PREBUILT_UNITS: &[(&str, UnitID)] = &[
    ("acre", 1),
    ("acre-foot", 160),
    ("ampere", 2),
    ("arc-minute", 3),
    ("arc-second", 4),
    ("astronomical-unit", 5),
    ("atmosphere", 6),
    ("bar", 7),
    ("barrel", 8),
    ("beaufort", 9),
    ("becquerel", 10),
    ("bit", 11),
    ("british-thermal-unit", 12),
    ("british-thermal-unit-it", 13),
    ("bu-jp", 14),
    ("bushel", 15),
    ("byte", 16),
    ("calorie", 17),
    ("calorie-it", 18),
    ("candela", 19),
    ("carat", 20),
    ("celsius", 21),
    ("centiliter", 161),
    ("centimeter", 162),
    ("century", 22),
    ("chain", 23),
    ("cho", 24),
    ("coulomb", 25),
    ("cubic-centimeter", 163),
    ("cubic-foot", 164),
    ("cubic-inch", 165),
    ("cubic-kilometer", 166),
    ("cubic-meter", 167),
    ("cubic-mile", 168),
    ("cubic-yard", 169),
    ("cup", 26),
    ("cup-imperial", 0),
    ("cup-jp", 27),
    ("cup-metric", 28),
    ("dalton", 29),
    ("day", 30),
    ("day-person", 31),
    ("decade", 32),
    ("deciliter", 170),
    ("decimeter", 171),
    ("degree", 33),
    ("dessert-spoon", 34),
    ("dessert-spoon-imperial", 35),
    ("dot", 36),
    ("dot-per-centimeter", 172),
    ("dot-per-inch", 173),
    ("dram", 37),
    ("drop", 38),
    ("dunam", 39),
    ("dyne", 157),
    ("earth-mass", 40),
    ("earth-radius", 41),
    ("electronvolt", 42),
    ("em", 43),
    ("fahrenheit", 44),
    ("farad", 45),
    ("fathom", 46),
    ("fluid-ounce", 47),
    ("fluid-ounce-imperial", 48),
    ("fluid-ounce-metric", 156),
    ("foodcalorie", 49),
    ("foot", 50),
    ("fortnight", 51),
    ("fun", 52),
    ("furlong", 53),
    ("g-force", 54),
    ("gallon", 55),
    ("gallon-imperial", 56),
    ("gasoline-energy-density", 57),
    ("generic", 174),
    ("gigabit", 175),
    ("gigabyte", 176),
    ("gigahertz", 177),
    ("gigawatt", 178),
    ("grain", 58),
    ("gram", 59),
    ("gray", 60),
    ("hectare", 61),
    ("hectoliter", 179),
    ("hectopascal", 180),
    ("henry", 62),
    ("hertz", 63),
    ("horsepower", 64),
    ("hour", 65),
    ("inch", 66),
    ("inch-ofhg", 181),
    ("item", 67),
    ("jigger", 68),
    ("jo-jp", 69),
    ("joule", 70),
    ("karat", 71),
    ("katal", 72),
    ("kelvin", 73),
    ("ken", 74),
    ("kilobit", 182),
    ("kilobyte", 183),
    ("kilocalorie", 184),
    ("kilogram", 75),
    ("kilogram-force", 76),
    ("kilohertz", 185),
    ("kilojoule", 186),
    ("kilometer", 187),
    ("kilometer-per-hour", 188),
    ("kilopascal", 189),
    ("kilowatt", 190),
    ("kilowatt-hour", 191),
    ("kilowatt-hour-per-100-kilometer", 192),
    ("knot", 77),
    ("koku", 78),
    ("kosaji", 79),
    ("light-speed", 80),
    ("light-year", 81),
    ("liter", 82),
    ("liter-per-100-kilometer", 193),
    ("liter-per-kilometer", 194),
    ("lumen", 83),
    ("lux", 84),
    ("megabit", 195),
    ("megabyte", 196),
    ("megahertz", 197),
    ("megaliter", 198),
    ("megapascal", 199),
    ("megapixel", 200),
    ("megawatt", 201),
    ("meter", 85),
    ("meter-per-second", 202),
    ("meter-per-square-second", 203),
    ("microgram", 204),
    ("micrometer", 205),
    ("microsecond", 206),
    ("mile", 86),
    ("mile-per-gallon", 207),
    ("mile-per-gallon-imperial", 208),
    ("mile-per-hour", 209),
    ("mile-scandinavian", 87),
    ("milliampere", 210),
    ("millibar", 211),
    ("milligram", 212),
    ("milligram-ofglucose-per-deciliter", 213),
    ("milliinch", 214),
    ("milliliter", 215),
    ("millimeter", 216),
    ("millimeter-ofhg", 217),
    ("millimole-per-liter", 218),
    ("millisecond", 219),
    ("milliwatt", 220),
    ("minute", 88),
    ("mole", 89),
    ("month", 90),
    ("month-person", 91),
    ("nanometer", 221),
    ("nanosecond", 222),
    ("nautical-mile", 92),
    ("newton", 93),
    ("newton-meter", 223),
    ("night", 94),
    ("ofglucose", 95),
    ("ofhg", 96),
    ("ohm", 97),
    ("osaji", 98),
    ("ounce", 99),
    ("ounce-troy", 100),
    ("parsec", 101),
    ("part", 113),
    ("part-per-1e6", 224),
    ("part-per-1e9", 225),
    ("pascal", 102),
    ("percent", 103),
    ("permille", 104),
    ("permyriad", 106),
    ("petabyte", 226),
    ("picometer", 227),
    ("pinch", 107),
    ("pint", 108),
    ("pint-imperial", 109),
    ("pint-metric", 110),
    ("pixel", 111),
    ("pixel-per-centimeter", 228),
    ("pixel-per-inch", 229),
    ("point", 112),
    ("pound", 114),
    ("pound-force", 115),
    ("pound-force-foot", 230),
    ("pound-force-per-square-inch", 231),
    ("poundal", 159),
    ("quart", 116),
    ("quart-imperial", 117),
    ("quarter", 118),
    ("radian", 119),
    ("rankine", 120),
    ("revolution", 121),
    ("ri-jp", 122),
    ("rin", 123),
    ("rod", 124),
    ("sai", 125),
    ("se-jp", 126),
    ("second", 127),
    ("shaku", 128),
    ("shaku-cloth", 129),
    ("shaku-length", 130),
    ("siemens", 131),
    ("sievert", 132),
    ("slug", 133),
    ("solar-luminosity", 134),
    ("solar-mass", 135),
    ("solar-radius", 136),
    ("square-centimeter", 232),
    ("square-foot", 233),
    ("square-inch", 234),
    ("square-kilometer", 235),
    ("square-meter", 236),
    ("square-mile", 237),
    ("square-yard", 238),
    ("steradian", 137),
    ("stone", 138),
    ("sun", 139),
    ("tablespoon", 140),
    ("teaspoon", 141),
    ("terabit", 239),
    ("terabyte", 240),
    ("tesla", 142),
    ("therm-us", 143),
    ("to-jp", 144),
    ("ton", 145),
    ("tonne", 146),
    ("volt", 147),
    ("watt", 148),
    ("weber", 149),
    ("week", 150),
    ("week-person", 151),
    ("yard", 152),
    ("year", 153),
    ("year-person", 154),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::measure::parser::ids::CLDR_IDS_TRIE;
    use alloc::collections::BTreeSet;

    #[test]
    fn test_sorted_by_cldr_id() {
        assert!(PREBUILT_UNITS.is_sorted_by(|a, b| a.0 < b.0));
    }

    #[test]
    fn test_ids_are_unique() {
        let ids: BTreeSet<UnitID> = PREBUILT_UNITS.iter().map(|&(_, id)| id).collect();
        assert_eq!(ids.len(), PREBUILT_UNITS.len());
    }

    #[test]
    fn test_round_trip() {
        for &(name, id) in PREBUILT_UNITS {
            assert_eq!(prebuilt_unit_id(name), Some(id), "{name}");
            assert_eq!(prebuilt_unit_cldr_id(id), Some(name), "{name}");
        }
        assert_eq!(prebuilt_unit_id("metre"), None);
    }

    #[test]
    fn test_simple_units_keep_their_parser_ids() {
        for (name, id) in CLDR_IDS_TRIE.iter() {
            match prebuilt_unit_id(&name) {
                Some(prebuilt_id) => assert_eq!(prebuilt_id as usize, id, "{name}"),
                // `portion` is an alias of `part`, and CLDR doesn't name it.
                None => assert_eq!(name, "portion"),
            }
        }
    }
}
