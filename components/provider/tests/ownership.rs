use icu_provider::structs::decimal::SymbolsV1;
use icu_provider::v2::*;
use std::borrow::Cow;
use std::default::Default;

#[allow(clippy::redundant_static_lifetimes)]
const DATA: &'static str = r#"{
    "zero_digit": "a",
    "decimal_separator": ".",
    "grouping_separator": ","
}"#;

fn check_zero_digit<'d, 'de>(receiver: &dyn DataReceiver<'d, 'de>, expected: char) {
    let decoder = DataReceiverDecoder(receiver);
    let data: &SymbolsV1 = decoder.borrow_payload()
        .expect("Data should be present")
        .expect("Type should be correct");
    assert_eq!(data.zero_digit, expected);
}

#[test]
fn test_basic() {
    let mut receiver: DataReceiverImpl<'_, SymbolsV1> = DataReceiverImpl {
        payload: Some(Cow::Owned(SymbolsV1::default())),
    };
    check_zero_digit(&receiver, '0');

    let json = &mut serde_json::Deserializer::from_str(DATA);
    receiver
        .set_to(&mut erased_serde::Deserializer::erase(json))
        .expect("Data should be well-formed");
    check_zero_digit(&receiver, 'a');
}
