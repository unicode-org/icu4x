extern crate icu_datap_json;

use icu_datap_json::JsonDataProvider;
use icu_util::datap;
use icu_util::datap::DataProvider;

#[test]
fn test_read_string() {
    let data = r#"
        {
            "decimal": {
                "symbols_v1_a": {
                    "zero_digit": "0",
                    "decimal_separator": ".",
                    "grouping_separator": ","
                }
            }
        }"#;
    let json_data_provider = JsonDataProvider::from_str(data).unwrap();
    let response = json_data_provider.load(datap::Request {
        locale: "root".to_string(),
        category: datap::Category::Decimal,
        key: datap::Key::Decimal(datap::decimal::Key::SymbolsV1),
        payload: None
    }).unwrap();
    let decimal_data: &datap::decimal::SymbolsV1 = response.borrow_payload().unwrap();
    assert_eq!(decimal_data, &datap::decimal::SymbolsV1 {
        zero_digit: '0',
        decimal_separator: ".".to_string(),
        grouping_separator: ",".to_string(),
    });
}
