use icu_data_exporter::DataExporter;

use std::convert::TryFrom;
use std::fs;

use icu_cldr_json_data_provider::CldrPluralsDataProvider;
use icu_data_provider::*;

use erased_serde;
use serde_json;

#[test]
fn test_basic() {
    let json_str = fs::read_to_string("tests/testdata/plurals.json").unwrap();
    let provider = CldrPluralsDataProvider::try_from(json_str.as_str()).unwrap();

    let data_key = DataKey {
        category: Category::Plurals,
        sub_category: "cardinal".parse().unwrap(),
        version: 1,
    };

    let mut data_exporter = DataExporter {
        data_provider: &provider,
        serialize_fn: |writer, obj| {
            let mut json = serde_json::Serializer::new(writer);
            obj.erased_serialize(&mut erased_serde::Serializer::erase(&mut json))?;
            Ok(())
        }
    };

    data_exporter.write_data_key::<plurals::PluralRuleStringsV1>(&data_key).unwrap();
}

