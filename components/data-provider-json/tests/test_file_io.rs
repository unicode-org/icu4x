use icu_data_provider_json;

use std::fs::File;
use std::io::BufReader;

use icu_data_provider as datap;
use icu_data_provider::DataProvider;
use icu_data_provider_json::JsonDataProvider;

#[test]
fn test_read_json() {
    let file = File::open("tests/testdata/all.json").unwrap();
    let reader = BufReader::new(file);
    let json_data_provider = JsonDataProvider::from_reader(reader).unwrap();
    println!("{:?}", json_data_provider); // Coverage for Debug trait
    let response = json_data_provider
        .load(&datap::Request {
            langid: "en-US".parse().unwrap(),
            category: datap::Category::Decimal,
            key: datap::decimal::Key::SymbolsV1.into(),
            payload: None,
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
