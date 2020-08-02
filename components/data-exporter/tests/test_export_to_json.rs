use icu_data_exporter::json_exporter;
use icu_data_exporter::JsonFileWriter;

use std::convert::TryFrom;
use std::fs;

use icu_cldr_json_data_provider::transform::PluralsProvider;
use icu_data_provider::icu_data_key;

use std::path::PathBuf;

#[test]
fn test_basic() {
    let json_str = fs::read_to_string("tests/testdata/plurals.json").unwrap();
    let provider = PluralsProvider::try_from(json_str.as_str()).unwrap();

    let mut json_options = json_exporter::Options::default();
    json_options.root = PathBuf::from("/tmp/icu4x_json");
    json_options.aliasing = json_exporter::AliasOption::Symlink;
    // let json_options = json_exporter::Options {
    //     ..Default::default()
    // };
    let mut json_file_writer = JsonFileWriter::try_new(&json_options).unwrap();

    let mut data_exporter = DataExporter {
        data_provider: &provider,
        file_writer: &mut json_file_writer,
    };

    data_exporter
        .write_data_key(
            &icu_data_key!(plurals: cardinal@1),
        )
        .unwrap();

    json_file_writer.flush().unwrap();
}
