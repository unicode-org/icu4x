use std::fs::File;
use std::io::BufReader;

use icu_data_provider::prelude::*;
use icu_data_provider_json::JsonDataProvider;

#[test]
fn test_read_json() {
    let file = File::open("tests/testdata/all.json").unwrap();
    let reader = BufReader::new(file);
    let json_data_provider = JsonDataProvider::from_reader(reader).unwrap();
    println!("{:?}", json_data_provider); // Coverage for Debug trait
    let validation_provider = datap::DataProviderValidator {
        data_provider: &json_data_provider,
    };
    let response = validation_provider
        .load(&datap::Request {
            data_key: icu_data_key!(decimal: symbols@1),
            data_entry: datap::DataEntry {
                variant: None,
                langid: "en-US".parse().unwrap(),
            },
        })
        .unwrap();
    let decimal_data: &datap::decimal::SymbolsV1 = response.borrow_payload().unwrap();
    assert_eq!(
        decimal_data,
        &datap::decimal::SymbolsV1 {
            zero_digit: '0',
            decimal_separator: ".".into(),
            grouping_separator: ",".into(),
        }
    );
}
