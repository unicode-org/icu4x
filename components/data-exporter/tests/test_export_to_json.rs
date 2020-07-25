use icu_data_exporter::DataExporter;
use icu_data_exporter::JsonFileWriter;

use std::convert::TryFrom;
use std::fs;

use icu_cldr_json_data_provider::CldrPluralsDataProvider;
use icu_data_provider::*;

use std::path::PathBuf;

#[test]
fn test_basic() {
    let json_str = fs::read_to_string("tests/testdata/plurals.json").unwrap();
    let provider = CldrPluralsDataProvider::try_from(json_str.as_str()).unwrap();

    let mut json_file_writer = JsonFileWriter {
        root: PathBuf::from("/tmp/icu4x_json"),
    };

    let mut data_exporter = DataExporter {
        data_provider: &provider,
        file_writer: &mut json_file_writer,
    };

    data_exporter
        .write_data_key::<plurals::PluralRuleStringsV1>(&icu_data_key!(plurals: cardinal@1))
        .unwrap();
}
