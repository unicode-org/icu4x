//! A temporary app used to generate bincode resource files out of
//! JSON CLDR resources.
//!
//! TODO: This will be replaced by a real DataProvider and DataExtractor.
use icu_pluralrules::data::io::json::get_resource;
use icu_pluralrules::PluralRuleType;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    {
        let res = unsafe { get_resource(PluralRuleType::Cardinal).unwrap() };
        let encoded: Vec<u8> = bincode::serialize(&res).unwrap();
        let mut buffer = File::create("./data/plurals.dat").expect("Opening file failed");
        buffer.write_all(&encoded).expect("Writing failed");
    }
    {
        let res = unsafe { get_resource(PluralRuleType::Ordinal).unwrap() };
        let encoded: Vec<u8> = bincode::serialize(&res).unwrap();
        let mut buffer = File::create("./data/ordinals.dat").expect("Opening file failed");
        buffer.write_all(&encoded).expect("Writing failed");
    }
}
