use std::fs::File;
use std::io::BufReader;

use icu_data_provider::prelude::*;
use icu_data_provider::structs;
use icu_data_provider::validator::DataProviderValidator;
use icu_fs_data_provider::JsonDataWarehouse;

#[test]
fn test_read_json() {
    let file = File::open("tests/testdata/all.json").unwrap();
    let reader = BufReader::new(file);
    let json_data_provider = JsonDataWarehouse::from_reader(reader).unwrap();
    println!("{:?}", json_data_provider); // Coverage for Debug trait
    let validation_provider = DataProviderValidator {
        data_provider: &json_data_provider.provider(),
    };
    let response = validation_provider
        .load(&data_provider::Request {
            data_key: icu_data_key!(decimal: symbols@1),
            data_entry: DataEntry {
                variant: None,
                langid: "en-US".parse().unwrap(),
            },
        })
        .unwrap();
    let decimal_data: &structs::decimal::SymbolsV1 = response.borrow_payload().unwrap();
    assert_eq!(
        decimal_data,
        &structs::decimal::SymbolsV1 {
            zero_digit: '0',
            decimal_separator: ".".into(),
            grouping_separator: ",".into(),
        }
    );
}
