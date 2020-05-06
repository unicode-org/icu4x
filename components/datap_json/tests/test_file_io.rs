extern crate icu_datap_json;

use std::fs::File;
use std::io::BufReader;

use icu_datap_json::JsonDataProvider;
use icu_util::datap;
use icu_util::datap::DataProvider;

#[test]
fn test_read_json() {
    let file = File::open("tests/testdata/all.json").unwrap();
    let reader = BufReader::new(file);
    let json_data_provider = JsonDataProvider::from_reader(reader).unwrap();
    let response = json_data_provider
        .load(datap::Request {
            locale: "root".to_string(),
            category: datap::Category::Decimal,
            key: datap::Key::Decimal(datap::decimal::Key::SymbolsV1),
            payload: None,
        })
        .unwrap();
    let decimal_data: &datap::decimal::SymbolsV1 = response.borrow_payload().unwrap();
    assert_eq!(
        decimal_data,
        &datap::decimal::SymbolsV1 {
            zero_digit: '0',
            decimal_separator: ".".to_string(),
            grouping_separator: ",".to_string(),
        }
    );
}
